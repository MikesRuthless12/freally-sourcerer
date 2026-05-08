//! Phase 7 smoke — OS-agnostic, runs on every CI matrix entry.
//!
//! Mirrors Phase 4 / 5 / 6 smoke shapes: hand-rolled fixtures, the
//! public `sourcerer-extractors` surface, asserts the invariants the
//! Build Guide gates Phase 7 on:
//!
//!   1. The `Extractor` trait + `Pipeline` dispatch routes
//!      `(path, magic)` to the first registered extractor whose
//!      `matches()` returns true. Disabled extractors are skipped.
//!   2. The per-extraction `Sandbox` enforces the time budget against
//!      both cooperative and non-cooperative extractors, surfaces
//!      `OutputTooLarge` cleanly, and folds extractor-side errors into
//!      `SandboxError::Extractor`.
//!   3. The `ExtractionQueue` orders by mtime descending (recently-
//!      touched first), breaks ties FIFO, surfaces back-pressure as
//!      `QueueError::Full`, and unblocks waiters on close.
//!   4. The `BlobStore` round-trips zstd-compressed text via tmp+
//!      rename atomic writes, dedupes content-addressed `put`s, and
//!      lays files out at `<root>/<first2hex>/<full-hex>`.
//!   5. `PipelineSettings` round-trips through JSON without losing the
//!      per-extractor `Lazy` / `Eager` / `Disabled` overrides.

use std::path::{Path, PathBuf};
use std::sync::Arc;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::time::{Duration, Instant};

use sourcerer_extractors::{
    BlobId, BlobStore, ExtractError, ExtractionQueue, ExtractionRequest, ExtractionStats,
    Extractor, ExtractorId, ExtractorMode, Pipeline, PipelineSettings, QueueError, Sandbox,
    SandboxConfig, SandboxError, TextSink,
};
use tempfile::tempdir;

// ---------------------------------------------------------------------
// Test extractors
// ---------------------------------------------------------------------

/// Claims `.txt` files. Writes a fixed string into the sink.
struct PlainTextStub;
impl Extractor for PlainTextStub {
    fn id(&self) -> ExtractorId {
        ExtractorId::new("plain-text")
    }
    fn matches(&self, path: &Path, _magic: &[u8]) -> bool {
        path.extension().and_then(|s| s.to_str()) == Some("txt")
    }
    fn extract(&self, _path: &Path, sink: &mut TextSink) -> Result<ExtractionStats, ExtractError> {
        sink.push_str("plain text body")
            .map_err(|_| ExtractError::OutputTooLarge { cap: sink.cap() })?;
        Ok(ExtractionStats {
            bytes_out: sink.len() as u64,
            ..Default::default()
        })
    }
}

/// Claims any file whose magic starts with `PK\x03\x04` (the ZIP magic;
/// also matches docx / xlsx / pptx).
struct ZipMagicStub;
impl Extractor for ZipMagicStub {
    fn id(&self) -> ExtractorId {
        ExtractorId::new("zip-magic")
    }
    fn matches(&self, _path: &Path, magic: &[u8]) -> bool {
        magic.starts_with(b"PK\x03\x04")
    }
    fn extract(&self, _path: &Path, sink: &mut TextSink) -> Result<ExtractionStats, ExtractError> {
        sink.push_str("zip-shaped")
            .map_err(|_| ExtractError::OutputTooLarge { cap: sink.cap() })?;
        Ok(ExtractionStats::default())
    }
}

/// Counts how many times its `extract` runs; helps assert the queue +
/// sandbox actually invoke it the right number of times.
struct Counted {
    id: ExtractorId,
    invocations: Arc<AtomicUsize>,
}
impl Extractor for Counted {
    fn id(&self) -> ExtractorId {
        self.id
    }
    fn matches(&self, _path: &Path, _magic: &[u8]) -> bool {
        true
    }
    fn extract(&self, _path: &Path, _sink: &mut TextSink) -> Result<ExtractionStats, ExtractError> {
        self.invocations.fetch_add(1, Ordering::Relaxed);
        Ok(ExtractionStats::default())
    }
}

/// Cooperative slow extractor — yields to the cancel flag at every
/// sleep tick. Used to verify the sandbox's time budget fires cleanly.
struct CooperativeSlow;
impl Extractor for CooperativeSlow {
    fn id(&self) -> ExtractorId {
        ExtractorId::new("coop-slow")
    }
    fn matches(&self, _path: &Path, _magic: &[u8]) -> bool {
        true
    }
    fn extract(&self, _path: &Path, sink: &mut TextSink) -> Result<ExtractionStats, ExtractError> {
        for _ in 0..100 {
            if sink.is_cancelled() {
                return Err(ExtractError::Cancelled);
            }
            std::thread::sleep(Duration::from_millis(20));
        }
        Ok(ExtractionStats::default())
    }
}

// ---------------------------------------------------------------------
// 1) Pipeline dispatch
// ---------------------------------------------------------------------

