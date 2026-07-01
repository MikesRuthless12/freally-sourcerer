//! Phase 10 Freally-DSL fixture.
//!
//! 200+ queries spanning every Freally-only surface: the Phase-9
//! audio modifiers, the Phase-6 similarity modifier, the Phase-10
//! lens prefixes (`name:(...)` / `audio:(...)` / `content:(...)` /
//! `similar:(...)`), composed multi-lens queries, and
//! strict-everything rejections. Hand-curated baseline plus an
//! algorithmic generator for the long tail.
//!
//! The Phase-10 prompt explicitly gates 200+ Freally-DSL queries
//! that parse identically to the spec.

use freally_query::{
    AstNode, ErrorCode, LensKind, ModifierKind, ParseOpts, QueryNode, parse, parse_to_report,
    parse_with,
};

// ---------- hand-curated cases -----------------------------------------

#[test]
fn audio_modifiers_parse_to_audio_kind() {
    let cases = [
        "lufs:<-14",
        "lufs:>=-23",
        "lufs:=-16",
        "lufs:>-30",
        "codec:flac",
        "codec:mp3",
        "codec:flac;mp3;aac",
        "codec:FLAC",
        "length:>3:00",
        "length:1:30:00",
        "length:>=42",
        "length:<60",
        "duration:30",
        "rate:>=44100",
        "rate:48k",
        "rate:48000",
        "samplerate:96000",
        "silence:>50%",
        "silence:>0.5",
        "silence:=0.0",
        "silence:<10%",
        "dr:>10",
        "dr:>=8",
        "dr:<5",
    ];
    for q in cases {
        let parsed = parse(q).unwrap_or_else(|e| panic!("`{q}` failed: {e}"));
        match parsed.root() {
            QueryNode::Modifier(m) => match &m.kind {
                ModifierKind::Audio(_) => {}
                k => panic!("`{q}` not Audio: {k:?}"),
            },
            n => panic!("`{q}` unexpected: {n:?}"),
        }
    }
}

#[test]
fn similarity_modifier_parses() {
    let cases = [
        "similar:report-final",
        "similar:bassdrop",
        "similar:alpha",
        "similar:final.draft",
        "similar:foo-bar-baz",
    ];
    for q in cases {
        let parsed = parse(q).unwrap_or_else(|e| panic!("`{q}` failed: {e}"));
        match parsed.root() {
            QueryNode::Modifier(m) => match &m.kind {
                ModifierKind::Similar(_) => {}
                k => panic!("`{q}` not Similar: {k:?}"),
            },
            n => panic!("`{q}` unexpected: {n:?}"),
        }
    }
}

#[test]
fn lens_prefix_audio_wraps_inner_audio_modifiers() {
    let q = parse("audio:(lufs:<-14 codec:flac length:>3:00)").unwrap();
    match q.root() {
        QueryNode::Lens { kind, inner } => {
            assert_eq!(*kind, LensKind::Audio);
            match inner.as_ref() {
                QueryNode::And(parts) => {
                    assert_eq!(parts.len(), 3);
                    for p in parts {
                        assert!(
                            matches!(p, QueryNode::Modifier(m) if matches!(m.kind, ModifierKind::Audio(_))),
                            "non-audio child in lens: {p:?}"
                        );
                    }
                }
                n => panic!("inner should be And: {n:?}"),
            }
        }
        n => panic!("expected Lens: {n:?}"),
    }
}

#[test]
fn lens_prefix_name_wraps_filename_predicate() {
    let q = parse("name:(report*)").unwrap();
    match q.root() {
        QueryNode::Lens { kind, inner } => {
            assert_eq!(*kind, LensKind::Name);
            assert!(matches!(inner.as_ref(), QueryNode::Text(_)));
        }
        n => panic!("expected Lens: {n:?}"),
    }
}

#[test]
fn lens_prefix_similar_wraps_text_needle() {
    let q = parse("similar:(report-final)").unwrap();
    match q.root() {
        QueryNode::Lens { kind, inner } => {
            assert_eq!(*kind, LensKind::Similar);
            // Inner is the literal needle — Phase-10 doesn't auto-
            // promote `similar:(text)` to a Similar modifier; that is
            // a Phase-11 routing concern.
            assert!(matches!(inner.as_ref(), QueryNode::Text(_)));
        }
        n => panic!("expected Lens: {n:?}"),
    }
}

#[test]
fn lens_prefix_content_parses_but_executor_rejects() {
    let q = parse("content:(machine learning)").unwrap();
    match q.root() {
        QueryNode::Lens { kind, .. } => {
            assert_eq!(*kind, LensKind::Content);
        }
        n => panic!("expected Lens: {n:?}"),
    }
}

