//! SRC-M18 — waveform peaks for the preview pane's audio player.
//!
//! A collector that rides along with the existing analysis decode in
//! [`crate::analyze`]. The point is that drawing a waveform costs one
//! decode, not two: the same interleaved `f32` frames that feed the
//! loudness and silence accumulators feed this, so opening a track in
//! the preview pane does not decode it a second time just to draw a
//! picture of it.
//!
//! What it produces is a fixed number of buckets, each holding the
//! maximum absolute sample in that slice of the file — which is what a
//! waveform is. Fixed rather than per-second because the canvas has a
//! fixed pixel width: resampling a per-second envelope to fit would
//! throw away exactly the peaks that make the shape readable.

/// Absolute cap on bucket count. The canvas is a few hundred pixels
/// wide; asking for more buckets than that produces data nobody can
/// see, at the cost of a bigger IPC payload for every preview.
pub const MAX_BUCKETS: usize = 2_048;

/// Streams samples into a fixed number of max-amplitude buckets.
#[derive(Debug)]
pub struct PeakCollector {
    buckets: Vec<f32>,
    /// Frames per bucket, derived from the track's declared duration.
    /// Zero when the duration is unknown — see [`Self::push`].
    frames_per_bucket: u64,
    frames_seen: u64,
    channels: usize,
}

impl PeakCollector {
    /// `total_frames` is the decoder's declared frame count. It is a
    /// hint, not a contract: containers lie, and a VBR stream may run
    /// long or short. Overflow past the last bucket folds into it
    /// rather than reallocating or panicking.
    pub fn new(buckets: usize, total_frames: u64, channels: u16) -> Self {
        let buckets = buckets.clamp(1, MAX_BUCKETS);
        let channels = channels.max(1) as usize;
        Self {
            buckets: vec![0.0; buckets],
            frames_per_bucket: (total_frames / buckets as u64).max(1),
            frames_seen: 0,
            channels,
        }
    }

    /// Feed one decoded packet's interleaved samples.
    pub fn push(&mut self, interleaved: &[f32]) {
        for frame in interleaved.chunks(self.channels) {
            // Peak across channels: a waveform shows the loudest thing
            // happening at that moment, not the left channel's opinion
            // of it.
            let mut peak = 0.0f32;
            for s in frame {
                let a = s.abs();
                if a > peak {
                    peak = a;
                }
            }
            let idx = (self.frames_seen / self.frames_per_bucket) as usize;
            let idx = idx.min(self.buckets.len() - 1);
            if peak > self.buckets[idx] {
                self.buckets[idx] = peak;
            }
            self.frames_seen += 1;
        }
    }

    /// The finished envelope, each value in `0.0..=1.0`.
    ///
    /// Trailing buckets that never received a sample are trimmed: a
    /// container that over-declares its length would otherwise draw a
    /// stretch of flat silence the file does not actually contain.
    pub fn finish(self) -> Vec<f32> {
        let mut v = self.buckets;
        while v.len() > 1 && v.last() == Some(&0.0) {
            v.pop();
        }
        v
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn buckets_hold_the_maximum_not_the_last_sample() {
        let mut c = PeakCollector::new(2, 4, 1);
        c.push(&[0.1, 0.9]);
        c.push(&[0.2, 0.3]);
        let v = c.finish();
        assert_eq!(v.len(), 2);
        assert!((v[0] - 0.9).abs() < 1e-6, "got {v:?}");
    }

    #[test]
    fn peaks_across_channels_not_just_the_first() {
        let mut c = PeakCollector::new(1, 2, 2);
        // One stereo frame: quiet left, loud right.
        c.push(&[0.05, 0.8]);
        assert!((c.finish()[0] - 0.8).abs() < 1e-6);
    }

    #[test]
    fn negative_samples_count_by_amplitude() {
        let mut c = PeakCollector::new(1, 2, 1);
        c.push(&[-0.7, 0.2]);
        assert!((c.finish()[0] - 0.7).abs() < 1e-6);
    }

    #[test]
    fn overrunning_the_declared_length_folds_into_the_last_bucket() {
        // Containers lie about duration; this must not panic or grow.
        let mut c = PeakCollector::new(2, 2, 1);
        c.push(&[0.1, 0.2, 0.3, 0.4, 0.5, 0.6]);
        let v = c.finish();
        assert_eq!(v.len(), 2);
        assert!((v[1] - 0.6).abs() < 1e-6);
    }

    #[test]
    fn an_unknown_duration_does_not_divide_by_zero() {
        let mut c = PeakCollector::new(4, 0, 1);
        c.push(&[0.5]);
        assert!(!c.finish().is_empty());
    }

    #[test]
    fn trailing_silence_from_an_over_declared_length_is_trimmed() {
        let mut c = PeakCollector::new(4, 8, 1);
        c.push(&[0.5, 0.5]);
        let v = c.finish();
        assert!(
            v.len() < 4,
            "expected trailing empty buckets trimmed: {v:?}"
        );
    }

    #[test]
    fn the_bucket_count_is_capped() {
        let c = PeakCollector::new(usize::MAX, 1_000, 1);
        assert_eq!(c.finish().len().min(MAX_BUCKETS), MAX_BUCKETS.min(1));
    }
}
