//! Query tokenizer + recursive-descent parser.
//!
//! Voidtools' Everything uses an unusual, undocumented-but-stable grammar
//! that we mirror 1:1 in `parse()`. Highlights:
//!
//! * Whitespace between atoms is implicit `AND`.
//! * `OR` and `AND` are case-insensitive keywords.
//! * `!` is a prefix-`NOT`; the `NOT` keyword exists too.
//! * `regex:<pattern>` introduces a raw regex; everything else is a
//!   literal substring (or a wildcard if it contains `*` / `?`).
//! * Quoted strings preserve internal whitespace and disable
//!   wildcard / regex parsing for that atom.
//! * `IDENT:VALUE` is a modifier; whitespace is forbidden either side
//!   of the colon (Everything's choice — we follow it for parity).
//! * Quick filters (`audio:` etc.) are modelled as no-value modifiers.

use std::str::FromStr;
use std::sync::Arc;
use std::time::{SystemTime, UNIX_EPOCH};

use regex::Regex;

use crate::ast::{
    AttribFlag, DateBound, ModifierKind, ModifierPredicate, Query, QueryNode, RelativeDate, SizeOp,
    SizeUnit, TextPattern,
};
use crate::error::ParseError;
use crate::quick_filters::QuickFilter;

const SECS_PER_DAY: i64 = 86_400;

/// Parse an Everything-syntax query string. The result holds the
/// original `s` so the plan cache can key on it.
pub fn parse(s: &str) -> Result<Query, ParseError> {
    let trimmed = s.trim();
    if trimmed.is_empty() {
        return Err(ParseError::Empty);
    }
    let tokens = tokenize(trimmed)?;
    let mut p = Parser::new(&tokens);
    let root = p.parse_or()?;
    if p.pos < p.tokens.len() {
        let bad = &p.tokens[p.pos];
        return Err(ParseError::UnexpectedToken {
            pos: bad.byte_pos,
            token: bad.lexeme.clone(),
        });
    }
    Ok(Query::new(s.to_string(), root))
}

#[derive(Debug, Clone, PartialEq, Eq)]
enum TokKind {
    /// Bare word, modifier-key, or modifier-key:value chunk. The
    /// parser later inspects whether `:` appears.
    Word,
    /// Quoted string. Outer quotes stripped by the tokenizer.
    Quoted,
    /// `(`
    LParen,
    /// `)`
    RParen,
    /// `!` prefix-not.
    Bang,
    /// Reserved keyword `AND` (case-insensitive).
    And,
    /// Reserved keyword `OR` (case-insensitive).
    Or,
    /// Reserved keyword `NOT` (case-insensitive).
    Not,
}

#[derive(Debug, Clone)]
struct Token {
    kind: TokKind,
    lexeme: String,
    byte_pos: usize,
}

