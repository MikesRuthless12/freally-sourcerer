//! SRC-M21 — what the index could not see, and why.
//!
//! A scan that cannot read a directory logs a warning and moves on. That
//! is the right behaviour — one locked folder must not abort an
//! afternoon's indexing — but it means the answer to "why isn't my file
//! showing up?" is buried in a log the user will never open, and the
//! result list looks identical whether a subtree was empty or
//! unreadable. Silence about what search *cannot* see is the worst
//! possible answer.
//!
//! This ledger records those skips so the UI can show them, grouped by
//! volume, with the per-OS fix.
//!
//! **Roots, not files.** `walkdir` reports one error for a directory it
//! cannot open and does not descend into it, so what lands here is
//! naturally the root of each unreadable subtree rather than every file
//! underneath. That is also what the user needs: "grant access to this
//! folder", not a list of ten thousand files inside it.

use std::path::{Path, PathBuf};

use serde::{Deserialize, Serialize};

/// Hard cap on retained entries.
///
/// Scanning a volume with a large protected tree can produce a lot of
/// these, and this list is held in memory, serialized to JSON, and sent
/// over IPC. Past a few thousand it stops being a report a person reads
/// and starts being a denial-of-service on the daemon's own memory. The
/// overflow is counted so the UI can say "and 12,043 more" rather than
/// quietly implying the list is complete.
pub const MAX_ENTRIES: usize = 2_000;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SkipReason {
    /// The OS refused the read. This is the one with a user-facing fix.
    PermissionDenied,
    /// Vanished between being listed and being opened — normal on a
    /// live filesystem, and not worth telling the user to fix.
    NotFound,
    /// Anything else: I/O error, too many links, a device that went
    /// away mid-scan.
    Other,
}

