//! Phase-5 perf gate: P50 ≤ 8 ms, P99 ≤ 16 ms on a 5 M-file dataset.
//!
//! Custom timing harness (no `criterion` dep — keeps the dep tree
//! lean for `cargo deny`). Uses the same SplitMix64 seed flow as the
//! `xtask gen-fixture` command so the bench's synthetic index is
//! byte-identical to what users build with `cargo run -p xtask --
//! gen-fixture`.
//!
//! Usage:
//!   cargo bench -p freally-query --bench filename_lens
//!   FREALLY_BENCH_COUNT=5000000 cargo bench ...   # the gate run
//!   FREALLY_BENCH_COUNT=20000   cargo bench ...   # quick local
//!
//! The default count is conservative (50 000) so a developer's laptop
//! finishes in seconds. The gate is enforced by the Phase-5 PR
//! reviewer — they bump `FREALLY_BENCH_COUNT` to 5 000 000 and
//! confirm the printed P50 / P99 land under 8 ms / 16 ms before the
//! merge.
//!
//! # The ladder, and why it exists
//!
//! A single 5 M run was attempted on 2026-08-22 and killed at 85 minutes
//! **without finishing its fixture**. It left nothing behind but a 1.7 GB
//! temp directory: no numbers, no partial answer, and no way to resume.
//!
//! The cost is not in the batch size — rows already go in 5 000 at a time
//! — it is that each additional row costs more than the last, as Tantivy
//! merges segments and the table outgrows SQLite's page cache. Running
//! the bench twenty-five times at 200 000 does **not** add up to 5 M
//! either: every run builds its own fixture and throws it away.
//!
//! So two knobs:
//!
//!   FREALLY_BENCH_LADDER=200000,500000,1000000,2000000,5000000
//!       Grow the fixture to each size in turn and benchmark at every
//!       stop, printing the build time for each leg. The scaling *curve*
//!       is the open question, and this answers it incrementally — a run
//!       that dies at 2 M still tells you most of what you wanted.
//!
//!   FREALLY_BENCH_DIR=D:/freally-bench-5m
//!       Keep the fixture instead of using a temp dir, and record how far
//!       it got in `bench-progress.json` beside it. A later run picks up
//!       where this one stopped. Without it the fixture is a temp dir, as
//!       before.
//!
//! Neither changes the default: with no `LADDER` and no `DIR`, this is
//! exactly the single-size, temp-dir run whose numbers the roadmap
//! records.

use std::path::PathBuf;
use std::sync::Arc;
use std::time::{Duration, Instant};

use freally_index::Index;
use freally_journal::JournalEvent;
use freally_query::{ExecOpts, execute, parse};

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

const SCENARIOS: &[(&str, &str)] = &[
    ("literal-hot", "report"),
    ("literal-rare", "lighthouse"),
    ("wildcard", "*.rs"),
    ("modifier", "ext:json"),
    ("compound", "report ext:pdf size:>1mb"),
    ("regex-anchor", r"regex:^report-"),
];

