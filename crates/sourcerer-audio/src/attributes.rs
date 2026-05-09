//! `AudioAttributes` — the structured fingerprint of an audio file.
//!
//! Stored in the audio cache, queried by the audio modifiers
//! (`lufs:`, `codec:`, `length:`, `rate:`, `silence:`, `dr:`).

use std::time::Duration;

use serde::{Deserialize, Serialize};

/// Custom serde for `f32` fields that can carry `±Infinity` / `NaN`.
/// `serde_json` defaults non-finite floats to `null` and then errors
/// on read — silently corrupting our cache. We round-trip through a
/// tiny tagged JSON shape: a finite value as a plain `f32`, the
/// special cases as the strings `"-inf"` / `"+inf"` / `"nan"`. The
/// cache file remains human-readable; the round-trip stays lossless.
mod lufs_serde {
    use serde::de::{Deserializer, Error};
    use serde::ser::Serializer;
    use serde::{Deserialize, Serialize};

    #[derive(Serialize, Deserialize)]
    #[serde(untagged)]
    enum Wire {
        Number(f32),
        Sentinel(String),
    }

    pub fn serialize<S: Serializer>(v: &f32, ser: S) -> Result<S::Ok, S::Error> {
        if v.is_finite() {
            Wire::Number(*v).serialize(ser)
        } else if v.is_nan() {
            Wire::Sentinel("nan".into()).serialize(ser)
        } else if v.is_sign_positive() {
            Wire::Sentinel("+inf".into()).serialize(ser)
        } else {
            Wire::Sentinel("-inf".into()).serialize(ser)
        }
    }

    pub fn deserialize<'de, D: Deserializer<'de>>(de: D) -> Result<f32, D::Error> {
        let w = Wire::deserialize(de)?;
        Ok(match w {
            Wire::Number(n) => n,
            Wire::Sentinel(s) => match s.as_str() {
                "-inf" => f32::NEG_INFINITY,
                "+inf" | "inf" => f32::INFINITY,
                "nan" => f32::NAN,
                other => return Err(D::Error::custom(format!("unknown LUFS sentinel `{other}`"))),
            },
        })
    }
}

/// Symphonia codec identifier — text rather than enum so we can round-
/// trip an unknown codec name through the cache without churning the
/// JSON schema. Sourced from `symphonia_core::codecs::CODEC_TYPE_*`'s
/// short identifier.
pub type AudioCodec = String;

/// One audio file's measured attributes.
///
/// `lufs_short_term_p99` and `lufs_short_term_p10` are the 99th /
/// 10th percentile of the short-term (3-second sliding window)
/// loudness samples taken every 100 ms. `dynamic_range_lu` is the
/// difference (P99 − P10), which matches the Phase 9 prompt's
/// definition of "dynamic range" and approximates EBU R128 LRA for
/// most material.
///
/// All loudness values are in LUFS (LU when relative); `f32::NEG_INFINITY`
/// signals "ebur128 reported `LoudnessTooLow`" — pure-silence inputs.
/// `peak_dbfs` is in dBFS (true peak via 4× oversampling); a peak of
/// 0.0 dBFS is full-scale, more-negative values are quieter.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AudioAttributes {
    /// Symphonia's short codec identifier (e.g. `"flac"`, `"mp3"`,
    /// `"aac"`, `"vorbis"`, `"pcm_s16le"`). Lower-cased.
    pub codec: AudioCodec,
    /// Sampling rate in Hz.
    pub sample_rate: u32,
    /// Channel count.
    pub channels: u16,
    /// Bits per sample for PCM containers; for compressed codecs this
    /// is the *effective* bit depth symphonia reports (often 16 / 24).
    /// `None` when the codec doesn't expose one.
    pub bit_depth: Option<u16>,
    /// Duration in nanoseconds — store as integer so the cache JSON
    /// round-trips exactly without float jitter. Use
    /// [`AudioAttributes::duration`] for an `std::time::Duration`.
    pub duration_ns: u64,
    /// Integrated programme loudness in LUFS (EBU R128 §3.2).
    #[serde(with = "lufs_serde")]
    pub lufs_integrated: f32,
    /// 99th-percentile of short-term (3-second sliding window) LUFS,
    /// sampled every 100 ms.
    #[serde(with = "lufs_serde")]
    pub lufs_short_term_p99: f32,
    /// 10th-percentile of short-term LUFS, sampled every 100 ms. Used
    /// as the "quiet floor" for `dynamic_range_lu`.
    #[serde(with = "lufs_serde")]
    pub lufs_short_term_p10: f32,
    /// True peak in dBFS via the ebur128 4× oversampler (max across
    /// channels).
    #[serde(with = "lufs_serde")]
    pub peak_dbfs: f32,
    /// Fraction of decoded samples below `−60 dBFS` (≈ silence).
    /// `[0.0, 1.0]`.
    pub silence_ratio: f32,
    /// Dynamic range in LU = `lufs_short_term_p99 − lufs_short_term_p10`.
    pub dynamic_range_lu: f32,
}

