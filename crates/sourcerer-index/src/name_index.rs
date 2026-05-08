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

use std::collections::HashMap;
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

    pub fn upsert(&self, file_id: u64, name_lower: &str) -> Result<(), IndexError> {
        let mut inner = self.inner.write();
        if let Some(&existing_row) = inner.by_file_id.get(&file_id) {
            Self::tombstone_row_locked(&mut inner, existing_row);
        }
        let start = inner.heap.len();
        let bytes = name_lower.as_bytes();
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
        if bytes.len() >= 3 {
            for w in bytes.windows(3) {
                let key = [w[0], w[1], w[2]];
                inner.trigrams.entry(key).or_default().push(row_id);
            }
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
        let bytes = q_lower.as_bytes();
        if bytes.len() < 3 {
            // Trigram index can't refine — fall back to "everything live".
            // Phase 5 will swap in a 1- and 2-gram fallback path; the
            // smoke test never hits this branch.
            return inner
                .file_ids
                .iter()
                .filter(|id| **id != u64::MAX)
                .copied()
                .collect();
        }
        let mut row_hits: Option<Vec<u32>> = None;
        for w in bytes.windows(3) {
            let key = [w[0], w[1], w[2]];
            let postings = match inner.trigrams.get(&key) {
                Some(v) => v.clone(),
                None => return Vec::new(),
            };
            row_hits = Some(match row_hits {
                None => postings,
                Some(prev) => intersect_sorted_dedup(&prev, &postings),
            });
            if row_hits.as_ref().is_some_and(Vec::is_empty) {
                return Vec::new();
            }
        }
        let mut out = Vec::new();
        let rows = row_hits.unwrap_or_default();
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

fn intersect_sorted_dedup(a: &[u32], b: &[u32]) -> Vec<u32> {
    // Postings are appended unsorted, so dedup on the fly with sets.
    use std::collections::BTreeSet;
    let sa: BTreeSet<u32> = a.iter().copied().collect();
    let sb: BTreeSet<u32> = b.iter().copied().collect();
    sa.intersection(&sb).copied().collect()
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
