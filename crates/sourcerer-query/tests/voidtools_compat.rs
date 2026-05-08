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

use sourcerer_query::QuickFilter as Qf;
use sourcerer_query::{
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
