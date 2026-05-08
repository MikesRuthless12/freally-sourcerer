//! Per-watch cursor for the Linux subscriber.
//!
//! Tracks bootstrap state, the watched root's `st_dev`, the chosen backend
//! (inotify or fanotify), and an advisory `last_event_time_ns` for resume
//! diagnostics. Cursors live under `~/.local/share/sourcerer/cursors/` —
//! the XDG_DATA_HOME location the Phase-0 spec wired for Linux.
//!
//! Why a cursor at all on Linux: neither inotify nor fanotify expose a
//! resumable global event ID (FSEvents on macOS does; the USN journal on
//! Windows does). The cursor's job here is therefore narrower —
//!
//! 1. Remember whether the heavy `getdents64` bootstrap already ran for
//!    this root, so a daemon restart skips straight to live events.
//! 2. Lock cursor reuse to the original device. If the user re-mounts a
//!    different volume at the same path, `device` mismatches and we
//!    bootstrap from scratch instead of trusting the prior state.
//! 3. Surface `backend` for diagnostics so a `sourcerer-indexd doctor`
//!    can tell at a glance whether the daemon is on the elevated
//!    fanotify path or the unprivileged inotify default.
//!
//! Why hash the path: cursor files are one-per-watch-root, and watch
//! roots can contain spaces / slashes / unprintables that don't survive
//! round-trip through a filename. We use a 64-bit FNV-1a fingerprint of
//! the UTF-8 form (matches Phase 2 / FSEvents); collisions in a single
//! user's watch set are not a real concern, and the file embeds the
//! original path so a mistaken collision would surface as "wrong root"
//! on the next load.

