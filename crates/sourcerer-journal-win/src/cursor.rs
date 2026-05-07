//! Per-volume USN journal cursor.
//!
//! Tracks where the subscriber left off on a given NTFS volume so a restart
//! resumes from the right USN instead of replaying the entire MFT. Cursors
//! live under `%LOCALAPPDATA%\Sourcerer\cursors\<volume_serial>.json`.
//!
//! `journal_id` doubles as a tamper / recreate marker: if the on-disk
//! `journal_id` no longer matches the volume's reported `UsnJournalID`, the
//! caller must discard `next_usn` and bootstrap from the MFT instead of
//! reading from the persisted USN.

use std::path::{Path, PathBuf};

use serde::{Deserialize, Serialize};

/// Stable per-volume cursor.
///
/// Serialized to JSON for human inspection during early phase work.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct VolumeCursor {
    /// Volume serial number from `GetVolumeInformationW`. Identifies the
    /// physical volume so a remounted USB stick keeps its cursor.
    pub volume_serial: u32,
    /// Volume's USN journal ID at the time of capture. Mismatch on
    /// reload means the journal was deleted/recreated; bootstrap again.
    pub journal_id: u64,
    /// Next USN to read on `subscribe()`.
    pub next_usn: i64,
    /// Filesystem name reported by `GetVolumeInformationW` (e.g. `NTFS`).
    /// Persisted for diagnostics; not used for resume decisions.
    pub fs_name: String,
}

impl VolumeCursor {
    /// Returns the cursor file path for the given volume serial under the
    /// supplied root directory (typically `%LOCALAPPDATA%\Sourcerer\cursors`).
    pub fn path_in(root: &Path, volume_serial: u32) -> PathBuf {
        root.join(format!("{volume_serial:08x}.json"))
    }

    /// Default cursor root: `%LOCALAPPDATA%\Sourcerer\cursors`. Falls back to
    /// `<system_temp>\Sourcerer\cursors` if `LOCALAPPDATA` is unset (e.g.
    /// service contexts that strip env vars).
    pub fn default_root() -> PathBuf {
        if let Some(local) = std::env::var_os("LOCALAPPDATA") {
            PathBuf::from(local).join("Sourcerer").join("cursors")
        } else {
            std::env::temp_dir().join("Sourcerer").join("cursors")
        }
    }

    /// Loads a cursor from disk. Returns `Ok(None)` if the file does not
    /// exist (first-run case).
    pub fn load(root: &Path, volume_serial: u32) -> Result<Option<Self>, CursorError> {
        let path = Self::path_in(root, volume_serial);
        match std::fs::read(&path) {
            Ok(bytes) => Ok(Some(serde_json::from_slice(&bytes)?)),
            Err(e) if e.kind() == std::io::ErrorKind::NotFound => Ok(None),
            Err(e) => Err(CursorError::Io(e)),
        }
    }

    /// Persists the cursor with process-crash atomicity: writes to
    /// `<file>.tmp` then renames over the target. A panic / kill -9
    /// mid-save can never strand a zero-byte cursor file.
    ///
    /// **Not power-fail-atomic.** This call does not `FlushFileBuffers`
    /// the data or the parent directory; a hard power loss between
    /// `rename` and the on-disk flush can still lose the most recent
    /// update (you'd resume from the previous saved cursor). The journal
    /// subscriber recovers from a stale cursor by replaying USN events,
    /// or — if the journal was recreated — re-bootstrapping the MFT.
    pub fn save(&self, root: &Path) -> Result<(), CursorError> {
        std::fs::create_dir_all(root)?;
        let path = Self::path_in(root, self.volume_serial);
        let tmp = path.with_extension("json.tmp");
        let bytes = serde_json::to_vec_pretty(self)?;
        std::fs::write(&tmp, bytes)?;
        std::fs::rename(&tmp, &path)?;
        Ok(())
    }
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

    fn sample() -> VolumeCursor {
        VolumeCursor {
            volume_serial: 0xDEAD_BEEF,
            journal_id: 0x1234_5678_9ABC_DEF0,
            next_usn: 0x0011_2233_4455_6677,
            fs_name: "NTFS".to_string(),
        }
    }

    #[test]
    fn round_trip_through_json() {
        let c = sample();
        let bytes = serde_json::to_vec(&c).unwrap();
        let back: VolumeCursor = serde_json::from_slice(&bytes).unwrap();
        assert_eq!(c, back);
    }

    #[test]
    fn save_then_load_round_trip() {
        let dir = tempfile::tempdir().unwrap();
        let c = sample();
        c.save(dir.path()).unwrap();
        let loaded = VolumeCursor::load(dir.path(), c.volume_serial).unwrap().unwrap();
        assert_eq!(c, loaded);
    }

    #[test]
    fn load_returns_none_when_absent() {
        let dir = tempfile::tempdir().unwrap();
        let loaded = VolumeCursor::load(dir.path(), 0xCAFE_BABE).unwrap();
        assert!(loaded.is_none());
    }

    #[test]
    fn cursor_path_uses_serial_as_hex() {
        let p = VolumeCursor::path_in(Path::new("X:\\cursors"), 0x0000_BEEF);
        assert!(p.to_string_lossy().ends_with("0000beef.json"));
    }
}
