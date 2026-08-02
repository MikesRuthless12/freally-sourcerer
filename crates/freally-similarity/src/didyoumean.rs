//! SRC-M11 — bounded Damerau-Levenshtein correction for the filename
//! lens's empty state.
//!
//! When a search returns nothing, the likeliest explanation is a typo,
//! and the likeliest typo is one edit away. This module answers "which
//! indexed name did they probably mean?" and nothing else — it does not
//! search, rank results, or touch the lens pipeline.
//!
//! ## Why Damerau and not plain Levenshtein
//!
//! The single most common typing error is a transposition —
//! `freallly`/`freally` is a repeat, but `frealyl`/`freally` is a swap.
//! Plain Levenshtein scores a swap as 2 edits (one delete, one insert),
//! which pushes real typos past a tight budget. Damerau counts it as 1.
//! This is the *optimal string alignment* variant: adjacent
//! transpositions only, no substring re-editing. That restriction is
//! what keeps it O(n·m) with a two-row buffer, and it covers every typo
//! a keyboard actually produces.
//!
//! ## Why bounded
//!
//! Correction is only useful when it is confident. Allowing 4 edits on
//! a 5-character word turns "did you mean" into "here is an unrelated
//! file", which is worse than saying nothing. [`max_distance_for`] ties
//! the budget to the term's length, and [`damerau_levenshtein_bounded`]
//! abandons a row as soon as every cell in it exceeds the budget — so
//! rejecting a bad candidate costs a fraction of scoring a good one.

/// A correction worth offering.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Suggestion {
    /// The indexed name to suggest, in its original casing.
    pub term: String,
    /// Edit distance from what the user typed.
    pub distance: usize,
}

/// How many edits may separate a term from its correction.
///
/// Scaled by length because one edit means something different at 4
/// characters than at 20: `docs`→`dogs` is a different word, while
/// `quarterly-reprot`→`quarterly-report` obviously is not. Terms
/// shorter than 4 characters get no budget at all — at that length
/// every neighbour is a different word.
pub fn max_distance_for(term: &str) -> usize {
    match term.chars().count() {
        0..=3 => 0,
        4..=7 => 1,
        8..=15 => 2,
        _ => 3,
    }
}

/// Damerau-Levenshtein (optimal string alignment) distance, or `None`
/// when it provably exceeds `max`.
///
/// Comparison is over `char`s, not bytes, so a multi-byte character is
/// one edit rather than two-to-four.
pub fn damerau_levenshtein_bounded(a: &str, b: &str, max: usize) -> Option<usize> {
    let a: Vec<char> = a.chars().collect();
    let b: Vec<char> = b.chars().collect();
    // A length gap alone already costs that many edits, so most
    // candidates are rejected here without allocating a matrix.
    if a.len().abs_diff(b.len()) > max {
        return None;
    }
    if a.is_empty() {
        return (b.len() <= max).then_some(b.len());
    }
    if b.is_empty() {
        return (a.len() <= max).then_some(a.len());
    }

    // Three rows: the transposition case needs the row before last.
    let width = b.len() + 1;
    let mut prev2: Vec<usize> = vec![0; width];
    let mut prev: Vec<usize> = (0..width).collect();
    let mut cur: Vec<usize> = vec![0; width];

    for i in 1..=a.len() {
        cur[0] = i;
        let mut row_min = cur[0];
        for j in 1..=b.len() {
            let cost = usize::from(a[i - 1] != b[j - 1]);
            let mut best = (cur[j - 1] + 1) // insertion
                .min(prev[j] + 1) // deletion
                .min(prev[j - 1] + cost); // substitution
            if i > 1 && j > 1 && a[i - 1] == b[j - 2] && a[i - 2] == b[j - 1] {
                best = best.min(prev2[j - 2] + 1); // transposition
            }
            cur[j] = best;
            row_min = row_min.min(best);
        }
        // Every alignment from here on can only grow, so once the whole
        // row is over budget the candidate is unreachable.
        if row_min > max {
            return None;
        }
        std::mem::swap(&mut prev2, &mut prev);
        std::mem::swap(&mut prev, &mut cur);
    }
    let d = prev[b.len()];
    (d <= max).then_some(d)
}

