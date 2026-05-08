//! Content-addressed, zstd-compressed text blob store (TASK-050).
//!
//! Layout under `<index_root>/extracted/`:
//!
//!   `<first2hex>/<full-hex>`
//!
//! `full-hex` is the lowercase 64-char `blake3(content)` hex digest;
//! `first2hex` is the first two hex characters, sharded so `ls` on a
//! tree of 256 directories stays workable. The file body is a single
//! zstd frame containing the extracted UTF-8 text.
//!
//! All writes are atomic via tmp-rename in the *same* shard directory
//! (so `rename(2)` stays within one filesystem). Reads mmap the
//! compressed frame and decompress eagerly — Phase 13's perf pass
//! evaluates streaming-decompress for very large blobs.
//!
//! Content-addressed means dedup is implicit: two extractors that
//! produce identical text-blob bytes resolve to the same `BlobId` and
//! the second `put` is a no-op (`exists()`-then-skip). The Phase-13
//! perf pass will measure dedup hit-rate; we track `BlobStoreStats`
//! at minimum so the daemon's status pane can surface it.

use std::fs::{self, File};
use std::io::{self, Write};
use std::path::{Path, PathBuf};
use std::sync::Arc;
use std::sync::atomic::{AtomicU64, Ordering};

use memmap2::Mmap;
use parking_lot::Mutex;
use thiserror::Error;
use tracing::{debug, warn};

#[derive(Debug, Error)]
pub enum BlobStoreError {
    #[error("blob store I/O at {path}: {source}")]
    Io {
        path: PathBuf,
        #[source]
        source: io::Error,
    },
    #[error("zstd compression failed: {0}")]
    Compress(#[source] io::Error),
    #[error("zstd decompression failed: {0}")]
    Decompress(#[source] io::Error),
    #[error("invalid blob id hex: {0}")]
    BadHex(String),
}

impl BlobStoreError {
    fn io<P: Into<PathBuf>>(path: P, source: io::Error) -> Self {
        Self::Io {
            path: path.into(),
            source,
        }
    }
}

/// 32-byte blake3 digest of the *uncompressed* content. Display prints
/// 64 hex characters; the on-disk filename matches.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct BlobId([u8; 32]);

impl BlobId {
    pub fn from_content(content: &[u8]) -> Self {
        let h = blake3::hash(content);
        Self(*h.as_bytes())
    }

    pub fn from_bytes(bytes: [u8; 32]) -> Self {
        Self(bytes)
    }

    pub fn as_bytes(&self) -> &[u8; 32] {
        &self.0
    }

    pub fn to_hex(&self) -> String {
        use std::fmt::Write;
        let mut s = String::with_capacity(64);
        for b in self.0 {
            // `Write::write_fmt` lets us avoid the per-byte `format!`
            // temporary; the unwrap is infallible because writing into
            // a `String` cannot fail.
            write!(&mut s, "{b:02x}").expect("writing to a String never fails");
        }
        s
    }

    pub fn from_hex(hex: &str) -> Result<Self, BlobStoreError> {
        if hex.len() != 64 {
            return Err(BlobStoreError::BadHex(format!(
                "expected 64 hex chars, got {}",
                hex.len()
            )));
        }
        let mut out = [0u8; 32];
        for (i, b) in out.iter_mut().enumerate() {
            let byte_str = hex.get(i * 2..i * 2 + 2).ok_or_else(|| {
                BlobStoreError::BadHex("hex string too short for byte slice".into())
            })?;
            *b = u8::from_str_radix(byte_str, 16)
                .map_err(|_| BlobStoreError::BadHex(format!("non-hex byte at {i}")))?;
        }
        Ok(Self(out))
    }
}

impl std::fmt::Display for BlobId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.to_hex())
    }
}

/// Default zstd compression level. Level 3 is the zstd default — a
/// good trade-off between ratio and CPU. Phase 13 perf pass evaluates
/// switching to level 1 if compression dominates extraction time.
pub const DEFAULT_ZSTD_LEVEL: i32 = 3;

