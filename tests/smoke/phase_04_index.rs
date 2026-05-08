//! Phase 4 smoke — OS-agnostic, runs on every CI matrix entry.
//!
//! Asserts the four invariants the Build Guide names for `sourcerer-index`:
//!
//!   1. `Index::open` materializes the per-OS directory tree.
//!   2. `apply()` round-trips Create / Modify / Delete / Rename / AttrChange
//!      across Tantivy, SQLite, and the custom name index.
//!   3. A simulated `kill -9` between commits leaves the on-disk state
//!      recoverable — re-opening rebuilds the name index from SQLite.
//!   4. The bounded queue surfaces back-pressure as `QueueFull` rather
//!      than dropping events (the contract Phases 1–3 promised the
//!      journal subscribers).

use std::path::PathBuf;

use sourcerer_index::{EventQueue, Index, IndexError};
use sourcerer_journal::JournalEvent;
use tempfile::tempdir;

fn create(path: &str) -> JournalEvent {
    JournalEvent::Create {
        path: PathBuf::from(path),
        size: 1024,
        mtime_ns: 1_700_000_000_000_000_000,
        ctime_ns: 1_700_000_000_000_000_000,
        attrs: 0,
    }
}

#[test]
fn open_materializes_full_directory_layout() {
    let dir = tempdir().unwrap();
    let _idx = Index::open(dir.path()).expect("Index::open");
    for sub in ["index.tantivy", "extracted"] {
        let p = dir.path().join(sub);
        assert!(p.exists(), "missing {}", p.display());
    }
    assert!(dir.path().join("files.db").exists());
}

#[test]
fn round_trip_create_modify_rename_delete_attrchange() {
    let dir = tempdir().unwrap();
    let idx = Index::open(dir.path()).unwrap();

    // Create three files.
    idx.apply(&[
        create("/tmp/sourcerer-smoke/alpha.md"),
        create("/tmp/sourcerer-smoke/beta.md"),
        create("/tmp/sourcerer-smoke/gamma.md"),
    ])
    .unwrap();
    idx.commit().unwrap();
    assert_eq!(idx.store().count().unwrap(), 3);
    let alpha_hits = idx.search_name("alpha", 16).unwrap();
    assert_eq!(alpha_hits.len(), 1);

    // Modify alpha — same path, new size.
    let alpha = PathBuf::from("/tmp/sourcerer-smoke/alpha.md");
    idx.apply(&[JournalEvent::Modify {
        path: alpha.clone(),
        size: 4096,
        mtime_ns: 1_710_000_000_000_000_000,
        attrs: 0,
    }])
    .unwrap();
    idx.commit().unwrap();
    assert_eq!(idx.store().count().unwrap(), 3);
    // Tantivy still finds alpha.
    let still_there = idx.search_name("alpha", 16).unwrap();
    assert_eq!(still_there.len(), 1);

    // AttrChange on beta.
    idx.apply(&[JournalEvent::AttrChange {
        path: PathBuf::from("/tmp/sourcerer-smoke/beta.md"),
        attrs: 0x80,
    }])
    .unwrap();
    idx.commit().unwrap();

    // Rename gamma → delta.
    idx.apply(&[JournalEvent::Rename {
        old_path: PathBuf::from("/tmp/sourcerer-smoke/gamma.md"),
        new_path: PathBuf::from("/tmp/sourcerer-smoke/delta.md"),
    }])
    .unwrap();
    idx.commit().unwrap();
    let gamma_hits = idx.search_name("gamma", 16).unwrap();
    assert!(
        gamma_hits.is_empty(),
        "rename should retire `gamma`: {gamma_hits:?}"
    );
    let delta_hits = idx.search_name("delta", 16).unwrap();
    assert_eq!(delta_hits.len(), 1);

    // Delete alpha.
    idx.apply(&[JournalEvent::Delete {
        path: alpha.clone(),
    }])
    .unwrap();
    idx.commit().unwrap();
    assert_eq!(idx.store().count().unwrap(), 2);
}

#[test]
fn recovery_from_simulated_kill_dash_9() {
    let dir = tempdir().unwrap();
    let p = PathBuf::from("/tmp/sourcerer-smoke/canary-recovery.dat");
    {
        let idx = Index::open(dir.path()).unwrap();
        idx.apply(&[create(p.to_str().unwrap())]).unwrap();
        idx.commit().unwrap();
        // Drop without explicit shutdown — kill -9 stand-in.
    }
    // Wipe the name-index files; SQLite WAL is the canonical store.
    for f in ["name.idx", "name.suf"] {
        let _ = std::fs::remove_file(dir.path().join(f));
    }
    let idx2 = Index::open(dir.path()).unwrap();
    let cands = idx2.name_index().candidates("canary");
    assert!(
        !cands.is_empty(),
        "post-recovery candidates should include canary; got {cands:?}"
    );
    assert_eq!(idx2.store().count().unwrap(), 1);
}

#[test]
fn manifest_persists_volume_cursor_across_commits() {
    let dir = tempdir().unwrap();
    {
        let idx = Index::open(dir.path()).unwrap();
        idx.apply(&[create("/tmp/sourcerer-smoke/cursor-witness.txt")])
            .unwrap();
        idx.record_cursor("C:", "USN-12345");
        idx.commit().unwrap();
    }
    let manifest_bytes = std::fs::read(dir.path().join("manifest.json")).unwrap();
    let manifest: serde_json::Value = serde_json::from_slice(&manifest_bytes).unwrap();
    assert_eq!(manifest["volume_cursors"]["C:"], "USN-12345");
    assert_eq!(manifest["tantivy_generation"], 1);
}

