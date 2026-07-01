//! [`Pipeline`] — the dispatcher that routes an `(path, magic_bytes)`
//! pair to the first registered [`Extractor`](crate::Extractor) that
//! claims it.
//!
//! Phase 7 ships the framework only; Phase 8 brings the actual
//! extractors and registers them with [`PipelineBuilder::register`]
//! at startup. The pipeline:
//!
//!   * Owns an `Arc<dyn Extractor>` per registration.
//!   * Honors per-extractor [`ExtractorMode`](crate::ExtractorMode)
//!     overrides — a `Disabled` extractor never dispatches.
//!   * Reads the first [`MAGIC_HEAD_BYTES`] of the candidate file via
//!     [`Pipeline::read_magic`] when the caller doesn't already have
//!     a magic slice. This is the *only* I/O the pipeline does on the
//!     dispatch path.
//!
//! Extension handling lives inside each `Extractor::matches`
//! implementation — the framework hands every extractor `(path, magic)`
//! and lets each one decide whether the suffix matters. That keeps the
//! framework agnostic to format-specific quirks (a `.docx` is a zip,
//! so the docx extractor wants both the `.docx` extension *and* the
//! `PK\x03\x04` magic to claim it).

use std::path::Path;
use std::sync::Arc;

use parking_lot::RwLock;

use crate::settings::{ExtractorMode, PipelineSettings};
use crate::{Extractor, ExtractorId};

/// Bytes the dispatcher reads from the head of every candidate file.
/// Big enough to cover every magic prefix Phase 8 cares about
/// (PDF / ZIP / OOXML / 7z / PNG / OGG / FLAC / WebP), small enough
/// that the read is essentially free.
pub const MAGIC_HEAD_BYTES: usize = 32;

/// Compile-time registration step. The framework owns no extractors of
/// its own — Phase 8 calls `Pipeline::builder().register(...)` at
/// daemon startup.
pub struct PipelineBuilder {
    extractors: Vec<Arc<dyn Extractor>>,
    settings: PipelineSettings,
}

impl PipelineBuilder {
    fn new(settings: PipelineSettings) -> Self {
        Self {
            extractors: Vec::new(),
            settings,
        }
    }

    /// Append an extractor. Registration order is dispatch order — the
    /// first extractor whose `matches()` returns true wins.
    pub fn register<E>(mut self, extractor: E) -> Self
    where
        E: Extractor + 'static,
    {
        self.extractors.push(Arc::new(extractor));
        self
    }

    /// Append an extractor that's already wrapped in `Arc` — useful for
    /// shared extractors (e.g. a tree-sitter parser pool that multiple
    /// language extractors share).
    pub fn register_arc(mut self, extractor: Arc<dyn Extractor>) -> Self {
        self.extractors.push(extractor);
        self
    }

    pub fn build(self) -> Pipeline {
        Pipeline {
            extractors: self.extractors,
            settings: Arc::new(RwLock::new(self.settings)),
        }
    }
}

#[derive(Clone)]
pub struct Pipeline {
    extractors: Vec<Arc<dyn Extractor>>,
    settings: Arc<RwLock<PipelineSettings>>,
}

impl Pipeline {
    pub fn builder() -> PipelineBuilder {
        PipelineBuilder::new(PipelineSettings::default())
    }

    pub fn builder_with_settings(settings: PipelineSettings) -> PipelineBuilder {
        PipelineBuilder::new(settings)
    }

    /// Number of registered extractors (live + disabled). Diagnostic.
    pub fn extractor_count(&self) -> usize {
        self.extractors.len()
    }

    /// Read up to [`MAGIC_HEAD_BYTES`] from the head of `path`. The
    /// dispatcher uses this; callers can use it too when they want to
    /// pre-feed the dispatch path (e.g. an in-memory blob).
    ///
    /// Loops on partial reads — `Read::read` is allowed to return
    /// fewer bytes than requested even when more are available
    /// (Windows `ReadFile` and Linux readahead-pressure are the usual
    /// suspects). We keep reading until the buffer is full or EOF
    /// flips us out.
    pub fn read_magic(path: &Path) -> std::io::Result<Vec<u8>> {
        use std::fs::File;
        use std::io::Read;
        // Stack-allocated; one heap allocation per dispatch (the
        // returned `Vec`) instead of two. Phase 13 perf pass evaluates
        // returning a `[u8; MAGIC_HEAD_BYTES]` directly to drop that
        // last allocation.
        let mut buf = [0u8; MAGIC_HEAD_BYTES];
        let mut f = File::open(path)?;
        let mut filled = 0;
        while filled < buf.len() {
            match f.read(&mut buf[filled..])? {
                0 => break,
                n => filled += n,
            }
        }
        Ok(buf[..filled].to_vec())
    }

    /// Look up the dispatch winner for `(path, magic)`. Skips extractors
    /// whose effective mode is `Disabled`. Returns `None` if no
    /// extractor claims the file — the daemon should treat that as
    /// "filename-only, no content lens" rather than an error.
    pub fn dispatch(&self, path: &Path, magic: &[u8]) -> Option<Arc<dyn Extractor>> {
        let settings = self.settings.read();
        for ex in &self.extractors {
            let id = ex.id();
            if matches!(settings.effective_mode(id), ExtractorMode::Disabled) {
                continue;
            }
            if ex.matches(path, magic) {
                return Some(Arc::clone(ex));
            }
        }
        None
    }

