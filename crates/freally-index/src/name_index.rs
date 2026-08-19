//! Custom name index — trigram inverted postings + lexicographic suffix
//! array, mmap-backed (TASK-030).
//!
//! The Phase-5 filename lens uses this as the candidate-set generator
//! before tantivy refines the predicate. The bench gate (P50 ≤ 8 ms,
//! P99 ≤ 16 ms on 5 M files) drives the layout choices below:
//!
//! * **Packed string heap.** All filenames live in one contiguous byte
//!   buffer; rows are `(start, len)`. A 5 M-file index averages ~16
//!   bytes per name → ~80 MB heap, well under the 1.5 GB budget the
//!   PRD calls out.
//! * **Trigram inverted postings.** `(c1, c2, c3) → Vec<RowId>` over
//!   lower-cased ASCII; non-ASCII names go through Unicode-lower then
//!   are re-windowed in 3-codepoint sliding windows. The map is what
//!   Phase 5 will hit first — it returns candidates in O(|matches|).
//! * **Suffix array.** Sorted `(row_id, byte_offset)` array over the
//!   packed heap. A binary search bracket gives every occurrence of a
//!   substring in O(|q| log N) without building an FM-index.
//!
//! On-disk layout when `flush()` runs:
//!
//!   `name.idx` — header + packed heap + row table + trigram postings.
//!   `name.suf` — sorted suffix array (mmap'd directly on `open`).
//!
//! Phase 4 ships a working in-memory build path plus the mmap-backed
//! load/flush. The Phase-5 query side will reuse the same maps without
//! changes.

use std::collections::{HashMap, HashSet};
use std::fs::OpenOptions;
use std::io::Write;
use std::path::{Path, PathBuf};
use std::sync::Arc;

use memmap2::Mmap;
use parking_lot::RwLock;

use crate::error::IndexError;

const MAGIC: &[u8; 8] = b"SRC-NAME";
const VERSION: u32 = 1;

/// Reserved row-id sentinel meaning "the row was logically removed but
/// the slot has not yet been reclaimed by a compaction pass."
const TOMBSTONE: u32 = u32::MAX;

#[derive(Default)]
struct Inner {
    /// Packed lower-cased filename heap.
    heap: Vec<u8>,
    /// `(start, len)` per row, indexed by `RowId`.
    rows: Vec<(u32, u32)>,
    /// `file_id` (from `Index`) for each row. Parallel to `rows`.
    file_ids: Vec<u64>,
    /// Reverse map for delete / rename — `file_id → RowId`.
    by_file_id: HashMap<u64, u32>,
    /// Trigram → `RowId` postings (sorted, deduped on flush).
    trigrams: HashMap<[u8; 3], Vec<u32>>,
    /// SRC-M23 — the same postings over names with punctuation and
    /// whitespace removed, so Ignore Punctuation / Ignore Whitespace can
    /// seed candidates instead of walking the whole index.
    ///
    /// `None` until the first query that needs it. Both modes are
    /// opt-in and off by default, so the majority of users never build
    /// this map and never pay its memory; once built, `upsert` keeps it
    /// current like the raw one.
    seed_trigrams: Option<HashMap<[u8; 3], Vec<u32>>>,
}

/// Custom name index. Cheap to clone — internally an `Arc<RwLock<…>>`.
#[derive(Clone)]
pub struct NameIndex {
    inner: Arc<RwLock<Inner>>,
    idx_path: PathBuf,
    suf_path: PathBuf,
    /// Suffix-array mmap, populated on `open` for read-side queries.
    /// Phase 4 only writes to this on `flush`; Phase 5 will read it.
    #[allow(dead_code)]
    suffix_mmap: Arc<RwLock<Option<Mmap>>>,
}

impl NameIndex {
    pub fn open(dir: &Path) -> Result<Self, IndexError> {
        std::fs::create_dir_all(dir).map_err(|e| IndexError::io(dir, e))?;
        let idx_path = dir.join("name.idx");
        let suf_path = dir.join("name.suf");
        let s = Self {
            inner: Arc::new(RwLock::new(Inner::default())),
            idx_path,
            suf_path,
            suffix_mmap: Arc::new(RwLock::new(None)),
        };
        s.load_if_exists()?;
        s.remap_suffix_array_if_present()?;
        Ok(s)
    }

