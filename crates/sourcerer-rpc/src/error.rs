use std::io;

use thiserror::Error;

use crate::frame::FrameError;

/// Top-level error type surfaced by the RPC client and server.
#[derive(Debug, Error)]
pub enum RpcError {
    #[error("io: {0}")]
    Io(#[from] io::Error),

    #[error("frame: {0}")]
    Frame(#[from] FrameError),

    #[error("json: {0}")]
    Json(#[from] serde_json::Error),

    #[error("peer rejected: {0}")]
    PeerRejected(String),

    #[error("transport closed")]
    TransportClosed,

    #[error("rpc method `{method}` not found")]
    MethodNotFound { method: String },

    #[error("rpc error: {code}: {message}")]
    Remote {
        code: i32,
        message: String,
        data: Option<serde_json::Value>,
    },

    #[error("invalid response (no matching request id)")]
    InvalidResponse,

    #[error("request canceled")]
    Canceled,

    #[error("{0}")]
    Other(String),
}

pub type RpcResult<T> = Result<T, RpcError>;

/// Standard JSON-RPC error codes plus Sourcerer-specific extensions.
pub mod codes {
    pub const PARSE_ERROR: i32 = -32700;
    pub const INVALID_REQUEST: i32 = -32600;
    pub const METHOD_NOT_FOUND: i32 = -32601;
    pub const INVALID_PARAMS: i32 = -32602;
    pub const INTERNAL_ERROR: i32 = -32603;

    /// Sourcerer-specific: the daemon refused the call because the index
    /// is currently corrupt and the integrity policy is `Strict`.
    pub const INDEX_INTEGRITY: i32 = -32000;

    /// Sourcerer-specific: the requested volume / folder / extractor is
    /// not registered.
    pub const NOT_FOUND: i32 = -32001;

    /// Sourcerer-specific: the daemon is shutting down.
    pub const SHUTTING_DOWN: i32 = -32002;
}
