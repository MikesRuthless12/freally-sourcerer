//! Sourcerer JSON-RPC transport.
//!
//! Length-prefixed (u32 BE) JSON-RPC 2.0 over a local-only Unix domain
//! socket (macOS / Linux) or named pipe (Windows). The transport is the
//! sole channel between `sourcerer-indexd` (the server) and the
//! Tauri UI plus the `sourcerer` CLI (clients). Auth: file-mode 0600 +
//! peer-uid check on Unix; pipe DACL restricted to the current user
//! SID on Windows. Any peer that fails the auth check is dropped at
//! accept-time before reading a single frame.
//!
//! ## Surface
//!
//! - `dto`: serde-shaped DTOs that mirror `apps/sourcerer-ui/src/lib/
//!   ipc/types.ts` byte-for-byte. Same names. Same lower-case enum
//!   variants. Phase 12 swaps the Phase-11 mock implementations behind
//!   these types without changing the UI.
//! - `frame`: length-prefixed framing. Every message is a `u32 BE`
//!   length followed by that many bytes of UTF-8 JSON. Maximum payload
//!   `MAX_FRAME_BYTES` so a hostile peer can't OOM the server with a
//!   single 4-GiB frame.
//! - `jsonrpc`: JSON-RPC 2.0 request / response / error envelopes.
//! - `service`: the `Service` trait — `sourcerer-indexd` implements it
//!   to dispatch incoming method calls; clients never see this.
//! - `server`: spawns the listener + per-connection task; emits
//!   server-side notifications (`query:batch`, `query:done`, etc.) as
//!   `Notification` frames the client routes to Tauri events.
//! - `client`: connect + request + notification subscription.
//! - `transport`: `cfg`-gated UDS / named-pipe primitives.

#![forbid(unsafe_op_in_unsafe_fn)]

pub mod client;
pub mod dto;
pub mod error;
pub mod frame;
pub mod jsonrpc;
pub mod path;
pub mod server;
pub mod service;
pub mod transport;

pub use client::{Client, ClientHandle, NotificationStream};
pub use dto::*;
pub use error::{RpcError, RpcResult};
pub use frame::{FrameError, FrameReader, FrameWriter, MAX_FRAME_BYTES};
pub use jsonrpc::{
    ErrorObject, JSONRPC_VERSION, Notification, Request, RequestId, Response, ResponseEnvelope,
};
pub use path::{SocketPath, default_socket_path};
#[cfg(windows)]
pub use path::{service_pipe_name, service_sddl};
pub use server::{Server, ServerConfig};
pub use service::{NotificationSink, Service};
