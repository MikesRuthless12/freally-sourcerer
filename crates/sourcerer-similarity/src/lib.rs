//! Sourcerer filename similarity lens (Phase 6).
//!
//! Exposes a [`SimilarityIndex`] that consumes the same `JournalEvent`
//! stream as `sourcerer-index` and answers near-duplicate filename
//! queries via bigram-MinHash + 16-band LSH:
//!
//! ```ignore
//! let sim = SimilarityIndex::open(dir)?;
//! sim.apply(&events)?;
//! sim.flush()?;
//! let hits = sim.candidates("report-final", &SimilarityOpts::default());
//! ```
//!
//! ## Why a separate index?
//!
//! Phase 4's `Index` answers *substring* queries (trigram pre-filter +
//! suffix array). Substring matching cannot answer "what's *similar* to
//! `report-final`?" — a typo (`report-fnial`), a suffix bump
//! (`report-final-v2`), or a prefix change (`weekly-report-final`)
//! breaks the substring contract entirely. MinHash + LSH encodes the
//! whole-name fingerprint and recovers all three.
//!
//! ## Public surface (Build Guide Phase 6)
//!
//! * [`SimilarityIndex::open`] — opens or creates `minhash.idx` rooted
//!   at the supplied directory.
//! * [`SimilarityIndex::apply`] — consumes a batch of journal events
//!   (Create / Rename / Delete) and updates the index. Modify and
//!   AttrChange are no-ops because the filename hasn't changed.
//! * [`SimilarityIndex::candidates`] — runs the LSH lookup, scores each
//!   candidate via Jaccard estimate, and returns the survivors.
//! * [`SimilarityIndex::flush`] — atomically rewrites `minhash.idx` via
//!   tmp-rename. The bands map is *not* persisted — rebuilt at open
//!   time from each row's signature.
//! * [`SimilarityIndex::live_row_count`] — diagnostic stat used by
//!   smoke tests + the indexd status pane.
//!
//! ## Daemon recovery flow
//!
//! `SimilarityIndex::open` returns `SimilarityError::Format` on a
//! tampered, truncated, or version-mismatched `minhash.idx`. The
//! Phase 11 daemon must catch that error, delete (or move aside) the
//! offending file, re-`open()`, then walk `Index::store::iter_all` and
//! re-`upsert` every live row to rebuild the LSH bands map from
//! scratch. SQLite is the canonical store; the similarity index is
//! always reconstructible from it. Don't bubble the error to the user
//! — surface it as a "rebuilding similarity index" toast and proceed.
//!
//! ## Memory footprint (Phase 13 perf-pass note)
//!
//! A `[u64; 128]` signature is 1 KiB per row. At the 5 M-file target
//! that's ~5 GiB just for signatures, well above the 1.5 GiB budget
//! the PRD calls out for the trigram name-index. Phase 13's perf pass
//! evaluates packed-signature representations (e.g. SimHash + 64-bit
//! fingerprints, or 8-bit truncated MinHash with proportionally
//! relaxed LSH params) before v0.19.84 ships the 5 M gate.

#![deny(rust_2018_idioms)]

pub mod error;
pub mod minhash;
mod store;

use std::collections::HashMap;
use std::path::{Path, PathBuf};
use std::sync::Arc;

use parking_lot::RwLock;
use sourcerer_journal::JournalEvent;
use tracing::{debug, warn};

pub use error::SimilarityError;
pub use minhash::{BANDS, K, MinHashFamily, ROWS_PER_BAND};

use store::{PersistedIndex, PersistedRow, TOMBSTONE_FILE_ID};

/// Default cap on candidates returned by a single similarity query.
const DEFAULT_CANDIDATE_CAP: usize = 1024;
/// Default Jaccard floor — results below this score are dropped.
pub const DEFAULT_JACCARD_THRESHOLD: f32 = 0.3;

/// Caller-controlled retrieval knobs. Defaults match the Phase-6 spec
/// (recall ≥ 95 % at Jaccard ≥ 0.30 on the synthetic 5 k corpus).
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct SimilarityOpts {
    /// Hits below this Jaccard estimate are dropped after scoring.
    pub jaccard_threshold: f32,
    /// Hard cap on the number of candidates returned. `0` means "no cap"
    /// — useful for benchmarks; the executor caller passes the
    /// query-level `candidate_cap` here.
    pub candidate_cap: usize,
}

