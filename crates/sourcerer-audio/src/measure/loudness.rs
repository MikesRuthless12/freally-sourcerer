//! Loudness measurement (EBU R128) wrapper around the `ebur128`
//! crate.
//!
//! Build Guide Phase 9: "two-pass: integrated LUFS, short-term P99
//! LUFS." We run a single-pass online measurement (ebur128 streams)
//! and tap `loudness_shortterm()` every `SHORT_TERM_SAMPLE_INTERVAL_MS`
//! to record a percentile-able series. After the file is fully
//! consumed, `loudness_global()` produces the integrated value and the
//! sorted short-term series gives us P99 + P10 → dynamic range.
//!
//! `ebur128`'s `Mode::TRUE_PEAK` flag enables the same crate's 4×
//! polyphase oversampler — that's the Phase 9 "true peak via 4×
//! oversampling" requirement satisfied without a hand-rolled FIR.

use ebur128::{EbuR128, Mode};

use crate::error::AudioError;

/// Standard EBU R128 short-term sliding window is 3 s; the spec's
/// recommended sampling interval for a percentile read-out is 100 ms.
/// This sets the resolution of `dynamic_range_lu`.
pub const SHORT_TERM_SAMPLE_INTERVAL_MS: u32 = 100;

/// Online accumulator. Push interleaved f32 frames via [`feed`]; once
/// the file is exhausted call [`finish`] to read out the integrated +
/// short-term percentiles + true peak in dBFS.
pub struct LoudnessAccumulator {
    inner: EbuR128,
    channels: u32,
    sample_rate: u32,
    /// Frames since the last short-term tap. Reset at every interval.
    frames_since_tap: u32,
    /// Frames per short-term tap (`sample_rate * interval_ms / 1000`).
    frames_per_tap: u32,
    /// Captured short-term LUFS samples (after the first 3-second
    /// settle).
    short_term_samples: Vec<f32>,
    /// Frames pushed in total. Used to skip percentile sampling
    /// before the 3-second window has filled.
    frames_total: u64,
}

// `channels` and `sample_rate` are loaded from `params` once and
// referenced throughout `feed` / `finish`. Surfacing them as accessors
// would invite Phase 11 callers to query the accumulator's geometry
// — but Phase 9 has no consumer for that and the dead-code lint
// flagged it; the fields stay private for the analyzer's use only.
#[allow(dead_code)]
const fn _phase_9_loudness_geometry_is_private() {}

/// Final loudness measurements once [`LoudnessAccumulator::finish`]
/// runs. All LUFS values are in `LUFS` (`f32::NEG_INFINITY` when the
/// file is too quiet for ebur128 to produce a finite value).
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct LoudnessReport {
    pub integrated: f32,
    pub short_term_p99: f32,
    pub short_term_p10: f32,
    /// True peak in dBFS, max over channels (4× oversampled by
    /// ebur128).
    pub true_peak_dbfs: f32,
}

impl LoudnessAccumulator {
    pub fn new(channels: u16, sample_rate: u32) -> Result<Self, AudioError> {
        let mode = Mode::I | Mode::S | Mode::TRUE_PEAK;
        let inner = EbuR128::new(channels as u32, sample_rate, mode)
            .map_err(|e| AudioError::Ebur128(e.to_string()))?;
        let frames_per_tap = (sample_rate as u64 * SHORT_TERM_SAMPLE_INTERVAL_MS as u64 / 1000)
            .min(u32::MAX as u64) as u32;
        Ok(Self {
            inner,
            channels: channels as u32,
            sample_rate,
            frames_since_tap: 0,
            frames_per_tap: frames_per_tap.max(1),
            short_term_samples: Vec::new(),
            frames_total: 0,
        })
    }

