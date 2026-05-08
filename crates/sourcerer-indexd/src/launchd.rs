//! macOS launchd-agent wiring for `sourcerer-indexd`.
//!
//! - `install()` writes the per-user launchd plist and `launchctl load -w`s
//!   it. `RunAtLoad=true` + `KeepAlive=true` mirror the Build Guide spec.
//! - `uninstall()` runs `launchctl unload -w` and deletes the plist.
//! - `run_as_service()` is the entry point launchd invokes via the plist's
//!   `ProgramArguments`. Phase 2 ships an SCM-shape only — Phase 4 will
//!   spawn the per-root subscriber + index core inside this body.
//!
//! The plist lives at
//! `~/Library/LaunchAgents/io.mikeweaver.sourcerer.indexd.plist` and the
//! agent runs as the logged-in user, never root. fanotify-style elevation
//! is a Linux concern (Phase 3); macOS doesn't need it for the
//! per-user FSEvents stream.

#![cfg(target_os = "macos")]

use std::path::{Path, PathBuf};
use std::process::Command;

use anyhow::{Context, Result};

const SERVICE_LABEL: &str = "io.mikeweaver.sourcerer.indexd";

/// Installs the launchd user agent. Idempotent: if the plist already
/// exists it's overwritten and `launchctl load -w` is rerun. The agent
/// is loaded into the current user's domain only.
pub fn install(binary_override: Option<&str>) -> Result<()> {
    let binary = match binary_override {
        Some(p) => PathBuf::from(p),
        None => std::env::current_exe().context("std::env::current_exe")?,
    };
    let binary = binary
        .canonicalize()
        .with_context(|| format!("canonicalize binary path `{}`", binary.display()))?;

    let plist_path = plist_path()?;
    let logs_dir = home()?.join("Library").join("Logs").join("Sourcerer");
    std::fs::create_dir_all(&logs_dir)
        .with_context(|| format!("create logs dir `{}`", logs_dir.display()))?;

    let plist = render_plist(&binary, &logs_dir);
    if let Some(parent) = plist_path.parent() {
        std::fs::create_dir_all(parent)
            .with_context(|| format!("create LaunchAgents dir `{}`", parent.display()))?;
    }
    std::fs::write(&plist_path, plist)
        .with_context(|| format!("write plist `{}`", plist_path.display()))?;

    // `launchctl load -w` is the cross-version compatible form (10.10+).
    // `bootstrap` is preferred on 10.13+ but the plain `load` path still
    // works there. We prefer the conservative form for now.
    let status = Command::new("/bin/launchctl")
        .arg("load")
        .arg("-w")
        .arg(&plist_path)
        .status()
        .context("running `launchctl load -w`")?;
    if !status.success() {
        anyhow::bail!(
            "`launchctl load -w {}` exited with {}; plist remains on disk",
            plist_path.display(),
            status
        );
    }

    println!(
        "Installed launchd agent `{SERVICE_LABEL}` -> `{}` (RunAtLoad + KeepAlive).",
        binary.display()
    );
    Ok(())
}

/// Removes the launchd registration. Idempotent over a missing plist.
pub fn uninstall() -> Result<()> {
    let plist_path = plist_path()?;
    if !plist_path.exists() {
        println!(
            "launchd agent `{SERVICE_LABEL}` is not installed (plist missing); nothing to do."
        );
        return Ok(());
    }

    let unload = Command::new("/bin/launchctl")
        .arg("unload")
        .arg("-w")
        .arg(&plist_path)
        .status()
        .context("running `launchctl unload -w`")?;
    if !unload.success() {
        // Surface a warning but still try to delete the plist — a partial
        // install where launchd lost track is best cleaned up by removing
        // the plist file.
        eprintln!(
            "warning: `launchctl unload -w {}` exited with {}; deleting plist anyway.",
            plist_path.display(),
            unload
        );
    }
    std::fs::remove_file(&plist_path)
        .with_context(|| format!("removing plist `{}`", plist_path.display()))?;
    println!("Uninstalled launchd agent `{SERVICE_LABEL}`.");
    Ok(())
}

/// Body invoked by launchd via the plist's `ProgramArguments`. Phase 2
/// only ensures the indexd binary launches and stays alive long enough
/// for `KeepAlive=true` to be meaningful — no per-root subscribers spin
/// up here yet.
pub fn run_as_service() -> Result<()> {
    tracing::info!(
        label = SERVICE_LABEL,
        "launchd agent body entered (Phase 2 keep-alive shell)"
    );
    // KeepAlive=true means launchd will restart us if we exit. Phase 4
    // wires the actual indexer body; for Phase 2 we wait on a no-op
    // signal-handler loop so the agent is observable.
    loop {
        std::thread::sleep(std::time::Duration::from_secs(60));
    }
}

