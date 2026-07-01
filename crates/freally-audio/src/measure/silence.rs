//! Silence-ratio counter.
//!
//! Build Guide Phase 9: "% of samples below −60 dBFS." We treat each
//! interleaved sample independently — a frame that's silent on one
//! channel and loud on the other counts the silent half toward the
//! ratio. That matches the Phase 9 prompt's wording ("samples", not
//! "frames") and keeps the math channel-count-invariant.
//!
//! ## Cross-format reading
//!
//! A consequence of per-sample (not per-frame) accounting: a mono
//! file with one silent half-second and a stereo file with one silent
//! channel and one loud channel will report different silence ratios
//! for what users perceive as "the same shape." A mixed library
//! query like `silence:>50%` therefore biases toward stereo content
//! whose channels frequently disagree (one-track-of-the-pair quiet).
//! Tooling that wants strict per-frame silence (a frame is silent
//! iff *every* channel sample in that frame is silent) should
//! pre-mix the file to mono, or call into [`SilenceCounter::add`]
//! one frame at a time after computing the per-frame max abs.
//!
//! For the long-form podcast triage use case the prompt names —
//! "silence:>50% length:>30:00" — most podcast audio is
//! channel-correlated (or mono outright), so the difference is small
//! in practice. The unit test
//! `mono_and_correlated_stereo_report_same_ratio` regresses this.

/// −60 dBFS in linear amplitude. Any abs(sample) below this threshold
/// counts as silent.
pub const SILENCE_THRESHOLD_LINEAR: f32 = 0.001; // 10^(-60/20) ≈ 0.001

#[derive(Debug, Default, Clone)]
pub struct SilenceCounter {
    total: u64,
    silent: u64,
}

impl SilenceCounter {
    pub fn new() -> Self {
        Self::default()
    }

    /// Update counters with a slice of interleaved samples.
    pub fn add(&mut self, samples: &[f32]) {
        // The hot path — keep it branch-light. We deliberately use
        // `<` rather than `<=` so a sample at exactly the threshold
        // counts as audible (matches the conservative reading of "below
        // −60 dBFS").
        let mut silent = 0u64;
        for &s in samples {
            // `abs` on f32 is one instruction (`fabss` on x86, AND
            // mask on ARM); branchless and tiny.
            if s.abs() < SILENCE_THRESHOLD_LINEAR {
                silent += 1;
            }
        }
        self.silent += silent;
        self.total += samples.len() as u64;
    }

    pub fn total(&self) -> u64 {
        self.total
    }

    pub fn silent(&self) -> u64 {
        self.silent
    }

    /// `silent / total`, clamped to `[0.0, 1.0]`. Returns `0.0` when
    /// `total == 0` (empty input is conventionally "no silence
    /// observed" rather than "fully silent").
    pub fn ratio(&self) -> f32 {
        if self.total == 0 {
            0.0
        } else {
            (self.silent as f64 / self.total as f64) as f32
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn empty_input_yields_zero_ratio() {
        let c = SilenceCounter::new();
        assert_eq!(c.ratio(), 0.0);
    }

    #[test]
    fn pure_silence_is_one() {
        let mut c = SilenceCounter::new();
        c.add(&[0.0; 1024]);
        assert!((c.ratio() - 1.0).abs() < 1e-6);
    }

    #[test]
    fn full_scale_is_zero() {
        let mut c = SilenceCounter::new();
        c.add(&[0.5_f32; 1024]);
        assert!(c.ratio() < 1e-6);
    }

    #[test]
    fn half_silent_is_one_half() {
        let mut c = SilenceCounter::new();
        c.add(&[0.0_f32; 512]);
        c.add(&[0.5_f32; 512]);
        assert!((c.ratio() - 0.5).abs() < 1e-6);
    }

    #[test]
    fn threshold_boundary_is_audible() {
        let mut c = SilenceCounter::new();
        // Sample at exactly the threshold counts as audible (`<` not
        // `<=`). Counter must be `0`.
        c.add(&[SILENCE_THRESHOLD_LINEAR; 100]);
        assert_eq!(c.silent(), 0);
    }

    #[test]
    fn negative_samples_handled_via_abs() {
        let mut c = SilenceCounter::new();
        // `−0.5` is well above the threshold — the counter treats it
        // as audible by checking `abs(sample) < threshold`.
        c.add(&[-0.5_f32; 100]);
        assert!(c.ratio() < 1e-6);
    }

    #[test]
    fn just_below_threshold_is_silent() {
        let mut c = SilenceCounter::new();
        // Half the threshold — definitely silent.
        c.add(&[SILENCE_THRESHOLD_LINEAR * 0.5; 100]);
        assert_eq!(c.silent(), 100);
    }

    #[test]
    fn mono_and_correlated_stereo_report_same_ratio() {
        // A mono file (`[a, b, c, …]`) and a stereo file built by
        // duplicating each frame across both channels (`[a, a, b, b,
        // c, c, …]`) carry identical perceptual content. Per-sample
        // accounting (this counter's implementation) returns the same
        // ratio for both — that's the documented "channel-count-
        // invariant" property the module-level docs claim.
        let mono: Vec<f32> = (0..100).map(|i| if i < 50 { 0.0 } else { 0.5 }).collect();
        let stereo: Vec<f32> = mono.iter().flat_map(|&s| [s, s]).collect();
        let mut a = SilenceCounter::new();
        a.add(&mono);
        let mut b = SilenceCounter::new();
        b.add(&stereo);
        assert!((a.ratio() - b.ratio()).abs() < 1e-6);
    }
}