use std::path::{Path, PathBuf};

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum WatchBackend {
    /// Default unprivileged path: recursive `inotify_add_watch` on every
    /// directory under the root.
    Inotify,
    /// Elevated path requiring CAP_SYS_ADMIN. Subscribes once via
    /// `fanotify_mark(FAN_MARK_FILESYSTEM)` with `FAN_REPORT_FID` so
    /// rename tracking survives overlayfs / Btrfs subvolume crossings
    /// that inotify cannot follow.
    Fanotify,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct WatchCursor {
    /// The watched root path, as supplied by the caller (canonicalized).
    pub root: PathBuf,
    /// `stat.st_dev` of `root` at open time. Mismatch on reload means the
    /// volume changed; cursor is discarded and we re-bootstrap.
    pub device: u64,
    /// Filesystem name reported by `statfs.f_fstypename` (e.g. `ext4`,
    /// `btrfs`, `zfs`, `xfs`). Diagnostics only.
    pub fs_name: String,
    /// Which backend the subscriber chose at open time. `Fanotify` only
    /// when CAP_SYS_ADMIN is present; otherwise `Inotify`.
    pub backend: WatchBackend,
    /// Whether the bootstrap walk has run at least once for this cursor.
    pub bootstrap_complete: bool,
    /// Wall-clock nanoseconds since the UNIX epoch of the last event we
    /// emitted under this cursor. Advisory only — neither inotify nor
    /// fanotify lets us resume from here on next open. Surfaced for
    /// diagnostics and for the indexd "freshness" UI.
    pub last_event_time_ns: i128,
}

impl WatchCursor {
    /// Default cursor root: `$XDG_DATA_HOME/sourcerer/cursors/` (defaults
    /// to `~/.local/share/sourcerer/cursors/` per the XDG basedir spec).
    /// Falls back to `<system_temp>/sourcerer/cursors` when neither
    /// `XDG_DATA_HOME` nor `HOME` is set (e.g. systemd unit contexts that
    /// strip env vars).
    pub fn default_root() -> PathBuf {
        if let Some(xdg) = std::env::var_os("XDG_DATA_HOME") {
            return PathBuf::from(xdg).join("sourcerer").join("cursors");
        }
        if let Some(home) = std::env::var_os("HOME") {
            return PathBuf::from(home)
                .join(".local")
                .join("share")
                .join("sourcerer")
                .join("cursors");
        }
        std::env::temp_dir().join("sourcerer").join("cursors")
    }

    /// Returns the cursor file path for the given watch root under the
    /// supplied cursor directory.
    pub fn path_in(cursor_root: &Path, watch_root: &Path) -> PathBuf {
        cursor_root.join(format!("{}.json", stable_key(watch_root)))
    }

    /// Loads a cursor from disk. Returns `Ok(None)` if the file does not
    /// exist (first-run case) or if the on-disk cursor's `root` differs
    /// from `watch_root` — guards against the (vanishingly rare) case
    /// where two distinct watch roots produce the same FNV-1a digest and
    /// would otherwise share a cursor file.
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
    /// from the previous saved cursor, or re-bootstrap on first run).
    /// Recovery is cheap: a stale `bootstrap_complete=false` triggers a
    /// fresh `getdents64` walk; a stale device id triggers the same.
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

/// Stable, filename-safe key derived from a path's UTF-8 bytes via FNV-1a
/// (matches the Phase-2 macOS implementation so a bug fixed in one place
/// can be ported across without re-deriving the hash).
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

    fn sample() -> WatchCursor {
        WatchCursor {
            root: PathBuf::from("/home/alice/Documents"),
            device: 0x0123_4567_89AB_CDEF,
            fs_name: "ext4".to_string(),
            backend: WatchBackend::Inotify,
            bootstrap_complete: true,
            last_event_time_ns: 1_700_000_000_000_000_000,
        }
    }

    #[test]
    fn round_trip_through_json() {
        let c = sample();
        let bytes = serde_json::to_vec(&c).unwrap();
        let back: WatchCursor = serde_json::from_slice(&bytes).unwrap();
        assert_eq!(c, back);
    }

    #[test]
    fn save_then_load_round_trip() {
        let dir = tempfile::tempdir().unwrap();
        let c = sample();
        c.save(dir.path()).unwrap();
        let loaded = WatchCursor::load(dir.path(), &c.root).unwrap().unwrap();
        assert_eq!(c, loaded);
    }

    #[test]
    fn load_returns_none_when_absent() {
        let dir = tempfile::tempdir().unwrap();
        let loaded = WatchCursor::load(dir.path(), Path::new("/missing/root")).unwrap();
        assert!(loaded.is_none());
    }

    #[test]
    fn load_discards_cursor_when_root_mismatches() {
        // Stage a cursor whose stored `root` differs from the requested
        // watch_root. `load()` must treat the mismatch as "not mine" so
        // a hash collision does not silently resume against the wrong
        // device / bootstrap state.
        let dir = tempfile::tempdir().unwrap();
        let requested = PathBuf::from("/home/alice/Documents");
        let stored = WatchCursor {
            root: PathBuf::from("/home/bob/SomethingElse"),
            ..sample()
        };
        let file_path = WatchCursor::path_in(dir.path(), &requested);
        std::fs::write(&file_path, serde_json::to_vec(&stored).unwrap()).unwrap();

        let loaded = WatchCursor::load(dir.path(), &requested).unwrap();
        assert!(loaded.is_none());
    }

    #[test]
    fn cursor_path_uses_stable_key() {
        let p1 = WatchCursor::path_in(Path::new("/cursors"), Path::new("/home/alice/Documents"));
        let p2 = WatchCursor::path_in(Path::new("/cursors"), Path::new("/home/alice/Documents"));
        let p3 = WatchCursor::path_in(Path::new("/cursors"), Path::new("/home/alice/Pictures"));
        assert_eq!(p1, p2);
        assert_ne!(p1, p3);
        assert!(p1.to_string_lossy().ends_with(".json"));
    }

    #[test]
    fn stable_key_is_16_hex_chars_and_deterministic_within_a_run() {
        let k = stable_key(Path::new("/home/alice/Documents"));
        assert_eq!(k.len(), 16);
        assert!(k.chars().all(|c| c.is_ascii_hexdigit()));
        assert_eq!(k, stable_key(Path::new("/home/alice/Documents")));
    }

    #[test]
    fn default_root_prefers_xdg_data_home() {
        // The XDG basedir spec says XDG_DATA_HOME wins over the
        // ~/.local/share default. Locking that in here so a future
        // refactor doesn't accidentally reorder the env probes.
        let prior_xdg = std::env::var_os("XDG_DATA_HOME");
        let prior_home = std::env::var_os("HOME");
        // SAFETY: tests in this crate are single-threaded for env access.
        unsafe {
            std::env::set_var("XDG_DATA_HOME", "/tmp/xdg-data-home");
            std::env::set_var("HOME", "/tmp/home");
        }
        let root = WatchCursor::default_root();
        assert_eq!(
            root,
            PathBuf::from("/tmp/xdg-data-home/sourcerer/cursors"),
            "XDG_DATA_HOME must take precedence over HOME"
        );
        unsafe {
            match prior_xdg {
                Some(v) => std::env::set_var("XDG_DATA_HOME", v),
                None => std::env::remove_var("XDG_DATA_HOME"),
            }
            match prior_home {
                Some(v) => std::env::set_var("HOME", v),
                None => std::env::remove_var("HOME"),
            }
        }
    }

    #[test]
    fn backend_serde_uses_lowercase_strings() {
        // The cursor file is human-inspectable JSON; `lowercase` keeps
        // `"backend": "inotify"` instead of `"Inotify"`.
        let json = serde_json::to_string(&WatchBackend::Inotify).unwrap();
        assert_eq!(json, "\"inotify\"");
        let json = serde_json::to_string(&WatchBackend::Fanotify).unwrap();
        assert_eq!(json, "\"fanotify\"");
    }
}