#[test]
fn back_pressure_signals_full_queue() {
    let q = EventQueue::new(4);
    for i in 0..4 {
        q.try_push(create(&format!("/tmp/q{i}"))).unwrap();
    }
    let err = q.try_push(create("/tmp/overflow")).unwrap_err();
    assert!(matches!(err, IndexError::QueueFull { capacity: 4, .. }));

    // Drain frees capacity for subsequent producers.
    let drained = q.drain(2);
    assert_eq!(drained.len(), 2);
    q.try_push(create("/tmp/post-drain")).unwrap();
}

#[test]
fn back_to_back_creates_dedupe_in_tantivy() {
    let dir = tempdir().unwrap();
    let idx = Index::open(dir.path()).unwrap();
    // Same path created twice — common after recovery replays.
    let p = "/tmp/sourcerer-smoke/dupe.md";
    idx.apply(&[create(p), create(p)]).unwrap();
    idx.commit().unwrap();
    let hits = idx.search_name("dupe", 16).unwrap();
    assert_eq!(
        hits.len(),
        1,
        "Tantivy delete-then-add should keep only one row per file_id"
    );
    assert_eq!(idx.store().count().unwrap(), 1);
}

#[test]
fn modify_before_create_synthesizes_create() {
    // The journal subscriber attaches mid-stream — the first event for
    // a given path can be Modify rather than Create. The index must
    // synthesize a Create rather than silently dropping the row.
    let dir = tempdir().unwrap();
    let idx = Index::open(dir.path()).unwrap();
    let path = PathBuf::from("/tmp/sourcerer-smoke/mid-stream.bin");
    idx.apply(&[JournalEvent::Modify {
        path: path.clone(),
        size: 256,
        mtime_ns: 1_700_000_000_000_000_000,
        attrs: 0,
    }])
    .unwrap();
    idx.commit().unwrap();
    assert_eq!(idx.store().count().unwrap(), 1);
    let hits = idx.search_name("mid-stream", 16).unwrap();
    assert_eq!(hits.len(), 1);
}

#[test]
fn rename_of_unknown_path_degrades_to_delete_plus_create() {
    // Cross-batch rename pairs that the journal subscribers can't pair
    // up surface as a Rename whose old_path was never indexed. The
    // index's contract is to coerce that into a Delete-of-old +
    // Create-of-new so the canonical store, Tantivy, and the name
    // index never disagree.
    let dir = tempdir().unwrap();
    let idx = Index::open(dir.path()).unwrap();
    let unknown_old = PathBuf::from("/tmp/sourcerer-smoke/no-prior.txt");
    let new_path = PathBuf::from("/tmp/sourcerer-smoke/now-known.txt");
    idx.apply(&[JournalEvent::Rename {
        old_path: unknown_old.clone(),
        new_path: new_path.clone(),
    }])
    .unwrap();
    idx.commit().unwrap();
    assert_eq!(idx.store().count().unwrap(), 1);
    let hits = idx.search_name("now-known", 16).unwrap();
    assert_eq!(hits.len(), 1);
    let stale = idx.search_name("no-prior", 16).unwrap();
    assert!(stale.is_empty(), "old half should not leak: {stale:?}");
}

#[test]
fn close_unblocks_push_blocking_and_wait_for_events() {
    use std::sync::Arc;
    use std::thread;
    use std::time::{Duration, Instant};

    let q = EventQueue::new(2);
    // Fill to capacity so push_blocking has to wait.
    q.try_push(create("/tmp/full-1")).unwrap();
    q.try_push(create("/tmp/full-2")).unwrap();

    let q_for_producer = q.clone();
    let producer =
        thread::spawn(move || q_for_producer.push_blocking(create("/tmp/blocked")).err());

    // Wait long enough for the producer to land in the wait state.
    thread::sleep(Duration::from_millis(50));
    q.close();
    let started = Instant::now();
    let err = producer
        .join()
        .expect("producer thread panicked")
        .expect("push_blocking should unblock as QueueFull on close");
    assert!(matches!(err, IndexError::QueueFull { .. }));
    assert!(
        started.elapsed() < Duration::from_secs(2),
        "close should wake push_blocking promptly",
    );

    // wait_for_events on a closed-and-then-drained queue returns false
    // rather than blocking forever.
    let q2 = Arc::new(EventQueue::new(2));
    q2.close();
    let _ = q2.drain(2);
    assert!(!q2.wait_for_events(Duration::from_millis(200)));
}

#[test]
fn try_push_after_close_refuses() {
    let q = EventQueue::new(2);
    q.close();
    let err = q.try_push(create("/tmp/post-close")).unwrap_err();
    assert!(matches!(err, IndexError::QueueFull { .. }));
}

#[test]
fn torn_manifest_does_not_block_open() {
    // A half-flushed manifest.json (anything that fails JSON parse)
    // must be treated as missing and re-derived rather than blocking
    // Index::open. SQLite + Tantivy meta are the canonical state.
    let dir = tempdir().unwrap();
    {
        let idx = Index::open(dir.path()).unwrap();
        idx.apply(&[create("/tmp/sourcerer-smoke/torn-canary.dat")])
            .unwrap();
        idx.commit().unwrap();
    }
    // Truncate the manifest mid-byte.
    let manifest = dir.path().join("manifest.json");
    std::fs::write(&manifest, b"{ \"version\": 1, \"applied_eve").unwrap();
    let idx = Index::open(dir.path()).expect("Index::open must recover from torn manifest");
    assert_eq!(idx.store().count().unwrap(), 1);
}
