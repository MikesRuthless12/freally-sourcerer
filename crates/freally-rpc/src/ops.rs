//! SRC-M16 — the operation journal behind Ctrl+Z.
//!
//! Every file operation Freally itself performs is recorded here before
//! the user can undo it. The journal is pure data plus the undo/redo
//! bookkeeping; performing the inverse is the caller's job, because only
//! the app process has filesystem access and the path gate.
//!
//! ## What can actually be undone
//!
//! A rename is exactly invertible: rename it back. Both halves are bare
//! names in one directory, so the inverse is as safe as the original.
//!
//! A delete is not, in general. Freally deletes to the OS trash, and the
//! only cross-platform crate for restoring from it (`trash::os_limited`)
//! is implemented for Windows and freedesktop Linux but **not macOS**. So
//! a delete is recorded either way — it belongs in the history the user
//! reads — but [`OperationEntry::undoable`] is false where the platform
//! cannot honour it, rather than offering an undo that would fail after
//! the user pressed it.

use std::collections::VecDeque;
use std::path::{Component, Path};

use serde::{Deserialize, Serialize};

/// How many operations are remembered across sessions. Past this the
/// oldest fall off: the journal is an undo stack, not an audit log, and
/// an unbounded one would grow forever on a machine that never restarts.
pub const DEFAULT_CAPACITY: usize = 50;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum OperationKind {
    Rename,
    BulkRename,
    /// Moved to the OS trash.
    Delete,
}

/// Why an entry cannot be undone. Sent as a code, not a sentence, so the
/// UI renders it in the user's locale.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum NotUndoable {
    /// This platform has no API for restoring from the trash.
    TrashRestoreUnsupported,
    /// The file has moved or changed since the operation, so putting it
    /// back would not mean what the user expects.
    SourceChanged,
}

/// One file's part in an operation.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct OperationItem {
    /// Path before the operation.
    pub from: String,
    /// Path after it. Empty for a delete — the file is in the trash, and
    /// its location there is the OS's business, not ours.
    #[serde(default)]
    pub to: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct OperationEntry {
    pub id: String,
    pub kind: OperationKind,
    /// Unix ms the operation completed.
    pub at_ms: u64,
    pub items: Vec<OperationItem>,
    /// True once undone; a redo puts it back.
    #[serde(default)]
    pub undone: bool,
    #[serde(default)]
    pub undoable: bool,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub not_undoable_reason: Option<NotUndoable>,
}

/// Largest batch a single journal entry may describe. A real bulk rename
/// is bounded by the selection; anything past this is not a shape the app
/// produces.
pub const MAX_ITEMS_PER_ENTRY: usize = 10_000;

/// Why a journal entry was refused at record time.
#[derive(Debug, thiserror::Error, PartialEq, Eq)]
pub enum EntryRejected {
    #[error("journal entry has no items")]
    Empty,
    #[error("journal entry describes more than {MAX_ITEMS_PER_ENTRY} items")]
    TooManyItems,
    #[error("journal entry has an empty path")]
    EmptyPath,
    #[error("journal entry path is not absolute")]
    NotAbsolute,
    #[error("journal entry path contains a `..` component")]
    Traversal,
    #[error("journal entry moves a file between directories")]
    CrossesDirectories,
    #[error("a delete carries no destination and cannot be replayed")]
    DeleteHasDestination,
}

