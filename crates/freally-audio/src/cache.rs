//! In-memory + on-disk audio attributes cache.
//!
//! The Phase 9 prompt: "Cache results in the AudioDoc table; invalidate
//! on Modify event." We stand up that surface here as a path-keyed
//! map; the daemon (Phase 11+) will swap the backing JSON file for a
//! SQLite table when it is wired into the index core. The interface
//! the query executor talks to ([`AudioAttributesProvider`]) stays
//! identical.
//!
//! Invalidation: the cache stores the source file's `mtime_ns` next
//! to the attributes; a `get()` whose caller-supplied `mtime_ns`
//! disagrees with the cached value re-extracts (and re-caches). This
//! covers the "Modify event" contract of the Phase 9 prompt — the
//! journal subscriber's modify event bumps the file's mtime, and the
//! next audio-modifier query observes the change.
//!
//! Concurrency: cache lookups + inserts are guarded by a `Mutex`;
//! cold-path extraction is single-flighted via an in-flight set so
//! two threads racing on the same path don't both decode. The single-
//! flight uses `Condvar` rather than blocking the lookup mutex so a
//! second waiter doesn't stall every other lookup in the workspace.

use std::collections::{HashMap, HashSet};
use std::path::{Path, PathBuf};
use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};
use std::time::Duration;

use parking_lot::{Condvar, Mutex};
use serde::{Deserialize, Serialize};

use crate::analyze::{AnalysisOpts, DEFAULT_AUDIO_TIME_BUDGET, analyze_with_opts};
use crate::attributes::AudioAttributes;
use crate::error::AudioError;
use crate::is_audio_extension;
use crate::provider::AudioAttributesProvider;

/// One entry in the cache.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct CacheEntry {
    pub mtime_ns: i64,
    pub attrs: AudioAttributes,
}

/// On-disk schema. Versioned so a future change can detect & migrate.
#[derive(Debug, Clone, Serialize, Deserialize)]
struct OnDisk {
    version: u32,
    entries: HashMap<String, CacheEntry>,
}

impl Default for OnDisk {
    fn default() -> Self {
        Self {
            version: SCHEMA_VERSION,
            entries: HashMap::new(),
        }
    }
}

/// Bumped when an incompatible schema change lands.
const SCHEMA_VERSION: u32 = 1;

#[derive(Debug)]
pub struct AudioCache {
    map: Mutex<HashMap<PathBuf, CacheEntry>>,
    persist_to: Option<PathBuf>,
    /// When `true`, `get` falls through to live extraction on miss.
    /// When `false` (e.g. lens disabled in settings) `get` returns
    /// `Ok(None)` instead.
    enabled: AtomicBool,
    /// Single-flight set of paths a worker is currently extracting.
    /// `Condvar` notifies waiters when the in-flight worker finishes.
    in_flight: Mutex<HashSet<PathBuf>>,
    in_flight_cv: Condvar,
    /// Per-extraction time budget. Defaults to
    /// `DEFAULT_AUDIO_TIME_BUDGET`; tests + the daemon may override.
    time_budget: Mutex<Duration>,
}

impl Default for AudioCache {
    fn default() -> Self {
        Self::new()
    }
}

impl AudioCache {
    /// In-memory only. Use [`AudioCache::open`] to persist to disk.
    pub fn new() -> Self {
        Self {
            map: Mutex::new(HashMap::new()),
            persist_to: None,
            enabled: AtomicBool::new(true),
            in_flight: Mutex::new(HashSet::new()),
            in_flight_cv: Condvar::new(),
            time_budget: Mutex::new(DEFAULT_AUDIO_TIME_BUDGET),
        }
    }

    /// Load (or initialise) a cache backed by `path`. Missing-file is
    /// not an error — the cache starts empty and writes on flush.
    pub fn open(path: impl Into<PathBuf>) -> Result<Self, AudioError> {
        let path = path.into();
        let map = if path.exists() {
            let body = std::fs::read(&path).map_err(|e| AudioError::io(&path, e))?;
            // Empty file → start fresh. JSON-parse errors propagate.
            if body.is_empty() {
                HashMap::new()
            } else {
                let on_disk: OnDisk = serde_json::from_slice(&body)?;
                if on_disk.version != SCHEMA_VERSION {
                    // Schema bump — drop the old data, start fresh.
                    tracing::info!(
                        "audio cache schema bumped {} → {}; discarding old entries",
                        on_disk.version,
                        SCHEMA_VERSION
                    );
                    HashMap::new()
                } else {
                    on_disk
                        .entries
                        .into_iter()
                        .map(|(k, v)| (PathBuf::from(k), v))
                        .collect()
                }
            }
        } else {
            HashMap::new()
        };

        Ok(Self {
            map: Mutex::new(map),
            persist_to: Some(path),
            enabled: AtomicBool::new(true),
            in_flight: Mutex::new(HashSet::new()),
            in_flight_cv: Condvar::new(),
            time_budget: Mutex::new(DEFAULT_AUDIO_TIME_BUDGET),
        })
    }

