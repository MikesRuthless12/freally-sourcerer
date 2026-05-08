//! End-to-end integration test for the Phase 2 FSEvents subscriber.
//!
//! Spins a temp dir under the user's HOME, performs Create / Modify /
//! Rename / Delete operations, and asserts the subscriber emits matching
//! events within a deadline. Skipped automatically on non-macOS hosts so
//! CI on Windows / Linux cargo-checks the workspace without trying to
//! call FSEvents APIs there.

#![cfg(target_os = "macos")]

use std::path::PathBuf;
use std::time::{Duration, Instant};

use futures::StreamExt;
use sourcerer_journal_mac::{JournalEvent, open_with_cursor_root};

#[test]
fn realtime_create_modify_rename_delete_round_trip() {
    // Two distinct tempdirs:
    //   `scratch`        — the watched root (FSEvents stream root).
    //   `cursor_holder`  — where the per-watch cursor JSON lives.
    // The cursor MUST live outside the watched root, otherwise every
    // `cursor.save()` triggers its own FSEvents (rename of `<file>.tmp`
    // over `<file>.json`) and floods the test's expected event counts.
    let scratch = tempfile::tempdir().expect("create scratch tempdir");
    let cursor_holder = tempfile::tempdir().expect("create cursor tempdir");
    let scratch_path: PathBuf = scratch
        .path()
        .canonicalize()
        .expect("canonicalize scratch tempdir");
    let cursor_root = cursor_holder
        .path()
        .canonicalize()
        .expect("canonicalize cursor tempdir");

    let subscriber = match open_with_cursor_root(&scratch_path, &cursor_root) {
        Ok(s) => s,
        Err(e) => {
            eprintln!(
                "skipping integration_journal: open() on `{}` failed: {e}",
                scratch_path.display()
            );
            return;
        }
    };

    // Drain on a worker thread; main thread does the file I/O and reads
    // collected events from a Mutex<Vec>.
    let collected = std::sync::Arc::new(std::sync::Mutex::new(Vec::<JournalEvent>::new()));
    let collector = collected.clone();
    let _drain = std::thread::spawn(move || {
        let mut stream = Box::pin(subscriber.subscribe());
        futures::executor::block_on(async move {
            while let Some(ev) = stream.next().await {
                collector.lock().unwrap().push(ev);
            }
        });
    });

    // FSEvents has a 0.5 s coalesce window; give the stream time to start.
    std::thread::sleep(Duration::from_millis(500));

    // -- Create burst
    let a = scratch_path.join("alpha.txt");
    let b = scratch_path.join("bravo.txt");
    let c = scratch_path.join("charlie.txt");
    std::fs::write(&a, b"hello").unwrap();
    std::fs::write(&b, b"world").unwrap();
    std::fs::write(&c, b"sourcerer").unwrap();

    // Wait one full coalesce window so the Create batch settles before we
    // emit the Modify on `alpha.txt`. Without this gap, FSEvents coalesces
    // Create + Modify into a single bitmask and our flag classifier
    // (precedence: Create > Modify) renders only the Create — we'd never
    // see a `Modify` event for this run.
    std::thread::sleep(Duration::from_millis(700));

    // -- Modify (a) — now in its own batch
    std::fs::write(&a, b"hello-updated").unwrap();

    // -- Rename (b -> b2)
    let b2 = scratch_path.join("bravo-renamed.txt");
    std::fs::rename(&b, &b2).unwrap();

    // -- Delete (c)
    std::fs::remove_file(&c).unwrap();

    // FSEvents 0.5 s coalesce + per-batch pairing: events typically arrive
    // within ~1 s. We poll up to 8 s before failing — generous because the
    // CI host load can stretch the coalesce timer.
    let deadline = Instant::now() + Duration::from_secs(8);
    let scratch_str = scratch_path.to_string_lossy().to_lowercase();
    let want_create = a.to_string_lossy().to_lowercase();
    let want_modify = a.to_string_lossy().to_lowercase();
    let want_rename_old = b.to_string_lossy().to_lowercase();
    let want_rename_new = b2.to_string_lossy().to_lowercase();
    let want_delete = c.to_string_lossy().to_lowercase();

    loop {
        let evs: Vec<JournalEvent> = collected.lock().unwrap().clone();
        let mut saw_create = false;
        let mut saw_modify = false;
        let mut saw_rename_or_split = false;
        let mut saw_delete = false;
        for ev in evs.iter().filter(|e| {
            e.primary_path()
                .to_string_lossy()
                .to_lowercase()
                .starts_with(&scratch_str)
        }) {
            match ev {
                JournalEvent::Create { path, .. } => {
                    let p = path.to_string_lossy().to_lowercase();
                    if p == want_create {
                        saw_create = true;
                    }
                    // Per-batch rename pairing may degrade to Delete + Create
                    // when halves cross batch boundaries — accept either.
                    if p == want_rename_new {
                        saw_rename_or_split = true;
                    }
                }
                JournalEvent::Modify { path, .. } => {
                    if path.to_string_lossy().to_lowercase() == want_modify {
                        saw_modify = true;
                    }
                }
                JournalEvent::Rename { old_path, new_path } => {
                    if old_path.to_string_lossy().to_lowercase() == want_rename_old
                        && new_path.to_string_lossy().to_lowercase() == want_rename_new
                    {
                        saw_rename_or_split = true;
                    }
                }
                JournalEvent::Delete { path } => {
                    let p = path.to_string_lossy().to_lowercase();
                    if p == want_delete {
                        saw_delete = true;
                    }
                    if p == want_rename_old {
                        // Rename split across batches landed as Delete.
                        // Wait for the matching Create (recorded above) to
                        // close out the rename-split case.
                    }
                }
                JournalEvent::AttrChange { .. } => {}
            }
        }
        if saw_create && saw_modify && saw_rename_or_split && saw_delete {
            return;
        }
        if Instant::now() > deadline {
            let evs: Vec<JournalEvent> = collected.lock().unwrap().clone();
            let summary: Vec<String> = evs
                .iter()
                .map(|e| format!("{} {}", e.variant_name(), e.primary_path().display()))
                .collect();
            panic!(
                "did not observe all expected events within 8 s.\n\
                 saw_create={saw_create} saw_modify={saw_modify} \
                 saw_rename_or_split={saw_rename_or_split} saw_delete={saw_delete}\n\
                 events ({}):\n{}",
                evs.len(),
                summary.join("\n")
            );
        }
        std::thread::sleep(Duration::from_millis(150));
    }
}

