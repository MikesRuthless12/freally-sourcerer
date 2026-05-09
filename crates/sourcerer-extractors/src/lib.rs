//! Sourcerer extractor framework (Phase 7).
//!
//! Phase 7 ships the **framework** — the trait, the dispatcher, the
//! per-extraction sandbox, the bounded extraction queue, the
//! content-addressed blob store, and the per-extractor mode (Lazy /
//! Eager / Disabled). Phase 8 brings the actual extractors (PDF,
//! Office, code, archive-peek, structured data, plain-text + Markdown)
//! and registers them with the dispatcher.
//!
//! ## Public surface (Build Guide Phase 7 prompt)
//!
//! ```ignore
//! pub trait Extractor: Send + Sync {
//!     fn id(&self) -> ExtractorId;
//!     fn matches(&self, path: &Path, magic: &[u8]) -> bool;
//!     fn extract(&self, path: &Path, sink: &mut TextSink)
//!         -> Result<ExtractionStats, ExtractError>;
//! }
//! pub struct Pipeline { /* dispatch by magic-byte + extension */ }
//! ```
//!
//! Layered on top:
//!
//! * [`pipeline::Pipeline`] — registers extractors at compile time,
//!   dispatches by `(path, magic)`, honors per-extractor mode.
//! * [`sandbox::Sandbox`] — per-extraction time + RSS budget. Time
//!   enforced cross-platform via a tokio-style cooperative cancel
//!   flag; RSS via `/proc/self/status` on Linux,
//!   `GetProcessMemoryInfo` on Windows, no-op on macOS (Phase 7
//!   prompt). Extractors check `TextSink::is_cancelled` between major
//!   work items to respect the budget.
//! * [`queue::ExtractionQueue`] — bounded priority queue, recently-
//!   touched first, back-pressure surfaces as `QueueError::Full`.
//! * [`blob::BlobStore`] — content-addressed, zstd-compressed,
//!   atomic-write blob store at `<index_root>/extracted/`.
//! * [`settings::PipelineSettings`] — per-extractor Lazy / Eager /
//!   Disabled mode, time / memory / sink budgets, queue capacity.
//!   JSON-serializable so Phase 12 can round-trip through the
//!   settings dialog.
//!
//! ## Cooperation contract
//!
//! Phase 8 extractors **must** check `sink.is_cancelled()` on coarse
//! boundaries (per-page, per-row, per-archive-entry) and bail with
//! `ExtractError::Cancelled` when set. Non-cooperative extractors
//! leak a worker thread per budget breach — the supervisor returns
//! `SandboxError::TimeBudget` after a short grace window even if the
//! worker never yields. Phase 13 evaluates subprocess isolation for
//! third-party extractors that don't hold up their end of the
//! contract.

#![deny(rust_2018_idioms)]

use std::path::Path;
use std::time::Duration;

pub mod blob;
pub mod error;
pub mod extractors;
pub mod pipeline;
pub mod queue;
pub mod sandbox;
pub mod settings;
pub mod sink;

pub use blob::{BlobId, BlobStore, BlobStoreError, BlobStoreStats, DEFAULT_ZSTD_LEVEL};
pub use error::{ExtractError, SandboxError};
pub use pipeline::{MAGIC_HEAD_BYTES, Pipeline, PipelineBuilder};
pub use queue::{ExtractionQueue, ExtractionRequest, QueueError};
pub use sandbox::{
    CANCEL_GRACE, SUPERVISOR_TICK, Sandbox, SandboxConfig, SandboxOutput, current_rss,
};
pub use settings::{
    DEFAULT_MEMORY_CEILING_BYTES, DEFAULT_QUEUE_CAPACITY, DEFAULT_TEXT_CAP_BYTES,
    DEFAULT_TIME_BUDGET, ExtractorMode, PipelineSettings, SettingsError,
};
pub use sink::{DEFAULT_TEXT_CAP_BYTES as SINK_DEFAULT_CAP, SinkOverflow, TextSink};

/// Stable identifier for an extractor. Phase 8 extractors instantiate
/// these as `ExtractorId::new("pdf")`, `ExtractorId::new("docx")`,
/// etc. The string content is the *settings key* — round-tripped
/// through [`PipelineSettings`]'s JSON.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct ExtractorId(&'static str);

impl ExtractorId {
    pub const fn new(s: &'static str) -> Self {
        Self(s)
    }

    pub fn as_str(self) -> &'static str {
        self.0
    }
}

impl std::fmt::Display for ExtractorId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.0)
    }
}

/// Stats one extraction reports back. Phase 8 extractors fill these
/// in; the framework forwards them to the daemon's status pane.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct ExtractionStats {
    /// Bytes the extractor *read* from the source file.
    pub bytes_in: u64,
    /// UTF-8 text bytes the extractor *wrote* into the sink.
    pub bytes_out: u64,
    /// Wall-clock time the extractor spent in `extract()`. Set to
    /// `Duration::ZERO` when the extractor doesn't measure it; the
    /// sandbox computes its own outer measurement and uses that for
    /// daemon reporting.
    pub elapsed: Duration,
}

/// The trait every Phase 8 extractor implements.
///
/// Implementors must be `Send + Sync` so the dispatcher can hold them
/// in `Arc<dyn Extractor>` and the sandbox can ship them across the
/// worker-thread boundary. They must be cheap to clone-as-Arc — store
/// any heavy state (parser pools, dictionaries) inside the impl, not
/// per-call.
pub trait Extractor: Send + Sync {
    /// Stable identifier used as the settings key.
    fn id(&self) -> ExtractorId;

    /// Cheap pre-flight: does this extractor want to claim
    /// `(path, magic)`? Called by the dispatcher in registration order;
    /// the first `true` wins. Implementors should consult both the
    /// path extension *and* the leading magic bytes — file names lie,
    /// magic bytes generally don't.
    fn matches(&self, path: &Path, magic: &[u8]) -> bool;

    /// Pull text from `path` into `sink`. Implementors should:
    ///
    /// 1. Loop in coarse iterations (per-page, per-row, per-entry).
    /// 2. Check `sink.is_cancelled()` at the top of each iteration
    ///    and return `ExtractError::Cancelled` when the flag is set.
    /// 3. Convert any `SinkOverflow` from `sink.push_str` /
    ///    `sink.push_bytes` into `ExtractError::OutputTooLarge`.
    /// 4. Surface format-specific failures as `Malformed` (parse
    ///    failure) or `Unsupported` (feature not handled).
    fn extract(&self, path: &Path, sink: &mut TextSink) -> Result<ExtractionStats, ExtractError>;
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::Arc;

    struct StubExt;
    impl Extractor for StubExt {
        fn id(&self) -> ExtractorId {
            ExtractorId::new("stub")
        }
        fn matches(&self, _p: &Path, _m: &[u8]) -> bool {
            true
        }
        fn extract(&self, _p: &Path, sink: &mut TextSink) -> Result<ExtractionStats, ExtractError> {
            sink.push_str("stub")
                .map_err(|_| ExtractError::OutputTooLarge { cap: sink.cap() })?;
            Ok(ExtractionStats {
                bytes_out: sink.len() as u64,
                ..Default::default()
            })
        }
    }

    #[test]
    fn extractor_is_object_safe() {
        let _: Arc<dyn Extractor> = Arc::new(StubExt);
    }

    #[test]
    fn extractor_id_round_trip() {
        let id = ExtractorId::new("alpha");
        assert_eq!(id.as_str(), "alpha");
        assert_eq!(format!("{id}"), "alpha");
    }
}
