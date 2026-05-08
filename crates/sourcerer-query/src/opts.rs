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
}

impl Default for SortSpec {
    fn default() -> Self {
        Self {
            field: SortField::Name,
            order: SortOrder::Asc,
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
