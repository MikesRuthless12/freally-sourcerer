//! Phase-5 filename-lens executor.
//!
//! Pipeline:
//!
//! 1. Plan the query (extract a trigram seed; classify which atoms are
//!    name-only vs. need full `FileRow` hydration).
//! 2. Stream `(file_id, name_lower)` candidates from the custom name
//!    index — either the trigram pre-filter or the live-row scan
//!    fall-back when there's no usable seed.
//! 3. Apply name-only predicates (literal / wildcard / regex) and the
//!    quick-filter extension test that doesn't need SQLite.
//! 4. Hydrate the survivors via `Store::get_many` (one statement per
//!    ~250 file_ids).
//! 5. Apply the rest of the predicates (size / date / path / parent /
//!    child / attrib / `ext:` modifier).
//! 6. Sort by `SortSpec`.
//!
//! `ResultSet` exposes both batch APIs (`first_batch` for the 16 ms
//! gate, `collect` for the tail) and an iterator. Phase-11's UI will
//! adopt streaming directly.

use std::cmp::Ordering;
use std::collections::HashMap;

use freally_audio::{AudioAttributes, AudioAttributesProvider};
use freally_index::{FileRow, Index};
use freally_similarity::{SimilarityIndex, SimilarityOpts};
use unicode_normalization::UnicodeNormalization;

use crate::ast::{
    AudioPredicate, DateBound, LensKind, ModifierKind, Query, QueryNode, SizeOp, TextPattern,
};
use crate::error::QueryError;
use crate::opts::{ExecOpts, MatchMode, SortField, SortOrder, SortSpec};
use crate::parser;

const NS_PER_DAY: i64 = 86_400 * 1_000_000_000;

/// Plan summary — what the optimizer chose. Cached by the plan cache
/// (`PlanCache`) and surfaced in `ExecStats`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ExecPlan {
    /// Trigram seed (lowercase). Empty string means "no usable seed —
    /// scan every live row".
    pub seed: String,
    /// True if any predicate beyond name-only matching needs SQLite.
    pub needs_hydration: bool,
}

#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct ExecStats {
    pub candidates: usize,
    pub name_survivors: usize,
    pub final_hits: usize,
    pub used_seed: bool,
}

#[derive(Debug, Clone)]
pub struct Hit {
    pub row: FileRow,
}

#[derive(Debug)]
pub struct ResultSet {
    rows: Vec<FileRow>,
    cursor: usize,
    pub plan: ExecPlan,
    pub stats: ExecStats,
}

impl ResultSet {
    pub fn total(&self) -> usize {
        self.rows.len()
    }

    pub fn rows(&self) -> &[FileRow] {
        &self.rows
    }

    /// First-batch helper — returns up to `n` results then advances
    /// the cursor. Subsequent calls drain the tail. Phase 11's UI uses
    /// this to render the 16ms first-batch gate while streaming the
    /// rest into the same list.
    pub fn first_batch(&mut self, n: usize) -> Vec<FileRow> {
        let end = (self.cursor + n).min(self.rows.len());
        let out = self.rows[self.cursor..end].to_vec();
        self.cursor = end;
        out
    }

    /// Drain everything past the current cursor — pairs with
    /// [`first_batch`] for the streaming pattern. The unread tail is
    /// what the caller usually wants; `into_all_rows` is the escape
    /// hatch when the cursor doesn't matter.
    pub fn collect(mut self) -> Vec<FileRow> {
        let tail = self.rows.split_off(self.cursor);
        self.cursor = self.rows.len();
        tail
    }

    /// Return every hit regardless of cursor position. Equivalent to
    /// `rs.rows().to_vec()` minus the clone.
    pub fn into_all_rows(self) -> Vec<FileRow> {
        self.rows
    }
}

impl Iterator for ResultSet {
    type Item = FileRow;
    fn next(&mut self) -> Option<FileRow> {
        if self.cursor >= self.rows.len() {
            return None;
        }
        let r = self.rows[self.cursor].clone();
        self.cursor += 1;
        Some(r)
    }
}

/// Build an [`ExecPlan`] for a parsed query. Pure — no Index access —
/// so it can be cached (Build-Guide spec: 16-entry plan cache).
pub fn plan(q: &Query, _opts: &ExecOpts) -> ExecPlan {
    let seed = pick_seed(q.root());
    ExecPlan {
        seed,
        needs_hydration: needs_hydration(q.root()),
    }
}

