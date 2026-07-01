//! Errors surfaced by the query crate.

#[derive(Debug, thiserror::Error)]
pub enum ParseError {
    #[error("unexpected end of input at byte {pos}")]
    UnexpectedEof { pos: usize },
    #[error("unexpected token `{token}` at byte {pos}")]
    UnexpectedToken { pos: usize, token: String },
    #[error("unbalanced parentheses at byte {pos}")]
    UnbalancedParens { pos: usize },
    #[error("invalid regex `{regex}`: {source}")]
    InvalidRegex {
        regex: String,
        #[source]
        source: regex::Error,
    },
    #[error("invalid wildcard pattern `{pattern}`: {message}")]
    InvalidWildcard { pattern: String, message: String },
    #[error("unknown modifier `{name}` at byte {pos}")]
    UnknownModifier { pos: usize, name: String },
    #[error("invalid modifier value for `{name}`: {value} ({reason})")]
    InvalidModifierValue {
        name: String,
        value: String,
        reason: String,
    },
    /// Phase 10: surfaced when `ParseOpts::strict_everything` is on
    /// and the parser encounters a Freally-only modifier or lens
    /// prefix. The `token` carries the offending source slice so the
    /// IPC layer can highlight it.
    #[error("`{token}` rejected by --strict-everything at byte {pos}: {reason}")]
    StrictEverythingViolation {
        pos: usize,
        token: String,
        reason: String,
    },
    #[error("empty query")]
    Empty,
}

#[derive(Debug, thiserror::Error)]
pub enum QueryError {
    #[error(transparent)]
    Parse(#[from] ParseError),
    #[error("index error: {0}")]
    Index(#[from] freally_index::IndexError),
    #[error("query references modifier `{0}` which has no executor in this build")]
    UnsupportedModifier(String),
    #[error(
        "query uses `similar:` but no similarity index was supplied — call execute_with(idx, Some(&sim), …)"
    )]
    SimilarityIndexUnavailable,
    #[error(
        "Phase 6 only supports `similar:` at the root or as a top-level AND child; \
         buried in NOT / OR / nested AND is rejected"
    )]
    UnsupportedSimilarPosition,
    #[error(
        "query uses an audio modifier (lufs / codec / length / rate / silence / dr) \
         but no audio provider was supplied — call execute_with_audio(idx, Some(&audio), …)"
    )]
    AudioProviderUnavailable,
    #[error("audio extractor: {0}")]
    Audio(#[from] freally_audio::AudioError),
    #[error("execution cancelled")]
    Cancelled,
}