fn tokenize(s: &str) -> Result<Vec<Token>, ParseError> {
    let bytes = s.as_bytes();
    let mut out = Vec::new();
    let mut i = 0;
    while i < bytes.len() {
        let b = bytes[i];
        if b.is_ascii_whitespace() {
            i += 1;
            continue;
        }
        if b == b'(' {
            out.push(Token {
                kind: TokKind::LParen,
                lexeme: "(".into(),
                byte_pos: i,
            });
            i += 1;
            continue;
        }
        if b == b')' {
            out.push(Token {
                kind: TokKind::RParen,
                lexeme: ")".into(),
                byte_pos: i,
            });
            i += 1;
            continue;
        }
        if b == b'!' {
            // Voidtools-Everything semantics: `!` is a prefix-NOT when
            // it sits at the start of input, after whitespace, or right
            // after a `(` / `)` / `!` / `AND` / `OR` / `NOT` token.
            // Glued mid-word (`foo!bar`) it's literal. The `)` case is
            // load-bearing: `(a)!b` parses as `(a) AND !b`, matching
            // Everything's documented behavior.
            let prev_byte_is_boundary =
                i == 0 || matches!(bytes[i - 1], b' ' | b'\t' | b'\r' | b'\n' | b'(' | b')');
            let prev_tok_is_boundary = out
                .last()
                .map(|t| {
                    matches!(
                        t.kind,
                        TokKind::LParen
                            | TokKind::RParen
                            | TokKind::And
                            | TokKind::Or
                            | TokKind::Not
                            | TokKind::Bang
                    )
                })
                .unwrap_or(true);
            if prev_byte_is_boundary || prev_tok_is_boundary {
                out.push(Token {
                    kind: TokKind::Bang,
                    lexeme: "!".into(),
                    byte_pos: i,
                });
                i += 1;
                continue;
            }
        }
        if b == b'"' {
            let start = i + 1;
            let mut j = start;
            while j < bytes.len() && bytes[j] != b'"' {
                j += 1;
            }
            if j >= bytes.len() {
                return Err(ParseError::UnexpectedEof { pos: i });
            }
            // SAFETY: between two `"` markers within a Rust `&str` the
            // bytes are still valid UTF-8 because we never sliced
            // mid-codepoint (we only stop on the ASCII `"` byte).
            let lex = std::str::from_utf8(&bytes[start..j])
                .map_err(|_| ParseError::UnexpectedToken {
                    pos: i,
                    token: "<invalid utf-8>".into(),
                })?
                .to_string();
            out.push(Token {
                kind: TokKind::Quoted,
                lexeme: lex,
                byte_pos: i,
            });
            i = j + 1;
            continue;
        }
        // Word — eat until whitespace or paren.
        let start = i;
        while i < bytes.len()
            && !bytes[i].is_ascii_whitespace()
            && bytes[i] != b'('
            && bytes[i] != b')'
        {
            i += 1;
        }
        let lex = std::str::from_utf8(&bytes[start..i])
            .map_err(|_| ParseError::UnexpectedToken {
                pos: start,
                token: "<invalid utf-8>".into(),
            })?
            .to_string();
        let kind = match lex.to_ascii_uppercase().as_str() {
            "AND" => TokKind::And,
            "OR" => TokKind::Or,
            "NOT" => TokKind::Not,
            _ => TokKind::Word,
        };
        out.push(Token {
            kind,
            lexeme: lex,
            byte_pos: start,
        });
    }
    Ok(out)
}

struct Parser<'a> {
    tokens: &'a [Token],
    pos: usize,
}

impl<'a> Parser<'a> {
    fn new(tokens: &'a [Token]) -> Self {
        Self { tokens, pos: 0 }
    }

    fn peek(&self) -> Option<&Token> {
        self.tokens.get(self.pos)
    }

    fn bump(&mut self) -> &Token {
        let t = &self.tokens[self.pos];
        self.pos += 1;
        t
    }

    fn parse_or(&mut self) -> Result<QueryNode, ParseError> {
        let mut alts = vec![self.parse_and()?];
        while matches!(self.peek().map(|t| &t.kind), Some(TokKind::Or)) {
            self.bump();
            alts.push(self.parse_and()?);
        }
        Ok(if alts.len() == 1 {
            alts.pop().unwrap()
        } else {
            QueryNode::Or(alts)
        })
    }

    fn parse_and(&mut self) -> Result<QueryNode, ParseError> {
        let mut conj = vec![self.parse_unary()?];
        loop {
            match self.peek().map(|t| &t.kind) {
                Some(TokKind::And) => {
                    self.bump();
                    conj.push(self.parse_unary()?);
                }
                Some(TokKind::Or) | Some(TokKind::RParen) | None => break,
                _ => {
                    // Implicit AND between adjacent atoms.
                    conj.push(self.parse_unary()?);
                }
            }
        }
        Ok(if conj.len() == 1 {
            conj.pop().unwrap()
        } else {
            QueryNode::And(conj)
        })
    }

    fn parse_unary(&mut self) -> Result<QueryNode, ParseError> {
        match self.peek().map(|t| &t.kind) {
            Some(TokKind::Bang) | Some(TokKind::Not) => {
                self.bump();
                let inner = self.parse_unary()?;
                Ok(QueryNode::Not(Box::new(inner)))
            }
            _ => self.parse_atom(),
        }
    }

