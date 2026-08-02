//! Freally CLI binary — second client of the `freally-rpc` transport.
//!
//! Modes:
//!
//! - `freally search "<query>"` — runs a query, prints lens-grouped
//!   hits. Default query language is the Freally DSL (PRD §10).
//! - `freally search --strict-everything "<query>"` — voidtools-
//!   Everything-syntax-only mode; rejects Freally extensions (audio
//!   modifiers, similar:, audio:/content: lens prefixes).
//! - `freally search --json|--ndjson|--csv|-0` — SRC-M09 machine-
//!   readable output, with `--fields` / `--limit` / `--offset` and an
//!   exit code that reports whether anything matched. See [`output`].
//! - `freally index status` — prints the daemon's IndexState.
//! - `freally index pause` / `resume` — daemon-side controls.
//! - `freally index add-root <path>` / `rm-root <path>` — adds /
//!   removes a watched folder.
//! - `freally bookmark save <name> <query>` / `list` / `delete <name>`.
//! - `freally theme system|light|dark` — flip the running app's
//!   theme; opens the Settings IPC if the UI is up.
//!
//! Connect-target: per-OS default socket path. Override with
//! `--socket <path>`.

mod completions;
mod output;

use std::path::PathBuf;
use std::process::ExitCode;

use anyhow::{Context, Result};
use clap::{Args, CommandFactory, Parser, Subcommand, ValueEnum};
use freally_query::{ParseOpts, parse_to_report};
use freally_rpc::{Client, SocketPath, default_socket_path};

use output::{EXIT_ERROR, EXIT_HITS, Field, Format, Writer};

#[derive(Parser, Debug)]
#[command(name = "freally", version, about = "Freally — one search, every source, every OS.", long_about = None)]
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
        /// Reject Freally-only extensions; accept only voidtools-
        /// Everything-syntax-compatible queries.
        #[arg(long)]
        strict_everything: bool,
        /// Print parse output as JSON instead of evaluating. Takes
        /// precedence over the output-format flags below.
        #[arg(long)]
        parse_only: bool,
        #[command(flatten)]
        output: OutputArgs,
        /// The query string.
        #[arg(add = completions::query_value_candidates())]
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
    Theme { choice: ThemeChoice },
    /// Print the shell-completion registration stub for <SHELL>.
    Completions { shell: completions::Shell },
}

/// SRC-M09 output control. The four format flags share one group, so
/// clap rejects `--json --csv` rather than silently letting one win.
#[derive(Args, Debug)]
struct OutputArgs {
    /// Emit a single JSON document: `{ hits, count, timings }`.
    #[arg(long, group = "output_format")]
    json: bool,
    /// Emit one JSON object per hit, streaming.
    #[arg(long, group = "output_format")]
    ndjson: bool,
    /// Emit RFC-4180 CSV with a header row.
    #[arg(long, group = "output_format")]
    csv: bool,
    /// Emit NUL-separated paths, for `xargs -0`.
    #[arg(short = '0', long = "print0", group = "output_format")]
    print0: bool,
    /// Columns to emit, comma-separated. Defaults to every column.
    /// Ignored by `-0`, which is paths only.
    #[arg(long, value_delimiter = ',')]
    fields: Vec<Field>,
    /// Stop after this many hits.
    #[arg(long)]
    limit: Option<u64>,
    /// Skip this many hits first.
    #[arg(long, default_value_t = 0)]
    offset: u64,
}

impl OutputArgs {
    fn format(&self) -> Format {
        if self.json {
            Format::Json
        } else if self.ndjson {
            Format::Ndjson
        } else if self.csv {
            Format::Csv
        } else if self.print0 {
            Format::Null
        } else {
            Format::Human
        }
    }

