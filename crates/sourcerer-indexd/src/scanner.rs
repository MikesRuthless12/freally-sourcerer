//! Filesystem scanner used by `folders.add` / `folders.rescan_all` /
//! `index.rebuild`. Feeds `JournalEvent::Create`s into the existing
//! `Index::apply` + `commit` pipeline.
//!
//! Two paths:
//!   1. **MFT fast path (Windows)** — when the requested root is on an
//!      NTFS volume and we can open the volume handle, use the existing
//!      `sourcerer_journal_win::JournalSubscriber::bootstrap()` (which
//!      drives `FSCTL_ENUM_USN_DATA` over the entire MFT). This is the
//!      same mechanism Everything uses; it walks ~1M files in seconds.
//!      Events are filtered by path-prefix so picking a sub-folder still
//!      works.
//!   2. **`walkdir` fallback** — used on non-Windows, on non-NTFS
//!      volumes, when the user lacks the privileges needed to open
//!      `\\.\C:` (FILE_FLAG_BACKUP_SEMANTICS — usually admin), or when
//!      the MFT path errors out.

use std::path::{Path, PathBuf};
use std::sync::Arc;
use std::time::{Instant, UNIX_EPOCH};

use sourcerer_index::Index;
use sourcerer_journal::JournalEvent;
use tracing::{info, warn};
use walkdir::WalkDir;

/// Batch size — how many `JournalEvent`s to accumulate before
/// flushing into Tantivy's writer. Bumped vs. the legacy
/// per-commit batch so the bootstrap path spends more time in
/// `add_document` and less time in syscalls.
const BATCH_SIZE: usize = 8192;

/// Progress-log cadence: emit a `scan progress` line every N events
/// applied to Tantivy. No commit happens here — the Tantivy writer
/// holds everything in its in-memory heap until `finalize_bootstrap`.
const PROGRESS_EVERY: u64 = 50_000;

/// Walk `root`, indexing every file via the fast bootstrap path
/// (Tantivy-only ingest + one bulk SQLite/name_index rebuild at the
/// end). Returns the number of files indexed.
///
/// Skips directories that fail to read (permission denied, locked,
/// missing) with a warn log rather than aborting the whole scan.
pub fn scan_folder(idx: Arc<Index>, root: PathBuf) -> Result<u64, anyhow::Error> {
    let started = Instant::now();
    info!(root = %root.display(), "scan start");

    let mut total: u64 = 0;
    let mut batch: Vec<JournalEvent> = Vec::with_capacity(BATCH_SIZE);
    let mut since_progress: u64 = 0;

    for entry in WalkDir::new(&root)
        .follow_links(false)
        .same_file_system(false)
        .into_iter()
        .filter_map(|res| match res {
            Ok(e) => Some(e),
            Err(e) => {
                warn!(error = %e, "walk entry skipped");
                None
            }
        })
    {
        // Index both files and directories so "Everything" mode (voidtools
        // parity) can list folders too. Skip symlinks/junctions to avoid
        // walking outside the requested root.
        let ft = entry.file_type();
        if !(ft.is_file() || ft.is_dir()) {
            continue;
        }
        let metadata = match entry.metadata() {
            Ok(m) => m,
            Err(e) => {
                warn!(path = %entry.path().display(), error = %e, "metadata read failed; skipping");
                continue;
            }
        };
        let mtime_ns = metadata
            .modified()
            .ok()
            .and_then(|t| t.duration_since(UNIX_EPOCH).ok())
            .map(|d| d.as_nanos() as i128)
            .unwrap_or(0);
        let ctime_ns = metadata
            .created()
            .ok()
            .and_then(|t| t.duration_since(UNIX_EPOCH).ok())
            .map(|d| d.as_nanos() as i128)
            .unwrap_or(mtime_ns);
        // Mark directories via the `attrs` bit so downstream UI / lens code
        // can distinguish them. Bit 0x10 matches Win32's
        // FILE_ATTRIBUTE_DIRECTORY for cross-platform consistency.
        let attrs: u32 = if ft.is_dir() { 0x10 } else { 0 };
        batch.push(JournalEvent::Create {
            path: entry.path().to_owned(),
            size: if ft.is_dir() { 0 } else { metadata.len() },
            mtime_ns,
            ctime_ns,
            attrs,
        });
        if batch.len() >= BATCH_SIZE {
            if let Err(e) = idx.bootstrap_apply(&batch) {
                warn!(error = %e, "index bootstrap_apply failed for batch; aborting scan");
                return Err(anyhow::anyhow!("index.bootstrap_apply: {e}"));
            }
            total += batch.len() as u64;
            since_progress += batch.len() as u64;
            batch.clear();
            if since_progress >= PROGRESS_EVERY {
                info!(total, "scan progress");
                since_progress = 0;
            }
        }
    }
    if !batch.is_empty() {
        if let Err(e) = idx.bootstrap_apply(&batch) {
            warn!(error = %e, "index bootstrap_apply failed for tail batch");
            return Err(anyhow::anyhow!("index.bootstrap_apply: {e}"));
        }
        total += batch.len() as u64;
    }
    let walk_elapsed = started.elapsed();
    info!(
        root = %root.display(),
        total,
        walk_ms = walk_elapsed.as_millis() as u64,
        "scan: walk done; finalizing"
    );

    // Single big commit + bulk SQLite/name_index rebuild from Tantivy.
    let finalize_start = Instant::now();
    let rebuilt = match idx.finalize_bootstrap() {
        Ok(n) => n,
        Err(e) => {
            warn!(error = %e, "finalize_bootstrap failed");
            return Err(anyhow::anyhow!("index.finalize_bootstrap: {e}"));
        }
    };
    let finalize_elapsed = finalize_start.elapsed();
    let total_elapsed = started.elapsed();
    info!(
        root = %root.display(),
        total,
        rebuilt,
        walk_ms = walk_elapsed.as_millis() as u64,
        finalize_ms = finalize_elapsed.as_millis() as u64,
        total_ms = total_elapsed.as_millis() as u64,
        "scan complete"
    );
    Ok(total)
}

