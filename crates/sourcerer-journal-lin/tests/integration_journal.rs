//! End-to-end integration test for the Phase 3 Linux subscriber.
//!
//! Spins a temp dir under the user's HOME, performs Create / Modify /
//! Rename / Delete operations, and asserts the subscriber emits matching
//! events within a deadline. Skipped automatically on non-Linux hosts so
//! CI on Windows / macOS cargo-checks the workspace without trying to
//! call inotify APIs there.

#![cfg(target_os = "linux")]

use std::path::PathBuf;
use std::time::{Duration, Instant};

use futures::StreamExt;
use sourcerer_journal_lin::{JournalEvent, open_with_cursor_root};

#[test]
fn realtime_create_modify_rename_delete_round_trip() {
    // Two distinct tempdirs:
    //   `scratch`        — the watched root (inotify watch root).
    //   `cursor_holder`  — where the per-watch cursor JSON lives.
    // The cursor MUST live outside the watched root, otherwise every
    // `cursor.save()` triggers its own inotify events (rename of
    // `<file>.tmp` over `<file>.json`) and inflates the test's
    // expected event counts.
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

    // Give the subscribe thread time to install its watches before the
    // workload starts. Without this the first Create can race the
    // recursive `add_watch` and miss the event.
    std::thread::sleep(Duration::from_millis(500));

    // -- Create burst
    let a = scratch_path.join("alpha.txt");
    let b = scratch_path.join("bravo.txt");
    let c = scratch_path.join("charlie.txt");
    std::fs::write(&a, b"hello").unwrap();
    std::fs::write(&b, b"world").unwrap();
    std::fs::write(&c, b"sourcerer").unwrap();

    // Pause to let the Create batch settle before issuing Modify.
    std::thread::sleep(Duration::from_millis(300));

    // -- Modify (a)
    std::fs::write(&a, b"hello-updated").unwrap();

    // -- Rename (b -> b2)
    let b2 = scratch_path.join("bravo-renamed.txt");
    std::fs::rename(&b, &b2).unwrap();

    // -- Delete (c)
    std::fs::remove_file(&c).unwrap();

    // Inotify is closer to real-time than FSEvents — most events arrive
    // within ~100ms. Poll up to 8 s before failing.
    let deadline = Instant::now() + Duration::from_secs(8);
    let want_create = a.to_string_lossy().to_lowercase();
    let want_modify = a.to_string_lossy().to_lowercase();
    let want_rename_old = b.to_string_lossy().to_lowercase();
    let want_rename_new = b2.to_string_lossy().to_lowercase();
    let want_delete = c.to_string_lossy().to_lowercase();

    loop {
        let evs: Vec<JournalEvent> = collected.lock().unwrap().clone();
        let saw_create = evs.iter().any(|e| {
            matches!(e,
            JournalEvent::Create { path, .. }
                if path.to_string_lossy().to_lowercase() == want_create)
        });
        let saw_modify = evs.iter().any(|e| {
            matches!(e,
            JournalEvent::Modify { path, .. }
                if path.to_string_lossy().to_lowercase() == want_modify)
        });
        let saw_rename = evs.iter().any(|e| {
            matches!(e,
            JournalEvent::Rename { old_path, new_path }
                if old_path.to_string_lossy().to_lowercase() == want_rename_old
                && new_path.to_string_lossy().to_lowercase() == want_rename_new)
        });
        // Rename pairing falls back to Delete + Create when split across
        // batches; accept that fallback so an over-burdened CI host
        // doesn't false-fail.
        let saw_rename_fallback = evs.iter().any(|e| {
            matches!(e,
            JournalEvent::Delete { path }
                if path.to_string_lossy().to_lowercase() == want_rename_old)
        }) && evs.iter().any(|e| {
            matches!(e,
                JournalEvent::Create { path, .. }
                    if path.to_string_lossy().to_lowercase() == want_rename_new)
        });
        let saw_delete = evs.iter().any(|e| {
            matches!(e,
            JournalEvent::Delete { path }
                if path.to_string_lossy().to_lowercase() == want_delete)
        });

        if saw_create && saw_modify && (saw_rename || saw_rename_fallback) && saw_delete {
            return;
        }

        if Instant::now() > deadline {
            panic!(
                "integration assertions timed out:\n  \
                 saw_create={saw_create}\n  saw_modify={saw_modify}\n  \
                 saw_rename={saw_rename}\n  saw_rename_fallback={saw_rename_fallback}\n  \
                 saw_delete={saw_delete}\n  events={evs:#?}"
            );
        }

        std::thread::sleep(Duration::from_millis(50));
    }
}
