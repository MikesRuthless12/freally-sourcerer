//! TASK-102 smoke — the daemon installer flow.
//!
//! The thing this guards is not the registration; it is what the
//! registration *starts*. `launchd.rs` and `systemd.rs` shipped their
//! Phase-2 / Phase-3 keep-alive shells — `loop { thread::sleep(60) }` —
//! for long enough that `freally-indexd install` on macOS and Linux
//! reported success, showed as loaded, and maintained no index at all.
//! Nothing caught it, because a supervisor watching for a live process
//! was getting exactly that.
//!
//! So the assertion below is the one that would have caught it: run the
//! installed daemon's body, and require that something can talk to it.
//!
//! Both parameters of `installed::run` are `None` for every real caller.
//! The test passes scratch paths instead, so it never touches the
//! machine's own index or binds its real service endpoint.

use std::sync::Arc;
use std::time::Duration;

use freally_rpc::{Client, IndexState, SocketPath, default_socket_path, service_socket_path};

/// A scratch endpoint that cannot collide with a real one, or with a
/// sibling test running at the same moment.
fn scratch_socket(dir: &std::path::Path, tag: &str) -> SocketPath {
    if cfg!(windows) {
        SocketPath::Pipe(format!(
            r"\\.\pipe\freally-indexd-test-{tag}-{}",
            std::process::id()
        ))
    } else {
        SocketPath::Path(dir.join(format!("{tag}.sock")))
    }
}

#[test]
fn the_installed_daemon_serves_rpc_and_then_stops_cleanly() {
    let tmp = tempfile::TempDir::new().unwrap();
    let index_root = tmp.path().join("index");
    let socket = scratch_socket(tmp.path(), "installed");

    let (ready_tx, ready_rx) = std::sync::mpsc::channel::<()>();
    let (stop_tx, stop_rx) = tokio::sync::oneshot::channel::<()>();
    let (done_tx, done_rx) = std::sync::mpsc::channel::<Result<(), String>>();

    let socket_for_daemon = socket.clone();
    let daemon = std::thread::spawn(move || {
        let rt = tokio::runtime::Builder::new_multi_thread()
            .enable_all()
            .build()
            .unwrap();
        let outcome = rt.block_on(freally_indexd::installed::run_with(
            Some(index_root),
            Some(socket_for_daemon),
            move || {
                let _ = ready_tx.send(());
            },
            async move {
                let _ = stop_rx.await;
            },
        ));
        let _ = done_tx.send(outcome.map_err(|e| e.to_string()));
    });

    // `on_ready` fires after the endpoint is bound, so a client may
    // connect from here on. Without it this would be a sleep-and-hope.
    ready_rx
        .recv_timeout(Duration::from_secs(30))
        .expect("the installed daemon never reported ready");

    let rt = tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()
        .unwrap();
    let state: IndexState = rt.block_on(async {
        // Binding and *accepting* are a few microseconds apart; a short
        // retry covers that without papering over a daemon that never
        // listens at all (the 30 s ready wait above already failed then).
        let mut last = None;
        for _ in 0..100 {
            match Client::connect(socket.clone()).await {
                Ok(c) => {
                    let c: Arc<_> = Arc::new(c);
                    return c
                        .call("index.state", serde_json::json!({}))
                        .await
                        .expect("index.state");
                }
                Err(e) => {
                    last = Some(e);
                    tokio::time::sleep(Duration::from_millis(50)).await;
                }
            }
        }
        panic!("could not connect to the installed daemon: {last:?}");
    });
    // A fresh scratch index has nothing in it — the point is that a real
    // typed answer came back over the wire, not a keep-alive loop.
    assert_eq!(state.files_total, 0);

    let _ = stop_tx.send(());
    let outcome = done_rx
        .recv_timeout(Duration::from_secs(30))
        .expect("the installed daemon ignored the stop signal");
    daemon.join().expect("daemon thread panicked");
    outcome.expect("the installed daemon returned an error on shutdown");
}

#[test]
fn the_installed_endpoint_is_not_the_one_the_app_spawns_a_child_on() {
    // The app probes the installed endpoint and falls back to spawning
    // its own daemon on the default one. If those were the same path,
    // the fallback would race the installed daemon for the bind and one
    // of the two would lose — silently, at boot.
    let installed = format!("{:?}", service_socket_path());
    let default = format!("{:?}", default_socket_path());
    assert_ne!(
        installed, default,
        "the installed daemon and the app's own child would fight over one endpoint"
    );
}
