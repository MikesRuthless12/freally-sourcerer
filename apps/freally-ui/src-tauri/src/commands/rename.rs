//! SRC-M15 bulk rename + SRC-M16 undo/redo — the commands that touch
//! the disk.
//!
//! ## The rule this module exists to enforce
//!
//! Build 1 established that write-side IPC must never accept a
//! caller-chosen path. A rename has a destination, so the frontend is
//! never allowed to supply one: it sends the selected paths and the
//! *rule*, and the backend derives every new name itself through
//! `freally_rpc::rename`. The derivation is re-run at apply time rather
//! than trusting the preview, so a frontend that tampered with the
//! preview table changes nothing.
//!
//! Three gates stand between a request and a `std::fs::rename`:
//!
//! 1. **Provenance.** Every source must be a path the daemon returned as
//!    a search hit this session, the same gate `files_delete` uses.
//! 2. **Name validity.** The derived name must be a bare file name —
//!    `freally_rpc::rename::validate_name` rejects separators, `..`,
//!    and everything a platform would refuse or silently rewrite.
//! 3. **Same directory.** The destination is rebuilt by joining the
//!    source's own parent with the derived name, so a rename can never
//!    relocate a file even if the first two gates were somehow passed.
//!
//! On top of that the batch is **all-or-nothing at the planning stage**:
//! if any row would collide, overwrite, or is invalid, nothing is
//! applied. Partially-applied bulk renames are the failure mode people
//! actually get hurt by.

use std::path::{Path, PathBuf};

use freally_rpc::rename::{
    RenameItem, RenameRule, RenameStatus, destination_for, is_same_file, perform_moves,
};
use serde::Serialize;
use tauri::State;

use super::known_paths::{KnownPaths, Provenance};
use crate::daemon;

/// Cap on one batch. Large enough for any real selection, small enough
/// that a runaway caller cannot make the app walk a million paths.
const MAX_BATCH: usize = 10_000;

#[derive(Debug, Serialize)]
pub struct RenamePreview {
    pub items: Vec<RenameItem>,
    pub will_apply: usize,
    pub blocked: bool,
}

/// Build the preview table. Pure planning plus one `exists` check per
/// row, which is the only part that needs the filesystem.
#[tauri::command]
pub fn files_rename_preview(
    paths: Vec<String>,
    rule: RenameRule,
    known: State<'_, KnownPaths>,
) -> Result<RenamePreview, String> {
    if paths.len() > MAX_BATCH {
        return Err(format!("selection exceeds {MAX_BATCH} files"));
    }
    known.verify_all(&paths, Provenance::QueryHit)?;

    let mut plan = freally_rpc::rename::plan(&paths, &rule);
    for item in &mut plan.items {
        if item.status != RenameStatus::Ok {
            continue;
        }
        match destination_for(&item.from, &item.to_name) {
            Err(_) => item.status = RenameStatus::Invalid,
            Ok(dest) => {
                // A destination that already exists is refused rather
                // than overwritten. `exists()` is a race against the
                // filesystem, but the apply re-checks and the rename
                // itself is the real guard — this is here so the user
                // sees the problem before pressing Apply.
                // A case-only rename finds a destination that *is* the
                // source; that is not a conflict.
                if dest.exists() && !is_same_file(Path::new(&item.from), &dest) {
                    item.status = RenameStatus::Exists;
                }
            }
        }
    }

    Ok(RenamePreview {
        will_apply: plan.will_apply_count(),
        blocked: plan.has_blocking(),
        items: plan.items,
    })
}

#[derive(Debug, Serialize)]
pub struct RenameOutcome {
    pub renamed: usize,
    /// Journal entry id, so the UI can show "Undo rename of 12 files".
    pub operation_id: Option<String>,
}

