//! Sourcerer indexer daemon.
//!
//! Phase 1 wired the Windows Service Control Manager scaffolding so the
//! daemon can be installed, uninstalled, and run as the
//! "Sourcerer-Indexd" service.
//!
//! Phase 2 added the macOS launchd-agent path: `install` writes a per-user
//! plist at `~/Library/LaunchAgents/io.mikeweaver.sourcerer.indexd.plist`
//! with `RunAtLoad=true` + `KeepAlive=true`, and the foreground `run`
//! mode subscribes to a path via the FSEvents subscriber for manual /
//! smoke-test invocation.
//!
//! Phase 3 adds the Linux systemd-user-unit path: `install` writes
//! `~/.config/systemd/user/sourcerer-indexd.service`
//! (Type=simple, Restart=always, WantedBy=default.target) and runs
//! `systemctl --user enable --now`. Foreground `run` opens the
//! inotify (or fanotify, when CAP_SYS_ADMIN is held) subscriber.
//! Phase 4 fills in the per-root subscriber + index core inside the
//! service body.

#[cfg(any(windows, target_os = "macos", target_os = "linux"))]
use anyhow::Context;
use anyhow::Result;
use clap::{Parser, Subcommand};

#[derive(Debug, Parser)]
#[command(name = "sourcerer-indexd", about = "Sourcerer indexer daemon", version)]
struct Cli {
    #[command(subcommand)]
    command: Option<Command>,
}

#[derive(Debug, Subcommand)]
enum Command {
    /// Run the indexer in the foreground (manual / dev / smoke-test mode).
    Run {
        /// Volume root to subscribe to. On Windows this is a drive root
        /// (e.g. `C:\`); on macOS it's any absolute directory served by
        /// FSEvents.
        #[arg(long)]
        volume: Option<String>,
        /// macOS-only alias for `--volume`. Accepts an absolute directory
        /// path; FSEvents events under that root are streamed.
        #[arg(long)]
        root: Option<String>,
        /// Print events to stdout instead of feeding them into the index.
        #[arg(long)]
        echo: bool,
    },
    /// Install the indexer as an OS-managed background service.
    ///
    /// - **Windows**: registers `Sourcerer-Indexd` with the Service
    ///   Control Manager (auto-start).
    /// - **macOS**: installs a launchd user agent at
    ///   `~/Library/LaunchAgents/io.mikeweaver.sourcerer.indexd.plist`
    ///   (RunAtLoad + KeepAlive).
    Install {
        /// Override the executable path registered with the OS service
        /// manager. Defaults to the location of the running binary.
        #[arg(long)]
        binary: Option<String>,
    },
    /// Uninstall the indexer service. Reverses `install`.
    Uninstall,
    /// Internal: the OS service manager (SCM on Win, launchd on macOS)
    /// invokes this via the registered service / `ProgramArguments`.
    /// End users should not call this directly.
    Service,
}

#[cfg(windows)]
mod windows_service;

#[cfg(target_os = "macos")]
mod launchd;

#[cfg(target_os = "linux")]
mod systemd;

fn main() -> Result<()> {
    init_tracing();
    let cli = Cli::parse();

    match cli.command.unwrap_or(Command::Run {
        volume: None,
        root: None,
        echo: true,
    }) {
        Command::Run { volume, root, echo } => run_foreground(volume, root, echo),
        #[cfg(windows)]
        Command::Install { binary } => windows_service::install(binary.as_deref()),
        #[cfg(target_os = "macos")]
        Command::Install { binary } => launchd::install(binary.as_deref()),
        #[cfg(target_os = "linux")]
        Command::Install { binary } => systemd::install(binary.as_deref()),
        #[cfg(all(not(windows), not(target_os = "macos"), not(target_os = "linux")))]
        Command::Install { .. } => {
            anyhow::bail!("`install` is only supported on Windows, macOS, and Linux.");
        }
        #[cfg(windows)]
        Command::Uninstall => windows_service::uninstall(),
        #[cfg(target_os = "macos")]
        Command::Uninstall => launchd::uninstall(),
        #[cfg(target_os = "linux")]
        Command::Uninstall => systemd::uninstall(),
        #[cfg(all(not(windows), not(target_os = "macos"), not(target_os = "linux")))]
        Command::Uninstall => {
            anyhow::bail!("`uninstall` is only supported on Windows, macOS, and Linux.");
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
            );
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
fn run_foreground(volume: Option<String>, _root: Option<String>, echo: bool) -> Result<()> {
    use futures::StreamExt;
    use std::path::PathBuf;

    let volume = volume
        .context("--volume is required (e.g. --volume C:\\). Phase 1 supports a single volume.")?;
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

    futures::executor::block_on(async move {
        let mut stream = Box::pin(subscriber.subscribe());
        while let Some(event) = stream.next().await {
            if echo {
                println!("{}", serde_json::to_string(&event).unwrap_or_default());
            }
        }
    });
    Ok(())
}

#[cfg(target_os = "macos")]
fn run_foreground(volume: Option<String>, root: Option<String>, echo: bool) -> Result<()> {
    use futures::StreamExt;
    use std::path::PathBuf;

    let target = root
        .or(volume)
        .context("--root (preferred on macOS) or --volume is required")?;
    let path = PathBuf::from(&target);

    tracing::info!(root = %path.display(), "opening FSEvents subscriber");
    let subscriber = sourcerer_journal_mac::open(&path)
        .with_context(|| format!("opening FSEvents stream on `{}`", path.display()))?;

    let cursor = subscriber.cursor();
    tracing::info!(
        root = %cursor.root.display(),
        device = cursor.device,
        last_event_id = cursor.last_event_id,
        fs = %cursor.fs_name,
        bootstrap_complete = cursor.bootstrap_complete,
        "subscriber opened"
    );

    futures::executor::block_on(async move {
        let mut stream = Box::pin(subscriber.subscribe());
        while let Some(event) = stream.next().await {
            if echo {
                println!("{}", serde_json::to_string(&event).unwrap_or_default());
            }
        }
    });
    Ok(())
}

#[cfg(target_os = "linux")]
fn run_foreground(volume: Option<String>, root: Option<String>, echo: bool) -> Result<()> {
    use futures::StreamExt;
    use std::path::PathBuf;

    let target = root
        .or(volume)
        .context("--root (preferred on Linux) or --volume is required")?;
    let path = PathBuf::from(&target);

    tracing::info!(root = %path.display(), "opening Linux journal subscriber");
    let subscriber = sourcerer_journal_lin::open(&path)
        .with_context(|| format!("opening journal subscriber on `{}`", path.display()))?;

    let cursor = subscriber.cursor();
    tracing::info!(
        root = %cursor.root.display(),
        device = cursor.device,
        fs = %cursor.fs_name,
        backend = ?cursor.backend,
        bootstrap_complete = cursor.bootstrap_complete,
        "subscriber opened"
    );

    futures::executor::block_on(async move {
        let mut stream = Box::pin(subscriber.subscribe());
        while let Some(event) = stream.next().await {
            if echo {
                println!("{}", serde_json::to_string(&event).unwrap_or_default());
            }
        }
    });
    Ok(())
}

#[cfg(all(not(windows), not(target_os = "macos"), not(target_os = "linux")))]
fn run_foreground(_volume: Option<String>, _root: Option<String>, _echo: bool) -> Result<()> {
    tracing::info!(
        "sourcerer-indexd: foreground run is supported on Windows, macOS, and Linux only"
    );
    Ok(())
}
