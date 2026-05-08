//! Phase 6 smoke — OS-agnostic, runs on every CI matrix entry.
//!
//! Mirrors Phase 5's smoke shape: a hand-built journal-event corpus, an
//! in-tree `tempfile` index, the public `parse` + `execute_with`
//! surface. Asserts the four invariants the Build Guide gates Phase 6
//! on:
//!
//!   1. The new `similar:<needle>` modifier parses, plans, and routes
//!      through the LSH-backed similarity index.
//!   2. Composing `similar:` with other modifiers (`ext:` / `size:`)
//!      filters correctly post-LSH.
//!   3. Calling the legacy `execute()` entry-point on a `similar:`
//!      query returns the typed `SimilarityIndexUnavailable` error so
//!      callers see a clear message, not empty results.
//!   4. `similar:` buried inside OR / NOT / nested AND surfaces the
//!      typed `UnsupportedSimilarPosition` error — Phase 6 is the
//!      top-level-only first cut; later phases lift the restriction.

use std::path::PathBuf;
use std::sync::Arc;

use sourcerer_index::Index;
use sourcerer_journal::JournalEvent;
use sourcerer_query::{ExecOpts, QueryError, execute, execute_with, parse};
use sourcerer_similarity::SimilarityIndex;
use tempfile::tempdir;

fn create(path: &str) -> JournalEvent {
    JournalEvent::Create {
        path: PathBuf::from(path),
        size: 1024,
        mtime_ns: 1_700_000_000_000_000_000,
        ctime_ns: 1_700_000_000_000_000_000,
        attrs: 0,
    }
}

fn create_sized(path: &str, size: u64) -> JournalEvent {
    JournalEvent::Create {
        path: PathBuf::from(path),
        size,
        mtime_ns: 1_700_000_000_000_000_000,
        ctime_ns: 1_700_000_000_000_000_000,
        attrs: 0,
    }
}

/// Build a paired (Index, SimilarityIndex) from the same JournalEvent
/// stream. The harness mirrors the wiring the Phase 11 daemon will do:
/// every Create / Rename / Delete fans out to both stores so they stay
/// keyed by the same `file_id`.
fn paired_indexes(events: &[JournalEvent]) -> (Arc<Index>, SimilarityIndex) {
    let dir_idx = tempdir().unwrap();
    let dir_sim = tempdir().unwrap();
    let idx = Index::open(dir_idx.path()).unwrap();
    let sim = SimilarityIndex::open(dir_sim.path()).unwrap();
    idx.apply(events).unwrap();
    idx.commit().unwrap();
    // Walk the canonical store to pull file_id → name pairs and feed
    // the similarity index. Phase 11's daemon will subscribe directly
    // to the JournalEvent stream; for the smoke we replay through
    // SQLite so the file_id derivation matches sourcerer-index exactly.
    idx.store()
        .iter_all(|row| {
            sim.upsert(row.file_id as u64, &row.name_lower).unwrap();
        })
        .unwrap();
    sim.flush().unwrap();
    std::mem::forget(dir_idx);
    std::mem::forget(dir_sim);
    (idx, sim)
}

#[test]
fn similar_modifier_finds_near_duplicates() {
    // Phase 6 LSH (K=128, b=16, r=8) has its recall knee at Jaccard
    // ≈ 0.73 — a long stem with a `-v2` suffix bump lands at Jaccard
    // ≈ 0.86, well above the knee (P[match] ≥ 0.99). Heavier edits
    // (multi-char appends, mid-stem typos) sit closer to the knee
    // by design and the spec's 95 % gate over a 5 000-name corpus
    // is exercised by `crates/sourcerer-similarity/tests/recall.rs`.
    let (idx, sim) = paired_indexes(&[
        create("/synth/projects/quarterly-report-final-summary.pdf"),
        create("/synth/projects/quarterly-report-final-summary-v2.pdf"),
        create("/synth/projects/totally-unrelated-binary.bin"),
        create("/synth/projects/another-pdf-document-here.pdf"),
    ]);
    let q = parse("similar:quarterly-report-final-summary").unwrap();
    let rs = execute_with(&idx, Some(&sim), &q, ExecOpts::default()).unwrap();
    let names: Vec<&str> = rs.rows().iter().map(|r| r.name.as_str()).collect();
    assert!(
        names.contains(&"quarterly-report-final-summary.pdf"),
        "exact match missed: {names:?}"
    );
    assert!(
        names.contains(&"quarterly-report-final-summary-v2.pdf"),
        "suffix bump missed: {names:?}"
    );
    assert!(
        !names.contains(&"totally-unrelated-binary.bin"),
        "unrelated leaked: {names:?}"
    );
    // Default sort is Jaccard desc — the exact match must lead.
    assert_eq!(names[0], "quarterly-report-final-summary.pdf");
}

#[test]
fn similar_compound_with_ext_filter() {
    let (idx, sim) = paired_indexes(&[
        create("/synth/quarterly-report-final-summary.pdf"),
        create("/synth/quarterly-report-final-summary.txt"),
        create("/synth/quarterly-report-final-summary-v2.pdf"),
    ]);
    let q = parse("similar:quarterly-report-final-summary ext:pdf").unwrap();
    let rs = execute_with(&idx, Some(&sim), &q, ExecOpts::default()).unwrap();
    let names: Vec<&str> = rs.rows().iter().map(|r| r.name.as_str()).collect();
    assert!(names.contains(&"quarterly-report-final-summary.pdf"));
    assert!(names.contains(&"quarterly-report-final-summary-v2.pdf"));
    assert!(
        !names.contains(&"quarterly-report-final-summary.txt"),
        "ext filter failed: {names:?}"
    );
}

