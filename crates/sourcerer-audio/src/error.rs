//! Audio-extractor error surface.

use std::path::PathBuf;

#[derive(Debug, thiserror::Error)]
pub enum AudioError {
    #[error("audio I/O at {path}: {source}")]
    Io {
        path: PathBuf,
        #[source]
        source: std::io::Error,
    },
    #[error("not an audio file (extension `{ext}` not recognised)")]
    NotAudio { ext: String },
    #[error("symphonia could not probe `{path}`: {reason}")]
    Probe { path: PathBuf, reason: String },
    #[error("symphonia decode failed at packet {packet}: {reason}")]
    Decode { packet: usize, reason: String },
    #[error("ebur128 setup error: {0}")]
    Ebur128(String),
    #[error("audio file has zero frames — cannot measure loudness")]
    Empty,
    #[error("unsupported configuration: {0}")]
    Unsupported(String),
    #[error("analysis cancelled")]
    Cancelled,
    #[error("audio cache JSON: {0}")]
    Json(#[from] serde_json::Error),
    #[error("audio cache: path is not valid UTF-8 — `{path}`")]
    NonUtf8Path { path: String },
}

impl AudioError {
    pub fn io(path: impl Into<PathBuf>, source: std::io::Error) -> Self {
        Self::Io {
            path: path.into(),
            source,
        }
    }
}