    fn parse_atom(&mut self) -> Result<QueryNode, ParseError> {
        let tok = self.peek().ok_or(ParseError::UnexpectedEof {
            pos: self
                .tokens
                .last()
                .map(|t| t.byte_pos + t.lexeme.len())
                .unwrap_or(0),
        })?;
        match tok.kind {
            TokKind::LParen => {
                let pos = tok.byte_pos;
                self.bump();
                if matches!(self.peek().map(|t| &t.kind), Some(TokKind::RParen)) {
                    self.bump();
                    return Ok(QueryNode::True);
                }
                let inner = self.parse_or()?;
                match self.peek().map(|t| &t.kind) {
                    Some(TokKind::RParen) => {
                        self.bump();
                        Ok(inner)
                    }
                    _ => Err(ParseError::UnbalancedParens { pos }),
                }
            }
            TokKind::RParen => Err(ParseError::UnbalancedParens { pos: tok.byte_pos }),
            TokKind::And | TokKind::Or | TokKind::Not | TokKind::Bang => {
                Err(ParseError::UnexpectedToken {
                    pos: tok.byte_pos,
                    token: tok.lexeme.clone(),
                })
            }
            TokKind::Quoted => {
                let lex = tok.lexeme.clone();
                self.bump();
                Ok(QueryNode::Text(TextPattern::Literal(lex)))
            }
            TokKind::Word => {
                let lex = tok.lexeme.clone();
                let pos = tok.byte_pos;
                self.bump();
                classify_word(&lex, pos)
            }
        }
    }
}

fn classify_word(lex: &str, pos: usize) -> Result<QueryNode, ParseError> {
    if let Some(colon_idx) = lex.find(':') {
        let key = &lex[..colon_idx];
        let val = &lex[colon_idx + 1..];
        if !key.is_empty() && key.chars().all(|c| c.is_ascii_alphabetic() || c == '_') {
            // "regex" is a textual escape into a regex term, not a modifier.
            if key.eq_ignore_ascii_case("regex") {
                let compiled = Regex::new(val).map_err(|e| ParseError::InvalidRegex {
                    regex: val.to_string(),
                    source: e,
                })?;
                return Ok(QueryNode::Text(TextPattern::Regex {
                    raw: val.to_string(),
                    compiled: Arc::new(compiled),
                }));
            }
            // Quick filters look like `audio:` (empty value) or `audio:foo`
            // (the suffix is treated as an extra literal AND).
            if let Ok(qf) = QuickFilter::from_str(key) {
                if val.is_empty() {
                    return Ok(QueryNode::QuickFilter(qf));
                }
                let extra = classify_simple_term(val)?;
                return Ok(QueryNode::And(vec![QueryNode::QuickFilter(qf), extra]));
            }
            return parse_modifier(key, val, pos).map(QueryNode::Modifier);
        }
    }
    classify_simple_term(lex)
}

fn classify_simple_term(lex: &str) -> Result<QueryNode, ParseError> {
    if lex.contains('*') || lex.contains('?') {
        let re = wildcard_to_regex(lex)?;
        return Ok(QueryNode::Text(TextPattern::Wildcard {
            raw: lex.to_string(),
            compiled: Arc::new(re),
        }));
    }
    Ok(QueryNode::Text(TextPattern::Literal(lex.to_string())))
}

fn wildcard_to_regex(pat: &str) -> Result<Regex, ParseError> {
    let mut re = String::with_capacity(pat.len() * 2 + 4);
    re.push_str("(?i)");
    for c in pat.chars() {
        match c {
            '*' => re.push_str(".*"),
            '?' => re.push('.'),
            // Wildcards anchor unspecified — Everything matches `*foo*`
            // semantics by default. Users who want strict anchors use
            // `regex:`. We don't anchor here so wildcard literals inside
            // longer names match the way Everything users expect.
            c if c.is_ascii_alphanumeric() => re.push(c),
            c => {
                re.push('\\');
                re.push(c);
            }
        }
    }
    Regex::new(&re).map_err(|e| ParseError::InvalidWildcard {
        pattern: pat.to_string(),
        message: e.to_string(),
    })
}

