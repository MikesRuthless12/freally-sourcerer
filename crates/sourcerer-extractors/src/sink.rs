//! Bounded byte sink used as the extractor → framework pipe.
//!
//! Every extractor receives a `&mut TextSink` from the framework and
//! pushes UTF-8 text into it. The sink:
//!
//!   1. Enforces a hard byte cap. Writes past the cap return
//!      [`SinkOverflow`]; the extractor must surface this as
//!      [`ExtractError::OutputTooLarge`](crate::ExtractError::OutputTooLarge).
//!   2. Holds the sandbox's cancel flag. Extractors that loop over
//!      large inputs should call [`TextSink::is_cancelled`] periodically
//!      and bail with [`ExtractError::Cancelled`](crate::ExtractError::Cancelled)
//!      when it returns true. The framework wires this up; standalone
//!      tests can pass a sink with no cancel flag (`is_cancelled` returns
//!      false forever).
//!
//! The sink intentionally does *not* implement `std::io::Write` —
//! `Write` swallows the cap-exceeded signal as a generic `io::Error`,
//! and we want a typed error path so the dispatcher can apply a
//! single retry/skip policy.

use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};

use thiserror::Error;

/// Hard byte cap on the sink. Phase 8 extractors set this per-format;
/// the framework's default is 16 MiB which is well above plausible
/// per-document text but small enough that a runaway extractor can't
/// fill the disk.
pub const DEFAULT_TEXT_CAP_BYTES: usize = 16 * 1024 * 1024;

#[derive(Debug, Error)]
#[error("sink full at cap {cap} bytes")]
pub struct SinkOverflow {
    pub cap: usize,
}

pub struct TextSink {
    buf: Vec<u8>,
    cap: usize,
    cancel: Option<Arc<AtomicBool>>,
}

impl TextSink {
    pub fn new(cap: usize) -> Self {
        Self {
            buf: Vec::with_capacity(cap.min(64 * 1024)),
            cap,
            cancel: None,
        }
    }

    pub fn with_cancel(cap: usize, cancel: Arc<AtomicBool>) -> Self {
        Self {
            buf: Vec::with_capacity(cap.min(64 * 1024)),
            cap,
            cancel: Some(cancel),
        }
    }

    pub fn cap(&self) -> usize {
        self.cap
    }

    pub fn len(&self) -> usize {
        self.buf.len()
    }

    pub fn is_empty(&self) -> bool {
        self.buf.is_empty()
    }

    /// Snapshot the canceled flag without taking the atomic on every
    /// byte. Extractors should call this on a coarse boundary
    /// (per-page, per-MB read, etc.).
    pub fn is_cancelled(&self) -> bool {
        self.cancel
            .as_ref()
            .is_some_and(|c| c.load(Ordering::Relaxed))
    }

    /// Append a UTF-8 string. Returns `Err(SinkOverflow)` if the cap
    /// would be exceeded; on overflow the partial write is *not*
    /// applied so the buffer stays bounded.
    pub fn push_str(&mut self, s: &str) -> Result<(), SinkOverflow> {
        self.push_bytes(s.as_bytes())
    }

    /// Append raw bytes. Phase 8 extractors should prefer
    /// [`push_str`](Self::push_str) — Phase 7 keeps this byte-level
    /// entry-point so binary-text formats (e.g. archive listings with
    /// embedded NULs from libarchive) can write through the same sink
    /// without a UTF-8 round-trip.
    pub fn push_bytes(&mut self, bytes: &[u8]) -> Result<(), SinkOverflow> {
        if self.buf.len().saturating_add(bytes.len()) > self.cap {
            return Err(SinkOverflow { cap: self.cap });
        }
        self.buf.extend_from_slice(bytes);
        Ok(())
    }

    pub fn into_bytes(self) -> Vec<u8> {
        self.buf
    }

    pub fn as_bytes(&self) -> &[u8] {
        &self.buf
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn writes_below_cap_succeed() {
        let mut sink = TextSink::new(16);
        sink.push_str("hello").unwrap();
        sink.push_str(" world").unwrap();
        assert_eq!(sink.as_bytes(), b"hello world");
    }

    #[test]
    fn write_at_cap_succeeds() {
        let mut sink = TextSink::new(5);
        sink.push_str("hello").unwrap();
        assert_eq!(sink.len(), 5);
    }

    #[test]
    fn write_past_cap_overflows_and_preserves_partial_state() {
        let mut sink = TextSink::new(8);
        sink.push_str("hello").unwrap();
        let err = sink.push_str(" world").unwrap_err();
        assert_eq!(err.cap, 8);
        // Partial write must not have applied.
        assert_eq!(sink.as_bytes(), b"hello");
    }

    #[test]
    fn cancel_flag_round_trips() {
        let cancel = Arc::new(AtomicBool::new(false));
        let sink = TextSink::with_cancel(16, Arc::clone(&cancel));
        assert!(!sink.is_cancelled());
        cancel.store(true, Ordering::Relaxed);
        assert!(sink.is_cancelled());
    }

    #[test]
    fn no_cancel_flag_is_never_cancelled() {
        let sink = TextSink::new(16);
        assert!(!sink.is_cancelled());
    }
}