impl Default for SimilarityOpts {
    fn default() -> Self {
        Self {
            jaccard_threshold: DEFAULT_JACCARD_THRESHOLD,
            candidate_cap: DEFAULT_CANDIDATE_CAP,
        }
    }
}

/// One scored hit. The executor sorts these by `jaccard` desc and
/// hydrates the corresponding `FileRow`s through the canonical store.
#[derive(Debug, Clone, PartialEq)]
pub struct SimilarityHit {
    pub file_id: u64,
    pub jaccard: f32,
}

/// Mmap-backed similarity index.
///
/// Internally an `Arc<RwLock<Inner>>`; `clone()` is cheap and shares
/// state across threads.
#[derive(Clone)]
pub struct SimilarityIndex {
    inner: Arc<RwLock<Inner>>,
    family: Arc<MinHashFamily>,
    path: PathBuf,
}

impl std::fmt::Debug for SimilarityIndex {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("SimilarityIndex")
            .field("path", &self.path)
            .field("live_rows", &self.live_row_count())
            .finish()
    }
}

#[derive(Default)]
struct Inner {
    /// Packed lower-cased filename heap.
    heap: Vec<u8>,
    /// Per-row state. Tombstoned rows keep their slot — append-only.
    rows: Vec<RowState>,
    /// `file_id → row_id`. Lets `apply(Rename / Delete)` find an
    /// existing row in O(1).
    by_file_id: HashMap<u64, u32>,
    /// LSH bands. `(band_idx, band_hash) → Vec<row_id>`. Per-row band
    /// hashes are derived from the row's stored signature, so this map
    /// is rebuilt on `open()`.
    bands: HashMap<(u8, u64), Vec<u32>>,
}

#[derive(Debug, Clone)]
struct RowState {
    file_id: u64,
    name_off: u32,
    name_len: u32,
    signature: [u64; K],
}

impl SimilarityIndex {
    /// Open or create a similarity index in `dir`. Loads `minhash.idx`
    /// if it exists; rebuilds the LSH bands map from the file's per-row
    /// signatures so a clean open + a `flush()`-then-reopen produce
    /// equivalent in-memory state.
    pub fn open(dir: &Path) -> Result<Self, SimilarityError> {
        std::fs::create_dir_all(dir).map_err(|e| SimilarityError::io(dir, e))?;
        let path = dir.join("minhash.idx");
        let s = Self {
            inner: Arc::new(RwLock::new(Inner::default())),
            family: Arc::new(MinHashFamily::new()),
            path,
        };
        s.load_if_exists()?;
        Ok(s)
    }

    fn load_if_exists(&self) -> Result<(), SimilarityError> {
        let bytes = match std::fs::read(&self.path) {
            Ok(b) => b,
            Err(e) if e.kind() == std::io::ErrorKind::NotFound => return Ok(()),
            Err(e) => return Err(SimilarityError::io(&self.path, e)),
        };
        let persisted = match store::decode(&bytes) {
            Ok(p) => p,
            Err(e) => {
                // Same posture as Phase 4's NameIndex: surface a clear
                // error and let the caller decide whether to discard +
                // rebuild from upstream. We don't auto-truncate.
                warn!(?e, path = %self.path.display(), "minhash.idx decode failed; index opens empty");
                return Err(e);
            }
        };
        let mut inner = self.inner.write();
        inner.heap = persisted.heap;
        for (row_id, prow) in persisted.rows.into_iter().enumerate() {
            inner.rows.push(RowState {
                file_id: prow.file_id,
                name_off: prow.name_off,
                name_len: prow.name_len,
                signature: prow.signature,
            });
            if prow.file_id != TOMBSTONE_FILE_ID {
                inner.by_file_id.insert(prow.file_id, row_id as u32);
                for band in 0..BANDS {
                    let key = MinHashFamily::band_hash(&prow.signature, band);
                    inner
                        .bands
                        .entry((band as u8, key))
                        .or_default()
                        .push(row_id as u32);
                }
            }
        }
        Ok(())
    }

    /// On-disk index path (used by tests + diagnostics).
    pub fn path(&self) -> &Path {
        &self.path
    }