fn parse_modifier(key: &str, value: &str, pos: usize) -> Result<ModifierPredicate, ParseError> {
    let key_lower = key.to_ascii_lowercase();
    let kind = match key_lower.as_str() {
        "size" => parse_size(value, key)?,
        "date" | "dm" | "dc" | "da" => parse_date(value, key)?,
        "ext" => parse_ext(value),
        "attrib" | "attr" | "attributes" => parse_attrib(value, key)?,
        "path" => ModifierKind::Path(value.to_string()),
        "parent" | "folder" => ModifierKind::Parent(value.to_string()),
        "child" | "name" => ModifierKind::Child(value.to_string()),
        // Reserved for future lenses (Phase 6/7/9) AND voidtools-
        // Everything muscle-memory tokens that don't have a Phase-5
        // semantic mapping yet (`wfn:` / `wholefilename:` / `case:` /
        // `count:` / `dupe:` / `regex:` toggle / `nodiacritics:`).
        // Standing Rule #8: every query that *parses* today must keep
        // parsing through Phase 14 — so we accept them here and let
        // `validate_supported` route the unimplemented ones to a
        // typed `QueryError::UnsupportedModifier` rather than silent
        // mismatch. Phase 11 surfaces these as a UI hint.
        "content" | "lufs" | "codec" | "channels" | "samplerate" | "length" | "similar"
        | "duration" | "type" | "lang" | "wfn" | "wholefilename" | "case" | "count" | "dupe"
        | "nodiacritics" => ModifierKind::Reserved {
            name: key_lower,
            value: value.to_string(),
        },
        _ => {
            return Err(ParseError::UnknownModifier {
                pos,
                name: key.to_string(),
            });
        }
    };
    Ok(ModifierPredicate { kind })
}

fn parse_size(value: &str, key: &str) -> Result<ModifierKind, ParseError> {
    let mut chars = value.chars().peekable();
    let op = match chars.peek() {
        Some('>') => {
            chars.next();
            if matches!(chars.peek(), Some('=')) {
                chars.next();
                SizeOp::Ge
            } else {
                SizeOp::Gt
            }
        }
        Some('<') => {
            chars.next();
            if matches!(chars.peek(), Some('=')) {
                chars.next();
                SizeOp::Le
            } else {
                SizeOp::Lt
            }
        }
        Some('=') => {
            chars.next();
            SizeOp::Eq
        }
        _ => SizeOp::Eq,
    };
    let rest: String = chars.collect();
    let (num_str, suffix) = split_number_suffix(&rest);
    if num_str.is_empty() {
        return Err(ParseError::InvalidModifierValue {
            name: key.to_string(),
            value: value.to_string(),
            reason: "missing number".into(),
        });
    }
    let n: f64 = num_str
        .parse()
        .map_err(|_| ParseError::InvalidModifierValue {
            name: key.to_string(),
            value: value.to_string(),
            reason: "not a number".into(),
        })?;
    let unit = match suffix.to_ascii_lowercase().as_str() {
        "" | "b" => SizeUnit::Byte,
        "k" | "kb" | "kib" => SizeUnit::Kilo,
        "m" | "mb" | "mib" => SizeUnit::Mega,
        "g" | "gb" | "gib" => SizeUnit::Giga,
        "t" | "tb" | "tib" => SizeUnit::Tera,
        _ => {
            return Err(ParseError::InvalidModifierValue {
                name: key.to_string(),
                value: value.to_string(),
                reason: format!("unknown size unit `{suffix}`"),
            });
        }
    };
    let bytes = (n * unit.multiplier() as f64) as u64;
    Ok(ModifierKind::Size { op, bytes })
}

fn split_number_suffix(s: &str) -> (&str, &str) {
    let mut split = s.len();
    for (i, c) in s.char_indices() {
        if !(c.is_ascii_digit() || c == '.') {
            split = i;
            break;
        }
    }
    s.split_at(split)
}

fn parse_date(value: &str, key: &str) -> Result<ModifierKind, ParseError> {
    let v = value.trim();
    let (op, body) = if let Some(rest) = v.strip_prefix(">=") {
        (SizeOp::Ge, rest)
    } else if let Some(rest) = v.strip_prefix("<=") {
        (SizeOp::Le, rest)
    } else if let Some(rest) = v.strip_prefix('>') {
        (SizeOp::Gt, rest)
    } else if let Some(rest) = v.strip_prefix('<') {
        (SizeOp::Lt, rest)
    } else if let Some(rest) = v.strip_prefix('=') {
        (SizeOp::Eq, rest)
    } else {
        (SizeOp::Eq, v)
    };
    let body_trim = body.trim();
    if let Ok(rd) = parse_relative_date(body_trim) {
        if op != SizeOp::Eq {
            return Err(ParseError::InvalidModifierValue {
                name: key.to_string(),
                value: value.to_string(),
                reason: "comparator not allowed with relative dates".into(),
            });
        }
        return Ok(ModifierKind::Date(DateBound::Relative(rd)));
    }
    let day = parse_iso_day(body_trim).ok_or_else(|| ParseError::InvalidModifierValue {
        name: key.to_string(),
        value: value.to_string(),
        reason: "expected YYYY-MM-DD or relative (today / yesterday / lastweek / …)".into(),
    })?;
    Ok(ModifierKind::Date(DateBound::Day { epoch_day: day, op }))
}