    fn load_if_exists(&self) -> Result<(), IndexError> {
        let bytes = match std::fs::read(&self.idx_path) {
            Ok(b) => b,
            Err(e) if e.kind() == std::io::ErrorKind::NotFound => return Ok(()),
            Err(e) => return Err(IndexError::io(&self.idx_path, e)),
        };
        if bytes.len() < 16 {
            return Err(IndexError::NameIndex("name.idx too short".into()));
        }
        if &bytes[0..8] != MAGIC {
            return Err(IndexError::NameIndex("name.idx magic mismatch".into()));
        }
        let version = u32::from_le_bytes(bytes[8..12].try_into().unwrap());
        if version != VERSION {
            return Err(IndexError::NameIndex(format!(
                "name.idx version {version} unsupported"
            )));
        }
        // Phase 4 is in-memory authoritative — we re-derive everything
        // from the SQLite store on open in `Index::open`. The header
        // check above is a corruption gate; the bytes themselves are
        // ignored, which keeps the format forwards-compatible while the
        // schema settles in Phase 5.
        Ok(())
    }

    fn remap_suffix_array_if_present(&self) -> Result<(), IndexError> {
        if !self.suf_path.exists() {
            return Ok(());
        }
        let f =
            std::fs::File::open(&self.suf_path).map_err(|e| IndexError::io(&self.suf_path, e))?;
        // SAFETY: the file is owned by the index daemon process; another
        // mutator going through `flush()` rewrites the file via a
        // tmp-rename, so existing mmaps stay valid until the next open.
        let mm = unsafe { Mmap::map(&f) }.map_err(|e| IndexError::io(&self.suf_path, e))?;
        *self.suffix_mmap.write() = Some(mm);
        Ok(())
    }

    /// Index `name_lower` under `file_id`.
    ///
    /// SRC-M12: a CJK name is stored as `name` + [`PHONETIC_SEP`] +
    /// its phonetic readings, so the trigram postings cover `wenjian`
    /// as well as `文件`. Augmenting here rather than at each of the
    /// five call sites keeps one place where the stored key is decided.
    /// Readers split the key with
    /// [`phonetic::plain_name`](crate::phonetic::plain_name); latin
    /// names are stored byte-for-byte as before.
    pub fn upsert(&self, file_id: u64, name_lower: &str) -> Result<(), IndexError> {
        let key = crate::phonetic::with_phonetic_keys(name_lower);
        let mut inner = self.inner.write();
        if let Some(&existing_row) = inner.by_file_id.get(&file_id) {
            Self::tombstone_row_locked(&mut inner, existing_row);
        }
        let start = inner.heap.len();
        let bytes = key.as_bytes();
        if bytes.len() > u32::MAX as usize {
            return Err(IndexError::NameIndex("filename exceeds 4 GiB".into()));
        }
        let len = bytes.len() as u32;
        inner.heap.extend_from_slice(bytes);
        let row_id = inner.rows.len() as u32;
        if row_id == TOMBSTONE {
            return Err(IndexError::NameIndex("row table exhausted".into()));
        }
        inner.rows.push((start as u32, len));
        inner.file_ids.push(file_id);
        inner.by_file_id.insert(file_id, row_id);
        // Trigrams over the lowercased bytes. We use a literal byte
        // window — non-ASCII multi-byte sequences are still valid
        // candidate keys; Phase 5 widens the window to grapheme-aware
        // tokenization.
        push_trigrams(&mut inner.trigrams, bytes, row_id);
        // Only once something has asked for it — see `seed_trigrams`.
        // The lazy build and this incremental maintenance have to
        // produce byte-identical postings, so they go through the same
        // function: a divergence between them is a silent miss, not an
        // error.
        if let Some(map) = inner.seed_trigrams.as_mut() {
            push_trigrams(map, seed_key(&key).as_bytes(), row_id);
        }
        Ok(())
    }

