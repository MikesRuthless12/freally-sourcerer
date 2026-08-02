//! Build 2 (v0.22.0) smoke — SRC-M15 bulk rename + SRC-M16 undo/redo,
//! against real files in a temp directory.
//!
//! The pure engine is unit-tested in `freally_rpc::rename`; this covers
//! the part those tests cannot: that a plan, applied to a real
//! filesystem, moves the files it said it would and nothing else, and
//! that the journal's inverse puts them back exactly.
//!
//! `destination_for` and `perform_moves` are the real functions the Tauri
//! command calls, not copies of them — they live in `freally-rpc` for
//! exactly that reason, since `src-tauri` is excluded from the workspace
//! and anything defined there is unreachable from `cargo test
//! --workspace`.
//!
//! Gates:
//!   1. A clean batch renames every file and leaves the directory with
//!      exactly the expected names.
//!   2. A batch that would overwrite an existing file changes nothing.
//!   3. A failed move rolls the whole batch back.
//!   4. The journal's inverse restores the original names exactly, and
//!      a redo re-applies them.

#![cfg(test)]

use std::collections::BTreeSet;
use std::path::{Path, PathBuf};

use freally_rpc::rename::{RenameRule, destination_for, perform_moves, plan};
use freally_rpc::{OperationEntry, OperationItem, OperationJournal, OperationKind};

fn touch(dir: &Path, name: &str) -> String {
    let p = dir.join(name);
    std::fs::write(&p, b"x").unwrap();
    p.to_string_lossy().into_owned()
}

fn names_in(dir: &Path) -> BTreeSet<String> {
    std::fs::read_dir(dir)
        .unwrap()
        .filter_map(|e| e.ok())
        .map(|e| e.file_name().to_string_lossy().into_owned())
        .collect()
}

fn moves_for(paths: &[String], rule: &RenameRule) -> Result<Vec<(PathBuf, PathBuf)>, String> {
    let p = plan(paths, rule);
    if p.has_blocking() {
        return Err("blocked".into());
    }
    let mut out = Vec::new();
    for item in p.items.iter().filter(|i| i.status.will_apply()) {
        let dest = destination_for(&item.from, &item.to_name).map_err(|e| e.to_string())?;
        if dest.exists() {
            return Err("destination exists".into());
        }
        out.push((PathBuf::from(&item.from), dest));
    }
    Ok(out)
}

#[test]
fn a_clean_batch_renames_every_file_and_touches_nothing_else() {
    let dir = tempfile::tempdir().unwrap();
    let paths = vec![
        touch(dir.path(), "img1.jpg"),
        touch(dir.path(), "img2.jpg"),
        touch(dir.path(), "notes.txt"),
    ];
    let rule = RenameRule {
        find: "img".into(),
        replace: "photo-{n:02}-".into(),
        counter_start: 1,
        ..Default::default()
    };

    let moves = moves_for(&paths, &rule).unwrap();
    assert_eq!(moves.len(), 2, "notes.txt does not match and is untouched");
    assert_eq!(perform_moves(&moves).unwrap(), 2);

    assert_eq!(
        names_in(dir.path()),
        BTreeSet::from([
            "photo-01-1.jpg".to_string(),
            "photo-02-2.jpg".to_string(),
            "notes.txt".to_string(),
        ])
    );
}

#[test]
fn a_batch_that_would_overwrite_an_existing_file_changes_nothing() {
    let dir = tempfile::tempdir().unwrap();
    let paths = vec![touch(dir.path(), "a.txt")];
    touch(dir.path(), "b.txt");
    let before = names_in(dir.path());

    let rule = RenameRule {
        find: "a".into(),
        replace: "b".into(),
        ..Default::default()
    };
    assert!(
        moves_for(&paths, &rule).is_err(),
        "renaming a.txt over b.txt must be refused"
    );
    assert_eq!(names_in(dir.path()), before, "the directory is untouched");
}

#[test]
fn a_move_never_destroys_an_existing_destination() {
    // `std::fs::rename` replaces the destination on every platform, so
    // the `exists()` check upstream is a check, not a guarantee: a sync
    // client can create the name in the window between the two. This is
    // the guarantee itself.
    let dir = tempfile::tempdir().unwrap();
    let a = touch(dir.path(), "a.txt");
    let victim = dir.path().join("b.txt");
    std::fs::write(&victim, b"MUST-SURVIVE").unwrap();

    let moves = vec![(PathBuf::from(&a), victim.clone())];
    assert!(
        perform_moves(&moves).is_err(),
        "renaming over an existing file must be refused by the kernel"
    );
    assert_eq!(std::fs::read(&victim).unwrap(), b"MUST-SURVIVE");
    assert!(Path::new(&a).exists(), "the source is left where it was");
}