fn needs_hydration(node: &QueryNode) -> bool {
    match node {
        QueryNode::Modifier(m) => match &m.kind {
            ModifierKind::Size { .. }
            | ModifierKind::Date(_)
            | ModifierKind::Path(_)
            | ModifierKind::Parent(_)
            | ModifierKind::Attrib(_)
            | ModifierKind::Ext(_) => true,
            ModifierKind::Child(_) => false,
            ModifierKind::Similar(_) => false,
            // Audio predicates need the FileRow's path + mtime_ns to
            // hit the AudioAttributesProvider.
            ModifierKind::Audio(_) => true,
            ModifierKind::Reserved { .. } => true,
        },
        // Quick filter shortcuts to `ext:` so it doesn't need full
        // hydration when used alone — the name index has the lower
        // name including its extension.
        QueryNode::QuickFilter(_) => false,
        QueryNode::Text(_) => false,
        QueryNode::True => false,
        QueryNode::Not(inner) => needs_hydration(inner),
        QueryNode::And(parts) | QueryNode::Or(parts) => parts.iter().any(needs_hydration),
        // Lens scopes are transparent for hydration — recurse into
        // the inner sub-query. `Content` is rejected at
        // `validate_supported`, so we never reach this for it.
        QueryNode::Lens { inner, .. } => needs_hydration(inner),
    }
}

/// Pick the longest literal substring (lowercased, ASCII-folded) we
/// can use as a trigram seed. Wildcards / regexes don't contribute —
/// the executor falls through to the live-row scan when none is found.
fn pick_seed(node: &QueryNode) -> String {
    fn collect(node: &QueryNode, out: &mut Vec<String>) {
        match node {
            QueryNode::Text(TextPattern::Literal(l)) => out.push(l.to_lowercase()),
            QueryNode::Text(_) => {}
            QueryNode::And(parts) => parts.iter().for_each(|p| collect(p, out)),
            QueryNode::Or(_) => {
                // OR breaks the seed monotonicity — bail on a seed and
                // let the executor scan. A smarter optimizer (Phase 13)
                // unions per-disjunct candidate sets; Phase 5 keeps it
                // simple to ship the gate.
            }
            QueryNode::Modifier(m) => {
                if let ModifierKind::Child(c) = &m.kind {
                    out.push(c.to_lowercase());
                }
            }
            QueryNode::QuickFilter(_) | QueryNode::True | QueryNode::Not(_) => {}
            // Lens scopes are transparent for seed picking — the
            // inner literal (if any) still drives trigram routing.
            QueryNode::Lens { inner, .. } => collect(inner, out),
        }
    }
    let mut cands = Vec::new();
    collect(node, &mut cands);
    cands
        .into_iter()
        .max_by_key(|s| s.len())
        .unwrap_or_default()
}

/// Run a parsed query against an open index. The Build Guide's
/// Phase-5 contract: emit the first 32 hits within 16ms on a 5M-file
/// dataset; stream the tail in the same `ResultSet` until `limit`.
///
/// This entry-point does not provide a similarity index. Queries that
/// reference a `similar:` modifier surface
/// `QueryError::SimilarityIndexUnavailable` so callers see a typed
/// error rather than empty results. Use [`execute_with`] to wire a
/// `SimilarityIndex` in. Audio-bearing queries surface
/// `QueryError::AudioProviderUnavailable` for the same reason; use
/// [`execute_with_audio`] to supply an `AudioAttributesProvider`.
pub fn execute(idx: &Index, q: &Query, opts: ExecOpts) -> Result<ResultSet, QueryError> {
    execute_with(idx, None, q, opts)
}

/// Run a parsed query with an optional similarity-index reference.
/// Mirrors [`execute`] for the filename-only case; routes any query
/// containing a top-level `similar:` modifier through the supplied
/// `SimilarityIndex`. Phase 6's surface — Phase 11's UI calls this
/// directly with `Some(sim_idx)` so the magic-moment lens grouping
/// works.
pub fn execute_with(
    idx: &Index,
    similarity: Option<&SimilarityIndex>,
    q: &Query,
    opts: ExecOpts,
) -> Result<ResultSet, QueryError> {
    execute_with_audio(idx, similarity, None, q, opts)
}

