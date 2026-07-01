//! Phase 10 IPC surface for `query.parse`.
//!
//! The Build-Guide prompt: "IPC `query.parse` returns AST + per-token
//! errors for live UI validation." This module synthesises a
//! serde-serializable [`ParseReport`] from the source query — the
//! token stream (with per-token spans + semantic kinds for syntax
//! highlighting), the parsed AST (when valid), and a list of typed
//! errors (each carrying a span + message + machine-readable code).
//!
//! `parse_to_report` never returns `Err` — that is the contract a UI
//! layer expects. Errors flow through `ParseReport::errors`. A query
//! that fails to parse still returns a complete token stream so the
//! search bar can keep highlighting while the user types.
//!
//! The serializable AST [`AstNode`] mirrors [`crate::QueryNode`]
//! minus the `Arc<Regex>` payload (regex source is kept as a string —
//! the IPC consumer compiles it on its own side if needed).
//!
//! Multi-error recovery: Phase 10 surfaces all strict-everything
//! violations in a single pass (each Word is independently
//! verifiable) plus the first non-strict parse error. Recover-and-
//! continue for arbitrary parse errors is a Phase 11 enhancement.

use serde::{Deserialize, Serialize};

use crate::ast::{
    AudioPredicate, DateBound, LensKind, ModifierKind, QueryNode, RelativeDate, SizeOp, TextPattern,
};
use crate::error::ParseError;
use crate::parser::{ParseOpts, TokKind, parse_with, tokenize};
use crate::quick_filters::QuickFilter;
use std::str::FromStr;

/// Closed half-open byte range into the source string, expressed as
/// `[start, end)`. `u32` is wide enough for any reasonable query
/// (the IPC consumer enforces an upstream cap; queries pushing past
/// 4 GiB are not a use case Freally ships toward).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct TokenSpan {
    pub start: u32,
    pub end: u32,
}

impl TokenSpan {
    pub fn new(start: usize, end: usize) -> Self {
        Self {
            start: start as u32,
            end: end as u32,
        }
    }
}

