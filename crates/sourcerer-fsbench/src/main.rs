//! `sourcerer-fsbench` — Phase 13 fast-bootstrap verifier.
//!
//! Two subcommands:
//!
//! - `gen --root <P> --count N [--depth D] [--fanout F]` — drops `N`
//!   regular files into a synthetic tree rooted at `<P>`. Use it to
//!   produce a reproducible 100k / 500k / 1M target before scanning.
//! - `scan --root <P> [--baseline]` — walks the tree once via the fast
//!   OS-specific primitive (`getdents64 + statx` on Linux, `getattrlistbulk`
//!   on macOS) and reports total file count, dir count, byte total, and
//!   wall-clock elapsed. With `--baseline`, falls back to the
//!   cross-platform `walkdir` crate so you can verify the fast path
//!   returns the same totals.
//!
//! The fast paths live in their own `cfg(target_os = ...)`-gated modules
//! so the workspace compiles cleanly on every host even when the
//! platform-specific syscall isn't available.

use std::path::PathBuf;
use std::time::Instant;

use anyhow::{Context, Result};
use clap::{Parser, Subcommand};

mod baseline;
#[cfg(target_os = "linux")]
mod fast_linux;
#[cfg(target_os = "macos")]
mod fast_macos;
mod tree_gen;

#[derive(Parser, Debug)]
#[command(
    name = "sourcerer-fsbench",
    about = "Generate synthetic trees and verify Sourcerer's fast-bootstrap walkers.",
    version
)]
struct Cli {
    #[command(subcommand)]
    cmd: Cmd,
}

#[derive(Subcommand, Debug)]
enum Cmd {
    /// Drop `count` synthetic regular files into a directory tree at
    /// `root`. The tree is balanced across `fanout` subdirectories per
    /// level, up to `depth` levels deep.
    Gen {
        /// Root directory for the generated tree. Must not already
        /// contain the marker file `.fsbench-tree`.
        #[arg(long)]
        root: PathBuf,
        /// Total number of regular files to create.
        #[arg(long)]
        count: usize,
        /// Maximum nesting depth (default 4 — enough that 1M files fit
        /// without any one directory holding more than ~50k entries).
        #[arg(long, default_value_t = 4)]
        depth: usize,
        /// Subdirectories per intermediate level (default 16).
        #[arg(long, default_value_t = 16)]
        fanout: usize,
        /// Bytes per generated file (default 0 — empty files; you're
        /// verifying enumeration, not I/O).
        #[arg(long, default_value_t = 0)]
        size: u64,
    },

    /// Walk `root` once and report file count, dir count, byte total,
    /// and elapsed time.
    Scan {
        /// Root directory to walk.
        #[arg(long)]
        root: PathBuf,
        /// Use the cross-platform `walkdir` crate instead of the
        /// OS-specific fast path. Pair with a second invocation (without
        /// `--baseline`) to confirm both walkers report identical totals.
        #[arg(long, default_value_t = false)]
        baseline: bool,
    },
}

#[derive(Debug, Default, Clone, Copy)]
pub struct ScanStats {
    pub files: u64,
    pub dirs: u64,
    pub bytes: u64,
}

fn main() -> Result<()> {
    let cli = Cli::parse();
    match cli.cmd {
        Cmd::Gen {
            root,
            count,
            depth,
            fanout,
            size,
        } => {
            let start = Instant::now();
            let stats = tree_gen::generate(&root, count, depth, fanout, size)
                .with_context(|| format!("generating {count} files under {}", root.display()))?;
            let elapsed = start.elapsed();
            println!(
                "gen: {} file(s) across {} dir(s) ({} bytes) in {:.2}s ({:.0} files/s)",
                stats.files,
                stats.dirs,
                stats.bytes,
                elapsed.as_secs_f64(),
                stats.files as f64 / elapsed.as_secs_f64().max(1e-9),
            );
        }
        Cmd::Scan { root, baseline } => {
            let start = Instant::now();
            let (stats, label) = if baseline {
                (baseline::scan(&root)?, "baseline (walkdir)")
            } else {
                (scan_fast(&root)?, fast_label())
            };
            let elapsed = start.elapsed();
            println!(
                "scan [{label}]: {} file(s), {} dir(s), {} bytes, {:.3}s ({:.0} files/s)",
                stats.files,
                stats.dirs,
                stats.bytes,
                elapsed.as_secs_f64(),
                (stats.files + stats.dirs) as f64 / elapsed.as_secs_f64().max(1e-9),
            );
        }
    }
    Ok(())
}

#[cfg(target_os = "linux")]
fn scan_fast(root: &std::path::Path) -> Result<ScanStats> {
    fast_linux::scan(root)
}

#[cfg(target_os = "macos")]
fn scan_fast(root: &std::path::Path) -> Result<ScanStats> {
    fast_macos::scan(root)
}

#[cfg(not(any(target_os = "linux", target_os = "macos")))]
fn scan_fast(root: &std::path::Path) -> Result<ScanStats> {
    // No platform-specific fast path on this host; fall back to walkdir
    // so the binary stays usable for verification (the user just won't
    // get the syscall-batched speed-up).
    eprintln!(
        "warning: no fast path on this OS — falling back to walkdir. \
         Run on Linux (getdents64 + statx) or macOS (getattrlistbulk) for the real thing."
    );
    baseline::scan(root)
}

#[cfg(target_os = "linux")]
fn fast_label() -> &'static str {
    "linux/getdents64+statx (parallel, AT_STATX_DONT_SYNC)"
}

#[cfg(target_os = "macos")]
fn fast_label() -> &'static str {
    "macos/getattrlistbulk"
}

#[cfg(not(any(target_os = "linux", target_os = "macos")))]
fn fast_label() -> &'static str {
    "fallback/walkdir"
}