/// Phase-9 entry point. Adds an optional [`AudioAttributesProvider`]
/// so audio-bearing queries (`lufs:` / `codec:` / `length:` / `rate:` /
/// `silence:` / `dr:`) filter against the audio cache. When the AST
/// has no audio modifier the parameter is ignored — there is no
/// performance penalty for a non-audio query.
pub fn execute_with_audio(
    idx: &Index,
    similarity: Option<&SimilarityIndex>,
    audio: Option<&dyn AudioAttributesProvider>,
    q: &Query,
    opts: ExecOpts,
) -> Result<ResultSet, QueryError> {
    validate_supported(q)?;
    // Phase 10: optimize the AST before planning so the executor's
    // AND iter().all() short-circuit picks up the cheap predicates
    // first. The original `q` is not mutated; the executor uses the
    // optimized clone for the rest of the pipeline.
    let optimized = crate::optimizer::optimize(q);
    let q = &optimized;
    let needs_audio = has_audio_anywhere(q.root());
    if needs_audio && audio.is_none() {
        return Err(QueryError::AudioProviderUnavailable);
    }
    if let Some(needle) = top_level_similar(q.root()) {
        return execute_similar(idx, similarity, audio, q, &opts, needle);
    }
    if has_similar_anywhere(q.root()) {
        // Phase 6 only routes Similar in the root or as a direct child
        // of a top-level AND. Anywhere else (NOT, OR, deeper nesting)
        // is rejected loudly so the UI can surface the limitation.
        return Err(QueryError::UnsupportedSimilarPosition);
    }
    let plan = plan(q, &opts);
    // `match_path` widens the search target from the lowercased
    // filename to the full path. The name index only has filenames, so
    // a trigram seed extracted from the query text would silently miss
    // path-only hits ("projects" hitting `/synth/projects/alpha.md`).
    // Phase 13 adds a path-trigram side index; for Phase 5 we drop down
    // to the live-row scan when the toggle is on. We deliberately do
    // NOT mutate `plan.seed` here — the plan is shared via `PlanCache`
    // and must stay invariant under the query string alone, so that
    // toggling `match_path` between two callers with the same query
    // doesn't poison each other's cached plan.
    let use_seed = !plan.seed.is_empty() && !opts.match_mode.match_path;
    let mut survivors_ids: Vec<u64> = Vec::new();
    let mut survivors_names: Vec<String> = Vec::new();
    let mut stats = ExecStats {
        used_seed: use_seed,
        ..ExecStats::default()
    };

    let cap = if opts.candidate_cap == 0 {
        usize::MAX
    } else {
        opts.candidate_cap
    };

    let evaluator = NameEvaluator::new(q.root(), &opts);

    // Phase 10 lens routing: an audio-only / similarity-only query
    // has no name-side predicate to filter by, so the per-row name
    // evaluation is a no-op. We skip it entirely — the per-row test
    // would still return `true`, but the call cost is non-zero. The
    // optimizer's `is_audio_only_route` hands us this hint.
    let skip_name_filter =
        opts.match_mode.match_path || crate::optimizer::is_audio_only_route(q.root());

    if use_seed {
        idx.name_index()
            .for_each_candidate_named(&plan.seed, cap, |fid, name| {
                stats.candidates += 1;
                if skip_name_filter || evaluator.matches(name) {
                    survivors_ids.push(fid);
                    survivors_names.push(String::from_utf8_lossy(name).into_owned());
                }
            });
    } else {
        let mut emitted = 0usize;
        idx.name_index().for_each_live(|fid, name| {
            stats.candidates += 1;
            if emitted >= cap {
                return false;
            }
            if skip_name_filter || evaluator.matches(name) {
                survivors_ids.push(fid);
                survivors_names.push(String::from_utf8_lossy(name).into_owned());
                emitted += 1;
            }
            true
        });
    }
    stats.name_survivors = survivors_ids.len();

    // Hydrate via SQLite. Required when any predicate beyond name-only
    // matching applies (size / date / path / parent / attrib / ext /
    // audio modifier) or when `match_path` widens the target to the
    // full path.
    let needs_full = plan.needs_hydration || opts.match_mode.match_path;
    let i64_ids: Vec<i64> = survivors_ids.iter().map(|&u| u as i64).collect();
    let mut rows: Vec<FileRow> = idx.store().get_many(&i64_ids)?;
    if needs_full {
        // Phase 9: collect audio rows that survive the non-audio
        // predicates first, then loop one more time to apply audio
        // predicates. Splitting the work keeps the hot path
        // (filename-only queries) free of audio-cache lookups, and
        // means audio-only queries pay one cache lookup per surviving
        // row rather than per-candidate.
        rows = filter_with_audio(rows, q.root(), &opts.match_mode, audio, needs_audio)?;
    }

    sort_rows(&mut rows, opts.sort);
    if opts.limit > 0 && rows.len() > opts.limit {
        rows.truncate(opts.limit);
    }
    stats.final_hits = rows.len();

    Ok(ResultSet {
        rows,
        cursor: 0,
        plan,
        stats,
    })
}

fn sort_rows(rows: &mut [FileRow], spec: SortSpec) {
    let cmp = |a: &FileRow, b: &FileRow| -> Ordering {
        match spec.field {
            // `Relevance` is only meaningful inside the similarity-lens
            // path (which sorts by Jaccard before calling here). On the
            // generic Phase-5 path it degrades to Name — matches the
            // Phase 11 UI's "Sort by Relevance" fallback for non-
            // similarity queries.
            SortField::Name | SortField::Relevance => a.name_lower.cmp(&b.name_lower),
            SortField::Path => a.path.cmp(&b.path),
            SortField::Size => a.size.cmp(&b.size),
            SortField::Date => a.mtime_ns.cmp(&b.mtime_ns),
            // Phase 5 collapses voidtools' "Type" (display-name from
            // the OS file-association — `Folder`, `JPEG image`, …)
            // onto raw extension. Phase 11's settings + extractor
            // registry restore the distinction; until then both sort
            // keys behave identically and the UI must label the two
            // entries separately for parity with Everything.
            SortField::Type | SortField::Ext => a.ext.cmp(&b.ext),
        }
    };
    match spec.order {
        SortOrder::Asc => rows.sort_by(cmp),
        SortOrder::Desc => rows.sort_by(|a, b| cmp(a, b).reverse()),
    }
}

