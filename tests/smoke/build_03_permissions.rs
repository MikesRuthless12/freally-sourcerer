//! Build 3 (v0.23.0) smoke — SRC-M21 permission health, wiring half.
//!
//! The ledger's own rules are unit-tested in
//! `crates/freally-indexd/src/permissions.rs`. What this proves is that
//! the scanner is actually holding it: that a real `scan_folder` clears
//! the previous pass's findings for the root it is about to re-walk, and
//! that a clean tree leaves the ledger empty.
//!
//! Deliberately not asserting on a *denied* directory: creating one
//! portably is not possible — `chmod 000` is a no-op for root on Linux
//! and meaningless on Windows, where it would take ACL surgery. That
//! path is covered by the unit tests, which construct the `io::Error`
//! directly rather than asking the OS to produce one.

use std::path::PathBuf;
use std::sync::{Arc, Mutex};

use freally_index::Index;
use freally_indexd::permissions::PermissionLedger;
use freally_indexd::scanner::scan_folder;

fn temp_tree() -> tempfile::TempDir {
    let dir = tempfile::tempdir().unwrap();
    std::fs::create_dir_all(dir.path().join("docs")).unwrap();
    std::fs::write(dir.path().join("docs/report.txt"), b"hello").unwrap();
    std::fs::write(dir.path().join("notes.md"), b"world").unwrap();
    dir
}

fn index_at(dir: &std::path::Path) -> Arc<Index> {
    Index::open(dir).unwrap()
}

#[test]
fn a_readable_tree_leaves_the_ledger_empty() {
    let tree = temp_tree();
    let idx_dir = tempfile::tempdir().unwrap();
    let ledger = Arc::new(Mutex::new(PermissionLedger::default()));

    let n = scan_folder(
        index_at(idx_dir.path()),
        tree.path().to_path_buf(),
        Some(ledger.clone()),
    )
    .unwrap();

    assert!(n >= 3, "expected the tree to be indexed, got {n} entries");
    let l = ledger.lock().unwrap();
    assert!(
        l.is_empty(),
        "a readable tree must not report anything as skipped: {:?}",
        l.entries()
    );
}

#[test]
fn a_rescan_clears_the_previous_findings_for_that_root() {
    // The failure this guards against is a report that only ever grows:
    // fix the permission, rescan, and the folder is still listed as
    // unreadable because nobody dropped the stale entry.
    let tree = temp_tree();
    let idx_dir = tempfile::tempdir().unwrap();
    let ledger = Arc::new(Mutex::new(PermissionLedger::default()));

    let stale = tree.path().join("docs");
    let other_root = PathBuf::from("/somewhere/else");
    {
        let mut l = ledger.lock().unwrap();
        let denied = std::io::Error::new(std::io::ErrorKind::PermissionDenied, "Access is denied");
        l.record(&stale, &denied, "old");
        l.record(&other_root, &denied, "old");
        assert_eq!(l.entries().len(), 2);
    }

    scan_folder(
        index_at(idx_dir.path()),
        tree.path().to_path_buf(),
        Some(ledger.clone()),
    )
    .unwrap();

    let l = ledger.lock().unwrap();
    assert_eq!(
        l.entries().len(),
        1,
        "only the rescanned root's entries should be dropped"
    );
    assert_eq!(l.entries()[0].path, other_root);
}

#[test]
fn scanning_without_a_ledger_still_works() {
    // The MFT path and the smoke harness both call in with `None`.
    let tree = temp_tree();
    let idx_dir = tempfile::tempdir().unwrap();
    let n = scan_folder(index_at(idx_dir.path()), tree.path().to_path_buf(), None).unwrap();
    assert!(n >= 3);
}