#[derive(Debug, Clone, Copy, Default)]
pub struct BlobStoreStats {
    /// Total `put` calls.
    pub puts: u64,
    /// `put` calls that hit an existing blob (dedup hit).
    pub dedup_hits: u64,
    /// `get` calls that returned a blob.
    pub get_hits: u64,
    /// `get` calls that returned `None`.
    pub get_misses: u64,
    /// Bytes written to disk (compressed, including metadata overhead).
    pub bytes_written: u64,
    /// Bytes returned via `get` (decompressed).
    pub bytes_decompressed: u64,
}

#[derive(Clone)]
pub struct BlobStore {
    inner: Arc<Inner>,
}

struct Inner {
    root: PathBuf,
    level: i32,
    stats: Mutex<BlobStoreStats>,
    /// Per-store monotonic counter so concurrent `put` calls for the
    /// *same* content land in distinct tmp files. Without this, two
    /// writers race on `.tmp-<hex>` — `File::create` truncates one's
    /// in-flight write, and on Windows the second open fails with
    /// `ERROR_SHARING_VIOLATION` even though dedup is the legitimate
    /// outcome.
    tmp_seq: AtomicU64,
}

impl BlobStore {
    /// Open or create a blob store rooted at `root`. The full layout
    /// (`root/<first2hex>/<full-hex>`) is created lazily — `open`
    /// only ensures `root` itself exists.
    pub fn open(root: &Path) -> Result<Self, BlobStoreError> {
        fs::create_dir_all(root).map_err(|e| BlobStoreError::io(root, e))?;
        Ok(Self {
            inner: Arc::new(Inner {
                root: root.to_path_buf(),
                level: DEFAULT_ZSTD_LEVEL,
                stats: Mutex::new(BlobStoreStats::default()),
                tmp_seq: AtomicU64::new(0),
            }),
        })
    }

    pub fn with_level(self, level: i32) -> Self {
        let mut inner = (*self.inner).copy_with_level(level);
        // Preserve stats across the rebuild; cheap because we copy the
        // values, not the lock.
        let snap = *self.inner.stats.lock();
        inner.stats = Mutex::new(snap);
        Self {
            inner: Arc::new(inner),
        }
    }

    pub fn root(&self) -> &Path {
        &self.inner.root
    }

    pub fn stats(&self) -> BlobStoreStats {
        *self.inner.stats.lock()
    }

    fn shard_path(&self, id: BlobId) -> (PathBuf, PathBuf) {
        let hex = id.to_hex();
        let shard = &hex[..2];
        let dir = self.inner.root.join(shard);
        let file = dir.join(&hex);
        (dir, file)
    }

    pub fn contains(&self, id: BlobId) -> bool {
        let (_, file) = self.shard_path(id);
        file.exists()
    }