struct NameEvaluator<'a> {
    root: &'a QueryNode,
    opts: &'a ExecOpts,
}

impl<'a> NameEvaluator<'a> {
    fn new(root: &'a QueryNode, opts: &'a ExecOpts) -> Self {
        Self { root, opts }
    }

    /// Name-side eval. The bytes are the lowercased filename from the
    /// name index. Modifiers that need SQLite return true (the full
    /// pass filters them out later).
    fn matches(&self, name_lower: &[u8]) -> bool {
        eval_name(self.root, name_lower, &self.opts.match_mode)
    }
}

/// Apply the post-hydration predicate filter, including (when
/// `needs_audio` is true) per-row audio-attribute lookups.
fn filter_with_audio(
    rows: Vec<FileRow>,
    root: &QueryNode,
    mm: &MatchMode,
    audio: Option<&dyn AudioAttributesProvider>,
    needs_audio: bool,
) -> Result<Vec<FileRow>, QueryError> {
    let mut out = Vec::with_capacity(rows.len());
    for r in rows {
        let path_lower = if mm.match_path {
            Some(r.path.to_string_lossy().to_lowercase())
        } else {
            None
        };
        let attrs: Option<AudioAttributes> = if needs_audio {
            // We pre-validated `audio.is_some()` at the top of
            // `execute_with_audio`, so the unwrap is structurally
            // safe.
            let provider = audio.expect("audio provider checked at entry");
            provider.get(&r.path, r.mtime_ns)?
        } else {
            None
        };
        if eval_full(root, &r, mm, path_lower.as_deref(), attrs.as_ref()) {
            out.push(r);
        }
    }
    Ok(out)
}

fn eval_name(node: &QueryNode, name_lower: &[u8], mm: &MatchMode) -> bool {
    match node {
        QueryNode::True => true,
        QueryNode::Text(p) => match_text(p, name_lower, mm),
        QueryNode::Not(inner) => !eval_name(inner, name_lower, mm),
        QueryNode::And(parts) => parts.iter().all(|p| eval_name(p, name_lower, mm)),
        QueryNode::Or(parts) => parts.iter().any(|p| eval_name(p, name_lower, mm)),
        QueryNode::Modifier(m) => match &m.kind {
            ModifierKind::Child(needle) => substring_match(name_lower, needle, mm),
            // Modifiers we can pre-filter by extension/name from the
            // lowercase name buffer. They still re-evaluate at the
            // full-record stage when hydration reads the canonical
            // value.
            ModifierKind::Ext(exts) => name_has_any_ext(name_lower, exts),
            // Everything else can't be decided on the name alone — let
            // it through and re-evaluate post-hydration.
            _ => true,
        },
        QueryNode::QuickFilter(qf) => name_has_any_ext(name_lower, qf.extensions()),
        QueryNode::Lens { inner, .. } => eval_name(inner, name_lower, mm),
    }
}

fn eval_full(
    node: &QueryNode,
    row: &FileRow,
    mm: &MatchMode,
    path_lower: Option<&str>,
    audio: Option<&AudioAttributes>,
) -> bool {
    match node {
        QueryNode::True => true,
        QueryNode::Text(p) => {
            let target = match path_lower {
                Some(pl) => pl,
                None => row.name_lower.as_str(),
            };
            match_text(p, target.as_bytes(), mm)
        }
        QueryNode::Not(inner) => !eval_full(inner, row, mm, path_lower, audio),
        QueryNode::And(parts) => parts
            .iter()
            .all(|p| eval_full(p, row, mm, path_lower, audio)),
        QueryNode::Or(parts) => parts
            .iter()
            .any(|p| eval_full(p, row, mm, path_lower, audio)),
        QueryNode::Modifier(m) => eval_modifier(&m.kind, row, audio),
        QueryNode::QuickFilter(qf) => row
            .ext
            .as_deref()
            .map(|e| qf.extensions().iter().any(|x| x.eq_ignore_ascii_case(e)))
            .unwrap_or(false),
        QueryNode::Lens { inner, .. } => eval_full(inner, row, mm, path_lower, audio),
    }
}

