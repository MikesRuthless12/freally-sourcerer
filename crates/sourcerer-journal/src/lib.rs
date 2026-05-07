//! Sourcerer journal facade — OS-agnostic event types and per-OS routing.
//!
//! Phase 0 shipped this as scaffolding only. Phase 1 wires the Windows
//! NTFS USN journal subscriber as the canonical implementation on
//! `cfg(windows)`; Phases 2 and 3 will mirror with FSEvents (macOS) and
//! inotify/fanotify (Linux).

#[cfg(windows)]
pub use sourcerer_journal_win::{
    open, JournalError, JournalEvent, JournalSubscriber, VolumeCursor,
};

#[cfg(target_os = "macos")]
pub use sourcerer_journal_mac::placeholder as macos_placeholder;

#[cfg(target_os = "linux")]
pub use sourcerer_journal_lin::placeholder as linux_placeholder;

// On non-Windows hosts, expose a typed-but-stubbed surface so the rest of the
// workspace (clippy, tests, docs) can compile without `cfg`-fences. Phase 2
// and Phase 3 replace this with real subscribers.
#[cfg(not(windows))]
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

    pub fn open(_volume: &Path) -> Result<JournalSubscriber, JournalError> {
        Err(JournalError::Unimplemented)
    }
}

#[cfg(not(windows))]
pub use portable_stub::{open, JournalError, JournalEvent, JournalSubscriber};

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
