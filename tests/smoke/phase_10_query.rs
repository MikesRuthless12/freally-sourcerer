//! Phase 10 smoke — OS-agnostic, runs on every CI matrix entry.
//!
//! Asserts the invariants the Build Guide names for Phase 10:
//!
//!   1. `parse_to_report` returns a complete `ParseReport` with
//!      per-token spans, an AST summary, and (under
//!      `--strict-everything`) every Freally-only modifier listed
//!      in `errors`.
//!   2. Strict-everything mode rejects audio modifiers, similarity,
//!      and the audio/content/similar lens prefixes via typed
//!      `ParseError::StrictEverythingViolation`. Pure-voidtools
//!      queries keep parsing.
//!   3. The `audio:(lufs:<-14 codec:flac)` lens prefix round-trips
//!      through `parse` → `Lens { kind: Audio, inner: And(...) }`
//!      and through `AstNode::Lens { lens: "audio", inner: And }`
//!      after `parse_to_report`.
//!   4. The optimizer reorders AND children by selectivity and
//!      preserves the match set on a tempdir-backed `Index`.
//!   5. Lens routing — `is_audio_only_route` returns true for a
//!      pure-audio query and the executor still emits the right
//!      hits.
//!   6. `parse_to_report` carries every error in a single pass
//!      (multiple strict-violations surface at once).
//!   7. `content:(...)` lens prefix surfaces
//!      `QueryError::UnsupportedModifier("content")` at execute time.
//!
//! Re-exported under `crates/freally-query/tests/phase_10_query.rs`
//! so the per-crate test runner picks it up alongside the cross-OS
//! smoke harness.

use std::fs::File;
use std::io::Write;
use std::path::{Path, PathBuf};

use freally_index::Index;
use freally_journal::JournalEvent;
use freally_query::{
    AstNode, ErrorCode, ExecOpts, LensKind, ModifierKind, ParseError, ParseOpts, QueryError,
    QueryNode, TokenKind, execute, is_audio_only_route, optimize, parse, parse_to_report,
    parse_with, selectivity_rank,
};

// ---------- (1) parse_to_report basic shape ----------------------------

#[test]
fn parse_to_report_returns_tokens_and_ast() {
    let r = parse_to_report("size:>1mb ext:pdf", ParseOpts::default());
    assert!(r.errors.is_empty());
    assert!(r.ast.is_some());
    assert_eq!(r.tokens.len(), 2);
    match &r.tokens[0].kind {
        TokenKind::Modifier { name } => assert_eq!(name, "size"),
        k => panic!("expected modifier got {k:?}"),
    }
    match &r.tokens[1].kind {
        TokenKind::Modifier { name } => assert_eq!(name, "ext"),
        k => panic!("expected modifier got {k:?}"),
    }
}

#[test]
fn parse_to_report_serializes_to_json_round_trip() {
    let r = parse_to_report(
        "audio:(lufs:<-14 codec:flac) ext:flac",
        ParseOpts::default(),
    );
    let json = serde_json::to_string(&r).expect("serialize");
    assert!(json.contains("audio_lufs"));
    assert!(json.contains("ext"));
    assert!(json.contains("lens"));
}

// ---------- (2) strict-everything mode ---------------------------------

#[test]
fn strict_everything_rejects_audio_modifiers() {
    for q in [
        "lufs:<-14",
        "codec:flac",
        "length:>3:00",
        "rate:48k",
        "silence:>50%",
        "dr:>10",
    ] {
        let err = parse_with(q, ParseOpts::strict()).unwrap_err();
        assert!(
            matches!(err, ParseError::StrictEverythingViolation { .. }),
            "{q} should reject"
        );
    }
}

#[test]
fn strict_everything_rejects_audio_lens_prefix() {
    let err = parse_with("audio:(lufs:<-14)", ParseOpts::strict()).unwrap_err();
    assert!(matches!(err, ParseError::StrictEverythingViolation { .. }));
}

#[test]
fn strict_everything_keeps_voidtools_pure_query() {
    parse_with(
        "size:>1mb date:lastweek ext:pdf attrib:H",
        ParseOpts::strict(),
    )
    .expect("voidtools-pure query parses under strict");
}

// ---------- (3) lens-prefix round-trip ---------------------------------

