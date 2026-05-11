//! Tauri command surface for the Phase 12 UI.
//!
//! Phase 12 swapped Phase 11's mock IPC layer for a real
//! `sourcerer-indexd` connection over `sourcerer-rpc`. Each command
//! body runs inside the daemon's tokio runtime — see `daemon.rs` —
//! and dispatches via `RpcClient::call(method, params)`. Streaming
//! results (`query.run`) flow through Tauri events emitted by the
//! daemon's notification stream.
//!
//! `query_parse` remains the single in-process command: live
//! tokenization in the search bar must match the production parser
//! exactly, and round-tripping through the daemon would add a
//! sub-millisecond keystroke-rate cost we don't need.

pub mod bookmarks;
pub mod custom_extractors;
pub mod excludes;
pub mod extractors;
pub mod files;
pub mod folders;
pub mod history;
pub mod icons;
pub mod index_state;
pub mod known_paths;
pub mod network;
pub mod query;
pub mod settings;
pub mod volumes;