fn parse_relative_date(s: &str) -> Result<RelativeDate, ()> {
    match s.to_ascii_lowercase().as_str() {
        "today" => Ok(RelativeDate::Today),
        "yesterday" => Ok(RelativeDate::Yesterday),
        "thisweek" | "this-week" => Ok(RelativeDate::ThisWeek),
        "lastweek" | "last-week" => Ok(RelativeDate::LastWeek),
        "thismonth" | "this-month" => Ok(RelativeDate::ThisMonth),
        "lastmonth" | "last-month" => Ok(RelativeDate::LastMonth),
        "thisyear" | "this-year" => Ok(RelativeDate::ThisYear),
        "lastyear" | "last-year" => Ok(RelativeDate::LastYear),
        _ => Err(()),
    }
}

/// Parse a `YYYY-MM-DD` into a Unix epoch-day (days since 1970-01-01).
/// Pure arithmetic — no date crate needed.
pub(crate) fn parse_iso_day(s: &str) -> Option<i64> {
    if s.len() != 10 {
        return None;
    }
    let bytes = s.as_bytes();
    if bytes[4] != b'-' || bytes[7] != b'-' {
        return None;
    }
    let y: i32 = std::str::from_utf8(&bytes[0..4]).ok()?.parse().ok()?;
    let m: u32 = std::str::from_utf8(&bytes[5..7]).ok()?.parse().ok()?;
    let d: u32 = std::str::from_utf8(&bytes[8..10]).ok()?.parse().ok()?;
    civil_to_epoch_day(y, m, d)
}

/// Howard Hinnant's days-from-civil algorithm. Public-domain in his
/// CC0-licensed write-up; one of the cleanest ways to skip pulling in
/// `chrono`/`time`. Returns days since 1970-01-01 (negative for earlier).
fn civil_to_epoch_day(y: i32, m: u32, d: u32) -> Option<i64> {
    if !(1..=12).contains(&m) || !(1..=31).contains(&d) {
        return None;
    }
    // Reject days that don't exist in the specified month (Feb 30,
    // Apr 31, etc.). Voidtools rejects these — Standing Rule #8 says
    // we follow.
    if d > days_in_month(y, m) {
        return None;
    }
    let m = m as i32;
    let d = d as i32;
    let y = if m <= 2 { y - 1 } else { y };
    let era = if y >= 0 { y } else { y - 399 } / 400;
    let yoe = (y - era * 400) as i64;
    let doy = ((153 * (if m > 2 { m - 3 } else { m + 9 }) + 2) / 5 + d - 1) as i64;
    let doe = yoe * 365 + yoe / 4 - yoe / 100 + doy;
    Some((era as i64) * 146097 + doe - 719468)
}

fn days_in_month(y: i32, m: u32) -> u32 {
    match m {
        1 | 3 | 5 | 7 | 8 | 10 | 12 => 31,
        4 | 6 | 9 | 11 => 30,
        2 => {
            if is_leap_year(y) {
                29
            } else {
                28
            }
        }
        _ => 0,
    }
}

fn is_leap_year(y: i32) -> bool {
    (y.rem_euclid(4) == 0 && y.rem_euclid(100) != 0) || y.rem_euclid(400) == 0
}

pub(crate) fn now_epoch_day() -> i64 {
    let secs = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|d| d.as_secs() as i64)
        .unwrap_or(0);
    secs.div_euclid(SECS_PER_DAY)
}