    /// Feed interleaved `f32` samples. The slice length must be a
    /// multiple of `channels`; the caller (the symphonia decode loop)
    /// guarantees this by interleaving inside the audio buffer
    /// converter.
    pub fn feed(&mut self, interleaved: &[f32]) -> Result<(), AudioError> {
        if interleaved.is_empty() {
            return Ok(());
        }
        // ebur128 takes interleaved `&[f32]` directly via add_frames_f32.
        self.inner
            .add_frames_f32(interleaved)
            .map_err(|e| AudioError::Ebur128(e.to_string()))?;
        let frames = (interleaved.len() / self.channels as usize) as u32;
        self.frames_total += frames as u64;
        self.frames_since_tap = self.frames_since_tap.saturating_add(frames);
        // Sample short-term loudness every interval. The first
        // 3 seconds yield NaN/-inf because the sliding window has not
        // filled — drop those out (we only collect once we have at
        // least 3 s of input).
        let three_s_frames = self.sample_rate as u64 * 3;
        while self.frames_since_tap >= self.frames_per_tap {
            self.frames_since_tap -= self.frames_per_tap;
            if self.frames_total >= three_s_frames {
                let s = self
                    .inner
                    .loudness_shortterm()
                    .map_err(|e| AudioError::Ebur128(e.to_string()))?
                    as f32;
                if s.is_finite() {
                    self.short_term_samples.push(s);
                }
            }
        }
        Ok(())
    }

    pub fn frames_total(&self) -> u64 {
        self.frames_total
    }

    /// Drain the accumulator and return the final report. Call once,
    /// after the source file is fully consumed.
    pub fn finish(mut self) -> Result<LoudnessReport, AudioError> {
        let integrated = match self.inner.loudness_global() {
            Ok(v) => v as f32,
            Err(e) => {
                // EBU R128's "too quiet" surface — ebur128 returns
                // a typed error in newer versions, in older versions
                // returns -inf. Map either to NEG_INFINITY for the
                // attribute schema.
                tracing::trace!("ebur128 loudness_global error: {e}");
                f32::NEG_INFINITY
            }
        };
        // For very short files (< 3 s) we never sampled short-term —
        // the percentiles are *undefined*. Surface `NEG_INFINITY` so
        // a `lufs:>x` query (which uses `cmp_op_f32`'s strict `>`)
        // never spuriously matches; the dynamic-range value collapses
        // to `0.0` because there is no spread to measure. The
        // alternative (back-filling with `integrated`) silently turned
        // `dr:>0` queries into "all clips ≥ 3 s long", which was a
        // misleading answer for the user.
        let short_circuit_no_window = self.short_term_samples.is_empty();
        // True peak: max across channels. ebur128 returns linear amp;
        // convert to dBFS with `20 * log10(peak)` (clamping zero to
        // −∞ to match the silence path).
        let mut peak_lin = 0.0_f64;
        for ch in 0..self.channels {
            // The crate API is `prev_true_peak`-style but the actual
            // method is `true_peak`. Errors only surface when TRUE_PEAK
            // wasn't enabled — we always enable it in `new`, so the
            // unwrap is safe.
            let v = self
                .inner
                .true_peak(ch)
                .map_err(|e| AudioError::Ebur128(e.to_string()))?;
            if v > peak_lin {
                peak_lin = v;
            }
        }
        let true_peak_dbfs = if peak_lin > 0.0 {
            (20.0 * peak_lin.log10()) as f32
        } else {
            f32::NEG_INFINITY
        };
        let (p99, p10) = if short_circuit_no_window {
            (f32::NEG_INFINITY, f32::NEG_INFINITY)
        } else {
            percentiles(&mut self.short_term_samples)
        };
        Ok(LoudnessReport {
            integrated,
            short_term_p99: p99,
            short_term_p10: p10,
            true_peak_dbfs,
        })
    }
}