fn main() {
    let count = std::env::var("FREALLY_BENCH_COUNT")
        .ok()
        .and_then(|s| s.parse::<usize>().ok())
        .unwrap_or(50_000);
    let seed: u64 = std::env::var("FREALLY_BENCH_SEED")
        .ok()
        .and_then(|s| s.parse::<u64>().ok())
        .unwrap_or(0xC0FFEE);
    // Each rung must be larger than the one before it: the fixture only
    // grows, so a smaller target would silently measure the previous size.
    let ladder = match std::env::var("FREALLY_BENCH_LADDER") {
        Ok(spec) => {
            let rungs: Vec<usize> = spec
                .split(',')
                .filter_map(|s| s.trim().parse::<usize>().ok())
                .collect();
            assert!(!rungs.is_empty(), "FREALLY_BENCH_LADDER parsed to nothing");
            assert!(
                rungs.windows(2).all(|w| w[0] < w[1]),
                "FREALLY_BENCH_LADDER must ascend; the fixture only grows"
            );
            rungs
        }
        Err(_) => vec![count],
    };
    let persist = std::env::var("FREALLY_BENCH_DIR").ok().map(PathBuf::from);

    println!("Phase-5 filename-lens bench");
    if ladder.len() > 1 {
        let rungs: Vec<String> = ladder.iter().map(|n| n.to_string()).collect();
        println!("  ladder:       {}", rungs.join(" -> "));
    } else {
        println!("  fixture rows: {}", ladder[0]);
    }
    println!("  seed:         {seed}");
    match &persist {
        Some(p) => println!("  fixture:      {} (kept, resumable)", p.display()),
        None => println!("  fixture:      temporary"),
    }
    println!();

    // Held for the lifetime of the run so a temp fixture is not deleted
    // out from under the index.
    let tmp;
    let root: PathBuf = match &persist {
        Some(p) => {
            std::fs::create_dir_all(p).expect("create the fixture directory");
            p.clone()
        }
        None => {
            tmp = tempfile::tempdir().unwrap();
            tmp.path().to_path_buf()
        }
    };

    let idx = Index::open(&root).expect("Index::open for bench");
    let mut built = Progress::load(&root, seed);
    let mut rng = SplitMix64::new(seed);
    if built > 0 {
        // Replay the generator without indexing anything. The number of
        // `next()` calls per row varies with the synthesized path depth,
        // so the stream cannot be seeked — only re-run. It is pure string
        // work and costs seconds even at millions of rows.
        let t0 = Instant::now();
        for _ in 0..built {
            let _ = synth_row(&mut rng);
        }
        println!(
            "  resuming at {built} rows (generator replayed in {:.1?})",
            t0.elapsed()
        );
    }

    let mut all_passed = true;

    for (rung, &target) in ladder.iter().enumerate() {
        if target > built {
            let before = built;
            let t0 = Instant::now();
            grow_fixture(&idx, &mut rng, &mut built, target, &root, seed);
            println!(
                "  built {target} rows (+{} in {:.1?})",
                target - before,
                t0.elapsed()
            );
        } else {
            println!("  {target} rows already present");
        }

        // Warm the trigram index at this size.
        let _ = execute(&idx, &parse("alpha").unwrap(), ExecOpts::default()).unwrap();

        if ladder.len() > 1 {
            println!("  --- {target} rows ---");
        }
        if !run_scenarios(&idx, ladder.len() > 1) {
            all_passed = false;
        }
        if rung + 1 < ladder.len() {
            println!();
        }
    }

    println!();
    if !all_passed {
        // Build-Guide §"Bench gate": CI fails the phase PR if the
        // bench regresses. Honoring it: a non-zero exit code surfaces
        // through cargo bench → CI.
        eprintln!(
            "Phase-5 perf gate: at least one scenario regressed past P50 ≤ 8 ms / P99 ≤ 16 ms."
        );
        if std::env::var("FREALLY_BENCH_GATE").as_deref() == Ok("1") {
            std::process::exit(2);
        }
    } else {
        println!("Phase-5 perf gate: all scenarios within budget.");
    }
}

/// Run every scenario against the index as it stands. Returns whether
/// all of them met the gate.
fn run_scenarios(idx: &Arc<Index>, in_ladder: bool) -> bool {
    let p50_target = Duration::from_millis(8);
    let p99_target = Duration::from_millis(16);
    let mut all_passed = true;
    for (label, q) in SCENARIOS {
        let summary = bench_query(idx, q, 200);
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
        let indent = if in_ladder { "    " } else { "  " };
        println!(
            "{indent}{label:<14} P50 {:>8.2?} [{p50_str}]  P99 {:>8.2?} [{p99_str}]  hits {:>6}  cand {:>8}  surv {:>7}  seed {}",
            summary.p50,
            summary.p99,
            summary.hits,
            summary.candidates,
            summary.survivors,
            if summary.used_seed { "yes" } else { "NO" }
        );
    }
    all_passed
}

/// How many rows the fixture in `dir` already holds, and under which
/// seed.
///
/// Recorded rather than counted from the index, because `synth_path` can
/// collide and an upsert then leaves fewer rows than were emitted —
/// resuming from the row *count* would replay part of the stream and
/// produce a different fixture. The emitted count is the only number that
/// reproduces the generator exactly.
struct Progress;

impl Progress {
    fn path(dir: &std::path::Path) -> PathBuf {
        dir.join("bench-progress.json")
    }