/// Semantic kind a UI layer uses for syntax highlighting. Mirrors
/// the parser's per-token classification, surfaced one level higher
/// so a `Word` is split into `Literal` / `Wildcard` / `Modifier{...}`
/// / `QuickFilter{...}` / `LensPrefix{...}` / `Regex` based on its
/// shape.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case", tag = "kind")]
pub enum TokenKind {
    Literal,
    Quoted,
    Wildcard,
    Regex,
    Modifier { name: String },
    QuickFilter { name: String },
    LensPrefix { lens: String },
    LParen,
    RParen,
    Bang,
    And,
    Or,
    Not,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TokenInfo {
    pub kind: TokenKind,
    pub span: TokenSpan,
    /// Original source text the span covers (includes outer quotes
    /// for `Quoted`, the trailing `:` for `Modifier` / `QuickFilter`,
    /// the trailing `:` for `LensPrefix`).
    pub text: String,
}

/// Machine-readable error code. The UI uses the code to pick a
/// localized hint string; the human-readable `message` is a fallback.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ErrorCode {
    Empty,
    UnexpectedEof,
    UnexpectedToken,
    UnbalancedParens,
    InvalidRegex,
    InvalidWildcard,
    UnknownModifier,
    InvalidModifierValue,
    StrictEverythingViolation,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ErrorInfo {
    pub span: TokenSpan,
    pub message: String,
    pub code: ErrorCode,
}

/// Parse report — the IPC payload returned by `query.parse`.
#[derive(Debug, Clone, Serialize)]
pub struct ParseReport {
    pub source: String,
    pub strict_everything: bool,
    /// Parsed AST (when the query parsed). Absent if a
    /// non-recoverable parse error fired — `errors` carries the
    /// reason. The token stream is always populated so the UI can
    /// keep highlighting partial input.
    pub ast: Option<AstNode>,
    pub tokens: Vec<TokenInfo>,
    pub errors: Vec<ErrorInfo>,
}

/// Serializable AST summary. Mirrors [`QueryNode`] but uses owned
/// primitives so the report round-trips through JSON.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case", tag = "kind")]
pub enum AstNode {
    Literal {
        text: String,
    },
    Wildcard {
        raw: String,
    },
    Regex {
        raw: String,
    },
    Modifier {
        name: String,
        /// Stable JSON-friendly description. The shape depends on
        /// the modifier; generic fields are kept as strings so the
        /// IPC consumer doesn't need every Phase-N modifier enum.
        detail: ModifierDetail,
    },
    QuickFilter {
        name: String,
    },
    Lens {
        lens: String,
        inner: Box<AstNode>,
    },
    And {
        children: Vec<AstNode>,
    },
    Or {
        children: Vec<AstNode>,
    },
    Not {
        inner: Box<AstNode>,
    },
    True,
}

/// Modifier detail. Each variant carries the smallest amount of
/// info the UI needs to render the modifier; the full Rust enum
/// shapes live in [`crate::ast`].
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case", tag = "shape")]
pub enum ModifierDetail {
    Size { op: String, bytes: u64 },
    DateRelative { name: String },
    DateDay { op: String, epoch_day: i64 },
    Ext { extensions: Vec<String> },
    Attrib { letters: String },
    Path { needle: String },
    Parent { needle: String },
    Child { needle: String },
    Similar { needle: String },
    AudioLufs { op: String, lufs: f32 },
    AudioCodec { codecs: Vec<String> },
    AudioLength { op: String, seconds: f32 },
    AudioRate { op: String, hz: u32 },
    AudioSilence { op: String, ratio: f32 },
    AudioDynamicRange { op: String, lu: f32 },
    Reserved { value: String },
}

/// Pure-function entry point. Returns a complete report for the
/// supplied source even if the query is malformed.
pub fn parse_to_report(s: &str, opts: ParseOpts) -> ParseReport {
    let mut tokens: Vec<TokenInfo> = Vec::new();
    let mut errors: Vec<ErrorInfo> = Vec::new();

    // Empty source: short-circuit with the canonical Empty error.
    if s.trim().is_empty() {
        errors.push(ErrorInfo {
            span: TokenSpan::new(0, s.len()),
            message: "empty query".into(),
            code: ErrorCode::Empty,
        });
        return ParseReport {
            source: s.to_string(),
            strict_everything: opts.strict_everything,
            ast: None,
            tokens,
            errors,
        };
    }

    // Phase-1 token stream — best-effort; bail with a single Error
    // if the lexer surfaces one (an unterminated quoted string).
    match tokenize(s) {
        Ok(toks) => {
            tokens.reserve(toks.len());
            // Two-pass token surfacing so we can look at the next
            // token (lens-prefix is `<key>:` followed by `(`).
            for i in 0..toks.len() {
                let t = &toks[i];
                let span = TokenSpan::new(t.byte_pos, t.byte_pos + t.byte_len);
                let text = s[t.byte_pos..t.byte_pos + t.byte_len].to_string();
                let kind = classify_token_kind(t, toks.get(i + 1));
                tokens.push(TokenInfo { kind, span, text });
            }
        }
        Err(err) => {
            errors.push(ErrorInfo::from_parse_error(&err, s));
            return ParseReport {
                source: s.to_string(),
                strict_everything: opts.strict_everything,
                ast: None,
                tokens,
                errors,
            };
        }
    }

    // Strict-everything: collect every Freally-only modifier /
    // lens prefix violation up-front so the UI sees them all at once
    // (rather than the first-found-then-bail behavior of `parse_with`).
    if opts.strict_everything {
        for ti in tokens.iter() {
            match &ti.kind {
                TokenKind::Modifier { name } if is_freally_only_modifier(name) => {
                    errors.push(ErrorInfo {
                        span: ti.span,
                        message: format!(
                            "modifier `{name}:` not in voidtools' Everything (strict-everything)"
                        ),
                        code: ErrorCode::StrictEverythingViolation,
                    });
                }
                // `name:(...)` is the only voidtools-coherent lens
                // prefix (alias of `child:`). Everything else is
                // Freally-only.
                TokenKind::LensPrefix { lens } if lens.as_str() != "name" => {
                    errors.push(ErrorInfo {
                        span: ti.span,
                        message: format!(
                            "lens prefix `{lens}:(` not in voidtools' Everything (strict-everything)"
                        ),
                        code: ErrorCode::StrictEverythingViolation,
                    });
                }
                _ => {}
            }
        }
    }

    // Run the real parser. Strict-violations are also raised here as
    // typed errors; the pre-scan above is the authoritative source for
    // strict violations (it walks every token and surfaces them all,
    // with spans covering the full Word). The parser bails on the
    // first violation it sees, with a span covering only the colon-
    // terminated prefix — so if the parse error is a strict-violation
    // AND the pre-scan already populated any strict-violation, the
    // parse-error is the duplicate-first-found copy and we drop it.
    match parse_with(s, opts) {
        Ok(q) => ParseReport {
            source: s.to_string(),
            strict_everything: opts.strict_everything,
            ast: Some(AstNode::from(q.root())),
            tokens,
            errors,
        },
        Err(err) => {
            let info = ErrorInfo::from_parse_error(&err, s);
            let already_have_strict = matches!(info.code, ErrorCode::StrictEverythingViolation)
                && errors
                    .iter()
                    .any(|e| matches!(e.code, ErrorCode::StrictEverythingViolation));
            if !already_have_strict {
                errors.push(info);
            }
            ParseReport {
                source: s.to_string(),
                strict_everything: opts.strict_everything,
                ast: None,
                tokens,
                errors,
            }
        }
    }
}

fn is_freally_only_modifier(key: &str) -> bool {
    matches!(
        key.to_ascii_lowercase().as_str(),
        "similar"
            | "lufs"
            | "codec"
            | "length"
            | "duration"
            | "rate"
            | "samplerate"
            | "silence"
            | "dr"
    )
}

/// Classify a parser-internal `Token` into a UI-shaped `TokenKind`.
/// Looks one token ahead to spot lens-prefix forms (`<key>:` followed
/// by `(`).
fn classify_token_kind(t: &crate::parser::Token, next: Option<&crate::parser::Token>) -> TokenKind {
    match t.kind {
        TokKind::LParen => TokenKind::LParen,
        TokKind::RParen => TokenKind::RParen,
        TokKind::Bang => TokenKind::Bang,
        TokKind::And => TokenKind::And,
        TokKind::Or => TokenKind::Or,
        TokKind::Not => TokenKind::Not,
        TokKind::Quoted => TokenKind::Quoted,
        TokKind::Word => classify_word_kind(t.lexeme.as_str(), next),
    }
}

fn classify_word_kind(lex: &str, next: Option<&crate::parser::Token>) -> TokenKind {
    if let Some(colon) = lex.find(':') {
        let key = &lex[..colon];
        let val = &lex[colon + 1..];
        if !key.is_empty() && key.chars().all(|c| c.is_ascii_alphabetic() || c == '_') {
            // `regex:<pattern>` is a regex term, not a modifier.
            if key.eq_ignore_ascii_case("regex") {
                return TokenKind::Regex;
            }
            // Lens prefix detection: empty value AND next is `(`.
            if val.is_empty()
                && matches!(next.map(|n| &n.kind), Some(TokKind::LParen))
                && let Some(lens) = is_lens_key(key)
            {
                return TokenKind::LensPrefix {
                    lens: lens.to_string(),
                };
            }
            // Quick filter detection (only for the empty-value form).
            // The `audio:foo` shape is parsed as `audio: AND foo`; we
            // surface the quick-filter token only when the user
            // typed `audio:`.
            if val.is_empty() && QuickFilter::from_str(key).is_ok() {
                return TokenKind::QuickFilter {
                    name: key.to_ascii_lowercase(),
                };
            }
            return TokenKind::Modifier {
                name: key.to_ascii_lowercase(),
            };
        }
    }
    if lex.contains('*') || lex.contains('?') {
        TokenKind::Wildcard
    } else {
        TokenKind::Literal
    }
}

fn is_lens_key(key: &str) -> Option<&'static str> {
    match key.to_ascii_lowercase().as_str() {
        "name" => Some("name"),
        "audio" => Some("audio"),
        "content" => Some("content"),
        "similar" => Some("similar"),
        _ => None,
    }
}

