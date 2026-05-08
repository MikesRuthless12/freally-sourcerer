//! Sourcerer journal facade — OS-agnostic event types and per-OS routing.
//!
//! Phase 0 shipped this as scaffolding only. Phase 1 wired the Windows NTFS
//! USN journal subscriber as the canonical implementation on `cfg(windows)`;
//! Phase 2 wires the macOS FSEvents subscriber on `cfg(target_os = "macos")`.
//! Phase 3 will fill in the Linux inotify/fanotify path; until then, Linux
//! uses the typed-but-stubbed surface in `portable_stub` so the rest of the
//! workspace (clippy, tests, docs) compiles cross-OS without `cfg`-fences.

#[cfg(windows)]
pub use sourcerer_journal_win::{
    JournalError, JournalEvent, JournalSubscriber, VolumeCursor, open, open_with_cursor_root,
};

#[cfg(target_os = "macos")]
pub use sourcerer_journal_mac::{
    JournalError, JournalEvent, JournalSubscriber, StreamCursor, open, open_with_cursor_root,
};

#[cfg(all(not(windows), not(target_os = "macos")))]
mod portable_stub {
    use std::path::{Path, PathBuf};

    use serde::{Deserialize, Serialize};

    #[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
    pub enum JournalEvent {
        Create {
            path: PathBuf,
            size: u64,
            mtime_ns: i128,
            ctime_ns: i128,
            attrs: u32,
        },
        Modify {
            path: PathBuf,
            size: u64,
            mtime_ns: i128,
            attrs: u32,
        },
        Delete {
            path: PathBuf,
        },
        Rename {
            old_path: PathBuf,
            new_path: PathBuf,
        },
        AttrChange {
            path: PathBuf,
            attrs: u32,
        },
    }

    #[derive(Debug, thiserror::Error)]
    pub enum JournalError {
        #[error("journal subscriber not yet implemented for this platform")]
        Unimplemented,
    }

    pub struct JournalSubscriber;

    pub fn open(_root: &Path) -> Result<JournalSubscriber, JournalError> {
        Err(JournalError::Unimplemented)
    }
}

#[cfg(all(not(windows), not(target_os = "macos")))]
pub use portable_stub::{JournalError, JournalEvent, JournalSubscriber, open};

#[cfg(test)]
mod tests {
    #[test]
    fn types_re_exported() {
        // Compile-time check: the canonical types are reachable through the
        // facade on every OS.
        let _: Option<super::JournalEvent> = None;
        let _: Option<super::JournalError> = None;
    }
}
