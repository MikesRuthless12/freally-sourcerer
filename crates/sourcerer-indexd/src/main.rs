//! Sourcerer indexer daemon.
//!
//! Phase 1 wires the Windows Service Control Manager scaffolding so the
//! daemon can be installed, uninstalled, and run as the
//! "Sourcerer-Indexd" service. Bootstrapping per-volume USN journal
//! subscribers happens once a configured volume list is available — for
//! Phase 1 the `run` subcommand accepts a single `--volume` flag for
//! manual / smoke-test invocation.

use anyhow::{Context, Result};
use clap::{Parser, Subcommand};

#[derive(Debug, Parser)]
#[command(
    name = "sourcerer-indexd",
    about = "Sourcerer indexer daemon",
    version
)]
struct Cli {
    #[command(subcommand)]
    command: Option<Command>,
}

#[derive(Debug, Subcommand)]
enum Command {
    /// Run the indexer in the foreground (manual / dev / smoke-test mode).
    Run {
        /// Volume root to subscribe to (e.g. `C:\`). Required on Windows.
        #[arg(long)]
        volume: Option<String>,
        /// Print events to stdout instead of feeding them into the index.
        #[arg(long)]
        echo: bool,
    },
    /// Install the Sourcerer-Indexd Windows Service. Windows-only.
    Install {
        /// Override the executable path registered with SCM. Defaults to
        /// the location of the running binary.
        #[arg(long)]
        binary: Option<String>,
    },
    /// Uninstall the Sourcerer-Indexd Windows Service. Windows-only.
    Uninstall,
    /// Internal: SCM-invoked entry point. End users should not call this
    /// directly — the Service Control Manager invokes it via the
    /// `ImagePath` registered at install time.
    Service,
}

#[cfg(windows)]
mod windows_service;

fn main() -> Result<()> {
    init_tracing();
    let cli = Cli::parse();

    match cli.command.unwrap_or(Command::Run {
        volume: None,
        echo: true,
    }) {
        Command::Run { volume, echo } => run_foreground(volume, echo),
        #[cfg(windows)]
        Command::Install { binary } => windows_service::install(binary.as_deref()),
        #[cfg(not(windows))]
        Command::Install { .. } => {
            anyhow::bail!("`install` is Windows-only. Use launchd / systemd-user units \
                           on macOS / Linux (Phases 2 and 3).");
        }
        #[cfg(windows)]
        Command::Uninstall => windows_service::uninstall(),
        #[cfg(not(windows))]
        Command::Uninstall => {
            anyhow::bail!("`uninstall` is Windows-only.");
        }
        #[cfg(windows)]
        Command::Service => windows_service::run_as_service(),
        #[cfg(not(windows))]
        Command::Service => {
            anyhow::bail!("`service` is Windows-only — invoked by the Service Control Manager.");
        }
    }
}

fn init_tracing() {
    let _ = tracing_subscriber::fmt()
        .with_env_filter(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| tracing_subscriber::EnvFilter::new("info")),
        )
        .try_init();
}

#[cfg(windows)]
fn run_foreground(volume: Option<String>, echo: bool) -> Result<()> {
    use futures::StreamExt;
    use std::path::PathBuf;

    let volume = volume.context(
        "--volume is required (e.g. --volume C:\\). Phase 1 supports a single volume.",
    )?;
    let path = PathBuf::from(&volume);

    tracing::info!(volume = %path.display(), "opening USN journal subscriber");
    let subscriber = sourcerer_journal_win::open(&path)
        .with_context(|| format!("opening USN journal on `{}`", path.display()))?;

    let cursor = subscriber.cursor();
    tracing::info!(
        volume_serial = format_args!("{:08x}", cursor.volume_serial),
        journal_id = cursor.journal_id,
        next_usn = cursor.next_usn,
        fs = %cursor.fs_name,
        "subscriber opened"
    );

    let runtime = build_runtime();
    runtime.block_on(async move {
        let mut stream = Box::pin(subscriber.subscribe());
        while let Some(event) = stream.next().await {
            if echo {
                println!("{}", serde_json::to_string(&event).unwrap_or_default());
            }
        }
    });
    Ok(())
}

#[cfg(not(windows))]
fn run_foreground(_volume: Option<String>, _echo: bool) -> Result<()> {
    tracing::info!("sourcerer-indexd Phase 1: foreground run is Windows-only");
    Ok(())
}

#[cfg(windows)]
fn build_runtime() -> RuntimeShim {
    RuntimeShim
}

#[cfg(windows)]
struct RuntimeShim;

#[cfg(windows)]
impl RuntimeShim {
    fn block_on<F: std::future::Future>(&self, f: F) -> F::Output {
        // Tiny single-threaded executor. The journal-win crate uses
        // `futures::channel::mpsc` which is fully Send, so we don't need a
        // multi-thread runtime here. Keeps the binary small.
        futures::executor::block_on(f)
    }
}
