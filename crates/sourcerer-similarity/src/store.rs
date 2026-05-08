//! On-disk persistence for the similarity index (`minhash.idx`).
//!
//! Mirrors `sourcerer-index`'s `name.idx` discipline: a small magic +
//! version header, all-or-nothing rewrite via tmp-rename, no in-place
//! mutation. A torn write at any point leaves the previous file intact.
//!
//! Phase 6 keeps the file format honest but simple — every flush rewrites
//! the whole index. Phase 13's perf pass swaps in an append-only journal
//! and a derived bands map; the v1 format on disk is a compatibility
//! anchor for that swap.
//!
//! On-disk layout (little-endian throughout):
//!
//! ```text
//! [Header — 32 bytes]
//!   magic        : 8 B  "SRC-MNHS"
//!   version      : u32  (1)
//!   k            : u32  (128)
//!   bands        : u32  (16)
//!   num_rows     : u32  (live + tombstoned slots)
//!   heap_len     : u64  packed name_lower bytes
//!
//! [Heap]  num bytes = heap_len
//!
//! [Rows]  num_rows entries, each fixed-size:
//!   file_id      : u64  (u64::MAX = tombstoned slot)
//!   name_off     : u32  (offset into heap)
//!   name_len     : u32
//!   signature    : K * u64 = 1024 B
//! ```
//!
//! The bands map is *not* persisted — `SimilarityIndex::open` rebuilds
//! it from each live row's signature. That's O(num_rows × BANDS) at open
//! time, comfortably under a second for the 5 M-file target.

use std::fs::OpenOptions;
use std::io::Write;
use std::path::{Path, PathBuf};

use crate::error::SimilarityError;
use crate::minhash::{BANDS, K};

pub(crate) const MAGIC: &[u8; 8] = b"SRC-MNHS";
pub(crate) const VERSION: u32 = 1;

pub(crate) const HEADER_LEN: usize = 8 + 4 + 4 + 4 + 4 + 8;
pub(crate) const ROW_FIXED_LEN: usize = 8 + 4 + 4 + K * 8;
/// Sentinel meaning "this slot is tombstoned." Rows are append-only, so
/// removal flips `file_id` to this value.
pub(crate) const TOMBSTONE_FILE_ID: u64 = u64::MAX;

#[derive(Debug, Clone)]
pub(crate) struct PersistedRow {
    pub file_id: u64,
    pub name_off: u32,
    pub name_len: u32,
    pub signature: [u64; K],
}

#[derive(Debug, Default)]
pub(crate) struct PersistedIndex {
    pub heap: Vec<u8>,
    pub rows: Vec<PersistedRow>,
}

/// Atomically write `bytes` to `path` via tmp-rename. Used by `flush()`.
pub(crate) fn atomic_write(path: &Path, bytes: &[u8]) -> Result<(), SimilarityError> {
    let tmp = tmp_path(path);
    {
        let mut f = OpenOptions::new()
            .write(true)
            .create(true)
            .truncate(true)
            .open(&tmp)
            .map_err(|e| SimilarityError::io(&tmp, e))?;
        f.write_all(bytes)
            .map_err(|e| SimilarityError::io(&tmp, e))?;
        f.sync_all().map_err(|e| SimilarityError::io(&tmp, e))?;
    }
    std::fs::rename(&tmp, path).map_err(|e| SimilarityError::io(path, e))?;
    Ok(())
}

fn tmp_path(path: &Path) -> PathBuf {
    let mut s = path.as_os_str().to_owned();
    s.push(".tmp");
    PathBuf::from(s)
}

/// Serialize an in-memory snapshot to bytes ready for `atomic_write`.
pub(crate) fn encode(idx: &PersistedIndex) -> Vec<u8> {
    let total_len = HEADER_LEN + idx.heap.len() + idx.rows.len() * ROW_FIXED_LEN;
    let mut out = Vec::with_capacity(total_len);
    out.extend_from_slice(MAGIC);
    out.extend_from_slice(&VERSION.to_le_bytes());
    out.extend_from_slice(&(K as u32).to_le_bytes());
    out.extend_from_slice(&(BANDS as u32).to_le_bytes());
    out.extend_from_slice(&(idx.rows.len() as u32).to_le_bytes());
    out.extend_from_slice(&(idx.heap.len() as u64).to_le_bytes());
    out.extend_from_slice(&idx.heap);
    for r in &idx.rows {
        out.extend_from_slice(&r.file_id.to_le_bytes());
        out.extend_from_slice(&r.name_off.to_le_bytes());
        out.extend_from_slice(&r.name_len.to_le_bytes());
        for v in r.signature {
            out.extend_from_slice(&v.to_le_bytes());
        }
    }
    out
}

