//! Restoring deleted files from the OS trash (HANDOFF §2.5).
//!
//! `files_delete` sends rows to the OS trash, which is recoverable — but it
//! recorded nothing, so the app's own Undo never offered a delete back. The
//! journal has modelled a `Delete` entry since M16 (`to` empty by
//! construction, and `NotUndoable::TrashRestoreUnsupported` exists for
//! exactly one purpose), and `freally_rpc::trash_restore_supported()` has been
//! sitting unused with a cfg identical to the `trash` crate's own
//! `os_limited` gate. This is the piece that was missing between them.
//!
//! Windows and freedesktop Linux can enumerate and restore trash contents.
//! macOS cannot: Finder owns "Put Back" and exposes no API for it, so a
//! delete there is recorded as **not undoable, with the reason attached**,
//! and the UI says so rather than offering an Undo that would fail. That is
//! the visible-rather-than-silent half of the same fix.
//!
//! The platform split is two `mod` blocks rather than a `#[cfg]` on each
//! item. `trash_restore_supported()` is a `const fn` and so cannot appear in
//! `#[cfg]` position, which means this predicate has to be written out
//! longhand — and every extra copy is a chance to drop a clause like
//! `not(target_os = "android")` and get a missing-function error on one CI
//! leg only. Two copies is the minimum; `support_matches_the_platform` pins
//! them against `trash_restore_supported()`.

#[cfg(test)]
use std::path::PathBuf;

#[cfg(any(
    target_os = "windows",
    all(
        unix,
        not(target_os = "macos"),
        not(target_os = "ios"),
        not(target_os = "android")
    )
))]
mod supported {
    use std::collections::HashMap;
    use std::path::{Path, PathBuf};

    /// Restore `paths` from the OS trash.
    ///
    /// Returns how many were restored. Errors describe what the user can do
    /// about it, because the common failures here are recoverable by hand
    /// (emptied the bin, restored it in Explorer already).
    pub fn restore_from_trash(paths: &[PathBuf]) -> Result<usize, String> {
        let items =
            trash::os_limited::list().map_err(|e| format!("reading the trash failed: {e}"))?;
        let wanted = select_restorable(&items, paths)?;
        let n = wanted.len();
        trash::os_limited::restore_all(wanted).map_err(|e| format!("restoring failed: {e}"))?;
        Ok(n)
    }

    /// Pick the trash entries matching `paths`, newest first.
    ///
    /// Split out from the restore so the matching rules are testable without
    /// a trash can. Two of them are load-bearing:
    ///
    /// * **Newest wins.** Deleting `notes.txt`, recreating it and deleting it
    ///   again leaves two entries with the same original path. Restoring the
    ///   older one would silently resurrect the wrong file.
    /// * **All or nothing.** A partial restore leaves the journal entry
    ///   marked undone while some files are still in the bin, and the user
    ///   has no way to tell which. Better to refuse and say what is missing.
    ///
    /// Indexed in one pass rather than scanned per wanted path.
    /// `TrashItem::original_path()` builds a fresh `PathBuf` on every call,
    /// so the naive nested loop cost a join and a compare for every
    /// (wanted × item) pair — undoing a 1000-file delete against a 5000-item
    /// bin is five million of each. This is the same rule, O(n + m).
    fn select_restorable(
        items: &[trash::TrashItem],
        paths: &[PathBuf],
    ) -> Result<Vec<trash::TrashItem>, String> {
        let mut newest: HashMap<PathBuf, &trash::TrashItem> = HashMap::with_capacity(items.len());
        for it in items {
            newest
                .entry(it.original_path())
                .and_modify(|best| {
                    if it.time_deleted > best.time_deleted {
                        *best = it;
                    }
                })
                .or_insert(it);
        }
        paths
            .iter()
            .map(|want| {
                newest.get(want).map(|it| (*it).clone()).ok_or_else(|| {
                    format!(
                        "{} is no longer in the trash — it may have been emptied \
                         or already restored",
                        display_name(want)
                    )
                })
            })
            .collect()
    }

    /// The file's own name, for an error a user reads. The full path is often
    /// long enough to bury the one part that identifies which file this was.
    ///
    /// Lives in here rather than at module level because `select_restorable`
    /// is its only caller: at module level it is dead code on macOS, and this
    /// crate builds with `-D warnings`.
    fn display_name(p: &Path) -> String {
        p.file_name()
            .map(|s| s.to_string_lossy().into_owned())
            .unwrap_or_else(|| p.to_string_lossy().into_owned())
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        fn display_name_prefers_the_file_name() {
            assert_eq!(
                display_name(Path::new("/vault/docs/report.pdf")),
                "report.pdf"
            );
        }

        #[test]
        fn display_name_falls_back_to_the_whole_path() {
            // A path ending in `..` has no file name; an empty string in the
            // error would be worse than showing the path.
            assert_eq!(display_name(Path::new("/vault/..")), "/vault/..");
        }
    }
}

#[cfg(not(any(
    target_os = "windows",
    all(
        unix,
        not(target_os = "macos"),
        not(target_os = "ios"),
        not(target_os = "android")
    )
)))]
mod supported {
    use std::path::PathBuf;

    /// Unreachable in practice — the entry is recorded as not undoable on
    /// this platform, so `apply_journal_entry` refuses before reaching here.
    /// A real error rather than an `unreachable!` because a stale journal
    /// written by a build that *did* support it is a file on disk, not a code
    /// path we control.
    pub fn restore_from_trash(_paths: &[PathBuf]) -> Result<usize, String> {
        Err("this platform cannot restore from the trash; use Finder's Put Back".into())
    }
}

pub use supported::restore_from_trash;

#[cfg(test)]
mod tests {
    use super::*;

    /// The reason a delete is or is not offered back must match what the
    /// `trash` crate can actually do. The two cfgs above are written out
    /// separately from `trash_restore_supported()`, and nothing else would
    /// catch them drifting apart.
    #[test]
    fn support_matches_the_platform_we_can_actually_restore_on() {
        let claimed = freally_rpc::trash_restore_supported();
        let actual = cfg!(any(
            target_os = "windows",
            all(
                unix,
                not(target_os = "macos"),
                not(target_os = "ios"),
                not(target_os = "android")
            )
        ));
        assert_eq!(
            claimed, actual,
            "trash_restore_supported() disagrees with the cfg guarding os_limited"
        );
    }

    /// On an unsupported platform the stub must refuse rather than panic,
    /// since a stale journal can still reach it.
    #[test]
    fn the_stub_refuses_rather_than_panicking_where_unsupported() {
        if freally_rpc::trash_restore_supported() {
            return;
        }
        assert!(restore_from_trash(&[PathBuf::from("/tmp/x")]).is_err());
    }
}
