//! Phase 5 smoke — OS-agnostic, runs on every CI matrix entry.
//!
//! Mirrors Phase 4's smoke shape: hand-built journal events, an
//! in-tree `tempfile` index, the public `parse` + `execute` surface.
//! Asserts the four invariants the Build Guide gates Phase 5 on:
//!
//!   1. The voidtools-syntax grammar — literal / wildcard / regex
//!      terms, `AND` / `OR` / `NOT` / `!` glue, `size:` / `date:` /
//!      `ext:` / `attrib:` / `path:` / `parent:` / `child:` modifiers,
//!      and the quick-filter aliases — all parse and route to the
//!      filename lens.
//!   2. `ExecOpts` toggles (`match_case`, `whole_word`, `match_path`,
//!      `match_diacritics`) take effect at execute time without
//!      changing the parsed query.
//!   3. `SortSpec` re-orders results across the documented fields.
//!   4. The 16-entry plan cache round-trips identical inputs without
//!      re-parsing.

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

fn fixture_index() -> std::sync::Arc<Index> {
    let dir = tempdir().unwrap();
    let idx = Index::open(dir.path()).unwrap();
    idx.apply(&[
        create(
            "/synth/projects/alpha/report-q1.pdf",
            1_500_000,
            day(20_100),
        ),
        create(
            "/synth/projects/alpha/report-q2.pdf",
            1_750_000,
            day(20_120),
        ),
        create("/synth/projects/alpha/draft.md", 4_000, day(20_115)),
        create("/synth/projects/beta/scratch.txt", 2_000, day(20_130)),
        create("/synth/inbox/song-final.flac", 80_000_000, day(20_120)),
        create("/synth/inbox/song-mix.wav", 110_000_000, day(20_130)),
        create("/synth/inbox/cover.png", 1_200_000, day(20_140)),
        create("/synth/code/lib/parser.rs", 16_000, day(20_125)),
        create("/synth/code/lib/lexer.rs", 12_000, day(20_125)),
        create("/synth/code/README.md", 5_000, day(20_125)),
    ])
    .unwrap();
    idx.commit().unwrap();
    std::mem::forget(dir);
    idx
}

fn day(epoch_day: i64) -> i128 {
    (epoch_day as i128) * 86_400 * 1_000_000_000
}

fn names(rows: &[sourcerer_index::FileRow]) -> Vec<&str> {
    rows.iter().map(|r| r.name.as_str()).collect()
}

#[test]
fn literal_substring_routes_through_filename_lens() {
    let idx = fixture_index();
    let q = parse("report").unwrap();
    let rs = execute(&idx, &q, ExecOpts::default()).unwrap();
    assert_eq!(rs.rows().len(), 2);
    for r in rs.rows() {
        assert!(r.name.contains("report"), "got {}", r.name);
    }
}

#[test]
fn wildcard_and_regex_lenses() {
    let idx = fixture_index();
    let rs = execute(&idx, &parse("*.rs").unwrap(), ExecOpts::default()).unwrap();
    assert_eq!(rs.rows().len(), 2);
    let rs = execute(
        &idx,
        &parse(r"regex:^report-q\d\.pdf$").unwrap(),
        ExecOpts::default(),
    )
    .unwrap();
    assert_eq!(rs.rows().len(), 2);
}

#[test]
fn boolean_glue_parses_and_executes() {
    let idx = fixture_index();
    let rs = execute(&idx, &parse("song AND mix").unwrap(), ExecOpts::default()).unwrap();
    assert_eq!(rs.rows().len(), 1);
    assert_eq!(rs.rows()[0].name, "song-mix.wav");

    let rs = execute(&idx, &parse("alpha OR beta").unwrap(), ExecOpts::default()).unwrap();
    assert!(rs.rows().is_empty(), "neither token appears in any *name*");

    let rs = execute(&idx, &parse("song !mix").unwrap(), ExecOpts::default()).unwrap();
    assert_eq!(rs.rows().len(), 1);
    assert_eq!(rs.rows()[0].name, "song-final.flac");
}

