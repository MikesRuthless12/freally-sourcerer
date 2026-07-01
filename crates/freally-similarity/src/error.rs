//! `SimilarityError` — surface the rest of the workspace consumes.

use std::path::PathBuf;

#[derive(Debug, thiserror::Error)]
pub enum SimilarityError {
    #[error("io error at `{path}`: {source}")]
    Io {
        path: PathBuf,
        #[source]
        source: std::io::Error,
    },
    #[error("similarity index format error: {0}")]
    Format(String),
    #[error("similarity index is closed")]
    Closed,
}

impl SimilarityError {
    pub(crate) fn io(path: impl Into<PathBuf>, source: std::io::Error) -> Self {
        Self::Io {
            path: path.into(),
            source,
        }
    }
}