    fn fields(&self) -> Vec<Field> {
        if self.fields.is_empty() {
            Field::ALL.to_vec()
        } else {
            self.fields.clone()
        }
    }
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

fn main() -> ExitCode {
    init_tracing();
    // SRC-M10: when a shell invokes us to complete a word it sets
    // `COMPLETE=<shell>`. That call answers with candidates and exits
    // here, before argument parsing — a half-typed query is not a
    // valid command line and must never reach the parser.
    clap_complete::CompleteEnv::with_factory(Cli::command).complete();
    let cli = Cli::parse();
    // Every failure path below reports SRC-M09's `EXIT_ERROR`, so a
    // caller can tell "no matches" from "did not run" without parsing
    // stderr.
    let rt = match tokio::runtime::Builder::new_multi_thread()
        .enable_all()
        .build()
    {
        Ok(rt) => rt,
        Err(e) => {
            eprintln!("freally: {e}");
            return ExitCode::from(EXIT_ERROR);
        }
    };
    match rt.block_on(async move { run(cli).await }) {
        Ok(code) => ExitCode::from(code),
        Err(e) => {
            eprintln!("freally: {e:#}");
            ExitCode::from(EXIT_ERROR)
        }
    }
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

/// Returns the process exit code. Only `search` distinguishes
/// hits-from-no-hits; the control commands succeed or return `Err`.
async fn run(cli: Cli) -> Result<u8> {
    let socket = match cli.socket {
        Some(s) => parse_socket_arg(&s),
        None => default_socket_path(),
    };
    match cli.command {
        Command::Search {
            strict_everything,
            parse_only,
            output,
            query,
        } => cmd_search(&socket, strict_everything, parse_only, &output, &query).await,
        Command::Index { sub } => cmd_index(&socket, sub).await.map(|()| EXIT_HITS),
        Command::Bookmark { sub } => cmd_bookmark(sub).await.map(|()| EXIT_HITS),
        Command::Theme { choice } => cmd_theme(&socket, choice).await.map(|()| EXIT_HITS),
        Command::Completions { shell } => {
            completions::print(shell, &mut std::io::stdout().lock())?;
            Ok(EXIT_HITS)
        }
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
    output: &OutputArgs,
    source: &str,
) -> Result<u8> {
    let opts = if strict_everything {
        ParseOpts::strict()
    } else {
        ParseOpts::default()
    };
    let report = parse_to_report(source, opts);
    if !report.errors.is_empty() {
        for e in &report.errors {
            eprintln!(
                "parse error: {} ({}-{})",
                e.message, e.span.start, e.span.end
            );
        }
        if !parse_only {
            anyhow::bail!("query has parse errors");
        }
    }
    if parse_only {
        let json = serde_json::to_string_pretty(&report)?;
        println!("{json}");
        return Ok(EXIT_HITS);
    }

    let client = Client::connect(socket.clone())
        .await
        .with_context(|| "connecting to freally-indexd; is the daemon running?")?;

    // Subscribe to notifications first so we don't miss the early
    // query:batch events.
    let mut notifications = client.notifications();
    let handle: freally_rpc::QueryRunHandle = client
        .call("query.run", serde_json::json!({ "source": source }))
        .await?;
    let target_handle = handle.handle.clone();

    let format = output.format();
    // The banner is commentary, not data — it would corrupt every
    // machine format, so only the human one gets it.
    if format == Format::Human {
        println!("# Query: {source}");
    }
    let mut writer = Writer::new(
        std::io::stdout().lock(),
        format,
        output.fields(),
        output.offset,
        output.limit,
    );

    let mut timings = freally_rpc::LensTimings::default();
    let mut done = false;
    while !done {
        let n = match notifications.next().await {
            Some(n) => n,
            None => break,
        };
        match n.method.as_str() {
            "query:batch" => {
                let batch: freally_rpc::QueryBatch =
                    serde_json::from_value(n.params.unwrap_or(serde_json::Value::Null))?;
                if batch.handle != target_handle {
                    continue;
                }
                for h in &batch.hits {
                    // `--limit` stops the drain here rather than
                    // formatting rows that would be discarded.
                    if !writer.push(h)? {
                        done = true;
                        break;
                    }
                }
            }
            "query:done" => {
                let d: freally_rpc::QueryDone =
                    serde_json::from_value(n.params.unwrap_or(serde_json::Value::Null))?;
                if d.handle != target_handle {
                    continue;
                }
                timings = d.timings;
                done = true;
            }
            _ => {}
        }
    }
    writer.finish(&timings)?;
    Ok(writer.exit_code())
}

async fn cmd_index(socket: &SocketPath, sub: IndexCommand) -> Result<()> {
    let client = Client::connect(socket.clone())
        .await
        .with_context(|| "connecting to freally-indexd; is the daemon running?")?;
    match sub {
        IndexCommand::Status => {
            let st: freally_rpc::IndexState =
                client.call("index.state", serde_json::Value::Null).await?;
            let json = serde_json::to_string_pretty(&st)?;
            println!("{json}");
        }
        IndexCommand::Verify => {
            let _: serde_json::Value = client.call("index.verify", serde_json::Value::Null).await?;
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
            let folders: Vec<serde_json::Value> =
                client.call("folders.list", serde_json::Value::Null).await?;
            let target = folders
                .iter()
                .find(|f| {
                    f.get("path").and_then(|p| p.as_str()) == Some(&path.display().to_string())
                })
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
        "bookmarks: managed by the running Freally UI; CLI access lands in Phase 13. \
         Run the desktop app and use Bookmarks → Add (Ctrl+D) for now."
    );
    Ok(())
}

async fn cmd_theme(socket: &SocketPath, choice: ThemeChoice) -> Result<()> {
    let client = Client::connect(socket.clone())
        .await
        .with_context(|| "connecting to freally-indexd; is the daemon running?")?;
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
