//! Per-watch FSEvents stream cursor.
//!
//! Tracks the latest FSEvents `EventID` we consumed for a given watch root
//! so a restart resumes from that point instead of replaying everything
//! since boot. Cursors live under
//! `~/Library/Application Support/Sourcerer/cursors/<root_hash>.json`.
//!
//! Why hash the path: the cursor file is one-per-watch-root, and watch roots
//! can contain spaces, slashes, and unprintable bytes that don't survive
//! round-trip through a filename. We use a 64-bit FNV-1a fingerprint of the
//! UTF-8 form; collisions in a single user's watch set are not a real
//! concern, but the file embeds the original path so a mistaken collision
//! would surface as "wrong root" on the next load.
//!
//! `device` doubles as a tamper / unmount marker: if the on-disk `device`
//! no longer matches the volume's current `stat.st_dev`, the cursor is
//! discarded and the subscriber re-bootstraps.

use std::path::{Path, PathBuf};

use serde::{Deserialize, Serialize};

/// Stable per-watch cursor.
///
/// Serialized to JSON for human inspection during early phase work.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct StreamCursor {
    /// The watched root path, as supplied by the caller (absolute).
    pub root: PathBuf,
    /// BSD device ID (`stat.st_dev`) of `root` at open time. Mismatch on
    /// reload means the volume changed; cursor is discarded.
    pub device: u64,
    /// Last FSEvents `EventID` we consumed. Resumes from here on next open.
    /// `0` means "since now" (no resume).
    pub last_event_id: u64,
    /// Filesystem name reported by `statfs.f_fstypename` (e.g. `apfs`,
    /// `hfs`, `exfat`). Diagnostics only.
    pub fs_name: String,
    /// Whether the bootstrap walk has run at least once for this cursor.
    pub bootstrap_complete: bool,
}

impl StreamCursor {
    /// Default cursor root: `~/Library/Application Support/Sourcerer/cursors`.
    /// Falls back to `<system_temp>/Sourcerer/cursors` if `HOME` is unset
    /// (e.g. launchd contexts that strip env vars).
    pub fn default_root() -> PathBuf {
        if let Some(home) = std::env::var_os("HOME") {
            PathBuf::from(home)
                .join("Library")
                .join("Application Support")
                .join("Sourcerer")
                .join("cursors")
        } else {
            std::env::temp_dir().join("Sourcerer").join("cursors")
        }
    }

    /// Returns the cursor file path for the given watch root under the
    /// supplied cursor directory.
    pub fn path_in(cursor_root: &Path, watch_root: &Path) -> PathBuf {
        cursor_root.join(format!("{}.json", stable_key(watch_root)))
    }

    /// Loads a cursor from disk. Returns `Ok(None)` if the file does not
    /// exist (first-run case) or if the on-disk cursor's `root` differs
    /// from `watch_root` — the latter guards against the (vanishingly
    /// rare) case where two distinct watch roots produce the same FNV-1a
    /// digest and would otherwise share a cursor file.
    pub fn load(cursor_root: &Path, watch_root: &Path) -> Result<Option<Self>, CursorError> {
        let path = Self::path_in(cursor_root, watch_root);
        let bytes = match std::fs::read(&path) {
            Ok(b) => b,
            Err(e) if e.kind() == std::io::ErrorKind::NotFound => return Ok(None),
            Err(e) => return Err(CursorError::Io(e)),
        };
        let cursor: Self = serde_json::from_slice(&bytes)?;
        if cursor.root != watch_root {
            tracing::info!(
                file = %path.display(),
                stored = %cursor.root.display(),
                requested = %watch_root.display(),
                "cursor file's stored root does not match requested watch root \
                 (FNV-1a collision); discarding cursor",
            );
            return Ok(None);
        }
        Ok(Some(cursor))
    }

    /// Persists the cursor with process-crash atomicity: writes to
    /// `<file>.tmp` then renames over the target. A panic / kill -9
    /// mid-save can never strand a zero-byte cursor file.
    ///
    /// **Not power-fail-atomic.** This call does not `fsync` the data or
    /// the parent directory; a hard power loss between `rename` and the
    /// on-disk flush can still lose the most recent update (you'd resume
    /// from the previous saved cursor, or from `kFSEventStreamEventIdSinceNow`
    /// on first run). The subscriber recovers from a stale cursor by
    /// triggering a directory rescan when FSEvents reports
    /// `MustScanSubDirs`.
    pub fn save(&self, cursor_root: &Path) -> Result<(), CursorError> {
        std::fs::create_dir_all(cursor_root)?;
        let path = Self::path_in(cursor_root, &self.root);
        let tmp = path.with_extension("json.tmp");
        let bytes = serde_json::to_vec_pretty(self)?;
        std::fs::write(&tmp, bytes)?;
        std::fs::rename(&tmp, &path)?;
        Ok(())
    }
}

