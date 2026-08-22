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
use std::sync::Arc;

use std::borrow::Cow;
use std::cell::OnceCell;

use freally_audio::{AudioAttributes, AudioAttributesProvider};
use freally_index::{DirStats, FileRow, Index};
use freally_similarity::{SimilarityIndex, SimilarityOpts};

use crate::ast::{
    AudioPredicate, DateBound, DupeKey, EmptyKind, LensKind, ModifierKind, Query, QueryNode,
    SizeOp, TextPattern,
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

/// What a duplicate cluster has in common. Carries the *values*, not a
/// rendered string: the header row is localised and byte-formatted by
/// the UI (every other number in the app is), and the executor has no
/// business owning presentation.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DupeGroupKey {
    /// Shared file name, when the query grouped by name.
    pub name: Option<String>,
    /// Shared byte size, when the query grouped by size.
    pub size: Option<u64>,
}

/// One cluster of duplicates (SRC-M07). Members are contiguous in
/// [`ResultSet::rows`], so the UI can render a header row followed by
/// `len` result rows without re-grouping client-side.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DupeGroup {
    /// What the members share.
    pub key: DupeGroupKey,
    /// Index of the group's first row in [`ResultSet::rows`].
    pub start: usize,
    /// How many rows belong to this group.
    pub len: usize,
}

#[derive(Debug)]
pub struct ResultSet {
    rows: Vec<FileRow>,
    cursor: usize,
    pub plan: ExecPlan,
    pub stats: ExecStats,
    /// Duplicate clusters, when the query used the `dupe:` family.
    /// Empty for every other query.
    pub dupe_groups: Vec<DupeGroup>,
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
            // SRC-M23 — anchored on the filename, which the name index
            // already carries, so no hydration either.
            ModifierKind::Child(_) | ModifierKind::NamePrefix(_) | ModifierKind::NameSuffix(_) => {
                false
            }
            ModifierKind::Similar(_) => false,
            // Audio predicates need the FileRow's path + mtime_ns to
            // hit the AudioAttributesProvider.
            ModifierKind::Audio(_) => true,
            // SRC-M07/M08 need the row's path (to key into `DirStats`),
            // size, and attrs — none of which the name index carries.
            ModifierKind::Empty(_)
            | ModifierKind::ChildCount { .. }
            | ModifierKind::DescendantCount { .. }
            | ModifierKind::Dupe(_) => true,
            // SRC-M14: the volume id lives on the SQLite row, not in
            // the name buffer.
            ModifierKind::Volume(_) => true,
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
            QueryNode::Modifier(m) => match &m.kind {
                // SRC-M23 — an anchored needle is still a literal
                // substring of the name, so it seeds trigrams just as
                // well as `name:` does.
                ModifierKind::Child(c)
                | ModifierKind::NamePrefix(c)
                | ModifierKind::NameSuffix(c) => out.push(c.to_lowercase()),
                _ => {}
            },
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
    execute_with_catalogs(idx, similarity, audio, None, q, opts)
}

