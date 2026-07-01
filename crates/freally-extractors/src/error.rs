//! Error types for the extractor framework.
//!
//! Two top-level errors:
//!
//!   * [`ExtractError`] — surfaced *by* an extractor. Anything an
//!     extractor wants to flag (parse failure, unsupported variant,
//!     I/O hiccup) collapses to one of these variants. The framework
//!     itself never constructs an `ExtractError`; callers do.
//!   * [`SandboxError`] — surfaced *by the sandbox* when it had to
//!     terminate the extraction (time budget exceeded, RSS ceiling
//!     exceeded) or on a wrapper failure (worker panicked, sink
//!     overflow caught at the framework level). Wraps an underlying
//!     `ExtractError` when the extractor returned cleanly.

use std::path::PathBuf;

use thiserror::Error;

/// Errors an [`Extractor`](crate::Extractor) can return from its
/// `extract` call. The set is small on purpose: Phase 7 ships the
/// framework, so the variants need to cover every failure mode a Phase 8
/// extractor might surface.
#[derive(Debug, Error)]
pub enum ExtractError {
    /// I/O failure reading the source file or a side resource.
    #[error("extractor I/O at {path}: {source}")]
    Io {
        path: PathBuf,
        #[source]
        source: std::io::Error,
    },

    /// The format was identified but parsing failed mid-stream.
    #[error("malformed input: {0}")]
    Malformed(String),

    /// The format is supported in principle but uses a feature this
    /// extractor doesn't handle yet (e.g. encrypted PDF, password-
    /// protected docx). Distinct from `Malformed` so the dispatcher can
    /// surface a different UI hint.
    #[error("unsupported variant: {0}")]
    Unsupported(String),

    /// The extractor wrote past the per-extraction byte cap configured
    /// on the [`TextSink`](crate::TextSink). The framework converts
    /// this to a [`SandboxError::OutputTooLarge`] when the extraction
    /// is run under a sandbox; standalone callers see it as-is.
    #[error("text output exceeded cap of {cap} bytes")]
    OutputTooLarge { cap: usize },

    /// The extractor noticed the cancel flag and bailed out
    /// cooperatively. Callers running outside a sandbox can ignore
    /// this; sandbox callers see it folded into `SandboxError::TimeBudget`
    /// or `SandboxError::MemoryCeiling`.
    #[error("extraction cancelled")]
    Cancelled,

    /// Catch-all for extractor-side errors that don't fit the variants
    /// above. Use sparingly — a typed variant is preferred so the UI
    /// can surface a meaningful message.
    #[error("extraction failed: {0}")]
    Other(String),
}

impl ExtractError {
    pub fn io<P: Into<PathBuf>>(path: P, source: std::io::Error) -> Self {
        Self::Io {
            path: path.into(),
            source,
        }
    }
}

/// Errors surfaced by the [`Sandbox`](crate::Sandbox) wrapper. The
/// sandbox owns the time + RSS budget; it converts an extractor-side
/// `Cancelled` into the more specific `TimeBudget` / `MemoryCeiling`
/// variants when it knows which budget tripped.
#[derive(Debug, Error)]
pub enum SandboxError {
    /// The extraction ran longer than the configured time budget. The
    /// sandbox set the cancel flag and waited for the worker to bail.
    #[error("time budget of {budget_ms}ms exceeded")]
    TimeBudget { budget_ms: u64 },

    /// Process RSS rose above the configured ceiling during the
    /// extraction. The sandbox set the cancel flag and waited for the
    /// worker to bail.
    #[error("memory ceiling of {ceiling_bytes} bytes exceeded (rss={rss_bytes})")]
    MemoryCeiling {
        ceiling_bytes: usize,
        rss_bytes: usize,
    },

    /// The worker thread panicked. The sandbox catches the panic so
    /// the indexd daemon stays up; the underlying payload is dropped
    /// (panic payload is not `Send + Display` in the general case).
    #[error("extractor worker panicked")]
    WorkerPanic,

    /// `std::thread::Builder::spawn` itself failed — typically a
    /// resource-exhaustion case (out of pthread IDs / TIDs / ulimits).
    /// Distinct from `WorkerPanic` so the daemon's retry policy can
    /// distinguish "wait + retry" from "extractor went off the rails".
    #[error("failed to spawn extractor worker thread: {source}")]
    SpawnFailed {
        #[source]
        source: std::io::Error,
    },

    /// The extractor's text-sink overflow surfaced into the sandbox.
    /// Phase-7 framework converts `ExtractError::OutputTooLarge` to
    /// this variant so the daemon can apply a single retry/skip policy
    /// for "too large" outcomes.
    #[error("text output exceeded cap of {cap} bytes")]
    OutputTooLarge { cap: usize },

    /// The extractor returned a clean [`ExtractError`] within budget.
    /// The sandbox passes it through without re-wrapping the inner
    /// chain.
    #[error(transparent)]
    Extractor(#[from] ExtractError),
}
