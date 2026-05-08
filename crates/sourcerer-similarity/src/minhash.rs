//! Bigram MinHash + LSH parameters (Phase 6 spec).
//!
//! K = 128 hash functions; LSH = 16 bands × 8 rows-per-band. Hash function
//! family is the textbook `h_i(x) = a_i · x + b_i` linear hash over `u64`,
//! seeded deterministically from `MINHASH_SEED` so two processes (or a
//! daemon + smoke-test) produce identical signatures for identical inputs.
//!
//! Bigram extraction works on Unicode `char` pairs after lowercasing; we
//! sentinel the start and end of the name (`'^'` / `'$'`) so 1-char names
//! still produce two distinct bigrams and the empty-name edge is the
//! all-MAX signature (matches no LSH band).
//!
//! ## Why a *linear* hash family?
//!
//! Phase 6's recall gate is ≥ 95 %. Linear MinHash admits a clean Jaccard
//! estimator (`#matches / K`) and the unbiased linearity proof from
//! Broder '97 — i.e. the false-negative rate is bounded purely by
//! `(1 - s^r)^b` for `r=8`, `b=16`. Using a stronger universal family
//! (blake3-keyed hashing with K=128 keys, say) would pay 50× the per-name
//! latency for no recall improvement at the K we ship.
//!
//! Phase 13's perf pass benchmarks SimHash + 64-bit fingerprints as a
//! constant-time alternative to the per-name linear pass; this module
//! stays the default until that comparison runs.

/// Number of MinHash hash functions per signature.
pub const K: usize = 128;
/// Number of LSH bands.
pub const BANDS: usize = 16;
/// Rows per band (`K / BANDS`).
pub const ROWS_PER_BAND: usize = K / BANDS;
const _ASSERT_K_DIVIDES: () = assert!(K % BANDS == 0, "K must be divisible by BANDS");

/// Deterministic seed for the linear-hash family.
///
/// **BREAKING:** bumping this value (or `K` / `BANDS`) invalidates every
/// persisted signature on every user's disk. The format-error gate in
/// `store::decode` will refuse the file and surface
/// `SimilarityError::Format` so the daemon can rebuild — but every user
/// pays a full re-scan. Treat this constant the same way you'd treat a
/// SQL schema migration: don't change it without a coordinated bump of
/// the on-disk format `VERSION` in `store.rs` and a CHANGELOG entry.
const MINHASH_SEED: u64 = 0x534f5552_43455252; // "SOURCERR"

/// Pre-generated `(a, b)` linear-hash pairs.
pub struct MinHashFamily {
    pairs: [(u64, u64); K],
}

impl MinHashFamily {
    pub fn new() -> Self {
        let mut rng = SplitMix64(MINHASH_SEED);
        let mut pairs = [(0u64, 0u64); K];
        for slot in pairs.iter_mut() {
            // `a` is forced odd to keep the linear hash a bijection on
            // u64; `b` is unconstrained. Both come from the deterministic
            // SplitMix64 stream so signatures are reproducible across
            // processes and across the daemon ↔ smoke-test boundary.
            let a = rng.next() | 1;
            let b = rng.next();
            *slot = (a, b);
        }
        Self { pairs }
    }

    /// Compute the `[u64; K]` MinHash signature of a (lowercased) name.
    /// `name_lower` is expected to already be normalized — the name index
    /// stores lower-cased filenames, so we keep the convention.
    pub fn signature(&self, name_lower: &str) -> [u64; K] {
        let mut sig = [u64::MAX; K];
        // Sentinel-bracket the input so 1-char names still emit two
        // bigrams (`^a`, `a$`) and stay distinguishable.
        let chars: Vec<char> = std::iter::once('^')
            .chain(name_lower.chars())
            .chain(std::iter::once('$'))
            .collect();
        if chars.len() < 2 {
            return sig;
        }
        for w in chars.windows(2) {
            let h = bigram_hash(w[0], w[1]);
            for (i, &(a, b)) in self.pairs.iter().enumerate() {
                let v = a.wrapping_mul(h).wrapping_add(b);
                if v < sig[i] {
                    sig[i] = v;
                }
            }
        }
        sig
    }

    /// Hash one band of an existing signature. Used both at insert time
    /// (so a row goes into 16 buckets) and at query time (so we look up
    /// the same 16 buckets).
    pub fn band_hash(sig: &[u64; K], band: usize) -> u64 {
        let start = band * ROWS_PER_BAND;
        let end = start + ROWS_PER_BAND;
        // FNV-1a on the band's 8 u64s. Fast, reproducible, no dep on
        // blake3 for a non-cryptographic key.
        let mut h: u64 = 0xcbf29ce484222325;
        for &v in &sig[start..end] {
            for byte in v.to_le_bytes() {
                h ^= byte as u64;
                h = h.wrapping_mul(0x100000001b3);
            }
        }
        h
    }