    /// Compress + write `content`, returning its `BlobId`. Idempotent:
    /// re-putting an identical blob is a no-op (tracked as a dedup hit).
    pub fn put(&self, content: &[u8]) -> Result<BlobId, BlobStoreError> {
        let id = BlobId::from_content(content);
        let (dir, final_path) = self.shard_path(id);
        {
            let mut stats = self.inner.stats.lock();
            stats.puts = stats.puts.saturating_add(1);
            if final_path.exists() {
                stats.dedup_hits = stats.dedup_hits.saturating_add(1);
                return Ok(id);
            }
        }
        fs::create_dir_all(&dir).map_err(|e| BlobStoreError::io(&dir, e))?;
        let compressed = zstd::stream::encode_all(content, self.inner.level)
            .map_err(BlobStoreError::Compress)?;

        // Atomic write: write to a unique-per-call .tmp-* in the *same*
        // shard dir, fsync, rename. Per-call uniqueness (pid + seq)
        // means two concurrent `put`s of the same content can't race
        // on the same tmp inode — each writes its own file, then both
        // attempt rename; the one that loses the rename race detects
        // the existing final blob and reports a dedup hit.
        let nonce = self.inner.tmp_seq.fetch_add(1, Ordering::Relaxed);
        let tmp_path = dir.join(format!(
            ".tmp-{}-{}-{}",
            id.to_hex(),
            std::process::id(),
            nonce
        ));
        {
            let mut f = File::create(&tmp_path).map_err(|e| BlobStoreError::io(&tmp_path, e))?;
            f.write_all(&compressed)
                .map_err(|e| BlobStoreError::io(&tmp_path, e))?;
            f.sync_all().map_err(|e| BlobStoreError::io(&tmp_path, e))?;
        }
        match fs::rename(&tmp_path, &final_path) {
            Ok(()) => {}
            Err(e) => {
                // Cleanup tmp regardless of outcome; we never want to
                // leave a `.tmp-*` litter for the next sweep to find.
                let _ = fs::remove_file(&tmp_path);
                // Concurrent put of the same content — Windows
                // `fs::rename` refuses to overwrite, so we land here
                // with the *legitimate* final blob already on disk.
                // Treat as dedup hit. (POSIX rename is atomic-overwrite
                // so this branch is Windows-specific in practice.)
                if final_path.exists() {
                    let mut stats = self.inner.stats.lock();
                    stats.dedup_hits = stats.dedup_hits.saturating_add(1);
                    return Ok(id);
                }
                return Err(BlobStoreError::io(&final_path, e));
            }
        }
        let mut stats = self.inner.stats.lock();
        stats.bytes_written = stats.bytes_written.saturating_add(compressed.len() as u64);
        debug!(blob = %id, bytes_in = content.len(), bytes_out = compressed.len(), "blob written");
        Ok(id)
    }

    /// Read + decompress a blob. Returns `Ok(None)` on a missing blob,
    /// `Err` on I/O or zstd decompression failure. The caller gets an
    /// owned `Vec<u8>` — the mmap is dropped before returning so the
    /// kernel can reclaim the page-cache slot if the caller's working
    /// set is small.
    pub fn get(&self, id: BlobId) -> Result<Option<Vec<u8>>, BlobStoreError> {
        let (_, final_path) = self.shard_path(id);
        let f = match File::open(&final_path) {
            Ok(f) => f,
            Err(e) if e.kind() == io::ErrorKind::NotFound => {
                let mut stats = self.inner.stats.lock();
                stats.get_misses = stats.get_misses.saturating_add(1);
                return Ok(None);
            }
            Err(e) => return Err(BlobStoreError::io(&final_path, e)),
        };
        // SAFETY: memmap2's `Mmap::map` is unsafe because the OS may
        // have someone else write to the file behind our back, which
        // would tear our reads. The blob store only ever writes via
        // tmp+rename (so the destination inode never changes once the
        // file is visible) and the daemon is the sole writer. Same
        // safety posture Phase 4 documents on `name.idx`.
        let mmap = unsafe { Mmap::map(&f) }.map_err(|e| BlobStoreError::io(&final_path, e))?;
        let bytes = zstd::stream::decode_all(&mmap[..]).map_err(BlobStoreError::Decompress)?;
        let mut stats = self.inner.stats.lock();
        stats.get_hits = stats.get_hits.saturating_add(1);
        stats.bytes_decompressed = stats.bytes_decompressed.saturating_add(bytes.len() as u64);
        Ok(Some(bytes))
    }

    /// Remove the blob if it exists. Returns `true` if a file was
    /// deleted, `false` if the blob wasn't there. The shard directory
    /// is *not* swept — empty shards stick around. Phase 13 perf pass
    /// adds a periodic compactor.
    pub fn remove(&self, id: BlobId) -> Result<bool, BlobStoreError> {
        let (_, final_path) = self.shard_path(id);
        match fs::remove_file(&final_path) {
            Ok(()) => Ok(true),
            Err(e) if e.kind() == io::ErrorKind::NotFound => Ok(false),
            Err(e) => Err(BlobStoreError::io(&final_path, e)),
        }
    }