/// SRC-M14 entry point. Adds the catalog registry so `volume:` can be
/// written the way a person thinks about a drive — `volume:orange` for
/// "Orange WD 4TB" — instead of only as the internal volume id.
///
/// Without a registry the modifier still works, matching the needle
/// against the row's volume id directly; that is the shape the CLI and
/// the test harnesses use.
pub fn execute_with_catalogs(
    idx: &Index,
    similarity: Option<&SimilarityIndex>,
    audio: Option<&dyn AudioAttributesProvider>,
    catalogs: Option<&dyn VolumeCatalogs>,
    q: &Query,
    opts: ExecOpts,
) -> Result<ResultSet, QueryError> {
    // Resolved once per query, not per row: a needle maps to the same
    // handful of volume ids for every candidate.
    let volumes = resolve_volume_needles(q.root(), catalogs);
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
        return execute_similar(idx, similarity, audio, q, &opts, needle, &volumes);
    }
    if has_similar_anywhere(q.root()) {
        // Phase 6 only routes Similar in the root or as a direct child
        // of a top-level AND. Anywhere else (NOT, OR, deeper nesting)
        // is rejected loudly so the UI can surface the limitation.
        return Err(QueryError::UnsupportedSimilarPosition);
    }
    // SRC-M07 carries the same positional rule for the same reason: the
    // dupe family is resolved as a whole-set post-pass, so a `dupe:`
    // buried under OR / NOT would silently mean something other than
    // what it reads like.
    let dupe_keys = collect_dupe_keys(q.root());
    // Count, don't just detect: `dupe:name !size-dupe:` collects one
    // top-level key, so an `is_empty()` guard would wave the buried one
    // through — and `strip_dupe_nodes` would then rewrite it to `True`,
    // silently returning zero hits with no error.
    if count_dupe_nodes(q.root()) != dupe_keys.len() {
        return Err(QueryError::UnsupportedDupePosition);
    }
    // With the keys harvested, the `dupe:` nodes have said everything
    // they have to say; strip them so the per-row evaluator never sees
    // a predicate it cannot answer.
    // `strip_dupe_nodes` already rebuilds the tree, and so does
    // `normalize_needles`; borrowing between them keeps it to one clone
    // on the common path instead of two.
    let stripped;
    let base = if dupe_keys.is_empty() {
        q.root()
    } else {
        stripped = strip_dupe_nodes(q.root());
        &stripped
    };
    // Both per-row passes read needles off this tree, so the ladder runs
    // here once rather than inside the matchers once per row.
    let per_row_root = normalize_needles(base, &opts.match_mode);
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
    // SRC-M23 had the same problem for a different reason: ignoring
    // punctuation or whitespace rewrites the text before comparing, so
    // the trigrams of the *raw* name no longer describe what the needle
    // asks for. Those modes now seed from a parallel set of postings
    // built over stripped names instead of dropping to a full scan —
    // see `NameIndex::for_each_seed_candidate_named`.
    let seedable = !plan.seed.is_empty() && !opts.match_mode.match_path;
    let use_stripped_seed = seedable && opts.match_mode.rewrites_text();
    let mut survivors_ids: Vec<u64> = Vec::new();
    let mut survivors_names: Vec<String> = Vec::new();
    let mut stats = ExecStats {
        used_seed: seedable,
        ..ExecStats::default()
    };

    // The dupe family decides membership from the whole set: a file
    // whose only partner falls past the cap is not truncated, it is
    // reclassified as a singleton and dropped. Truncating the *output*
    // (via `opts.limit`) is fine; truncating the input is not.
    let cap = if opts.candidate_cap == 0 || !dupe_keys.is_empty() {
        usize::MAX
    } else {
        opts.candidate_cap
    };

    let evaluator = NameEvaluator::new(&per_row_root, &opts);

    // Phase 10 lens routing: an audio-only / similarity-only query
    // has no name-side predicate to filter by, so the per-row name
    // evaluation is a no-op. We skip it entirely — the per-row test
    // would still return `true`, but the call cost is non-zero. The
    // optimizer's `is_audio_only_route` hands us this hint.
    // Match Case joins the list for a structural reason: the name index
    // stores lowercased keys and nothing else, so the pre-filter has no
    // cased text to test against. Left in, it rejected every row before
    // hydration could look at the real name — `Report` with Match Case on
    // returned nothing at all. The trigram seed still applies, because it
    // is built from the lowercased needle and so selects a superset of
    // the cased answer; only the per-row test has to wait.
    let skip_name_filter = opts.match_mode.match_case
        || opts.match_mode.match_path
        || crate::optimizer::is_audio_only_route(q.root());

    if seedable {
        let mut collect = |fid: u64, key: &[u8]| {
            stats.candidates += 1;
            // The stored key may carry SRC-M12 readings; only the
            // name half is the row's identity, so that is what
            // sorting and hydration see.
            let (name, phonetic) = split_phonetic(key);
            if skip_name_filter || evaluator.matches(name, phonetic) {
                survivors_ids.push(fid);
                survivors_names.push(String::from_utf8_lossy(name).into_owned());
            }
        };
        if use_stripped_seed {
            let seed = freally_index::name_index::seed_key(&plan.seed);
            idx.name_index()
                .for_each_seed_candidate_named(&seed, cap, &mut collect);
        } else {
            idx.name_index()
                .for_each_candidate_named(&plan.seed, cap, &mut collect);
        }
    } else {
        let mut emitted = 0usize;
        idx.name_index().for_each_live(|fid, key| {
            stats.candidates += 1;
            if emitted >= cap {
                return false;
            }
            let (name, phonetic) = split_phonetic(key);
            if skip_name_filter || evaluator.matches(name, phonetic) {
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
    // Match Case needs `FileRow.name`, which only hydration provides —
    // the name buffer is lowercased.
    let needs_full =
        plan.needs_hydration || opts.match_mode.match_path || opts.match_mode.match_case;
    let i64_ids: Vec<i64> = survivors_ids.iter().map(|&u| u as i64).collect();
    let mut rows: Vec<FileRow> = idx.store().get_many(&i64_ids)?;
    let dirs = dir_stats_for(idx, q.root())?;
    if needs_full {
        // Phase 9: collect audio rows that survive the non-audio
        // predicates first, then loop one more time to apply audio
        // predicates. Splitting the work keeps the hot path
        // (filename-only queries) free of audio-cache lookups, and
        // means audio-only queries pay one cache lookup per surviving
        // row rather than per-candidate.
        rows = filter_with_audio(
            rows,
            &per_row_root,
            &opts.match_mode,
            audio,
            needs_audio,
            &dirs,
            &volumes,
        )?;
    }

    // SRC-M07: the dupe family is a set predicate, so it resolves after
    // every per-row predicate has narrowed the candidates. Grouping
    // supplies its own ordering (members contiguous under their
    // cluster), which is why it replaces `sort_rows` rather than
    // running before it.
    let dupe_groups = if dupe_keys.is_empty() {
        sort_rows(&mut rows, opts.sort);
        Vec::new()
    } else {
        group_duplicates(&mut rows, &dupe_keys, opts.sort)
    };

    if opts.limit > 0 && rows.len() > opts.limit {
        rows.truncate(opts.limit);
    }
    let dupe_groups = clamp_groups(dupe_groups, rows.len());
    stats.final_hits = rows.len();

    Ok(ResultSet {
        rows,
        cursor: 0,
        plan,
        stats,
        dupe_groups,
    })
}

/// Does any modifier in the tree satisfy `pred`? One walk for every
/// "is this kind of predicate present" question the executor asks.
fn any_modifier(node: &QueryNode, pred: &dyn Fn(&ModifierKind) -> bool) -> bool {
    match node {
        QueryNode::Modifier(m) => pred(&m.kind),
        QueryNode::Not(inner) => any_modifier(inner, pred),
        QueryNode::And(parts) | QueryNode::Or(parts) => parts.iter().any(|p| any_modifier(p, pred)),
        QueryNode::Text(_) | QueryNode::QuickFilter(_) | QueryNode::True => false,
        QueryNode::Lens { inner, .. } => any_modifier(inner, pred),
    }
}

/// The index-wide directory shape — derived (and memoized on the index)
/// only when the parsed query asks an emptiness question. Every other
/// query gets the empty shape, whose `counts()` reports zeroes and which
/// nothing will consult.
fn dir_stats_for(idx: &Index, root: &QueryNode) -> Result<Arc<DirStats>, QueryError> {
    if !needs_dir_stats(root) {
        return Ok(Arc::new(DirStats::default()));
    }
    Ok(idx.dir_stats()?)
}

fn needs_dir_stats(node: &QueryNode) -> bool {
    any_modifier(node, &|k| {
        matches!(
            k,
            ModifierKind::Empty(_)
                | ModifierKind::ChildCount { .. }
                | ModifierKind::DescendantCount { .. }
        )
    })
}

/// Collect the `dupe:` keys that sit at the root or as direct children
/// of a top-level AND — the only positions [`validate_supported`]
/// allows. Multiple keys compose (`dupe:name size-dupe:` keeps rows
/// that share both).
fn collect_dupe_keys(node: &QueryNode) -> Vec<DupeKey> {
    let mut out = Vec::new();
    let mut push = |k: DupeKey| {
        if !out.contains(&k) {
            out.push(k);
        }
    };
    match node {
        QueryNode::Modifier(m) => {
            if let ModifierKind::Dupe(k) = &m.kind {
                push(*k);
            }
        }
        QueryNode::And(parts) => {
            for p in parts {
                if let QueryNode::Modifier(m) = p
                    && let ModifierKind::Dupe(k) = &m.kind
                {
                    push(*k);
                }
            }
        }
        _ => {}
    }
    out
}

/// How many `dupe:` nodes the tree holds, anywhere. Compared against
/// the count [`collect_dupe_keys`] could reach so a node in an
/// unsupported position is rejected rather than quietly stripped.
fn count_dupe_nodes(node: &QueryNode) -> usize {
    match node {
        QueryNode::Modifier(m) => usize::from(matches!(m.kind, ModifierKind::Dupe(_))),
        QueryNode::Not(inner) => count_dupe_nodes(inner),
        QueryNode::And(parts) | QueryNode::Or(parts) => parts.iter().map(count_dupe_nodes).sum(),
        QueryNode::Text(_) | QueryNode::QuickFilter(_) | QueryNode::True => 0,
        QueryNode::Lens { inner, .. } => count_dupe_nodes(inner),
    }
}

/// Replace every `dupe:` node with [`QueryNode::True`].
///
/// The dupe family is resolved as a whole-set post-pass, so the per-row
/// evaluator must not see it — and "not per-row" is better expressed by
/// the AST node that already means identity than by a `=> true` arm
/// buried in `eval_modifier`. With this rewrite, `eval_modifier` can
/// fail loud on a `Dupe` the way it already does for `Similar` and
/// `Reserved`.
fn strip_dupe_nodes(node: &QueryNode) -> QueryNode {
    match node {
        QueryNode::Modifier(m) if matches!(m.kind, ModifierKind::Dupe(_)) => QueryNode::True,
        QueryNode::Not(inner) => QueryNode::Not(Box::new(strip_dupe_nodes(inner))),
        QueryNode::And(parts) => QueryNode::And(parts.iter().map(strip_dupe_nodes).collect()),
        QueryNode::Or(parts) => QueryNode::Or(parts.iter().map(strip_dupe_nodes).collect()),
        QueryNode::Lens { kind, inner } => QueryNode::Lens {
            kind: *kind,
            inner: Box::new(strip_dupe_nodes(inner)),
        },
        other => other.clone(),
    }
}

/// Keep only rows that share their key with at least one other row, and
/// reorder them so each cluster is contiguous. Directories are dropped:
/// two folders with the same name and a meaningless `size` are not
/// duplicate *files*, and reporting them would bury the real hits.
fn group_duplicates(rows: &mut Vec<FileRow>, keys: &[DupeKey], sort: SortSpec) -> Vec<DupeGroup> {
    let shape = DupeShape::of(keys);
    let mut buckets: HashMap<String, Vec<FileRow>> = HashMap::new();
    for row in rows.drain(..) {
        if is_directory(&row) {
            continue;
        }
        buckets.entry(shape.key_of(&row)).or_default().push(row);
    }

    // Singletons aren't duplicates. Ordering the survivors by key is
    // what makes the output stable for a given input — insertion order
    // would depend on the candidate stream.
    let mut order: Vec<String> = buckets
        .iter()
        .filter(|(_, members)| members.len() > 1)
        .map(|(k, _)| k.clone())
        .collect();
    order.sort();

    let mut groups = Vec::with_capacity(order.len());
    for key in order {
        let mut members = buckets.remove(&key).expect("key came from `buckets`");
        sort_rows(&mut members, sort);
        groups.push(DupeGroup {
            key: shape.describe(&members[0]),
            start: rows.len(),
            len: members.len(),
        });
        rows.append(&mut members);
    }
    groups
}

/// Which components a `dupe:` query groups by, resolved once per query
/// rather than re-walking `keys` for every row.
#[derive(Debug, Clone, Copy)]
struct DupeShape {
    by_name: bool,
    by_size: bool,
}

impl DupeShape {
    fn of(keys: &[DupeKey]) -> Self {
        Self {
            by_name: keys
                .iter()
                .any(|k| matches!(k, DupeKey::Name | DupeKey::NameSize)),
            by_size: keys
                .iter()
                .any(|k| matches!(k, DupeKey::Size | DupeKey::NameSize)),
        }
    }

    /// Rows land in the same cluster when every requested component
    /// matches. `\u{0}` separates the components so a name ending in
    /// digits can't collide with a name plus a size.
    fn key_of(self, row: &FileRow) -> String {
        use std::fmt::Write as _;
        let mut key = String::with_capacity(row.name_lower.len() + 24);
        if self.by_name {
            key.push_str(&row.name_lower);
        }
        key.push('\u{0}');
        if self.by_size {
            let _ = write!(key, "{}", row.size);
        }
        key
    }

    fn describe(self, sample: &FileRow) -> DupeGroupKey {
        DupeGroupKey {
            name: self.by_name.then(|| sample.name.clone()),
            size: self.by_size.then_some(sample.size),
        }
    }
}

/// Drop or shorten groups that fall past `len` after the `limit`
/// truncation, so every `DupeGroup` still describes a real slice of the
/// row vector.
fn clamp_groups(groups: Vec<DupeGroup>, len: usize) -> Vec<DupeGroup> {
    groups
        .into_iter()
        .filter_map(|mut g| {
            if g.start >= len {
                return None;
            }
            g.len = g.len.min(len - g.start);
            Some(g)
        })
        .collect()
}

fn sort_rows(rows: &mut [FileRow], spec: SortSpec) {
    // SRC-M24 — every string column goes through the same comparator, so
    // "sort by natural order" does not quietly mean "only the name
    // column".
    let text = |a: &str, b: &str| -> Ordering {
        if spec.natural {
            crate::natural::natural_cmp(a, b)
        } else {
            a.cmp(b)
        }
    };
    let cmp = |a: &FileRow, b: &FileRow| -> Ordering {
        match spec.field {
            // `Relevance` is only meaningful inside the similarity-lens
            // path (which sorts by Jaccard before calling here). On the
            // generic Phase-5 path it degrades to Name — matches the
            // Phase 11 UI's "Sort by Relevance" fallback for non-
            // similarity queries.
            SortField::Name | SortField::Relevance => text(&a.name_lower, &b.name_lower),
            // With natural sort off this stays `PathBuf::cmp`, which
            // orders by component rather than by byte — the ordering
            // this column has always had. Natural sort has to read the
            // path as text to see the digit runs at all, so the two
            // branches are not just "same order, digits aware"; that is
            // the one column where the toggle changes more than digits.
            // `to_string_lossy` per comparison would be O(n log n)
            // conversions of both operands — on Windows a full WTF-8
            // validity scan each time. The keys are built once below.
            SortField::Path => a.path.cmp(&b.path),
            SortField::Size => a.size.cmp(&b.size),
            SortField::Date => a.mtime_ns.cmp(&b.mtime_ns),
            // Phase 5 collapses voidtools' "Type" (display-name from
            // the OS file-association — `Folder`, `JPEG image`, …)
            // onto raw extension. Phase 11's settings + extractor
            // registry restore the distinction; until then both sort
            // keys behave identically and the UI must label the two
            // entries separately for parity with Everything.
            // Extensionless files keep sorting first, which is what
            // `Option::cmp` did before.
            SortField::Type | SortField::Ext => match (a.ext.as_deref(), b.ext.as_deref()) {
                (Some(x), Some(y)) => text(x, y),
                (x, y) => x.is_some().cmp(&y.is_some()),
            },
        }
    };
    // SRC-M24 — the path column is the one case that needs a string it
    // does not already have. Decorate-sort-undecorate so each row is
    // converted once rather than once per comparison; with natural sort
    // off, `PathBuf::cmp` needs no key at all.
    if spec.natural && spec.field == SortField::Path {
        let asc = spec.order == SortOrder::Asc;
        // One conversion per row instead of two per comparison.
        let mut keyed: Vec<(String, FileRow)> = rows
            .iter()
            .cloned()
            .map(|r| (r.path.to_string_lossy().into_owned(), r))
            .collect();
        keyed.sort_by(|(x, _), (y, _)| {
            let o = crate::natural::natural_cmp(x, y);
            if asc { o } else { o.reverse() }
        });
        for (slot, (_, row)) in rows.iter_mut().zip(keyed) {
            *slot = row;
        }
        return;
    }
    match spec.order {
        SortOrder::Asc => rows.sort_by(cmp),
        SortOrder::Desc => rows.sort_by(|a, b| cmp(a, b).reverse()),
    }
}

struct NameEvaluator<'a> {
    root: &'a Prepared,
    opts: &'a ExecOpts,
}

impl<'a> NameEvaluator<'a> {
    fn new(root: &'a Prepared, opts: &'a ExecOpts) -> Self {
        Self { root, opts }
    }

    /// Name-side eval. The bytes are the lowercased filename from the
    /// name index. Modifiers that need SQLite return true (the full
    /// pass filters them out later).
    ///
    /// `phonetic` carries SRC-M12's readings for a CJK name. Only text
    /// predicates consult it — `ext:` and the quick filters read the
    /// name alone, because `文件.txt`'s reading has no extension and
    /// matching one against it would be nonsense.
    fn matches(&self, name_lower: &[u8], phonetic: Option<&[u8]>) -> bool {
        eval_name(self.root, name_lower, phonetic, &self.opts.match_mode)
    }
}

/// Split a stored name-index key into `(name, phonetic)`.
///
/// SRC-M12 stores CJK names as `name` + separator + readings; every
/// other name is returned unchanged with `None`.
fn split_phonetic(key: &[u8]) -> (&[u8], Option<&[u8]>) {
    const SEP: u8 = freally_index::phonetic::PHONETIC_SEP as u8;
    match key.iter().position(|&b| b == SEP) {
        Some(i) => (&key[..i], Some(&key[i + 1..])),
        None => (key, None),
    }
}

/// SRC-M14. Supplies the executor with the volume ids a user-typed
/// `volume:` needle refers to. Implemented by `freally-indexd` over its
/// catalog registry; the executor never learns what a catalog is.
pub trait VolumeCatalogs {
    /// Volume ids whose catalog name or id matches `needle`. An empty
    /// result means "no catalog matches", which is a query that
    /// legitimately returns nothing.
    fn resolve(&self, needle: &str) -> Vec<String>;
}

/// The registry used when a caller supplies none: a needle stands for
/// itself, so `volume:win-d` matches that volume id directly.
///
/// This exists so the predicate has exactly one shape. Branching inside
/// the evaluator on "was a registry wired?" would mean the CLI and the
/// tests exercise a different `volume:` than the daemon ships — the
/// branch that reaches users being the one with the least coverage.
struct IdentityCatalogs;

impl VolumeCatalogs for IdentityCatalogs {
    fn resolve(&self, needle: &str) -> Vec<String> {
        vec![needle.to_string()]
    }
}

/// Every `volume:` needle in a query, mapped to the volume ids it
/// resolved to.
type VolumeNeedles = std::collections::HashMap<String, Vec<String>>;

/// Walk the AST once and pre-resolve each distinct `volume:` needle.
fn resolve_volume_needles(
    root: &QueryNode,
    catalogs: Option<&dyn VolumeCatalogs>,
) -> VolumeNeedles {
    let identity = IdentityCatalogs;
    let catalogs: &dyn VolumeCatalogs = catalogs.unwrap_or(&identity);
    let mut out = VolumeNeedles::new();
    fn walk(node: &QueryNode, catalogs: &dyn VolumeCatalogs, out: &mut VolumeNeedles) {
        match node {
            QueryNode::Modifier(m) => {
                if let ModifierKind::Volume(needle) = &m.kind {
                    if !out.contains_key(needle) {
                        out.insert(needle.clone(), catalogs.resolve(needle));
                    }
                }
            }
            QueryNode::Not(inner) | QueryNode::Lens { inner, .. } => walk(inner, catalogs, out),
            QueryNode::And(parts) | QueryNode::Or(parts) => {
                for p in parts {
                    walk(p, catalogs, out);
                }
            }
            QueryNode::Text(_) | QueryNode::QuickFilter(_) | QueryNode::True => {}
        }
    }
    walk(root, catalogs, &mut out);
    out
}

/// Apply the post-hydration predicate filter, including (when
/// `needs_audio` is true) per-row audio-attribute lookups.
fn filter_with_audio(
    rows: Vec<FileRow>,
    root: &Prepared,
    mm: &MatchMode,
    audio: Option<&dyn AudioAttributesProvider>,
    needs_audio: bool,
    dirs: &DirStats,
    volumes: &VolumeNeedles,
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
        let phonetic = if mm.match_phonetic {
            freally_index::phonetic::phonetic_keys(&r.name_lower)
        } else {
            None
        };
        let ctx = EvalCtx {
            mm,
            path_lower: path_lower.as_deref(),
            parent_lower: OnceCell::new(),
            audio: attrs.as_ref(),
            dirs,
            volumes,
            phonetic: phonetic.as_deref(),
        };
        if eval_full(root, &r, &ctx) {
            out.push(r);
        }
    }
    Ok(out)
}

/// Everything a per-row predicate needs beyond the row itself. All four
/// fields are invariant across one row's whole evaluation, so they
/// travel as one borrowed struct rather than as four positional
/// arguments threaded through two recursive evaluators.
struct EvalCtx<'a> {
    mm: &'a MatchMode,
    /// The row's lowercased full path, when `match_path` widened the
    /// text target. `None` means "match against the name".
    path_lower: Option<&'a str>,
    /// The row's lowercased parent *component*, computed on first use.
    ///
    /// `path_lower` cannot stand in: `parent:` matches the last component
    /// only, so `parent:docs` must not be satisfied by `/docs/a/b.txt`.
    /// Lazy rather than computed alongside `path_lower` because it is worth
    /// nothing to the queries that carry no `parent:` — which is nearly all
    /// of them — and a second walk of the tree to find out is more code than
    /// this. Once per row either way, however many `parent:` terms there are.
    parent_lower: OnceCell<Option<String>>,
    audio: Option<&'a AudioAttributes>,
    dirs: &'a DirStats,
    /// SRC-M12 readings for the row's name, when the toggle is on and
    /// the name holds CJK.
    ///
    /// Derived per row here rather than read from the name index: this
    /// pass works from hydrated SQLite records, whose `name_lower` is
    /// the bare name. Without it the hydrated pass would re-test the
    /// text predicate against a name the reading matched and throw the
    /// row away again.
    phonetic: Option<&'a str>,
    /// SRC-M14 `volume:` needles already resolved to volume ids.
    volumes: &'a VolumeNeedles,
}

