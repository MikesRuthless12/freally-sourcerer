//! Build 3 (v0.23.0) smoke — SRC-M24 natural sort, end to end.
//!
//! The comparator itself is unit-tested in
//! `crates/freally-query/src/natural.rs`. What this proves is the
//! wiring: that `execute` actually returns rows in natural order by
//! default, that the settings opt-out reaches `sort_rows`, and that the
//! ordering is applied to every string column rather than to the name
//! column alone.
//!
//! Same shape as the Build 1 DSL smoke: hand-built journal events into a
//! `tempfile` index, then the public `parse` + `execute` surface.

use std::path::PathBuf;

use freally_index::Index;
use freally_journal::JournalEvent;
use freally_query::{ExecOpts, SortField, SortOrder, SortSpec, execute, parse};
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

/// Names chosen so byte order and natural order disagree on every one:
/// byte order gives shot1, shot10, shot2, shot20, shot3.
fn fixture_index() -> std::sync::Arc<Index> {
    let dir = tempdir().unwrap();
    let idx = Index::open(dir.path()).unwrap();
    idx.apply(&[
        file("/reel/shot10.mov", 10),
        file("/reel/shot2.mov", 20),
        file("/reel/shot1.mov", 30),
        file("/reel/shot20.mov", 40),
        file("/reel/shot3.mov", 50),
    ])
    .unwrap();
    idx.commit().unwrap();
    std::mem::forget(dir);
    idx
}

fn names_with(sort: SortSpec) -> Vec<String> {
    let idx = fixture_index();
    let q = parse("shot").unwrap();
    let rs = execute(
        &idx,
        &q,
        ExecOpts {
            sort,
            ..Default::default()
        },
    )
    .unwrap();
    rs.rows().iter().map(|r| r.name.clone()).collect()
}

#[test]
fn the_default_name_sort_is_natural() {
    // No SortSpec supplied at all — the point of SRC-M24 is that this is
    // the default, not an opt-in.
    assert_eq!(
        names_with(SortSpec::default()),
        vec![
            "shot1.mov",
            "shot2.mov",
            "shot3.mov",
            "shot10.mov",
            "shot20.mov"
        ]
    );
}

#[test]
fn the_opt_out_restores_byte_ordering() {
    assert_eq!(
        names_with(SortSpec {
            natural: false,
            ..Default::default()
        }),
        vec![
            "shot1.mov",
            "shot10.mov",
            "shot2.mov",
            "shot20.mov",
            "shot3.mov"
        ]
    );
}

#[test]
fn descending_natural_order_is_the_ascending_order_reversed() {
    assert_eq!(
        names_with(SortSpec {
            order: SortOrder::Desc,
            ..Default::default()
        }),
        vec![
            "shot20.mov",
            "shot10.mov",
            "shot3.mov",
            "shot2.mov",
            "shot1.mov"
        ]
    );
}

#[test]
fn the_path_column_is_natural_too() {
    // Sorting by path must not quietly fall back to byte order — every
    // string column goes through the same comparator.
    let by_path = names_with(SortSpec {
        field: SortField::Path,
        ..Default::default()
    });
    assert_eq!(
        by_path,
        vec![
            "shot1.mov",
            "shot2.mov",
            "shot3.mov",
            "shot10.mov",
            "shot20.mov"
        ]
    );
}

#[test]
fn a_non_string_column_is_untouched() {
    // Size ordering has nothing to do with digit runs in names; this
    // guards against the comparator being wired in too broadly.
    assert_eq!(
        names_with(SortSpec {
            field: SortField::Size,
            ..Default::default()
        }),
        vec![
            "shot10.mov",
            "shot2.mov",
            "shot1.mov",
            "shot20.mov",
            "shot3.mov"
        ]
    );
}