    /// Convenience: read magic from `path` then dispatch. Returns
    /// `None` on a no-match *and* on any I/O error reading magic
    /// (treated equivalently — the daemon falls back to filename-only).
    /// I/O errors are logged at `debug` level rather than warn so a
    /// noisy permissions-denied folder doesn't drown the indexd log.
    pub fn dispatch_path(&self, path: &Path) -> Option<Arc<dyn Extractor>> {
        let magic = match Self::read_magic(path) {
            Ok(m) => m,
            Err(e) => {
                tracing::debug!(?e, path = %path.display(), "magic-read failed; skipping dispatch");
                return None;
            }
        };
        self.dispatch(path, &magic)
    }

    /// Effective mode for an extractor — settings override > pipeline
    /// default. Diagnostic + queue-scheduling helper.
    pub fn mode_for(&self, id: ExtractorId) -> ExtractorMode {
        self.settings.read().effective_mode(id)
    }

    /// Replace the settings snapshot. The `IndexD` daemon calls this
    /// when the user toggles a per-extractor mode in the UI; new
    /// dispatches see the new settings immediately.
    pub fn replace_settings(&self, settings: PipelineSettings) {
        *self.settings.write() = settings;
    }

    pub fn settings_snapshot(&self) -> PipelineSettings {
        self.settings.read().clone()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::sink::TextSink;
    use crate::{ExtractError, ExtractionStats, ExtractorId};
    use std::path::Path;

    struct ExtA;
    struct ExtB;
    struct NoOp;

    impl Extractor for ExtA {
        fn id(&self) -> ExtractorId {
            ExtractorId::new("a")
        }
        fn matches(&self, path: &Path, _magic: &[u8]) -> bool {
            path.extension().and_then(|e| e.to_str()) == Some("a")
        }
        fn extract(
            &self,
            _path: &Path,
            sink: &mut TextSink,
        ) -> Result<ExtractionStats, ExtractError> {
            sink.push_str("a")
                .map_err(|_| ExtractError::OutputTooLarge { cap: sink.cap() })?;
            Ok(ExtractionStats::default())
        }
    }

    impl Extractor for ExtB {
        fn id(&self) -> ExtractorId {
            ExtractorId::new("b")
        }
        fn matches(&self, _path: &Path, magic: &[u8]) -> bool {
            magic.starts_with(b"BBBB")
        }
        fn extract(
            &self,
            _path: &Path,
            sink: &mut TextSink,
        ) -> Result<ExtractionStats, ExtractError> {
            sink.push_str("b")
                .map_err(|_| ExtractError::OutputTooLarge { cap: sink.cap() })?;
            Ok(ExtractionStats::default())
        }
    }

    impl Extractor for NoOp {
        fn id(&self) -> ExtractorId {
            ExtractorId::new("noop")
        }
        fn matches(&self, _path: &Path, _magic: &[u8]) -> bool {
            true
        }
        fn extract(
            &self,
            _path: &Path,
            _sink: &mut TextSink,
        ) -> Result<ExtractionStats, ExtractError> {
            Ok(ExtractionStats::default())
        }
    }

    #[test]
    fn first_match_wins() {
        let p = Pipeline::builder().register(ExtA).register(NoOp).build();
        let hit = p.dispatch(Path::new("/tmp/foo.a"), b"some bytes").unwrap();
        assert_eq!(hit.id(), ExtractorId::new("a"));
    }

    #[test]
    fn extension_then_magic_dispatch() {
        let p = Pipeline::builder().register(ExtA).register(ExtB).build();
        let by_ext = p.dispatch(Path::new("/tmp/foo.a"), b"BBBB junk").unwrap();
        assert_eq!(by_ext.id(), ExtractorId::new("a"));
        let by_magic = p
            .dispatch(Path::new("/tmp/whatever"), b"BBBB junk")
            .unwrap();
        assert_eq!(by_magic.id(), ExtractorId::new("b"));
    }

    #[test]
    fn no_match_returns_none() {
        let p = Pipeline::builder().register(ExtA).register(ExtB).build();
        assert!(p.dispatch(Path::new("/tmp/x.txt"), b"plain text").is_none());
    }

    #[test]
    fn disabled_extractor_skipped() {
        let mut settings = PipelineSettings::default();
        settings.set_mode(ExtractorId::new("a"), ExtractorMode::Disabled);
        let p = Pipeline::builder_with_settings(settings)
            .register(ExtA)
            .register(NoOp)
            .build();
        let hit = p.dispatch(Path::new("/tmp/foo.a"), b"").unwrap();
        // Disabled "a" was skipped; NoOp catches everything.
        assert_eq!(hit.id(), ExtractorId::new("noop"));
    }

    #[test]
    fn replace_settings_round_trips() {
        let p = Pipeline::builder().register(ExtA).register(NoOp).build();
        let mut s = p.settings_snapshot();
        s.set_mode(ExtractorId::new("a"), ExtractorMode::Disabled);
        p.replace_settings(s);
        let hit = p.dispatch(Path::new("/tmp/foo.a"), b"").unwrap();
        assert_eq!(hit.id(), ExtractorId::new("noop"));
    }
}