impl ErrorInfo {
    fn from_parse_error(err: &ParseError, source: &str) -> Self {
        let (start, end, code, msg) = match err {
            ParseError::Empty => (0, source.len(), ErrorCode::Empty, "empty query".to_string()),
            ParseError::UnexpectedEof { pos } => (
                *pos,
                source.len(),
                ErrorCode::UnexpectedEof,
                format!("{err}"),
            ),
            ParseError::UnexpectedToken { pos, token } => (
                *pos,
                pos + token.len(),
                ErrorCode::UnexpectedToken,
                format!("{err}"),
            ),
            ParseError::UnbalancedParens { pos } => {
                (*pos, pos + 1, ErrorCode::UnbalancedParens, format!("{err}"))
            }
            ParseError::InvalidRegex { regex, .. } => (
                0,
                source.len().min(regex.len() + 6),
                ErrorCode::InvalidRegex,
                format!("{err}"),
            ),
            ParseError::InvalidWildcard { pattern, .. } => (
                source.find(pattern.as_str()).unwrap_or(0),
                source.find(pattern.as_str()).unwrap_or(0) + pattern.len(),
                ErrorCode::InvalidWildcard,
                format!("{err}"),
            ),
            ParseError::UnknownModifier { pos, name } => (
                *pos,
                pos + name.len() + 1,
                ErrorCode::UnknownModifier,
                format!("{err}"),
            ),
            ParseError::InvalidModifierValue { name, value, .. } => {
                let needle = format!("{name}:{value}");
                let span_start = source.find(needle.as_str()).unwrap_or(0);
                (
                    span_start,
                    span_start + needle.len(),
                    ErrorCode::InvalidModifierValue,
                    format!("{err}"),
                )
            }
            ParseError::StrictEverythingViolation { pos, token, .. } => (
                *pos,
                pos + token.len(),
                ErrorCode::StrictEverythingViolation,
                format!("{err}"),
            ),
        };
        ErrorInfo {
            span: TokenSpan::new(start, end),
            message: msg,
            code,
        }
    }
}

