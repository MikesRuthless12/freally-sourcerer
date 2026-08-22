//! Freally indexer daemon — binary entry point.
//!
//! This binary is a thin shim over the [`freally_indexd`] library. The
//! library exposes the daemon body so the Tauri app can also embed the
//! daemon when developing in-process.
//!
//! Phase 12 wiring:
//!
//! - `run` opens the index at the standard per-OS path and starts the
//!   RPC server at `default_socket_path()`. The Tauri app launches this
//!   as a sidecar process at boot.
//! - `install` / `uninstall` / `status` register, deregister and
//!   report on the OS-native service entry (Windows SCM / launchd /
//!   systemd-user).
//! - `service` is the entry point invoked by the OS service manager.
//!   Same body as `run`, but wrapped in the platform's service
//!   reporting conventions.

use anyhow::Result;
use clap::{Parser, Subcommand};
use freally_indexd::{DaemonOptions, DaemonState};
use std::sync::Arc;

#[cfg(target_os = "macos")]
mod launchd;
#[cfg(target_os = "linux")]
mod systemd;
#[cfg(windows)]
mod windows_service;

#[derive(Debug, Parser)]
#[command(name = "freally-indexd", about = "Freally indexer daemon", version)]
struct Cli {
    /// SRC-M17 — keep the index, config and logs in a `Data/` folder
    /// beside this binary, and refuse to register anything with the OS.
    /// Also turned on by a `portable.flag` file beside the binary.
    #[arg(long, global = true)]
    portable: bool,
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
    /// Report whether the indexer is installed as a service, and what
    /// the OS service manager currently makes of it.
    Status,
    /// Internal: the OS service manager invokes this.
    Service,
}

fn main() -> Result<()> {
    // Parse before logging: portable mode decides where the log goes,
    // and `--portable` is only visible after the parse.
    let cli = Cli::parse();
    if cli.portable {
        freally_rpc::portable::activate();
    }
    init_tracing();
    let cmd = cli.command.unwrap_or(Command::Run {
        index_root: None,
        socket: None,
    });
    // SRC-M17 — a portable install owns nothing outside its own `Data/`
    // folder. Registering a service would write to the SCM, a LaunchAgent
    // plist, or a systemd unit, all of which outlive the USB stick and
    // point at a path that will not be there next boot.
    if matches!(
        cmd,
        Command::Install { .. } | Command::Uninstall | Command::Service
    ) && freally_rpc::portable::is_active()
    {
        anyhow::bail!(
            "portable mode does not register an OS service. Run `freally-indexd run` \
             instead, or remove the `portable.flag` file beside this binary to install \
             normally."
        );
    }
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
        Command::Status => windows_service::status(),
        #[cfg(target_os = "macos")]
        Command::Status => launchd::status(),
        #[cfg(target_os = "linux")]
        Command::Status => systemd::status(),
        #[cfg(all(not(windows), not(target_os = "macos"), not(target_os = "linux")))]
        Command::Status => {
            anyhow::bail!("`status` is only supported on Windows, macOS, and Linux.")
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
    let filter = tracing_subscriber::EnvFilter::try_from_default_env()
        .unwrap_or_else(|_| tracing_subscriber::EnvFilter::new("info"));
    // A portable install is normally double-clicked, so stderr goes
    // nowhere a user can read. Log to `Data/logs/` instead — and fall
    // back to stderr if that file cannot be opened, rather than losing
    // the diagnostics that would explain why.
    if let Some(file) = freally_rpc::portable::open_log("indexd.log") {
        let _ = tracing_subscriber::fmt()
            .with_env_filter(filter)
            .with_ansi(false)
            .with_writer(std::sync::Mutex::new(file))
            .try_init();
        return;
    }
    let _ = tracing_subscriber::fmt()
        .with_env_filter(filter)
        .with_writer(std::io::stderr)
        .try_init();
}

fn run_foreground(index_root: Option<String>, socket: Option<String>) -> Result<()> {
    let rt = tokio::runtime::Builder::new_multi_thread()
        .enable_all()
        .build()?;
    rt.block_on(async {
        // An explicit `--index-root` still wins in portable mode: the
        // Tauri shell always passes one, and a smoke test needs to be
        // able to point a portable daemon at a scratch directory.
        let opts = DaemonOptions {
            index_root: index_root
                .map(Into::into)
                .or_else(freally_rpc::portable::index_root),
            ..Default::default()
        };
        let state: Arc<DaemonState> = DaemonState::open(opts)?;
        let socket_path = match socket {
            Some(s) => freally_rpc::parse_socket(&s),
            None => freally_rpc::portable::socket_path()
                .unwrap_or_else(freally_rpc::default_socket_path),
        };
        tracing::info!("freally-indexd starting; socket={socket_path:?}");
        let mut handle = freally_indexd::spawn_at(state.clone(), socket_path).await?;
        // Block until either the accept loop exits (terminal) or the
        // process is asked to stop. The stop half is not decoration: the
        // two lines after this commit whatever the watchers have pending,
        // and the default disposition of SIGTERM (or a console Ctrl-C)
        // is to kill the process before they run.
        tokio::select! {
            _ = &mut handle => {}
            () = stop_signal() => {
                tracing::info!("freally-indexd: stop signal received");
                handle.abort();
            }
        }
        // Stop live journaling before persisting: the consumer commits
        // whatever it has pending on the way out, so dropping the
        // process without this abandons applied-but-uncommitted events.
        state.watchers.shutdown();
        let _ = state.persist().await;
        Ok::<(), anyhow::Error>(())
    })?;
    Ok(())
}

/// Resolves when the OS asks this process to stop.
#[cfg(unix)]
async fn stop_signal() {
    freally_indexd::installed::unix_stop_signal().await
}

/// Windows has no SIGTERM. A foreground run is a console app, so Ctrl-C
/// is the signal; the installed service takes an entirely different path
/// through `windows_service`, which the SCM drives.
#[cfg(not(unix))]
async fn stop_signal() {
    let _ = tokio::signal::ctrl_c().await;
}
