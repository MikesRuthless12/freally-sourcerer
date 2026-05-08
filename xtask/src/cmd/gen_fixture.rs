//! `xtask gen-fixture` — synthesise a deterministic file-record set
//! and apply it through `sourcerer-index` so the Phase-5 bench /
//! smoke can run against a real on-disk index.
//!
//! Build-Guide spec is "synthetic 5M-file dataset". The default size
//! here is 200 000 — the bench runner overrides via `--count` to land
//! at the larger gate during local pre-PR runs without bogging CI.
//!
//! The fixture is fully synthetic — paths, sizes, and timestamps come
//! from a SplitMix64 PRNG seeded by `--seed`. Re-running with the same
//! seed + count produces a byte-identical index.

use std::path::{Path, PathBuf};

use anyhow::Result;
use sourcerer_index::Index;
use sourcerer_journal::JournalEvent;

const ADJECTIVES: &[&str] = &[
    "alpha",
    "beta",
    "gamma",
    "delta",
    "draft",
    "final",
    "interim",
    "legacy",
    "modern",
    "ancient",
    "shiny",
    "dusty",
    "tagged",
    "untitled",
    "primary",
    "secondary",
    "archived",
    "scratch",
    "summary",
    "report",
];
const NOUNS: &[&str] = &[
    "project",
    "notes",
    "minutes",
    "agenda",
    "spec",
    "design",
    "diagram",
    "log",
    "trace",
    "build",
    "release",
    "patch",
    "manifest",
    "blueprint",
    "rooster",
    "horizon",
    "canyon",
    "lighthouse",
    "telescope",
    "compass",
];
const EXTS: &[&str] = &[
    "txt", "md", "rs", "py", "js", "ts", "json", "csv", "yaml", "toml", "pdf", "png", "jpg", "wav",
    "mp3", "flac", "doc", "docx", "xls", "xlsx",
];

const COMMIT_BATCH: usize = 5_000;

pub fn run(out_dir: PathBuf, count: usize, seed: u64) -> Result<()> {
    if out_dir.exists() {
        // The fixture overwrites the index in-place, but warn against
        // accidentally pointing at a real index root.
        if out_dir.join("files.db").exists() {
            eprintln!(
                "WARN: {} already contains a Sourcerer index — overwriting",
                out_dir.display()
            );
        }
    }
    std::fs::create_dir_all(&out_dir)?;
    let idx = Index::open(&out_dir)?;
    let mut rng = SplitMix64::new(seed);
    let started = std::time::Instant::now();
    let mut emitted = 0usize;
    let mut batch: Vec<JournalEvent> = Vec::with_capacity(COMMIT_BATCH);

    while emitted < count {
        let path = synth_path(&mut rng);
        let size = rng.next() % (50 * 1024 * 1024);
        let mtime_ns = synth_mtime_ns(&mut rng);
        batch.push(JournalEvent::Create {
            path,
            size,
            mtime_ns,
            ctime_ns: mtime_ns,
            attrs: 0,
        });
        emitted += 1;
        if batch.len() >= COMMIT_BATCH {
            idx.apply(&batch)?;
            batch.clear();
            idx.commit()?;
            if emitted % 50_000 == 0 {
                println!(
                    "  emitted {emitted} / {count} ({:.1}/s)",
                    emitted as f64 / started.elapsed().as_secs_f64()
                );
            }
        }
    }
    if !batch.is_empty() {
        idx.apply(&batch)?;
        idx.commit()?;
    }
    println!(
        "gen-fixture: {} rows in {:.1}s ({} per second)",
        emitted,
        started.elapsed().as_secs_f64(),
        (emitted as f64 / started.elapsed().as_secs_f64()) as u64
    );
    println!("index root: {}", out_dir.display());
    Ok(())
}

fn synth_path(rng: &mut SplitMix64) -> PathBuf {
    let depth = (rng.next() % 4) + 2;
    let mut p = root_for_target();
    for _ in 0..depth {
        p.push(pick(NOUNS, rng));
    }
    let stem = format!(
        "{}-{}-{:05}",
        pick(ADJECTIVES, rng),
        pick(NOUNS, rng),
        rng.next() % 100_000
    );
    let ext = pick(EXTS, rng);
    p.push(format!("{stem}.{ext}"));
    p
}

fn root_for_target() -> PathBuf {
    if cfg!(windows) {
        PathBuf::from("C:\\synth")
    } else {
        PathBuf::from("/synth")
    }
}

fn synth_mtime_ns(rng: &mut SplitMix64) -> i128 {
    // Anchor at 2024-01-01 + up to 2 years of uniform spread.
    const ANCHOR_NS: i128 = 1_704_067_200 * 1_000_000_000;
    const SPREAD_NS: i128 = 2 * 365 * 86_400 * 1_000_000_000;
    let off = (rng.next() % SPREAD_NS as u64) as i128;
    ANCHOR_NS + off
}

fn pick<'a>(arr: &'a [&'a str], rng: &mut SplitMix64) -> &'a str {
    arr[(rng.next() as usize) % arr.len()]
}

/// SplitMix64 — small, deterministic, good enough for synthetic-fixture
/// generation. We don't pull in `rand` / `getrandom` so the xtask works
/// in offline / sandboxed builds.
struct SplitMix64 {
    state: u64,
}

impl SplitMix64 {
    fn new(seed: u64) -> Self {
        Self {
            state: seed.wrapping_add(0x9E37_79B9_7F4A_7C15),
        }
    }
    fn next(&mut self) -> u64 {
        self.state = self.state.wrapping_add(0x9E37_79B9_7F4A_7C15);
        let mut z = self.state;
        z = (z ^ (z >> 30)).wrapping_mul(0xBF58_476D_1CE4_E5B9);
        z = (z ^ (z >> 27)).wrapping_mul(0x94D0_49BB_1331_11EB);
        z ^ (z >> 31)
    }
}

/// Convenience: where `tests` and benches default the fixture root.
pub fn default_fixture_root() -> PathBuf {
    crate::workspace_root()
        .join("target")
        .join("phase05-fixture")
}

#[allow(dead_code)]
pub fn ensure_default(count: usize, seed: u64) -> Result<PathBuf> {
    let root = default_fixture_root();
    if !root.join("files.db").exists() {
        run(root.clone(), count, seed)?;
    }
    Ok(root)
}

#[allow(dead_code)]
pub fn touch(_p: &Path) {}