    /// Number of *live* rows. Tombstoned slots are excluded.
    pub fn live_row_count(&self) -> usize {
        let inner = self.inner.read();
        inner
            .rows
            .iter()
            .filter(|r| r.file_id != TOMBSTONE_FILE_ID)
            .count()
    }

    /// Insert-or-replace by `file_id`. The caller passes a filename
    /// (lower- or mixed-case); the index normalizes it (lowercase +
    /// strip the trailing extension) before computing the MinHash
    /// signature. The full lower-cased name is still kept in the heap
    /// for diagnostics and potential future lens features. Mirrors
    /// `NameIndex::upsert`.
    pub fn upsert(&self, file_id: u64, name: &str) -> Result<(), SimilarityError> {
        if file_id == TOMBSTONE_FILE_ID {
            return Err(SimilarityError::Format(
                "u64::MAX is the tombstone sentinel and cannot be a real file_id".into(),
            ));
        }
        let name_lower = name.to_lowercase();
        let stem_for_hash = strip_extension(&name_lower);
        let signature = self.family.signature(stem_for_hash);
        let mut inner = self.inner.write();
        if let Some(&existing_row) = inner.by_file_id.get(&file_id) {
            tombstone_row_locked(&mut inner, existing_row);
        }
        let bytes = name_lower.as_bytes();
        if bytes.len() > u32::MAX as usize {
            return Err(SimilarityError::Format("filename exceeds 4 GiB".into()));
        }
        let name_off = inner.heap.len() as u32;
        let name_len = bytes.len() as u32;
        inner.heap.extend_from_slice(bytes);
        let row_id = inner.rows.len() as u32;
        if row_id == u32::MAX {
            return Err(SimilarityError::Format("row table exhausted".into()));
        }
        inner.rows.push(RowState {
            file_id,
            name_off,
            name_len,
            signature,
        });
        inner.by_file_id.insert(file_id, row_id);
        for band in 0..BANDS {
            let key = MinHashFamily::band_hash(&signature, band);
            inner
                .bands
                .entry((band as u8, key))
                .or_default()
                .push(row_id);
        }
        Ok(())
    }

    /// Tombstone the row for `file_id`. No-op if the file isn't indexed.
    pub fn remove(&self, file_id: u64) -> Result<(), SimilarityError> {
        let mut inner = self.inner.write();
        if let Some(row_id) = inner.by_file_id.remove(&file_id) {
            tombstone_row_locked(&mut inner, row_id);
        }
        Ok(())
    }

    /// Apply a batch of journal events. Only `Create` / `Rename` /
    /// `Delete` matter — `Modify` and `AttrChange` don't change the
    /// filename, so they're no-ops here.
    pub fn apply(&self, events: &[JournalEvent]) -> Result<(), SimilarityError> {
        for ev in events {
            match ev {
                JournalEvent::Create { path, .. } => {
                    let (fid, name) = file_id_and_name(path);
                    self.upsert(fid, &name)?;
                }
                JournalEvent::Rename { old_path, new_path } => {
                    let (old_fid, _) = file_id_and_name(old_path);
                    self.remove(old_fid)?;
                    let (new_fid, new_name) = file_id_and_name(new_path);
                    self.upsert(new_fid, &new_name)?;
                }
                JournalEvent::Delete { path } => {
                    let (fid, _) = file_id_and_name(path);
                    self.remove(fid)?;
                }
                JournalEvent::Modify { .. } | JournalEvent::AttrChange { .. } => {}
            }
        }
        Ok(())
    }