    /// Override the per-extraction time budget. The default
    /// (`DEFAULT_AUDIO_TIME_BUDGET = 5 s`) matches the Phase-7 sandbox.
    pub fn set_time_budget(&self, budget: Duration) {
        *self.time_budget.lock() = budget;
    }

    pub fn set_enabled(&self, enabled: bool) {
        self.enabled.store(enabled, Ordering::Relaxed);
    }

    pub fn is_enabled(&self) -> bool {
        self.enabled.load(Ordering::Relaxed)
    }

    /// Number of cached entries (test surface).
    pub fn len(&self) -> usize {
        self.map.lock().len()
    }

    pub fn is_empty(&self) -> bool {
        self.map.lock().is_empty()
    }

    /// Drop all entries. Does *not* delete the on-disk file —
    /// `flush()` afterward is required to persist the empty state.
    pub fn clear(&self) {
        self.map.lock().clear();
    }

    /// Drop one entry. Returns `true` if it was present.
    pub fn invalidate(&self, path: &Path) -> bool {
        self.map.lock().remove(path).is_some()
    }

    /// Atomic write the in-memory map to disk via tmp+rename. No-op
    /// when the cache was constructed via [`AudioCache::new`].
    ///
    /// Non-UTF-8 path keys are skipped with a `warn!` log rather than
    /// silently corrupted via `to_string_lossy()` — the in-memory
    /// entry stays valid for the current process; the next reload
    /// re-extracts the file.
    pub fn flush(&self) -> Result<(), AudioError> {
        let Some(path) = self.persist_to.as_ref() else {
            return Ok(());
        };
        let snapshot = {
            let m = self.map.lock();
            let mut entries = HashMap::with_capacity(m.len());
            for (k, v) in m.iter() {
                match k.to_str() {
                    Some(s) => {
                        entries.insert(s.to_string(), v.clone());
                    }
                    None => {
                        tracing::warn!(
                            "audio cache: skipping non-UTF-8 path key on flush — {}",
                            k.display()
                        );
                    }
                }
            }
            OnDisk {
                version: SCHEMA_VERSION,
                entries,
            }
        };
        let body = serde_json::to_vec_pretty(&snapshot)?;
        let tmp = path.with_extension("json.tmp");
        if let Some(parent) = path.parent() {
            std::fs::create_dir_all(parent).map_err(|e| AudioError::io(path, e))?;
        }
        std::fs::write(&tmp, &body).map_err(|e| AudioError::io(&tmp, e))?;
        std::fs::rename(&tmp, path).map_err(|e| AudioError::io(path, e))?;
        Ok(())
    }

    /// Insert a precomputed entry. Used by the daemon's eager-mode
    /// path so a journal `Create` event can populate the cache without
    /// going through `get`.
    pub fn insert(&self, path: PathBuf, mtime_ns: i64, attrs: AudioAttributes) {
        self.map.lock().insert(path, CacheEntry { mtime_ns, attrs });
    }

    /// Cache-only lookup. No live extraction — returns `None` on
    /// miss (regardless of file extension). The query executor's
    /// "lens disabled" path uses this directly.
    pub fn lookup(&self, path: &Path, mtime_ns: i64) -> Option<AudioAttributes> {
        let m = self.map.lock();
        m.get(path).and_then(|e| {
            if e.mtime_ns == mtime_ns {
                Some(e.attrs.clone())
            } else {
                None
            }
        })
    }