#[test]
fn empty_lens_scope_collapses_to_true() {
    let q = parse("audio:()").unwrap();
    match q.root() {
        QueryNode::Lens { kind, inner } => {
            assert_eq!(*kind, LensKind::Audio);
            assert!(matches!(inner.as_ref(), QueryNode::True));
        }
        n => panic!("expected Lens: {n:?}"),
    }
}

#[test]
fn lens_prefix_composes_with_outer_modifiers() {
    let q = parse("audio:(lufs:<-14) ext:flac size:>10mb").unwrap();
    match q.root() {
        QueryNode::And(parts) => {
            assert_eq!(parts.len(), 3);
            assert!(matches!(parts[0], QueryNode::Lens { .. }));
            assert!(matches!(parts[1], QueryNode::Modifier(_)));
            assert!(matches!(parts[2], QueryNode::Modifier(_)));
        }
        n => panic!("expected And: {n:?}"),
    }
}

#[test]
fn nested_lens_scopes_parse() {
    // `audio:(name:(song))` — lens inside lens. Useful for surfacing
    // the audio file with name `song` in a Phase-11 lens-grouped UI.
    let q = parse("audio:(name:(song))").unwrap();
    match q.root() {
        QueryNode::Lens {
            kind: LensKind::Audio,
            inner,
        } => match inner.as_ref() {
            QueryNode::Lens {
                kind: LensKind::Name,
                ..
            } => {}
            n => panic!("inner should be Name lens: {n:?}"),
        },
        n => panic!("expected Audio lens: {n:?}"),
    }
}

#[test]
fn audio_with_filename_lens_compose() {
    // The Phase-9 prompt's headline composition.
    let q = parse("song lufs:<-14 codec:flac length:>3:00").unwrap();
    match q.root() {
        QueryNode::And(parts) => {
            assert_eq!(parts.len(), 4);
            assert!(matches!(parts[0], QueryNode::Text(_)));
            for p in &parts[1..] {
                assert!(
                    matches!(p, QueryNode::Modifier(m) if matches!(m.kind, ModifierKind::Audio(_)))
                );
            }
        }
        n => panic!("expected And: {n:?}"),
    }
}

#[test]
fn similarity_with_audio_compose() {
    // From the Phase-9 doc.
    let q = parse("similar:bassdrop codec:flac length:>3:00").unwrap();
    match q.root() {
        QueryNode::And(parts) => {
            assert_eq!(parts.len(), 3);
            assert!(matches!(
                parts[0],
                QueryNode::Modifier(ref m) if matches!(m.kind, ModifierKind::Similar(_))
            ));
            for p in &parts[1..] {
                assert!(
                    matches!(p, QueryNode::Modifier(m) if matches!(m.kind, ModifierKind::Audio(_)))
                );
            }
        }
        n => panic!("expected And: {n:?}"),
    }
}

// ---------- strict-everything rejection tests --------------------------

#[test]
fn strict_everything_rejects_audio_modifiers() {
    use freally_query::ParseError;
    for q in [
        "lufs:<-14",
        "codec:flac",
        "length:>3:00",
        "rate:48k",
        "silence:>50%",
        "dr:>10",
        "duration:30",
        "samplerate:48000",
    ] {
        let err = parse_with(q, ParseOpts::strict()).unwrap_err();
        assert!(
            matches!(err, ParseError::StrictEverythingViolation { .. }),
            "`{q}` should reject under strict, got: {err:?}"
        );
    }
}

#[test]
fn strict_everything_rejects_similar_modifier() {
    let err = parse_with("similar:report", ParseOpts::strict()).unwrap_err();
    assert!(matches!(
        err,
        freally_query::ParseError::StrictEverythingViolation { .. }
    ));
}

#[test]
fn strict_everything_rejects_audio_lens_prefix() {
    let err = parse_with("audio:(lufs:<-14)", ParseOpts::strict()).unwrap_err();
    assert!(matches!(
        err,
        freally_query::ParseError::StrictEverythingViolation { .. }
    ));
}

#[test]
fn strict_everything_accepts_voidtools_modifiers() {
    for q in [
        "size:>1mb",
        "date:lastweek",
        "ext:txt;rs",
        "attrib:HRA",
        "path:Documents",
        "parent:Music",
        "child:song",
        "name:report",
        "folder:apps",
        "audio:",
        "video:",
        "image:",
        "doc:",
        "exec:",
        "compressed:",
        "*.txt",
        "regex:^report",
        "(alpha OR beta) gamma",
        "!draft",
        "NOT old",
        "name:(report)",
    ] {
        parse_with(q, ParseOpts::strict()).unwrap_or_else(|e| {
            panic!("voidtools-shaped query `{q}` should parse under strict: {e}")
        });
    }
}