// ---------- AstNode conversions ----------------------------------------

impl From<&QueryNode> for AstNode {
    fn from(node: &QueryNode) -> Self {
        match node {
            QueryNode::True => AstNode::True,
            QueryNode::Text(p) => match p {
                TextPattern::Literal(s) => AstNode::Literal { text: s.clone() },
                TextPattern::Wildcard { raw, .. } => AstNode::Wildcard { raw: raw.clone() },
                TextPattern::Regex { raw, .. } => AstNode::Regex { raw: raw.clone() },
            },
            QueryNode::Modifier(m) => AstNode::Modifier {
                name: modifier_name(&m.kind).to_string(),
                detail: ModifierDetail::from(&m.kind),
            },
            QueryNode::QuickFilter(qf) => AstNode::QuickFilter {
                name: format!("{qf:?}").to_lowercase(),
            },
            QueryNode::And(parts) => AstNode::And {
                children: parts.iter().map(AstNode::from).collect(),
            },
            QueryNode::Or(parts) => AstNode::Or {
                children: parts.iter().map(AstNode::from).collect(),
            },
            QueryNode::Not(inner) => AstNode::Not {
                inner: Box::new(AstNode::from(inner.as_ref())),
            },
            QueryNode::Lens { kind, inner } => AstNode::Lens {
                lens: lens_kind_str(*kind).to_string(),
                inner: Box::new(AstNode::from(inner.as_ref())),
            },
        }
    }
}

