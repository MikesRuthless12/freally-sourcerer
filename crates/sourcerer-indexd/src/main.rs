//! Sourcerer indexer daemon — binary entry point.
//!
//! This binary is a thin shim over the [`sourcerer_indexd`] library. The
//! library exposes the daemon body so the Tauri app can also embed the
//! daemon when developing in-process.
//!
//! Phase 12 wiring:
//!
//! - `run` opens the index at the standard per-OS path and starts the
//!   RPC server at `default_socket_path()`. The Tauri app launches this
//!   as a sidecar process at boot.
//! - `install` / `uninstall` register / deregister the OS-native
//!   service entry (Windows SCM / launchd / systemd-user). Unchanged
//!   from Phase 1–3.
//! - `service` is the entry point invoked by the OS service manager.
//!   Same body as `run`, but wrapped in the platform's service
//!   reporting conventions.

use anyhow::Result;
use clap::{Parser, Subcommand};
use sourcerer_indexd::{DaemonOptions, DaemonState};
use std::sync::Arc;

#[cfg(windows)]
mod windows_service;
#[cfg(target_os = "macos")]
mod launchd;
#[cfg(target_os = "linux")]
mod systemd;

#[derive(Debug, Parser)]
#[command(name = "sourcerer-indexd", about = "Sourcerer indexer daemon", version)]
struct Cli {
    #[command(subcommand)]
    command: Option<Command>,
}

#[derive(Debug, Subcommand)]
enum Command {
    /// Run the indexer in the foreground (manual / dev / smoke-test
    /// mode). Opens the index at the per-OS standard path and starts
    /// the RPC server at `default_socket_path()`. Logs to stderr.
    Run {
        /// Optional override for the index root.
        #[arg(long)]
        index_root: Option<String>,
        /// Optional override for the socket / pipe path.
        #[arg(long)]
        socket: Option<String>,
    },
    /// Install the indexer as an OS-managed background service.
    Install {
        #[arg(long)]
        binary: Option<String>,
    },
    /// Uninstall the indexer service. Reverses `install`.
    Uninstall,
    /// Internal: the OS service manager invokes this.
    Service,
}

fn main() -> Result<()> {
    init_tracing();
    let cli = Cli::parse();
    let cmd = cli.command.unwrap_or(Command::Run {
        index_root: None,
        socket: None,
    });
    match cmd {
        Command::Run { index_root, socket } => run_foreground(index_root, socket),
        #[cfg(windows)]
        Command::Install { binary } => windows_service::install(binary.as_deref()),
        #[cfg(target_os = "macos")]
        Command::Install { binary } => launchd::install(binary.as_deref()),
        #[cfg(target_os = "linux")]
        Command::Install { binary } => systemd::install(binary.as_deref()),
        #[cfg(all(not(windows), not(target_os = "macos"), not(target_os = "linux")))]
        Command::Install { .. } => {
            anyhow::bail!("`install` is only supported on Windows, macOS, and Linux.")
        }
        #[cfg(windows)]
        Command::Uninstall => windows_service::uninstall(),
        #[cfg(target_os = "macos")]
        Command::Uninstall => launchd::uninstall(),
        #[cfg(target_os = "linux")]
        Command::Uninstall => systemd::uninstall(),
        #[cfg(all(not(windows), not(target_os = "macos"), not(target_os = "linux")))]
        Command::Uninstall => {
            anyhow::bail!("`uninstall` is only supported on Windows, macOS, and Linux.")
        }
        #[cfg(windows)]
        Command::Service => windows_service::run_as_service(),
        #[cfg(target_os = "macos")]
        Command::Service => launchd::run_as_service(),
        #[cfg(target_os = "linux")]
        Command::Service => systemd::run_as_service(),
        #[cfg(all(not(windows), not(target_os = "macos"), not(target_os = "linux")))]
        Command::Service => {
            anyhow::bail!(
                "`service` is invoked by the OS service manager and is only supported \
                 on Windows, macOS, and Linux."
            )
        }
    }
}

fn init_tracing() {
    let _ = tracing_subscriber::fmt()
        .with_env_filter(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| tracing_subscriber::EnvFilter::new("info")),
        )
        .with_writer(std::io::stderr)
        .try_init();
}

fn run_foreground(index_root: Option<String>, socket: Option<String>) -> Result<()> {
    let rt = tokio::runtime::Builder::new_multi_thread()
        .enable_all()
        .build()?;
    rt.block_on(async {
        let opts = DaemonOptions {
            index_root: index_root.map(Into::into),
            ..Default::default()
        };
        let state: Arc<DaemonState> = DaemonState::open(opts)?;
        let socket_path = match socket {
            Some(s) => parse_socket_arg(&s),
            None => sourcerer_rpc::default_socket_path(),
        };
        tracing::info!("sourcerer-indexd starting; socket={socket_path:?}");
        let handle = sourcerer_indexd::spawn_at(state.clone(), socket_path).await?;
        // Block until either the accept loop exits (terminal) or the
        // process is signaled. SIGINT/SIGTERM handling on Unix uses
        // tokio::signal; on Windows we let Ctrl-C bubble through the
        // service control plane.
        let _ = handle.await;
        let _ = state.persist().await;
        Ok::<(), anyhow::Error>(())
    })?;
    Ok(())
}

fn parse_socket_arg(s: &str) -> sourcerer_rpc::SocketPath {
    if s.starts_with(r"\\.\pipe\") || s.starts_with(r"\\?\pipe\") {
        sourcerer_rpc::SocketPath::Pipe(s.to_string())
    } else {
        sourcerer_rpc::SocketPath::Path(std::path::PathBuf::from(s))
    }
}