/// Resolve a relative-date alias to the half-open day range
/// `[start, end)` (Unix epoch days). Used by both the executor and
/// the parser's diagnostic round-trips.
pub(crate) fn relative_day_range(rd: RelativeDate) -> (i64, i64) {
    let today = now_epoch_day();
    let weekday = epoch_day_weekday(today); // 0 = Monday
    match rd {
        RelativeDate::Today => (today, today + 1),
        RelativeDate::Yesterday => (today - 1, today),
        RelativeDate::ThisWeek => {
            let start = today - weekday as i64;
            (start, start + 7)
        }
        RelativeDate::LastWeek => {
            let start = today - weekday as i64 - 7;
            (start, start + 7)
        }
        RelativeDate::ThisMonth => {
            let (y, m, _) = epoch_day_to_civil(today);
            let start = civil_to_epoch_day(y, m, 1).unwrap_or(today);
            let (ny, nm) = if m == 12 { (y + 1, 1) } else { (y, m + 1) };
            let end = civil_to_epoch_day(ny, nm, 1).unwrap_or(start + 31);
            (start, end)
        }
        RelativeDate::LastMonth => {
            let (y, m, _) = epoch_day_to_civil(today);
            let (py, pm) = if m == 1 { (y - 1, 12) } else { (y, m - 1) };
            let start = civil_to_epoch_day(py, pm, 1).unwrap_or(today - 31);
            let end = civil_to_epoch_day(y, m, 1).unwrap_or(today);
            (start, end)
        }
        RelativeDate::ThisYear => {
            let (y, _, _) = epoch_day_to_civil(today);
            let start = civil_to_epoch_day(y, 1, 1).unwrap_or(today);
            let end = civil_to_epoch_day(y + 1, 1, 1).unwrap_or(start + 366);
            (start, end)
        }
        RelativeDate::LastYear => {
            let (y, _, _) = epoch_day_to_civil(today);
            let start = civil_to_epoch_day(y - 1, 1, 1).unwrap_or(today);
            let end = civil_to_epoch_day(y, 1, 1).unwrap_or(today);
            (start, end)
        }
    }
}

/// Inverse of `civil_to_epoch_day`. Used only by relative-date math
/// so we can keep the algorithm self-contained.
fn epoch_day_to_civil(z: i64) -> (i32, u32, u32) {
    let z = z + 719468;
    let era = if z >= 0 { z } else { z - 146096 } / 146097;
    let doe = z - era * 146097;
    let yoe = (doe - doe / 1460 + doe / 36524 - doe / 146096) / 365;
    let y = yoe + era * 400;
    let doy = doe - (365 * yoe + yoe / 4 - yoe / 100);
    let mp = (5 * doy + 2) / 153;
    let d = (doy - (153 * mp + 2) / 5 + 1) as u32;
    let m = (if mp < 10 { mp + 3 } else { mp - 9 }) as u32;
    let y = if m <= 2 { y + 1 } else { y };
    (y as i32, m, d)
}

fn epoch_day_weekday(epoch_day: i64) -> u32 {
    // 1970-01-01 was a Thursday (weekday 3 in Mon-based indexing).
    ((epoch_day + 3).rem_euclid(7)) as u32
}

fn parse_ext(value: &str) -> ModifierKind {
    let exts: Vec<String> = value
        .split(';')
        .map(str::trim)
        .filter(|s| !s.is_empty())
        .map(|s| s.trim_start_matches('.').to_ascii_lowercase())
        .collect();
    ModifierKind::Ext(exts)
}

fn parse_attrib(value: &str, key: &str) -> Result<ModifierKind, ParseError> {
    let mut flags = Vec::new();
    for c in value.chars() {
        if c.is_ascii_whitespace() {
            continue;
        }
        match AttribFlag::from_letter(c) {
            Some(f) => {
                if !flags.contains(&f) {
                    flags.push(f);
                }
            }
            None => {
                return Err(ParseError::InvalidModifierValue {
                    name: key.to_string(),
                    value: value.to_string(),
                    reason: format!("unknown attrib letter `{c}`"),
                });
            }
        }
    }
    if flags.is_empty() {
        return Err(ParseError::InvalidModifierValue {
            name: key.to_string(),
            value: value.to_string(),
            reason: "no recognised attrib letters".into(),
        });
    }
    Ok(ModifierKind::Attrib(flags))
}

#[cfg(test)]
mod tests {
    use super::*;

    fn pp(s: &str) -> Query {
        parse(s).expect("parse")
    }

    #[test]
    fn empty_query_errors() {
        assert!(matches!(parse("   "), Err(ParseError::Empty)));
    }

    #[test]
    fn literal_term() {
        let q = pp("report");
        match q.root() {
            QueryNode::Text(TextPattern::Literal(l)) => assert_eq!(l, "report"),
            n => panic!("unexpected: {n:?}"),
        }
    }

