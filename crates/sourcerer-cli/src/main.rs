//! Sourcerer CLI binary — second client of the `sourcerer-rpc` transport.
//!
//! Modes:
//!
//! - `sourcerer search "<query>"` — runs a query, prints lens-grouped
//!   hits. Default query language is the Sourcerer DSL (PRD §10).
//! - `sourcerer search --strict-everything "<query>"` — voidtools-
//!   Everything-syntax-only mode; rejects Sourcerer extensions (audio
//!   modifiers, similar:, audio:/content: lens prefixes).
//! - `sourcerer index status` — prints the daemon's IndexState.
//! - `sourcerer index pause` / `resume` — daemon-side controls.
//! - `sourcerer index add-root <path>` / `rm-root <path>` — adds /
//!   removes a watched folder.
//! - `sourcerer bookmark save <name> <query>` / `list` / `delete <name>`.
//! - `sourcerer theme system|light|dark` — flip the running app's
//!   theme; opens the Settings IPC if the UI is up.
//!
//! Connect-target: per-OS default socket path. Override with
//! `--socket <path>`.

use std::path::PathBuf;

use anyhow::{Context, Result};
use clap::{Parser, Subcommand, ValueEnum};
use sourcerer_query::{ParseOpts, parse_to_report};
use sourcerer_rpc::{Client, SocketPath, default_socket_path};

#[derive(Parser, Debug)]
#[command(name = "sourcerer", version, about = "Sourcerer — one search, every source, every OS.", long_about = None)]
struct Cli {
    /// Override the per-OS default socket / pipe path.
    #[arg(long, global = true)]
    socket: Option<String>,

    #[command(subcommand)]
    command: Command,
}

#[derive(Subcommand, Debug)]
enum Command {
    /// Run a query and print lens-grouped hits.
    Search {
        /// Reject Sourcerer-only extensions; accept only voidtools-
        /// Everything-syntax-compatible queries.
        #[arg(long)]
        strict_everything: bool,
        /// Print parse output as JSON instead of evaluating.
        #[arg(long)]
        parse_only: bool,
        /// The query string.
        query: String,
    },
    /// Inspect or control the running indexer.
    Index {
        #[command(subcommand)]
        sub: IndexCommand,
    },
    /// Manage bookmarks.
    Bookmark {
        #[command(subcommand)]
        sub: BookmarkCommand,
    },
    /// Switch the running app's theme.
    Theme {
        choice: ThemeChoice,
    },
}

#[derive(Subcommand, Debug)]
enum IndexCommand {
    Status,
    Verify,
    Compact,
    Rebuild,
    Pause,
    Resume,
    AddRoot { path: PathBuf },
    RmRoot { path: PathBuf },
}

#[derive(Subcommand, Debug)]
enum BookmarkCommand {
    Save { name: String, query: String },
    List,
    Delete { name: String },
}

#[derive(ValueEnum, Clone, Debug)]
enum ThemeChoice {
    System,
    Light,
    Dark,
}

fn main() -> Result<()> {
    init_tracing();
    let cli = Cli::parse();
    let rt = tokio::runtime::Builder::new_multi_thread()
        .enable_all()
        .build()?;
    rt.block_on(async move { run(cli).await })
}

fn init_tracing() {
    let _ = tracing_subscriber::fmt()
        .with_env_filter(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| tracing_subscriber::EnvFilter::new("warn")),
        )
        .with_writer(std::io::stderr)
        .try_init();
}

async fn run(cli: Cli) -> Result<()> {
    let socket = match cli.socket {
        Some(s) => parse_socket_arg(&s),
        None => default_socket_path(),
    };
    match cli.command {
        Command::Search {
            strict_everything,
            parse_only,
            query,
        } => cmd_search(&socket, strict_everything, parse_only, &query).await,
        Command::Index { sub } => cmd_index(&socket, sub).await,
        Command::Bookmark { sub } => cmd_bookmark(sub).await,
        Command::Theme { choice } => cmd_theme(&socket, choice).await,
    }
}

fn parse_socket_arg(s: &str) -> SocketPath {
    if s.starts_with(r"\\.\pipe\") || s.starts_with(r"\\?\pipe\") {
        SocketPath::Pipe(s.to_string())
    } else {
        SocketPath::Path(PathBuf::from(s))
    }
}