impl AudioAttributes {
    pub fn duration(&self) -> Duration {
        Duration::from_nanos(self.duration_ns)
    }

    /// Length in seconds, returned as `f32` for the query DSL's
    /// comparator path (`length:>180` matches "longer than 3
    /// minutes"). The raw nanosecond field is the source of truth;
    /// this helper just casts.
    pub fn length_seconds(&self) -> f32 {
        (self.duration_ns as f64 / 1_000_000_000.0) as f32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn fixture() -> AudioAttributes {
        AudioAttributes {
            codec: "flac".into(),
            sample_rate: 44_100,
            channels: 2,
            bit_depth: Some(16),
            duration_ns: 134_000_000_000, // 2:14
            lufs_integrated: -16.0,
            lufs_short_term_p99: -10.5,
            lufs_short_term_p10: -22.5,
            peak_dbfs: -1.0,
            silence_ratio: 0.0,
            dynamic_range_lu: 12.0,
        }
    }

    #[test]
    fn duration_round_trip() {
        let a = fixture();
        assert_eq!(a.duration(), Duration::from_nanos(134_000_000_000));
        assert!((a.length_seconds() - 134.0).abs() < 1e-3);
    }

    #[test]
    fn json_round_trip() {
        let a = fixture();
        let j = serde_json::to_string(&a).unwrap();
        let b: AudioAttributes = serde_json::from_str(&j).unwrap();
        assert_eq!(a, b);
    }

    #[test]
    fn negative_infinity_round_trip() {
        // Pure-silence files report integrated LUFS / peak as −∞.
        // The custom `lufs_serde` helper must round-trip non-finite
        // values losslessly through JSON.
        let mut a = fixture();
        a.lufs_integrated = f32::NEG_INFINITY;
        a.peak_dbfs = f32::NEG_INFINITY;
        let j = serde_json::to_string(&a).expect("serialize ±∞ LUFS");
        let b: AudioAttributes = serde_json::from_str(&j).expect("deserialize ±∞ LUFS");
        assert_eq!(b.lufs_integrated, f32::NEG_INFINITY);
        assert_eq!(b.peak_dbfs, f32::NEG_INFINITY);
    }

    #[test]
    fn positive_infinity_round_trip() {
        let mut a = fixture();
        a.lufs_integrated = f32::INFINITY;
        let j = serde_json::to_string(&a).unwrap();
        let b: AudioAttributes = serde_json::from_str(&j).unwrap();
        assert_eq!(b.lufs_integrated, f32::INFINITY);
    }

    #[test]
    fn nan_round_trip() {
        let mut a = fixture();
        a.lufs_integrated = f32::NAN;
        let j = serde_json::to_string(&a).unwrap();
        let b: AudioAttributes = serde_json::from_str(&j).unwrap();
        assert!(b.lufs_integrated.is_nan());
    }

    #[test]
    fn unknown_sentinel_rejects() {
        // A handcrafted cache file that names an unknown sentinel
        // surfaces a typed JSON error rather than silently producing
        // a wrong value.
        let bad = r#"{
            "codec":"flac","sample_rate":44100,"channels":2,"bit_depth":16,
            "duration_ns":134000000000,
            "lufs_integrated":"haunted",
            "lufs_short_term_p99":-10.5,"lufs_short_term_p10":-22.5,
            "peak_dbfs":-1.0,"silence_ratio":0.0,"dynamic_range_lu":12.0
        }"#;
        let res: Result<AudioAttributes, _> = serde_json::from_str(bad);
        assert!(res.is_err());
    }
}