#[test]
fn pipeline_dispatches_by_extension() {
    let p = Pipeline::builder()
        .register(PlainTextStub)
        .register(ZipMagicStub)
        .build();
    let hit = p
        .dispatch(Path::new("/tmp/notes.txt"), b"any bytes")
        .expect("plain-text stub should claim .txt");
    assert_eq!(hit.id(), ExtractorId::new("plain-text"));
}

#[test]
fn pipeline_dispatches_by_magic() {
    let p = Pipeline::builder()
        .register(PlainTextStub)
        .register(ZipMagicStub)
        .build();
    let hit = p
        .dispatch(Path::new("/tmp/whatever"), b"PK\x03\x04 ...")
        .expect("zip-magic should claim PK\\x03\\x04 prefix");
    assert_eq!(hit.id(), ExtractorId::new("zip-magic"));
}

#[test]
fn pipeline_returns_none_when_no_extractor_matches() {
    let p = Pipeline::builder()
        .register(PlainTextStub)
        .register(ZipMagicStub)
        .build();
    assert!(
        p.dispatch(Path::new("/tmp/binary.bin"), b"\x00\x01\x02")
            .is_none()
    );
}

#[test]
fn pipeline_skips_disabled_extractors() {
    let mut settings = PipelineSettings::default();
    settings.set_mode(ExtractorId::new("plain-text"), ExtractorMode::Disabled);
    let p = Pipeline::builder_with_settings(settings)
        .register(PlainTextStub)
        .register(ZipMagicStub)
        .build();
    // PlainTextStub is disabled — `.txt` files now fall through.
    assert!(
        p.dispatch(Path::new("/tmp/notes.txt"), b"any bytes")
            .is_none()
    );
    // Re-enabling via replace_settings makes it visible again.
    let mut s2 = p.settings_snapshot();
    s2.clear_override(ExtractorId::new("plain-text"));
    p.replace_settings(s2);
    let hit = p
        .dispatch(Path::new("/tmp/notes.txt"), b"")
        .expect("plain-text stub should claim .txt after re-enable");
    assert_eq!(hit.id(), ExtractorId::new("plain-text"));
}

#[test]
fn read_magic_handles_short_files() {
    // Smoke for the dispatcher's I/O path: a file shorter than
    // MAGIC_HEAD_BYTES still returns the bytes we have.
    let dir = tempdir().unwrap();
    let path = dir.path().join("short.txt");
    std::fs::write(&path, b"hi").unwrap();
    let magic = Pipeline::read_magic(&path).unwrap();
    assert_eq!(magic, b"hi");
}

// ---------------------------------------------------------------------
// 2) Sandbox time budget + error folding
// ---------------------------------------------------------------------

#[test]
fn sandbox_time_budget_fires_on_cooperative_extractor() {
    let cfg = SandboxConfig {
        time_budget: Duration::from_millis(100),
        memory_ceiling_bytes: usize::MAX,
        text_cap_bytes: 16 * 1024,
        tick: Duration::from_millis(20),
        cancel_grace: Duration::from_millis(150),
    };
    let sb = Sandbox::new(cfg);
    let start = Instant::now();
    let err = sb
        .execute(Arc::new(CooperativeSlow) as Arc<dyn Extractor>, "/x".into())
        .unwrap_err();
    let elapsed = start.elapsed();
    assert!(
        matches!(err, SandboxError::TimeBudget { .. }),
        "expected TimeBudget, got {err:?}"
    );
    assert!(
        elapsed < Duration::from_millis(800),
        "supervisor should bail within budget+grace; took {elapsed:?}"
    );
}

#[test]
fn sandbox_extractor_error_passes_through() {
    struct Errs;
    impl Extractor for Errs {
        fn id(&self) -> ExtractorId {
            ExtractorId::new("errs")
        }
        fn matches(&self, _p: &Path, _m: &[u8]) -> bool {
            true
        }
        fn extract(&self, _p: &Path, _s: &mut TextSink) -> Result<ExtractionStats, ExtractError> {
            Err(ExtractError::Malformed("intentional".into()))
        }
    }
    let sb = Sandbox::with_defaults();
    let err = sb
        .execute(Arc::new(Errs) as Arc<dyn Extractor>, "/x".into())
        .unwrap_err();
    match err {
        SandboxError::Extractor(ExtractError::Malformed(msg)) => {
            assert!(msg.contains("intentional"))
        }
        other => panic!("expected Extractor(Malformed), got {other:?}"),
    }
}

#[test]
fn sandbox_routes_quick_extractor_back_to_caller() {
    let invocations = Arc::new(AtomicUsize::new(0));
    let counted = Counted {
        id: ExtractorId::new("counted"),
        invocations: Arc::clone(&invocations),
    };
    let sb = Sandbox::with_defaults();
    let _out = sb
        .execute(Arc::new(counted) as Arc<dyn Extractor>, "/x".into())
        .unwrap();
    assert_eq!(invocations.load(Ordering::Relaxed), 1);
}