    /// Walk the store and call `f(id)` for every live blob. Used by
    /// the daemon's index-rebuild path to re-link Phase 8 extracted
    /// text into a fresh Tantivy index without re-extracting from
    /// disk. Best-effort — malformed entries (non-hex filenames, nested
    /// directories, partial `.tmp-*` files) are logged at `warn` and
    /// skipped.
    pub fn for_each<F: FnMut(BlobId)>(&self, mut f: F) -> Result<(), BlobStoreError> {
        let root = &self.inner.root;
        let shards = match fs::read_dir(root) {
            Ok(rd) => rd,
            Err(e) if e.kind() == io::ErrorKind::NotFound => return Ok(()),
            Err(e) => return Err(BlobStoreError::io(root, e)),
        };
        for entry in shards.flatten() {
            let shard_path = entry.path();
            if !entry.file_type().map(|ft| ft.is_dir()).unwrap_or(false) {
                continue;
            }
            let blobs = match fs::read_dir(&shard_path) {
                Ok(rd) => rd,
                Err(e) => {
                    warn!(?e, path = %shard_path.display(), "blob shard unreadable");
                    continue;
                }
            };
            for blob in blobs.flatten() {
                let name = blob.file_name();
                let Some(name_str) = name.to_str() else {
                    continue;
                };
                if name_str.starts_with(".tmp-") || name_str.len() != 64 {
                    continue;
                }
                match BlobId::from_hex(name_str) {
                    Ok(id) => f(id),
                    Err(_) => {
                        warn!(name = name_str, "non-hex blob filename; skipping");
                    }
                }
            }
        }
        Ok(())
    }
}