/// Apply the rename. Re-derives every name from the rule — the preview
/// is a display artifact, not an instruction.
#[tauri::command]
pub fn files_rename_apply(
    paths: Vec<String>,
    rule: RenameRule,
    known: State<'_, KnownPaths>,
) -> Result<RenameOutcome, String> {
    if paths.len() > MAX_BATCH {
        return Err(format!("selection exceeds {MAX_BATCH} files"));
    }
    known.verify_all(&paths, Provenance::QueryHit)?;

    let plan = freally_rpc::rename::plan(&paths, &rule);
    if plan.has_blocking() {
        return Err("the rename has conflicts; nothing was changed".into());
    }

    // Resolve every destination before moving a single file, so a batch
    // that cannot fully succeed does not half-succeed.
    let mut moves: Vec<(PathBuf, PathBuf)> = Vec::new();
    for item in plan.items.iter().filter(|i| i.status.will_apply()) {
        let dest = destination_for(&item.from, &item.to_name).map_err(|e| e.to_string())?;
        if dest.exists() && !is_same_file(Path::new(&item.from), &dest) {
            return Err("a destination already exists; nothing was changed".into());
        }
        moves.push((PathBuf::from(&item.from), dest));
    }
    if moves.is_empty() {
        return Ok(RenameOutcome {
            renamed: 0,
            operation_id: None,
        });
    }

    let renamed = perform_moves(&moves).map_err(|e| e.to_string())?;
    let items: Vec<freally_rpc::OperationItem> = moves
        .iter()
        .map(|(from, to)| freally_rpc::OperationItem {
            from: from.to_string_lossy().into_owned(),
            to: to.to_string_lossy().into_owned(),
        })
        .collect();

    // Renamed files are legitimate targets for follow-up operations
    // (including the undo), so grant them the same provenance their
    // sources had - and revoke the old name, which no longer refers to
    // anything the daemon returned. Leaving it granted would let a
    // later, unrelated file created at that path inherit a permission
    // it was never given.
    for (from, to) in &moves {
        known.add(&to.to_string_lossy());
        known.revoke(&from.to_string_lossy());
    }

    let kind = if renamed == 1 {
        freally_rpc::OperationKind::Rename
    } else {
        freally_rpc::OperationKind::BulkRename
    };
    let operation_id = record_operation(kind, items);

    Ok(RenameOutcome {
        renamed,
        operation_id,
    })
}

/// Push an entry onto the daemon's journal. A failure here is logged and
/// swallowed: the files really were renamed, and refusing to report that
/// because the history could not be written would be worse than losing
/// one undo entry.
fn record_operation(
    kind: freally_rpc::OperationKind,
    items: Vec<freally_rpc::OperationItem>,
) -> Option<String> {
    let id = format!(
        "op-{}",
        std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .map(|d| d.as_micros())
            .unwrap_or(0)
    );
    let entry = freally_rpc::OperationEntry {
        id: id.clone(),
        kind,
        at_ms: super::bookmarks::now_ms(),
        items,
        undone: false,
        // Renames are always invertible. A delete would not be, but
        // deletes are not journaled - see the module docs on M16.
        undoable: true,
        not_undoable_reason: None,
    };
    let daemon = daemon::get()?;
    match serde_json::to_value(&entry) {
        Ok(v) => match daemon.call_void("ops.record", v) {
            Ok(()) => Some(id),
            Err(e) => {
                tracing::warn!(error = %e, "recording the operation for undo failed");
                None
            }
        },
        Err(_) => None,
    }
}

/// Undo or redo one journal entry.
///
/// The paths come from the daemon's journal, not from the frontend, and
/// every move is re-validated the same way a fresh rename would be:
/// bare name, same directory, destination free. That matters because
/// after a restart the journal's paths are not in this session's
/// known-path registry — the journal is its own provenance, and it still
/// does not get to skip the structural checks.
fn apply_journal_entry(
    entry: &freally_rpc::OperationEntry,
    redo: bool,
    known: &KnownPaths,
) -> Result<usize, String> {
    if !entry.undoable {
        return Err("this operation cannot be undone".into());
    }
    if !matches!(
        entry.kind,
        freally_rpc::OperationKind::Rename | freally_rpc::OperationKind::BulkRename
    ) {
        return Err("only renames can be undone".into());
    }

    let pairs: Vec<freally_rpc::OperationItem> = if redo {
        entry.items.clone()
    } else {
        entry.inverse_items()
    };

    let mut moves: Vec<(PathBuf, PathBuf)> = Vec::with_capacity(pairs.len());
    for p in &pairs {
        if p.from.is_empty() || p.to.is_empty() {
            return Err("journal entry is incomplete".into());
        }
        let from = PathBuf::from(&p.from);
        let to = PathBuf::from(&p.to);
        // Structural checks only. `validate_name` is deliberately NOT
        // applied here: it constrains names this tool *creates*, and the
        // undo destination is a name that already existed on disk.
        // Running it would make a file legitimately called `my:notes.txt`
        // on Linux impossible to restore, permanently — and because the
        // entry stays live, it would block undo of everything older too.
        let name = to
            .file_name()
            .ok_or_else(|| "journal entry has no destination name".to_string())?;
        if from.parent() != to.parent() {
            return Err("journal entry crosses directories".into());
        }
        let dest = to
            .parent()
            .ok_or_else(|| "journal entry has no parent directory".to_string())?
            .join(name);
        if !from.exists() {
            return Err("a file has moved since this operation; nothing was changed".into());
        }
        moves.push((from, dest));
    }

    // Destinations are deliberately NOT pre-checked for existence. A
    // batch that shifted names along a chain (a→b, b→c) unwinds in
    // reverse, so each destination is freed by the move immediately
    // before it — pre-checking would reject exactly the case
    // `inverse_items` reverses the order to support. `perform_moves`
    // refuses to clobber at the syscall, which is the real guarantee.
    let moved = perform_moves(&moves).map_err(|e| e.to_string())?;
    for (from, to) in &moves {
        known.add(&to.to_string_lossy());
        known.revoke(&from.to_string_lossy());
    }
    Ok(moved)
}

