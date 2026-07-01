//! Phase 10 query optimizer.
//!
//! Three transformations:
//!
//! 1. **AND-child reordering by selectivity.** In an `AND` chain we
//!    place cheap predicates first so short-circuit evaluation picks
//!    them up. The sort is stable — equal-rank children retain their
//!    source order so users keep deterministic plans.
//! 2. **Lens routing.** A query that contains only audio-shaped
//!    predicates (and the always-true scaffolding) can skip the
//!    filename-trigram pre-filter entirely; the executor enumerates
//!    live rows directly. Surfaced via [`is_audio_only_route`] /
//!    [`is_similarity_only_route`].
//! 3. **Short-circuit AND eval.** The reorder above sets up the
//!    plan; the executor's `iter().all(...)` chain short-circuits
//!    as a side-effect.
//!
//! `parse()` keeps producing the user's source-ordered AST so the
//! 50-query voidtools fixture (Standing Rule #8) and the parser
//! unit tests stay untouched. Optimization happens at execute-time
//! (and inside `PlanCache::get_or_plan`) so every actual run
//! benefits without changing the user-visible AST shape.

use crate::ast::{LensKind, ModifierKind, Query, QueryNode, TextPattern};

/// Optimize a parsed query: AND children reordered by selectivity,
/// recursing through OR / Not / Lens. Returns a fresh `Query` (the
/// original is unchanged).
pub fn optimize(q: &Query) -> Query {
    let new_root = optimize_node(q.root());
    Query::new(q.source().to_string(), new_root)
}

fn optimize_node(node: &QueryNode) -> QueryNode {
    match node {
        QueryNode::And(parts) => {
            let mut children: Vec<QueryNode> = parts.iter().map(optimize_node).collect();
            // Stable sort so equal-rank children retain their source
            // order. Phase 11's plan-explainer surfaces both the
            // pre-optimization order and the reordered plan.
            children.sort_by_key(|c| selectivity_rank(c) as i32);
            QueryNode::And(children)
        }
        QueryNode::Or(parts) => QueryNode::Or(parts.iter().map(optimize_node).collect()),
        QueryNode::Not(inner) => QueryNode::Not(Box::new(optimize_node(inner))),
        QueryNode::Lens { kind, inner } => QueryNode::Lens {
            kind: *kind,
            inner: Box::new(optimize_node(inner)),
        },
        leaf @ (QueryNode::Text(_)
        | QueryNode::Modifier(_)
        | QueryNode::QuickFilter(_)
        | QueryNode::True) => leaf.clone(),
    }
}

/// Selectivity rank — lower runs first. Calibrated against the
/// Phase 5/6/9 executor's per-predicate cost: name-buffer-only
/// predicates rank lower than predicates that need full-row
/// hydration; predicates that hit a per-row provider (audio cache,
/// similarity LSH) rank highest. Stable across builds — Phase 11's
/// plan-explainer renders this rank to the user.
pub fn selectivity_rank(node: &QueryNode) -> u8 {
    match node {
        QueryNode::True => 0,
        // Tier 1: name-buffer-only.
        QueryNode::Text(TextPattern::Literal(_)) => 10,
        QueryNode::QuickFilter(_) => 11,
        QueryNode::Modifier(m) => match &m.kind {
            // `child:` is `name:`-equivalent — cheap, name-buffer-only.
            ModifierKind::Child(_) => 12,
            // Extension lookup is a bytewise compare on the name's
            // tail; cheap.
            ModifierKind::Ext(_) => 13,
            // Hydration-required, but compact byte/numeric compares.
            ModifierKind::Size { .. } => 20,
            ModifierKind::Date(_) => 22,
            ModifierKind::Attrib(_) => 24,
            ModifierKind::Path(_) | ModifierKind::Parent(_) => 30,
            // Per-row provider lookup — the most expensive.
            ModifierKind::Audio(_) => 80,
            // LSH lookup is per-query, not per-row, but the routing
            // cost dominates so it sorts late among siblings.
            ModifierKind::Similar(_) => 85,
            ModifierKind::Reserved { .. } => 90,
        },
        QueryNode::Text(TextPattern::Wildcard { .. }) => 40,
        QueryNode::Text(TextPattern::Regex { .. }) => 50,
        QueryNode::Not(inner) => selectivity_rank(inner).saturating_add(2),
        QueryNode::And(_) | QueryNode::Or(_) => 60,
        // Lens scopes are evaluated by their inner; rank them just
        // above mid-tier so a Lens-wrapped audio block still sorts
        // after literals / extensions.
        QueryNode::Lens { .. } => 70,
    }
}

/// True when the query has no name-side predicate at all — it's
/// audio-only and the executor can skip the trigram pre-filter
/// because every live row is a candidate. Pure audio-modifier
/// scaffolding (incl. `audio:(...)` lens prefixes that wrap audio
/// modifiers) qualifies.
pub fn is_audio_only_route(node: &QueryNode) -> bool {
    fn all_audio(node: &QueryNode) -> bool {
        match node {
            QueryNode::True => true,
            QueryNode::Modifier(m) => matches!(m.kind, ModifierKind::Audio(_)),
            QueryNode::And(parts) => parts.iter().all(all_audio),
            // Lens scopes are transparent — recurse.
            QueryNode::Lens { inner, .. } => all_audio(inner),
            QueryNode::Not(inner) => all_audio(inner),
            // OR — if every disjunct is audio, the union is too.
            QueryNode::Or(parts) => parts.iter().all(all_audio),
            // Anything name-shaped breaks the all-audio property.
            QueryNode::Text(_) | QueryNode::QuickFilter(_) => false,
        }
    }
    all_audio(node)
}

