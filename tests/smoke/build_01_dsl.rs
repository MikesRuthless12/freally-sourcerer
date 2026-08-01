//! Build 1 (v0.21.0) smoke — the DSL half: SRC-M07 `dupe:` and
//! SRC-M08 `empty:` / `child-count:` / `descendant-count:`.
//!
//! OS-agnostic, runs on every CI matrix entry. Same shape as the
//! Phase-5 smoke: hand-built journal events into a `tempfile` index,
//! then the public `parse` + `execute` surface.
//!
//! Gates:
//!   1. Both families parse, including the hyphenated keys, and a
//!      hyphenated *non*-modifier still classifies as a literal term
//!      (Standing Rule #8).
//!   2. `empty:` distinguishes zero-byte files, childless folders, and
//!      the roots of nested empty chains — from index data alone.
//!   3. `child-count:` / `descendant-count:` answer only for folders.
//!   4. `dupe:` / `name-dupe:` / `size-dupe:` cluster real duplicates,
//!      drop singletons, and expose contiguous groups.
//!   5. A `dupe:` buried under OR / NOT is a typed error, not a
//!      silently-wrong result set.

use std::path::PathBuf;

use freally_index::{ATTR_DIRECTORY, Index};
use freally_journal::JournalEvent;
use freally_query::{ExecOpts, QueryError, execute, parse};
use tempfile::tempdir;

fn file(path: &str, size: u64) -> JournalEvent {
    JournalEvent::Create {
        path: PathBuf::from(path),
        size,
        mtime_ns: 0,
        ctime_ns: 0,
        attrs: 0,
    }
}

fn folder(path: &str) -> JournalEvent {
    JournalEvent::Create {
        path: PathBuf::from(path),
        size: 0,
        mtime_ns: 0,
        ctime_ns: 0,
        attrs: ATTR_DIRECTORY as u32,
    }
}

/// Fixture tree:
///
/// ```text
/// /vault/
///   docs/                     2 children, 2 descendants
///     report.pdf   (1200 B)
///     notes.txt    (0 B)      <- empty file
///   backup/                   1 child, 2 descendants
///     docs/                   1 child
///       report.pdf (1200 B)   <- name+size duplicate
///   media/
///     track.flac   (900 B)
///     track-copy.flac (900 B) <- size-only duplicate
///   hollow/                   0 children              <- empty folder
///   shell/                    root of an empty chain
///     inner/
///       deepest/
/// ```
fn fixture_index() -> std::sync::Arc<Index> {
    let dir = tempdir().unwrap();
    let idx = Index::open(dir.path()).unwrap();
    idx.apply(&[
        folder("/vault"),
        folder("/vault/docs"),
        file("/vault/docs/report.pdf", 1200),
        file("/vault/docs/notes.txt", 0),
        folder("/vault/backup"),
        folder("/vault/backup/docs"),
        file("/vault/backup/docs/report.pdf", 1200),
        folder("/vault/media"),
        file("/vault/media/track.flac", 900),
        file("/vault/media/track-copy.flac", 900),
        folder("/vault/hollow"),
        folder("/vault/shell"),
        folder("/vault/shell/inner"),
        folder("/vault/shell/inner/deepest"),
    ])
    .unwrap();
    idx.commit().unwrap();
    std::mem::forget(dir);
    idx
}

fn names(rs: &freally_query::ResultSet) -> Vec<String> {
    rs.rows().iter().map(|r| r.name.clone()).collect()
}

fn paths(rs: &freally_query::ResultSet) -> Vec<String> {
    let mut out: Vec<String> = rs
        .rows()
        .iter()
        .map(|r| r.path.to_string_lossy().replace('\\', "/"))
        .collect();
    out.sort();
    out
}

fn run(src: &str) -> freally_query::ResultSet {
    let idx = fixture_index();
    let q = parse(src).unwrap_or_else(|e| panic!("`{src}` failed to parse: {e}"));
    execute(&idx, &q, ExecOpts::default()).unwrap_or_else(|e| panic!("`{src}` failed: {e}"))
}

// ---------- SRC-M08: emptiness ----------------------------------------

#[test]
fn empty_file_finds_zero_byte_files_only() {
    let rs = run("empty:file");
    assert_eq!(names(&rs), vec!["notes.txt"]);
}

#[test]
fn empty_folder_finds_childless_directories() {
    let rs = run("empty:folder");
    assert_eq!(
        paths(&rs),
        vec!["/vault/hollow", "/vault/shell/inner/deepest"]
    );
}