impl OperationEntry {
    /// Structural check applied **before** an entry is admitted to the
    /// journal.
    ///
    /// The journal drives real renames, and the code that executes it
    /// deliberately skips `validate_name` — an undo restores a name that
    /// already existed on disk, and that validator constrains names this
    /// tool *creates*. So the shape has to be pinned here instead, at the
    /// boundary where an entry arrives, rather than trusted later.
    ///
    /// This constrains *shape*, not *target*: a same-directory pair is
    /// still a same-directory pair wherever it points. Confining which
    /// directories a peer may name is the transport's job — see the
    /// per-user partitioning in `freally-indexd`.
    pub fn validate_shape(&self) -> Result<(), EntryRejected> {
        if self.items.is_empty() {
            return Err(EntryRejected::Empty);
        }
        if self.items.len() > MAX_ITEMS_PER_ENTRY {
            return Err(EntryRejected::TooManyItems);
        }
        for it in &self.items {
            if self.kind == OperationKind::Delete {
                // A delete's `to` is empty by construction; a non-empty
                // one would be a rename wearing a delete's label, which
                // the executor refuses to replay anyway.
                if !it.to.is_empty() {
                    return Err(EntryRejected::DeleteHasDestination);
                }
                continue;
            }
            if it.from.is_empty() || it.to.is_empty() {
                return Err(EntryRejected::EmptyPath);
            }
            let (from, to) = (Path::new(&it.from), Path::new(&it.to));
            if !from.is_absolute() || !to.is_absolute() {
                return Err(EntryRejected::NotAbsolute);
            }
            if [from, to]
                .iter()
                .any(|p| p.components().any(|c| c == Component::ParentDir))
            {
                return Err(EntryRejected::Traversal);
            }
            if from.parent() != to.parent() {
                return Err(EntryRejected::CrossesDirectories);
            }
        }
        Ok(())
    }

    /// The inverse of this entry — what an undo has to perform. Renames
    /// run in reverse order so that a batch which shifted names along a
    /// chain unwinds without colliding with itself.
    pub fn inverse_items(&self) -> Vec<OperationItem> {
        self.items
            .iter()
            .rev()
            .map(|i| OperationItem {
                from: i.to.clone(),
                to: i.from.clone(),
            })
            .collect()
    }
}

/// The undo/redo stack, persisted by the daemon.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OperationJournal {
    #[serde(default)]
    entries: VecDeque<OperationEntry>,
    #[serde(default = "default_capacity")]
    capacity: usize,
}

fn default_capacity() -> usize {
    DEFAULT_CAPACITY
}

impl Default for OperationJournal {
    fn default() -> Self {
        Self {
            entries: VecDeque::new(),
            capacity: DEFAULT_CAPACITY,
        }
    }
}

impl OperationJournal {
    pub fn with_capacity(capacity: usize) -> Self {
        Self {
            entries: VecDeque::new(),
            capacity: capacity.max(1),
        }
    }

    /// Newest last.
    pub fn entries(&self) -> impl Iterator<Item = &OperationEntry> {
        self.entries.iter()
    }

    pub fn len(&self) -> usize {
        self.entries.len()
    }

    pub fn is_empty(&self) -> bool {
        self.entries.is_empty()
    }

    /// Record a completed operation.
    ///
    /// Doing something new discards the redo stack, the same as every
    /// other undo implementation: once history has diverged, "redo"
    /// would replay a change against a tree that no longer matches the
    /// one it was recorded against.
    pub fn record(&mut self, entry: OperationEntry) {
        self.entries.retain(|e| !e.undone);
        self.entries.push_back(entry);
        while self.entries.len() > self.capacity {
            self.entries.pop_front();
        }
    }

    /// The entry Ctrl+Z would act on: the newest one still in effect.
    ///
    /// Returns `None` when the newest live entry cannot be undone,
    /// rather than skipping past it to an older one — undoing out of
    /// order would put a file back into a state a later operation has
    /// already moved on from.
    pub fn next_undo(&self) -> Option<&OperationEntry> {
        let last = self.entries.iter().rev().find(|e| !e.undone)?;
        last.undoable.then_some(last)
    }

    /// The entry Ctrl+Shift+Z would act on: the oldest undone entry, so
    /// a run of undos redoes in the order it was undone.
    pub fn next_redo(&self) -> Option<&OperationEntry> {
        self.entries.iter().find(|e| e.undone)
    }

    /// Flip an entry's undone flag after the caller performed the work.
    /// Returns false when the id is unknown, so a stale UI cannot
    /// desynchronize the journal from the disk.
    pub fn set_undone(&mut self, id: &str, undone: bool) -> bool {
        match self.entries.iter_mut().find(|e| e.id == id) {
            Some(e) => {
                e.undone = undone;
                true
            }
            None => false,
        }
    }

    pub fn clear(&mut self) {
        self.entries.clear();
    }
}