    fn load(dir: &std::path::Path, seed: u64) -> usize {
        let Ok(raw) = std::fs::read_to_string(Self::path(dir)) else {
            return 0;
        };
        let v: serde_json::Value = match serde_json::from_str(&raw) {
            Ok(v) => v,
            Err(_) => return 0,
        };
        let recorded_seed = v.get("seed").and_then(|s| s.as_u64()).unwrap_or(0);
        // Two seeds in one directory would interleave two different
        // corpora into one index, and every number after that would be
        // measuring a fixture nobody can reproduce.
        assert_eq!(
            recorded_seed,
            seed,
            "fixture at {} was built with seed {recorded_seed}; refusing to extend it with seed {seed}",
            dir.display()
        );
        v.get("emitted").and_then(|e| e.as_u64()).unwrap_or(0) as usize
    }

    fn save(dir: &std::path::Path, seed: u64, emitted: usize) {
        let body = serde_json::json!({ "seed": seed, "emitted": emitted });
        let _ = std::fs::write(Self::path(dir), body.to_string());
    }
}

struct Summary {
    p50: Duration,
    p99: Duration,
    hits: usize,
    /// Rows the name index handed the filter. A FAIL is unactionable
    /// without this: it separates "the seed did not narrow anything" from
    /// "the seed narrowed fine and the work after it is slow".
    candidates: usize,
    survivors: usize,
    used_seed: bool,
}

fn bench_query(idx: &Arc<Index>, q: &str, samples: usize) -> Summary {
    let parsed = parse(q).expect("bench query parses");
    let mut hits = 0usize;
    let mut candidates = 0usize;
    let mut survivors = 0usize;
    let mut used_seed = false;
    let mut samples_v: Vec<Duration> = Vec::with_capacity(samples);
    for _ in 0..samples {
        let opts = ExecOpts::default();
        let t0 = Instant::now();
        let rs = execute(idx, &parsed, opts).expect("bench execute");
        let elapsed = t0.elapsed();
        hits = rs.rows().len();
        candidates = rs.stats.candidates;
        survivors = rs.stats.name_survivors;
        used_seed = rs.stats.used_seed;
        samples_v.push(elapsed);
    }
    samples_v.sort();
    let p50 = samples_v[samples / 2];
    let p99 = samples_v[(samples * 99 / 100).min(samples_v.len() - 1)];
    Summary {
        p50,
        p99,
        hits,
        candidates,
        survivors,
        used_seed,
    }
}

/// One synthetic row, drawn from `rng` in a fixed order.
///
/// Split out so the resume path can replay the generator without
/// indexing: `synth_path` consumes a variable number of `next()` calls
/// (the path depth is itself drawn from the stream), so the sequence
/// cannot be seeked, only re-run.
fn synth_row(rng: &mut SplitMix64) -> JournalEvent {
    let path = synth_path(rng);
    let size = rng.next() % (50 * 1024 * 1024);
    let mtime_ns = synth_mtime_ns(rng);
    JournalEvent::Create {
        path,
        size,
        mtime_ns,
        ctime_ns: mtime_ns,
        attrs: 0,
    }
}

/// Extend the fixture from `*built` rows up to `target`.
///
/// Progress is recorded after every commit, so a killed run loses at most
/// one `COMMIT_BATCH` rather than the whole fixture — which is what the
/// 85-minute 5 M attempt lost.
fn grow_fixture(
    idx: &Arc<Index>,
    rng: &mut SplitMix64,
    built: &mut usize,
    target: usize,
    dir: &std::path::Path,
    seed: u64,
) {
    let mut batch: Vec<JournalEvent> = Vec::with_capacity(COMMIT_BATCH);
    while *built < target {
        batch.push(synth_row(rng));
        *built += 1;
        if batch.len() >= COMMIT_BATCH {
            idx.apply(&batch).expect("apply batch");
            batch.clear();
            idx.commit().expect("commit");
            Progress::save(dir, seed, *built);
        }
    }
    if !batch.is_empty() {
        idx.apply(&batch).expect("apply tail");
        idx.commit().expect("commit tail");
        Progress::save(dir, seed, *built);
    }
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