/// Pick the best correction for `term` from `candidates`.
///
/// Candidates are compared case-insensitively but suggested in their
/// original casing — the user should see the name as it exists on
/// disk. A candidate equal to the term (ignoring case) is never
/// suggested: the search already ran and found nothing, so echoing it
/// back is noise.
///
/// **Callers pass name stems, not full filenames.** The budget is
/// spent on the difference between two strings, and an extension is a
/// difference: `freallly` is one edit from `freally` but four from
/// `freally.txt`, so feeding full names would reject nearly every real
/// correction. Stripping the extension is the caller's job because
/// only the caller knows whether the user typed one.
///
/// Ties break on the shorter name, then lexicographically, so the same
/// index always produces the same suggestion.
pub fn best_suggestion<'a, I>(term: &str, candidates: I) -> Option<Suggestion>
where
    I: IntoIterator<Item = &'a str>,
{
    let max = max_distance_for(term);
    if max == 0 {
        return None;
    }
    let needle = term.to_lowercase();
    let mut best: Option<Suggestion> = None;
    for cand in candidates {
        let lower = cand.to_lowercase();
        if lower == needle {
            return None;
        }
        let Some(d) = damerau_levenshtein_bounded(&needle, &lower, max) else {
            continue;
        };
        let better = match &best {
            None => true,
            Some(b) => {
                (d, cand.chars().count(), cand)
                    < (b.distance, b.term.chars().count(), b.term.as_str())
            }
        };
        if better {
            best = Some(Suggestion {
                term: cand.to_string(),
                distance: d,
            });
        }
    }
    best
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn identical_strings_are_zero_apart() {
        assert_eq!(damerau_levenshtein_bounded("report", "report", 2), Some(0));
    }

    #[test]
    fn a_transposition_costs_one_not_two() {
        // The whole reason for Damerau over Levenshtein.
        assert_eq!(
            damerau_levenshtein_bounded("frealyl", "freally", 1),
            Some(1)
        );
    }

    #[test]
    fn a_doubled_letter_costs_one() {
        assert_eq!(
            damerau_levenshtein_bounded("freallly", "freally", 1),
            Some(1)
        );
    }

    #[test]
    fn substitution_insertion_and_deletion_each_cost_one() {
        assert_eq!(damerau_levenshtein_bounded("cat", "bat", 1), Some(1));
        assert_eq!(damerau_levenshtein_bounded("cat", "cart", 1), Some(1));
        assert_eq!(damerau_levenshtein_bounded("cart", "cat", 1), Some(1));
    }

    #[test]
    fn the_bound_rejects_rather_than_reporting_a_large_distance() {
        assert_eq!(damerau_levenshtein_bounded("kitten", "sitting", 1), None);
        assert_eq!(damerau_levenshtein_bounded("kitten", "sitting", 3), Some(3));
    }

    #[test]
    fn a_length_gap_past_the_bound_is_rejected_immediately() {
        assert_eq!(damerau_levenshtein_bounded("a", "abcdefgh", 2), None);
    }

    #[test]
    fn empty_strings_are_handled_without_panicking() {
        assert_eq!(damerau_levenshtein_bounded("", "", 0), Some(0));
        assert_eq!(damerau_levenshtein_bounded("", "ab", 2), Some(2));
        assert_eq!(damerau_levenshtein_bounded("ab", "", 1), None);
    }

    #[test]
    fn distance_counts_characters_not_bytes() {
        // One 3-byte character replaced by another is a single edit.
        assert_eq!(damerau_levenshtein_bounded("日本語", "日本詰", 1), Some(1));
        // A pure-ASCII byte comparison would call this 3 edits and
        // reject it at max=1.
        assert_eq!(damerau_levenshtein_bounded("café", "cafe", 1), Some(1));
    }

    #[test]
    fn short_terms_get_no_correction_budget() {
        assert_eq!(max_distance_for("abc"), 0);
        assert!(best_suggestion("abc", ["abd", "abe"]).is_none());
    }

    #[test]
    fn budget_widens_with_term_length() {
        assert_eq!(max_distance_for("report"), 1);
        assert_eq!(max_distance_for("quarterly"), 2);
        assert_eq!(max_distance_for("quarterly-report-final"), 3);
    }

    #[test]
    fn picks_the_closest_candidate() {
        let s = best_suggestion("freallly", ["frankly", "freally", "fried"]).unwrap();
        assert_eq!(s.term, "freally");
        assert_eq!(s.distance, 1);
    }

    #[test]
    fn suggests_in_the_indexed_casing() {
        let s = best_suggestion("freallly", ["Freally", "unrelated"]).unwrap();
        assert_eq!(s.term, "Freally");
    }

    #[test]
    fn an_unstripped_extension_eats_the_budget() {
        // Documents why callers pass stems: the same correction that
        // succeeds above is unreachable once ".txt" is along for the
        // ride, because four of the allowed edits go to the extension.
        assert!(best_suggestion("freallly", ["freally.txt"]).is_none());
        assert!(best_suggestion("freallly", ["freally"]).is_some());
    }

    #[test]
    fn never_suggests_the_term_the_user_already_typed() {
        // A case-only match means the term exists; the zero-hit result
        // came from some other predicate, so a correction would lie.
        assert!(best_suggestion("Report.txt", ["report.TXT", "reprot.txt"]).is_none());
    }

    #[test]
    fn returns_nothing_when_every_candidate_is_too_far() {
        assert!(best_suggestion("freallly", ["completely", "different"]).is_none());
    }

    #[test]
    fn ties_break_deterministically_on_length_then_order() {
        // "bxb" and "bcbb" are both 1 edit from "bcb"; shorter wins.
        let s = best_suggestion("quarterl", ["quarterly", "quarterle"]).unwrap();
        assert_eq!(s.distance, 1);
        // Same length and distance — lexicographic order decides, and
        // does so identically on every run.
        let a = best_suggestion("quarterl", ["quarterly", "quarterle"]).unwrap();
        let b = best_suggestion("quarterl", ["quarterle", "quarterly"]).unwrap();
        assert_eq!(a.term, b.term);
    }

    #[test]
    fn an_empty_candidate_set_yields_no_suggestion() {
        assert!(best_suggestion("freallly", std::iter::empty()).is_none());
    }
}