fn modifier_name(kind: &ModifierKind) -> &'static str {
    match kind {
        ModifierKind::Size { .. } => "size",
        ModifierKind::Date(_) => "date",
        ModifierKind::Ext(_) => "ext",
        ModifierKind::Attrib(_) => "attrib",
        ModifierKind::Path(_) => "path",
        ModifierKind::Parent(_) => "parent",
        ModifierKind::Child(_) => "child",
        ModifierKind::Similar(_) => "similar",
        ModifierKind::Audio(p) => match p {
            AudioPredicate::Lufs { .. } => "lufs",
            AudioPredicate::Codec(_) => "codec",
            AudioPredicate::Length { .. } => "length",
            AudioPredicate::Rate { .. } => "rate",
            AudioPredicate::Silence { .. } => "silence",
            AudioPredicate::DynamicRange { .. } => "dr",
        },
        ModifierKind::Reserved { .. } => "reserved",
    }
}

fn lens_kind_str(k: LensKind) -> &'static str {
    k.as_str()
}

fn op_str(op: SizeOp) -> &'static str {
    match op {
        SizeOp::Lt => "<",
        SizeOp::Le => "<=",
        SizeOp::Eq => "=",
        SizeOp::Ge => ">=",
        SizeOp::Gt => ">",
    }
}

fn rel_date_name(rd: RelativeDate) -> &'static str {
    match rd {
        RelativeDate::Today => "today",
        RelativeDate::Yesterday => "yesterday",
        RelativeDate::ThisWeek => "thisweek",
        RelativeDate::LastWeek => "lastweek",
        RelativeDate::ThisMonth => "thismonth",
        RelativeDate::LastMonth => "lastmonth",
        RelativeDate::ThisYear => "thisyear",
        RelativeDate::LastYear => "lastyear",
    }
}