/// Put every literal needle in the tree through [`normalized`] once, up
/// front, instead of once per candidate row.
///
/// The needle is invariant for the whole query while the target changes
/// every row, so the needle is the half of each comparison that can be
/// hoisted out of the loop. On a query that falls back to a full scan —
/// which is every query under Ignore Punctuation until the stripped key
/// lands — that is one normalization instead of several million.
///
/// This does not live on the cached `ExecPlan`. `PlanCache` keys on the
/// query string alone while the ladder depends on the match mode, so two
/// callers running one query under different modes would poison each
/// other's entry. That is a property of the cache as written rather than
/// a law — the key could grow — but the cache has no production callers
/// today, so widening it would be speculative work on dead code.
///
/// `path:` and `parent:` take the case fold only. Their matchers never
/// folded diacritics or honoured the SRC-M23 modes, and quietly widening
/// them here would be a behaviour change wearing a refactor's clothes.
/// A query tree whose literal needles have already been through
/// [`normalized`].
///
/// Every matcher assumes this: `literal_match`, `substring_match` and
/// `anchored_match` normalize the *target* and compare the needle as
/// given. That contract used to be prose in half a dozen comment blocks
/// and enforced nowhere, so a third execution path that assembled an
/// `EvalCtx` by hand would get quietly wrong answers rather than a
/// compile error — `Café` would simply stop matching `cafe`, and no test
/// would catch it, because the suite drives the two prepared entry
/// points. Now the per-row evaluators will not accept anything else.
///
/// [`normalize_needles`] is the only way to build one.
#[derive(Debug)]
struct Prepared(QueryNode);

