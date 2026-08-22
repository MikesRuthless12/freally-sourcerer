//! The body the **installed** daemon runs — the Windows service, the
//! launchd agent, and the systemd user unit all share this one.
//!
//! It exists because the three had drifted apart. Windows grew a real
//! daemon body when the service work landed; `launchd.rs` and
//! `systemd.rs` were still carrying their Phase-2 / Phase-3 keep-alive
//! shells, a `loop { thread::sleep(60) }` that transitions to "running"
//! and then serves nothing. So on macOS and Linux, `freally-indexd
//! install` reported success, the agent showed as loaded, and no index
//! was ever maintained. Nothing detected that, because a keep-alive loop
//! is exactly what `KeepAlive=true` and `Restart=always` are watching
//! for — the *supervisor* was satisfied.
//!
//! What differs per OS is only how the process is told to start and
//! stop, so that is what the two parameters carry:
//!
//! - `on_ready` fires once the RPC endpoint is bound and the daemon can
//!   serve. Windows reports `SERVICE_RUNNING` to the SCM here; launchd
//!   and systemd have nothing to report and pass a no-op.
//! - `stop` resolves when the supervisor asks the daemon to stop —
//!   `SERVICE_CONTROL_STOP` on Windows, `SIGTERM` under launchd and
//!   systemd. Whichever arrives, the shutdown path below is the same,
//!   and it matters: without it the process dies on the default SIGTERM
//!   disposition, abandoning journal events that were applied but not
//!   yet committed.

use std::future::Future;
use std::path::PathBuf;
use std::sync::Arc;

use anyhow::{Context, Result};
use freally_index::service_index_root;
use freally_rpc::{ServerConfig, SocketPath, service_socket_path};

use crate::{DaemonOptions, DaemonState, service};

/// Open the index, bind the well-known service endpoint, start scanning,
/// and serve until `stop` resolves or the accept loop dies.
pub async fn run<R, S>(on_ready: R, stop: S) -> Result<()>
where
    R: FnOnce(),
    S: Future<Output = ()>,
{
    run_with(None, None, on_ready, stop).await
}

/// [`run`] against caller-chosen paths.
///
/// A test seam, not part of the contract: it exists so the smoke test can
/// drive this body against a scratch directory and endpoint instead of the
/// machine's own index, which is the only way to assert that it *serves*
/// rather than sleeps. Every production caller goes through [`run`].
#[doc(hidden)]
pub async fn run_with<R, S>(
    index_root: Option<PathBuf>,
    socket: Option<SocketPath>,
    on_ready: R,
    stop: S,
) -> Result<()>
where
    R: FnOnce(),
    S: Future<Output = ()>,
{
    let index_root = match index_root {
        Some(p) => p,
        None => service_index_root().context("service_index_root")?,
    };
    tracing::info!(index_root = %index_root.display(), "installed daemon: opening state");

    let opts = DaemonOptions {
        index_root: Some(index_root),
        // Windows installs one machine-wide service running as SYSTEM,
        // so the per-user undo journal is refused rather than shared
        // between whoever happens to be logged in. macOS and Linux
        // install a per-user agent, which has exactly one user and can
        // keep its journal.
        shared_multi_user: cfg!(windows),
        ..Default::default()
    };
    let state: Arc<DaemonState> = DaemonState::open(opts)?;

    let socket = socket.unwrap_or_else(service_socket_path);
    tracing::info!(?socket, "installed daemon: binding RPC endpoint");
    // `spawn_with`, not a second copy: it owns the catalogs-before-watchers
    // reconcile order, which the Windows service's old private copy had
    // already drifted from once.
    let mut server_handle = crate::spawn_with(state.clone(), server_config(socket)).await?;

    on_ready();

    // Scan whatever is configured without waiting to be asked. The point
    // of an installed daemon is that the index is warm before the UI
    // opens — and the UI may not open at all if Freally is being driven
    // from the CLI or the HTTP endpoint.
    let folders = state.folders.read().await.clone();
    if folders.is_empty() {
        tracing::info!("installed daemon: no folders configured yet; waiting for IPC");
    } else {
        tracing::info!(count = folders.len(), "installed daemon: initial scans");
        for f in folders {
            service::scan(&state, &std::path::PathBuf::from(&f.path));
        }
    }

    tokio::select! {
        _ = &mut server_handle => {
            tracing::warn!("installed daemon: RPC accept loop exited unexpectedly");
        }
        () = stop => {
            tracing::info!("installed daemon: stop requested");
            server_handle.abort();
        }
    }

    // Stop live journaling before persisting: the consumer commits
    // whatever it has pending on the way out, so returning without this
    // abandons applied-but-uncommitted events.
    state.watchers.shutdown();
    let _ = state.persist().await;
    tracing::info!("installed daemon: clean shutdown");
    Ok(())
}

/// The Windows service pipe needs a DACL that lets unelevated user
/// processes reach a SYSTEM-owned endpoint. A Unix socket needs no
/// equivalent — it is created under the user's own directory and the
/// server already checks the peer uid on accept.
#[cfg(windows)]
fn server_config(socket: SocketPath) -> ServerConfig {
    ServerConfig {
        socket,
        sddl_override: Some(freally_rpc::service_sddl()),
    }
}

#[cfg(not(windows))]
fn server_config(socket: SocketPath) -> ServerConfig {
    ServerConfig::new(socket)
}

/// The whole body of a launchd agent or a systemd user unit: build a
/// runtime, run [`run`], stop on SIGTERM. The two differ only in the line
/// they log, which is why they pass it in rather than each carrying a
/// copy of this.
#[cfg(unix)]
pub fn run_unix_agent(what: &str) -> Result<()> {
    tracing::info!(unit = what, "entering the installed daemon body");
    let rt = tokio::runtime::Builder::new_multi_thread()
        .enable_all()
        .build()
        .context("building the daemon's tokio runtime")?;
    rt.block_on(run(|| {}, unix_stop_signal()))
}

/// Resolves on `SIGTERM` or `SIGINT`. This is what `systemctl --user
/// stop` and `launchctl bootout` send, and their default disposition is
/// to kill the process outright — which would skip the persist above.
#[cfg(unix)]
pub async fn unix_stop_signal() {
    use tokio::signal::unix::{SignalKind, signal};
    let mut term = match signal(SignalKind::terminate()) {
        Ok(s) => s,
        Err(e) => {
            tracing::warn!(error = %e, "cannot install SIGTERM handler; stop will be abrupt");
            return std::future::pending().await;
        }
    };
    let mut int = match signal(SignalKind::interrupt()) {
        Ok(s) => s,
        Err(e) => {
            tracing::warn!(error = %e, "cannot install SIGINT handler");
            term.recv().await;
            return;
        }
    };
    tokio::select! {
        _ = term.recv() => {}
        _ = int.recv() => {}
    }
}