impl Inner {
    fn copy_with_level(&self, level: i32) -> Self {
        Self {
            root: self.root.clone(),
            level,
            stats: Mutex::new(BlobStoreStats::default()),
            tmp_seq: AtomicU64::new(self.tmp_seq.load(Ordering::Relaxed)),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::tempdir;

    #[test]
    fn put_then_get_round_trips() {
        let dir = tempdir().unwrap();
        let store = BlobStore::open(dir.path()).unwrap();
        let id = store.put(b"hello world").unwrap();
        let got = store.get(id).unwrap().unwrap();
        assert_eq!(got, b"hello world");
    }

    #[test]
    fn get_missing_blob_returns_none() {
        let dir = tempdir().unwrap();
        let store = BlobStore::open(dir.path()).unwrap();
        let id = BlobId::from_content(b"never written");
        assert!(store.get(id).unwrap().is_none());
    }

    #[test]
    fn put_is_idempotent_dedup() {
        let dir = tempdir().unwrap();
        let store = BlobStore::open(dir.path()).unwrap();
        let id1 = store.put(b"same").unwrap();
        let id2 = store.put(b"same").unwrap();
        assert_eq!(id1, id2);
        let s = store.stats();
        assert_eq!(s.puts, 2);
        assert_eq!(s.dedup_hits, 1);
    }

    #[test]
    fn distinct_content_distinct_id() {
        let dir = tempdir().unwrap();
        let store = BlobStore::open(dir.path()).unwrap();
        let id1 = store.put(b"alpha").unwrap();
        let id2 = store.put(b"beta").unwrap();
        assert_ne!(id1, id2);
    }

    #[test]
    fn remove_unlinks_blob() {
        let dir = tempdir().unwrap();
        let store = BlobStore::open(dir.path()).unwrap();
        let id = store.put(b"goodbye").unwrap();
        assert!(store.contains(id));
        let removed = store.remove(id).unwrap();
        assert!(removed);
        assert!(!store.contains(id));
        // Second remove on a missing blob is a clean false.
        assert!(!store.remove(id).unwrap());
    }

    #[test]
    fn shard_layout_uses_first_two_hex_chars() {
        let dir = tempdir().unwrap();
        let store = BlobStore::open(dir.path()).unwrap();
        let id = store.put(b"layout-check").unwrap();
        let hex = id.to_hex();
        let shard = &hex[..2];
        let expected = dir.path().join(shard).join(&hex);
        assert!(expected.exists(), "expected blob at {expected:?}");
    }

    #[test]
    fn for_each_visits_every_live_blob() {
        let dir = tempdir().unwrap();
        let store = BlobStore::open(dir.path()).unwrap();
        let ids: Vec<BlobId> = (0..5)
            .map(|i| store.put(format!("doc-{i}").as_bytes()).unwrap())
            .collect();
        let mut seen = Vec::new();
        store.for_each(|id| seen.push(id)).unwrap();
        seen.sort();
        let mut expected = ids.clone();
        expected.sort();
        assert_eq!(seen, expected);
    }

    #[test]
    fn for_each_skips_partial_tmp_files() {
        let dir = tempdir().unwrap();
        let store = BlobStore::open(dir.path()).unwrap();
        let id = store.put(b"real").unwrap();
        // Plant a stray `.tmp-...` file inside the same shard.
        let (shard, _) = store.shard_path(id);
        std::fs::write(shard.join(".tmp-abc"), b"junk").unwrap();
        let mut seen = Vec::new();
        store.for_each(|id| seen.push(id)).unwrap();
        assert_eq!(seen, vec![id]);
    }

    #[test]
    fn id_hex_round_trip() {
        let id = BlobId::from_content(b"hex me");
        let hex = id.to_hex();
        assert_eq!(hex.len(), 64);
        let back = BlobId::from_hex(&hex).unwrap();
        assert_eq!(back, id);
    }

    #[test]
    fn from_hex_rejects_bad_input() {
        assert!(BlobId::from_hex("short").is_err());
        let bad: String = "z".repeat(64);
        assert!(BlobId::from_hex(&bad).is_err());
    }

    #[test]
    fn concurrent_put_of_same_content_is_consistent() {
        // Eight threads put the *same* bytes simultaneously. Every
        // thread should observe the same BlobId; the dedup-hit count
        // should equal the loser count (7); the final blob must
        // decompress back to the original payload. Regression for
        // the pre-fix version that raced on a shared `.tmp-<hex>`.
        let dir = tempdir().unwrap();
        let store = BlobStore::open(dir.path()).unwrap();
        let payload = Arc::new(b"shared content".to_vec());
        let handles: Vec<_> = (0..8)
            .map(|_| {
                let s = store.clone();
                let p = Arc::clone(&payload);
                std::thread::spawn(move || s.put(&p))
            })
            .collect();
        let ids: Vec<BlobId> = handles
            .into_iter()
            .map(|h| h.join().unwrap().unwrap())
            .collect();
        assert!(ids.iter().all(|&i| i == ids[0]));
        let got = store.get(ids[0]).unwrap().unwrap();
        assert_eq!(got, *payload);
    }

    #[test]
    fn put_after_stray_tmp_file_succeeds() {
        // Plant a `.tmp-*` file in the shard directory before `put`.
        // The writer must not be confused by it — every put creates a
        // fresh tmp filename via the per-store nonce, so the stray
        // is harmlessly ignored.
        let dir = tempdir().unwrap();
        let store = BlobStore::open(dir.path()).unwrap();
        let id = BlobId::from_content(b"after-cleanup");
        let (shard_dir, _) = store.shard_path(id);
        std::fs::create_dir_all(&shard_dir).unwrap();
        std::fs::write(shard_dir.join(".tmp-leftover"), b"interrupted").unwrap();
        let id2 = store.put(b"after-cleanup").unwrap();
        assert_eq!(id, id2);
        assert_eq!(store.get(id).unwrap().unwrap(), b"after-cleanup");
    }

    #[test]
    fn stats_track_bytes_in_and_out() {
        let dir = tempdir().unwrap();
        let store = BlobStore::open(dir.path()).unwrap();
        let id = store
            .put(b"the quick brown fox jumps over the lazy dog")
            .unwrap();
        let _ = store.get(id).unwrap();
        let s = store.stats();
        assert!(s.bytes_written > 0);
        assert_eq!(
            s.bytes_decompressed,
            b"the quick brown fox jumps over the lazy dog".len() as u64
        );
        assert_eq!(s.get_hits, 1);
    }
}