impl std::ops::Deref for Prepared {
    type Target = QueryNode;
    fn deref(&self) -> &QueryNode {
        &self.0
    }
}

fn normalize_needles(node: &QueryNode, mm: &MatchMode) -> Prepared {
    Prepared(normalize_node(node, mm))
}

fn normalize_node(node: &QueryNode, mm: &MatchMode) -> QueryNode {
    // One ladder for every needle on the tree. `child:` and the anchored
    // pair used to have their own, folding case unconditionally because
    // the name index they ran against had no case to match; now that
    // Match Case routes them at the hydrated row they fold on the same
    // terms a bare term does, which makes the two rules one rule.
    let fold = |s: &String| normalized(s, mm).into_owned();
    let pathish = pathish_mode(mm);
    let fold_pathish = |s: &String| normalized(s, &pathish).into_owned();
    match node {
        QueryNode::Text(TextPattern::Literal(l)) => QueryNode::Text(TextPattern::Literal(fold(l))),
        QueryNode::Modifier(m) => {
            let kind = match &m.kind {
                ModifierKind::Child(c) => ModifierKind::Child(fold(c)),
                ModifierKind::NamePrefix(c) => ModifierKind::NamePrefix(fold(c)),
                ModifierKind::NameSuffix(c) => ModifierKind::NameSuffix(fold(c)),
                // See `pathish_mode`. Their targets below run the same mode,
                // so needle and target still agree.
                ModifierKind::Path(p) => ModifierKind::Path(fold_pathish(p)),
                ModifierKind::Parent(p) => ModifierKind::Parent(fold_pathish(p)),
                other => other.clone(),
            };
            QueryNode::Modifier(crate::ast::ModifierPredicate { kind })
        }
        QueryNode::Not(inner) => QueryNode::Not(Box::new(normalize_node(inner, mm))),
        QueryNode::And(parts) => {
            QueryNode::And(parts.iter().map(|p| normalize_node(p, mm)).collect())
        }
        QueryNode::Or(parts) => {
            QueryNode::Or(parts.iter().map(|p| normalize_node(p, mm)).collect())
        }
        QueryNode::Lens { kind, inner } => QueryNode::Lens {
            kind: *kind,
            inner: Box::new(normalize_node(inner, mm)),
        },
        // Wildcards and regexes carry a compiled pattern rather than a
        // needle, and quick filters carry no text at all.
        QueryNode::Text(_) | QueryNode::QuickFilter(_) | QueryNode::True => node.clone(),
    }
}