    pub fn remove(&self, file_id: u64) -> Result<(), IndexError> {
        let mut inner = self.inner.write();
        if let Some(row_id) = inner.by_file_id.remove(&file_id) {
            Self::tombstone_row_locked(&mut inner, row_id);
        }
        Ok(())
    }

    fn tombstone_row_locked(inner: &mut Inner, row_id: u32) {
        if let Some(slot) = inner.file_ids.get_mut(row_id as usize) {
            *slot = u64::MAX;
        }
        if let Some((_, len)) = inner.rows.get_mut(row_id as usize) {
            *len = 0; // mark empty; the heap bytes get reclaimed on flush.
        }
    }

    pub fn live_row_count(&self) -> usize {
        let inner = self.inner.read();
        inner.file_ids.iter().filter(|id| **id != u64::MAX).count()
    }

    /// Number of trigram → postings entries. Useful for the smoke test
    /// without reaching into private state.
    pub fn trigram_buckets(&self) -> usize {
        self.inner.read().trigrams.len()
    }

    /// Return RowIds whose name shares any trigram with `q_lower`. The
    /// caller refines the candidate set with an exact substring check.
    /// Phase 4's smoke test uses this to validate end-to-end indexing.
    ///
    /// **PERF (Phase 13)**: this implementation pays a per-trigram
    /// `BTreeSet` round-trip on intersection because the postings are
    /// kept append-only and de-duplicated only at `flush()` time. On a
    /// 5M-file index hot trigrams (`"the"`, `".js"`, `"_20"`) have
    /// hundreds of thousands of postings each, so the BTreeSet path
    /// will not hit the P50 ≤ 8 ms / P99 ≤ 16 ms gate the filename
    /// lens needs. Phase 5 swaps in a sorted-postings two-pointer
    /// merge; Phase 13's perf pass widens to DC3 / SA-IS for the
    /// suffix array and a packed-postings on-disk layout.
    pub fn candidates(&self, q_lower: &str) -> Vec<u64> {
        let inner = self.inner.read();
        let rows = trigram_intersection(&inner, q_lower.as_bytes());
        let mut out = Vec::new();
        for r in rows {
            if let Some(&fid) = inner.file_ids.get(r as usize)
                && fid != u64::MAX
            {
                out.push(fid);
            }
        }
        out.sort_unstable();
        out.dedup();
        out
    }

    /// Phase-5 hot path: yield `(file_id, name_lower_bytes)` for every
    /// row whose name shares trigrams with `q_lower`, up to `cap`. The
    /// callback receives borrowed bytes so the caller can run literal /
    /// wildcard / regex matches without copying into a `String`. The
    /// read lock is held for the duration — callers must not call back
    /// into the index from inside `f`.
    ///
    /// `cap == 0` means "no cap"; the executor passes a real cap so a
    /// pathological 1-grapheme query can't spend the whole 16ms budget
    /// in this loop.
    pub fn for_each_candidate_named<F>(&self, q_lower: &str, cap: usize, f: F)
    where
        F: FnMut(u64, &[u8]),
    {
        let inner = self.inner.read();
        emit_candidates(&inner, &inner.trigrams, q_lower.as_bytes(), cap, f);
    }

    /// SRC-M23: [`for_each_candidate_named`](Self::for_each_candidate_named)
    /// over names with punctuation and whitespace stripped.
    ///
    /// `q_seed` must already have been through [`seed_key`]. The callback
    /// still receives the **raw** stored key — the caller's matcher does
    /// its own normalization, and the row's identity for sorting and
    /// hydration is its real name.
    ///
    /// The stripped key is a *superset* filter for every ignore mode, not
    /// just the both-on one: dropping characters uniformly from both sides
    /// preserves "is a contiguous substring", so a needle that matches
    /// under Ignore Punctuation alone still matches once whitespace is
    /// dropped from both sides too. Candidates are then re-tested under
    /// the mode the user actually asked for, exactly as they are on the
    /// raw path.
    pub fn for_each_seed_candidate_named<F>(&self, q_seed: &str, cap: usize, f: F)
    where
        F: FnMut(u64, &[u8]),
    {
        let bytes = q_seed.as_bytes();
        // A needle this short has no trigrams to intersect, so every live
        // row is a candidate and the stripped postings would go unread.
        // Building them here would be a whole-index walk thrown away — and
        // this is reachable by typing, since the seed shrinks as the query
        // does.
        if bytes.len() < 3 {
            let inner = self.inner.read();
            emit_candidates(&inner, &inner.trigrams, bytes, cap, f);
            return;
        }
        self.ensure_seed_trigrams();
        let inner = self.inner.read();
        let map = inner
            .seed_trigrams
            .as_ref()
            .expect("ensure_seed_trigrams ran above");
        emit_candidates(&inner, map, bytes, cap, f);
    }