    #[test]
    fn quoted_preserves_whitespace() {
        let q = pp("\"final draft.docx\"");
        match q.root() {
            QueryNode::Text(TextPattern::Literal(l)) => assert_eq!(l, "final draft.docx"),
            n => panic!("unexpected: {n:?}"),
        }
    }

    #[test]
    fn wildcard_compiles() {
        let q = pp("*.txt");
        match q.root() {
            QueryNode::Text(TextPattern::Wildcard { raw, compiled }) => {
                assert_eq!(raw, "*.txt");
                assert!(compiled.is_match("hello.txt"));
                assert!(!compiled.is_match("foo.md"));
            }
            n => panic!("unexpected: {n:?}"),
        }
    }

    #[test]
    fn regex_term() {
        let q = pp("regex:^report-\\d+$");
        match q.root() {
            QueryNode::Text(TextPattern::Regex { raw, compiled }) => {
                assert_eq!(raw, "^report-\\d+$");
                assert!(compiled.is_match("report-42"));
                assert!(!compiled.is_match("xreport-42"));
            }
            n => panic!("unexpected: {n:?}"),
        }
    }

    #[test]
    fn implicit_and() {
        let q = pp("report draft");
        match q.root() {
            QueryNode::And(parts) => assert_eq!(parts.len(), 2),
            n => panic!("unexpected: {n:?}"),
        }
    }

    #[test]
    fn explicit_or_lower_precedence() {
        let q = pp("a b OR c");
        match q.root() {
            QueryNode::Or(alts) => {
                assert_eq!(alts.len(), 2);
                assert!(matches!(&alts[0], QueryNode::And(_)));
                assert!(matches!(&alts[1], QueryNode::Text(_)));
            }
            n => panic!("unexpected: {n:?}"),
        }
    }

    #[test]
    fn bang_prefix_is_not() {
        let q = pp("!draft");
        assert!(matches!(q.root(), QueryNode::Not(_)));
    }

    #[test]
    fn paren_grouping() {
        let q = pp("(a OR b) c");
        match q.root() {
            QueryNode::And(parts) => {
                assert!(matches!(&parts[0], QueryNode::Or(_)));
                assert!(matches!(&parts[1], QueryNode::Text(_)));
            }
            n => panic!("unexpected: {n:?}"),
        }
    }

    #[test]
    fn empty_paren_is_true() {
        let q = pp("()");
        assert!(matches!(q.root(), QueryNode::True));
    }

    #[test]
    fn size_modifier_with_units() {
        let q = pp("size:>100mb");
        match q.root() {
            QueryNode::Modifier(m) => match &m.kind {
                ModifierKind::Size { op, bytes } => {
                    assert_eq!(*op, SizeOp::Gt);
                    assert_eq!(*bytes, 100 * 1024 * 1024);
                }
                k => panic!("unexpected: {k:?}"),
            },
            n => panic!("unexpected: {n:?}"),
        }
    }

    #[test]
    fn ext_modifier_semicolon_list() {
        let q = pp("ext:txt;rs;py");
        match q.root() {
            QueryNode::Modifier(m) => match &m.kind {
                ModifierKind::Ext(exts) => assert_eq!(exts, &["txt", "rs", "py"]),
                k => panic!("unexpected: {k:?}"),
            },
            n => panic!("unexpected: {n:?}"),
        }
    }

    #[test]
    fn attrib_letters() {
        let q = pp("attrib:HRA");
        match q.root() {
            QueryNode::Modifier(m) => match &m.kind {
                ModifierKind::Attrib(flags) => {
                    assert!(flags.contains(&AttribFlag::Hidden));
                    assert!(flags.contains(&AttribFlag::ReadOnly));
                    assert!(flags.contains(&AttribFlag::Archive));
                }
                k => panic!("unexpected: {k:?}"),
            },
            n => panic!("unexpected: {n:?}"),
        }
    }

    #[test]
    fn date_relative() {
        let q = pp("date:lastweek");
        match q.root() {
            QueryNode::Modifier(m) => match &m.kind {
                ModifierKind::Date(DateBound::Relative(RelativeDate::LastWeek)) => {}
                k => panic!("unexpected: {k:?}"),
            },
            n => panic!("unexpected: {n:?}"),
        }
    }

