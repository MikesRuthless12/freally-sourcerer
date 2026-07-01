//! Cross-platform baseline walker, used as the ground-truth comparison
//! for the OS-specific fast paths. `walkdir` ultimately calls the same
//! kernel APIs (`readdir` → `getdents64` on Linux, `readdir` →
//! `getdirentries` on macOS) but does NOT batch stats; every entry costs
//! one extra `lstat` syscall. The fast paths cut that down.

use std::path::Path;

use anyhow::{Context, Result};
use walkdir::WalkDir;

use crate::ScanStats;

pub fn scan(root: &Path) -> Result<ScanStats> {
    let mut stats = ScanStats::default();
    for entry in WalkDir::new(root).follow_links(false) {
        let entry = entry.with_context(|| format!("walking under {}", root.display()))?;
        let ft = entry.file_type();
        if ft.is_dir() {
            stats.dirs += 1;
        } else if ft.is_file() {
            stats.files += 1;
            // Avoid the extra `metadata()` syscall when the user only
            // cares about counts; `WalkDir` already pulled the file
            // type from the `getdents64` `d_type` byte. We DO want the
            // size though, since the baseline's role is "ground truth"
            // for the fast path which reports sizes too.
            if let Ok(md) = entry.metadata() {
                stats.bytes += md.len();
            }
        }
    }
    Ok(stats)
}