fn eval_modifier(kind: &ModifierKind, row: &FileRow, audio: Option<&AudioAttributes>) -> bool {
    match kind {
        ModifierKind::Size { op, bytes } => cmp_op(*op, row.size, *bytes),
        ModifierKind::Date(b) => eval_date(b, row.mtime_ns),
        ModifierKind::Ext(exts) => row
            .ext
            .as_deref()
            .map(|e| exts.iter().any(|x| x.eq_ignore_ascii_case(e)))
            .unwrap_or(false),
        ModifierKind::Attrib(flags) => {
            let mask: u64 = flags.iter().copied().fold(0u64, |m, f| m | f.bit());
            row.attrs & mask == mask
        }
        ModifierKind::Path(needle) => {
            let p = row.path.to_string_lossy().to_lowercase();
            p.contains(&needle.to_lowercase())
        }
        ModifierKind::Parent(needle) => row
            .path
            .parent()
            .and_then(|p| p.file_name())
            .and_then(|s| s.to_str())
            .map(|s| s.to_lowercase().contains(&needle.to_lowercase()))
            .unwrap_or(false),
        ModifierKind::Child(needle) => row.name_lower.contains(&needle.to_lowercase()),
        ModifierKind::Similar(_) => {
            // `execute_with` routes Similar-bearing queries through the
            // similarity-lens path before reaching here. Hitting this
            // arm means a caller bypassed `execute_with` and went
            // straight to `eval_modifier` — fail loud, the same way the
            // Reserved arm does.
            debug_assert!(
                false,
                "similar: modifier reached eval_modifier — caller skipped execute_with"
            );
            false
        }
        ModifierKind::Audio(pred) => match audio {
            Some(attrs) => eval_audio_predicate(pred, attrs),
            // No cached audio attributes — either the row's path
            // isn't audio, or the cache miss returned None
            // (e.g. extractor disabled). Either way, the predicate
            // doesn't match this row.
            None => false,
        },
        ModifierKind::Reserved { name, .. } => {
            // `validate_supported` runs at the top of `execute()` and
            // turns Reserved modifiers into `QueryError::Unsupported-
            // Modifier` before evaluation begins. Reaching this arm
            // means a caller bypassed the gate (only possible if they
            // build a Query AST by hand) — fail loud.
            debug_assert!(
                false,
                "reserved modifier `{name}` reached eval_modifier — caller skipped validate_supported"
            );
            false
        }
    }
}

/// Resolve a single audio-modifier predicate against a row's
/// extracted attributes. Pure — no I/O, no cache access.
fn eval_audio_predicate(pred: &AudioPredicate, attrs: &AudioAttributes) -> bool {
    match pred {
        AudioPredicate::Lufs { op, lufs } => cmp_op_f32(*op, attrs.lufs_integrated, *lufs),
        AudioPredicate::Codec(needles) => needles
            .iter()
            .any(|n| attrs.codec.eq_ignore_ascii_case(n.as_str())),
        AudioPredicate::Length { op, seconds } => cmp_op_f32(*op, attrs.length_seconds(), *seconds),
        AudioPredicate::Rate { op, hz } => cmp_op(*op, attrs.sample_rate, *hz),
        AudioPredicate::Silence { op, ratio } => cmp_op_f32(*op, attrs.silence_ratio, *ratio),
        AudioPredicate::DynamicRange { op, lu } => cmp_op_f32(*op, attrs.dynamic_range_lu, *lu),
    }
}

/// `f32`-aware comparator for audio modifiers. `Eq` uses an absolute
/// epsilon of `1e-3` so a user-typed value like `lufs:-23` matches a
/// computed `-23.000123` cleanly. The epsilon also smooths the
/// percent-of-silence path (`silence:=0.5` matches `[0.499, 0.501]`).
/// `NaN` on either side returns `false` instead of trapping; non-
/// finite values flow through the strict ordering arms unchanged
/// (`-inf > x` is `false` for any finite `x`, which is the desired
/// behavior for sub-3-second clips whose short-term percentiles
/// surface as `NEG_INFINITY`).
pub(crate) const AUDIO_EQ_EPSILON: f32 = 1e-3;

fn cmp_op_f32(op: SizeOp, a: f32, b: f32) -> bool {
    if a.is_nan() || b.is_nan() {
        return false;
    }
    match op {
        SizeOp::Lt => a < b,
        SizeOp::Le => a <= b,
        SizeOp::Eq => (a - b).abs() < AUDIO_EQ_EPSILON,
        SizeOp::Ge => a >= b,
        SizeOp::Gt => a > b,
    }
}

fn eval_date(bound: &DateBound, mtime_ns: i64) -> bool {
    match bound {
        DateBound::Day { epoch_day, op } => {
            let row_day = mtime_ns.div_euclid(NS_PER_DAY);
            cmp_op(*op, row_day, *epoch_day)
        }
        DateBound::Relative(rd) => {
            let (start, end) = parser::relative_day_range(*rd);
            let row_day = mtime_ns.div_euclid(NS_PER_DAY);
            row_day >= start && row_day < end
        }
    }
}

fn cmp_op<T: Ord>(op: SizeOp, a: T, b: T) -> bool {
    match op {
        SizeOp::Lt => a < b,
        SizeOp::Le => a <= b,
        SizeOp::Eq => a == b,
        SizeOp::Ge => a >= b,
        SizeOp::Gt => a > b,
    }
}