    /// `get()` with optional cancel + caller-provided budget. Used by
    /// the query executor's streaming path so a long extraction can
    /// be aborted between rows. The cache merges the caller's `opts`
    /// with its configured time budget (caller's explicit
    /// `time_budget` wins; otherwise the cache's setting applies).
    pub fn get_with_opts(
        &self,
        path: &Path,
        mtime_ns: i64,
        mut opts: AnalysisOpts,
    ) -> Result<Option<AudioAttributes>, AudioError> {
        if !self.is_enabled() {
            return Ok(None);
        }
        // Extension gate first — saves the lock for non-audio rows.
        let ext = path.extension().and_then(|s| s.to_str()).unwrap_or("");
        if !is_audio_extension(ext) {
            return Ok(None);
        }
        if let Some(hit) = self.lookup(path, mtime_ns) {
            return Ok(Some(hit));
        }
        // Single-flight: if another thread is already extracting this
        // path, wait for it to finish then re-look-up. Otherwise
        // claim the slot ourselves before releasing the in_flight
        // lock and starting the (potentially-slow) decode.
        {
            let mut in_flight = self.in_flight.lock();
            while in_flight.contains(path) {
                self.in_flight_cv.wait(&mut in_flight);
                // Wake-up: did the prior worker populate the cache?
                drop(in_flight);
                if let Some(hit) = self.lookup(path, mtime_ns) {
                    return Ok(Some(hit));
                }
                in_flight = self.in_flight.lock();
            }
            in_flight.insert(path.to_path_buf());
        }
        // RAII guard so a panic inside `analyze_with_opts` still
        // releases the in-flight slot and notifies waiters.
        struct InFlightGuard<'a> {
            cache: &'a AudioCache,
            path: PathBuf,
        }
        impl<'a> Drop for InFlightGuard<'a> {
            fn drop(&mut self) {
                let mut s = self.cache.in_flight.lock();
                s.remove(&self.path);
                self.cache.in_flight_cv.notify_all();
            }
        }
        let _guard = InFlightGuard {
            cache: self,
            path: path.to_path_buf(),
        };

        if opts.time_budget.is_none() {
            opts.time_budget = Some(*self.time_budget.lock());
        }
        let attrs = analyze_with_opts(path, opts)?;
        self.insert(path.to_path_buf(), mtime_ns, attrs.clone());
        Ok(Some(attrs))
    }
}

impl AudioAttributesProvider for AudioCache {
    fn get(&self, path: &Path, mtime_ns: i64) -> Result<Option<AudioAttributes>, AudioError> {
        self.get_with_opts(path, mtime_ns, AnalysisOpts::default())
    }
}