/// Spawn the scan on a blocking thread so the RPC handler returns
/// immediately and the indexd worker pool isn't held by a multi-minute
/// directory walk. Tries the MFT fast path on Windows; falls back to
/// `walkdir` if that errors.
pub fn spawn_scan(idx: Arc<Index>, root: &Path) {
    let root = root.to_path_buf();
    tokio::task::spawn_blocking(move || {
        #[cfg(windows)]
        {
            match mft::try_mft_scan(idx.clone(), &root) {
                Ok(n) => {
                    info!(root = %root.display(), files = n, "MFT scan ok");
                    return;
                }
                Err(e) => {
                    warn!(
                        root = %root.display(),
                        error = %e,
                        "MFT scan unavailable; falling back to walkdir"
                    );
                }
            }
        }
        if let Err(e) = scan_folder(idx, root.clone()) {
            warn!(root = %root.display(), error = %e, "scan failed");
        }
    });
}

#[cfg(windows)]
mod mft {
    use super::*;
    use futures::StreamExt;
    use futures::executor::block_on;

    /// Try the FSCTL_ENUM_USN_DATA fast path. Returns `Err` if the
    /// requested root isn't a Windows drive letter, the volume can't be
    /// opened, or any other MFT-specific failure — caller falls back to
    /// `walkdir`.
    pub fn try_mft_scan(idx: Arc<Index>, root: &Path) -> Result<u64, anyhow::Error> {
        let volume_root = volume_root_of(root)
            .ok_or_else(|| anyhow::anyhow!("path is not on a drive-letter volume"))?;
        info!(
            volume = %volume_root.display(),
            root = %root.display(),
            "MFT scan: opening journal"
        );
        let subscriber = sourcerer_journal_win::open(&volume_root)
            .map_err(|e| anyhow::anyhow!("open journal: {e}"))?;
        let stream = subscriber.bootstrap();
        let mut stream = Box::pin(stream);

        // Lowercase prefix for case-insensitive filtering — NTFS is case-
        // preserving but case-insensitive at the API level.
        let prefix = root.to_string_lossy().to_lowercase();
        let prefix_with_sep = if prefix.ends_with('\\') || prefix.ends_with('/') {
            prefix.clone()
        } else {
            format!("{prefix}\\")
        };

        let started = Instant::now();
        let mut total: u64 = 0;
        let mut seen: u64 = 0;
        let mut batch: Vec<JournalEvent> = Vec::with_capacity(BATCH_SIZE);
        let mut since_progress: u64 = 0;

        while let Some(event) = block_on(stream.next()) {
            seen += 1;
            let keep = match &event {
                JournalEvent::Create { path, .. } => {
                    let p = path.to_string_lossy().to_lowercase();
                    // Either an exact match for the root itself (rare —
                    // bootstrap skips dirs) or a descendant under it.
                    p == prefix || p.starts_with(&prefix_with_sep)
                }
                _ => false,
            };
            if !keep {
                continue;
            }
            batch.push(event);
            if batch.len() >= BATCH_SIZE {
                idx.bootstrap_apply(&batch)?;
                total += batch.len() as u64;
                since_progress += batch.len() as u64;
                batch.clear();
                if since_progress >= PROGRESS_EVERY {
                    info!(total, seen, "MFT scan progress");
                    since_progress = 0;
                }
            }
        }
        if !batch.is_empty() {
            idx.bootstrap_apply(&batch)?;
            total += batch.len() as u64;
        }
        let walk_elapsed = started.elapsed();
        info!(
            volume = %volume_root.display(),
            total,
            seen,
            walk_ms = walk_elapsed.as_millis() as u64,
            "MFT scan: walk done; finalizing"
        );

        let finalize_start = Instant::now();
        let rebuilt = idx.finalize_bootstrap()?;
        let finalize_elapsed = finalize_start.elapsed();
        let total_elapsed = started.elapsed();
        info!(
            volume = %volume_root.display(),
            total,
            rebuilt,
            seen,
            walk_ms = walk_elapsed.as_millis() as u64,
            finalize_ms = finalize_elapsed.as_millis() as u64,
            total_ms = total_elapsed.as_millis() as u64,
            "MFT scan complete"
        );
        Ok(total)
    }

    /// Returns the volume root (e.g. `C:\`) for a path like `C:\Users\…`.
    /// Returns `None` for UNC paths, paths without a drive letter, etc.
    fn volume_root_of(p: &Path) -> Option<PathBuf> {
        let s = p.to_string_lossy();
        let bytes = s.as_bytes();
        if bytes.len() >= 2 && bytes[1] == b':' && bytes[0].is_ascii_alphabetic() {
            let mut out = String::with_capacity(3);
            out.push(bytes[0] as char);
            out.push_str(":\\");
            Some(PathBuf::from(out))
        } else {
            None
        }
    }
}