#[test]
fn a_case_only_rename_is_allowed_even_where_the_filesystem_is_case_insensitive() {
    // On NTFS and APFS the destination "exists" because it *is* the
    // source. Refusing that would make every Case transform impossible
    // on the two platforms most users are on.
    let dir = tempfile::tempdir().unwrap();
    let from = touch(dir.path(), "report.txt");
    let to = dir.path().join("REPORT.txt");

    assert_eq!(perform_moves(&[(PathBuf::from(&from), to)]).unwrap(), 1);
    let got = names_in(dir.path());
    assert_eq!(got.len(), 1);
    assert!(
        got.contains("REPORT.txt") || got.contains("report.txt"),
        "got {got:?}"
    );
}

#[test]
fn a_failed_move_rolls_the_whole_batch_back() {
    let dir = tempfile::tempdir().unwrap();
    let a = touch(dir.path(), "a.txt");
    let before = names_in(dir.path());

    // The second move is impossible: its source does not exist. The
    // first has already succeeded by then, so the rollback is what keeps
    // the directory consistent.
    let moves = vec![
        (PathBuf::from(&a), dir.path().join("renamed.txt")),
        (
            dir.path().join("does-not-exist.txt"),
            dir.path().join("other.txt"),
        ),
    ];
    assert!(perform_moves(&moves).is_err());
    assert_eq!(
        names_in(dir.path()),
        before,
        "a half-applied batch is the failure people get hurt by"
    );
}

#[test]
fn undo_restores_the_original_names_and_redo_reapplies_them() {
    let dir = tempfile::tempdir().unwrap();
    let paths = vec![touch(dir.path(), "one.txt"), touch(dir.path(), "two.txt")];
    let original = names_in(dir.path());

    let rule = RenameRule {
        find: "o".into(),
        replace: "0".into(),
        ..Default::default()
    };
    let moves = moves_for(&paths, &rule).unwrap();
    perform_moves(&moves).unwrap();
    let renamed = names_in(dir.path());
    assert_ne!(renamed, original);

    let entry = OperationEntry {
        id: "op-1".into(),
        kind: OperationKind::BulkRename,
        at_ms: 1,
        items: moves
            .iter()
            .map(|(f, t)| OperationItem {
                from: f.to_string_lossy().into_owned(),
                to: t.to_string_lossy().into_owned(),
            })
            .collect(),
        undone: false,
        undoable: true,
        not_undoable_reason: None,
    };
    let mut journal = OperationJournal::default();
    journal.record(entry.clone());
    assert_eq!(journal.next_undo().unwrap().id, "op-1");

    // Undo.
    let inverse: Vec<(PathBuf, PathBuf)> = entry
        .inverse_items()
        .iter()
        .map(|i| (PathBuf::from(&i.from), PathBuf::from(&i.to)))
        .collect();
    perform_moves(&inverse).unwrap();
    journal.set_undone("op-1", true);
    assert_eq!(names_in(dir.path()), original, "undo is exact");
    assert_eq!(journal.next_redo().unwrap().id, "op-1");

    // Redo.
    let forward: Vec<(PathBuf, PathBuf)> = entry
        .items
        .iter()
        .map(|i| (PathBuf::from(&i.from), PathBuf::from(&i.to)))
        .collect();
    perform_moves(&forward).unwrap();
    journal.set_undone("op-1", false);
    assert_eq!(names_in(dir.path()), renamed, "redo is exact");
}

#[test]
fn a_chain_rename_unwinds_without_colliding_with_itself() {
    // a→b and b→c in one batch. Undoing in forward order would rename
    // b back to a first, and then have no b to move to c.
    let dir = tempfile::tempdir().unwrap();
    let a = touch(dir.path(), "a");
    let b = touch(dir.path(), "b");

    let moves = vec![
        (PathBuf::from(&b), dir.path().join("c")),
        (PathBuf::from(&a), dir.path().join("b")),
    ];
    perform_moves(&moves).unwrap();
    assert_eq!(
        names_in(dir.path()),
        BTreeSet::from(["b".to_string(), "c".to_string()])
    );

    let entry = OperationEntry {
        id: "op-1".into(),
        kind: OperationKind::BulkRename,
        at_ms: 1,
        items: moves
            .iter()
            .map(|(f, t)| OperationItem {
                from: f.to_string_lossy().into_owned(),
                to: t.to_string_lossy().into_owned(),
            })
            .collect(),
        undone: false,
        undoable: true,
        not_undoable_reason: None,
    };
    let inverse: Vec<(PathBuf, PathBuf)> = entry
        .inverse_items()
        .iter()
        .map(|i| (PathBuf::from(&i.from), PathBuf::from(&i.to)))
        .collect();
    perform_moves(&inverse).unwrap();
    assert_eq!(
        names_in(dir.path()),
        BTreeSet::from(["a".to_string(), "b".to_string()]),
        "reverse order is what makes the chain unwind"
    );
}