#[test]
fn bare_empty_covers_files_and_folders() {
    let rs = run("empty:");
    assert_eq!(
        paths(&rs),
        vec![
            "/vault/docs/notes.txt",
            "/vault/hollow",
            "/vault/shell/inner/deepest",
        ]
    );
}

#[test]
fn empty_roots_collapses_nested_empty_chains() {
    // `/vault/shell` holds no files at any depth, so it is the root.
    // `inner` and `deepest` collapse into it. `/vault/hollow` is its
    // own root. `/vault` itself is not — it holds real files.
    let rs = run("empty:roots");
    assert_eq!(paths(&rs), vec!["/vault/hollow", "/vault/shell"]);
}

#[test]
fn child_count_counts_direct_children_of_folders() {
    let rs = run("child-count:2");
    assert_eq!(paths(&rs), vec!["/vault/docs", "/vault/media"]);
}

#[test]
fn child_count_takes_comparators() {
    let rs = run("child-count:>=4");
    assert_eq!(paths(&rs), vec!["/vault"], "/vault holds 5 subfolders");
}

#[test]
fn descendant_count_reaches_every_depth() {
    let rs = run("descendant-count:>=3");
    let got = paths(&rs);
    assert!(
        got.contains(&"/vault".to_string()),
        "/vault has 13 descendants, got {got:?}"
    );
    assert!(
        !got.contains(&"/vault/docs".to_string()),
        "/vault/docs has only 2 descendants, got {got:?}"
    );
}

#[test]
fn count_modifiers_never_match_files() {
    // A file has no children — `child-count:0` must not sweep every
    // file in the index into the result set.
    let rs = run("child-count:0");
    for row in rs.rows() {
        assert!(
            row.attrs & ATTR_DIRECTORY != 0,
            "{} is not a directory",
            row.path.display()
        );
    }
}

#[test]
fn emptiness_composes_with_other_predicates() {
    let rs = run("empty:file ext:txt");
    assert_eq!(names(&rs), vec!["notes.txt"]);

    let rs = run("empty:file ext:pdf");
    assert!(rs.rows().is_empty());
}

// ---------- SRC-M07: duplicates ---------------------------------------

#[test]
fn dupe_groups_same_name_and_size() {
    let rs = run("dupe:");
    assert_eq!(
        paths(&rs),
        vec!["/vault/backup/docs/report.pdf", "/vault/docs/report.pdf"],
        "track.flac and track-copy.flac differ by name, so name+size excludes them"
    );
    assert_eq!(rs.dupe_groups.len(), 1);
    let g = &rs.dupe_groups[0];
    assert_eq!((g.start, g.len), (0, 2));
    // The group carries the shared values, not a rendered header — the
    // UI localises and byte-formats them.
    assert_eq!(g.key.name.as_deref(), Some("report.pdf"));
    assert_eq!(g.key.size, Some(1200));
}

#[test]
fn a_name_only_group_carries_no_size_and_vice_versa() {
    let rs = run("name-dupe:");
    let key = &rs.dupe_groups[0].key;
    assert_eq!(key.name.as_deref(), Some("report.pdf"));
    assert_eq!(key.size, None, "name-dupe: does not group by size");

    let rs = run("size-dupe:");
    let key = &rs.dupe_groups[0].key;
    assert!(key.size.is_some());
    assert_eq!(key.name, None, "size-dupe: does not group by name");
}

#[test]
fn name_dupe_ignores_size() {
    let rs = run("name-dupe:");
    assert_eq!(
        paths(&rs),
        vec!["/vault/backup/docs/report.pdf", "/vault/docs/report.pdf"]
    );
}

#[test]
fn size_dupe_ignores_name() {
    let rs = run("size-dupe:");
    let got = paths(&rs);
    assert!(
        got.contains(&"/vault/media/track.flac".to_string())
            && got.contains(&"/vault/media/track-copy.flac".to_string()),
        "the two 900-byte flacs must group, got {got:?}"
    );
    assert!(
        got.contains(&"/vault/docs/report.pdf".to_string()),
        "the two 1200-byte pdfs also share a size, got {got:?}"
    );
}

#[test]
fn dupe_groups_are_contiguous_and_cover_every_row() {
    let rs = run("size-dupe:");
    let mut covered = 0usize;
    for g in &rs.dupe_groups {
        assert_eq!(g.start, covered, "groups must tile the row vector in order");
        assert!(g.len >= 2, "a singleton is not a duplicate group");
        covered += g.len;
    }
    assert_eq!(covered, rs.rows().len());
}

#[test]
fn dupe_never_reports_directories() {
    // `/vault/docs` and `/vault/backup/docs` share a name and a
    // (meaningless) size of 0 — they must not be reported as
    // duplicate files.
    let rs = run("dupe:");
    for row in rs.rows() {
        assert!(
            row.attrs & ATTR_DIRECTORY == 0,
            "{} is a directory",
            row.path.display()
        );
    }
}