#[test]
fn ext_size_date_modifiers_compose() {
    let idx = fixture_index();
    let q = parse("ext:pdf size:>1mb").unwrap();
    let rs = execute(&idx, &q, ExecOpts::default()).unwrap();
    assert_eq!(rs.rows().len(), 2);
    for r in rs.rows() {
        assert_eq!(r.ext.as_deref(), Some("pdf"));
        assert!(r.size > 1024 * 1024);
    }

    // 20_120 is 2025-01-13. Pick a cutoff that excludes the q1 report
    // and includes everything else.
    let q = parse("ext:pdf date:>2025-01-12").unwrap();
    let rs = execute(&idx, &q, ExecOpts::default()).unwrap();
    let n = names(rs.rows());
    assert_eq!(n, vec!["report-q2.pdf"]);
}

#[test]
fn quick_filter_audio_routes_to_extensions() {
    let idx = fixture_index();
    let rs = execute(&idx, &parse("audio:").unwrap(), ExecOpts::default()).unwrap();
    let n = names(rs.rows());
    assert!(n.contains(&"song-final.flac"));
    assert!(n.contains(&"song-mix.wav"));
    assert_eq!(n.len(), 2);
}

#[test]
fn match_path_widens_target() {
    let idx = fixture_index();
    let opts = ExecOpts {
        match_mode: MatchMode {
            match_path: true,
            ..Default::default()
        },
        ..Default::default()
    };
    let rs = execute(&idx, &parse("projects").unwrap(), opts).unwrap();
    // Three files live under /synth/projects/.
    assert_eq!(rs.rows().len(), 4);
}

#[test]
fn whole_word_excludes_partials() {
    let idx = fixture_index();
    let opts = ExecOpts {
        match_mode: MatchMode {
            whole_word: true,
            ..Default::default()
        },
        ..Default::default()
    };
    let rs = execute(&idx, &parse("song").unwrap(), opts).unwrap();
    let n = names(rs.rows());
    assert_eq!(n.len(), 2);
    assert!(n.contains(&"song-final.flac"));
    assert!(n.contains(&"song-mix.wav"));
}

#[test]
fn sort_specs_reorder_results() {
    let idx = fixture_index();
    // Sort by size desc — the largest is song-mix.wav at 110 MB.
    let opts = ExecOpts {
        sort: SortSpec {
            field: SortField::Size,
            order: SortOrder::Desc,
        },
        ..Default::default()
    };
    let rs = execute(&idx, &parse("audio:").unwrap(), opts).unwrap();
    let n = names(rs.rows());
    assert_eq!(n[0], "song-mix.wav");
    assert_eq!(n[1], "song-final.flac");
}

#[test]
fn plan_cache_round_trips_identical_inputs() {
    let idx = fixture_index();
    let cache = PlanCache::default16();
    let opts = ExecOpts::default();
    let _ = cache.get_or_plan("ext:rs", &opts).unwrap();
    let (q2, _p2) = cache.get_or_plan("ext:rs", &opts).unwrap();
    let rs = execute(&idx, &q2, opts).unwrap();
    assert_eq!(rs.rows().len(), 2);
    assert_eq!(cache.len(), 1);
}

#[test]
fn first_batch_then_tail_streams() {
    let idx = fixture_index();
    let mut rs = execute(&idx, &parse("*.md").unwrap(), ExecOpts::default()).unwrap();
    let first = rs.first_batch(1);
    assert_eq!(first.len(), 1);
    let tail = rs.collect();
    // 3 markdown files in the fixture (.md from draft.md, README.md);
    // wildcard `*.md` matches both. Plus `*.md` matches only files
    // ending in `.md` — exactly two here.
    let total = first.len() + tail.len();
    assert!(total >= 2, "expected ≥2 *.md hits got {total}");
}
