//! Sourcerer xtask — build helpers.
//!
//! Subcommands:
//!   - i18n-lint: verify every locale file has the same keys as the English source.
//!   - third-party-notices: (Phase 0 stub) regenerate THIRD-PARTY-NOTICES.md.
//!   - icon-build: render the SVG master to .ico / .icns / Hicolor PNG set.
//!   - release: (Phase 0 stub) wire the release pipeline in Phase 13.
//!   - gen-fixture: build a synthetic Sourcerer index for the Phase-5
//!     filename-lens bench / smoke runs.

use std::path::PathBuf;

use clap::{Parser, Subcommand};

mod cmd;

#[derive(Parser, Debug)]
#[command(name = "xtask", version, about = "Sourcerer build helpers")]
struct Cli {
    #[command(subcommand)]
    command: Cmd,
}

#[derive(Subcommand, Debug)]
enum Cmd {
    /// Verify every locale .ftl matches the English key-set.
    I18nLint {
        /// Override the locales root.
        #[arg(long)]
        locales: Option<PathBuf>,
    },
    /// (Stub) regenerate THIRD-PARTY-NOTICES.md.
    ThirdPartyNotices,
    /// Render the SVG master to .ico, .icns, and Hicolor PNGs.
    IconBuild {
        /// Override the assets root.
        #[arg(long)]
        assets: Option<PathBuf>,
    },
    /// (Stub) Phase 0 placeholder; release pipeline lands in Phase 13.
    Release,
    /// Generate a synthetic Sourcerer index for Phase-5 perf runs.
    GenFixture {
        /// Output index root.
        #[arg(long)]
        out: Option<PathBuf>,
        /// Number of synthetic file rows.
        #[arg(long, default_value_t = 200_000)]
        count: usize,
        /// PRNG seed; same seed + count = identical fixture.
        #[arg(long, default_value_t = 0xC0FFEE_u64)]
        seed: u64,
    },
}

fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();
    match cli.command {
        Cmd::I18nLint { locales } => cmd::i18n_lint::run(locales),
        Cmd::ThirdPartyNotices => cmd::third_party_notices::run(),
        Cmd::IconBuild { assets } => cmd::icon_build::run(assets),
        Cmd::Release => cmd::release::run(),
        Cmd::GenFixture { out, count, seed } => {
            let out_dir = out.unwrap_or_else(cmd::gen_fixture::default_fixture_root);
            cmd::gen_fixture::run(out_dir, count, seed)
        }
    }
}

pub(crate) fn workspace_root() -> PathBuf {
    let manifest = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    manifest
        .parent()
        .map(PathBuf::from)
        .unwrap_or_else(|| manifest.clone())
}