#[test]
fn similar_compound_with_size_filter() {
    let (idx, sim) = paired_indexes(&[
        create_sized("/synth/quarterly-report-final-summary.pdf", 100),
        create_sized("/synth/quarterly-report-final-summary-v2.pdf", 5_000_000),
    ]);
    let q = parse("similar:quarterly-report-final-summary size:>1mb").unwrap();
    let rs = execute_with(&idx, Some(&sim), &q, ExecOpts::default()).unwrap();
    let names: Vec<&str> = rs.rows().iter().map(|r| r.name.as_str()).collect();
    // The 100-byte exact-match drops; the 5MB suffix-bump survives.
    assert_eq!(names, vec!["quarterly-report-final-summary-v2.pdf"]);
}

#[test]
fn similar_query_strips_extension_for_matching() {
    // The user types `similar:report-final` (no extension); the index
    // should match it against `report-final.pdf` because the
    // SimilarityIndex strips the trailing extension before computing
    // signatures. Without that, the trailing `.pdf` bigrams would
    // shave ~4-5 from the signature and push short-stem matches below
    // the LSH knee.
    let (idx, sim) = paired_indexes(&[create("/synth/quarterly-report-final-summary.pdf")]);
    let q = parse("similar:quarterly-report-final-summary").unwrap();
    let rs = execute_with(&idx, Some(&sim), &q, ExecOpts::default()).unwrap();
    assert_eq!(rs.rows().len(), 1);
    assert_eq!(rs.rows()[0].name, "quarterly-report-final-summary.pdf");
}

#[test]
fn execute_without_similarity_returns_typed_error() {
    let (idx, _sim) = paired_indexes(&[create("/synth/whatever.txt")]);
    let q = parse("similar:whatever").unwrap();
    let err = execute(&idx, &q, ExecOpts::default()).unwrap_err();
    assert!(
        matches!(err, QueryError::SimilarityIndexUnavailable),
        "got {err:?}"
    );
}

#[test]
fn similar_inside_or_is_rejected() {
    let (idx, sim) = paired_indexes(&[create("/synth/whatever.txt")]);
    // `similar:foo OR bar` — Phase 6 only supports top-level Similar.
    let q = parse("similar:foo OR bar").unwrap();
    let err = execute_with(&idx, Some(&sim), &q, ExecOpts::default()).unwrap_err();
    assert!(
        matches!(err, QueryError::UnsupportedSimilarPosition),
        "got {err:?}"
    );
}

#[test]
fn similar_inside_not_is_rejected() {
    let (idx, sim) = paired_indexes(&[create("/synth/whatever.txt")]);
    let q = parse("!similar:foo").unwrap();
    let err = execute_with(&idx, Some(&sim), &q, ExecOpts::default()).unwrap_err();
    assert!(
        matches!(err, QueryError::UnsupportedSimilarPosition),
        "got {err:?}"
    );
}

#[test]
fn empty_similar_value_is_a_parse_error() {
    let err = parse("similar:").unwrap_err();
    let msg = format!("{err}");
    assert!(msg.contains("similar"), "got {msg}");
}

#[test]
fn similar_query_persists_through_simulated_restart() {
    let dir_idx = tempdir().unwrap();
    let dir_sim = tempdir().unwrap();
    let events = vec![
        create("/synth/quarterly-report-final-summary.pdf"),
        create("/synth/quarterly-report-final-summary-v2.pdf"),
        create("/synth/totally-different-name.txt"),
    ];
    {
        let idx = Index::open(dir_idx.path()).unwrap();
        let sim = SimilarityIndex::open(dir_sim.path()).unwrap();
        idx.apply(&events).unwrap();
        idx.commit().unwrap();
        idx.store()
            .iter_all(|row| {
                sim.upsert(row.file_id as u64, &row.name_lower).unwrap();
            })
            .unwrap();
        sim.flush().unwrap();
    }
    // Reopen — the smoke test exercises that minhash.idx round-trips a
    // working similarity index through a process-restart cycle.
    let idx = Index::open(dir_idx.path()).unwrap();
    let sim = SimilarityIndex::open(dir_sim.path()).unwrap();
    assert_eq!(sim.live_row_count(), 3);
    let q = parse("similar:quarterly-report-final-summary").unwrap();
    let rs = execute_with(&idx, Some(&sim), &q, ExecOpts::default()).unwrap();
    let names: Vec<&str> = rs.rows().iter().map(|r| r.name.as_str()).collect();
    assert!(
        names.contains(&"quarterly-report-final-summary.pdf"),
        "exact match missed: {names:?}"
    );
    assert!(
        names.contains(&"quarterly-report-final-summary-v2.pdf"),
        "suffix bump missed: {names:?}"
    );
    assert!(
        !names.contains(&"totally-different-name.txt"),
        "unrelated leaked: {names:?}"
    );
}
