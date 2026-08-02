//! `JournalEvent` + `JournalError` — the Phase-3 Linux public surface.
//!
//! Mirrored from `freally-journal-win` so the facade in `freally-journal`
//! re-exports a single shape on every OS. `JournalEvent` is byte-for-byte
//! identical across platforms; `JournalError` carries Linux-specific failure
//! modes (inotify init, fanotify upgrade gating, getdents64 walks).

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
    /// renames. Used by the smoke / integration harness to filter events
    /// down to the watched scratch tree.
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
/// change stream. Mirrors `freally_journal_win::JournalPosition` so the
/// daemon's watcher supervisor can detect a stream recreate / wrap without
/// knowing the per-OS cursor type.
///
/// `generation` identifies the stream itself — a change means the OS threw
/// away the old stream, so any events between the two are lost and the index
/// needs a rebuild. `offset` is the monotonic position within that stream.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct JournalPosition {
    /// `stat.st_dev` of the watched root.
    pub generation: u64,
    /// Wall-clock nanoseconds of the last emitted event. Advisory only —
    /// neither inotify nor fanotify has a resumable cursor.
    pub offset: u64,
}

#[derive(Debug, thiserror::Error)]
pub enum JournalError {
    #[error("watch root must be an absolute, existing directory: {0}")]
    InvalidRoot(PathBuf),
    #[error("inotify_init1 failed: {0}")]
    InotifyInit(#[source] std::io::Error),
    #[error("inotify_add_watch on `{path}` failed: {source}")]
    InotifyAddWatch {
        path: PathBuf,
        #[source]
        source: std::io::Error,
    },
    #[error("inotify read failed: {0}")]
    InotifyRead(#[source] std::io::Error),
    #[error("fanotify_init failed: {0}")]
    FanotifyInit(#[source] std::io::Error),
    #[error("fanotify_mark on `{path}` failed: {source}")]
    FanotifyMark {
        path: PathBuf,
        #[source]
        source: std::io::Error,
    },
    /// fanotify upgrade requires CAP_SYS_ADMIN. The default code path does
    /// NOT surface this as an error — `open()` falls back to inotify and
    /// the daemon advertises the upgrade option through the indexd UI. The
    /// variant exists so a caller that explicitly opted into fanotify
    /// (Phase 13's `force_fanotify=true` knob) gets a distinct error.
    #[error(
        "fanotify upgrade requires CAP_SYS_ADMIN; falling back to inotify. \
         Run `pkexec freally-indexd elevate` (polkit) to enable fanotify."
    )]
    MissingCapability,
    #[error("opening watch root `{0}` failed: {1}")]
    OpenRoot(PathBuf, #[source] std::io::Error),
    #[error("statfs(`{0}`) failed: {1}")]
    Statfs(PathBuf, #[source] std::io::Error),
    #[error("getdents64 walk of `{0}` failed: {1}")]
    WalkFailed(PathBuf, #[source] std::io::Error),
    #[error("cursor persistence error: {0}")]
    Cursor(#[from] crate::cursor::CursorError),
    #[error("operation not supported on this platform; freally-journal-lin is Linux-only")]
    UnsupportedPlatform,
}