/// Can this subtree be decided from the name buffer alone?
///
/// `eval_name` answers "true" for anything it cannot decide, so the
/// full post-hydration pass gets to see the row. Under `NOT` that
/// convention inverts into "false" — a definite reject — and the row
/// never reaches the pass that could actually answer. So a negation
/// over an undecidable subtree has to let the row through instead.
fn name_decidable(node: &QueryNode) -> bool {
    match node {
        QueryNode::Modifier(m) => matches!(
            m.kind,
            ModifierKind::Child(_)
                | ModifierKind::NamePrefix(_)
                | ModifierKind::NameSuffix(_)
                | ModifierKind::Ext(_)
        ),
        QueryNode::Text(_) | QueryNode::QuickFilter(_) | QueryNode::True => true,
        QueryNode::Not(inner) => name_decidable(inner),
        QueryNode::And(parts) | QueryNode::Or(parts) => parts.iter().all(name_decidable),
        QueryNode::Lens { inner, .. } => name_decidable(inner),
    }
}

fn eval_name(node: &QueryNode, name_lower: &[u8], phonetic: Option<&[u8]>, mm: &MatchMode) -> bool {
    // SRC-M12: a text predicate matches if either the name or its
    // phonetic reading satisfies it. Gated on the toggle, so with the
    // setting off the readings sitting in the index are inert and
    // matching is byte-identical to pre-Build-2.
    let phon =
        |mm: &MatchMode| -> Option<&[u8]> { if mm.match_phonetic { phonetic } else { None } };
    match node {
        QueryNode::True => true,
        QueryNode::Text(p) => {
            match_text(p, name_lower, mm) || phon(mm).is_some_and(|ph| match_text(p, ph, mm))
        }
        // A negation we cannot decide here must not reject the row —
        // `!size:>1mb`, `!empty:folder`, `NOT lufs:<-14` all need the
        // hydrated pass to answer.
        QueryNode::Not(inner) if !name_decidable(inner) => true,
        QueryNode::Not(inner) => !eval_name(inner, name_lower, phonetic, mm),
        QueryNode::And(parts) => parts.iter().all(|p| eval_name(p, name_lower, phonetic, mm)),
        QueryNode::Or(parts) => parts.iter().any(|p| eval_name(p, name_lower, phonetic, mm)),
        QueryNode::Modifier(m) => match &m.kind {
            ModifierKind::Child(needle) => {
                substring_match(name_lower, needle, mm)
                    || phon(mm).is_some_and(|ph| substring_match(ph, needle, mm))
            }
            // SRC-M23. No phonetic fallback: a reading is appended to
            // the name behind a separator, so "starts with" and "ends
            // with" against it would anchor on the wrong text.
            ModifierKind::NamePrefix(needle) => anchored_match(name_lower, needle, mm, true),
            ModifierKind::NameSuffix(needle) => anchored_match(name_lower, needle, mm, false),
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
        QueryNode::Lens { inner, .. } => eval_name(inner, name_lower, phonetic, mm),
    }
}

impl EvalCtx<'_> {
    /// The row's parent component, folded the way `pathish_mode` folds
    /// the needle. `None` when the path has no parent, or a non-UTF-8 one.
    fn parent_lower(&self, row: &FileRow) -> Option<&str> {
        self.parent_lower
            .get_or_init(|| {
                row.path
                    .parent()
                    .and_then(|p| p.file_name())
                    .and_then(|s| s.to_str())
                    .map(|s| normalized(s, &pathish_mode(self.mm)).into_owned())
            })
            .as_deref()
    }
}

/// What a text predicate compares against for this row.
///
/// Normally the lowercased name, or the lowercased path when Match Path
/// widened the target. Under Match Case it is the name (or path) as the
/// filesystem spells it, because that is the only copy with any case in
/// it — everything the name index holds has already been folded.
fn text_target<'a>(row: &'a FileRow, ctx: &EvalCtx<'a>) -> Cow<'a, str> {
    match (ctx.mm.match_case, ctx.path_lower) {
        (true, Some(_)) => row.path.to_string_lossy(),
        (true, None) => Cow::Borrowed(row.name.as_str()),
        (false, Some(p)) => Cow::Borrowed(p),
        (false, None) => Cow::Borrowed(row.name_lower.as_str()),
    }
}

/// The same choice for the filename-only modifiers. `child:Report` has to
/// answer the way the bare term `Report` does — one query written two
/// ways is one query.
fn name_target<'a>(row: &'a FileRow, ctx: &EvalCtx<'_>) -> &'a str {
    if ctx.mm.match_case {
        row.name.as_str()
    } else {
        row.name_lower.as_str()
    }
}

fn eval_full(node: &QueryNode, row: &FileRow, ctx: &EvalCtx<'_>) -> bool {
    match node {
        QueryNode::True => true,
        QueryNode::Text(p) => {
            let target = text_target(row, ctx);
            match_text(p, target.as_bytes(), ctx.mm)
                || ctx
                    .phonetic
                    .is_some_and(|ph| match_text(p, ph.as_bytes(), ctx.mm))
        }
        QueryNode::Not(inner) => !eval_full(inner, row, ctx),
        QueryNode::And(parts) => parts.iter().all(|p| eval_full(p, row, ctx)),
        QueryNode::Or(parts) => parts.iter().any(|p| eval_full(p, row, ctx)),
        QueryNode::Modifier(m) => eval_modifier(&m.kind, row, ctx),
        QueryNode::QuickFilter(qf) => row_has_quick_filter_ext(row, *qf),
        QueryNode::Lens { inner, .. } => eval_full(inner, row, ctx),
    }
}

fn row_has_quick_filter_ext(row: &FileRow, qf: crate::quick_filters::QuickFilter) -> bool {
    row.ext
        .as_deref()
        .map(|e| qf.extensions().iter().any(|x| x.eq_ignore_ascii_case(e)))
        .unwrap_or(false)
}

/// Is this row a directory? Reads the portable `attrs` projection the
/// journal subscribers write on every OS.
fn is_directory(row: &FileRow) -> bool {
    row.attrs & freally_index::ATTR_DIRECTORY != 0
}

/// SRC-M08 `empty:` resolution. Every arm reads index data only — the
/// filesystem is never touched.
fn eval_empty(kind: EmptyKind, row: &FileRow, dirs: &DirStats) -> bool {
    let is_dir = is_directory(row);
    let empty_file = !is_dir && row.size == 0;
    let empty_folder = || is_dir && dirs.counts(&row.path).children == 0;
    match kind {
        EmptyKind::File => empty_file,
        EmptyKind::Folder => empty_folder(),
        EmptyKind::Roots => is_dir && dirs.is_empty_subtree_root(&row.path),
        EmptyKind::Any => empty_file || empty_folder(),
    }
}