/// Whether this build can restore from the OS trash. Windows and
/// freedesktop Linux expose it; macOS does not.
pub const fn trash_restore_supported() -> bool {
    cfg!(any(
        target_os = "windows",
        all(
            unix,
            not(target_os = "macos"),
            not(target_os = "ios"),
            not(target_os = "android")
        )
    ))
}

#[cfg(test)]
mod tests {
    use super::*;

    fn rename_entry(id: &str, from: &str, to: &str) -> OperationEntry {
        OperationEntry {
            id: id.to_string(),
            kind: OperationKind::Rename,
            at_ms: 1,
            items: vec![OperationItem {
                from: from.into(),
                to: to.into(),
            }],
            undone: false,
            undoable: true,
            not_undoable_reason: None,
        }
    }

    /// An absolute path for the host platform. `Path::is_absolute` on
    /// Windows needs a drive prefix, so a POSIX-shaped literal would not
    /// exercise the check the way a real path does.
    fn abs(rest: &str) -> String {
        if cfg!(windows) {
            format!("C:\\{}", rest.replace('/', "\\"))
        } else {
            format!("/{rest}")
        }
    }

    #[test]
    fn a_well_formed_rename_entry_is_admitted() {
        assert_eq!(
            rename_entry("1", &abs("a/x"), &abs("a/y")).validate_shape(),
            Ok(())
        );
    }

    #[test]
    fn an_entry_that_would_move_a_file_out_of_its_directory_is_refused() {
        // The journal drives real renames and the executor does not
        // re-derive the destination, so a cross-directory pair here would
        // become a cross-directory move at replay.
        assert_eq!(
            rename_entry("1", &abs("a/x"), &abs("b/x")).validate_shape(),
            Err(EntryRejected::CrossesDirectories)
        );
    }

    #[test]
    fn traversal_and_relative_paths_are_refused() {
        assert_eq!(
            rename_entry("1", &abs("a/../etc/x"), &abs("a/../etc/y")).validate_shape(),
            Err(EntryRejected::Traversal)
        );
        assert_eq!(
            rename_entry("1", "a/x", "a/y").validate_shape(),
            Err(EntryRejected::NotAbsolute)
        );
    }

    #[test]
    fn an_empty_or_oversized_entry_is_refused() {
        let mut e = rename_entry("1", &abs("a/x"), &abs("a/y"));
        e.items.clear();
        assert_eq!(e.validate_shape(), Err(EntryRejected::Empty));

        let mut big = rename_entry("2", &abs("a/x"), &abs("a/y"));
        big.items = (0..MAX_ITEMS_PER_ENTRY + 1)
            .map(|i| OperationItem {
                from: abs(&format!("a/x{i}")),
                to: abs(&format!("a/y{i}")),
            })
            .collect();
        assert_eq!(big.validate_shape(), Err(EntryRejected::TooManyItems));
    }

    #[test]
    fn a_delete_may_carry_no_destination() {
        let mut del = rename_entry("1", &abs("a/gone"), "");
        del.kind = OperationKind::Delete;
        assert_eq!(del.validate_shape(), Ok(()));

        // A rename wearing a delete's label is refused.
        let mut sneaky = rename_entry("2", &abs("a/x"), &abs("a/y"));
        sneaky.kind = OperationKind::Delete;
        assert_eq!(
            sneaky.validate_shape(),
            Err(EntryRejected::DeleteHasDestination)
        );
    }

    #[test]
    fn the_newest_operation_is_the_one_undone_first() {
        let mut j = OperationJournal::default();
        j.record(rename_entry("1", "/a/x", "/a/y"));
        j.record(rename_entry("2", "/a/p", "/a/q"));
        assert_eq!(j.next_undo().unwrap().id, "2");
    }