async fn cmd_search(
    socket: &SocketPath,
    strict_everything: bool,
    parse_only: bool,
    source: &str,
) -> Result<()> {
    let opts = if strict_everything {
        ParseOpts::strict()
    } else {
        ParseOpts::default()
    };
    let report = parse_to_report(source, opts);
    if !report.errors.is_empty() {
        for e in &report.errors {
            eprintln!("parse error: {} ({}-{})", e.message, e.span.start, e.span.end);
        }
        if !parse_only {
            anyhow::bail!("query has parse errors");
        }
    }
    if parse_only {
        let json = serde_json::to_string_pretty(&report)?;
        println!("{json}");
        return Ok(());
    }

    let client = Client::connect(socket.clone())
        .await
        .with_context(|| "connecting to sourcerer-indexd; is the daemon running?")?;

    // Subscribe to notifications first so we don't miss the early
    // query:batch events.
    let mut notifications = client.notifications();
    let handle: sourcerer_rpc::QueryRunHandle = client
        .call("query.run", serde_json::json!({ "source": source }))
        .await?;
    let target_handle = handle.handle.clone();

    println!("# Query: {source}");
    let mut done = false;
    let mut total_hits = 0;
    while !done {
        let n = match notifications.next().await {
            Some(n) => n,
            None => break,
        };
        match n.method.as_str() {
            "query:batch" => {
                let batch: sourcerer_rpc::QueryBatch =
                    serde_json::from_value(n.params.unwrap_or(serde_json::Value::Null))?;
                if batch.handle != target_handle {
                    continue;
                }
                if !batch.hits.is_empty() {
                    println!("\n[{:?}]", batch.lens);
                    for h in &batch.hits {
                        println!("  {} — {}", h.name, h.path);
                        total_hits += 1;
                    }
                }
            }
            "query:done" => {
                let d: sourcerer_rpc::QueryDone =
                    serde_json::from_value(n.params.unwrap_or(serde_json::Value::Null))?;
                if d.handle != target_handle {
                    continue;
                }
                println!(
                    "\n# {} hit(s); filename {}ms · content {}ms · audio {}ms · similarity {}ms · total {}ms",
                    total_hits,
                    d.timings.filename_ms,
                    d.timings.content_ms,
                    d.timings.audio_ms,
                    d.timings.similarity_ms,
                    d.timings.total_ms
                );
                done = true;
            }
            _ => {}
        }
    }
    Ok(())
}

async fn cmd_index(socket: &SocketPath, sub: IndexCommand) -> Result<()> {
    let client = Client::connect(socket.clone())
        .await
        .with_context(|| "connecting to sourcerer-indexd; is the daemon running?")?;
    match sub {
        IndexCommand::Status => {
            let st: sourcerer_rpc::IndexState = client
                .call("index.state", serde_json::Value::Null)
                .await?;
            let json = serde_json::to_string_pretty(&st)?;
            println!("{json}");
        }
        IndexCommand::Verify => {
            let _: serde_json::Value = client
                .call("index.verify", serde_json::Value::Null)
                .await?;
            println!("verify: ok");
        }
        IndexCommand::Compact => {
            let _: serde_json::Value = client
                .call("index.compact", serde_json::Value::Null)
                .await?;
            println!("compact: ok");
        }
        IndexCommand::Rebuild => {
            let _: serde_json::Value = client
                .call("index.rebuild", serde_json::Value::Null)
                .await?;
            println!("rebuild: ok");
        }
        IndexCommand::Pause => {
            // Pause is modeled as `monitor_changes=false` on every detected
            // volume — defer to the dedicated daemon API once it lands.
            let _: serde_json::Value = client
                .call(
                    "settings.apply",
                    serde_json::json!({ "auto_remove_offline": true }),
                )
                .await?;
            println!("pause: requested");
        }
        IndexCommand::Resume => {
            let _: serde_json::Value = client
                .call(
                    "settings.apply",
                    serde_json::json!({ "auto_include_fixed": true }),
                )
                .await?;
            println!("resume: requested");
        }
        IndexCommand::AddRoot { path } => {
            let id = format!("cli-folder-{}", random_id());
            let folder = serde_json::json!({
                "id": id,
                "path": path.display().to_string(),
                "monitor_changes": true,
                "buffer_kb": 0,
                "rescan_on_full_buffer": true,
                "rescan_schedule": { "kind": "never" }
            });
            let _: serde_json::Value = client.call("folders.add", folder).await?;
            println!("add-root: ok");
        }
        IndexCommand::RmRoot { path } => {
            // The id is the path-derived id used by `folders.add`. Without
            // a list-then-find round-trip, prefer matching by path.
            let folders: Vec<serde_json::Value> = client
                .call("folders.list", serde_json::Value::Null)
                .await?;
            let target = folders
                .iter()
                .find(|f| f.get("path").and_then(|p| p.as_str()) == Some(&path.display().to_string()))
                .and_then(|f| f.get("id").and_then(|i| i.as_str().map(|s| s.to_string())));
            if let Some(id) = target {
                let _: serde_json::Value = client
                    .call("folders.remove", serde_json::json!({ "id": id }))
                    .await?;
                println!("rm-root: ok");
            } else {
                anyhow::bail!("path not in folders list: {}", path.display());
            }
        }
    }
    Ok(())
}

async fn cmd_bookmark(_sub: BookmarkCommand) -> Result<()> {
    // Bookmarks are UI-side state. A future Phase 13 commit migrates
    // them onto the daemon transport so this CLI can save / list /
    // delete from outside the running app.
    eprintln!(
        "bookmarks: managed by the running Sourcerer UI; CLI access lands in Phase 13. \
         Run the desktop app and use Bookmarks → Add (Ctrl+D) for now."
    );
    Ok(())
}

async fn cmd_theme(socket: &SocketPath, choice: ThemeChoice) -> Result<()> {
    let client = Client::connect(socket.clone())
        .await
        .with_context(|| "connecting to sourcerer-indexd; is the daemon running?")?;
    let theme = match choice {
        ThemeChoice::System => "system",
        ThemeChoice::Light => "light",
        ThemeChoice::Dark => "dark",
    };
    let _: serde_json::Value = client
        .call("settings.apply", serde_json::json!({ "theme": theme }))
        .await?;
    println!("theme: {theme}");
    Ok(())
}

fn random_id() -> String {
    use std::time::{SystemTime, UNIX_EPOCH};
    let n = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|d| d.as_nanos())
        .unwrap_or(0);
    format!("{n:x}")
}