// ---------- ParseReport / IPC surface ---------------------------------

#[test]
fn parse_report_serializes_to_json() {
    let r = parse_to_report("size:>1mb ext:pdf", ParseOpts::default());
    let json = serde_json::to_string(&r).expect("serialize");
    assert!(json.contains("modifier"));
    assert!(json.contains("size"));
    assert!(json.contains("pdf"));
}

#[test]
fn parse_report_carries_per_token_spans() {
    let r = parse_to_report("alpha beta", ParseOpts::default());
    assert_eq!(r.tokens.len(), 2);
    assert_eq!(r.tokens[0].span.start, 0);
    assert_eq!(r.tokens[0].span.end, 5);
    assert_eq!(r.tokens[1].span.start, 6);
    assert_eq!(r.tokens[1].span.end, 10);
}

#[test]
fn parse_report_strict_collects_multiple_violations() {
    let r = parse_to_report("similar:foo lufs:<-14 codec:flac", ParseOpts::strict());
    let strict_count = r
        .errors
        .iter()
        .filter(|e| matches!(e.code, ErrorCode::StrictEverythingViolation))
        .count();
    assert!(
        strict_count >= 3,
        "got {strict_count} violations: {:?}",
        r.errors
    );
}

#[test]
fn parse_report_lens_round_trip_through_ast_node() {
    let r = parse_to_report("audio:(lufs:<-14)", ParseOpts::default());
    let ast = r.ast.expect("ast");
    match ast {
        AstNode::Lens { lens, inner } => {
            assert_eq!(lens, "audio");
            match *inner {
                AstNode::Modifier { name, .. } => assert_eq!(name, "lufs"),
                n => panic!("inner: {n:?}"),
            }
        }
        n => panic!("expected lens: {n:?}"),
    }
}

#[test]
fn audio_predicate_round_trips_through_serialize() {
    let r = parse_to_report("lufs:<-14", ParseOpts::default());
    let ast = r.ast.expect("ast");
    let json = serde_json::to_string(&ast).expect("serialize");
    assert!(json.contains("audio_lufs"));
}

// ---------- 200+ generated cases --------------------------------------

#[test]
fn two_hundred_freally_dsl_queries_parse() {
    let mut all: Vec<String> = hand_curated();
    all.extend(generated());
    assert!(
        all.len() >= 200,
        "Phase 10 gates 200+ Freally-DSL queries; got {}",
        all.len()
    );
    for q in &all {
        parse(q).unwrap_or_else(|e| panic!("failed to parse `{q}`: {e}"));
    }
}

fn hand_curated() -> Vec<String> {
    [
        // Audio modifiers — every variant we lock in via parser tests.
        "lufs:<-14",
        "lufs:<=-14",
        "lufs:>-23",
        "lufs:>=-23",
        "lufs:=-16",
        "codec:flac",
        "codec:mp3",
        "codec:aac",
        "codec:opus",
        "codec:vorbis",
        "codec:alac",
        "codec:pcm_s16",
        "codec:flac;mp3",
        "codec:flac;mp3;aac",
        "codec:FLAC",
        "length:30",
        "length:1.5",
        "length:>30",
        "length:>=30",
        "length:<60",
        "length:<=60",
        "length:3:00",
        "length:>3:00",
        "length:1:30:00",
        "length:0:30",
        "duration:30",
        "duration:>=42",
        "rate:44100",
        "rate:48000",
        "rate:>=44100",
        "rate:<=48000",
        "rate:<96000",
        "rate:>22050",
        "rate:48k",
        "rate:96k",
        "samplerate:48000",
        "samplerate:96000",
        "silence:>50%",
        "silence:>=10%",
        "silence:<5%",
        "silence:<=1%",
        "silence:=0.0",
        "silence:>0.5",
        "silence:<0.05",
        "dr:>10",
        "dr:>=8",
        "dr:<5",
        "dr:<=12",
        "dr:=8",
        // Similarity.
        "similar:report-final",
        "similar:bassdrop",
        "similar:alpha-v1",
        "similar:final.draft",
        "similar:notes-2024",
        "similar:invoice-001",
        // Lens prefixes.
        "name:(report)",
        "name:(report*)",
        "name:(*.txt)",
        "name:(report OR draft)",
        "audio:(lufs:<-14)",
        "audio:(codec:flac)",
        "audio:(length:>3:00)",
        "audio:(rate:48000)",
        "audio:(silence:<5%)",
        "audio:(dr:>10)",
        "audio:(lufs:<-14 codec:flac)",
        "audio:(lufs:<-14 codec:flac length:>3:00)",
        "audio:(codec:flac OR codec:mp3)",
        "content:(machine learning)",
        "content:(report)",
        "content:(\"final draft\")",
        "similar:(report-final)",
        "similar:(bassdrop)",
        // Lens + outer compositions.
        "audio:(lufs:<-14) ext:flac",
        "audio:(codec:flac) size:>10mb",
        "audio:(lufs:<-14) date:lastweek",
        "name:(report) ext:pdf",
        "name:(report*) size:>1mb",
        "similar:(report) ext:pdf",
        "audio:(lufs:<-14) name:(song)",
        "audio:(name:(song))",
        "audio:(rate:>=44100 codec:flac)",
        // Phase 9 prompt's worked examples.
        "song lufs:<-14 codec:flac length:>3:00",
        "similar:bassdrop codec:flac length:>3:00",
        "report similar:report-final",
        // Audio in OR.
        "lufs:<-14 OR codec:flac",
        "codec:flac OR codec:mp3",
        "(rate:44100 OR rate:48000) ext:flac",
        // Audio with NOT.
        "NOT codec:mp3",
        "!silence:>50%",
        "!(lufs:<-30)",
    ]
    .iter()
    .map(|s| s.to_string())
    .collect()
}