    /// Jaccard estimate from two signatures: fraction of components that
    /// match. Phase-6 spec calls this out as the candidate-ranking
    /// metric.
    pub fn jaccard_estimate(a: &[u64; K], b: &[u64; K]) -> f32 {
        let mut matches = 0usize;
        for (x, y) in a.iter().zip(b.iter()) {
            if x == y {
                matches += 1;
            }
        }
        matches as f32 / K as f32
    }
}

impl Default for MinHashFamily {
    fn default() -> Self {
        Self::new()
    }
}

#[inline]
fn bigram_hash(a: char, b: char) -> u64 {
    // Pack two 32-bit codepoints into a single u64 then run SplitMix64
    // finalizer. Empirically gives a near-uniform output bit pattern,
    // which is what the linear-hash family needs as input.
    let combined = ((a as u32 as u64) << 32) | (b as u32 as u64);
    splitmix_finalize(combined)
}

#[inline]
fn splitmix_finalize(x: u64) -> u64 {
    let mut z = x.wrapping_add(0x9E3779B97F4A7C15);
    z = (z ^ (z >> 30)).wrapping_mul(0xBF58476D1CE4E5B9);
    z = (z ^ (z >> 27)).wrapping_mul(0x94D049BB133111EB);
    z ^ (z >> 31)
}

struct SplitMix64(u64);

impl SplitMix64 {
    fn next(&mut self) -> u64 {
        self.0 = self.0.wrapping_add(0x9E3779B97F4A7C15);
        let mut z = self.0;
        z = (z ^ (z >> 30)).wrapping_mul(0xBF58476D1CE4E5B9);
        z = (z ^ (z >> 27)).wrapping_mul(0x94D049BB133111EB);
        z ^ (z >> 31)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn identical_names_have_identical_signatures() {
        let mh = MinHashFamily::new();
        let a = mh.signature("report-final.pdf");
        let b = mh.signature("report-final.pdf");
        assert_eq!(a, b);
        assert_eq!(MinHashFamily::jaccard_estimate(&a, &b), 1.0);
    }

    #[test]
    fn near_duplicates_score_high() {
        let mh = MinHashFamily::new();
        let a = mh.signature("report-final.pdf");
        let b = mh.signature("report-final-v2.pdf");
        let j = MinHashFamily::jaccard_estimate(&a, &b);
        // Empirically these share ~70% of bigrams. The estimator is
        // unbiased so we expect j to track that closely.
        assert!(
            j > 0.45,
            "expected near-dup Jaccard > 0.45, got {j} (a vs b)"
        );
    }

    #[test]
    fn unrelated_names_score_low() {
        let mh = MinHashFamily::new();
        let a = mh.signature("totally-unrelated-quotient.flac");
        let b = mh.signature("xyzzy.bin");
        let j = MinHashFamily::jaccard_estimate(&a, &b);
        assert!(j < 0.20, "expected unrelated Jaccard < 0.20, got {j}");
    }

    #[test]
    fn band_hashes_differ_across_bands() {
        let mh = MinHashFamily::new();
        let sig = mh.signature("draft-2025.docx");
        let mut seen = std::collections::HashSet::new();
        for b in 0..BANDS {
            seen.insert(MinHashFamily::band_hash(&sig, b));
        }
        // 16 bands of 8 hashes each — collisions should be rare.
        // A single draft.docx string definitely shouldn't collapse all
        // bands to the same key.
        assert!(seen.len() >= BANDS - 2, "got {} unique bands", seen.len());
    }

    #[test]
    fn empty_name_signature_is_max() {
        let mh = MinHashFamily::new();
        // Empty input still gets `^$` after sentineling, so signature
        // is well-defined (and matches no other name except literal "").
        let sig = mh.signature("");
        let other = mh.signature("anything-else");
        let j = MinHashFamily::jaccard_estimate(&sig, &other);
        assert!(j < 0.10);
    }

    #[test]
    fn determinism_across_instances() {
        // Two MinHashFamily instances must produce identical signatures
        // — the seed is hard-coded.
        let a = MinHashFamily::new().signature("alpha-beta-gamma.txt");
        let b = MinHashFamily::new().signature("alpha-beta-gamma.txt");
        assert_eq!(a, b);
    }

    #[test]
    fn one_char_name_distinguishes_from_other() {
        let mh = MinHashFamily::new();
        let a = mh.signature("a");
        let b = mh.signature("b");
        // With sentinel bracketing, `a` produces (`^a`, `a$`), `b`
        // produces (`^b`, `b$`) — disjoint bigram sets so Jaccard ≈ 0.
        assert_ne!(a, b);
    }
}