/// True when the query routes through the similarity lens (a
/// top-level `Similar` modifier or a `similar:(...)` lens scope at
/// the root or as a top-level AND child).
pub fn is_similarity_route(node: &QueryNode) -> bool {
    match node {
        QueryNode::Modifier(m) => matches!(m.kind, ModifierKind::Similar(_)),
        QueryNode::Lens { kind, .. } => matches!(kind, LensKind::Similar),
        QueryNode::And(parts) => parts.iter().any(is_similarity_route),
        _ => false,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::parser::parse;

    fn ranked(s: &str) -> Vec<u8> {
        let q = parse(s).expect("parse");
        match q.root() {
            QueryNode::And(parts) => parts.iter().map(selectivity_rank).collect(),
            _ => panic!("expected And"),
        }
    }

    fn ranked_optimized(s: &str) -> Vec<u8> {
        let q = optimize(&parse(s).expect("parse"));
        match q.root() {
            QueryNode::And(parts) => parts.iter().map(selectivity_rank).collect(),
            n => panic!("expected And, got {n:?}"),
        }
    }

    #[test]
    fn and_children_reorder_by_selectivity() {
        // Pre-opt: regex (50), literal (10), size (20). Post-opt
        // should be sorted ascending: 10, 20, 50.
        let pre = ranked("regex:^foo bar size:>1mb");
        let post = ranked_optimized("regex:^foo bar size:>1mb");
        assert_eq!(pre, vec![50, 10, 20]);
        assert_eq!(post, vec![10, 20, 50]);
    }

    #[test]
    fn equal_rank_children_keep_source_order() {
        // Two literals at rank 10 — the optimizer is a stable sort,
        // so source order survives.
        let q = optimize(&parse("alpha beta").expect("parse"));
        match q.root() {
            QueryNode::And(parts) => match (&parts[0], &parts[1]) {
                (
                    QueryNode::Text(TextPattern::Literal(a)),
                    QueryNode::Text(TextPattern::Literal(b)),
                ) => {
                    assert_eq!(a, "alpha");
                    assert_eq!(b, "beta");
                }
                other => panic!("unexpected: {other:?}"),
            },
            n => panic!("expected And: {n:?}"),
        }
    }

    #[test]
    fn or_children_keep_source_order() {
        // OR doesn't get reordered — order matters for stable hit
        // ordering.
        let q = optimize(&parse("alpha OR beta").expect("parse"));
        match q.root() {
            QueryNode::Or(parts) => match (&parts[0], &parts[1]) {
                (
                    QueryNode::Text(TextPattern::Literal(a)),
                    QueryNode::Text(TextPattern::Literal(b)),
                ) => {
                    assert_eq!(a, "alpha");
                    assert_eq!(b, "beta");
                }
                other => panic!("unexpected: {other:?}"),
            },
            n => panic!("expected Or: {n:?}"),
        }
    }

    #[test]
    fn lens_inner_reorders() {
        // Reordering recurses into Lens scopes.
        let q = optimize(&parse("audio:(rate:>=44100 codec:flac)").expect("parse"));
        match q.root() {
            QueryNode::Lens { inner, .. } => match inner.as_ref() {
                QueryNode::And(parts) => {
                    let ranks: Vec<u8> = parts.iter().map(selectivity_rank).collect();
                    let mut sorted = ranks.clone();
                    sorted.sort();
                    assert_eq!(ranks, sorted);
                }
                n => panic!("inner should be And: {n:?}"),
            },
            n => panic!("expected Lens: {n:?}"),
        }
    }

    #[test]
    fn audio_only_route_detected_for_pure_audio() {
        let q = parse("lufs:<-14 codec:flac").unwrap();
        assert!(is_audio_only_route(q.root()));
    }

    #[test]
    fn audio_only_route_rejects_name_predicate() {
        let q = parse("song lufs:<-14").unwrap();
        assert!(!is_audio_only_route(q.root()));
    }

    #[test]
    fn audio_only_route_through_lens_scope() {
        let q = parse("audio:(lufs:<-14 codec:flac)").unwrap();
        assert!(is_audio_only_route(q.root()));
    }

    #[test]
    fn similarity_route_detected_at_root() {
        let q = parse("similar:report-final").unwrap();
        assert!(is_similarity_route(q.root()));
    }

    #[test]
    fn similarity_route_detected_through_lens_scope() {
        let q = parse("similar:(report-final)").unwrap();
        assert!(is_similarity_route(q.root()));
    }

    #[test]
    fn audio_modifier_sorts_after_literals() {
        // `lufs:<-14 song` — literal (10) should land before audio (80)
        // after optimization regardless of source order.
        let q = optimize(&parse("lufs:<-14 song").expect("parse"));
        match q.root() {
            QueryNode::And(parts) => {
                assert!(matches!(parts[0], QueryNode::Text(_)));
                assert!(matches!(parts[1], QueryNode::Modifier(_)));
            }
            n => panic!("expected And: {n:?}"),
        }
    }
}
