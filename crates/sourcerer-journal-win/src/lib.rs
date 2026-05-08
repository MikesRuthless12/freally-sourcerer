//! Windows NTFS USN journal subscriber for Sourcerer (Phase 1).
//!
//! Public surface (mirrored later by `sourcerer-journal-mac` in Phase 2 and
//! `sourcerer-journal-lin` in Phase 3):
//!
//! ```ignore
//! use sourcerer_journal_win::{open, JournalEvent};
//! use std::path::Path;
//! use futures::StreamExt;
//!
//! # async fn demo() -> Result<(), sourcerer_journal_win::JournalError> {
//! let sub = open(Path::new("C:\\"))?;
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

pub mod cursor;
pub mod event;
pub mod reasons;

#[cfg(windows)]
pub mod ffi;
#[cfg(windows)]
pub mod subscriber;

pub use cursor::{CursorError, VolumeCursor};
pub use event::{JournalError, JournalEvent};

#[cfg(windows)]
pub use subscriber::{JournalSubscriber, open, open_with_cursor_root};

/// Stub `open()` for non-Windows hosts so the workspace builds cross-OS.
/// The real subscriber is Windows-only by design.
#[cfg(not(windows))]
pub fn open(_volume: &std::path::Path) -> Result<JournalSubscriber, JournalError> {
    Err(JournalError::UnsupportedPlatform)
}

#[cfg(not(windows))]
pub struct JournalSubscriber {
    _private: (),
}

#[cfg(not(windows))]
impl JournalSubscriber {
    pub fn bootstrap(&self) -> impl futures::Stream<Item = JournalEvent> + Send + 'static {
        futures::stream::empty()
    }
    pub fn subscribe(&self) -> impl futures::Stream<Item = JournalEvent> + Send + 'static {
        futures::stream::empty()
    }
}