/// Read + validate the on-disk file. Format / magic / version / size
/// mismatches all surface as `SimilarityError::Format` so the caller can
/// decide whether to discard and rebuild.
pub(crate) fn decode(bytes: &[u8]) -> Result<PersistedIndex, SimilarityError> {
    if bytes.len() < HEADER_LEN {
        return Err(SimilarityError::Format(format!(
            "minhash.idx truncated: got {} bytes, need ≥ {}",
            bytes.len(),
            HEADER_LEN
        )));
    }
    if &bytes[0..8] != MAGIC {
        return Err(SimilarityError::Format("magic mismatch".into()));
    }
    let version = u32::from_le_bytes(bytes[8..12].try_into().unwrap());
    if version != VERSION {
        return Err(SimilarityError::Format(format!(
            "unsupported minhash.idx version {version}"
        )));
    }
    let k = u32::from_le_bytes(bytes[12..16].try_into().unwrap()) as usize;
    let bands = u32::from_le_bytes(bytes[16..20].try_into().unwrap()) as usize;
    if k != K || bands != BANDS {
        return Err(SimilarityError::Format(format!(
            "minhash params mismatch: file says k={k}, bands={bands}; \
             this build expects k={K}, bands={BANDS}"
        )));
    }
    let num_rows = u32::from_le_bytes(bytes[20..24].try_into().unwrap()) as usize;
    let heap_len = u64::from_le_bytes(bytes[24..32].try_into().unwrap()) as usize;

    let body_off = HEADER_LEN;
    let heap_end = body_off
        .checked_add(heap_len)
        .ok_or_else(|| SimilarityError::Format("heap_len overflows usize".into()))?;
    let rows_end = heap_end
        .checked_add(num_rows.saturating_mul(ROW_FIXED_LEN))
        .ok_or_else(|| SimilarityError::Format("row table overflows usize".into()))?;
    if bytes.len() < rows_end {
        return Err(SimilarityError::Format(format!(
            "minhash.idx truncated: got {} bytes, need ≥ {}",
            bytes.len(),
            rows_end
        )));
    }

    let heap = bytes[body_off..heap_end].to_vec();
    let mut rows = Vec::with_capacity(num_rows);
    let mut p = heap_end;
    for _ in 0..num_rows {
        let file_id = u64::from_le_bytes(bytes[p..p + 8].try_into().unwrap());
        p += 8;
        let name_off = u32::from_le_bytes(bytes[p..p + 4].try_into().unwrap());
        p += 4;
        let name_len = u32::from_le_bytes(bytes[p..p + 4].try_into().unwrap());
        p += 4;
        let mut signature = [0u64; K];
        for slot in signature.iter_mut() {
            *slot = u64::from_le_bytes(bytes[p..p + 8].try_into().unwrap());
            p += 8;
        }
        // Bounds-check the heap reference. A truncated write would
        // surface here as `Format`, not as an OOB panic later.
        if name_len != 0 && file_id != TOMBSTONE_FILE_ID {
            let s = name_off as usize;
            let e = s + name_len as usize;
            if e > heap.len() {
                return Err(SimilarityError::Format(format!(
                    "row name slice [{s}..{e}] exceeds heap length {}",
                    heap.len()
                )));
            }
        }
        rows.push(PersistedRow {
            file_id,
            name_off,
            name_len,
            signature,
        });
    }
    Ok(PersistedIndex { heap, rows })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::minhash::MinHashFamily;

    #[test]
    fn header_round_trips() {
        let mh = MinHashFamily::new();
        let mut idx = PersistedIndex::default();
        idx.heap.extend_from_slice(b"hello.txt");
        idx.rows.push(PersistedRow {
            file_id: 42,
            name_off: 0,
            name_len: 9,
            signature: mh.signature("hello.txt"),
        });
        let bytes = encode(&idx);
        let back = decode(&bytes).unwrap();
        assert_eq!(back.heap, idx.heap);
        assert_eq!(back.rows.len(), 1);
        assert_eq!(back.rows[0].file_id, 42);
        assert_eq!(back.rows[0].signature, idx.rows[0].signature);
    }

    #[test]
    fn truncated_file_errors_format() {
        let mut idx = PersistedIndex::default();
        idx.heap.extend_from_slice(b"abcd");
        idx.rows.push(PersistedRow {
            file_id: 1,
            name_off: 0,
            name_len: 4,
            signature: [0; K],
        });
        let bytes = encode(&idx);
        let truncated = &bytes[..bytes.len() - 16];
        let err = decode(truncated).unwrap_err();
        assert!(matches!(err, SimilarityError::Format(_)));
    }

    #[test]
    fn bad_magic_errors_format() {
        let mut idx = PersistedIndex::default();
        idx.rows.push(PersistedRow {
            file_id: 1,
            name_off: 0,
            name_len: 0,
            signature: [0; K],
        });
        let mut bytes = encode(&idx);
        bytes[0..8].copy_from_slice(b"GARBAGE!");
        let err = decode(&bytes).unwrap_err();
        assert!(matches!(err, SimilarityError::Format(_)));
    }

    #[test]
    fn out_of_bounds_name_offset_errors_format() {
        let mut idx = PersistedIndex::default();
        idx.heap.extend_from_slice(b"abcd");
        idx.rows.push(PersistedRow {
            file_id: 1,
            name_off: 0,
            name_len: 999, // past end of 4-byte heap
            signature: [0; K],
        });
        let bytes = encode(&idx);
        let err = decode(&bytes).unwrap_err();
        assert!(matches!(err, SimilarityError::Format(_)));
    }
}