    /// Build the stripped postings if nothing has needed them yet.
    ///
    /// The scan itself runs under a **read** lock and the result is
    /// installed under a short write lock. Doing the whole walk while
    /// holding the write lock would block every query and every scanner
    /// `upsert` for its duration — seconds on a multi-million-file index,
    /// and it happens on the user's first keystroke after switching an
    /// ignore mode on, which is the worst possible moment.
    fn ensure_seed_trigrams(&self) {
        if self.inner.read().seed_trigrams.is_some() {
            return;
        }
        let map = {
            let inner = self.inner.read();
            let mut map: HashMap<[u8; 3], Vec<u32>> = HashMap::new();
            for row_id in 0..inner.rows.len() as u32 {
                if inner.file_ids.get(row_id as usize).copied() == Some(u64::MAX) {
                    continue;
                }
                let Some(name) = name_bytes(&inner, row_id) else {
                    continue;
                };
                let Ok(text) = std::str::from_utf8(name) else {
                    continue;
                };
                push_trigrams(&mut map, seed_key(text).as_bytes(), row_id);
            }
            map
        };
        let mut inner = self.inner.write();
        // Another caller may have installed one while the read lock was
        // released. Theirs is as good as ours, and rows written in the gap
        // are in theirs and not ours — so keep it and drop this.
        if inner.seed_trigrams.is_none() {
            inner.seed_trigrams = Some(map);
        }
    }

    /// SRC-M11: yield `(file_id, name_lower_bytes)` for rows that share
    /// at least `min_shared` trigrams with `q_lower`, up to `cap` rows.
    ///
    /// This is [`for_each_candidate_named`](Self::for_each_candidate_named)'s
    /// tolerant sibling, and it exists because that one cannot answer
    /// the question "did they mean…?". Substring search intersects
    /// every trigram of the needle, so a single typo introduces one
    /// trigram the target lacks and the intersection collapses to
    /// nothing — which is precisely the case where a correction is
    /// worth offering. Counting shared trigrams instead of requiring
    /// all of them keeps a one-edit neighbour in the candidate set.
    ///
    /// Only ever called on the zero-hit path, so the work is bounded by
    /// `cap` and paid once, when the user is already looking at an
    /// empty result list.
    ///
    /// A needle shorter than three bytes has no trigrams and yields
    /// nothing: at that length every name in the index is a
    /// "candidate", and correcting a two-character term is guesswork.
    pub fn for_each_fuzzy_candidate<F>(
        &self,
        q_lower: &str,
        min_shared: usize,
        cap: usize,
        mut f: F,
    ) where
        F: FnMut(u64, &[u8]),
    {
        let bytes = q_lower.as_bytes();
        if bytes.len() < 3 || min_shared == 0 {
            return;
        }
        let inner = self.inner.read();
        // Count how many of the needle's trigrams each row carries.
        // Distinct trigrams only — a repeated trigram in the needle
        // would otherwise let one row's single match count twice.
        let mut seen_trigrams: HashSet<[u8; 3]> = HashSet::new();
        let mut shared: HashMap<u32, usize> = HashMap::new();
        for w in bytes.windows(3) {
            let key = [w[0], w[1], w[2]];
            if !seen_trigrams.insert(key) {
                continue;
            }
            let Some(postings) = inner.trigrams.get(&key) else {
                continue;
            };
            let mut last: Option<u32> = None;
            for &row in postings {
                // Postings are append-only until flush, so a row can
                // appear more than once for the same trigram.
                if last == Some(row) {
                    continue;
                }
                last = Some(row);
                *shared.entry(row).or_insert(0) += 1;
            }
        }
        let mut emitted = 0usize;
        // Sort by descending overlap so `cap` keeps the most promising
        // rows rather than whichever the hash map happened to yield
        // first — and so the candidate set is deterministic.
        let mut ranked: Vec<(u32, usize)> = shared
            .into_iter()
            .filter(|&(_, n)| n >= min_shared)
            .collect();
        ranked.sort_unstable_by(|a, b| b.1.cmp(&a.1).then(a.0.cmp(&b.0)));
        for (row, _) in ranked {
            if let Some(&fid) = inner.file_ids.get(row as usize)
                && fid != u64::MAX
                && let Some(name) = name_bytes(&inner, row)
            {
                f(fid, name);
                emitted += 1;
                if cap != 0 && emitted >= cap {
                    return;
                }
            }
        }
    }