#[derive(Debug, Serialize)]
pub struct UndoOutcome {
    pub moved: usize,
    pub id: String,
}

fn journal_entry(id: &str) -> Result<freally_rpc::OperationEntry, String> {
    let daemon = daemon::get().ok_or_else(|| "daemon not initialized".to_string())?;
    let listing: serde_json::Value = daemon
        .call("ops.list", serde_json::Value::Null)
        .map_err(|e| e.to_string())?;
    let entries: Vec<freally_rpc::OperationEntry> =
        serde_json::from_value(listing.get("entries").cloned().unwrap_or_default())
            .map_err(|e| e.to_string())?;
    entries
        .into_iter()
        .find(|e| e.id == id)
        .ok_or_else(|| "unknown operation".to_string())
}

fn mark_undone(id: &str, undone: bool) -> Result<(), String> {
    let daemon = daemon::get().ok_or_else(|| "daemon not initialized".to_string())?;
    daemon
        .call_void(
            "ops.set_undone",
            serde_json::json!({ "id": id, "undone": undone }),
        )
        .map_err(|e| e.to_string())
}

/// Flip the journal flag *first*, then move the files, rolling the flag
/// back if the move fails.
///
/// The other order is worse in a way that is hard to recover from: if
/// the files move and then the flag write fails, the disk and the
/// journal disagree, and because `next_undo` deliberately never skips
/// past the newest live entry, that entry becomes a permanent tombstone
/// blocking undo of everything older. Flipping first can only ever leave
/// the flag wrong in the direction that makes the entry *retryable*.
fn run_journal_op(id: &str, redo: bool, known: &KnownPaths) -> Result<usize, String> {
    let entry = journal_entry(id)?;
    if redo && !entry.undone {
        return Err("that operation has not been undone".into());
    }
    if !redo && entry.undone {
        return Err("that operation has already been undone".into());
    }

    mark_undone(id, !redo)?;
    match apply_journal_entry(&entry, redo, known) {
        Ok(moved) => Ok(moved),
        Err(e) => {
            // Best effort: if this also fails the entry is left claiming
            // a state the disk does not have, but it stays reachable
            // from the opposite direction.
            let _ = mark_undone(id, redo);
            Err(e)
        }
    }
}

#[tauri::command]
pub fn ops_undo(id: String, known: State<'_, KnownPaths>) -> Result<UndoOutcome, String> {
    let moved = run_journal_op(&id, false, &known)?;
    Ok(UndoOutcome { moved, id })
}

#[tauri::command]
pub fn ops_redo(id: String, known: State<'_, KnownPaths>) -> Result<UndoOutcome, String> {
    let moved = run_journal_op(&id, true, &known)?;
    Ok(UndoOutcome { moved, id })
}

#[tauri::command]
pub fn ops_list() -> Result<serde_json::Value, String> {
    let daemon = daemon::get().ok_or_else(|| "daemon not initialized".to_string())?;
    daemon
        .call("ops.list", serde_json::Value::Null)
        .map_err(|e| e.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a_destination_is_built_from_the_source_parent() {
        let d = destination_for("/vault/docs/a.txt", "b.txt").unwrap();
        assert_eq!(d, PathBuf::from("/vault/docs/b.txt"));
    }

    #[test]
    fn a_name_that_tries_to_escape_the_directory_is_refused() {
        for name in ["../evil", "sub/evil", "sub\\evil", "..", "/abs"] {
            assert!(
                destination_for("/vault/docs/a.txt", name).is_err(),
                "{name} was accepted"
            );
        }
    }

    #[test]
    fn an_absolute_destination_cannot_be_smuggled_in_as_a_name() {
        // The classic Path::join footgun: joining an absolute path
        // replaces the base entirely. validate_name catches it first.
        assert!(destination_for("/vault/a.txt", "/etc/passwd").is_err());
        assert!(destination_for("/vault/a.txt", "C:\\Windows\\x").is_err());
    }
}