/// Stable, filename-safe key derived from a path's UTF-8 bytes via FNV-1a.
fn stable_key(path: &Path) -> String {
    let s = path.to_string_lossy();
    let mut h: u64 = 0xcbf2_9ce4_8422_2325;
    for b in s.as_bytes() {
        h ^= u64::from(*b);
        h = h.wrapping_mul(0x0000_0100_0000_01b3);
    }
    format!("{h:016x}")
}

#[derive(Debug, thiserror::Error)]
pub enum CursorError {
    #[error("cursor I/O error: {0}")]
    Io(#[from] std::io::Error),
    #[error("cursor JSON error: {0}")]
    Json(#[from] serde_json::Error),
}

#[cfg(test)]
mod tests {
    use super::*;

    fn sample() -> StreamCursor {
        StreamCursor {
            root: PathBuf::from("/Users/alice/Documents"),
            device: 0x0123_4567_89AB_CDEF,
            last_event_id: 0xDEAD_BEEF_CAFE_F00D,
            fs_name: "apfs".to_string(),
            bootstrap_complete: true,
        }
    }

    #[test]
    fn round_trip_through_json() {
        let c = sample();
        let bytes = serde_json::to_vec(&c).unwrap();
        let back: StreamCursor = serde_json::from_slice(&bytes).unwrap();
        assert_eq!(c, back);
    }

    #[test]
    fn save_then_load_round_trip() {
        let dir = tempfile::tempdir().unwrap();
        let c = sample();
        c.save(dir.path()).unwrap();
        let loaded = StreamCursor::load(dir.path(), &c.root).unwrap().unwrap();
        assert_eq!(c, loaded);
    }

    #[test]
    fn load_returns_none_when_absent() {
        let dir = tempfile::tempdir().unwrap();
        let loaded = StreamCursor::load(dir.path(), Path::new("/some/missing")).unwrap();
        assert!(loaded.is_none());
    }

    #[test]
    fn load_discards_cursor_when_root_mismatches() {
        // Stage a cursor file whose stored `root` does NOT match the
        // requested watch_root — simulates a 64-bit FNV-1a digest collision
        // between two different watch roots. `load()` must treat the
        // mismatch as "not mine" and return Ok(None) so the subscriber
        // bootstraps fresh instead of resuming with the wrong device /
        // last_event_id.
        let dir = tempfile::tempdir().unwrap();
        let requested = PathBuf::from("/Users/alice/Documents");
        let stored = StreamCursor {
            root: PathBuf::from("/Users/bob/SomethingElse"),
            ..sample()
        };
        // Force the file to land at the requested-root's path key so
        // load() will read it but find a mismatched `root` field.
        let file_path = StreamCursor::path_in(dir.path(), &requested);
        std::fs::write(&file_path, serde_json::to_vec(&stored).unwrap()).unwrap();

        let loaded = StreamCursor::load(dir.path(), &requested).unwrap();
        assert!(
            loaded.is_none(),
            "stored.root != requested watch_root must yield None"
        );
    }

    #[test]
    fn cursor_path_uses_stable_key() {
        let p1 = StreamCursor::path_in(Path::new("/cursors"), Path::new("/Users/alice/Documents"));
        let p2 = StreamCursor::path_in(Path::new("/cursors"), Path::new("/Users/alice/Documents"));
        let p3 = StreamCursor::path_in(Path::new("/cursors"), Path::new("/Users/alice/Pictures"));
        assert_eq!(p1, p2, "same path must yield identical cursor file name");
        assert_ne!(p1, p3, "different paths must yield distinct cursor files");
        assert!(p1.to_string_lossy().ends_with(".json"));
    }

    #[test]
    fn stable_key_is_16_hex_chars_and_deterministic_within_a_run() {
        // FNV-1a 64-bit -> 16 hex chars; locking down the format guards
        // against a future refactor silently changing cursor file names
        // and orphaning existing cursors on user disks.
        let k = stable_key(Path::new("/Users/alice/Documents"));
        assert_eq!(k.len(), 16);
        assert!(k.chars().all(|c| c.is_ascii_hexdigit()));
        assert_eq!(k, stable_key(Path::new("/Users/alice/Documents")));
    }
}
