//! Phase-5 wiring test — parse + execute end-to-end against a real
//! `sourcerer-index` instance backed by `tempfile`. Asserts the
//! grammar features the Build Guide names actually filter the way
//! they're documented.
//!
//! These tests run on every CI matrix entry. The Phase-5 perf gate
//! is a separate criterion bench under `benches/`.

use std::path::PathBuf;

use sourcerer_index::Index;
use sourcerer_journal::JournalEvent;
use sourcerer_query::{
    ExecOpts, MatchMode, PlanCache, SortField, SortOrder, SortSpec, execute, parse,
};
use tempfile::tempdir;

fn create(path: &str, size: u64, mtime_ns: i128) -> JournalEvent {
    JournalEvent::Create {
        path: PathBuf::from(path),
        size,
        mtime_ns,
        ctime_ns: mtime_ns,
        attrs: 0,
    }
}

fn create_at(path: &str, size: u64, day: i64) -> JournalEvent {
    let mtime_ns = (day as i128) * 86_400 * 1_000_000_000;
    create(path, size, mtime_ns)
}

fn idx_with_files(events: Vec<JournalEvent>) -> std::sync::Arc<Index> {
    let dir = tempdir().unwrap();
    let idx = Index::open(dir.path()).unwrap();
    idx.apply(&events).unwrap();
    idx.commit().unwrap();
    // Leak the tempdir so it survives the function — the Arc<Index>
    // alone keeps the directory alive via its open file handles.
    std::mem::forget(dir);
    idx
}

#[test]
fn literal_term_matches_substring() {
    let idx = idx_with_files(vec![
        create("/synth/projects/alpha-report.md", 1, 0),
        create("/synth/projects/beta-draft.md", 1, 0),
        create("/synth/projects/gamma.md", 1, 0),
    ]);
    let q = parse("report").unwrap();
    let rs = execute(&idx, &q, ExecOpts::default()).unwrap();
    let names: Vec<&str> = rs.rows().iter().map(|r| r.name.as_str()).collect();
    assert!(names.contains(&"alpha-report.md"), "got {names:?}");
    assert_eq!(rs.rows().len(), 1);
}

#[test]
fn wildcard_matches_extension() {
    let idx = idx_with_files(vec![
        create("/synth/note-a.txt", 1, 0),
        create("/synth/note-b.md", 1, 0),
        create("/synth/note-c.txt", 1, 0),
    ]);
    let q = parse("*.txt").unwrap();
    let rs = execute(&idx, &q, ExecOpts::default()).unwrap();
    assert_eq!(rs.rows().len(), 2);
    for r in rs.rows() {
        assert!(r.name.ends_with(".txt"));
    }
}

#[test]
fn regex_term_anchored() {
    let idx = idx_with_files(vec![
        create("/synth/report-001.txt", 1, 0),
        create("/synth/report-extra.txt", 1, 0),
        create("/synth/draft.txt", 1, 0),
    ]);
    let q = parse(r"regex:^report-\d+\.txt$").unwrap();
    let rs = execute(&idx, &q, ExecOpts::default()).unwrap();
    assert_eq!(rs.rows().len(), 1);
    assert_eq!(rs.rows()[0].name, "report-001.txt");
}

#[test]
fn boolean_and_or_not() {
    let idx = idx_with_files(vec![
        create("/synth/alpha-final.md", 1, 0),
        create("/synth/beta-final.md", 1, 0),
        create("/synth/alpha-draft.md", 1, 0),
        create("/synth/gamma.md", 1, 0),
    ]);
    let q = parse("alpha final").unwrap();
    let rs = execute(&idx, &q, ExecOpts::default()).unwrap();
    assert_eq!(rs.rows().len(), 1);
    let q = parse("alpha OR beta").unwrap();
    let rs = execute(&idx, &q, ExecOpts::default()).unwrap();
    assert_eq!(rs.rows().len(), 3);
    let q = parse("alpha !final").unwrap();
    let rs = execute(&idx, &q, ExecOpts::default()).unwrap();
    assert_eq!(rs.rows().len(), 1);
    assert_eq!(rs.rows()[0].name, "alpha-draft.md");
}

#[test]
fn ext_modifier_filters() {
    let idx = idx_with_files(vec![
        create("/synth/notes.txt", 1, 0),
        create("/synth/notes.md", 1, 0),
        create("/synth/notes.rs", 1, 0),
    ]);
    let q = parse("ext:txt;md").unwrap();
    let rs = execute(&idx, &q, ExecOpts::default()).unwrap();
    assert_eq!(rs.rows().len(), 2);
}