fn name_has_any_ext(name_lower: &[u8], exts: &[impl AsRef<str>]) -> bool {
    let s = match std::str::from_utf8(name_lower) {
        Ok(s) => s,
        Err(_) => return false,
    };
    let dot = match s.rfind('.') {
        Some(i) => i,
        None => return false,
    };
    let ext = &s[dot + 1..];
    exts.iter().any(|e| ext.eq_ignore_ascii_case(e.as_ref()))
}

fn match_text(pattern: &TextPattern, target: &[u8], mm: &MatchMode) -> bool {
    let target_str = match std::str::from_utf8(target) {
        Ok(s) => s,
        Err(_) => return false,
    };
    let folded_target;
    let target_eff: &str = if mm.match_diacritics {
        target_str
    } else {
        folded_target = strip_diacritics(target_str);
        &folded_target
    };
    match pattern {
        TextPattern::Literal(needle) => {
            let needle_eff: String = if mm.match_diacritics {
                if mm.match_case {
                    needle.clone()
                } else {
                    needle.to_lowercase()
                }
            } else {
                strip_diacritics(needle)
            };
            literal_match(target_eff, &needle_eff, mm)
        }
        TextPattern::Wildcard { compiled, .. } => compiled.is_match(target_eff),
        TextPattern::Regex { compiled, .. } => {
            if mm.match_case {
                compiled.is_match(target_eff)
            } else {
                // Re-run case-insensitively by relying on regex's own
                // (?i) prefix when the user didn't supply one. We don't
                // mutate the cached compiled regex — instead we lower
                // both sides and run a new match.
                let ci_target = target_eff.to_lowercase();
                compiled.is_match(&ci_target)
            }
        }
    }
}

fn literal_match(target_lower_or_cased: &str, needle: &str, mm: &MatchMode) -> bool {
    let target_eff = if mm.match_case {
        target_lower_or_cased
    } else {
        // The name index already lowercased; ensure needle matches.
        target_lower_or_cased
    };
    let needle_lc = if mm.match_case {
        needle.to_string()
    } else {
        needle.to_lowercase()
    };
    if mm.whole_word {
        whole_word_contains(target_eff, &needle_lc)
    } else {
        target_eff.contains(&needle_lc)
    }
}

fn whole_word_contains(haystack: &str, needle: &str) -> bool {
    let mut start = 0usize;
    while let Some(pos) = haystack[start..].find(needle) {
        let abs = start + pos;
        let before_ok = abs == 0
            || !haystack[..abs]
                .chars()
                .next_back()
                .map(is_word_char)
                .unwrap_or(false);
        let end = abs + needle.len();
        let after_ok = end == haystack.len()
            || !haystack[end..]
                .chars()
                .next()
                .map(is_word_char)
                .unwrap_or(false);
        if before_ok && after_ok {
            return true;
        }
        start = abs + needle.chars().next().map(|c| c.len_utf8()).unwrap_or(1);
    }
    false
}

fn is_word_char(c: char) -> bool {
    c.is_alphanumeric() || c == '_'
}

fn substring_match(target_lower: &[u8], needle: &str, mm: &MatchMode) -> bool {
    let s = match std::str::from_utf8(target_lower) {
        Ok(s) => s,
        Err(_) => return false,
    };
    let needle_lc = needle.to_lowercase();
    let target_eff = if mm.match_diacritics {
        s.to_string()
    } else {
        strip_diacritics(s)
    };
    let needle_eff = if mm.match_diacritics {
        needle_lc
    } else {
        strip_diacritics(&needle_lc)
    };
    target_eff.contains(&needle_eff)
}

/// NFKD-decompose, drop combining marks, recompose. Cheap diacritic
/// stripper — not perfect for every script (Phase 5 perf-pass note)
/// but matches Everything's "Match Diacritics" toggle behavior.
fn strip_diacritics(s: &str) -> String {
    s.nfkd().filter(|c| !is_combining_mark(*c)).collect()
}

fn is_combining_mark(c: char) -> bool {
    matches!(c as u32,
        0x0300..=0x036F | 0x1AB0..=0x1AFF | 0x1DC0..=0x1DFF |
        0x20D0..=0x20FF | 0xFE20..=0xFE2F)
}

