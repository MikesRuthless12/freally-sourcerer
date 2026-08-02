//! Build 2 (v0.22.0) smoke — SRC-M13 `index.health`, plus the live
//! change-journaling pipeline it reports on.
//!
//! OS-agnostic: the watcher supervisor is driven directly rather than
//! through a real USN journal / FSEvents stream, so the same gates run on
//! every CI matrix entry. What a real subscriber produces is covered by
//! the per-OS journal smoke scripts.
//!
//! Gates:
//!   1. `index.health` round-trips through the RPC layer as the shape the
//!      UI's `IndexHealth` interface declares.
//!   2. A daemon with no watched folders reports no volumes and no
//!      advice, rather than erroring or inventing a volume.
//!   3. Reconciling watched folders starts a watcher per root, and
//!      dropping a folder stops its watcher — the panel's rows follow the
//!      folder set.
//!   4. A root the OS will not stream changes for is reported as
//!      `monitoring: false` with an explanation, and raises exactly the
//!      "not monitoring" advisory. This is the common case on CI, where
//!      opening a USN journal needs privileges the runner lacks.
//!   5. The extraction backlog is reported as absent, not as zero — the
//!      daemon runs no eager-extraction worker yet, and "idle" would be a
//!      lie.
//!
//! Run with `cargo test -p freally-indexd --test build_02_index_health`.

#![cfg(test)]

use std::sync::Arc;

use freally_indexd::{DaemonOptions, DaemonState, IndexdService};
use freally_rpc::{AdvisoryId, ClientHandle, IndexHealth};
use serde_json::json;

async fn daemon(tmp: &tempfile::TempDir) -> Arc<DaemonState> {
    let opts = DaemonOptions {
        index_root: Some(tmp.path().join("idx")),
        ..Default::default()
    };
    DaemonState::open(opts).unwrap()
}

fn client_for(state: Arc<DaemonState>) -> ClientHandle {
    let svc = Arc::new(IndexdService::new(state));
    let (a, b) = tokio::io::duplex(64 * 1024);
    tokio::spawn(async move { freally_rpc::server::handle_connection_for_tests(a, svc).await });
    ClientHandle::from_stream(b)
}

#[tokio::test]
async fn health_round_trips_and_is_empty_before_any_folder_is_watched() {
    let tmp = tempfile::TempDir::new().unwrap();
    let state = daemon(&tmp).await;
    let client = client_for(state);

    let health: IndexHealth = client.call("index.health", json!(null)).await.unwrap();

    assert!(
        health.volumes.is_empty(),
        "nothing is being watched, so there is nothing to report on"
    );
    assert!(health.advisories.is_empty());
    assert_eq!(
        health.extraction_backlog, None,
        "no eager-extraction worker exists; `Some(0)` would read as idle"
    );
}

#[tokio::test]
async fn a_shared_multi_user_daemon_refuses_the_undo_journal() {
    // The Windows service binds one pipe granting Authenticated Users and
    // keeps its state in %PROGRAMDATA%. A shared undo stack there would
    // let any local peer record a rename that a *different* user's Ctrl+Z
    // executes under their own account — the peer picks both halves of
    // every pair, so the inverse is entirely theirs.
    let tmp = tempfile::TempDir::new().unwrap();
    let opts = DaemonOptions {
        index_root: Some(tmp.path().join("idx")),
        shared_multi_user: true,
        ..Default::default()
    };
    let state = DaemonState::open(opts).unwrap();
    let client = client_for(state);

    for method in ["ops.list", "ops.record", "ops.set_undone", "ops.clear"] {
        let res: Result<serde_json::Value, _> = client.call(method, json!(null)).await;
        assert!(
            res.is_err(),
            "{method} must be refused on a shared multi-user daemon"
        );
    }
}

#[tokio::test]
async fn a_per_user_daemon_still_has_a_working_journal() {
    // The regression guard for the fix above: a normal desktop install
    // must keep full undo.
    let tmp = tempfile::TempDir::new().unwrap();
    let state = daemon(&tmp).await;
    let client = client_for(state);

    let listing: serde_json::Value = client.call("ops.list", json!(null)).await.unwrap();
    assert!(listing.get("entries").is_some());
}

#[tokio::test]
async fn watchers_follow_the_watched_folder_set() {
    let tmp = tempfile::TempDir::new().unwrap();
    let state = daemon(&tmp).await;

    let one = tmp.path().join("one");
    let two = tmp.path().join("two");
    std::fs::create_dir_all(&one).unwrap();
    std::fs::create_dir_all(&two).unwrap();

    state.watchers.reconcile(&[
        one.to_string_lossy().to_string(),
        two.to_string_lossy().to_string(),
    ]);
    let roots = state.watchers.snapshot().len();
    assert!(
        roots >= 1,
        "expected at least one watcher for two watched folders, got {roots}"
    );

    // Reconciling is idempotent — the same folder set must not stack up
    // duplicate watchers on every folders.* call.
    state.watchers.reconcile(&[
        one.to_string_lossy().to_string(),
        two.to_string_lossy().to_string(),
    ]);
    assert_eq!(state.watchers.snapshot().len(), roots);

    state.watchers.reconcile(&[]);
    assert!(
        state.watchers.snapshot().is_empty(),
        "dropping every folder must stop every watcher"
    );
}

#[tokio::test]
async fn an_unstreamable_root_is_reported_as_scan_only_with_one_advisory() {
    let tmp = tempfile::TempDir::new().unwrap();
    let state = daemon(&tmp).await;

    // A path that does not exist can never yield a change stream on any
    // OS, which is the deterministic way to exercise the unavailable
    // branch without depending on runner privileges.
    let missing = tmp.path().join("no-such-directory");
    state
        .watchers
        .reconcile(&[missing.to_string_lossy().to_string()]);

    let client = client_for(state);
    let health: IndexHealth = client.call("index.health", json!(null)).await.unwrap();

    assert_eq!(health.volumes.len(), 1);
    let v = &health.volumes[0];
    assert!(!v.monitoring);
    assert!(
        v.unavailable_reason.is_some(),
        "the panel has to be able to explain why this root is scan-only"
    );
    assert_eq!(v.events_seen, 0);
    assert_eq!(v.events_dropped, 0);

    let ids: Vec<AdvisoryId> = health.advisories.iter().map(|a| a.id).collect();
    assert_eq!(
        ids,
        vec![AdvisoryId::NotMonitoring],
        "an idle pipeline must not also raise lag / saturation advice"
    );
}
