//! Numeric-aware string ordering (SRC-M24).
//!
//! Byte ordering puts `file10` before `file2`, because `1` precedes `2`.
//! Natural ordering reads a run of digits as one number, so `file2`
//! comes first and `v1.9` precedes `v1.10`.
//!
//! The comparator works on bytes rather than chars. Every caller already
//! hands it either a lowercased name or a raw path, and for UTF-8 the
//! byte order and the code-point order are the same — so this changes
//! ordering only where digits are involved, which is the whole point.
//!
//! There is a twin of this in `apps/freally-ui/src/lib/util/natural.ts`,
//! because the result list re-sorts client-side. The two are not
//! identical — the UI compares non-digit runs with `localeCompare`, so
//! accents and case keep behaving the way they already did there, while
//! this side only ever sees a lowercased name. What they *must* agree on
//! is the numeric behaviour, and the vector in
//! `natural_matches_the_ui_twin` is mirrored in `natural.test.ts` so a
//! change made to one comparator and not the other fails on one side.

use std::cmp::Ordering;

/// Compare two strings so that embedded digit runs order numerically.
///
/// Total and stable: two strings that differ only in zero-padding
/// (`07` vs `7`) are ordered by padding as a last resort, so a sort never
/// leaves their relative order up to the input.
pub fn natural_cmp(a: &str, b: &str) -> Ordering {
    let (a, b) = (a.as_bytes(), b.as_bytes());
    let (mut i, mut j) = (0usize, 0usize);
    // First padding difference seen, applied only if everything else
    // ties. Deciding on padding the moment it is seen would let
    // `file07b` beat `file7a` on the padding of a number that is equal.
    let mut padding = Ordering::Equal;

    while i < a.len() && j < b.len() {
        if a[i].is_ascii_digit() && b[j].is_ascii_digit() {
            let ia = digit_run_end(a, i);
            let jb = digit_run_end(b, j);
            let (x, y) = (&a[i..ia], &b[j..jb]);
            match cmp_digit_run(x, y) {
                Ordering::Equal => {
                    if padding == Ordering::Equal {
                        padding = x.len().cmp(&y.len());
                    }
                    i = ia;
                    j = jb;
                }
                other => return other,
            }
        } else {
            // Covers the mixed case too: a digit byte and a non-digit
            // byte are never equal, so this returns rather than looping.
            match a[i].cmp(&b[j]) {
                Ordering::Equal => {
                    i += 1;
                    j += 1;
                }
                other => return other,
            }
        }
    }

    (a.len() - i).cmp(&(b.len() - j)).then(padding)
}

fn digit_run_end(s: &[u8], from: usize) -> usize {
    let mut k = from;
    while k < s.len() && s[k].is_ascii_digit() {
        k += 1;
    }
    k
}

/// Compare two digit runs by value, without parsing them.
///
/// Parsing would cap the comparison at `u64::MAX`; a 25-digit run in a
/// filename is unusual but not invalid, and it would then compare equal
/// to every other overlong run. Comparing significant-digit count and
/// then bytes is exact for any length.
fn cmp_digit_run(x: &[u8], y: &[u8]) -> Ordering {
    let xs = strip_leading_zeros(x);
    let ys = strip_leading_zeros(y);
    xs.len().cmp(&ys.len()).then_with(|| xs.cmp(ys))
}

fn strip_leading_zeros(v: &[u8]) -> &[u8] {
    let zeros = v.iter().take_while(|b| **b == b'0').count();
    if zeros == v.len() {
        // An all-zero run is the number zero — keep one digit so it
        // compares as a value rather than as an empty slice.
        &v[v.len() - 1..]
    } else {
        &v[zeros..]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn sorted(mut v: Vec<&str>) -> Vec<&str> {
        v.sort_by(|a, b| natural_cmp(a, b));
        v
    }

    #[test]
    fn digits_order_by_value_not_by_first_character() {
        assert_eq!(
            sorted(vec!["file10", "file2", "file1"]),
            vec!["file1", "file2", "file10"]
        );
    }

    #[test]
    fn version_numbers_order_segment_by_segment() {
        assert_eq!(
            sorted(vec!["v1.10", "v1.9", "v1.2"]),
            vec!["v1.2", "v1.9", "v1.10"]
        );
    }

    #[test]
    fn leading_zeros_do_not_change_the_value() {
        assert_eq!(natural_cmp("file007", "file7"), Ordering::Greater);
        assert_eq!(natural_cmp("file007a", "file7b"), Ordering::Less);
    }

    #[test]
    fn an_all_zero_run_is_the_number_zero() {
        assert_eq!(natural_cmp("f0", "f1"), Ordering::Less);
        assert_eq!(natural_cmp("f000", "f1"), Ordering::Less);
    }

    #[test]
    fn runs_longer_than_u64_still_compare_by_value() {
        let big = "f99999999999999999999999999999";
        let bigger = "f99999999999999999999999999999999";
        assert_eq!(natural_cmp(big, bigger), Ordering::Less);
    }

    #[test]
    fn a_prefix_sorts_before_what_extends_it() {
        assert_eq!(natural_cmp("file", "file1"), Ordering::Less);
        assert_eq!(natural_cmp("file1", "file"), Ordering::Greater);
    }

    #[test]
    fn strings_without_digits_keep_byte_order() {
        assert_eq!(natural_cmp("apple", "banana"), Ordering::Less);
        assert_eq!(natural_cmp("a", "a"), Ordering::Equal);
    }

    #[test]
    fn the_comparator_is_total() {
        // Every pair must be consistently ordered, or `sort_by` is free
        // to produce a different answer on a different input order.
        let items = ["a1", "a01", "a001", "a2", "a10", "b1", "a"];
        for x in items {
            for y in items {
                assert_eq!(
                    natural_cmp(x, y),
                    natural_cmp(y, x).reverse(),
                    "antisymmetry broken for {x:?} vs {y:?}"
                );
            }
        }
    }

    /// Mirrored in the UI twin's test. Deliberately all-lowercase and
    /// ASCII: the two comparators differ on case and accents by design
    /// (see the module note), so a shared vector may only exercise the
    /// numeric behaviour they are required to agree on.
    #[test]
    fn natural_matches_the_ui_twin() {
        assert_eq!(
            sorted(vec![
                "img12.png",
                "img10.png",
                "img2.png",
                "img1.png",
                "img3.png",
                "img.png",
            ]),
            vec![
                "img.png",
                "img1.png",
                "img2.png",
                "img3.png",
                "img10.png",
                "img12.png",
            ]
        );
    }
}
