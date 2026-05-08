//! `JournalEvent` + `JournalError` — the Phase-2 macOS public surface.
//!
//! Mirrored from `sourcerer-journal-win` so the facade in `sourcerer-journal`
//! re-exports a single shape on every OS. `JournalEvent` is byte-for-byte
//! identical across platforms; `JournalError` carries OS-specific failure
//! modes (FSEvents-stream creation, `statfs` reads, run-loop scheduling).

use std::path::PathBuf;

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

impl JournalEvent {
    /// Returns the "primary" path of an event, picking `new_path` for
    /// renames. Used by the integration / smoke harness to filter events to
    /// a scratch directory.
    pub fn primary_path(&self) -> &std::path::Path {
        match self {
            JournalEvent::Create { path, .. }
            | JournalEvent::Modify { path, .. }
            | JournalEvent::Delete { path }
            | JournalEvent::AttrChange { path, .. } => path,
            JournalEvent::Rename { new_path, .. } => new_path,
        }
    }

    pub fn variant_name(&self) -> &'static str {
        match self {
            JournalEvent::Create { .. } => "Create",
            JournalEvent::Modify { .. } => "Modify",
            JournalEvent::Delete { .. } => "Delete",
            JournalEvent::Rename { .. } => "Rename",
            JournalEvent::AttrChange { .. } => "AttrChange",
        }
    }
}

#[derive(Debug, thiserror::Error)]
pub enum JournalError {
    #[error("watch root must be an absolute, existing directory: {0}")]
    InvalidRoot(PathBuf),
    #[error("FSEventStreamCreate returned NULL for `{0}`")]
    StreamCreateFailed(PathBuf),
    #[error("FSEventStreamStart returned false for `{0}`")]
    StreamStartFailed(PathBuf),
    #[error("opening watch root `{0}` failed: {1}")]
    OpenRoot(PathBuf, #[source] std::io::Error),
    #[error("statfs(`{0}`) failed: {1}")]
    Statfs(PathBuf, #[source] std::io::Error),
    #[error("filesystem walk of `{0}` failed: {1}")]
    WalkFailed(PathBuf, #[source] std::io::Error),
    #[error("cursor persistence error: {0}")]
    Cursor(#[from] crate::cursor::CursorError),
    #[error("operation not supported on this platform; sourcerer-journal-mac is macOS-only")]
    UnsupportedPlatform,
}
