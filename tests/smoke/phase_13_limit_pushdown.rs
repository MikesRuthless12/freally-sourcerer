//! TASK-100 — the limit pushdown must not change *which* rows come back.
//!
//! The executor used to hydrate every surviving row and only then truncate
//! to `ExecOpts::limit`: on the bench's `literal-hot` scenario, 10 035
//! SQLite rows fetched to return 1 000. It now orders the survivor *names*
//! — which the name index already holds, lowercased — and hydrates only the
//! page that survives that sort. `literal-hot` P50 went 31.6 ms → 4.3 ms.
//!
//! That is worth nothing if the page is a different page, so every
//! assertion here runs the same query twice: once with a limit (the
//! pushdown) and once with `limit: 0`, which is uncapped and therefore
//! takes the whole-set path the pushdown replaced. The pushed-down page
//! has to equal the head of the uncapped answer, exactly and in order.

use std::path::PathBuf;
use std::sync::Arc;

use freally_index::{FileRow, Index};
use freally_journal::JournalEvent;
use freally_query::{ExecOpts, SortField, SortOrder, SortSpec, execute, parse};
use tempfile::tempdir;

/// Odd on purpose. Every name in the fixture is a tied pair, so an odd
/// limit always cuts a pair in half — in either sort direction. An even
/// limit lands on a group boundary and the page comes out right whether
/// or not the ordering is total, which is a test that proves nothing.
const LIMIT: usize = 9;

/// 80 rows sharing one literal — 40 names, **each duplicated** into a
/// second directory, so every rank in the ordering is a tie.
///
/// Ties are the whole risk in this change: the pushdown picks its page
/// with `select_nth_unstable_by`, which may return any member of a tied
/// group unless the comparator is a total order. Distinct names would pass
/// whether or not that holds. Sizes stay distinct across the two copies so
/// the size-sort case below still has something to order by.
fn fixture() -> Arc<Index> {
    let dir = tempdir().unwrap();
    let idx = Index::open(dir.path()).unwrap();
    let mut events = Vec::new();
    for i in 0..40u64 {
        events.push(create(&format!("/synth/a/item-{i:02}.txt"), 1_000 + i));
        events.push(create(&format!("/synth/b/item-{i:02}.txt"), 9_000 + i));
    }
    idx.apply(&events).unwrap();
    idx.commit().unwrap();
    // The index outlives the TempDir guard on purpose — the same trick
    // `phase_05_filename_lens` uses so a fixture can be returned by value.
    std::mem::forget(dir);
    idx
}

fn create(path: &str, size: u64) -> JournalEvent {
    JournalEvent::Create {
        path: PathBuf::from(path),
        size,
        mtime_ns: 1_704_067_200 * 1_000_000_000,
        ctime_ns: 1_704_067_200 * 1_000_000_000,
        attrs: 0,
    }
}

/// Paths, not names: half the fixture's names are duplicated, so names
/// alone could not tell a wrong page from a right one.
fn paths(rows: &[FileRow]) -> Vec<String> {
    rows.iter()
        .map(|r| r.path.to_string_lossy().into_owned())
        .collect()
}

fn run(idx: &Index, query: &str, sort: SortSpec, limit: usize) -> Vec<String> {
    let q = parse(query).unwrap();
    let rs = execute(
        idx,
        &q,
        ExecOpts {
            sort,
            limit,
            ..Default::default()
        },
    )
    .unwrap();
    paths(rs.rows())
}

/// `limit: 0` is uncapped, which is the whole-set path; `LIMIT` is the
/// pushdown. They have to agree on the head.
fn assert_page_matches(idx: &Index, query: &str, sort: SortSpec) {
    let all = run(idx, query, sort, 0);
    assert!(
        all.len() > LIMIT,
        "{query} must return more than {LIMIT} rows or this proves nothing — got {}",
        all.len()
    );
    let page = run(idx, query, sort, LIMIT);
    assert_eq!(
        page,
        all[..LIMIT],
        "{query} paged differently under {sort:?}"
    );
}

fn sort_by(field: SortField, order: SortOrder, natural: bool) -> SortSpec {
    SortSpec {
        field,
        order,
        natural,
    }
}

#[test]
fn the_pushed_down_page_is_the_page_the_full_sort_would_have_kept() {
    let idx = fixture();
    // Both directions and both comparators: `natural` picks between
    // `natural_cmp` and a byte compare, and Desc reverses the whole
    // comparison including its tie-break. Each is a separate chance for
    // the pushdown's copy of the ordering to drift from `sort_rows`.
    for order in [SortOrder::Asc, SortOrder::Desc] {
        for natural in [true, false] {
            assert_page_matches(&idx, "item", sort_by(SortField::Name, order, natural));
        }
    }
}

#[test]
fn the_page_does_not_move_between_runs() {
    let idx = fixture();
    let sort = sort_by(SortField::Name, SortOrder::Asc, true);
    let first = run(&idx, "item", sort, LIMIT);
    for _ in 0..8 {
        assert_eq!(
            run(&idx, "item", sort, LIMIT),
            first,
            "the partition returned a different set of tied rows"
        );
    }
}

#[test]
fn a_sort_the_name_buffer_cannot_answer_still_pages_correctly() {
    let idx = fixture();
    // Size, date and ext live on the SQLite row, so the pushdown has to
    // stand down and let every survivor hydrate. This is the assertion
    // that fails if that guard is ever dropped.
    for field in [SortField::Size, SortField::Date, SortField::Ext] {
        for order in [SortOrder::Asc, SortOrder::Desc] {
            assert_page_matches(&idx, "item", sort_by(field, order, true));
        }
    }
}

#[test]
fn a_predicate_that_needs_hydration_still_pages_correctly() {
    let idx = fixture();
    // `size:` cannot be answered from the name buffer, so these rows have
    // to survive SQLite before the limit may be applied. Pushing the limit
    // ahead of that filter would page from the wrong set.
    let sort = sort_by(SortField::Name, SortOrder::Asc, true);
    assert_page_matches(&idx, "item size:>1005", sort);
    assert_page_matches(&idx, "item ext:txt", sort);
}
