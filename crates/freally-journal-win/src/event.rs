//! `JournalEvent` + `JournalError` — the Phase-1 public surface.
//!
//! Mirrored verbatim by the macOS and Linux crates in Phases 2 and 3 so the
//! `freally-index` crate can consume any subscriber through a single shape.

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
    /// Convenience accessor used by the integration test harness — returns
    /// the "primary" path of an event, picking `new_path` for renames.
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

/// OS-agnostic snapshot of where a subscriber currently sits in its
/// change stream. Mirrored by the macOS and Linux crates so the daemon's
/// watcher supervisor can detect a journal recreate / wrap without
/// knowing the per-OS cursor type.
///
/// `generation` identifies the stream itself — a change means the OS
/// threw away the old stream, so any events between the two are lost and
/// the index needs a rebuild. `offset` is the monotonic position within
/// that stream; it going *backwards* under a stable generation means the
/// stream wrapped and we were reseated.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct JournalPosition {
    /// Windows: the USN journal id. macOS: the BSD device id. Linux: the
    /// `st_dev` of the watched root.
    pub generation: u64,
    /// Windows: `NextUsn`. macOS: the last FSEvents event id. Linux: the
    /// wall-clock nanoseconds of the last emitted event (advisory — the
    /// inotify/fanotify backends have no resumable cursor).
    pub offset: u64,
}

#[derive(Debug, thiserror::Error)]
pub enum JournalError {
    #[error("volume path is not a Windows drive root (expected e.g. `C:\\`): {0}")]
    InvalidVolumePath(PathBuf),
    #[error("FSCTL_QUERY_USN_JOURNAL failed: {0}")]
    QueryJournal(#[source] std::io::Error),
    #[error("FSCTL_ENUM_USN_DATA failed: {0}")]
    EnumMft(#[source] std::io::Error),
    #[error("FSCTL_READ_USN_JOURNAL failed: {0}")]
    ReadJournal(#[source] std::io::Error),
    #[error("opening volume `{0}` failed: {1}")]
    OpenVolume(PathBuf, #[source] std::io::Error),
    #[error("resolving file `{frn}` to a path failed: {source}")]
    ResolvePath {
        frn: u64,
        #[source]
        source: std::io::Error,
    },
    #[error("cursor persistence error: {0}")]
    Cursor(#[from] crate::cursor::CursorError),
    #[error("operation not supported on this platform; freally-journal-win is Windows-only")]
    UnsupportedPlatform,
}