impl SkipReason {
    pub fn from_io(e: &std::io::Error) -> Self {
        match e.kind() {
            std::io::ErrorKind::PermissionDenied => SkipReason::PermissionDenied,
            std::io::ErrorKind::NotFound => SkipReason::NotFound,
            _ => SkipReason::Other,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SkippedPath {
    pub path: PathBuf,
    pub reason: SkipReason,
    /// The OS's own message. Kept verbatim — it names the actual
    /// mechanism ("Operation not permitted" vs "Access is denied"),
    /// which is the difference between a TCC prompt and an ACL.
    pub detail: String,
    /// Volume this path belongs to, so the report can group by device.
    pub volume: String,
}

/// Everything a scan could not read, grouped for display.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct PermissionLedger {
    entries: Vec<SkippedPath>,
    /// The paths already in `entries`, for the dedupe test in
    /// [`Self::record`]. Derived state, so it is rebuilt on
    /// deserialization rather than persisted — see `seen()`.
    #[serde(skip)]
    seen: std::collections::HashSet<PathBuf>,
    /// Entries dropped after [`MAX_ENTRIES`], by reason. Counted rather
    /// than stored so the totals stay honest.
    dropped: u64,
}

impl PermissionLedger {
    pub fn record(&mut self, path: &Path, err: &std::io::Error, volume: &str) {
        let reason = SkipReason::from_io(err);
        // A path that vanished mid-scan is noise: nothing is broken and
        // there is nothing to fix. Counting it would make every report
        // on a busy machine look alarming.
        if reason == SkipReason::NotFound {
            return;
        }
        if self.entries.len() >= MAX_ENTRIES {
            self.dropped += 1;
            return;
        }
        // A rescan re-walks the same tree, so the same directory would
        // otherwise be recorded again on every pass. This runs from the
        // scanner's error path, once per unreadable directory, so the
        // linear scan it replaces was O(entries) per call all the way up
        // to `MAX_ENTRIES`.
        self.rebuild_seen_after_load();
        if !self.seen.insert(path.to_path_buf()) {
            return;
        }
        self.entries.push(SkippedPath {
            path: path.to_path_buf(),
            reason,
            detail: err.to_string(),
            volume: volume.to_string(),
        });
    }

    /// Re-derive `seen` after a load from disk.
    ///
    /// `seen` is `#[serde(skip)]`, so deserialization is the *only*
    /// thing that can leave it out of step: a ledger read back off disk
    /// arrives with entries and an empty set. Every mutator in this file
    /// keeps the two together, so an empty set beside a non-empty
    /// `entries` is exactly that case and nothing else.
    ///
    /// Testing emptiness rather than comparing lengths on purpose: a
    /// length check reads as a general staleness probe, and would
    /// silently report "fresh" for a future count-preserving edit to
    /// `entries` that it cannot actually detect.
    fn rebuild_seen_after_load(&mut self) {
        if !self.seen.is_empty() || self.entries.is_empty() {
            return;
        }
        self.seen = self.entries.iter().map(|e| e.path.clone()).collect();
    }

    /// Drop everything under `root` before rescanning it, so a fixed
    /// permission stops being reported.
    pub fn clear_under(&mut self, root: &Path) {
        self.entries.retain(|e| !e.path.starts_with(root));
        self.seen.retain(|p| !p.starts_with(root));
        // `dropped` counted entries we never kept, so we cannot tell
        // which root they belonged to. Clearing it on any rescan is the
        // honest choice: the alternative is a number that only ever
        // grows and describes a scan that no longer happened.
        self.dropped = 0;
    }

    pub fn entries(&self) -> &[SkippedPath] {
        &self.entries
    }

    pub fn dropped(&self) -> u64 {
        self.dropped
    }

    /// Skips the user can actually do something about.
    pub fn denied_count(&self) -> usize {
        self.entries
            .iter()
            .filter(|e| e.reason == SkipReason::PermissionDenied)
            .count()
    }

    pub fn is_empty(&self) -> bool {
        self.entries.is_empty() && self.dropped == 0
    }
}

// ---------- the report the UI renders -----------------------------------

/// Which guided fix applies here. Resolved on the daemon side so the
/// answer follows the machine the index actually lives on, not the
/// machine the UI happens to be rendering on — they differ whenever a
/// client is pointed at a remote endpoint.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Guidance {
    /// macOS: the folder is behind TCC. Full Disk Access fixes it.
    MacosFullDiskAccess,
    /// Linux: an unreadable mode/ACL, or a mount the user cannot
    /// traverse. Elevation or a mount option fixes it.
    LinuxPermissions,
    /// Windows: an NTFS ACL denies the current user.
    WindowsAcl,
    Unknown,
}

pub fn platform_guidance() -> Guidance {
    #[cfg(target_os = "macos")]
    {
        Guidance::MacosFullDiskAccess
    }
    #[cfg(target_os = "linux")]
    {
        Guidance::LinuxPermissions
    }
    #[cfg(windows)]
    {
        Guidance::WindowsAcl
    }
    #[cfg(not(any(target_os = "macos", target_os = "linux", windows)))]
    {
        Guidance::Unknown
    }
}

/// On macOS, is Full Disk Access already granted?
///
/// Probed by reading a directory that TCC protects unconditionally. If
/// the read succeeds this process holds FDA; `PermissionDenied` means it
/// does not. Any other error (the path not existing on a fresh account)
/// is inconclusive, and inconclusive must not render as "not granted" —
/// telling someone to fix a setting that is already correct is worse
/// than saying nothing.
#[cfg(target_os = "macos")]
pub fn full_disk_access() -> Option<bool> {
    let probe = std::env::var_os("HOME")
        .map(PathBuf::from)?
        .join("Library/Application Support/com.apple.TCC");
    match std::fs::read_dir(&probe) {
        Ok(_) => Some(true),
        Err(e) if e.kind() == std::io::ErrorKind::PermissionDenied => Some(false),
        Err(_) => None,
    }
}

#[cfg(not(target_os = "macos"))]
pub fn full_disk_access() -> Option<bool> {
    None
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VolumeGroup {
    pub volume: String,
    pub denied: usize,
    pub other: usize,
    pub entries: Vec<SkippedPath>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PermissionReport {
    /// Paths the user can act on.
    pub denied: usize,
    /// Everything else that was skipped.
    pub other: usize,
    /// Entries past [`MAX_ENTRIES`] that were counted but not kept.
    pub dropped: u64,
    pub by_volume: Vec<VolumeGroup>,
    pub guidance: Guidance,
    /// macOS only; `None` where the question does not apply or could
    /// not be answered.
    pub full_disk_access: Option<bool>,
}

pub fn report(ledger: &PermissionLedger) -> PermissionReport {
    let mut by_volume: Vec<VolumeGroup> = Vec::new();
    for e in ledger.entries() {
        let group = match by_volume.iter_mut().find(|g| g.volume == e.volume) {
            Some(g) => g,
            None => {
                by_volume.push(VolumeGroup {
                    volume: e.volume.clone(),
                    denied: 0,
                    other: 0,
                    entries: Vec::new(),
                });
                by_volume.last_mut().expect("just pushed")
            }
        };
        if e.reason == SkipReason::PermissionDenied {
            group.denied += 1;
        } else {
            group.other += 1;
        }
        group.entries.push(e.clone());
    }
    // Worst first — the volume with the most unreadable subtrees is the
    // one worth fixing.
    by_volume.sort_by(|a, b| {
        b.denied
            .cmp(&a.denied)
            .then_with(|| a.volume.cmp(&b.volume))
    });
    PermissionReport {
        denied: ledger.denied_count(),
        other: ledger.entries().len() - ledger.denied_count(),
        dropped: ledger.dropped(),
        by_volume,
        guidance: platform_guidance(),
        full_disk_access: full_disk_access(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn denied() -> std::io::Error {
        std::io::Error::new(std::io::ErrorKind::PermissionDenied, "Access is denied")
    }

    #[test]
    fn records_a_denied_directory() {
        let mut l = PermissionLedger::default();
        l.record(Path::new("/vault/private"), &denied(), "/");
        assert_eq!(l.entries().len(), 1);
        assert_eq!(l.denied_count(), 1);
        assert_eq!(l.entries()[0].reason, SkipReason::PermissionDenied);
    }

    #[test]
    fn a_vanished_path_is_not_a_permission_problem() {
        let mut l = PermissionLedger::default();
        let gone = std::io::Error::new(std::io::ErrorKind::NotFound, "no such file");
        l.record(Path::new("/tmp/gone"), &gone, "/");
        assert!(
            l.is_empty(),
            "a race with a deleting process is not a fault"
        );
    }

    #[test]
    fn a_rescan_does_not_double_count_the_same_directory() {
        let mut l = PermissionLedger::default();
        for _ in 0..5 {
            l.record(Path::new("/vault/private"), &denied(), "/");
        }
        assert_eq!(l.entries().len(), 1);
    }

    #[test]
    fn overflow_is_counted_rather_than_silently_dropped() {
        let mut l = PermissionLedger::default();
        for i in 0..(MAX_ENTRIES + 25) {
            l.record(&PathBuf::from(format!("/vault/{i}")), &denied(), "/");
        }
        assert_eq!(l.entries().len(), MAX_ENTRIES);
        assert_eq!(l.dropped(), 25);
    }

    #[test]
    fn clearing_a_root_leaves_other_roots_alone() {
        let mut l = PermissionLedger::default();
        l.record(Path::new("/a/private"), &denied(), "/");
        l.record(Path::new("/b/private"), &denied(), "/");
        l.clear_under(Path::new("/a"));
        assert_eq!(l.entries().len(), 1);
        assert_eq!(l.entries()[0].path, Path::new("/b/private"));
    }

    #[test]
    fn other_io_errors_are_kept_but_not_counted_as_denials() {
        let mut l = PermissionLedger::default();
        let broken = std::io::Error::other("device not ready");
        l.record(Path::new("/mnt/dead"), &broken, "/mnt");
        assert_eq!(l.entries().len(), 1);
        assert_eq!(l.denied_count(), 0, "only denials have a user-facing fix");
    }
}

#[cfg(test)]
mod report_tests {
    use super::*;

    fn denied() -> std::io::Error {
        std::io::Error::new(std::io::ErrorKind::PermissionDenied, "Access is denied")
    }

    #[test]
    fn groups_by_volume_worst_first() {
        let mut l = PermissionLedger::default();
        l.record(Path::new("/a/one"), &denied(), "/a");
        l.record(Path::new("/b/one"), &denied(), "/b");
        l.record(Path::new("/b/two"), &denied(), "/b");
        let r = report(&l);
        assert_eq!(r.denied, 3);
        assert_eq!(r.by_volume.len(), 2);
        assert_eq!(r.by_volume[0].volume, "/b", "the volume worth fixing first");
        assert_eq!(r.by_volume[0].denied, 2);
    }

    #[test]
    fn separates_denials_from_other_failures() {
        let mut l = PermissionLedger::default();
        l.record(Path::new("/a/one"), &denied(), "/a");
        l.record(Path::new("/a/two"), &std::io::Error::other("i/o"), "/a");
        let r = report(&l);
        assert_eq!(r.denied, 1);
        assert_eq!(r.other, 1);
        assert_eq!(r.by_volume[0].denied, 1);
        assert_eq!(r.by_volume[0].other, 1);
    }

    #[test]
    fn an_empty_ledger_reports_nothing_to_fix() {
        let r = report(&PermissionLedger::default());
        assert_eq!(r.denied, 0);
        assert_eq!(r.dropped, 0);
        assert!(r.by_volume.is_empty());
    }
}