#[test]
fn audio_lens_prefix_round_trips_through_ast() {
    let q = parse("audio:(lufs:<-14 codec:flac)").unwrap();
    match q.root() {
        QueryNode::Lens { kind, inner } => {
            assert_eq!(*kind, LensKind::Audio);
            match inner.as_ref() {
                QueryNode::And(parts) => {
                    assert_eq!(parts.len(), 2);
                    for p in parts {
                        match p {
                            QueryNode::Modifier(m) => {
                                assert!(matches!(m.kind, ModifierKind::Audio(_)))
                            }
                            n => panic!("non-audio child: {n:?}"),
                        }
                    }
                }
                n => panic!("expected And, got {n:?}"),
            }
        }
        n => panic!("expected Lens, got {n:?}"),
    }
}

#[test]
fn lens_prefix_round_trips_through_ast_node_summary() {
    let r = parse_to_report("audio:(lufs:<-14)", ParseOpts::default());
    let ast = r.ast.expect("ast");
    match ast {
        AstNode::Lens { lens, inner } => {
            assert_eq!(lens, "audio");
            assert!(matches!(*inner, AstNode::Modifier { .. }));
        }
        n => panic!("expected Lens AstNode: {n:?}"),
    }
}

// ---------- (4) optimizer preserves match set --------------------------

fn create(path: &str, size: u64, mtime_ns: i128) -> JournalEvent {
    JournalEvent::Create {
        path: PathBuf::from(path),
        size,
        mtime_ns,
        ctime_ns: mtime_ns,
        attrs: 0,
    }
}

fn small_index() -> std::sync::Arc<Index> {
    let dir = tempfile::tempdir().unwrap();
    let idx = Index::open(dir.path()).unwrap();
    idx.apply(&[
        create("/synth/projects/alpha-report.md", 1_000, 0),
        create("/synth/projects/beta-draft.md", 5_000, 0),
        create("/synth/projects/gamma.txt", 10_000, 0),
        create("/synth/projects/delta.pdf", 20_000_000, 0),
        create("/synth/projects/epsilon.pdf", 100_000, 0),
    ])
    .unwrap();
    idx.commit().unwrap();
    std::mem::forget(dir);
    idx
}

#[test]
fn optimizer_preserves_match_set() {
    let idx = small_index();
    // Pre-optimization order: regex (rank 50), literal (rank 10),
    // size (rank 20). After optimize: literal, size, regex.
    let q = parse("regex:^report report size:<2000").unwrap();
    let pre_rs = execute(&idx, &q, ExecOpts::default()).unwrap();
    let pre_names: std::collections::HashSet<String> =
        pre_rs.rows().iter().map(|r| r.name.clone()).collect();

    let q_opt = optimize(&q);
    let post_rs = execute(&idx, &q_opt, ExecOpts::default()).unwrap();
    let post_names: std::collections::HashSet<String> =
        post_rs.rows().iter().map(|r| r.name.clone()).collect();

    assert_eq!(pre_names, post_names);
}

#[test]
fn optimizer_reorders_audio_after_literal() {
    let q = parse("lufs:<-14 song").unwrap();
    let opt = optimize(&q);
    match opt.root() {
        QueryNode::And(parts) => {
            assert!(matches!(parts[0], QueryNode::Text(_)));
            assert!(matches!(parts[1], QueryNode::Modifier(_)));
        }
        n => panic!("expected And: {n:?}"),
    }
}

#[test]
fn optimizer_does_not_reorder_or_children() {
    let q = parse("alpha OR beta").unwrap();
    let opt = optimize(&q);
    match opt.root() {
        QueryNode::Or(parts) => match (&parts[0], &parts[1]) {
            (
                QueryNode::Text(freally_query::TextPattern::Literal(a)),
                QueryNode::Text(freally_query::TextPattern::Literal(b)),
            ) => {
                assert_eq!(a, "alpha");
                assert_eq!(b, "beta");
            }
            other => panic!("unexpected: {other:?}"),
        },
        n => panic!("expected Or: {n:?}"),
    }
}

// ---------- (5) lens routing -------------------------------------------

#[test]
fn lens_routing_audio_only_detected() {
    let q = parse("lufs:<-14 codec:flac").unwrap();
    assert!(is_audio_only_route(q.root()));
}

#[test]
fn lens_routing_rejects_query_with_name_predicate() {
    let q = parse("song lufs:<-14").unwrap();
    assert!(!is_audio_only_route(q.root()));
}

#[test]
fn lens_routing_through_audio_lens_scope() {
    let q = parse("audio:(lufs:<-14 codec:flac)").unwrap();
    assert!(is_audio_only_route(q.root()));
}

