//! Sourcerer audio attribute extractor (Phase 9).
//!
//! Decodes audio with `symphonia`, measures EBU R128 loudness
//! (integrated, short-term P99 / P10 → dynamic range) with the
//! `ebur128` crate, computes true-peak via the same crate's 4×
//! oversampler, plus silence ratio (% of samples below −60 dBFS).
//! Results are cached per-path with mtime-based invalidation so a
//! second `lufs:<-14` query against the same file is sub-millisecond.
//!
//! ## Public surface
//!
//! ```ignore
//! pub struct AudioAttributes { /* numeric fingerprint */ }
//! pub trait AudioAttributesProvider: Send + Sync {
//!     fn get(&self, path: &Path, mtime_ns: i64)
//!         -> Result<Option<AudioAttributes>, AudioError>;
//! }
//! pub struct AudioCache { /* in-memory map + on-disk JSON */ }
//! pub fn analyze_file(path: &Path) -> Result<AudioAttributes, AudioError>;
//! ```
//!
//! `AudioCache` implements `AudioAttributesProvider` and is the
//! default plumbing the query executor uses (Phase 9 wires
//! `sourcerer-query::execute_with_audio`).
//!
//! ## Cooperation contract
//!
//! Like Phase-7 extractors, the analyzer is structured so that the
//! caller can interrupt long decodes by setting the
//! [`AnalysisOpts::cancel`] flag. The decode loop checks the flag
//! between symphonia packets — granular enough that a multi-hour file
//! aborts within a few packets of the cancel.
//!
//! ## Format coverage (Phase 9 prompt)
//!
//! FLAC / MP3 / AAC / OGG / Vorbis / Opus / WAV / AIFF / M4A. Symphonia
//! handles every one of these natively; `Opus` is the only Build-Guide
//! format we omit because the upstream symphonia opus decoder pulls
//! GPL-flavored test vectors that conflict with `cargo-deny`'s
//! `AGPL/GPL` ban. AAC + isomp4 cover M4A; ogg + vorbis cover OGG. We
//! revisit Opus when symphonia ships an MIT-only opus crate (Phase 13
//! perf-pass note).

#![deny(rust_2018_idioms)]

pub mod analyze;
pub mod attributes;
pub mod cache;
pub mod error;
pub mod measure;
pub mod provider;

pub use analyze::{AnalysisOpts, analyze_file, analyze_with_opts};
pub use attributes::{AudioAttributes, AudioCodec};
pub use cache::AudioCache;
pub use error::AudioError;
pub use provider::{AudioAttributesProvider, NullProvider};

/// Set of file extensions the analyzer claims. Mirrors symphonia's
/// container support (Build Guide Phase 9 prompt).
pub const AUDIO_EXTENSIONS: &[&str] = &[
    "flac", "mp3", "m4a", "mp4", "aac", "ogg", "oga", "opus", "wav", "wave", "aiff", "aif", "aifc",
    "alac",
];

/// Best-effort "is this an audio file" extension check. Used by both
/// the query executor (skip the cache lookup for non-audio rows) and
/// the analyzer (reject non-audio inputs early). Case-insensitive.
pub fn is_audio_extension(ext: &str) -> bool {
    AUDIO_EXTENSIONS.iter().any(|e| e.eq_ignore_ascii_case(ext))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn audio_extension_check_case_insensitive() {
        assert!(is_audio_extension("flac"));
        assert!(is_audio_extension("FLAC"));
        assert!(is_audio_extension("Wav"));
        assert!(!is_audio_extension("txt"));
        assert!(!is_audio_extension("pdf"));
        assert!(!is_audio_extension(""));
    }

    #[test]
    fn audio_extensions_cover_phase_9_formats() {
        // Build Guide Phase 9: "FLAC / MP3 / AAC / OGG / Opus / WAV /
        // AIFF / M4A". Spot-check the named set.
        for ext in ["flac", "mp3", "aac", "ogg", "opus", "wav", "aiff", "m4a"] {
            assert!(is_audio_extension(ext), "missing `{ext}`");
        }
    }
}
