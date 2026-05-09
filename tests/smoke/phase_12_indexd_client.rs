//! Phase 12 smoke — `sourcerer-indexd` IndexdService round-trip via in-memory
//! duplex (sub-second integration test that doesn't touch the OS).
//!
//! The test asserts every Phase 12 method actually returns the right
//! shape, and that `query.run` streams `query:batch` / `query:done`
//! notifications back to the client.
//!
//! Run with `cargo test -p sourcerer-indexd --test phase_12_indexd_client`.

#![cfg(test)]

use std::sync::Arc;

use serde_json::json;
use sourcerer_indexd::{DaemonOptions, DaemonState, IndexdService};
use sourcerer_rpc::ClientHandle;

#[tokio::test]
async fn query_run_streams_batches_and_done() {
    let tmp = tempfile::TempDir::new().unwrap();
    let opts = DaemonOptions {
        index_root: Some(tmp.path().join("idx")),
        ..Default::default()
    };
    let state = DaemonState::open(opts).unwrap();
    let svc = Arc::new(IndexdService::new(state));

    let (a, b) = tokio::io::duplex(64 * 1024);
    let _server = tokio::spawn(async move {
        sourcerer_rpc::server::handle_connection_for_tests(a, svc).await
    });
    let client = ClientHandle::from_stream(b);
    let mut notif = client.notifications();

    let handle: sourcerer_rpc::QueryRunHandle = client
        .call("query.run", json!({ "source": "demo" }))
        .await
        .unwrap();
    assert!(handle.handle.starts_with('h'));

    let mut batches = 0;
    let mut got_done = false;
    let timeout = tokio::time::sleep(std::time::Duration::from_secs(5));
    tokio::pin!(timeout);
    while !got_done {
        tokio::select! {
            _ = &mut timeout => panic!("did not see query:done within timeout"),
            n = notif.next() => {
                let n = n.expect("notification stream closed");
                if n.method == "query:batch" { batches += 1; }
                if n.method == "query:done" { got_done = true; }
            }
        }
    }
    assert!(batches >= 4, "expected at least one batch per lens, got {batches}");
}

#[tokio::test]
async fn index_state_returns_typed_view() {
    let tmp = tempfile::TempDir::new().unwrap();
    let opts = DaemonOptions {
        index_root: Some(tmp.path().join("idx")),
        ..Default::default()
    };
    let state = DaemonState::open(opts).unwrap();
    let svc = Arc::new(IndexdService::new(state));

    let (a, b) = tokio::io::duplex(64 * 1024);
    let _s = tokio::spawn(async move {
        sourcerer_rpc::server::handle_connection_for_tests(a, svc).await
    });
    let client = ClientHandle::from_stream(b);
    let st: sourcerer_rpc::IndexState =
        client.call("index.state", serde_json::Value::Null).await.unwrap();
    // A fresh tempdir Index has zero files but the call shape must
    // survive byte-stable JSON serialization.
    assert!(st.message.contains("Indexed"));
}

#[tokio::test]
async fn extractors_list_and_set_mode_round_trip() {
    let tmp = tempfile::TempDir::new().unwrap();
    let opts = DaemonOptions {
        index_root: Some(tmp.path().join("idx")),
        ..Default::default()
    };
    let state = DaemonState::open(opts).unwrap();
    let svc = Arc::new(IndexdService::new(state));

    let (a, b) = tokio::io::duplex(64 * 1024);
    let _s = tokio::spawn(async move {
        sourcerer_rpc::server::handle_connection_for_tests(a, svc).await
    });
    let client = ClientHandle::from_stream(b);

    let list: Vec<sourcerer_rpc::ExtractorInfo> =
        client.call("extractors.list", serde_json::Value::Null).await.unwrap();
    assert!(list.iter().any(|e| e.id == "pdf"));

    let _: serde_json::Value = client
        .call(
            "extractors.set_mode",
            json!({ "id": "pdf", "mode": "eager" }),
        )
        .await
        .unwrap();
}

#[tokio::test]
async fn excludes_round_trip() {
    let tmp = tempfile::TempDir::new().unwrap();
    let opts = DaemonOptions {
        index_root: Some(tmp.path().join("idx")),
        ..Default::default()
    };
    let state = DaemonState::open(opts).unwrap();
    let svc = Arc::new(IndexdService::new(state));

    let (a, b) = tokio::io::duplex(64 * 1024);
    let _s = tokio::spawn(async move {
        sourcerer_rpc::server::handle_connection_for_tests(a, svc).await
    });
    let client = ClientHandle::from_stream(b);

    let cur: sourcerer_rpc::ExcludeRules =
        client.call("excludes.get", serde_json::Value::Null).await.unwrap();
    assert!(cur.list_enabled);

    let new = sourcerer_rpc::ExcludeRules {
        exclude_hidden: true,
        ..cur
    };
    let _: serde_json::Value = client
        .call("excludes.set", serde_json::to_value(new).unwrap())
        .await
        .unwrap();
    let after: sourcerer_rpc::ExcludeRules =
        client.call("excludes.get", serde_json::Value::Null).await.unwrap();
    assert!(after.exclude_hidden);
}

#[tokio::test]
async fn no_canned_rs_in_tree() {
    // Phase-12 contract: the Phase-11 mock layer is gone. The smoke
    // test asserts that `apps/sourcerer-ui/src-tauri/src/commands/`
    // contains no `canned.rs` file.
    let here = std::path::Path::new(env!("CARGO_MANIFEST_DIR"));
    // Walk up to the workspace root and check the UI side.
    let workspace = here
        .ancestors()
        .find(|p| p.join("Cargo.lock").exists())
        .expect("could not find workspace root from CARGO_MANIFEST_DIR");
    let canned = workspace
        .join("apps/sourcerer-ui/src-tauri/src/commands/canned.rs");
    assert!(
        !canned.exists(),
        "Phase 11 mock layer must be removed; found: {}",
        canned.display()
    );
}