/// Up-front gate: refuse to execute a query that names a Phase-5
/// reserved modifier so callers see a typed error instead of empty
/// results.
pub fn validate_supported(q: &Query) -> Result<(), QueryError> {
    fn walk(node: &QueryNode) -> Result<(), QueryError> {
        match node {
            QueryNode::Modifier(m) => {
                if let ModifierKind::Reserved { name, .. } = &m.kind {
                    return Err(QueryError::UnsupportedModifier(name.clone()));
                }
                Ok(())
            }
            QueryNode::Not(inner) => walk(inner),
            QueryNode::And(parts) | QueryNode::Or(parts) => {
                for p in parts {
                    walk(p)?;
                }
                Ok(())
            }
            QueryNode::Text(_) | QueryNode::QuickFilter(_) | QueryNode::True => Ok(()),
            // Phase 10 ships parse-time support for `name:(...)` /
            // `audio:(...)` / `content:(...)` / `similar:(...)`. The
            // executor today treats `Name` / `Audio` / `Similar` as
            // transparent wrappers (the inner predicates still
            // dispatch through Phase 5 / 6 / 9 paths). `Content` has
            // no executor — Phase 8 ships the content extractors, but
            // Phase 11+ wires the lens routing into the daemon. Until
            // then we surface a typed `UnsupportedModifier("content")`
            // so the UI can render a clear "content lens not yet
            // available" hint instead of returning empty results.
            QueryNode::Lens { kind, inner } => {
                if matches!(kind, LensKind::Content) {
                    return Err(QueryError::UnsupportedModifier("content".into()));
                }
                walk(inner)
            }
        }
    }
    walk(q.root())
}

/// Extract a `similar:` needle that appears at the root or as a direct
/// child of a top-level AND. Returns `None` if no `Similar` modifier is
/// reachable that way (callers then check `has_similar_anywhere` to
/// decide between "Phase-5 path" and "buried Similar — reject").
fn top_level_similar(node: &QueryNode) -> Option<&str> {
    match node {
        QueryNode::Modifier(m) => match &m.kind {
            ModifierKind::Similar(s) => Some(s.as_str()),
            _ => None,
        },
        QueryNode::And(parts) => {
            for p in parts {
                if let QueryNode::Modifier(m) = p
                    && let ModifierKind::Similar(s) = &m.kind
                {
                    return Some(s.as_str());
                }
            }
            None
        }
        // A top-level `similar:(...)` lens with a Text inner is
        // treated like the `similar:<needle>` modifier so the lens-
        // prefix syntax stays useful at execute time. `similar:("foo
        // bar")` (quoted) routes the same way.
        QueryNode::Lens {
            kind: LensKind::Similar,
            inner,
        } => match inner.as_ref() {
            QueryNode::Text(TextPattern::Literal(s)) => Some(s.as_str()),
            _ => None,
        },
        _ => None,
    }
}

fn has_similar_anywhere(node: &QueryNode) -> bool {
    match node {
        QueryNode::Modifier(m) => matches!(m.kind, ModifierKind::Similar(_)),
        QueryNode::Not(inner) => has_similar_anywhere(inner),
        QueryNode::And(parts) | QueryNode::Or(parts) => parts.iter().any(has_similar_anywhere),
        QueryNode::Text(_) | QueryNode::QuickFilter(_) | QueryNode::True => false,
        QueryNode::Lens { kind, inner } => {
            // A `similar:(...)` lens itself counts as "anywhere"; for
            // other lens kinds, recurse into the inner.
            matches!(kind, LensKind::Similar) || has_similar_anywhere(inner)
        }
    }
}

/// Phase-9 walk — does the AST contain any audio modifier? Drives the
/// "should the executor look up audio attributes per row?" branch.
fn has_audio_anywhere(node: &QueryNode) -> bool {
    match node {
        QueryNode::Modifier(m) => matches!(m.kind, ModifierKind::Audio(_)),
        QueryNode::Not(inner) => has_audio_anywhere(inner),
        QueryNode::And(parts) | QueryNode::Or(parts) => parts.iter().any(has_audio_anywhere),
        QueryNode::Text(_) | QueryNode::QuickFilter(_) | QueryNode::True => false,
        // `audio:(...)` lens scopes contribute their inner audio
        // modifiers; non-audio lenses still recurse so a buried
        // audio modifier under `name:(...)` etc. is detected.
        QueryNode::Lens { inner, .. } => has_audio_anywhere(inner),
    }
}