    /// LSH-backed candidate retrieval. Returns hits sorted by Jaccard
    /// estimate descending; the caller can re-sort or filter further.
    ///
    /// Mechanics:
    /// 1. Compute the query's MinHash signature.
    /// 2. For each of the 16 bands, look up matching `row_id`s in the
    ///    bands map.
    /// 3. Union the candidates.
    /// 4. Compute the Jaccard estimate for each unique candidate; drop
    ///    those below `opts.jaccard_threshold`.
    /// 5. Sort by Jaccard desc, truncate to `opts.candidate_cap`.
    pub fn candidates(&self, query: &str, opts: &SimilarityOpts) -> Vec<SimilarityHit> {
        let query_lower = query.to_lowercase();
        let q_stem = strip_extension(&query_lower);
        let q_sig = self.family.signature(q_stem);
        let inner = self.inner.read();
        // De-dup row_ids using a bitmap-like Vec<bool> sized to
        // inner.rows.len(); a HashSet would also work but the bitmap is
        // 8× cheaper and we have the size up-front.
        let mut seen = vec![false; inner.rows.len()];
        let mut candidates: Vec<u32> = Vec::new();
        for band in 0..BANDS {
            let key = MinHashFamily::band_hash(&q_sig, band);
            if let Some(postings) = inner.bands.get(&(band as u8, key)) {
                for &row_id in postings {
                    let idx = row_id as usize;
                    if idx < seen.len() && !seen[idx] {
                        seen[idx] = true;
                        candidates.push(row_id);
                    }
                }
            }
        }
        let mut hits: Vec<SimilarityHit> = Vec::with_capacity(candidates.len());
        for row_id in candidates {
            let row = match inner.rows.get(row_id as usize) {
                Some(r) if r.file_id != TOMBSTONE_FILE_ID => r,
                _ => continue,
            };
            let j = MinHashFamily::jaccard_estimate(&q_sig, &row.signature);
            if j >= opts.jaccard_threshold {
                hits.push(SimilarityHit {
                    file_id: row.file_id,
                    jaccard: j,
                });
            }
        }
        // Order: Jaccard desc; ties broken by file_id asc for deterministic
        // output across runs.
        hits.sort_by(|a, b| {
            b.jaccard
                .partial_cmp(&a.jaccard)
                .unwrap_or(std::cmp::Ordering::Equal)
                .then_with(|| a.file_id.cmp(&b.file_id))
        });
        if opts.candidate_cap > 0 && hits.len() > opts.candidate_cap {
            hits.truncate(opts.candidate_cap);
        }
        hits
    }

    /// Persist the current state to `minhash.idx` atomically. Mirrors
    /// `NameIndex::flush` — same tmp-rename discipline, same
    /// "all-or-nothing per `commit()`" contract.
    pub fn flush(&self) -> Result<(), SimilarityError> {
        let inner = self.inner.read();
        let snapshot = PersistedIndex {
            heap: inner.heap.clone(),
            rows: inner
                .rows
                .iter()
                .map(|r| PersistedRow {
                    file_id: r.file_id,
                    name_off: r.name_off,
                    name_len: r.name_len,
                    signature: r.signature,
                })
                .collect(),
        };
        let bytes = store::encode(&snapshot);
        store::atomic_write(&self.path, &bytes)?;
        debug!(
            path = %self.path.display(),
            rows = snapshot.rows.len(),
            heap_bytes = snapshot.heap.len(),
            "minhash.idx flushed"
        );
        Ok(())
    }
}

fn tombstone_row_locked(inner: &mut Inner, row_id: u32) {
    let idx = row_id as usize;
    if let Some(row) = inner.rows.get_mut(idx) {
        // Strip the row out of every band before flipping `file_id`,
        // so a stale band posting can't surface a tombstoned hit.
        for band in 0..BANDS {
            let key = MinHashFamily::band_hash(&row.signature, band);
            if let Some(postings) = inner.bands.get_mut(&(band as u8, key)) {
                postings.retain(|&r| r != row_id);
                if postings.is_empty() {
                    inner.bands.remove(&(band as u8, key));
                }
            }
        }
        row.file_id = TOMBSTONE_FILE_ID;
        row.name_len = 0;
    }
}

/// Derive `(file_id, name_lower)` from a journal-event path. Mirrors
/// `sourcerer-index`'s `derive_file_id` so the two indexes stay keyed by
/// the same value. We can't depend on `sourcerer-index` here without a
/// cycle, so we re-implement the same blake3-truncated hash inline.
fn file_id_and_name(path: &std::path::Path) -> (u64, String) {
    let bytes = path.as_os_str().as_encoded_bytes();
    // Same construction as `sourcerer_index::derive_file_id`; the two
    // crates need to agree on this identity for the executor to match
    // up SimilarityHit::file_id with FileRow::file_id.
    let h = blake3_truncated(bytes);
    let name = path
        .file_name()
        .and_then(|s| s.to_str())
        .map(|s| s.to_lowercase())
        .unwrap_or_default();
    (h, name)
}

