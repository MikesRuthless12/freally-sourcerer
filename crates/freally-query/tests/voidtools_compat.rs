//! Voidtools-syntax compatibility fixture (Build Guide Phase 5).
//!
//! 50 real Everything queries — every one must parse without error
//! and the resulting AST must classify the way the spec says.
//! Standing Rule #8: every query that parses today must keep parsing
//! through Phase 14. This file is the regression gate.
//!
//! Sources for the queries: voidtools' Everything user guide examples,
//! their search-syntax reference page, and the most-asked questions
//! on their support forum (the cases users *expect* to work).

use freally_query::QuickFilter as Qf;
use freally_query::{
    AttribFlag, DateBound, ModifierKind, ParseError, QueryNode, RelativeDate, SizeOp, TextPattern,
    parse,
};

#[derive(Debug, Clone)]
struct Case {
    /// Source query.
    q: &'static str,
    /// Expected shape — checked against the parsed root.
    shape: Shape,
}

#[derive(Debug, Clone)]
enum Shape {
    Literal(&'static str),
    Wildcard(&'static str),
    Regex,
    QuickFilter(Qf),
    AndOf(usize),
    OrOf(usize),
    Not,
    SizeGt(u64),
    SizeLt(u64),
    SizeEq(u64),
    DateRel(RelativeDate),
    DateDay,
    Ext(Vec<&'static str>),
    Attrib(Vec<AttribFlag>),
    Path(&'static str),
    Parent(&'static str),
    Child(&'static str),
    True,
}

fn cases() -> Vec<Case> {
    use Shape::*;
    vec![
        // 1-10: bare terms / wildcards / regex
        Case {
            q: "report",
            shape: Literal("report"),
        },
        Case {
            q: "REPORT",
            shape: Literal("REPORT"),
        },
        Case {
            q: "Final Report",
            shape: AndOf(2),
        },
        Case {
            q: "\"final report\"",
            shape: Literal("final report"),
        },
        Case {
            q: "*.txt",
            shape: Wildcard("*.txt"),
        },
        Case {
            q: "report?.doc",
            shape: Wildcard("report?.doc"),
        },
        Case {
            q: "regex:^report-\\d+$",
            shape: Regex,
        },
        Case {
            q: "(report)",
            shape: Literal("report"),
        },
        Case {
            q: "()",
            shape: True,
        },
        Case {
            q: "draft AND \"q4 plan\"",
            shape: AndOf(2),
        },
        // 11-20: boolean glue
        Case {
            q: "alpha OR beta",
            shape: OrOf(2),
        },
        Case {
            q: "alpha OR beta OR gamma",
            shape: OrOf(3),
        },
        Case {
            q: "alpha beta OR gamma",
            shape: OrOf(2),
        },
        Case {
            q: "!draft",
            shape: Not,
        },
        Case {
            q: "NOT draft",
            shape: Not,
        },
        Case {
            q: "report !old",
            shape: AndOf(2),
        },
        Case {
            q: "(a OR b) c",
            shape: AndOf(2),
        },
        Case {
            q: "(a OR b) AND c",
            shape: AndOf(2),
        },
        Case {
            q: "a OR (b c)",
            shape: OrOf(2),
        },
        Case {
            q: "!(a b)",
            shape: Not,
        },
        // 21-30: size + date modifiers
        Case {
            q: "size:>100mb",
            shape: SizeGt(100 * 1024 * 1024),
        },
        Case {
            q: "size:<1gb",
            shape: SizeLt(1024 * 1024 * 1024),
        },
        Case {
            q: "size:42",
            shape: SizeEq(42),
        },
        Case {
            q: "size:>1.5mb",
            shape: SizeGt((1.5 * 1024.0 * 1024.0) as u64),
        },
        Case {
            q: "date:today",
            shape: DateRel(RelativeDate::Today),
        },
        Case {
            q: "date:lastweek",
            shape: DateRel(RelativeDate::LastWeek),
        },
        Case {
            q: "date:thisweek",
            shape: DateRel(RelativeDate::ThisWeek),
        },
        Case {
            q: "date:thismonth",
            shape: DateRel(RelativeDate::ThisMonth),
        },
        Case {
            q: "date:>2024-01-01",
            shape: DateDay,
        },
        Case {
            q: "date:<=2024-12-31",
            shape: DateDay,
        },
        // 31-40: ext + attrib + path family modifiers
        Case {
            q: "ext:txt",
            shape: Ext(vec!["txt"]),
        },
        Case {
            q: "ext:txt;rs;py",
            shape: Ext(vec!["txt", "rs", "py"]),
        },
        Case {
            q: "ext:.md",
            shape: Ext(vec!["md"]),
        },
        Case {
            q: "attrib:H",
            shape: Attrib(vec![AttribFlag::Hidden]),
        },
        Case {
            q: "attrib:HRA",
            shape: Attrib(vec![
                AttribFlag::Hidden,
                AttribFlag::ReadOnly,
                AttribFlag::Archive,
            ]),
        },
        Case {
            q: "path:Documents",
            shape: Path("Documents"),
        },
        Case {
            q: "parent:Projects",
            shape: Parent("Projects"),
        },
        Case {
            q: "child:src",
            shape: Child("src"),
        },
        Case {
            q: "name:report",
            shape: Child("report"),
        },
        Case {
            q: "folder:apps",
            shape: Parent("apps"),
        },
        // 41-50: quick filters and combinations
        Case {
            q: "audio:",
            shape: QuickFilter(Qf::Audio),
        },
        Case {
            q: "video:",
            shape: QuickFilter(Qf::Video),
        },
        Case {
            q: "image:",
            shape: QuickFilter(Qf::Image),
        },
        Case {
            q: "document:",
            shape: QuickFilter(Qf::Document),
        },
        Case {
            q: "executable:",
            shape: QuickFilter(Qf::Executable),
        },
        Case {
            q: "archive:",
            shape: QuickFilter(Qf::Archive),
        },
        Case {
            q: "audio: size:>50mb",
            shape: AndOf(2),
        },
        Case {
            q: "draft size:<10mb date:lastweek",
            shape: AndOf(3),
        },
        Case {
            q: "report ext:pdf",
            shape: AndOf(2),
        },
        Case {
            q: "(report OR draft) ext:pdf size:>1mb",
            shape: AndOf(3),
        },
        // 51-60: extra wildcard / regex shapes
        Case {
            q: "draft*",
            shape: Wildcard("draft*"),
        },
        Case {
            q: "*draft",
            shape: Wildcard("*draft"),
        },
        Case {
            q: "*draft*",
            shape: Wildcard("*draft*"),
        },
        Case {
            q: "report?",
            shape: Wildcard("report?"),
        },
        Case {
            q: "?eport",
            shape: Wildcard("?eport"),
        },
        Case {
            q: "*.{txt,md}",
            shape: Wildcard("*.{txt,md}"),
        },
        Case {
            q: "regex:^[A-Z]\\w+$",
            shape: Regex,
        },
        Case {
            q: "regex:\\d{4}-\\d{2}-\\d{2}",
            shape: Regex,
        },
        Case {
            q: "regex:.*\\.tar\\.gz$",
            shape: Regex,
        },
        Case {
            q: "regex:^report-final",
            shape: Regex,
        },
        // 61-70: extra size shapes (units + comparators)
        Case {
            q: "size:50kb",
            shape: SizeEq(50 * 1024),
        },
        Case {
            q: "size:>5gb",
            shape: SizeGt(5u64 * 1024 * 1024 * 1024),
        },
        Case {
            q: "size:<10b",
            shape: SizeLt(10),
        },
        Case {
            q: "size:1tb",
            shape: SizeEq(1024u64 * 1024 * 1024 * 1024),
        },
        Case {
            q: "size:0.5gb",
            shape: SizeEq((0.5 * 1024.0 * 1024.0 * 1024.0) as u64),
        },
        Case {
            q: "size:>0",
            shape: SizeGt(0),
        },
        Case {
            q: "size:=42b",
            shape: SizeEq(42),
        },
        Case {
            q: "size:>1024",
            shape: SizeGt(1024),
        },
        Case {
            q: "size:<2048",
            shape: SizeLt(2048),
        },
        // 71-80: extra date shapes
        Case {
            q: "date:yesterday",
            shape: DateRel(RelativeDate::Yesterday),
        },
        Case {
            q: "date:lastmonth",
            shape: DateRel(RelativeDate::LastMonth),
        },
        Case {
            q: "date:thisyear",
            shape: DateRel(RelativeDate::ThisYear),
        },
        Case {
            q: "date:lastyear",
            shape: DateRel(RelativeDate::LastYear),
        },
        Case {
            q: "date:>=2024-06-01",
            shape: DateDay,
        },
        Case {
            q: "date:<2025-01-01",
            shape: DateDay,
        },
        Case {
            q: "date:=2024-03-04",
            shape: DateDay,
        },
        Case {
            q: "dm:lastweek",
            shape: DateRel(RelativeDate::LastWeek),
        },
        Case {
            q: "dc:today",
            shape: DateRel(RelativeDate::Today),
        },
        Case {
            q: "da:>2024-12-25",
            shape: DateDay,
        },
        // 81-90: ext + attrib variants
        Case {
            q: "ext:rs",
            shape: Ext(vec!["rs"]),
        },
        Case {
            q: "ext:.json",
            shape: Ext(vec!["json"]),
        },
        Case {
            q: "ext:c;cpp;h;hpp",
            shape: Ext(vec!["c", "cpp", "h", "hpp"]),
        },
        Case {
            q: "ext:JPG;PNG;GIF",
            shape: Ext(vec!["jpg", "png", "gif"]),
        },
        Case {
            q: "attr:H",
            shape: Attrib(vec![AttribFlag::Hidden]),
        },
        Case {
            q: "attributes:R",
            shape: Attrib(vec![AttribFlag::ReadOnly]),
        },
        Case {
            q: "attrib:HSC",
            shape: Attrib(vec![
                AttribFlag::Hidden,
                AttribFlag::System,
                AttribFlag::Compressed,
            ]),
        },
        Case {
            q: "attrib:DLE",
            shape: Attrib(vec![
                AttribFlag::Directory,
                AttribFlag::Reparse,
                AttribFlag::Encrypted,
            ]),
        },
        Case {
            q: "attrib:T",
            shape: Attrib(vec![AttribFlag::Temporary]),
        },
        Case {
            q: "attrib:O",
            shape: Attrib(vec![AttribFlag::Offline]),
        },
        // 91-100: path / parent / child + aliases
        Case {
            q: "path:Documents",
            shape: Path("Documents"),
        },
        Case {
            q: "path:src/lib",
            shape: Path("src/lib"),
        },
        Case {
            q: "parent:tests",
            shape: Parent("tests"),
        },
        Case {
            q: "folder:apps",
            shape: Parent("apps"),
        },
        Case {
            q: "child:main",
            shape: Child("main"),
        },
        Case {
            q: "name:report",
            shape: Child("report"),
        },
        Case {
            q: "path:\"Program Files\"",
            shape: AndOf(2),
        },
        Case {
            q: "parent:Music child:song",
            shape: AndOf(2),
        },
        Case {
            q: "path:Photos ext:jpg",
            shape: AndOf(2),
        },
        Case {
            q: "name:report ext:pdf size:>1mb",
            shape: AndOf(3),
        },
        // 101-110: quick-filter aliases + compositions
        Case {
            q: "doc:",
            shape: QuickFilter(Qf::Document),
        },
        Case {
            q: "docs:",
            shape: QuickFilter(Qf::Document),
        },
        Case {
            q: "pic:",
            shape: QuickFilter(Qf::Image),
        },
        Case {
            q: "picture:",
            shape: QuickFilter(Qf::Image),
        },
        Case {
            q: "pics:",
            shape: QuickFilter(Qf::Image),
        },
        Case {
            q: "exec:",
            shape: QuickFilter(Qf::Executable),
        },
        Case {
            q: "compressed:",
            shape: QuickFilter(Qf::Archive),
        },
        Case {
            q: "video: size:>500mb",
            shape: AndOf(2),
        },
        Case {
            q: "image: ext:png",
            shape: AndOf(2),
        },
        Case {
            q: "exe: size:<1mb",
            shape: AndOf(2),
        },
        // 111-120: complex boolean compositions
        Case {
            q: "(alpha AND beta) OR (gamma AND delta)",
            shape: OrOf(2),
        },
        Case {
            q: "report AND NOT draft",
            shape: AndOf(2),
        },
        Case {
            q: "(report OR draft) NOT old",
            shape: AndOf(2),
        },
        Case {
            q: "alpha !beta gamma",
            shape: AndOf(3),
        },
        Case {
            q: "!(a OR b)",
            shape: Not,
        },
        Case {
            q: "((alpha))",
            shape: Literal("alpha"),
        },
        Case {
            q: "alpha (beta OR gamma) delta",
            shape: AndOf(3),
        },
        Case {
            q: "size:>1mb size:<10mb",
            shape: AndOf(2),
        },
        Case {
            q: "(size:>1mb) AND (date:lastweek)",
            shape: AndOf(2),
        },
        Case {
            q: "regex:^[a-z]+$ ext:txt",
            shape: AndOf(2),
        },
    ]
}

fn assert_shape(node: &QueryNode, shape: &Shape) {
    use Shape::*;
    match (node, shape) {
        (QueryNode::Text(TextPattern::Literal(actual)), Literal(want)) => {
            assert_eq!(actual, want, "literal mismatch");
        }
        (QueryNode::Text(TextPattern::Wildcard { raw, .. }), Wildcard(want)) => {
            assert_eq!(raw, want, "wildcard mismatch");
        }
        (QueryNode::Text(TextPattern::Regex { .. }), Regex) => {}
        (QueryNode::QuickFilter(actual), Shape::QuickFilter(want)) => {
            assert_eq!(actual, want, "quick filter mismatch");
        }
        (QueryNode::And(parts), AndOf(n)) => {
            assert_eq!(parts.len(), *n, "AND arity mismatch");
        }
        (QueryNode::Or(parts), OrOf(n)) => {
            assert_eq!(parts.len(), *n, "OR arity mismatch");
        }
        (QueryNode::Not(_), Not) => {}
        (QueryNode::True, True) => {}
        (QueryNode::Modifier(m), SizeGt(b)) => match &m.kind {
            ModifierKind::Size {
                op: SizeOp::Gt,
                bytes,
            } => assert_eq!(bytes, b),
            k => panic!("expected size:>; got {k:?}"),
        },
        (QueryNode::Modifier(m), SizeLt(b)) => match &m.kind {
            ModifierKind::Size {
                op: SizeOp::Lt,
                bytes,
            } => assert_eq!(bytes, b),
            k => panic!("expected size:<; got {k:?}"),
        },
        (QueryNode::Modifier(m), SizeEq(b)) => match &m.kind {
            ModifierKind::Size {
                op: SizeOp::Eq,
                bytes,
            } => assert_eq!(bytes, b),
            k => panic!("expected size:=; got {k:?}"),
        },
        (QueryNode::Modifier(m), DateRel(want)) => match &m.kind {
            ModifierKind::Date(DateBound::Relative(rd)) => assert_eq!(rd, want),
            k => panic!("expected date:relative; got {k:?}"),
        },
        (QueryNode::Modifier(m), DateDay) => match &m.kind {
            ModifierKind::Date(DateBound::Day { .. }) => {}
            k => panic!("expected date:day; got {k:?}"),
        },
        (QueryNode::Modifier(m), Ext(want)) => match &m.kind {
            ModifierKind::Ext(actual) => {
                let actual: Vec<&str> = actual.iter().map(String::as_str).collect();
                assert_eq!(actual, *want);
            }
            k => panic!("expected ext:; got {k:?}"),
        },
        (QueryNode::Modifier(m), Attrib(want)) => match &m.kind {
            ModifierKind::Attrib(actual) => {
                for f in want {
                    assert!(actual.contains(f), "missing attrib flag {f:?}");
                }
            }
            k => panic!("expected attrib:; got {k:?}"),
        },
        (QueryNode::Modifier(m), Path(want)) => match &m.kind {
            ModifierKind::Path(actual) => assert_eq!(actual, want),
            k => panic!("expected path:; got {k:?}"),
        },
        (QueryNode::Modifier(m), Parent(want)) => match &m.kind {
            ModifierKind::Parent(actual) => assert_eq!(actual, want),
            k => panic!("expected parent:; got {k:?}"),
        },
        (QueryNode::Modifier(m), Child(want)) => match &m.kind {
            ModifierKind::Child(actual) => assert_eq!(actual, want),
            k => panic!("expected child:; got {k:?}"),
        },
        (n, s) => panic!("shape mismatch — got {n:?} expected {s:?}"),
    }
}

#[test]
fn fifty_voidtools_queries_parse() {
    let cs = cases();
    assert!(
        cs.len() >= 50,
        "Build Guide gates 50+ queries: got {}",
        cs.len()
    );
    for case in cs {
        let q = parse(case.q).unwrap_or_else(|e| panic!("failed `{}`: {e}", case.q));
        assert_shape(q.root(), &case.shape);
    }
}

/// Phase 10 lifted the voidtools-syntax fixture from 50 to 300+
/// queries. Hand-curated cases cover the documented voidtools
/// surface; the generator below permutes building blocks
/// combinatorially to stress every parser path the hand-curated
/// list might miss. Standing Rule #8 regression gate.
#[test]
fn three_hundred_voidtools_queries_parse() {
    let mut all: Vec<String> = cases().into_iter().map(|c| c.q.to_string()).collect();
    all.extend(generated_voidtools_queries());
    assert!(
        all.len() >= 300,
        "Phase 10 gates 300+ queries; got {}",
        all.len()
    );
    for q in &all {
        parse(q).unwrap_or_else(|e| panic!("failed to parse `{q}`: {e}"));
    }
}

/// Generator: emits a deterministic sequence of voidtools-shaped
/// queries by walking small arrays of building blocks. The output
/// is intentionally over-the-300-target so an additional case can
/// be added to the hand-curated list without dropping below the
/// gate. Each generated query is parse-only — the assertion is
/// "no parse error" since the combinatorial space is too large to
/// hand-verify shapes for every output.
fn generated_voidtools_queries() -> Vec<String> {
    let terms = ["report", "draft", "alpha", "beta", "song", "photo"];
    let wildcards = ["*.txt", "report*", "*draft*", "*.{md,rst}", "?eport"];
    let modifiers = [
        "size:>1mb",
        "size:<100kb",
        "size:42",
        "date:today",
        "date:lastweek",
        "date:>2024-01-01",
        "ext:txt",
        "ext:rs;py",
        "attrib:H",
        "attrib:R",
        "path:Documents",
        "parent:Music",
        "child:report",
    ];
    let glue = ["AND", "OR", "NOT"];
    let qfilters = ["audio:", "video:", "image:", "document:"];

    let mut out: Vec<String> = Vec::with_capacity(300);

    // 1. Term × glue × term — 6×3×6 = 108 cases.
    for a in &terms {
        for g in &glue {
            for b in &terms {
                if a == b {
                    continue;
                }
                out.push(format!("{a} {g} {b}"));
            }
        }
    }
    // 2. Term + modifier — 6×13 = 78 cases.
    for a in &terms {
        for m in &modifiers {
            out.push(format!("{a} {m}"));
        }
    }
    // 3. Quick filter + modifier — 4×13 = 52 cases.
    for q in &qfilters {
        for m in &modifiers {
            out.push(format!("{q} {m}"));
        }
    }
    // 4. Wildcard + modifier — 5×4 = 20 cases.
    for w in &wildcards {
        for m in modifiers.iter().take(4) {
            out.push(format!("{w} {m}"));
        }
    }
    // 5. Parenthesised compositions — small handful for shape
    //    coverage that the term×glue×term path doesn't exercise.
    out.extend([
        "(alpha OR beta) gamma".to_string(),
        "(report AND draft) OR final".to_string(),
        "(a OR b) (c OR d)".to_string(),
        "alpha (beta OR gamma)".to_string(),
        "!(alpha AND beta)".to_string(),
        "report !(draft AND old)".to_string(),
        "size:>1mb (alpha OR beta)".to_string(),
        "(size:>1mb OR size:<10kb)".to_string(),
        "((alpha))".to_string(),
        "(((alpha)))".to_string(),
    ]);

    out
}

#[test]
fn parse_error_diagnostics_format_cleanly() {
    // The parse-error variants must Display without panicking and
    // include the byte position. Phase 11's UI surfaces these in a
    // status bar.
    let inputs = ["", "()))", "regex:[", "frobnicate:42", "size:abc"];
    for q in inputs {
        match parse(q) {
            Ok(_) => panic!("expected error for `{q}`"),
            Err(e) => {
                let _ = format!("{e}");
                match e {
                    ParseError::Empty
                    | ParseError::UnbalancedParens { .. }
                    | ParseError::UnexpectedToken { .. }
                    | ParseError::InvalidRegex { .. }
                    | ParseError::UnknownModifier { .. }
                    | ParseError::InvalidModifierValue { .. } => {}
                    other => panic!("unexpected variant for `{q}`: {other:?}"),
                }
            }
        }
    }
}
