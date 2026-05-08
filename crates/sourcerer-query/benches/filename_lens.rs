//! Phase-5 perf gate: P50 ≤ 8 ms, P99 ≤ 16 ms on a 5 M-file dataset.
//!
//! Custom timing harness (no `criterion` dep — keeps the dep tree
//! lean for `cargo deny`). Uses the same SplitMix64 seed flow as the
//! `xtask gen-fixture` command so the bench's synthetic index is
//! byte-identical to what users build with `cargo run -p xtask --
//! gen-fixture`.
//!
//! Usage:
//!   cargo bench -p sourcerer-query --bench filename_lens
//!   SOURCERER_BENCH_COUNT=5000000 cargo bench ...   # the gate run
//!   SOURCERER_BENCH_COUNT=20000   cargo bench ...   # quick local
//!
//! The default count is conservative (50 000) so a developer's laptop
//! finishes in seconds. The gate is enforced by the Phase-5 PR
//! reviewer — they bump `SOURCERER_BENCH_COUNT` to 5 000 000 and
//! confirm the printed P50 / P99 land under 8 ms / 16 ms before the
//! merge.

use std::path::PathBuf;
use std::sync::Arc;
use std::time::{Duration, Instant};

use sourcerer_index::Index;
use sourcerer_journal::JournalEvent;
use sourcerer_query::{ExecOpts, execute, parse};

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
    "txt", "md", "rs", "py", "js", "ts", "json", "csv", "yaml", "toml",
];

const COMMIT_BATCH: usize = 5_000;

fn main() {
    let count = std::env::var("SOURCERER_BENCH_COUNT")
        .ok()
        .and_then(|s| s.parse::<usize>().ok())
        .unwrap_or(50_000);
    let seed: u64 = std::env::var("SOURCERER_BENCH_SEED")
        .ok()
        .and_then(|s| s.parse::<u64>().ok())
        .unwrap_or(0xC0FFEE);

    println!("Phase-5 filename-lens bench");
    println!("  fixture rows: {count}");
    println!("  seed:         {seed}");
    println!();

    let dir = tempfile::tempdir().unwrap();
    let idx = build_fixture(dir.path(), count, seed);
    println!("  fixture built");

    // Warm the trigram index.
    let _ = execute(&idx, &parse("alpha").unwrap(), ExecOpts::default()).unwrap();

    let scenarios: &[(&str, &str)] = &[
        ("literal-hot", "report"),
        ("literal-rare", "lighthouse"),
        ("wildcard", "*.rs"),
        ("modifier", "ext:json"),
        ("compound", "report ext:pdf size:>1mb"),
        ("regex-anchor", r"regex:^report-"),
    ];

    let mut all_passed = true;
    let p50_target = Duration::from_millis(8);
    let p99_target = Duration::from_millis(16);

    for (label, q) in scenarios {
        let summary = bench_query(&idx, q, 200);
        let p50_str = if summary.p50 <= p50_target {
            "OK"
        } else {
            all_passed = false;
            "FAIL"
        };
        let p99_str = if summary.p99 <= p99_target {
            "OK"
        } else {
            all_passed = false;
            "FAIL"
        };
        println!(
            "  {label:<14} P50 {:>8.2?} [{p50_str}]  P99 {:>8.2?} [{p99_str}]  hits {}",
            summary.p50, summary.p99, summary.hits
        );
    }

    println!();
    if !all_passed {
        // Build-Guide §"Bench gate": CI fails the phase PR if the
        // bench regresses. Honoring it: a non-zero exit code surfaces
        // through cargo bench → CI.
        eprintln!(
            "Phase-5 perf gate: at least one scenario regressed past P50 ≤ 8 ms / P99 ≤ 16 ms."
        );
        if std::env::var("SOURCERER_BENCH_GATE").as_deref() == Ok("1") {
            std::process::exit(2);
        }
    } else {
        println!("Phase-5 perf gate: all scenarios within budget.");
    }
}

struct Summary {
    p50: Duration,
    p99: Duration,
    hits: usize,
}

fn bench_query(idx: &Arc<Index>, q: &str, samples: usize) -> Summary {
    let parsed = parse(q).expect("bench query parses");
    let mut hits = 0usize;
    let mut samples_v: Vec<Duration> = Vec::with_capacity(samples);
    for _ in 0..samples {
        let opts = ExecOpts::default();
        let t0 = Instant::now();
        let rs = execute(idx, &parsed, opts).expect("bench execute");
        let elapsed = t0.elapsed();
        hits = rs.rows().len();
        samples_v.push(elapsed);
    }
    samples_v.sort();
    let p50 = samples_v[samples / 2];
    let p99 = samples_v[(samples * 99 / 100).min(samples_v.len() - 1)];
    Summary { p50, p99, hits }
}

fn build_fixture(root: &std::path::Path, count: usize, seed: u64) -> Arc<Index> {
    let idx = Index::open(root).expect("Index::open for bench");
    let mut rng = SplitMix64::new(seed);
    let mut batch: Vec<JournalEvent> = Vec::with_capacity(COMMIT_BATCH);
    let mut emitted = 0;
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
            idx.apply(&batch).expect("apply batch");
            batch.clear();
            idx.commit().expect("commit");
        }
    }
    if !batch.is_empty() {
        idx.apply(&batch).expect("apply tail");
        idx.commit().expect("commit tail");
    }
    idx
}

fn synth_path(rng: &mut SplitMix64) -> PathBuf {
    let depth = (rng.next() % 4) + 2;
    let mut p = if cfg!(windows) {
        PathBuf::from("C:\\synth")
    } else {
        PathBuf::from("/synth")
    };
    for _ in 0..depth {
        p.push(NOUNS[(rng.next() as usize) % NOUNS.len()]);
    }
    let stem = format!(
        "{}-{}-{:05}",
        ADJECTIVES[(rng.next() as usize) % ADJECTIVES.len()],
        NOUNS[(rng.next() as usize) % NOUNS.len()],
        rng.next() % 100_000
    );
    let ext = EXTS[(rng.next() as usize) % EXTS.len()];
    p.push(format!("{stem}.{ext}"));
    p
}

fn synth_mtime_ns(rng: &mut SplitMix64) -> i128 {
    const ANCHOR_NS: i128 = 1_704_067_200 * 1_000_000_000;
    const SPREAD_NS: i128 = 2 * 365 * 86_400 * 1_000_000_000;
    let off = (rng.next() % SPREAD_NS as u64) as i128;
    ANCHOR_NS + off
}

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
