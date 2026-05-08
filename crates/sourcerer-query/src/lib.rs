//! Sourcerer query DSL — Phase 5 filename-lens surface.
//!
//! The Build Guide's Phase 5 prompt names three public symbols:
//!
//! ```ignore
//! pub struct Query { /* parsed AST */ }
//! pub fn parse(s: &str) -> Result<Query, ParseError>;
//! pub fn execute(idx: &Index, q: &Query, opts: ExecOpts)
//!     -> Result<ResultSet, QueryError>;
//! ```
//!
//! The grammar is a 1:1 shim over voidtools' Everything — literal /
//! wildcard / regex terms; `AND` / `OR` / `NOT` / `!` boolean glue;
//! `size:` / `date:` / `ext:` / `attrib:` / `path:` / `parent:` /
//! `child:` modifiers; the same quick-filter aliases (`audio:` /
//! `video:` / `image:` / `document:` / `executable:` / `archive:`).
//!
//! Lens routing happens at execute-time: Phase 5 implements only the
//! filename lens; Phase 7's content lens, Phase 9's audio lens, and
//! Phase 6's similarity lens parse the same DSL and dispatch from
//! [`Query::lens_route`].
//!
//! Standing Rule #8 — the Phase 5 grammar is a forever contract:
//! every query that parses today must keep parsing in every later
//! phase. The voidtools-syntax fixture in `tests/voidtools_compat.rs`
//! is the regression gate.

#![deny(rust_2018_idioms)]

pub mod ast;
pub mod cache;
pub mod error;
pub mod exec;
pub mod opts;
pub mod parser;
pub mod quick_filters;

pub use ast::{
    AttribFlag, DateBound, ModifierKind, ModifierPredicate, Query, QueryNode, RelativeDate, SizeOp,
    SizeUnit, TextPattern,
};
pub use cache::PlanCache;
pub use error::{ParseError, QueryError};
pub use exec::{ExecPlan, ExecStats, Hit, ResultSet, execute};
pub use opts::{ExecOpts, MatchMode, SortField, SortOrder, SortSpec};
pub use parser::parse;
pub use quick_filters::QuickFilter;