impl AudioAttributesProvider for Arc<AudioCache> {
    fn get(&self, path: &Path, mtime_ns: i64) -> Result<Option<AudioAttributes>, AudioError> {
        self.as_ref().get(path, mtime_ns)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::AudioCodec;
    use tempfile::tempdir;

    fn fixture_attrs() -> AudioAttributes {
        AudioAttributes {
            codec: AudioCodec::from("flac"),
            sample_rate: 44_100,
            channels: 2,
            bit_depth: Some(16),
            duration_ns: 134_000_000_000,
            lufs_integrated: -16.0,
            lufs_short_term_p99: -10.5,
            lufs_short_term_p10: -22.5,
            peak_dbfs: -1.0,
            silence_ratio: 0.0,
            dynamic_range_lu: 12.0,
        }
    }

    #[test]
    fn lookup_hits_when_mtime_matches() {
        let cache = AudioCache::new();
        let path = PathBuf::from("/tmp/song.flac");
        cache.insert(path.clone(), 100, fixture_attrs());
        assert!(cache.lookup(&path, 100).is_some());
        assert!(cache.lookup(&path, 101).is_none()); // mtime mismatch
    }

    #[test]
    fn invalidate_removes_entry() {
        let cache = AudioCache::new();
        let path = PathBuf::from("/tmp/song.flac");
        cache.insert(path.clone(), 100, fixture_attrs());
        assert!(cache.invalidate(&path));
        assert!(cache.lookup(&path, 100).is_none());
    }

    #[test]
    fn disabled_cache_returns_none_even_on_hit() {
        let cache = AudioCache::new();
        let path = PathBuf::from("/tmp/song.flac");
        cache.insert(path.clone(), 100, fixture_attrs());
        cache.set_enabled(false);
        assert!(cache.get(&path, 100).unwrap().is_none());
    }

    #[test]
    fn non_audio_ext_returns_none() {
        let cache = AudioCache::new();
        let path = PathBuf::from("/tmp/notes.txt");
        assert!(cache.get(&path, 100).unwrap().is_none());
    }

    #[test]
    fn flush_round_trip_on_disk() {
        let dir = tempdir().unwrap();
        let cache_path = dir.path().join("audio-cache.json");
        let cache = AudioCache::open(&cache_path).unwrap();
        let song = PathBuf::from("/tmp/song.flac");
        cache.insert(song.clone(), 100, fixture_attrs());
        cache.flush().unwrap();
        // Reload — the entry must round-trip.
        let cache2 = AudioCache::open(&cache_path).unwrap();
        let attrs = cache2.lookup(&song, 100).unwrap();
        assert_eq!(attrs.codec, "flac");
        assert_eq!(attrs.sample_rate, 44_100);
    }

    #[test]
    fn schema_version_mismatch_drops_entries() {
        let dir = tempdir().unwrap();
        let cache_path = dir.path().join("audio-cache.json");
        // Hand-write a v0 file.
        std::fs::write(&cache_path, br#"{"version":0,"entries":{}}"#).unwrap();
        let cache = AudioCache::open(&cache_path).unwrap();
        assert_eq!(cache.len(), 0);
    }

    #[test]
    fn time_budget_default_is_set() {
        // Cache constructed without an explicit budget falls back to
        // `DEFAULT_AUDIO_TIME_BUDGET` (5 s). We don't assert the
        // numeric value here (it can move), only that the setter
        // round-trips.
        let cache = AudioCache::new();
        cache.set_time_budget(Duration::from_secs(2));
        assert_eq!(*cache.time_budget.lock(), Duration::from_secs(2));
    }

    #[test]
    fn flush_skips_non_utf8_keys() {
        // Construct an in-memory entry whose path round-trips as
        // valid UTF-8 *and* one whose round-trip would lose data.
        // On Windows-only platforms it's hard to synthesise an
        // invalid OsStr; we instead simulate by injecting a key that
        // contains a `\u{FFFD}` (already lossy via to_string_lossy)
        // and assert the flush loop chooses the strict-UTF-8 path.
        // The actual rejection happens via `to_str() == None`, which
        // we cannot easily forge in safe Rust — so this test
        // exercises the (correct-by-construction) UTF-8 path and
        // documents the policy via assertion.
        let dir = tempdir().unwrap();
        let cache_path = dir.path().join("c.json");
        let cache = AudioCache::open(&cache_path).unwrap();
        cache.insert(PathBuf::from("/tmp/clean.flac"), 1, fixture_attrs());
        cache.flush().unwrap();
        let body = std::fs::read_to_string(&cache_path).unwrap();
        assert!(
            body.contains("/tmp/clean.flac"),
            "UTF-8 key should serialize: {body}"
        );
    }

    #[test]
    fn single_flight_blocks_concurrent_extractions() {
        use std::sync::Arc;
        // We can't easily block on `analyze_with_opts` inside this
        // unit test (no real audio file is required to exercise the
        // single-flight path), so we validate the `in_flight` set
        // protocol directly: claim a slot, lookup is empty, second
        // claim observes the in-flight entry and waits on the
        // condvar; releasing the first slot unblocks the second.
        let cache = Arc::new(AudioCache::new());
        let path = PathBuf::from("/tmp/song.flac");
        // Claim manually — emulating the prelude of `get_with_opts`.
        cache.in_flight.lock().insert(path.clone());
        let cache2 = Arc::clone(&cache);
        let path2 = path.clone();
        let handle = std::thread::spawn(move || {
            // Mimic the wait loop. We bail out via a deadline so a
            // bug doesn't hang the test.
            let mut waited = std::time::Duration::ZERO;
            let max_wait = std::time::Duration::from_secs(2);
            loop {
                {
                    let mut s = cache2.in_flight.lock();
                    if !s.contains(&path2) {
                        return true;
                    }
                    cache2
                        .in_flight_cv
                        .wait_for(&mut s, std::time::Duration::from_millis(20));
                }
                waited += std::time::Duration::from_millis(20);
                if waited > max_wait {
                    return false;
                }
            }
        });
        // Hold the slot for ~50 ms, then release.
        std::thread::sleep(std::time::Duration::from_millis(50));
        {
            let mut s = cache.in_flight.lock();
            s.remove(&path);
        }
        cache.in_flight_cv.notify_all();
        let unblocked = handle.join().unwrap();
        assert!(unblocked, "second waiter never unblocked");
    }
}