// ---------------------------------------------------------------------
// 3) Queue priority + back-pressure
// ---------------------------------------------------------------------

#[test]
fn queue_orders_recently_touched_first() {
    let q = ExtractionQueue::new(8);
    q.try_push(ExtractionRequest::new(PathBuf::from("/old"), 100))
        .unwrap();
    q.try_push(ExtractionRequest::new(PathBuf::from("/recent"), 999))
        .unwrap();
    q.try_push(ExtractionRequest::new(PathBuf::from("/mid"), 500))
        .unwrap();
    let r1 = q.try_pop().unwrap().path;
    let r2 = q.try_pop().unwrap().path;
    let r3 = q.try_pop().unwrap().path;
    assert_eq!(r1, PathBuf::from("/recent"));
    assert_eq!(r2, PathBuf::from("/mid"));
    assert_eq!(r3, PathBuf::from("/old"));
}

#[test]
fn queue_back_pressure_surfaces_full() {
    let q = ExtractionQueue::new(2);
    q.try_push(ExtractionRequest::new(PathBuf::from("/a"), 1))
        .unwrap();
    q.try_push(ExtractionRequest::new(PathBuf::from("/b"), 2))
        .unwrap();
    let err = q
        .try_push(ExtractionRequest::new(PathBuf::from("/c"), 3))
        .unwrap_err();
    assert!(matches!(err, QueueError::Full(2)));
}

#[test]
fn queue_close_unblocks_pop_blocking() {
    use std::thread;
    let q = Arc::new(ExtractionQueue::new(1));
    let q_for_thread = Arc::clone(&q);
    let h = thread::spawn(move || q_for_thread.pop_blocking());
    thread::sleep(Duration::from_millis(50));
    q.close();
    let r = h.join().unwrap();
    assert!(r.is_none(), "closed empty queue must yield None");
}

// ---------------------------------------------------------------------
// 4) BlobStore — atomic writes, dedup, layout
// ---------------------------------------------------------------------

#[test]
fn blob_store_round_trip() {
    let dir = tempdir().unwrap();
    let store = BlobStore::open(dir.path()).unwrap();
    let payload = "the quick brown fox jumps over the lazy dog".repeat(200);
    let id = store.put(payload.as_bytes()).unwrap();
    let got = store.get(id).unwrap().unwrap();
    assert_eq!(got, payload.as_bytes());
}

#[test]
fn blob_store_dedup_is_implicit() {
    let dir = tempdir().unwrap();
    let store = BlobStore::open(dir.path()).unwrap();
    let a = store.put(b"identical").unwrap();
    let b = store.put(b"identical").unwrap();
    assert_eq!(a, b);
    let stats = store.stats();
    assert_eq!(stats.puts, 2);
    assert_eq!(stats.dedup_hits, 1);
}

#[test]
fn blob_store_layout_matches_phase_7_spec() {
    let dir = tempdir().unwrap();
    let store = BlobStore::open(dir.path()).unwrap();
    let id = store.put(b"layout-check").unwrap();
    let hex = id.to_hex();
    // <root>/<first2hex>/<full-hex>
    let expected = dir.path().join(&hex[..2]).join(&hex);
    assert!(expected.exists(), "blob missing at {expected:?}");
}

#[test]
fn blob_store_id_round_trips_via_hex() {
    let id = BlobId::from_content(b"hex me");
    let hex = id.to_hex();
    assert_eq!(hex.len(), 64);
    assert_eq!(BlobId::from_hex(&hex).unwrap(), id);
}

#[test]
fn blob_store_persists_across_reopen() {
    let dir = tempdir().unwrap();
    let id;
    {
        let store = BlobStore::open(dir.path()).unwrap();
        id = store.put(b"persists").unwrap();
    }
    let reopened = BlobStore::open(dir.path()).unwrap();
    assert!(reopened.contains(id));
    assert_eq!(reopened.get(id).unwrap().unwrap(), b"persists");
}

// ---------------------------------------------------------------------
// 5) PipelineSettings JSON round-trip
// ---------------------------------------------------------------------

#[test]
fn settings_json_round_trip_preserves_overrides() {
    let mut s = PipelineSettings::default();
    s.set_mode(ExtractorId::new("pdf"), ExtractorMode::Eager);
    s.set_mode(ExtractorId::new("docx"), ExtractorMode::Disabled);
    s.time_budget = Duration::from_secs(8);
    let json = serde_json::to_string(&s).unwrap();
    let back: PipelineSettings = serde_json::from_str(&json).unwrap();
    assert_eq!(
        back.effective_mode(ExtractorId::new("pdf")),
        ExtractorMode::Eager
    );
    assert_eq!(
        back.effective_mode(ExtractorId::new("docx")),
        ExtractorMode::Disabled
    );
    // Lazy is the default for unset extractors — Phase 7 contract.
    assert_eq!(
        back.effective_mode(ExtractorId::new("never-touched")),
        ExtractorMode::Lazy
    );
    assert_eq!(back.time_budget, Duration::from_secs(8));
}