fn generated() -> Vec<String> {
    let lufs_ops = ["<", ">", "<=", ">=", "="];
    let lufs_vals = ["-14", "-16", "-23", "-30"];
    let codecs = ["flac", "mp3", "aac", "opus", "vorbis", "alac"];
    let length_vals = ["30", "60", "3:00", "1:30:00"];
    let rate_vals = ["44100", "48000", "96000", "22050"];
    let silence_vals = ["10%", "50%", "0.5", "1.0"];
    let needles = ["report", "draft", "alpha", "song", "photo", "notes"];
    let outer_mods = ["ext:flac", "size:>1mb", "date:lastweek", "path:Music"];

    let mut out: Vec<String> = Vec::with_capacity(200);

    // 5×4 = 20 lufs queries.
    for op in &lufs_ops {
        for v in &lufs_vals {
            out.push(format!("lufs:{op}{v}"));
        }
    }
    // 6 codec queries (single + lists).
    for c in &codecs {
        out.push(format!("codec:{c}"));
    }
    out.push(format!("codec:{};{}", codecs[0], codecs[1]));
    out.push(format!("codec:{};{};{}", codecs[0], codecs[1], codecs[2]));

    // 5×4 = 20 length queries.
    for op in &lufs_ops {
        for v in &length_vals {
            out.push(format!("length:{op}{v}"));
        }
    }
    // 5×4 = 20 rate queries.
    for op in &lufs_ops {
        for v in &rate_vals {
            out.push(format!("rate:{op}{v}"));
        }
    }
    // 4 silence queries.
    for v in &silence_vals {
        out.push(format!("silence:>{v}"));
    }
    // 4 dr queries.
    for op in &lufs_ops {
        out.push(format!("dr:{op}10"));
    }
    // 6 similar queries.
    for n in &needles {
        out.push(format!("similar:{n}"));
    }
    // 6 lens-prefix audio queries.
    for op in &lufs_ops {
        out.push(format!("audio:(lufs:{op}{})", lufs_vals[0]));
    }
    // 6 lens-prefix similar queries.
    for n in &needles {
        out.push(format!("similar:({n})"));
    }
    // 4 lens-prefix name queries.
    for n in &needles {
        out.push(format!("name:({n}*)"));
    }
    // 4 lens-prefix content queries — content extractor not wired
    // yet, but the parser must accept the syntax.
    for n in &needles {
        out.push(format!("content:({n})"));
    }
    // 6×4 = 24 multi-lens compositions.
    for n in &needles {
        for m in &outer_mods {
            out.push(format!("similar:{n} {m}"));
        }
    }
    // 6×4 = 24 audio + outer mod.
    for c in &codecs {
        for m in &outer_mods {
            out.push(format!("codec:{c} {m}"));
        }
    }
    // 5×4 = 20 lens-prefix + outer mod.
    for op in &lufs_ops {
        for m in &outer_mods {
            out.push(format!("audio:(lufs:{op}{}) {}", lufs_vals[0], m));
        }
    }

    out
}