fn eval_modifier(kind: &ModifierKind, row: &FileRow, ctx: &EvalCtx<'_>) -> bool {
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
        // Needle lowercased once by `normalize_needles`.
        ModifierKind::Path(needle) => match ctx.path_lower {
            // `match_path` already lowered this row's path for the text
            // predicate; lowering it a second time here would allocate
            // the same string again for every hydrated row.
            Some(p) => p.contains(needle),
            None => row.path.to_string_lossy().to_lowercase().contains(needle),
        },
        // SRC-M14. A row with no volume can never belong to a catalog,
        // so it never matches — including rows indexed before M14,
        // whose volume is empty. Those need a rescan to become
        // filterable, which is exactly what an empty badge tells the
        // user.
        ModifierKind::Volume(needle) => {
            if row.volume.is_empty() {
                return false;
            }
            // Always populated — an absent registry resolves through
            // `IdentityCatalogs`, so there is one predicate rather than
            // one per wiring. Compared case-insensitively because volume
            // ids carry the platform's own casing (`win-C`) while the
            // user types whatever they like; the id set is a handful of
            // entries, so this beats lowercasing the row per candidate.
            match ctx.volumes.get(needle) {
                Some(ids) => ids.iter().any(|id| id.eq_ignore_ascii_case(&row.volume)),
                None => false,
            }
        }
        // Needle folded once by `normalize_needles`; the target is folded
        // once per row by `EvalCtx::parent_lower` rather than once per
        // evaluation, which is what `path_lower` already does for `path:`.
        ModifierKind::Parent(needle) => ctx
            .parent_lower(row)
            .is_some_and(|parent| parent.contains(needle)),
        // These three run the same normalization ladder as their
        // `eval_name` counterparts, and must keep doing so. Both passes
        // execute on every query: the name pass filters candidates, this
        // one re-tests the hydrated row. A bare `contains` /
        // `starts_with` here quietly rejects rows the name pass accepted
        // as soon as any hydrating modifier joins the query, so
        // `name^:café` and `name^:café size:>1mb` would answer
        // differently.
        ModifierKind::Child(needle) => {
            substring_match(name_target(row, ctx).as_bytes(), needle, ctx.mm)
                || ctx
                    .phonetic
                    .is_some_and(|ph| substring_match(ph.as_bytes(), needle, ctx.mm))
        }
        ModifierKind::NamePrefix(needle) => {
            anchored_match(name_target(row, ctx).as_bytes(), needle, ctx.mm, true)
        }
        ModifierKind::NameSuffix(needle) => {
            anchored_match(name_target(row, ctx).as_bytes(), needle, ctx.mm, false)
        }
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
        ModifierKind::Audio(pred) => match ctx.audio {
            Some(attrs) => eval_audio_predicate(pred, attrs),
            // No cached audio attributes — either the row's path
            // isn't audio, or the cache miss returned None
            // (e.g. extractor disabled). Either way, the predicate
            // doesn't match this row.
            None => false,
        },
        ModifierKind::Empty(kind) => eval_empty(*kind, row, ctx.dirs),
        // A file has no children, so `child-count:0` must not sweep
        // every file in the index into the results — the question is
        // only meaningful for folders.
        ModifierKind::ChildCount { op, count } => {
            is_directory(row) && cmp_op(*op, ctx.dirs.counts(&row.path).children, *count)
        }
        ModifierKind::DescendantCount { op, count } => {
            is_directory(row) && cmp_op(*op, ctx.dirs.counts(&row.path).descendants, *count)
        }
        ModifierKind::Dupe(_) => {
            // `execute_with_audio` strips the dupe family out of the AST
            // (`strip_dupe_nodes`) before per-row evaluation, because
            // whether a row is a duplicate depends on other rows.
            // Reaching this arm means a caller built a Query by hand and
            // skipped that — fail loud, like Similar and Reserved.
            debug_assert!(
                false,
                "dupe: modifier reached eval_modifier — caller skipped execute_with"
            );
            false
        }
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
    match pattern {
        // The needle went through the ladder once, when the query was
        // prepared — see `normalize_needles`. Only the target is
        // per-row work from here.
        TextPattern::Literal(needle) => literal_match(target_str, needle, mm),
        // Wildcards and regexes stop one rung short: their syntax is
        // written against the name as it is spelled, so dropping
        // punctuation from the target would quietly change what `*.txt`
        // asks for.
        TextPattern::Wildcard { compiled, .. } => {
            compiled.is_match(&folded(Cow::Borrowed(target_str), mm))
        }
        TextPattern::Regex { compiled, .. } => {
            let target_eff = folded(Cow::Borrowed(target_str), mm);
            if mm.match_case {
                compiled.is_match(&target_eff)
            } else {
                // Re-run case-insensitively by relying on regex's own
                // (?i) prefix when the user didn't supply one. We don't
                // mutate the cached compiled regex — instead we lower
                // both sides and run a new match.
                compiled.is_match(&target_eff.to_lowercase())
            }
        }
    }
}

/// Compare a prepared needle against one row's target.
///
/// The needle has already been through [`normalized`]; the target has
/// not, and is the half that changes every row. SRC-M23's strip runs on
/// both sides — stripping only the target would make `foobar` find
/// `foo-bar` while leaving `foo-bar` unable to find `foobar`, which
/// reads as a bug rather than as a match mode.
fn literal_match(target: &str, needle: &str, mm: &MatchMode) -> bool {
    // Both sides run the same ladder; `mm` decides whether case folds,
    // and the needle has already been through it once in
    // `normalize_needles`. Idempotence is what makes that safe.
    let target_eff = normalized(target, mm);
    // Whole-word is meaningless once the separators that define word
    // boundaries have been removed — `foo-bar` becomes one word. The
    // stripped comparison is the more specific request, so it wins.
    if mm.whole_word && !mm.rewrites_text() {
        whole_word_contains(&target_eff, needle)
    } else {
        target_eff.contains(needle)
    }
}

/// Remove the character classes the active match mode ignores.
///
/// Uses Unicode's own categories rather than an ASCII list, so `’`
/// and `–` are dropped alongside `'` and `-`.
fn strip_ignored(s: &str, mm: &MatchMode) -> String {
    s.chars().filter(|c| !is_ignored(*c, mm)).collect()
}

/// Does the active match mode drop this character before comparing?
///
/// The punctuation test is `freally_index`'s, not a local copy. The
/// stripped trigram key is built from the same predicate, and a seed
/// that dropped *less* than a mode drops would turn that superset
/// filter into a silent miss — so the two cannot be allowed to drift.
fn is_ignored(c: char, mm: &MatchMode) -> bool {
    let drop_punct = mm.ignore_punctuation && freally_index::name_index::is_punctuation(c);
    let drop_space = mm.ignore_whitespace && c.is_whitespace();
    drop_punct || drop_space
}

/// Rung one of the ladder: fold diacritics away unless the user asked to
/// keep them.
///
/// Split out because wildcard and regex patterns stop here — see
/// [`match_text`]. Borrows straight back for an ASCII string, which has
/// no combining marks to strip and is what most filenames are.
fn folded<'a>(c: Cow<'a, str>, mm: &MatchMode) -> Cow<'a, str> {
    if mm.match_diacritics || c.is_ascii() {
        c
    } else {
        Cow::Owned(strip_diacritics(&c))
    }
}

/// The match mode `path:` and `parent:` are compared under.
///
/// Those two have always been case-insensitive whatever the match mode says,
/// and have never folded diacritics or dropped ignore-classes. That was a
/// hardcoded `to_lowercase()` sitting *inside* the ladder — the one function
/// whose job is that there is only one ladder. Expressing it as a mode
/// instead keeps that promise: there is still one `normalized`, still one
/// knob, and the shorter ladder is now a value rather than a branch.
///
/// It is a `MatchMode` rather than a rung mask because the difference is not
/// a subset — `match_case` is *inverted*, not skipped. Saying so in the type
/// is the honest version.
///
/// Widening these two to the full ladder is a real behaviour change, to make
/// deliberately rather than as a side effect of tidying this up.
fn pathish_mode(mm: &MatchMode) -> MatchMode {
    MatchMode {
        // Both sides of a `path:` comparison are folded, so opting out under
        // Match Case would compare a cased needle against a folded target and
        // match nothing at all.
        match_case: false,
        // `true` means "leave diacritics alone" — the ladder folds them only
        // when this is off.
        match_diacritics: true,
        ignore_punctuation: false,
        ignore_whitespace: false,
        ..*mm
    }
}