/// Phase-6 similarity-lens execution path. Replaces the Phase-5
/// trigram pre-filter with an LSH lookup against the supplied
/// `SimilarityIndex`. The remaining predicates (size / date / path /
/// parent / attrib / ext / child / quick-filter / regex / wildcard /
/// literal / audio) still apply post-hydration so a query like
/// `similar:report-final ext:pdf` or `similar:bassdrop codec:flac
/// length:>3:00` filters down correctly.
fn execute_similar(
    idx: &Index,
    similarity: Option<&SimilarityIndex>,
    audio: Option<&dyn AudioAttributesProvider>,
    q: &Query,
    opts: &ExecOpts,
    needle: &str,
) -> Result<ResultSet, QueryError> {
    let sim = similarity.ok_or(QueryError::SimilarityIndexUnavailable)?;
    let cap = if opts.candidate_cap == 0 {
        usize::MAX
    } else {
        opts.candidate_cap
    };
    let sim_opts = SimilarityOpts {
        candidate_cap: cap.min(usize::MAX),
        ..SimilarityOpts::default()
    };
    let hits = sim.candidates(&needle.to_lowercase(), &sim_opts);
    let mut stats = ExecStats {
        candidates: hits.len(),
        used_seed: !needle.is_empty(),
        ..ExecStats::default()
    };

    let mut jaccard_by_id: HashMap<i64, f32> = HashMap::with_capacity(hits.len());
    let mut ordered_ids: Vec<i64> = Vec::with_capacity(hits.len());
    for h in hits {
        let i_id = h.file_id as i64;
        if jaccard_by_id.insert(i_id, h.jaccard).is_none() {
            ordered_ids.push(i_id);
        }
    }
    let mut rows: Vec<FileRow> = idx.store().get_many(&ordered_ids)?;
    stats.name_survivors = rows.len();

    let needs_audio = has_audio_anywhere(q.root());
    let mut filtered = Vec::with_capacity(rows.len());
    for r in rows.drain(..) {
        let path_lower = if opts.match_mode.match_path {
            Some(r.path.to_string_lossy().to_lowercase())
        } else {
            None
        };
        let attrs: Option<AudioAttributes> = if needs_audio {
            let provider = audio.expect("audio provider checked at entry");
            provider.get(&r.path, r.mtime_ns)?
        } else {
            None
        };
        if similarity_row_matches(
            q.root(),
            &r,
            &opts.match_mode,
            path_lower.as_deref(),
            attrs.as_ref(),
        ) {
            filtered.push(r);
        }
    }
    let mut rows = filtered;

    // Sort. If the user kept the default (Name+Asc), we override to
    // Jaccard desc — that's the only sensible order for a similarity
    // query and it matches what voidtools' Everything calls "Sort by
    // Relevance." Any other explicit `SortSpec` (Size / Date / Path /
    // …) is honored — the user knows what they want.
    if matches!(
        opts.sort,
        SortSpec {
            field: SortField::Name,
            order: SortOrder::Asc,
        }
    ) || matches!(opts.sort.field, SortField::Relevance)
    {
        rows.sort_by(|a, b| {
            let ja = jaccard_by_id.get(&a.file_id).copied().unwrap_or(0.0);
            let jb = jaccard_by_id.get(&b.file_id).copied().unwrap_or(0.0);
            jb.partial_cmp(&ja)
                .unwrap_or(std::cmp::Ordering::Equal)
                .then_with(|| a.file_id.cmp(&b.file_id))
        });
    } else {
        sort_rows(&mut rows, opts.sort);
    }
    if opts.limit > 0 && rows.len() > opts.limit {
        rows.truncate(opts.limit);
    }
    stats.final_hits = rows.len();

    Ok(ResultSet {
        rows,
        cursor: 0,
        plan: ExecPlan {
            seed: needle.to_lowercase(),
            needs_hydration: true,
        },
        stats,
    })
}

/// Mirror of `eval_full` with the `Similar` modifier short-circuited
/// to `true` — every row reaching this point was a similarity-LSH
/// candidate, so re-evaluating that predicate is both redundant and
/// (since we don't carry the LSH-side Jaccard score in here) wrong.
/// All *other* predicates — text / wildcard / regex / size / date /
/// path / parent / child / attrib / ext / quick-filter / audio — run
/// through the same Phase-5/9 logic so a composed query like
/// `similar:foo ext:pdf size:>1mb` or
/// `similar:bassdrop codec:flac length:>3:00` still filters
/// correctly.
fn similarity_row_matches(
    node: &QueryNode,
    row: &FileRow,
    mm: &MatchMode,
    path_lower: Option<&str>,
    audio: Option<&AudioAttributes>,
) -> bool {
    match node {
        QueryNode::True => true,
        QueryNode::Text(p) => {
            let target = match path_lower {
                Some(pl) => pl,
                None => row.name_lower.as_str(),
            };
            match_text(p, target.as_bytes(), mm)
        }
        QueryNode::Not(inner) => !similarity_row_matches(inner, row, mm, path_lower, audio),
        QueryNode::And(parts) => parts
            .iter()
            .all(|p| similarity_row_matches(p, row, mm, path_lower, audio)),
        QueryNode::Or(parts) => parts
            .iter()
            .any(|p| similarity_row_matches(p, row, mm, path_lower, audio)),
        QueryNode::Modifier(m) => match &m.kind {
            ModifierKind::Similar(_) => true,
            _ => eval_modifier(&m.kind, row, audio),
        },
        QueryNode::QuickFilter(qf) => row
            .ext
            .as_deref()
            .map(|e| qf.extensions().iter().any(|x| x.eq_ignore_ascii_case(e)))
            .unwrap_or(false),
        // A `similar:(...)` lens whose inner was already handled by
        // the LSH path (the only way we reach `similarity_row_matches`
        // is via `execute_similar`) is short-circuited to `true` —
        // matches the `Similar` modifier short-circuit above. Other
        // lens kinds recurse so their inner predicates filter.
        QueryNode::Lens {
            kind: LensKind::Similar,
            ..
        } => true,
        QueryNode::Lens { inner, .. } => similarity_row_matches(inner, row, mm, path_lower, audio),
    }
}