#[test]
fn dupe_composes_with_other_predicates() {
    let rs = run("dupe: ext:pdf");
    assert_eq!(rs.rows().len(), 2);

    let rs = run("dupe: ext:flac");
    assert!(
        rs.rows().is_empty(),
        "the flacs differ by name, so name+size finds no pair"
    );
}

#[test]
fn buried_dupe_is_a_typed_error() {
    let idx = fixture_index();
    for src in [
        "report OR dupe:",
        "!dupe:",
        "NOT size-dupe:",
        // A top-level key must not license a buried one: the buried
        // node would be stripped to `True`, and `!True` rejects every
        // row — zero hits with no error at all.
        "dupe:name !size-dupe:",
        "dupe: (report OR name-dupe:)",
    ] {
        let q = parse(src).unwrap_or_else(|e| panic!("`{src}` failed to parse: {e}"));
        match execute(&idx, &q, ExecOpts::default()) {
            Err(QueryError::UnsupportedDupePosition) => {}
            other => panic!("`{src}` expected UnsupportedDupePosition, got {other:?}"),
        }
    }
}

#[test]
fn negating_an_emptiness_predicate_returns_the_complement() {
    // `eval_name` answers "true" for anything it cannot decide from the
    // name buffer so the hydrated pass can decide it. Under `NOT` that
    // convention inverts to a definite reject, and the row never
    // reaches the pass that could answer — so `!empty:` used to return
    // nothing at all.
    let all = run("*").rows().len();
    let empty_files = run("empty:file").rows().len();
    let not_empty_files = run("!empty:file").rows().len();
    assert!(empty_files > 0 && not_empty_files > 0);
    assert_eq!(
        empty_files + not_empty_files,
        all,
        "`empty:file` and `!empty:file` must partition the index"
    );

    let folders = run("empty:folder").rows().len();
    assert!(folders > 0);
    assert_eq!(run("NOT empty:folder").rows().len(), all - folders);
}

#[test]
fn empty_roots_reports_the_top_of_a_wholly_file_free_tree() {
    // Every row synthesises a `counts` entry for its parent, so a
    // membership test against `counts` alone would treat the scan
    // root's own parent as indexed and report no roots at all.
    let dir = tempdir().unwrap();
    let idx = Index::open(dir.path()).unwrap();
    idx.apply(&[
        folder("/photos"),
        folder("/photos/2024"),
        folder("/photos/2024/raw"),
    ])
    .unwrap();
    idx.commit().unwrap();
    std::mem::forget(dir);

    let q = parse("empty:roots").unwrap();
    let rs = execute(&idx, &q, ExecOpts::default()).unwrap();
    let got: Vec<String> = rs
        .rows()
        .iter()
        .map(|r| r.path.to_string_lossy().replace('\\', "/"))
        .collect();
    assert_eq!(got, vec!["/photos"], "the whole tree is one empty root");
}

#[test]
fn name_dupe_and_size_dupe_reject_a_value() {
    // `name-dupe:size` is a plausible slip for `dupe:size`; silently
    // meaning "name" would be worse than saying so.
    for src in ["name-dupe:size", "size-dupe:name"] {
        assert!(
            parse(src).is_err(),
            "`{src}` should not silently pick a different key"
        );
    }
}

// ---------- Standing Rule #8 -------------------------------------------

#[test]
fn hyphenated_non_modifiers_still_parse_as_literals() {
    // Widening the modifier-key charset to `-` wholesale would turn
    // these into `UnknownModifier` parse errors.
    for src in ["re-name:v2", "some-thing:else", "x-y-z:1"] {
        let parsed = parse(src).unwrap_or_else(|e| panic!("`{src}` must parse as text: {e}"));
        let rendered = format!("{:?}", parsed.root());
        assert!(
            rendered.contains("Text"),
            "`{src}` classified as {rendered}, expected a Text node"
        );
    }
}

#[test]
fn dupe_and_empty_survive_strict_everything() {
    // Both families exist in voidtools' Everything 1.5, so
    // `--strict-everything` must not reject them.
    use freally_query::{ParseOpts, parse_with};
    for src in [
        "dupe:",
        "name-dupe:",
        "size-dupe:",
        "empty:",
        "empty:folder",
        "child-count:>0",
        "descendant-count:<10",
    ] {
        parse_with(src, ParseOpts::strict())
            .unwrap_or_else(|e| panic!("`{src}` rejected under strict-everything: {e}"));
    }
}