/// The normalization ladder, in one place: case-fold → strip diacritics
/// → drop the classes the mode ignores.
///
/// Every matcher runs exactly this, in exactly this order, so a new
/// match-mode flag is one edit rather than four — and the matchers
/// cannot drift apart again, which is what let `name^:` answer
/// differently depending on the rest of the query.
///
/// Each rung hands its input straight back when it has nothing to do, so
/// the common case — an ASCII lowercase name under the default mode —
/// walks the string and allocates nothing.
///
/// Case folds on both sides unless Match Case is on. The name index
/// stores lowercased keys, so folding a target is normally a scan that
/// finds nothing to do — but *normally* is not always: NFKD can put
/// uppercase back into an already-lowercased string (`№` decomposes to
/// `No`), and a target-side rung that opted out on the "it is already
/// lowercase" assumption would then compare `No5` against a folded needle
/// of `no5`. There is no case where one side should fold and the other
/// should not, so there is no knob for it.
fn normalized<'a>(s: &'a str, mm: &MatchMode) -> Cow<'a, str> {
    let fold_case = !mm.match_case;
    // Diacritics first, then case. NFKD is a *compatibility* decomposition,
    // so it can turn an uncased character into cased letters — `№` becomes
    // `No`, `㎅` becomes `kB` — and folding case first would leave that `N`
    // and `B` standing against an already-lowercased target. The code
    // before the refactor ran these the other way round, which is the
    // bug `the_ladder_folds_diacritics_before_case` pins.
    let folded_out = folded(Cow::Borrowed(s), mm);
    let lowered = if fold_case && changes_under_lowercase(&folded_out) {
        Cow::Owned(folded_out.to_lowercase())
    } else {
        folded_out
    };
    if mm.rewrites_text() && lowered.chars().any(|c| is_ignored(c, mm)) {
        Cow::Owned(strip_ignored(&lowered, mm))
    } else {
        lowered
    }
}

