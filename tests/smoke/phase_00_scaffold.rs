//! Phase 0 smoke — OS-agnostic, runs on every CI matrix entry.
//!
//! Verifies the repo invariants any OS can check from inside a Rust process:
//!   1. The 18 expected locale .ftl files are present.
//!   2. The Cargo workspace lists every crate from the Build Guide.
//!   3. The Tauri app's tauri.conf.json declares the cross-platform identifier
//!      and the 1100x720 dark window.
//!   4. The icon master SVG is present.
//!
//! The companion `cargo build --all` + `pnpm tauri build --debug` exit-code
//! check lives in the per-OS shell scripts.

use std::path::{Path, PathBuf};

fn workspace_root() -> PathBuf {
    let mut p = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    while !p.join("Cargo.toml").exists() || !p.join("docs").exists() {
        if !p.pop() {
            panic!("could not locate workspace root from CARGO_MANIFEST_DIR");
        }
    }
    p
}

fn read(path: &Path) -> String {
    std::fs::read_to_string(path).unwrap_or_else(|e| panic!("read {}: {e}", path.display()))
}

#[test]
fn all_eighteen_locales_present() {
    let root = workspace_root().join("locales");
    let codes = [
        "en", "es", "zh-CN", "hi", "ar", "pt-BR", "ru", "ja", "de", "fr", "ko", "it", "tr", "vi",
        "pl", "nl", "id", "uk",
    ];
    for code in codes {
        let p = root.join(code).join("sourcerer.ftl");
        assert!(p.exists(), "missing locale file: {}", p.display());
    }
}

#[test]
fn workspace_lists_every_build_guide_crate() {
    let toml = read(&workspace_root().join("Cargo.toml"));
    let expected = [
        "crates/sourcerer-journal",
        "crates/sourcerer-journal-win",
        "crates/sourcerer-journal-mac",
        "crates/sourcerer-journal-lin",
        "crates/sourcerer-index",
        "crates/sourcerer-extractors",
        "crates/sourcerer-audio",
        "crates/sourcerer-similarity",
        "crates/sourcerer-query",
        "crates/sourcerer-http",
        "crates/sourcerer-i18n",
        "crates/sourcerer-cli",
        "crates/sourcerer-indexd",
        "xtask",
    ];
    for member in expected {
        assert!(
            toml.contains(member),
            "workspace Cargo.toml missing member {member}"
        );
    }
}

#[test]
fn tauri_conf_has_identifier_and_window() {
    let conf = read(
        &workspace_root()
            .join("apps")
            .join("sourcerer-ui")
            .join("src-tauri")
            .join("tauri.conf.json"),
    );
    assert!(conf.contains("\"io.mikeweaver.sourcerer\""));
    assert!(conf.contains("\"width\": 1100"));
    assert!(conf.contains("\"height\": 720"));
    assert!(conf.contains("\"theme\": \"Dark\""));
    assert!(conf.contains("\"version\": \"0.19.84\""));
}

#[test]
fn icon_master_svg_present() {
    let svg = workspace_root()
        .join("assets")
        .join("icons")
        .join("sourcerer.svg");
    assert!(svg.exists(), "missing {}", svg.display());
}

#[test]
fn deny_toml_bans_agpl() {
    let deny = read(&workspace_root().join("deny.toml"));
    assert!(deny.contains("AGPL"), "deny.toml must mention AGPL ban");
    // Ensure AGPL is NOT in the allow list.
    let allow_block_start = deny
        .find("allow = [")
        .expect("deny.toml has an `allow = [...]` block");
    let allow_block_end = allow_block_start
        + deny[allow_block_start..]
            .find(']')
            .expect("allow block has a closing bracket");
    let allow_block = &deny[allow_block_start..allow_block_end];
    assert!(
        !allow_block.contains("AGPL"),
        "AGPL must not appear in deny.toml allow-list"
    );
}