    /// Phase-5 fallback path for queries with no trigram seed (regex,
    /// pure-modifier, or wildcard like `*.txt` whose static body is
    /// shorter than three bytes). Walks every live row in `RowId`
    /// order. Stops early when the callback returns `false`.
    pub fn for_each_live<F>(&self, mut f: F)
    where
        F: FnMut(u64, &[u8]) -> bool,
    {
        let inner = self.inner.read();
        for (row_id, &fid) in inner.file_ids.iter().enumerate() {
            if fid == u64::MAX {
                continue;
            }
            if let Some(name) = name_bytes(&inner, row_id as u32) {
                if !f(fid, name) {
                    return;
                }
            }
        }
    }

    /// Persist the in-memory state to `name.idx` + `name.suf` atomically
    /// via tmp-rename. Called from `Index::commit`.
    ///
    /// **PERF (Phase 13)**: Phase 4 writes both files every commit but
    /// `load_if_exists` ignores everything past the magic header — the
    /// recovery path is canonical via SQLite replay (Build Guide).
    /// Phase 5 wires the read side and trims the v1 format if it turns
    /// out the on-disk body is duplicated effort. The suffix array is
    /// also built with a naïve `sort_unstable_by` over full byte tails;
    /// for the 5M-file dataset Phase 13 swaps in DC3 / SA-IS, caps
    /// suffix length at 64 bytes, or defers construction to a
    /// background task.
    pub fn flush(&self) -> Result<(), IndexError> {
        let inner = self.inner.read();
        // Header.
        let mut idx = Vec::with_capacity(64 + inner.heap.len());
        idx.extend_from_slice(MAGIC);
        idx.extend_from_slice(&VERSION.to_le_bytes());
        idx.extend_from_slice(&(inner.rows.len() as u32).to_le_bytes());
        idx.extend_from_slice(&(inner.heap.len() as u64).to_le_bytes());
        // Heap.
        idx.extend_from_slice(&inner.heap);
        // Row table.
        for (start, len) in &inner.rows {
            idx.extend_from_slice(&start.to_le_bytes());
            idx.extend_from_slice(&len.to_le_bytes());
        }
        // file_id table parallel to rows.
        for fid in &inner.file_ids {
            idx.extend_from_slice(&fid.to_le_bytes());
        }
        // Trigram postings: count + (key, count, payload).
        idx.extend_from_slice(&(inner.trigrams.len() as u32).to_le_bytes());
        for (key, postings) in &inner.trigrams {
            idx.extend_from_slice(key);
            let dedup = dedup_sorted_view(postings);
            idx.extend_from_slice(&(dedup.len() as u32).to_le_bytes());
            for r in dedup {
                idx.extend_from_slice(&r.to_le_bytes());
            }
        }
        atomic_write(&self.idx_path, &idx)?;

        // Suffix array: every byte position in the heap that isn't the
        // tail of a tombstoned row contributes a (row_id, offset) pair,
        // sorted by the suffix it points to. This is the simplest
        // correct construction; Phase 13's perf pass swaps in DC3 / SA-IS.
        let mut suffixes: Vec<(u32, u32)> = Vec::with_capacity(inner.heap.len());
        for (row_id, (start, len)) in inner.rows.iter().enumerate() {
            if *len == 0 {
                continue;
            }
            for i in 0..*len {
                suffixes.push((row_id as u32, *start + i));
            }
        }
        suffixes.sort_unstable_by(|a, b| {
            let sa = &inner.heap[a.1 as usize..];
            let sb = &inner.heap[b.1 as usize..];
            sa.cmp(sb)
        });
        let mut suf_bytes = Vec::with_capacity(8 + suffixes.len() * 8);
        suf_bytes.extend_from_slice(MAGIC);
        suf_bytes.extend_from_slice(&VERSION.to_le_bytes());
        suf_bytes.extend_from_slice(&(suffixes.len() as u64).to_le_bytes());
        for (row, off) in &suffixes {
            suf_bytes.extend_from_slice(&row.to_le_bytes());
            suf_bytes.extend_from_slice(&off.to_le_bytes());
        }
        atomic_write(&self.suf_path, &suf_bytes)?;
        drop(inner);
        // Re-mmap so subsequent reads see the just-written file.
        self.remap_suffix_array_if_present()?;
        Ok(())
    }
}

