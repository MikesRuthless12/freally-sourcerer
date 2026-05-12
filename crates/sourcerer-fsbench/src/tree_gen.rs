//! Synthetic tree generator. Creates `count` regular files under `root`
//! balanced across `fanout` subdirectories per level, up to `depth`
//! levels deep. A balanced fanout keeps any one directory under ~50k
//! entries even for the 1M target.

use std::fs::{self, File};
use std::io::Write;
use std::path::{Path, PathBuf};

use anyhow::{Context, Result, bail};

use crate::ScanStats;

/// Marker file written to `root/.fsbench-tree` so a subsequent `gen`
/// against the same path refuses to clobber an unrelated directory.
const MARKER: &str = ".fsbench-tree";

pub fn generate(
    root: &Path,
    count: usize,
    depth: usize,
    fanout: usize,
    size: u64,
) -> Result<ScanStats> {
    if depth == 0 {
        bail!("depth must be ≥ 1");
    }
    if fanout == 0 {
        bail!("fanout must be ≥ 1");
    }

    // Refuse to write into a non-empty directory unless it's already an
    // fsbench tree — protects against accidentally pointing at $HOME.
    if root.exists() {
        let marker = root.join(MARKER);
        let has_marker = marker.exists();
        let is_empty = fs::read_dir(root)
            .with_context(|| format!("reading {}", root.display()))?
            .next()
            .is_none();
        if !is_empty && !has_marker {
            bail!(
                "{} is not empty and is not an existing fsbench tree (no `.fsbench-tree` \
                 marker). Refusing to clobber.",
                root.display()
            );
        }
        if has_marker {
            // Clean the previous tree before regenerating.
            for entry in fs::read_dir(root)? {
                let entry = entry?;
                let p = entry.path();
                if p.is_dir() {
                    fs::remove_dir_all(&p)?;
                } else {
                    fs::remove_file(&p)?;
                }
            }
        }
    } else {
        fs::create_dir_all(root).with_context(|| format!("creating root {}", root.display()))?;
    }
    fs::write(root.join(MARKER), b"sourcerer-fsbench\n")
        .with_context(|| format!("writing marker in {}", root.display()))?;

    // Pre-compute the directory layout. We want `count` total files,
    // balanced across `fanout ^ (depth-1)` leaf directories plus the
    // intermediate levels' own files. Simplest: put every file at the
    // deepest level, with leaf dirs sharing remainders.
    let leaves = leaf_count(depth, fanout);
    let mut leaf_paths: Vec<PathBuf> = Vec::with_capacity(leaves);
    build_layout(
        root,
        0,
        depth,
        fanout,
        &mut PathBuf::from(""),
        &mut leaf_paths,
    )?;

    let per_leaf = count / leaves.max(1);
    let extra = count % leaves.max(1);

    let mut stats = ScanStats {
        files: 1, // the marker
        dirs: count_dirs(depth, fanout, leaves) as u64,
        bytes: 0,
    };

    let payload = if size > 0 {
        vec![0u8; size as usize]
    } else {
        Vec::new()
    };

    for (i, dir) in leaf_paths.iter().enumerate() {
        let n_here = per_leaf + if i < extra { 1 } else { 0 };
        for j in 0..n_here {
            let f = dir.join(format!("f-{j:06}.dat"));
            let mut h = File::create(&f).with_context(|| format!("creating {}", f.display()))?;
            if size > 0 {
                h.write_all(&payload)?;
                stats.bytes += size;
            }
            stats.files += 1;
        }
    }
    Ok(stats)
}

fn leaf_count(depth: usize, fanout: usize) -> usize {
    fanout.saturating_pow(depth.saturating_sub(1) as u32).max(1)
}

fn count_dirs(depth: usize, fanout: usize, leaves: usize) -> usize {
    // Geometric series 1 + f + f^2 + … + f^(d-1) = leaves * f / (f-1) for f>1
    if fanout == 1 {
        return depth;
    }
    // 1 (root) + intermediate dirs. We just sum the level sizes.
    let mut total = 0usize;
    let mut at_level = 1usize;
    for _ in 0..depth {
        total += at_level;
        at_level = at_level.saturating_mul(fanout);
    }
    let _ = leaves;
    total
}

/// Recursive build: create `dir/d-XX` subdirs `fanout` wide down to
/// `depth` levels. The deepest level is where files land.
fn build_layout(
    root: &Path,
    level: usize,
    depth: usize,
    fanout: usize,
    rel: &mut PathBuf,
    out: &mut Vec<PathBuf>,
) -> Result<()> {
    if level == depth.saturating_sub(1) {
        let leaf = root.join(&rel);
        fs::create_dir_all(&leaf).with_context(|| format!("creating leaf {}", leaf.display()))?;
        out.push(leaf);
        return Ok(());
    }
    for i in 0..fanout {
        rel.push(format!("d-{i:03}"));
        build_layout(root, level + 1, depth, fanout, rel, out)?;
        rel.pop();
    }
    Ok(())
}