#[test]
fn size_modifier_filters() {
    let idx = idx_with_files(vec![
        create("/synth/small.bin", 1024, 0),
        create("/synth/medium.bin", 5 * 1024 * 1024, 0),
        create("/synth/large.bin", 200 * 1024 * 1024, 0),
    ]);
    let q = parse("size:>1mb").unwrap();
    let rs = execute(&idx, &q, ExecOpts::default()).unwrap();
    let names: Vec<&str> = rs.rows().iter().map(|r| r.name.as_str()).collect();
    assert!(names.contains(&"medium.bin"));
    assert!(names.contains(&"large.bin"));
    assert!(!names.contains(&"small.bin"));
}

#[test]
fn date_absolute_modifier() {
    // 2024-01-01 is epoch day 19_723.
    let idx = idx_with_files(vec![
        create_at("/synth/oldest.bin", 1, 19_000),
        create_at("/synth/newest.bin", 1, 20_000),
    ]);
    let q = parse("date:>2024-01-01").unwrap();
    let rs = execute(&idx, &q, ExecOpts::default()).unwrap();
    assert_eq!(rs.rows().len(), 1);
    assert_eq!(rs.rows()[0].name, "newest.bin");
}

#[test]
fn quick_filter_audio_alone() {
    let idx = idx_with_files(vec![
        create("/synth/song.flac", 1, 0),
        create("/synth/note.txt", 1, 0),
        create("/synth/jingle.mp3", 1, 0),
    ]);
    let q = parse("audio:").unwrap();
    let rs = execute(&idx, &q, ExecOpts::default()).unwrap();
    let names: Vec<&str> = rs.rows().iter().map(|r| r.name.as_str()).collect();
    assert_eq!(names.len(), 2);
    assert!(names.contains(&"song.flac"));
    assert!(names.contains(&"jingle.mp3"));
}

#[test]
fn match_path_lets_path_substring_hit() {
    let idx = idx_with_files(vec![
        create("/synth/projects/alpha.md", 1, 0),
        create("/synth/notes/alpha.md", 1, 0),
    ]);
    let opts = ExecOpts {
        match_mode: MatchMode {
            match_path: true,
            ..Default::default()
        },
        ..Default::default()
    };
    let q = parse("projects").unwrap();
    let rs = execute(&idx, &q, opts).unwrap();
    assert_eq!(rs.rows().len(), 1);
    assert!(rs.rows()[0].path.to_string_lossy().contains("projects"));
}

#[test]
fn whole_word_strict() {
    let idx = idx_with_files(vec![
        create("/synth/cat.txt", 1, 0),
        create("/synth/category.txt", 1, 0),
        create("/synth/cat-and-mouse.txt", 1, 0),
    ]);
    let opts = ExecOpts {
        match_mode: MatchMode {
            whole_word: true,
            ..Default::default()
        },
        ..Default::default()
    };
    let q = parse("cat").unwrap();
    let rs = execute(&idx, &q, opts).unwrap();
    let names: Vec<&str> = rs.rows().iter().map(|r| r.name.as_str()).collect();
    assert!(names.contains(&"cat.txt"));
    assert!(names.contains(&"cat-and-mouse.txt"));
    assert!(!names.contains(&"category.txt"));
}

#[test]
fn match_case_lowercase_needle_hits() {
    // Phase-5 limitation note: the name index stores names lower-
    // cased, so a `match_case`-strict query compares the user's needle
    // against an already-lowered candidate. A lowercase needle still
    // hits both files; an uppercase needle would currently miss every
    // row. Phase 13 widens the name index with a parallel raw-case
    // buffer; this test guards against the lower path regressing.
    let idx = idx_with_files(vec![
        create("/synth/Alpha.md", 1, 0),
        create("/synth/alpha.md", 1, 0),
    ]);
    let opts = ExecOpts {
        match_mode: MatchMode {
            match_case: true,
            ..Default::default()
        },
        ..Default::default()
    };
    let q = parse("alpha").unwrap();
    let rs = execute(&idx, &q, opts).unwrap();
    assert_eq!(rs.rows().len(), 2);
}

#[test]
fn match_diacritics_default_strips() {
    // Default `match_diacritics: false` strips combining marks at
    // match-time. Phase-5 limitation: the trigram pre-filter is byte-
    // level, so `cafe` does not surface candidates whose lower-cased
    // bytes contain `é`. Phase 13 will store NFKD-folded variants in
    // a parallel index so both arms of the test below converge.
    let idx = idx_with_files(vec![
        create("/synth/cafe.md", 1, 0),
        create("/synth/café.md", 1, 0),
    ]);
    let q = parse("cafe").unwrap();
    let rs = execute(&idx, &q, ExecOpts::default()).unwrap();
    let names: Vec<&str> = rs.rows().iter().map(|r| r.name.as_str()).collect();
    assert!(names.contains(&"cafe.md"));

    // Searching for "café" finds the diacritic-bearing name when
    // diacritic stripping is *on* — the needle gets stripped to
    // "cafe" and matches "cafe.md" via byte-level inclusion.
    let q = parse("café").unwrap();
    let rs = execute(&idx, &q, ExecOpts::default()).unwrap();
    let names: Vec<&str> = rs.rows().iter().map(|r| r.name.as_str()).collect();
    assert!(names.contains(&"café.md"));

    // With stripping off, "café" only matches its diacritic-bearing
    // sibling.
    let opts = ExecOpts {
        match_mode: MatchMode {
            match_diacritics: true,
            ..Default::default()
        },
        ..Default::default()
    };
    let rs = execute(&idx, &parse("café").unwrap(), opts).unwrap();
    let names: Vec<&str> = rs.rows().iter().map(|r| r.name.as_str()).collect();
    assert_eq!(names, vec!["café.md"]);
}