/// Strip the trailing extension from a (lowercased) filename. The
/// rule mirrors `Path::file_stem` for the common case but operates on
/// a `&str` so we don't have to round-trip through `Path`. Edge cases:
/// a leading-dot file (`.bashrc`) keeps its body, a no-extension name
/// returns unchanged, a name ending in `.` returns the body without
/// the trailing dot.
fn strip_extension(name_lower: &str) -> &str {
    match name_lower.rfind('.') {
        // Dotfile: `.bashrc` has no extension to strip — keep the
        // whole leading-dot name in the bigram set.
        Some(0) => name_lower,
        Some(idx) => &name_lower[..idx],
        None => name_lower,
    }
}

/// Stable 64-bit identifier for a path — `blake3(OsStr-bytes)[..8]`
/// little-endian, identical construction to
/// `sourcerer_index::derive_file_id`. The two crates need to agree on
/// this hash so a `SimilarityHit::file_id` round-trips through
/// `Index::store::get_many` cleanly. Phase 13's perf pass widens both
/// sides to a 128-bit ULID; until then this duplication is the
/// cross-crate contract.
fn blake3_truncated(bytes: &[u8]) -> u64 {
    let h = blake3::hash(bytes);
    let head: [u8; 8] = h.as_bytes()[..8]
        .try_into()
        .expect("blake3 output is 32 bytes");
    // Sourcerer-index stores the `file_id` as `i64` (SQLite INTEGER)
    // but exposes it back as `u64` at every consumer boundary. Match
    // that bit-cast so the two crates' file_ids compare bitwise-equal.
    let signed = i64::from_le_bytes(head);
    signed as u64
}

#[cfg(test)]
mod tests {
    use super::*;
    use sourcerer_journal::JournalEvent;
    use std::path::PathBuf;
    use tempfile::tempdir;

    fn create(path: &str) -> JournalEvent {
        JournalEvent::Create {
            path: PathBuf::from(path),
            size: 1,
            mtime_ns: 0,
            ctime_ns: 0,
            attrs: 0,
        }
    }

    #[test]
    fn upsert_then_candidates_find_self() {
        let dir = tempdir().unwrap();
        let sim = SimilarityIndex::open(dir.path()).unwrap();
        sim.upsert(7, "report-final-2024.pdf").unwrap();
        let hits = sim.candidates("report-final-2024.pdf", &SimilarityOpts::default());
        assert_eq!(hits.len(), 1);
        assert_eq!(hits[0].file_id, 7);
        assert!((hits[0].jaccard - 1.0).abs() < 1e-6);
    }

    #[test]
    fn near_duplicate_recovered() {
        // Phase 6 LSH (K=128, b=16, r=8) has its recall-curve knee at
        // Jaccard ≈ 0.73 — variations above the knee recall reliably,
        // below it sag by design. We use long stems here so a single
        // suffix bump lands at Jaccard ≈ 0.88 (P[match] ≥ 0.99); short
        // stems would sit right at the knee and flake. The spec's
        // 95 % recall gate over a 5 000-name corpus lives in
        // `tests/recall.rs`.
        let dir = tempdir().unwrap();
        let sim = SimilarityIndex::open(dir.path()).unwrap();
        sim.upsert(1, "alpha-beta-gamma-delta-final-summary.pdf")
            .unwrap();
        sim.upsert(2, "alpha-beta-gamma-delta-final-summary-v2.pdf")
            .unwrap();
        sim.upsert(3, "totally-unrelated-name.bin").unwrap();
        let hits = sim.candidates(
            "alpha-beta-gamma-delta-final-summary.pdf",
            &SimilarityOpts::default(),
        );
        let ids: Vec<u64> = hits.iter().map(|h| h.file_id).collect();
        assert!(ids.contains(&1), "exact match missed: {ids:?}");
        assert!(ids.contains(&2), "suffix bump on long stem missed: {ids:?}");
        assert!(!ids.contains(&3), "unrelated leaked: {ids:?}");
    }

    #[test]
    fn remove_tombstones_row() {
        let dir = tempdir().unwrap();
        let sim = SimilarityIndex::open(dir.path()).unwrap();
        sim.upsert(11, "alpha-draft.txt").unwrap();
        assert_eq!(sim.live_row_count(), 1);
        sim.remove(11).unwrap();
        assert_eq!(sim.live_row_count(), 0);
        let hits = sim.candidates("alpha-draft.txt", &SimilarityOpts::default());
        assert!(
            hits.is_empty(),
            "removed row should not surface (got {hits:?})"
        );
    }

