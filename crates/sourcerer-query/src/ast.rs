//! Query AST.
//!
//! The grammar is voidtools-Everything-shaped:
//!
//! ```text
//! query      := or
//! or         := and ("OR" and)*
//! and        := unary (("AND" | <whitespace>) unary)*
//! unary      := ("NOT" | "!") unary | atom
//! atom       := group | modifier | term
//! group      := "(" or ")"
//! modifier   := IDENT ":" value
//! value      := word | "\"" raw "\""
//! term       := word                    -- literal substring
//!             | "regex:" word            -- raw regex
//!             | wildcard                 -- contains * or ?
//! ```
//!
//! Reuse note: every later phase's parser additions (Phase 6's
//! `similar:`, Phase 7-9's content / audio modifiers, Phase 10's
//! cross-lens compositional sugar) extends this same AST rather than
//! forking.

use std::sync::Arc;

use regex::Regex;

use crate::quick_filters::QuickFilter;

/// Parsed top-level query. Holds the original input plus a (root)
/// AST. Holding the source string is convenient for diagnostics and
/// for the plan cache's hash key.
#[derive(Clone)]
pub struct Query {
    pub(crate) source: String,
    pub(crate) root: Arc<QueryNode>,
}

impl Query {
    pub fn source(&self) -> &str {
        &self.source
    }
    pub fn root(&self) -> &QueryNode {
        &self.root
    }
    pub(crate) fn new(source: String, root: QueryNode) -> Self {
        Self {
            source,
            root: Arc::new(root),
        }
    }
}

impl std::fmt::Debug for Query {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Query")
            .field("source", &self.source)
            .field("root", &self.root)
            .finish()
    }
}

#[derive(Debug, Clone)]
pub enum QueryNode {
    /// A name-side text predicate.
    Text(TextPattern),
    /// A modifier predicate (`size:` / `date:` / `ext:` / `attrib:` /
    /// `path:` / `parent:` / `child:`, plus the future-lens reservations).
    Modifier(ModifierPredicate),
    /// Quick filter alias, expanded to an extension set at execute time.
    QuickFilter(QuickFilter),
    /// Logical conjunction.
    And(Vec<QueryNode>),
    /// Logical disjunction.
    Or(Vec<QueryNode>),
    /// Logical negation.
    Not(Box<QueryNode>),
    /// Always-true atom — only produced by an empty group `()`.
    True,
}

#[derive(Debug, Clone)]
pub enum TextPattern {
    /// Literal substring (case folding handled at match-time per
    /// `MatchMode::match_case`).
    Literal(String),
    /// Wildcard — `*` matches any run of chars (incl. `/`),
    /// `?` matches a single char. Compiled to a regex at parse time
    /// for fast iteration.
    Wildcard { raw: String, compiled: Arc<Regex> },
    /// Raw regex — case-insensitivity is layered in at match-time.
    Regex { raw: String, compiled: Arc<Regex> },
}

#[derive(Debug, Clone)]
pub struct ModifierPredicate {
    pub kind: ModifierKind,
}