fn plist_path() -> Result<PathBuf> {
    Ok(home()?
        .join("Library")
        .join("LaunchAgents")
        .join(format!("{SERVICE_LABEL}.plist")))
}

fn home() -> Result<PathBuf> {
    std::env::var_os("HOME")
        .map(PathBuf::from)
        .context("$HOME is not set; cannot locate ~/Library/LaunchAgents")
}

fn render_plist(binary: &Path, logs_dir: &Path) -> String {
    let stdout = logs_dir.join("indexd.log");
    let stderr = logs_dir.join("indexd.err");
    format!(
        "<?xml version=\"1.0\" encoding=\"UTF-8\"?>\n\
         <!DOCTYPE plist PUBLIC \"-//Apple//DTD PLIST 1.0//EN\" \
         \"http://www.apple.com/DTDs/PropertyList-1.0.dtd\">\n\
         <plist version=\"1.0\">\n\
         <dict>\n\
         \t<key>Label</key>\n\
         \t<string>{label}</string>\n\
         \t<key>ProgramArguments</key>\n\
         \t<array>\n\
         \t\t<string>{binary}</string>\n\
         \t\t<string>service</string>\n\
         \t</array>\n\
         \t<key>RunAtLoad</key>\n\
         \t<true/>\n\
         \t<key>KeepAlive</key>\n\
         \t<true/>\n\
         \t<key>ProcessType</key>\n\
         \t<string>Background</string>\n\
         \t<key>StandardOutPath</key>\n\
         \t<string>{stdout}</string>\n\
         \t<key>StandardErrorPath</key>\n\
         \t<string>{stderr}</string>\n\
         </dict>\n\
         </plist>\n",
        label = SERVICE_LABEL,
        binary = xml_escape(&binary.display().to_string()),
        stdout = xml_escape(&stdout.display().to_string()),
        stderr = xml_escape(&stderr.display().to_string()),
    )
}

/// Minimal XML-1.0 attribute / text escape — the only special chars we
/// might see in a path are `&`, `<`, `>`, and (very rarely) `"` / `'`.
/// We only emit text content, not attributes, so quote-escaping is for
/// belt-and-braces correctness.
fn xml_escape(s: &str) -> String {
    let mut out = String::with_capacity(s.len());
    for c in s.chars() {
        match c {
            '&' => out.push_str("&amp;"),
            '<' => out.push_str("&lt;"),
            '>' => out.push_str("&gt;"),
            '"' => out.push_str("&quot;"),
            '\'' => out.push_str("&apos;"),
            other => out.push(other),
        }
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn render_plist_contains_required_keys() {
        let plist = render_plist(
            Path::new("/usr/local/bin/sourcerer-indexd"),
            Path::new("/Users/me/Library/Logs/Sourcerer"),
        );
        assert!(plist.contains("<string>io.mikeweaver.sourcerer.indexd</string>"));
        assert!(plist.contains("<key>RunAtLoad</key>"));
        assert!(plist.contains("<key>KeepAlive</key>"));
        assert!(plist.contains("<true/>"));
        assert!(plist.contains("/usr/local/bin/sourcerer-indexd"));
        assert!(plist.contains("<string>service</string>"));
        assert!(plist.contains("indexd.log"));
        assert!(plist.contains("indexd.err"));
    }

    #[test]
    fn xml_escape_handles_ampersand_and_brackets() {
        assert_eq!(xml_escape("a&b"), "a&amp;b");
        assert_eq!(xml_escape("<x>"), "&lt;x&gt;");
        assert_eq!(xml_escape("\"q\""), "&quot;q&quot;");
        assert_eq!(xml_escape("/Users/me/Movies"), "/Users/me/Movies");
    }

    #[test]
    fn plist_path_uses_home() {
        // Set HOME for the test, derive plist path, restore.
        let prior = std::env::var_os("HOME");
        // SAFETY: tests in this crate are not parallelized to the same
        // env var; std::env::set_var is safe here. Rust 2024 marks
        // env mutation `unsafe` to flag the global-mutation hazard.
        unsafe {
            std::env::set_var("HOME", "/tmp/test-home");
        }
        let path = plist_path().unwrap();
        assert_eq!(
            path,
            PathBuf::from(
                "/tmp/test-home/Library/LaunchAgents/io.mikeweaver.sourcerer.indexd.plist"
            )
        );
        unsafe {
            match prior {
                Some(v) => std::env::set_var("HOME", v),
                None => std::env::remove_var("HOME"),
            }
        }
    }
}
