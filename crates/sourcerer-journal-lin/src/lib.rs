//! Linux inotify+fanotify journal subscriber for Sourcerer (Phase 3).
//!
//! Public surface mirrors `sourcerer-journal-win` (Phase 1) and
//! `sourcerer-journal-mac` (Phase 2) so the facade in
//! `sourcerer-journal` re-exports the same shape on every OS:
//!
//! ```ignore
//! use sourcerer_journal_lin::{open, JournalEvent};
//! use std::path::Path;
//! use futures::StreamExt;
//!
//! # async fn demo() -> Result<(), sourcerer_journal_lin::JournalError> {
//! let sub = open(Path::new("/home/me/Documents"))?;
//! let mut bootstrap = Box::pin(sub.bootstrap());
//! while let Some(ev) = bootstrap.next().await {
//!     // seed the index
//! }
//! let mut realtime = Box::pin(sub.subscribe());
//! while let Some(ev) = realtime.next().await {
//!     // apply the event
//! }
//! # Ok(()) }
//! ```
//!
//! Builds compile cleanly on Windows + macOS too — the inotify /
//! fanotify-backed modules are gated behind `cfg(target_os = "linux")`
//! and the rest of the workspace sees a typed-but-stubbed surface.
//!
//! ## Backend choice
//!
//! The subscriber picks between two backends at `open()` time:
//!
//! - **inotify** (default): `inotify_add_watch` recursively across every
//!   directory under the root. No privileges required. The chosen
//!   backend is recorded in the cursor JSON for diagnostics.
//! - **fanotify** (CAP_SYS_ADMIN required): one `fanotify_mark` call
//!   covers the entire mount with `FAN_REPORT_DFID_NAME`, giving
//!   correct rename tracking on overlayfs / Btrfs subvolume crossings.
//!
//! The daemon's UI surfaces a "fanotify upgrade available" banner when
//! `CAP_SYS_ADMIN` is missing; the polkit policy at
//! `/usr/share/polkit-1/actions/io.mikeweaver.sourcerer.policy` brokers
//! the elevation.

pub mod cursor;
pub mod event;
pub mod flags;

#[cfg(target_os = "linux")]
pub mod ffi;
#[cfg(target_os = "linux")]
pub mod subscriber;

pub use cursor::{CursorError, WatchBackend, WatchCursor};
pub use event::{JournalError, JournalEvent};

#[cfg(target_os = "linux")]
pub use subscriber::{JournalSubscriber, open, open_with_cursor_root};

/// Stub `open()` for non-Linux hosts so the workspace builds cross-OS.
/// The real subscriber is Linux-only by design.
#[cfg(not(target_os = "linux"))]
pub fn open(_root: &std::path::Path) -> Result<JournalSubscriber, JournalError> {
    Err(JournalError::UnsupportedPlatform)
}

/// `open()` test variant. Stubbed on non-Linux for the same reason as
/// [`open`].
#[cfg(not(target_os = "linux"))]
pub fn open_with_cursor_root(
    _root: &std::path::Path,
    _cursor_root: &std::path::Path,
) -> Result<JournalSubscriber, JournalError> {
    Err(JournalError::UnsupportedPlatform)
}

#[cfg(not(target_os = "linux"))]
pub struct JournalSubscriber {
    _private: (),
}

#[cfg(not(target_os = "linux"))]
impl JournalSubscriber {
    pub fn bootstrap(&self) -> impl futures::Stream<Item = JournalEvent> + Send + 'static {
        futures::stream::empty()
    }

    pub fn subscribe(&self) -> impl futures::Stream<Item = JournalEvent> + Send + 'static {
        futures::stream::empty()
    }

    pub fn root(&self) -> &std::path::Path {
        std::path::Path::new("")
    }

    pub fn cursor(&self) -> WatchCursor {
        WatchCursor {
            root: std::path::PathBuf::new(),
            device: 0,
            fs_name: String::new(),
            backend: WatchBackend::Inotify,
            bootstrap_complete: false,
            last_event_time_ns: 0,
        }
    }
}