#[derive(Debug, Clone)]
pub enum ModifierKind {
    /// `size:>100mb`, `size:<1gb`, `size:=42`, `size:42` (bare = `==`).
    Size { op: SizeOp, bytes: u64 },
    /// `date:` predicate. Either an absolute / relative point with an
    /// implicit comparator (today / yesterday → equality on the day),
    /// or a bound (`>2024-01-01`).
    Date(DateBound),
    /// `ext:txt` or `ext:txt;rs;py` — semicolon-separated, lowercased.
    Ext(Vec<String>),
    /// `attrib:H`, `attrib:HRA` etc. — Everything's single-letter set.
    Attrib(Vec<AttribFlag>),
    /// `path:Documents` — substring on the canonicalised full path.
    Path(String),
    /// `parent:Projects` — substring on the immediate parent dir name.
    Parent(String),
    /// `child:src` — substring on the filename.
    Child(String),
    /// Future-lens reservations (Phase 6/7/9). Parsed for forward
    /// compatibility but the Phase-5 executor errors with
    /// `QueryError::UnsupportedModifier` when these run.
    Reserved { name: String, value: String },
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SizeOp {
    Lt,
    Le,
    Eq,
    Ge,
    Gt,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SizeUnit {
    Byte,
    Kilo,
    Mega,
    Giga,
    Tera,
}

impl SizeUnit {
    pub fn multiplier(self) -> u64 {
        match self {
            SizeUnit::Byte => 1,
            SizeUnit::Kilo => 1024,
            SizeUnit::Mega => 1024 * 1024,
            SizeUnit::Giga => 1024 * 1024 * 1024,
            SizeUnit::Tera => 1024u64 * 1024 * 1024 * 1024,
        }
    }
}

/// `date:` value, normalised at parse time. Comparisons happen against
/// `mtime_ns` at execute time.
#[derive(Debug, Clone)]
pub enum DateBound {
    /// Equality on a calendar day (matches Everything's `date:today`,
    /// `date:yesterday`, `date:2024-03-04`).
    Day { epoch_day: i64, op: SizeOp },
    /// Range from a relative anchor (today / yesterday / lastweek /
    /// thisweek / lastmonth / thismonth / lastyear / thisyear).
    Relative(RelativeDate),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RelativeDate {
    Today,
    Yesterday,
    ThisWeek,
    LastWeek,
    ThisMonth,
    LastMonth,
    ThisYear,
    LastYear,
}

/// File-attribute single-letters, modelled after Windows `attrib`. The
/// flag bit-positions mirror `dwFileAttributes` for the common subset
/// — Phase 5 uses them as a logical set; the journal subscribers
/// already map per-OS attribute bits onto the shared `attrs` field.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AttribFlag {
    /// `R` — read-only.
    ReadOnly,
    /// `H` — hidden.
    Hidden,
    /// `S` — system.
    System,
    /// `A` — archive.
    Archive,
    /// `D` — directory.
    Directory,
    /// `C` — compressed.
    Compressed,
    /// `E` — encrypted.
    Encrypted,
    /// `T` — temporary.
    Temporary,
    /// `O` — offline / cloud-only.
    Offline,
    /// `L` — symbolic link / reparse point.
    Reparse,
}

impl AttribFlag {
    pub fn from_letter(c: char) -> Option<Self> {
        match c.to_ascii_uppercase() {
            'R' => Some(AttribFlag::ReadOnly),
            'H' => Some(AttribFlag::Hidden),
            'S' => Some(AttribFlag::System),
            'A' => Some(AttribFlag::Archive),
            'D' => Some(AttribFlag::Directory),
            'C' => Some(AttribFlag::Compressed),
            'E' => Some(AttribFlag::Encrypted),
            'T' => Some(AttribFlag::Temporary),
            'O' => Some(AttribFlag::Offline),
            'L' => Some(AttribFlag::Reparse),
            _ => None,
        }
    }

    /// Bit position into the unified `attrs` field. Mirrors the
    /// Windows `FILE_ATTRIBUTE_*` constants for the common bits;
    /// macOS / Linux subscribers project their flags onto the same
    /// space so this stays portable.
    pub fn bit(self) -> u64 {
        match self {
            AttribFlag::ReadOnly => 0x0000_0001,
            AttribFlag::Hidden => 0x0000_0002,
            AttribFlag::System => 0x0000_0004,
            AttribFlag::Directory => 0x0000_0010,
            AttribFlag::Archive => 0x0000_0020,
            AttribFlag::Temporary => 0x0000_0100,
            AttribFlag::Reparse => 0x0000_0400,
            AttribFlag::Compressed => 0x0000_0800,
            AttribFlag::Offline => 0x0000_1000,
            AttribFlag::Encrypted => 0x0000_4000,
        }
    }
}