/// Phase-5 trigram intersection for a query needle. Postings are
/// monotonic-by-`row_id` (a row's id only ever grows because `upsert`
/// pushes onto a `Vec` whose len is the next row's id) so the
/// intersection is a textbook two-pointer merge with same-value-skip
/// to dedup the rare per-name repeated trigram (e.g. `aaaa` storing
/// `aaa` twice). The `BTreeSet` predecessor was the documented Phase-5
/// perf swap — Build Guide §`name_index` PERF note.
fn trigram_intersection(inner: &Inner, bytes: &[u8]) -> Vec<u32> {
    trigram_intersection_in(&inner.trigrams, inner, bytes)
}

/// SRC-M23 — `is_punctuation` and `seed_key` are the one definition of
/// what the stripped key drops. `freally-query`'s match-mode strip
/// defers to the same predicate, so the seed can never drop *less* than
/// a mode does, which is what would turn the superset filter into a
/// silent miss.
pub fn is_punctuation(c: char) -> bool {
    // `char` has no `is_punctuation`, and pulling in a Unicode-category
    // crate to satisfy one predicate is not worth the dependency. Over
    // the non-ASCII range "not alphanumeric, not whitespace, not a
    // control character" is exactly punctuation and symbols.
    c.is_ascii_punctuation()
        || (!c.is_ascii() && !c.is_alphanumeric() && !c.is_whitespace() && !c.is_control())
}

/// The stripped form a name is indexed under, and the form a needle has
/// to be put in to look it up: punctuation and whitespace removed.
///
/// Borrows straight back when there is nothing to drop, which is the
/// common case for a needle and for most filenames.
pub fn seed_key(s: &str) -> std::borrow::Cow<'_, str> {
    if s.chars().any(|c| is_punctuation(c) || c.is_whitespace()) {
        std::borrow::Cow::Owned(
            s.chars()
                .filter(|c| !is_punctuation(*c) && !c.is_whitespace())
                .collect(),
        )
    } else {
        std::borrow::Cow::Borrowed(s)
    }
}

/// Push every 3-byte window of `bytes` into `map` under `row_id`.
///
/// Three callers: the raw postings and the stripped postings in `upsert`,
/// and the lazy stripped build in `ensure_seed_trigrams`. The last two
/// must agree byte for byte or the seed index stops describing the index
/// it mirrors — and the symptom of that is a query returning nothing,
/// with nothing logged.
fn push_trigrams(map: &mut HashMap<[u8; 3], Vec<u32>>, bytes: &[u8], row_id: u32) {
    if bytes.len() < 3 {
        return;
    }
    for w in bytes.windows(3) {
        map.entry([w[0], w[1], w[2]]).or_default().push(row_id);
    }
}

