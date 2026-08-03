// Numeric-aware string ordering (SRC-M24).
//
// Byte ordering puts `file10` before `file2`, because `1` precedes `2`.
// Natural ordering reads a run of digits as one number, so `file2` comes
// first and `v1.9` precedes `v1.10`.
//
// Twin of `crates/freally-query/src/natural.rs`, which the daemon uses.
// The two are not identical: the non-digit runs go through
// `localeCompare` here so accents and case keep behaving the way the
// result list already behaved, while the Rust side only ever sees a
// lowercased name. What they must agree on is the numeric behaviour, and
// the vector in `natural.test.ts` is mirrored in that file's
// `natural_matches_the_ui_twin`.

/** ASCII digit test by code point.
 *
 * A `/\d/.test(s[k])` here allocates a one-character string and enters
 * the regex engine for every character of every comparison, and this
 * runs O(n log n) times per sort. */
function isDigit(s: string, i: number): boolean {
  const c = s.charCodeAt(i);
  return c >= 48 && c <= 57;
}

/** A regex is fine for a one-shot "does this string contain a digit". */
const DIGITS = /\d/;

/** Reused across every comparison. Constructing a collator per chunk —
 *  which is what a bare `localeCompare` does — is roughly an order of
 *  magnitude slower. */
const COLLATOR = new Intl.Collator(undefined, { sensitivity: "variant" });

/** Length of the run starting at `i` — all digits, or all non-digits. */
function runEnd(s: string, i: number, digits: boolean): number {
  let k = i;
  while (k < s.length && isDigit(s, k) === digits) k += 1;
  return k;
}

/** Compare two digit runs by value, without parsing them.
 *
 * `Number()` would lose precision past 2^53; a long run of digits in a
 * filename is unusual but not invalid, and two different overlong runs
 * would then compare equal. Comparing significant-digit count and then
 * the digits themselves is exact at any length. */
function compareDigitRun(x: string, y: string): number {
  const xs = x.replace(/^0+(?=\d)/, "");
  const ys = y.replace(/^0+(?=\d)/, "");
  if (xs.length !== ys.length) return xs.length - ys.length;
  return xs < ys ? -1 : xs > ys ? 1 : 0;
}

/**
 * Compare two strings so that embedded digit runs order numerically.
 *
 * Total and stable: strings differing only in zero-padding (`07` vs `7`)
 * are ordered by padding as a last resort, so a sort never leaves their
 * relative order up to the input.
 */
export function naturalCompare(a: string, b: string): number {
  // Nothing to be numeric about — hand the whole string to the collator
  // rather than to the chunked path below, which can only compare a
  // prefix at a time and would lose the context a collator needs for
  // contractions and accents.
  if (!DIGITS.test(a) && !DIGITS.test(b)) return COLLATOR.compare(a, b);

  let i = 0;
  let j = 0;
  // First padding difference seen, applied only if everything else ties.
  // Deciding on it the moment it appears would let `file07b` beat
  // `file7a` on the padding of a number that is equal.
  let padding = 0;

  while (i < a.length && j < b.length) {
    const aDigit = isDigit(a, i);
    const bDigit = isDigit(b, j);

    if (aDigit && bDigit) {
      const ia = runEnd(a, i, true);
      const jb = runEnd(b, j, true);
      const x = a.slice(i, ia);
      const y = b.slice(j, jb);
      const n = compareDigitRun(x, y);
      if (n !== 0) return n;
      if (padding === 0) padding = x.length - y.length;
      i = ia;
      j = jb;
      continue;
    }

    // Compare the non-digit stretch, but only as far as both sides have
    // one. A run can be cut short by a digit on one side and not the
    // other — `img.png` against `img1.png` gives runs of 7 and 3 — and
    // comparing those whole would answer on length instead of on `.`
    // against `1`, putting `img.png` last.
    const ia = aDigit ? i + 1 : runEnd(a, i, false);
    const jb = bDigit ? j + 1 : runEnd(b, j, false);
    const n = Math.min(ia - i, jb - j);
    const c = COLLATOR.compare(a.slice(i, i + n), b.slice(j, j + n));
    if (c !== 0) return c;
    i += n;
    j += n;
  }

  const rest = a.length - i - (b.length - j);
  return rest !== 0 ? rest : padding;
}