    #[test]
    fn rename_replaces_old_with_new() {
        let dir = tempdir().unwrap();
        let sim = SimilarityIndex::open(dir.path()).unwrap();
        sim.apply(&[create("/synth/draft.md")]).unwrap();
        sim.apply(&[JournalEvent::Rename {
            old_path: PathBuf::from("/synth/draft.md"),
            new_path: PathBuf::from("/synth/final.md"),
        }])
        .unwrap();
        assert_eq!(sim.live_row_count(), 1);
        let hits = sim.candidates("draft.md", &SimilarityOpts::default());
        assert!(
            hits.iter().all(|h| h.jaccard < 0.99),
            "old name should not perfect-match anymore"
        );
        let hits = sim.candidates("final.md", &SimilarityOpts::default());
        assert!(
            hits.iter().any(|h| h.jaccard > 0.95),
            "new name should now match"
        );
    }

    #[test]
    fn flush_then_reopen_round_trips() {
        let dir = tempdir().unwrap();
        {
            let sim = SimilarityIndex::open(dir.path()).unwrap();
            sim.upsert(1, "alpha-draft.txt").unwrap();
            sim.upsert(2, "alpha-final.txt").unwrap();
            sim.upsert(3, "beta-final.txt").unwrap();
            sim.flush().unwrap();
        }
        let sim = SimilarityIndex::open(dir.path()).unwrap();
        assert_eq!(sim.live_row_count(), 3);
        let hits = sim.candidates("alpha-final.txt", &SimilarityOpts::default());
        let ids: Vec<u64> = hits.iter().map(|h| h.file_id).collect();
        assert!(ids.contains(&2));
    }

    #[test]
    fn modify_and_attr_change_are_noops() {
        let dir = tempdir().unwrap();
        let sim = SimilarityIndex::open(dir.path()).unwrap();
        sim.apply(&[
            create("/x/keep.txt"),
            JournalEvent::Modify {
                path: PathBuf::from("/x/keep.txt"),
                size: 999,
                mtime_ns: 1,
                attrs: 0,
            },
            JournalEvent::AttrChange {
                path: PathBuf::from("/x/keep.txt"),
                attrs: 0xff,
            },
        ])
        .unwrap();
        assert_eq!(sim.live_row_count(), 1);
    }

    #[test]
    fn threshold_filters_results() {
        let dir = tempdir().unwrap();
        let sim = SimilarityIndex::open(dir.path()).unwrap();
        sim.upsert(1, "alpha-beta-gamma-delta.txt").unwrap();
        sim.upsert(2, "totally-different-name.bin").unwrap();
        let opts = SimilarityOpts {
            jaccard_threshold: 0.99,
            ..Default::default()
        };
        let hits = sim.candidates("alpha-beta-gamma-delta.txt", &opts);
        assert_eq!(hits.len(), 1);
        assert_eq!(hits[0].file_id, 1);
    }

    #[test]
    fn opening_corrupt_file_returns_format_error() {
        let dir = tempdir().unwrap();
        std::fs::write(dir.path().join("minhash.idx"), b"not even close").unwrap();
        let err = SimilarityIndex::open(dir.path()).unwrap_err();
        assert!(matches!(err, SimilarityError::Format(_)));
    }

    #[test]
    fn candidates_sort_jaccard_descending() {
        let dir = tempdir().unwrap();
        let sim = SimilarityIndex::open(dir.path()).unwrap();
        sim.upsert(1, "alpha-final.pdf").unwrap();
        sim.upsert(2, "alpha-finall.pdf").unwrap(); // 1-char typo
        sim.upsert(3, "alpha-fixed.pdf").unwrap(); // moderate edit
        let hits = sim.candidates("alpha-final.pdf", &SimilarityOpts::default());
        let ids: Vec<u64> = hits.iter().map(|h| h.file_id).collect();
        assert_eq!(ids[0], 1, "exact match should be first: {hits:?}");
        // Subsequent results are scored — assert the Jaccard sequence
        // is monotonically non-increasing (Phase-6 ordering contract).
        for w in hits.windows(2) {
            assert!(w[0].jaccard >= w[1].jaccard, "non-monotonic: {hits:?}");
        }
    }
}
