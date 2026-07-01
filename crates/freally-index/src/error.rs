//! `IndexError` — surface the rest of the workspace consumes.

use std::path::PathBuf;

#[derive(Debug, thiserror::Error)]
pub enum IndexError {
    #[error("index root path is invalid: {0}")]
    InvalidRoot(PathBuf),
    #[error("io error at `{path}`: {source}")]
    Io {
        path: PathBuf,
        #[source]
        source: std::io::Error,
    },
    #[error("tantivy error: {0}")]
    Tantivy(String),
    #[error("sqlite error: {0}")]
    Sqlite(#[from] rusqlite::Error),
    #[error("name-index error: {0}")]
    NameIndex(String),
    #[error("manifest error: {0}")]
    Manifest(String),
    #[error("indexer queue full (back-pressure): capacity = {capacity}, pending = {pending}")]
    QueueFull { capacity: usize, pending: usize },
    #[error("integrity check failed: {0}")]
    Integrity(String),
}

impl IndexError {
    pub(crate) fn io(path: impl Into<PathBuf>, source: std::io::Error) -> Self {
        Self::Io {
            path: path.into(),
            source,
        }
    }
}

impl From<tantivy::TantivyError> for IndexError {
    fn from(err: tantivy::TantivyError) -> Self {
        IndexError::Tantivy(err.to_string())
    }
}

impl From<tantivy::query::QueryParserError> for IndexError {
    fn from(err: tantivy::query::QueryParserError) -> Self {
        IndexError::Tantivy(err.to_string())
    }
}

impl From<tantivy::directory::error::OpenDirectoryError> for IndexError {
    fn from(err: tantivy::directory::error::OpenDirectoryError) -> Self {
        IndexError::Tantivy(err.to_string())
    }
}