/// Yield `(file_id, name)` for the rows `bytes` selects out of `trigrams`,
/// skipping tombstones, stopping at `cap`.
///
/// Both public readers are this function with a different postings map.
/// They were briefly two copies, and the copies had already diverged: one
/// materialized every live row id into a `Vec` for the short-needle case
/// where the other streamed. `cap` accounting and the tombstone test now
/// live in one place, which is what stops the two seeding paths answering
/// differently.
fn emit_candidates<F>(
    inner: &Inner,
    trigrams: &HashMap<[u8; 3], Vec<u32>>,
    bytes: &[u8],
    cap: usize,
    mut f: F,
) where
    F: FnMut(u64, &[u8]),
{
    let mut emitted = 0usize;
    let mut emit = |row_id: u32, fid: u64, f: &mut F| -> bool {
        let Some(name) = name_bytes(inner, row_id) else {
            return true;
        };
        f(fid, name);
        emitted += 1;
        cap == 0 || emitted < cap
    };
    // Below three bytes there is nothing to intersect, so every live row
    // is a candidate — streamed rather than collected, so `cap` can stop
    // it early instead of after materializing the whole index.
    if bytes.len() < 3 {
        for (row_id, &fid) in inner.file_ids.iter().enumerate() {
            if fid != u64::MAX && !emit(row_id as u32, fid, &mut f) {
                return;
            }
        }
        return;
    }
    for r in trigram_intersection_in(trigrams, inner, bytes) {
        if let Some(&fid) = inner.file_ids.get(r as usize)
            && fid != u64::MAX
            && !emit(r, fid, &mut f)
        {
            return;
        }
    }
}

fn trigram_intersection_in(
    trigrams: &HashMap<[u8; 3], Vec<u32>>,
    inner: &Inner,
    bytes: &[u8],
) -> Vec<u32> {
    if bytes.len() < 3 {
        // Caller (the Phase-5 fallbacks) handles short needles.
        return inner
            .file_ids
            .iter()
            .enumerate()
            .filter_map(|(i, fid)| (*fid != u64::MAX).then_some(i as u32))
            .collect();
    }
    let mut row_hits: Option<Vec<u32>> = None;
    for w in bytes.windows(3) {
        let key = [w[0], w[1], w[2]];
        let postings = match trigrams.get(&key) {
            Some(v) => v,
            None => return Vec::new(),
        };
        row_hits = Some(match row_hits {
            None => dedup_sorted_view(postings),
            Some(prev) => intersect_sorted(&prev, postings),
        });
        if row_hits.as_ref().is_some_and(Vec::is_empty) {
            return Vec::new();
        }
    }
    row_hits.unwrap_or_default()
}

fn intersect_sorted(a: &[u32], b: &[u32]) -> Vec<u32> {
    let mut out = Vec::with_capacity(a.len().min(b.len()));
    let (mut i, mut j) = (0usize, 0usize);
    let mut last: Option<u32> = None;
    while i < a.len() && j < b.len() {
        match a[i].cmp(&b[j]) {
            std::cmp::Ordering::Less => i += 1,
            std::cmp::Ordering::Greater => j += 1,
            std::cmp::Ordering::Equal => {
                if last != Some(a[i]) {
                    out.push(a[i]);
                    last = Some(a[i]);
                }
                i += 1;
                j += 1;
            }
        }
    }
    out
}

fn name_bytes(inner: &Inner, row_id: u32) -> Option<&[u8]> {
    let (start, len) = *inner.rows.get(row_id as usize)?;
    if len == 0 {
        return None;
    }
    let s = start as usize;
    let e = s + len as usize;
    inner.heap.get(s..e)
}

fn dedup_sorted_view(postings: &[u32]) -> Vec<u32> {
    let mut v: Vec<u32> = postings.to_vec();
    v.sort_unstable();
    v.dedup();
    v
}

