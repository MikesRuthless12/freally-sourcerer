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
    #[error("empty query")]
    Empty,
}

#[derive(Debug, thiserror::Error)]
pub enum QueryError {
    #[error(transparent)]
    Parse(#[from] ParseError),
    #[error("index error: {0}")]
    Index(#[from] sourcerer_index::IndexError),
    #[error("query references modifier `{0}` which is not implemented in Phase 5")]
    UnsupportedModifier(String),
    #[error("execution cancelled")]
    Cancelled,
}