/// Would `to_lowercase` change this string?
///
/// The obvious `any(char::is_uppercase)` is not the same question:
/// titlecase characters (`ǅ`, `ǈ`, `ᾈ`) are `Lt`, not `Lu`, so they are not
/// `is_uppercase` and would slip through unfolded while the lowercased
/// target had already folded them. Asking each character directly costs a
/// scan and still allocates nothing when the answer is no.
fn changes_under_lowercase(s: &str) -> bool {
    s.chars().any(|c| {
        let mut lower = c.to_lowercase();
        lower.next() != Some(c) || lower.next().is_some()
    })
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

/// `child:` / `name:`. Prepared needle, lowercased target from the name
/// index.
///
/// SRC-M23 — `name:foo-bar` has to behave like the bare term `foo-bar`
/// under Ignore Punctuation. Running the same ladder as
/// [`literal_match`] is what keeps one query written two ways answering
/// one way.
fn substring_match(target_lower: &[u8], needle: &str, mm: &MatchMode) -> bool {
    match std::str::from_utf8(target_lower) {
        Ok(s) => normalized(s, mm).contains(needle),
        Err(_) => false,
    }
}

/// SRC-M23 — `name^:` / `name$:`. Same normalization ladder as
/// [`substring_match`], anchored at one end instead of free-floating.
///
/// The ignore-punctuation / ignore-whitespace strip runs last, so
/// `name^:foo` still matches `foo-bar.txt` when those modes are on.
fn anchored_match(target_lower: &[u8], needle: &str, mm: &MatchMode, prefix: bool) -> bool {
    let s = match std::str::from_utf8(target_lower) {
        Ok(s) => s,
        Err(_) => return false,
    };
    let target_eff = normalized(s, mm);
    if prefix {
        target_eff.starts_with(needle)
    } else {
        target_eff.ends_with(needle)
    }
}

/// Cheap diacritic stripper — not perfect for every script (Phase 5
/// perf-pass note) but matches Everything's "Match Diacritics" toggle.
///
/// `freally_index` owns the definition because the stripped trigram key
/// is built from it; a second copy here could drift into folding less
/// than the seed does, and the seed would stop being a superset.
use freally_index::name_index::strip_diacritics;

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
    volumes: &VolumeNeedles,
) -> Result<ResultSet, QueryError> {
    let sim = similarity.ok_or(QueryError::SimilarityIndexUnavailable)?;
    // Same prepared-needle contract as the trigram path: the matchers
    // this route reaches expect needles that have already been through
    // the ladder, so this tree has to make the same trip.
    let per_row_root = normalize_needles(q.root(), &opts.match_mode);
    let cap = if opts.candidate_cap == 0 {
        usize::MAX
    } else {
        opts.candidate_cap
    };
    let sim_opts = SimilarityOpts {
        // `cap` is already `usize::MAX` when uncapped, so no clamp needed.
        candidate_cap: cap,
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

    let dirs = dir_stats_for(idx, q.root())?;
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
        let phonetic = if opts.match_mode.match_phonetic {
            freally_index::phonetic::phonetic_keys(&r.name_lower)
        } else {
            None
        };
        let ctx = EvalCtx {
            mm: &opts.match_mode,
            path_lower: path_lower.as_deref(),
            parent_lower: OnceCell::new(),
            audio: attrs.as_ref(),
            dirs: &dirs,
            phonetic: phonetic.as_deref(),
            volumes,
        };
        if similarity_row_matches(&per_row_root, &r, &ctx) {
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
            // "Did the user leave the sort alone?" — whether digit runs
            // read as numbers has no bearing on that.
            ..
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
        // `similar:` and `dupe:` are both root-position-only, so a
        // query can never take the similarity path *and* group
        // duplicates.
        dupe_groups: Vec::new(),
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
fn similarity_row_matches(root: &Prepared, row: &FileRow, ctx: &EvalCtx<'_>) -> bool {
    similarity_node_matches(root, row, ctx)
}

fn similarity_node_matches(node: &QueryNode, row: &FileRow, ctx: &EvalCtx<'_>) -> bool {
    match node {
        QueryNode::True => true,
        QueryNode::Text(p) => {
            let target = text_target(row, ctx);
            match_text(p, target.as_bytes(), ctx.mm)
        }
        QueryNode::Not(inner) => !similarity_node_matches(inner, row, ctx),
        QueryNode::And(parts) => parts.iter().all(|p| similarity_node_matches(p, row, ctx)),
        QueryNode::Or(parts) => parts.iter().any(|p| similarity_node_matches(p, row, ctx)),
        QueryNode::Modifier(m) => match &m.kind {
            ModifierKind::Similar(_) => true,
            _ => eval_modifier(&m.kind, row, ctx),
        },
        QueryNode::QuickFilter(qf) => row_has_quick_filter_ext(row, *qf),
        // A `similar:(...)` lens whose inner was already handled by
        // the LSH path (the only way we reach `similarity_row_matches`
        // is via `execute_similar`) is short-circuited to `true` —
        // matches the `Similar` modifier short-circuit above. Other
        // lens kinds recurse so their inner predicates filter.
        QueryNode::Lens {
            kind: LensKind::Similar,
            ..
        } => true,
        QueryNode::Lens { inner, .. } => similarity_node_matches(inner, row, ctx),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn mode(f: impl FnOnce(&mut MatchMode)) -> MatchMode {
        let mut m = MatchMode::default();
        f(&mut m);
        m
    }

    #[test]
    fn the_ladder_runs_diacritics_then_case_then_the_ignored_classes() {
        let all = mode(|m| {
            m.ignore_punctuation = true;
            m.ignore_whitespace = true;
        });
        assert_eq!(normalized("My Café-Notes.TXT", &all), "mycafenotestxt");
        // Same input, one rung disabled at a time.
        assert_eq!(
            normalized("My Café-Notes.TXT", &MatchMode::default()),
            "my cafe-notes.txt"
        );
        assert_eq!(
            normalized("My Café-Notes.TXT", &mode(|m| m.match_diacritics = true)),
            "my café-notes.txt"
        );
        // An already-lowercased target — what a name-index key looks
        // like. It still goes through the same ladder, with nothing for
        // the case rung to do.
        assert_eq!(
            normalized("my café-notes.txt", &MatchMode::default()),
            "my cafe-notes.txt"
        );
    }

    #[test]
    fn the_ladder_borrows_when_it_has_nothing_to_do() {
        // The hot path: an ASCII lowercase name under the default mode
        // is walked and handed straight back, no allocation. This is the
        // property that lets the matchers call it once per candidate row.
        let m = MatchMode::default();
        assert!(matches!(
            normalized("alpha-report.md", &m),
            Cow::Borrowed(_)
        ));
        // Punctuation only costs an allocation once the mode asks for it.
        assert!(matches!(
            normalized("alpha-report.md", &mode(|m| m.ignore_punctuation = true)),
            Cow::Owned(_)
        ));
        // A name with no punctuation still borrows under that mode.
        assert!(matches!(
            normalized("alphareport", &mode(|m| m.ignore_punctuation = true)),
            Cow::Borrowed(_)
        ));
    }

    #[test]
    fn the_ladder_folds_diacritics_before_case() {
        // NFKD is a *compatibility* decomposition, so it can emit cased
        // letters from an uncased character. Folding case first left that
        // `N` standing: `№5` became `No5` and stopped matching `no5`.
        //
        // It is also why the target side has no opt-out. A name-index key
        // is stored lowercased, so folding it again looks redundant — but
        // NFKD can put uppercase back into it after the fact.
        let m = MatchMode::default();
        assert_eq!(normalized("№5", &m), "no5");
        assert_eq!(normalized("№5-report.txt", &m), "no5-report.txt");
    }

    #[test]
    fn the_case_fold_catches_titlecase_too() {
        // `ǅ` is Lt, not Lu, so `char::is_uppercase` answers false for it.
        // The guard that asked that question handed it through unfolded
        // while the target had already folded it.
        let m = MatchMode::default();
        assert_eq!(normalized("ǅungla", &m), "dzungla");
        assert!(changes_under_lowercase("ǅ"));
        assert!(!changes_under_lowercase("already lower"));
    }

    #[test]
    fn the_ladder_is_idempotent() {
        // `normalize_needles` runs it on the needle and the matchers run
        // it on the target, so a needle that came back through a matcher
        // must not change again.
        for m in [
            MatchMode::default(),
            mode(|m| m.ignore_punctuation = true),
            mode(|m| m.ignore_whitespace = true),
            mode(|m| m.match_diacritics = true),
        ] {
            for s in ["My Café-Notes.TXT", "plain", "ünïcödé", "a b-c_d"] {
                let once = normalized(s, &m).into_owned();
                assert_eq!(normalized(&once, &m), once, "{s:?} under {m:?}");
            }
        }
    }

    #[test]
    fn the_pathish_mode_stops_at_the_case_fold() {
        // `path:` and `parent:` have never folded diacritics or ignore
        // classes. That was a hardcoded `to_lowercase()` inside the ladder;
        // it is a named mode now, and this is the behaviour it has to keep.
        let m = mode(|m| {
            m.ignore_punctuation = true;
            m.ignore_whitespace = true;
        });
        assert_eq!(normalized("My Café-Notes.TXT", &m), "mycafenotestxt");
        assert_eq!(
            normalized("My Café-Notes.TXT", &pathish_mode(&m)),
            "my café-notes.txt"
        );
    }

    #[test]
    fn the_pathish_mode_folds_even_under_match_case() {
        // Both sides of a `path:` comparison are folded, so the needle must
        // fold too — opting out here would compare `Reports` against a
        // lowercased path and match nothing at all.
        let m = mode(|m| m.match_case = true);
        assert_eq!(normalized("Reports", &pathish_mode(&m)), "reports");
        // Whereas the full ladder honours the flag, which is the whole point
        // of Match Case.
        assert_eq!(normalized("Reports", &m), "Reports");
    }

    #[test]
    fn path_and_parent_needles_take_the_pathish_mode() {
        // Read through `normalize_needles`, so the mode is pinned where the
        // query actually picks it rather than only at the ladder.
        let m = mode(|m| m.ignore_punctuation = true);
        let q = crate::parse("path:My-Café parent:My-Café").expect("parses");
        let mut seen = Vec::new();
        fn walk(n: &QueryNode, out: &mut Vec<String>) {
            match n {
                QueryNode::Modifier(m) => match &m.kind {
                    ModifierKind::Path(p) | ModifierKind::Parent(p) => out.push(p.clone()),
                    _ => {}
                },
                QueryNode::And(parts) | QueryNode::Or(parts) => {
                    parts.iter().for_each(|p| walk(p, out))
                }
                QueryNode::Not(i) => walk(i, out),
                QueryNode::Lens { inner, .. } => walk(inner, out),
                _ => {}
            }
        }
        walk(&normalize_needles(q.root(), &m), &mut seen);
        // Lowercased, but the diacritic and the hyphen both survive.
        assert_eq!(seen, vec!["my-café".to_string(), "my-café".to_string()]);
    }

    #[test]
    fn parent_matches_the_last_component_only() {
        // The cached target is the parent *component*, not the whole path —
        // reusing `path_lower` here would make `parent:docs` true for any
        // row anywhere under a `docs` directory.
        let ctx_mm = MatchMode::default();
        let dirs = DirStats::default();
        let volumes = VolumeNeedles::default();
        let row = FileRow {
            file_id: 1,
            path: std::path::PathBuf::from("/docs/archive/notes.txt"),
            name: "notes.txt".into(),
            name_lower: "notes.txt".into(),
            ext: Some("txt".into()),
            size: 0,
            mtime_ns: 0,
            ctime_ns: 0,
            attrs: 0,
            volume: String::new(),
        };
        let ctx = EvalCtx {
            mm: &ctx_mm,
            path_lower: None,
            parent_lower: OnceCell::new(),
            audio: None,
            dirs: &dirs,
            phonetic: None,
            volumes: &volumes,
        };
        assert_eq!(ctx.parent_lower(&row), Some("archive"));
        // And the cache answers the same on the second read.
        assert_eq!(ctx.parent_lower(&row), Some("archive"));
    }

    #[test]
    fn normalize_needles_leaves_compiled_patterns_alone() {
        // Wildcards and regexes match against a target that stopped one
        // rung short, so rewriting their source here would desynchronize
        // the two sides.
        let q = parser::parse("*.TXT").unwrap();
        let before = format!("{:?}", q.root());
        let after = format!("{:?}", *normalize_needles(q.root(), &MatchMode::default()));
        assert_eq!(before, after);
    }

    #[test]
    fn normalize_needles_folds_the_needles_the_matchers_stopped_folding() {
        let punct = mode(|m| m.ignore_punctuation = true);
        let q = parser::parse("name^:My-Report").unwrap();
        match &*normalize_needles(q.root(), &punct) {
            QueryNode::Modifier(m) => match &m.kind {
                ModifierKind::NamePrefix(n) => assert_eq!(n, "myreport"),
                other => panic!("expected NamePrefix, got {other:?}"),
            },
            other => panic!("expected Modifier, got {other:?}"),
        }
    }
}