    #[test]
    fn date_absolute() {
        let q = pp("date:>2024-01-01");
        match q.root() {
            QueryNode::Modifier(m) => match &m.kind {
                ModifierKind::Date(DateBound::Day { op, .. }) => assert_eq!(*op, SizeOp::Gt),
                k => panic!("unexpected: {k:?}"),
            },
            n => panic!("unexpected: {n:?}"),
        }
    }

    #[test]
    fn unknown_modifier_errors() {
        let err = parse("frobnicate:42").unwrap_err();
        assert!(matches!(err, ParseError::UnknownModifier { .. }));
    }

    #[test]
    fn quick_filter_alone() {
        let q = pp("audio:");
        match q.root() {
            QueryNode::QuickFilter(QuickFilter::Audio) => {}
            n => panic!("unexpected: {n:?}"),
        }
    }

    #[test]
    fn quick_filter_with_suffix() {
        let q = pp("audio:beat");
        match q.root() {
            QueryNode::And(parts) => {
                assert!(matches!(
                    &parts[0],
                    QueryNode::QuickFilter(QuickFilter::Audio)
                ));
                match &parts[1] {
                    QueryNode::Text(TextPattern::Literal(l)) => assert_eq!(l, "beat"),
                    n => panic!("unexpected: {n:?}"),
                }
            }
            n => panic!("unexpected: {n:?}"),
        }
    }

    #[test]
    fn bang_after_rparen_is_not() {
        // Voidtools-Everything compat: `(a)!b` parses as `(a) AND !b`.
        // Regression gate for the tokenizer's prev-byte-is-boundary
        // check including `)`.
        let q = pp("(a)!b");
        match q.root() {
            QueryNode::And(parts) => {
                assert_eq!(parts.len(), 2);
                match &parts[0] {
                    QueryNode::Text(TextPattern::Literal(l)) => assert_eq!(l, "a"),
                    n => panic!("unexpected: {n:?}"),
                }
                match &parts[1] {
                    QueryNode::Not(_) => {}
                    n => panic!("expected NOT got {n:?}"),
                }
            }
            n => panic!("unexpected: {n:?}"),
        }
    }

    #[test]
    fn invalid_calendar_days_reject() {
        // Feb 30 / Apr 31 / Nov 31 / non-leap Feb 29 must not slip
        // past `parse_iso_day`.
        assert_eq!(parse_iso_day("2024-02-30"), None);
        assert_eq!(parse_iso_day("2024-04-31"), None);
        assert_eq!(parse_iso_day("2023-02-29"), None);
        // Sanity: real calendar days still round-trip.
        assert!(parse_iso_day("2024-02-29").is_some()); // leap
        assert!(parse_iso_day("2024-04-30").is_some());
    }

    #[test]
    fn voidtools_reserved_toggles_parse() {
        // Standing Rule #8: every query that parses today must keep
        // parsing through Phase 14. The `wfn:` / `case:` / `count:` /
        // `dupe:` family is Everything muscle-memory; we accept-and-
        // reserve so users typing them get a typed
        // `QueryError::UnsupportedModifier` at execute time, not a
        // parse error.
        for q in [
            "wfn:foo",
            "wholefilename:foo",
            "case:foo",
            "count:1",
            "dupe:foo",
            "type:audio",
        ] {
            let parsed = parse(q).unwrap_or_else(|e| panic!("`{q}` failed: {e}"));
            match parsed.root() {
                QueryNode::Modifier(m) => match &m.kind {
                    ModifierKind::Reserved { .. } => {}
                    k => panic!("`{q}` not Reserved: {k:?}"),
                },
                n => panic!("`{q}` unexpected: {n:?}"),
            }
        }
    }

    #[test]
    fn epoch_round_trip() {
        // 2024-03-04 → 19_786 days since 1970-01-01
        assert_eq!(parse_iso_day("2024-03-04"), Some(19_786));
        let (y, m, d) = epoch_day_to_civil(19_786);
        assert_eq!((y, m, d), (2024, 3, 4));
    }

    #[test]
    fn epoch_negative_dates_round_trip() {
        // 1969-12-31 → -1; the `era` arithmetic must not skip below 1970.
        let day = parse_iso_day("1969-12-31").unwrap();
        assert_eq!(day, -1);
        let (y, m, d) = epoch_day_to_civil(day);
        assert_eq!((y, m, d), (1969, 12, 31));
    }
}