// ---------- (6) parse_to_report multi-error pass -----------------------

#[test]
fn strict_pre_scan_collects_three_violations() {
    let r = parse_to_report("similar:foo lufs:<-14 codec:flac", ParseOpts::strict());
    let violations: Vec<_> = r
        .errors
        .iter()
        .filter(|e| matches!(e.code, ErrorCode::StrictEverythingViolation))
        .collect();
    assert!(
        violations.len() >= 3,
        "expected ≥3 violations, got {}: {:?}",
        violations.len(),
        r.errors
    );
}

// ---------- (7) content lens surfaces typed error ----------------------

#[test]
fn content_lens_prefix_surfaces_unsupported_modifier_at_execute() {
    let idx = small_index();
    let q = parse("content:(machine learning)").unwrap();
    let err = execute(&idx, &q, ExecOpts::default()).unwrap_err();
    match err {
        QueryError::UnsupportedModifier(name) => assert_eq!(name, "content"),
        other => panic!("expected UnsupportedModifier, got {other:?}"),
    }
}

// ---------- bonus: selectivity_rank monotonicity sanity ---------------

#[test]
fn selectivity_rank_orders_cheap_before_expensive() {
    let lit = parse("alpha").unwrap();
    let audio = parse("lufs:<-14").unwrap();
    let regex = parse("regex:^foo").unwrap();
    assert!(selectivity_rank(lit.root()) < selectivity_rank(audio.root()));
    assert!(selectivity_rank(lit.root()) < selectivity_rank(regex.root()));
}

// ---------- bonus: synthetic WAV smoke for lens-prefix end-to-end -----

fn write_silent_wav(path: &Path, sr: u32, ch: u16, secs: f32) -> std::io::Result<()> {
    let mut f = File::create(path)?;
    let bits_per_sample: u16 = 16;
    let frames = (sr as f32 * secs) as u32;
    let data_size = frames * ch as u32 * 2;
    let chunk_size: u32 = 36 + data_size;
    let byte_rate = sr * ch as u32 * 2;
    let block_align: u16 = ch * 2;
    f.write_all(b"RIFF")?;
    f.write_all(&chunk_size.to_le_bytes())?;
    f.write_all(b"WAVE")?;
    f.write_all(b"fmt ")?;
    f.write_all(&16u32.to_le_bytes())?;
    f.write_all(&1u16.to_le_bytes())?;
    f.write_all(&ch.to_le_bytes())?;
    f.write_all(&sr.to_le_bytes())?;
    f.write_all(&byte_rate.to_le_bytes())?;
    f.write_all(&block_align.to_le_bytes())?;
    f.write_all(&bits_per_sample.to_le_bytes())?;
    f.write_all(b"data")?;
    f.write_all(&data_size.to_le_bytes())?;
    let zeros = vec![0u8; data_size as usize];
    f.write_all(&zeros)?;
    Ok(())
}

#[test]
fn audio_lens_prefix_filters_via_provider() {
    use std::sync::Arc;
    let dir = tempfile::tempdir().unwrap();
    let media = dir.path().join("media");
    std::fs::create_dir_all(&media).unwrap();
    let song = media.join("song.wav");
    write_silent_wav(&song, 44_100, 2, 1.0).unwrap();

    let idx_dir = dir.path().join("index");
    let idx = Index::open(&idx_dir).unwrap();
    let mtime: i128 = 1_700_000_000_000_000_000_i128;
    idx.apply(&[JournalEvent::Create {
        path: song.clone(),
        size: std::fs::metadata(&song).map(|m| m.len()).unwrap_or(0),
        mtime_ns: mtime,
        ctime_ns: mtime,
        attrs: 0,
    }])
    .unwrap();
    idx.commit().unwrap();

    let cache: Arc<freally_audio::AudioCache> =
        Arc::new(freally_audio::AudioCache::open(dir.path().join("c.json")).unwrap());
    let provider: &dyn freally_audio::AudioAttributesProvider = cache.as_ref();

    // `audio:(silence:>0.99)` — lens prefix wraps a silence
    // predicate. The synthetic silent WAV should match.
    let q = parse("audio:(silence:>0.99)").unwrap();
    let result = freally_query::execute_with_audio(
        &idx,
        None,
        Some(provider),
        &q,
        ExecOpts {
            limit: 100,
            ..ExecOpts::default()
        },
    )
    .unwrap();
    assert_eq!(result.rows().len(), 1, "expected the silent file to match");
}