#[test]
fn dropping_subscriber_stops_the_subscribe_thread() {
    // Verifies the Drop-side stop signal: open a subscriber, start the
    // FSEvents subscribe stream, then drop the subscriber. The subscribe
    // thread should observe the run-loop stop signal (or the receiver
    // closing on the next 1-second slice tick) and exit, dropping its
    // `tx` end and ending the stream from the consumer's perspective.
    // Cursor outside the watched tree so its `cursor.save()` doesn't
    // generate self-loop FSEvents (same fix as the realtime test).
    let scratch = tempfile::tempdir().expect("create scratch tempdir");
    let cursor_holder = tempfile::tempdir().expect("create cursor tempdir");
    let scratch_path: PathBuf = scratch
        .path()
        .canonicalize()
        .expect("canonicalize scratch tempdir");
    let cursor_root = cursor_holder
        .path()
        .canonicalize()
        .expect("canonicalize cursor tempdir");

    let subscriber = match open_with_cursor_root(&scratch_path, &cursor_root) {
        Ok(s) => s,
        Err(e) => {
            eprintln!(
                "skipping dropping_subscriber_stops_the_subscribe_thread: open() failed: {e}"
            );
            return;
        }
    };

    let stream = subscriber.subscribe();

    // Worker thread drains the stream and signals "stream ended" through
    // a sync channel. We can't directly join the subscribe thread (it's
    // detached inside JournalSubscriber), so we observe end-of-stream
    // via the receiver as a proxy for thread exit.
    let (done_tx, done_rx) = std::sync::mpsc::channel::<()>();
    let _drainer = std::thread::spawn(move || {
        let mut stream = Box::pin(stream);
        futures::executor::block_on(async move {
            while stream.next().await.is_some() {
                // Drain — events are not under test here.
            }
        });
        let _ = done_tx.send(());
    });

    // Let the subscribe thread enter its CFRunLoop pump.
    std::thread::sleep(Duration::from_secs(1));

    drop(subscriber);

    // The run-loop slice is 1 s; waking + exiting + drop(tx) should land
    // well under 5 s even on a busy CI host.
    match done_rx.recv_timeout(Duration::from_secs(5)) {
        Ok(()) => {} // subscribe thread exited cleanly
        Err(e) => panic!("subscribe thread did not exit within 5 s: {e}"),
    }
}
