//! Execution-side toggles: match-mode flags, sort spec, batch limits.

/// Per-query match-mode flags. Voidtools' Everything calls these
/// "Match Case", "Match Whole Word", "Match Path", and "Match
/// Diacritics"; the names map 1:1 here for the parity surface.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct MatchMode {
    pub match_case: bool,
    pub whole_word: bool,
    pub match_path: bool,
    pub match_diacritics: bool,
    /// SRC-M12 — let a latin (or jamo) term match a CJK name through
    /// its phonetic reading, so `wenjian` finds `文件`.
    ///
    /// Opt-in, and off by default, because it widens what a query can
    /// hit: with it on, `ni` matches every name containing `に`. The
    /// keys are indexed either way, so toggling this takes effect on
    /// the next query rather than requiring a reindex.
    pub match_phonetic: bool,
    /// SRC-M23 — drop punctuation from both sides before comparing, so
    /// `foobar` finds `foo-bar` and `foo_bar` finds `foobar`.
    ///
    /// "Punctuation" is Unicode's own definition plus symbols, so it
    /// covers `-`, `_`, `.`, `'` and their non-ASCII equivalents without
    /// a hand-maintained list.
    pub ignore_punctuation: bool,
    /// SRC-M23 — drop whitespace from both sides before comparing, so
    /// `myreport` finds `my report`.
    pub ignore_whitespace: bool,
}

impl MatchMode {
    /// True when a flag rewrites the text before comparing, which means
    /// the trigram seed built over raw names can no longer be trusted to
    /// contain the matching rows.
    ///
    /// `foo-bar` indexes the trigrams `foo`, `oo-`, `o-b`, `-ba`, `bar`.
    /// The needle `foobar` asks for `oob` and `oba`, which that row does
    /// not have — so seeding from trigrams would return nothing at all,
    /// silently, which is the failure mode SRC-M12 hit with phonetic
    /// readings. The executor falls back to a full scan instead; these
    /// flags are opt-in and off by default, so nobody pays for it
    /// without asking.
    pub fn rewrites_text(&self) -> bool {
        self.ignore_punctuation || self.ignore_whitespace
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SortField {
    Name,
    Path,
    Size,
    Date,
    Type,
    Ext,
    /// Phase-6 similarity lens: order by Jaccard estimate descending.
    /// Only meaningful when the active query carries a `similar:`
    /// modifier; on non-similarity queries the executor falls through
    /// to `Name` ordering.
    Relevance,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SortOrder {
    Asc,
    Desc,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SortSpec {
    pub field: SortField,
    pub order: SortOrder,
    /// SRC-M24 — read digit runs as numbers, so `file2` precedes
    /// `file10`. On by default because byte ordering on numbered files
    /// is the wrong answer often enough that Everything made natural
    /// sort its default too; Settings → Results turns it off for anyone
    /// who wants raw ordering back.
    pub natural: bool,
}

impl Default for SortSpec {
    fn default() -> Self {
        Self {
            field: SortField::Name,
            order: SortOrder::Asc,
            natural: true,
        }
    }
}

/// Caller-controlled execution knobs.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ExecOpts {
    pub match_mode: MatchMode,
    pub sort: SortSpec,
    /// Cap on the candidate set drawn from the trigram pre-filter.
    /// `0` means "no cap"; the default keeps the executor honest on
    /// pathological prefix queries.
    pub candidate_cap: usize,
    /// First-batch hint: the executor returns at least this many hits
    /// before yielding to the caller. Build-Guide bench gate is
    /// "first batch within 16ms" — the default sits at the Everything
    /// UI default of 32.
    pub first_batch: usize,
    /// Hard cap on the result set the executor will return.
    pub limit: usize,
}

impl Default for ExecOpts {
    fn default() -> Self {
        Self {
            match_mode: MatchMode::default(),
            sort: SortSpec::default(),
            candidate_cap: 100_000,
            first_batch: 32,
            limit: 1_000,
        }
    }
}