fn atomic_write(path: &Path, bytes: &[u8]) -> Result<(), IndexError> {
    let tmp = tmp_path(path);
    {
        let mut f = OpenOptions::new()
            .write(true)
            .create(true)
            .truncate(true)
            .open(&tmp)
            .map_err(|e| IndexError::io(&tmp, e))?;
        f.write_all(bytes).map_err(|e| IndexError::io(&tmp, e))?;
        f.sync_all().map_err(|e| IndexError::io(&tmp, e))?;
    }
    std::fs::rename(&tmp, path).map_err(|e| IndexError::io(path, e))?;
    Ok(())
}

fn tmp_path(path: &Path) -> PathBuf {
    let mut s = path.as_os_str().to_owned();
    s.push(".tmp");
    PathBuf::from(s)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn index_with(names: &[&str]) -> (tempfile::TempDir, NameIndex) {
        let dir = tempfile::tempdir().expect("tempdir");
        let ni = NameIndex::open(dir.path()).expect("open");
        for (i, n) in names.iter().enumerate() {
            ni.upsert(i as u64, &n.to_lowercase()).expect("upsert");
        }
        (dir, ni)
    }

    fn fuzzy(ni: &NameIndex, q: &str, min_shared: usize) -> Vec<String> {
        let mut out = Vec::new();
        ni.for_each_fuzzy_candidate(q, min_shared, 32, |_, name| {
            out.push(String::from_utf8_lossy(name).to_string());
        });
        out
    }

    #[test]
    fn a_typo_finds_no_substring_candidate_but_does_find_a_fuzzy_one() {
        let (_d, ni) = index_with(&["freally", "unrelated"]);
        // The substring path intersects every trigram, so the stray
        // `lll` in the typo collapses it to nothing. That failure is
        // exactly what SRC-M11 exists to rescue.
        let mut strict = Vec::new();
        ni.for_each_candidate_named("freallly", 32, |_, n| {
            strict.push(String::from_utf8_lossy(n).to_string())
        });
        assert!(
            strict.is_empty(),
            "substring path unexpectedly matched: {strict:?}"
        );

        let loose = fuzzy(&ni, "freallly", 3);
        assert!(loose.contains(&"freally".to_string()), "got {loose:?}");
    }

    #[test]
    fn candidates_are_ordered_by_shared_trigram_count() {
        let (_d, ni) = index_with(&["quarterly-report", "quarterly-budget", "zzzzzz"]);
        let out = fuzzy(&ni, "quarterly-reprot", 3);
        assert_eq!(
            out.first().map(String::as_str),
            Some("quarterly-report"),
            "closest name should rank first, got {out:?}"
        );
        assert!(!out.contains(&"zzzzzz".to_string()));
    }

    #[test]
    fn the_cap_bounds_the_candidate_set() {
        let names: Vec<String> = (0..50).map(|i| format!("report-{i:02}")).collect();
        let refs: Vec<&str> = names.iter().map(String::as_str).collect();
        let (_d, ni) = index_with(&refs);
        let mut n = 0usize;
        ni.for_each_fuzzy_candidate("report", 3, 5, |_, _| n += 1);
        assert_eq!(n, 5);
    }

    #[test]
    fn a_needle_shorter_than_a_trigram_yields_nothing() {
        let (_d, ni) = index_with(&["freally"]);
        assert!(fuzzy(&ni, "fr", 1).is_empty());
        assert!(fuzzy(&ni, "", 1).is_empty());
    }

    #[test]
    fn a_removed_row_is_never_offered() {
        let (_d, ni) = index_with(&["freally", "freallz"]);
        ni.remove(0).expect("remove");
        let out = fuzzy(&ni, "freallly", 3);
        assert!(!out.contains(&"freally".to_string()), "got {out:?}");
    }

    #[test]
    fn raising_min_shared_narrows_the_set() {
        let (_d, ni) = index_with(&["quarterly-report", "quarterly-budget"]);
        let loose = fuzzy(&ni, "quarterly-report", 3).len();
        let tight = fuzzy(&ni, "quarterly-report", 12).len();
        assert!(tight <= loose, "loose={loose} tight={tight}");
        assert!(tight >= 1, "the exact name must still qualify");
    }
}