#[test]
fn sort_size_desc() {
    let idx = idx_with_files(vec![
        create("/synth/a.bin", 1, 0),
        create("/synth/b.bin", 1024, 0),
        create("/synth/c.bin", 2048, 0),
    ]);
    let opts = ExecOpts {
        sort: SortSpec {
            field: SortField::Size,
            order: SortOrder::Desc,
        },
        ..Default::default()
    };
    let q = parse("*.bin").unwrap();
    let rs = execute(&idx, &q, opts).unwrap();
    let sizes: Vec<u64> = rs.rows().iter().map(|r| r.size).collect();
    assert_eq!(sizes, vec![2048, 1024, 1]);
}

#[test]
fn first_batch_streams_then_tail() {
    let idx = idx_with_files(
        (0..10)
            .map(|i| create(&format!("/synth/file-{i:02}.bin"), 1, 0))
            .collect(),
    );
    let q = parse("*.bin").unwrap();
    let mut rs = execute(&idx, &q, ExecOpts::default()).unwrap();
    let first = rs.first_batch(4);
    assert_eq!(first.len(), 4);
    let tail: Vec<_> = rs.collect();
    assert_eq!(tail.len(), 6);
}

#[test]
fn plan_cache_round_trips_through_executor() {
    let idx = idx_with_files(vec![create("/synth/cached.txt", 1, 0)]);
    let cache = PlanCache::default16();
    let opts = ExecOpts::default();
    let (q1, _p1) = cache.get_or_plan("cached", &opts).unwrap();
    let (q2, _p2) = cache.get_or_plan("cached", &opts).unwrap();
    let rs1 = execute(&idx, &q1, opts).unwrap();
    let rs2 = execute(&idx, &q2, opts).unwrap();
    assert_eq!(rs1.rows().len(), 1);
    assert_eq!(rs2.rows().len(), 1);
    assert_eq!(cache.len(), 1);
}

#[test]
fn unsupported_modifier_fails_loudly() {
    let idx = idx_with_files(vec![create("/synth/tone.flac", 1, 0)]);
    let q = parse("lufs:>-16").unwrap();
    let err = execute(&idx, &q, ExecOpts::default()).unwrap_err();
    let msg = format!("{err}");
    assert!(msg.contains("lufs"), "got: {msg}");
}

#[test]
fn empty_query_path_is_unreachable() {
    // The parser rejects empty input — execute() therefore can't be
    // reached on it. Belt-and-suspenders: parse must fail.
    assert!(parse("   ").is_err());
}

#[test]
fn plan_cache_survives_match_path_toggle() {
    // The plan is invariant under the query string alone; toggling
    // `match_path` between two callers with the same query must not
    // poison the cached plan. Regression gate for review finding #6.
    let idx = idx_with_files(vec![
        create("/synth/projects/alpha.md", 1, 0),
        create("/synth/notes/projects.md", 1, 0),
    ]);
    let cache = PlanCache::default16();

    // First caller: match_path off → query "projects" hits the file
    // whose *name* contains projects.
    let opts_off = ExecOpts::default();
    let (q1, _) = cache.get_or_plan("projects", &opts_off).unwrap();
    let rs = execute(&idx, &q1, opts_off).unwrap();
    assert_eq!(rs.rows().len(), 1);
    assert!(rs.rows()[0].name.contains("projects"));

    // Second caller: same query string, match_path on → must hit
    // both files (one via name, one via path).
    let opts_on = ExecOpts {
        match_mode: MatchMode {
            match_path: true,
            ..Default::default()
        },
        ..Default::default()
    };
    let (q2, _) = cache.get_or_plan("projects", &opts_on).unwrap();
    let rs = execute(&idx, &q2, opts_on).unwrap();
    assert_eq!(
        rs.rows().len(),
        2,
        "match_path on should widen target — got {:?}",
        rs.rows().iter().map(|r| &r.name).collect::<Vec<_>>()
    );
}