impl From<&ModifierKind> for ModifierDetail {
    fn from(kind: &ModifierKind) -> Self {
        match kind {
            ModifierKind::Size { op, bytes } => ModifierDetail::Size {
                op: op_str(*op).to_string(),
                bytes: *bytes,
            },
            ModifierKind::Date(b) => match b {
                DateBound::Day { op, epoch_day } => ModifierDetail::DateDay {
                    op: op_str(*op).to_string(),
                    epoch_day: *epoch_day,
                },
                DateBound::Relative(rd) => ModifierDetail::DateRelative {
                    name: rel_date_name(*rd).to_string(),
                },
            },
            ModifierKind::Ext(exts) => ModifierDetail::Ext {
                extensions: exts.clone(),
            },
            ModifierKind::Attrib(flags) => {
                let letters: String = flags
                    .iter()
                    .map(|f| match f {
                        crate::ast::AttribFlag::ReadOnly => 'R',
                        crate::ast::AttribFlag::Hidden => 'H',
                        crate::ast::AttribFlag::System => 'S',
                        crate::ast::AttribFlag::Archive => 'A',
                        crate::ast::AttribFlag::Directory => 'D',
                        crate::ast::AttribFlag::Compressed => 'C',
                        crate::ast::AttribFlag::Encrypted => 'E',
                        crate::ast::AttribFlag::Temporary => 'T',
                        crate::ast::AttribFlag::Offline => 'O',
                        crate::ast::AttribFlag::Reparse => 'L',
                    })
                    .collect();
                ModifierDetail::Attrib { letters }
            }
            ModifierKind::Path(s) => ModifierDetail::Path { needle: s.clone() },
            ModifierKind::Parent(s) => ModifierDetail::Parent { needle: s.clone() },
            ModifierKind::Child(s) => ModifierDetail::Child { needle: s.clone() },
            ModifierKind::Similar(s) => ModifierDetail::Similar { needle: s.clone() },
            ModifierKind::Audio(p) => match p {
                AudioPredicate::Lufs { op, lufs } => ModifierDetail::AudioLufs {
                    op: op_str(*op).to_string(),
                    lufs: *lufs,
                },
                AudioPredicate::Codec(c) => ModifierDetail::AudioCodec { codecs: c.clone() },
                AudioPredicate::Length { op, seconds } => ModifierDetail::AudioLength {
                    op: op_str(*op).to_string(),
                    seconds: *seconds,
                },
                AudioPredicate::Rate { op, hz } => ModifierDetail::AudioRate {
                    op: op_str(*op).to_string(),
                    hz: *hz,
                },
                AudioPredicate::Silence { op, ratio } => ModifierDetail::AudioSilence {
                    op: op_str(*op).to_string(),
                    ratio: *ratio,
                },
                AudioPredicate::DynamicRange { op, lu } => ModifierDetail::AudioDynamicRange {
                    op: op_str(*op).to_string(),
                    lu: *lu,
                },
            },
            ModifierKind::Reserved { value, .. } => ModifierDetail::Reserved {
                value: value.clone(),
            },
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn rep(s: &str) -> ParseReport {
        parse_to_report(s, ParseOpts::default())
    }

    fn rep_strict(s: &str) -> ParseReport {
        parse_to_report(s, ParseOpts::strict())
    }

    #[test]
    fn empty_source_emits_typed_empty_error() {
        let r = rep("");
        assert!(r.ast.is_none());
        assert_eq!(r.errors.len(), 1);
        assert_eq!(r.errors[0].code, ErrorCode::Empty);
        assert!(r.tokens.is_empty());
    }

    #[test]
    fn whitespace_only_source_emits_empty() {
        let r = rep("   \t  ");
        assert_eq!(r.errors[0].code, ErrorCode::Empty);
    }

    #[test]
    fn literal_token_classifies() {
        let r = rep("report");
        assert!(r.errors.is_empty());
        assert_eq!(r.tokens.len(), 1);
        assert!(matches!(r.tokens[0].kind, TokenKind::Literal));
        assert_eq!(r.tokens[0].span, TokenSpan::new(0, 6));
    }

    #[test]
    fn wildcard_classifies() {
        let r = rep("*.txt");
        assert!(matches!(r.tokens[0].kind, TokenKind::Wildcard));
    }

    #[test]
    fn quoted_includes_outer_quotes_in_span() {
        let r = rep("\"final draft\"");
        assert_eq!(r.tokens.len(), 1);
        assert!(matches!(r.tokens[0].kind, TokenKind::Quoted));
        assert_eq!(r.tokens[0].span, TokenSpan::new(0, 13));
        assert_eq!(r.tokens[0].text, "\"final draft\"");
    }

    #[test]
    fn modifier_token_carries_name() {
        let r = rep("size:>100mb");
        assert_eq!(r.tokens.len(), 1);
        match &r.tokens[0].kind {
            TokenKind::Modifier { name } => assert_eq!(name, "size"),
            k => panic!("expected modifier got {k:?}"),
        }
    }

    #[test]
    fn quick_filter_token_classifies_for_bare_form() {
        let r = rep("audio:");
        match &r.tokens[0].kind {
            TokenKind::QuickFilter { name } => assert_eq!(name, "audio"),
            k => panic!("expected quick filter got {k:?}"),
        }
    }

    #[test]
    fn lens_prefix_classifies_when_followed_by_lparen() {
        let r = rep("audio:(lufs:<-14)");
        // Tokens: `audio:` + `(` + `lufs:<-14` + `)`
        assert_eq!(r.tokens.len(), 4);
        match &r.tokens[0].kind {
            TokenKind::LensPrefix { lens } => assert_eq!(lens, "audio"),
            k => panic!("expected lens prefix got {k:?}"),
        }
        assert!(matches!(r.tokens[1].kind, TokenKind::LParen));
        assert!(matches!(r.tokens[2].kind, TokenKind::Modifier { .. }));
        assert!(matches!(r.tokens[3].kind, TokenKind::RParen));
    }

    #[test]
    fn report_round_trips_to_json() {
        let r = rep("size:>100mb ext:txt");
        let s = serde_json::to_string(&r).expect("serialize");
        assert!(s.contains("size"));
        assert!(s.contains("modifier"));
    }

    #[test]
    fn ast_summary_shape_for_modifier() {
        let r = rep("size:>100mb");
        let ast = r.ast.expect("ast");
        match ast {
            AstNode::Modifier { name, detail } => {
                assert_eq!(name, "size");
                match detail {
                    ModifierDetail::Size { op, bytes } => {
                        assert_eq!(op, ">");
                        assert_eq!(bytes, 100 * 1024 * 1024);
                    }
                    d => panic!("unexpected detail: {d:?}"),
                }
            }
            n => panic!("unexpected node: {n:?}"),
        }
    }

    #[test]
    fn strict_everything_collects_multiple_violations() {
        // Three Freally-only modifiers in one query — strict mode
        // surfaces them all in `errors` (rather than just the first
        // one as `parse_with` would).
        let r = rep_strict("similar:foo lufs:<-14 codec:flac");
        let strict_violations: Vec<_> = r
            .errors
            .iter()
            .filter(|e| matches!(e.code, ErrorCode::StrictEverythingViolation))
            .collect();
        assert!(
            strict_violations.len() >= 3,
            "expected ≥3 strict violations, got {}: {:?}",
            strict_violations.len(),
            r.errors
        );
    }

    #[test]
    fn strict_everything_on_lens_prefix_reports() {
        let r = rep_strict("audio:(lufs:<-14)");
        let lens_violations: Vec<_> = r
            .errors
            .iter()
            .filter(|e| matches!(e.code, ErrorCode::StrictEverythingViolation))
            .filter(|e| e.message.contains("lens prefix"))
            .collect();
        assert!(
            !lens_violations.is_empty(),
            "expected lens-prefix strict violation, got {:?}",
            r.errors
        );
    }

    #[test]
    fn strict_everything_keeps_voidtools_modifiers() {
        // Pure-voidtools query under strict mode parses without
        // any errors.
        let r = rep_strict("size:>1mb date:lastweek ext:pdf");
        assert!(r.errors.is_empty(), "unexpected errors: {:?}", r.errors);
        assert!(r.ast.is_some());
    }

    #[test]
    fn parse_error_surfaces_with_span() {
        let r = rep("frobnicate:42");
        assert_eq!(r.errors.len(), 1);
        assert_eq!(r.errors[0].code, ErrorCode::UnknownModifier);
        // Span covers the modifier key + `:`.
        let txt = &r.source[r.errors[0].span.start as usize..r.errors[0].span.end as usize];
        assert!(txt.starts_with("frobnicate"));
    }

    #[test]
    fn unbalanced_paren_reports_position() {
        let r = rep("(a OR b");
        assert!(!r.errors.is_empty());
    }

    #[test]
    fn lens_prefix_round_trip_in_ast() {
        let r = rep("similar:(report-final) ext:pdf");
        let ast = r.ast.expect("ast");
        // Top-level AND of (Lens, Modifier).
        match ast {
            AstNode::And { children } => {
                assert_eq!(children.len(), 2);
                assert!(matches!(children[0], AstNode::Lens { .. }));
                assert!(matches!(children[1], AstNode::Modifier { .. }));
            }
            n => panic!("unexpected: {n:?}"),
        }
    }
}
