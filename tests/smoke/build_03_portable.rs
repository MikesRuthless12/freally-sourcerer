//! SRC-M17 smoke — portable mode.
//!
//! Portable mode is defined relative to the *executable*, not the
//! working directory, so these tests stage a copy of the real
//! `freally-indexd` binary in a scratch directory and drive it there.
//! Testing it in place would drop a `portable.flag` into `target/debug`
//! and quietly make every other binary in the workspace portable for the
//! rest of the run.
//!
//! What is proven here:
//!
//! 1. A `portable.flag` beside the binary turns portable mode on with no
//!    command line at all — the USB-stick case, where the user only ever
//!    double-clicks.
//! 2. `--portable` turns it on without a flag file.
//! 3. Either way the daemon refuses to register an OS service. This is
//!    the half of SRC-M17 that is about *not* doing something, and it is
//!    the half that is easy to regress silently.
//! 4. A portable daemon writes its index and its log into `Data/` beside
//!    the binary, and nowhere else.

use std::path::{Path, PathBuf};
use std::process::Command;
use std::time::{Duration, Instant};

const DAEMON: &str = env!("CARGO_BIN_EXE_freally-indexd");

fn staged_daemon(dir: &Path) -> PathBuf {
    let src = PathBuf::from(DAEMON);
    let dst = dir.join(src.file_name().expect("daemon binary has a file name"));
    std::fs::copy(&src, &dst).expect("stage the daemon binary into the scratch dir");
    dst
}

/// The daemon opens its index during boot, so the directory appears
/// within a second or two on a warm machine. The generous ceiling is for
/// a cold CI runner, not for a hang: the assertion below fails on
/// timeout rather than waiting forever.
fn wait_for(path: &Path, limit: Duration) -> bool {
    let start = Instant::now();
    while start.elapsed() < limit {
        if path.exists() {
            return true;
        }
        std::thread::sleep(Duration::from_millis(100));
    }
    false
}

#[test]
fn a_flag_file_beside_the_binary_blocks_service_registration() {
    let tmp = tempfile::tempdir().unwrap();
    let bin = staged_daemon(tmp.path());
    std::fs::write(tmp.path().join("portable.flag"), b"").unwrap();

    let out = Command::new(&bin).arg("install").output().unwrap();

    assert!(
        !out.status.success(),
        "portable install must fail, got success"
    );
    let err = String::from_utf8_lossy(&out.stderr);
    assert!(
        err.contains("portable mode does not register an OS service"),
        "expected the portable refusal, got: {err}"
    );
}

#[test]
fn the_portable_switch_blocks_service_registration_too() {
    let tmp = tempfile::tempdir().unwrap();
    let bin = staged_daemon(tmp.path());
    // No flag file this time — the switch alone must be enough.
    let out = Command::new(&bin)
        .arg("--portable")
        .arg("install")
        .output()
        .unwrap();

    assert!(!out.status.success());
    let err = String::from_utf8_lossy(&out.stderr);
    assert!(
        err.contains("portable mode does not register an OS service"),
        "expected the portable refusal, got: {err}"
    );
}

#[test]
fn uninstall_is_refused_as_well() {
    // A portable install never registered anything, so letting
    // `uninstall` through would deregister the *installed* copy on the
    // host machine — the opposite of leaving no trace.
    let tmp = tempfile::tempdir().unwrap();
    let bin = staged_daemon(tmp.path());
    let out = Command::new(&bin)
        .arg("--portable")
        .arg("uninstall")
        .output()
        .unwrap();

    assert!(!out.status.success());
    assert!(
        String::from_utf8_lossy(&out.stderr)
            .contains("portable mode does not register an OS service")
    );
}

#[test]
fn a_portable_daemon_keeps_its_index_and_log_beside_the_binary() {
    let tmp = tempfile::tempdir().unwrap();
    let bin = staged_daemon(tmp.path());
    std::fs::write(tmp.path().join("portable.flag"), b"").unwrap();

    // An explicit socket keeps this test off the real per-user endpoint
    // even if the layout logic regresses.
    let socket = if cfg!(windows) {
        format!(r"\\.\pipe\freally-smoke-portable-{}", std::process::id())
    } else {
        tmp.path().join("smoke.sock").display().to_string()
    };
    let mut child = Command::new(&bin)
        .arg("run")
        .arg("--socket")
        .arg(&socket)
        .spawn()
        .expect("spawn the portable daemon");

    let data = tmp.path().join("Data");
    let index = data.join("index");
    let log = data.join("logs").join("indexd.log");
    let index_appeared = wait_for(&index, Duration::from_secs(60));
    let log_appeared = wait_for(&log, Duration::from_secs(60));

    let _ = child.kill();
    let _ = child.wait();

    assert!(
        index_appeared,
        "portable daemon did not create {} — it wrote its index somewhere else",
        index.display()
    );
    assert!(
        log_appeared,
        "portable daemon did not create {} — a double-clicked binary has no \
         console, so losing the log means losing every diagnostic",
        log.display()
    );
}