    #[test]
    fn undoing_then_redoing_walks_back_and_forth() {
        let mut j = OperationJournal::default();
        j.record(rename_entry("1", "/a/x", "/a/y"));
        j.record(rename_entry("2", "/a/p", "/a/q"));

        assert!(j.set_undone("2", true));
        assert_eq!(j.next_undo().unwrap().id, "1", "undo moves down the stack");
        assert_eq!(j.next_redo().unwrap().id, "2", "and 2 is redoable");

        assert!(j.set_undone("1", true));
        assert!(j.next_undo().is_none(), "nothing left to undo");
        assert_eq!(
            j.next_redo().unwrap().id,
            "1",
            "redo replays in the order things were undone"
        );
    }

    #[test]
    fn a_new_operation_discards_the_redo_stack() {
        let mut j = OperationJournal::default();
        j.record(rename_entry("1", "/a/x", "/a/y"));
        j.set_undone("1", true);
        assert!(j.next_redo().is_some());

        j.record(rename_entry("2", "/a/p", "/a/q"));
        assert!(
            j.next_redo().is_none(),
            "history diverged; replaying the old branch would corrupt the tree"
        );
        assert_eq!(j.len(), 1);
    }

    #[test]
    fn an_unundoable_operation_blocks_undo_rather_than_being_skipped() {
        let mut j = OperationJournal::default();
        j.record(rename_entry("1", "/a/x", "/a/y"));
        let mut del = rename_entry("2", "/a/gone", "");
        del.kind = OperationKind::Delete;
        del.undoable = false;
        del.not_undoable_reason = Some(NotUndoable::TrashRestoreUnsupported);
        j.record(del);

        // Undoing "1" here would rename /a/y back to /a/x as if the
        // delete had never happened, silently reordering history.
        assert!(j.next_undo().is_none());
    }

    #[test]
    fn the_journal_is_bounded() {
        let mut j = OperationJournal::with_capacity(3);
        for i in 0..5 {
            j.record(rename_entry(&i.to_string(), "/a/x", "/a/y"));
        }
        assert_eq!(j.len(), 3);
        let ids: Vec<&str> = j.entries().map(|e| e.id.as_str()).collect();
        assert_eq!(ids, vec!["2", "3", "4"], "oldest fall off");
    }

    #[test]
    fn a_capacity_of_zero_is_clamped_so_recording_still_works() {
        let mut j = OperationJournal::with_capacity(0);
        j.record(rename_entry("1", "/a/x", "/a/y"));
        assert_eq!(j.len(), 1);
    }

    #[test]
    fn an_unknown_id_cannot_desynchronize_the_journal() {
        let mut j = OperationJournal::default();
        j.record(rename_entry("1", "/a/x", "/a/y"));
        assert!(!j.set_undone("nope", true));
        assert!(!j.entries().next().unwrap().undone);
    }

    #[test]
    fn the_inverse_of_a_batch_unwinds_in_reverse_order() {
        // A shift-along-a-chain rename: a→b, b→c. Undoing forwards
        // would rename b→a first and then find no b to move to c.
        let entry = OperationEntry {
            id: "1".into(),
            kind: OperationKind::BulkRename,
            at_ms: 1,
            items: vec![
                OperationItem {
                    from: "/d/a".into(),
                    to: "/d/b".into(),
                },
                OperationItem {
                    from: "/d/b".into(),
                    to: "/d/c".into(),
                },
            ],
            undone: false,
            undoable: true,
            not_undoable_reason: None,
        };
        let inv = entry.inverse_items();
        assert_eq!(inv[0].from, "/d/c");
        assert_eq!(inv[0].to, "/d/b");
        assert_eq!(inv[1].from, "/d/b");
        assert_eq!(inv[1].to, "/d/a");
    }

    #[test]
    fn an_empty_journal_offers_neither_undo_nor_redo() {
        let j = OperationJournal::default();
        assert!(j.next_undo().is_none());
        assert!(j.next_redo().is_none());
        assert!(j.is_empty());
    }

    #[test]
    fn a_journal_round_trips_through_json() {
        let mut j = OperationJournal::default();
        j.record(rename_entry("1", "/a/x", "/a/y"));
        j.set_undone("1", true);
        let s = serde_json::to_string(&j).unwrap();
        let back: OperationJournal = serde_json::from_str(&s).unwrap();
        assert_eq!(back.len(), 1);
        assert_eq!(back.next_redo().unwrap().id, "1");
    }
}