/// Compute the 99th and 10th percentile of a `&mut [f32]`. Sorts the
/// slice in-place — the caller's vec is consumed by `finish` so the
/// mutation is fine. Uses `partial_cmp` since values can be -inf
/// (filtered out at sampling time, but we belt-and-brace).
fn percentiles(xs: &mut [f32]) -> (f32, f32) {
    if xs.is_empty() {
        return (f32::NEG_INFINITY, f32::NEG_INFINITY);
    }
    xs.sort_by(|a, b| a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Equal));
    let n = xs.len();
    // Standard percentile pick: index = round((p/100) * (n-1)).
    let idx = |p: f32| -> usize {
        let i = ((p / 100.0) * (n as f32 - 1.0)).round() as i32;
        i.clamp(0, n as i32 - 1) as usize
    };
    let p99 = xs[idx(99.0)];
    let p10 = xs[idx(10.0)];
    (p99, p10)
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Generate a 1 kHz sine wave at the requested amplitude (0..=1.0)
    /// and length. Returns interleaved stereo `f32`.
    fn sine_wave(amplitude: f32, sample_rate: u32, channels: u16, secs: f32) -> Vec<f32> {
        let frames = (sample_rate as f32 * secs) as usize;
        let mut out = Vec::with_capacity(frames * channels as usize);
        let two_pi_f = 2.0 * std::f32::consts::PI * 1000.0 / sample_rate as f32;
        for i in 0..frames {
            let s = (i as f32 * two_pi_f).sin() * amplitude;
            for _ in 0..channels {
                out.push(s);
            }
        }
        out
    }

    #[test]
    fn percentile_helper_basics() {
        let mut xs: Vec<f32> = (0..=100).map(|x| x as f32).collect();
        let (p99, p10) = percentiles(&mut xs);
        assert!((p99 - 99.0).abs() < 1e-6);
        assert!((p10 - 10.0).abs() < 1e-6);
    }

    #[test]
    fn percentile_helper_empty_returns_neg_inf() {
        let mut xs: Vec<f32> = vec![];
        let (p99, p10) = percentiles(&mut xs);
        assert_eq!(p99, f32::NEG_INFINITY);
        assert_eq!(p10, f32::NEG_INFINITY);
    }

    #[test]
    fn one_khz_sine_at_minus_23_dbfs_integrates_near_minus_23_lufs() {
        // 1 kHz sine at −23 dBFS reads ≈ −23 LUFS post-K-weighting at
        // 1 kHz (K-weighting is approximately flat in the 1 kHz band).
        // A 5-second clip is more than the 3-second short-term window
        // so the short-term P99/P10 also fall near −23.
        let sr = 48_000;
        let amp = 10f32.powf(-23.0 / 20.0);
        let pcm = sine_wave(amp, sr, 2, 5.0);
        let mut acc = LoudnessAccumulator::new(2, sr).unwrap();
        // Feed in 1024-frame chunks to exercise the short-term tap.
        for chunk in pcm.chunks(2048) {
            acc.feed(chunk).unwrap();
        }
        let report = acc.finish().unwrap();
        assert!(
            (report.integrated - -23.0).abs() < 1.0,
            "integrated: {} (expected ≈ −23)",
            report.integrated
        );
        assert!(
            report.true_peak_dbfs > -25.0 && report.true_peak_dbfs < -22.0,
            "true peak: {} (expected ≈ −23 dBFS)",
            report.true_peak_dbfs
        );
    }

    #[test]
    fn pure_silence_yields_neg_infinity_integrated() {
        let sr = 48_000;
        let pcm = vec![0.0_f32; (sr * 2 * 5) as usize]; // 5 s stereo silence
        let mut acc = LoudnessAccumulator::new(2, sr).unwrap();
        for chunk in pcm.chunks(2048) {
            acc.feed(chunk).unwrap();
        }
        let report = acc.finish().unwrap();
        assert_eq!(
            report.integrated,
            f32::NEG_INFINITY,
            "pure silence should integrate to −∞"
        );
        assert_eq!(report.true_peak_dbfs, f32::NEG_INFINITY);
    }

    #[test]
    fn under_three_second_clip_short_term_is_neg_inf() {
        // <3 s clip — short-term sampling never fires; the report
        // surfaces `NEG_INFINITY` for both percentiles so callers see
        // the "no window" condition explicitly. Dynamic range
        // computed from these collapses to `0`. The integrated value
        // is still reported normally.
        let sr = 48_000;
        let amp = 10f32.powf(-20.0 / 20.0);
        let pcm = sine_wave(amp, sr, 2, 1.0);
        let mut acc = LoudnessAccumulator::new(2, sr).unwrap();
        acc.feed(&pcm).unwrap();
        let report = acc.finish().unwrap();
        assert_eq!(report.short_term_p99, f32::NEG_INFINITY);
        assert_eq!(report.short_term_p10, f32::NEG_INFINITY);
        // Integrated value still reported.
        assert!(report.integrated.is_finite());
        assert!((report.integrated - -20.0).abs() < 1.5);
    }
}
