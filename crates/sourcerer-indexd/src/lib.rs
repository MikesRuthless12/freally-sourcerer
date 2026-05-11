//! Sourcerer indexer daemon — library entrypoint.
//!
//! `sourcerer-indexd` runs as a long-lived daemon. The process is
//! invoked four ways:
//!
//! 1. `sourcerer-indexd run` — foreground; for dev / smoke tests.
//! 2. `sourcerer-indexd install` / `uninstall` — register / deregister
//!    with the per-OS service manager (SCM / launchd / systemd).
//! 3. `sourcerer-indexd service` — the entry point that the OS service
//!    manager invokes; runs the daemon under the platform's
//!    service-control conventions.
//! 4. Linked into `sourcerer-ui` as a child process started at app boot;
//!    the UI is the primary RPC client.
//!
//! The library half (this file + `service`, `state`, `volumes`,
//! `bookmarks`) holds the cross-platform daemon body. The binary
//! (`main.rs`) is a thin shim that parses CLI flags and dispatches.

pub mod bookmarks;
pub mod history;
pub mod scanner;
pub mod service;
pub mod settings;
pub mod state;
pub mod volumes;

pub use service::IndexdService;
pub use state::{DaemonOptions, DaemonState};

use std::sync::Arc;

use anyhow::Result;
use sourcerer_rpc::{Server, ServerConfig, SocketPath, default_socket_path};
use tokio::task::JoinHandle;

/// Spawn the RPC server with the standard per-OS socket path.
///
/// Returns a `JoinHandle` that resolves when the server's accept loop
/// terminates (e.g. on shutdown). The caller can drop the returned
/// `Arc<DaemonState>` to release the indexer; the running tokio tasks
/// hold their own clones.
pub async fn spawn_default(state: Arc<DaemonState>) -> Result<JoinHandle<()>> {
    spawn_at(state, default_socket_path()).await
}

/// Spawn the RPC server at a caller-chosen socket / pipe. Used by smoke
/// tests to bind to a temp path.
pub async fn spawn_at(state: Arc<DaemonState>, socket: SocketPath) -> Result<JoinHandle<()>> {
    let service = Arc::new(IndexdService::new(state));
    let server = Server::new(ServerConfig::new(socket));
    Ok(server.spawn(service))
}
