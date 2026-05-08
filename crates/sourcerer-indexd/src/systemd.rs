//! Linux systemd-user-unit wiring for `sourcerer-indexd`.
//!
//! - `install()` writes the per-user systemd unit and runs
//!   `systemctl --user enable --now sourcerer-indexd.service`. The unit
//!   has `Type=simple`, `Restart=always`, `WantedBy=default.target` per
//!   the Phase-3 spec.
//! - `uninstall()` runs `systemctl --user disable --now` and deletes
//!   the unit file.
//! - `run_as_service()` is the entry point systemd invokes via the
//!   unit's `ExecStart`. Phase 3 ships an SCM-shape only — Phase 4
//!   will spawn the per-root subscriber + index core inside this body.
//!
//! The unit lives at `~/.config/systemd/user/sourcerer-indexd.service`
//! and runs as the logged-in user, never root. The optional fanotify
//! upgrade is brokered by polkit (action
//! `io.mikeweaver.sourcerer.elevate`); see `polkit/io.mikeweaver.
//! sourcerer.policy` for the policy file the installer drops at
//! `/usr/share/polkit-1/actions/`.

#![cfg(target_os = "linux")]

use std::path::{Path, PathBuf};
use std::process::Command;

use anyhow::{Context, Result};

const SERVICE_NAME: &str = "sourcerer-indexd.service";

/// Installs the systemd user unit. Idempotent: if the file already
/// exists it's overwritten and `systemctl --user enable --now` is
/// rerun. The unit is enabled into the current user's `default.target`
/// so it auto-starts at login.
pub fn install(binary_override: Option<&str>) -> Result<()> {
    let binary = match binary_override {
        Some(p) => PathBuf::from(p),
        None => std::env::current_exe().context("std::env::current_exe")?,
    };
    let binary = binary
        .canonicalize()
        .with_context(|| format!("canonicalize binary path `{}`", binary.display()))?;

    let unit_path = unit_path()?;
    let logs_dir = home()?
        .join(".local")
        .join("share")
        .join("sourcerer")
        .join("logs");
    std::fs::create_dir_all(&logs_dir)
        .with_context(|| format!("create logs dir `{}`", logs_dir.display()))?;

    let unit = render_unit(&binary, &logs_dir);
    if let Some(parent) = unit_path.parent() {
        std::fs::create_dir_all(parent)
            .with_context(|| format!("create systemd user dir `{}`", parent.display()))?;
    }
    std::fs::write(&unit_path, unit)
        .with_context(|| format!("write unit file `{}`", unit_path.display()))?;

    let reload = Command::new("systemctl")
        .args(["--user", "daemon-reload"])
        .status()
        .context("running `systemctl --user daemon-reload`")?;
    if !reload.success() {
        anyhow::bail!(
            "`systemctl --user daemon-reload` exited with {reload}; \
             unit file remains on disk for manual recovery"
        );
    }

    let enable = Command::new("systemctl")
        .args(["--user", "enable", "--now", SERVICE_NAME])
        .status()
        .context("running `systemctl --user enable --now`")?;
    if !enable.success() {
        anyhow::bail!(
            "`systemctl --user enable --now {SERVICE_NAME}` exited with {enable}; \
             unit file remains on disk for manual recovery"
        );
    }

    println!(
        "Installed systemd user unit `{SERVICE_NAME}` -> `{}` (Restart=always, WantedBy=default.target).",
        binary.display()
    );
    Ok(())
}

/// Removes the systemd registration. Idempotent over a missing unit.
pub fn uninstall() -> Result<()> {
    let unit_path = unit_path()?;
    if !unit_path.exists() {
        println!(
            "systemd user unit `{SERVICE_NAME}` is not installed (file missing); nothing to do."
        );
        return Ok(());
    }

    let disable = Command::new("systemctl")
        .args(["--user", "disable", "--now", SERVICE_NAME])
        .status()
        .context("running `systemctl --user disable --now`")?;
    if !disable.success() {
        // Surface a warning but still try to delete the unit file — a
        // partial install where systemd lost track is best cleaned up
        // by removing the file and reloading.
        eprintln!(
            "warning: `systemctl --user disable --now {SERVICE_NAME}` exited with {disable}; \
             deleting unit file anyway."
        );
    }

    std::fs::remove_file(&unit_path)
        .with_context(|| format!("removing unit file `{}`", unit_path.display()))?;

    let _ = Command::new("systemctl")
        .args(["--user", "daemon-reload"])
        .status();

    println!("Uninstalled systemd user unit `{SERVICE_NAME}`.");
    Ok(())
}

/// Body invoked by systemd via the unit's `ExecStart`. Phase 3 only
/// ensures the indexd binary launches and stays alive long enough for
/// `Restart=always` to be meaningful — no per-root subscribers spin up
/// here yet (Phase 4 wires those).
pub fn run_as_service() -> Result<()> {
    tracing::info!(
        unit = SERVICE_NAME,
        "systemd user-unit body entered (Phase 3 keep-alive shell)"
    );
    // Restart=always means systemd will respawn us if we exit. Phase 4
    // wires the actual indexer body; for Phase 3 we wait on a no-op
    // sleep loop so the unit is observable via `systemctl --user status`.
    loop {
        std::thread::sleep(std::time::Duration::from_secs(60));
    }
}

fn unit_path() -> Result<PathBuf> {
    Ok(home()?
        .join(".config")
        .join("systemd")
        .join("user")
        .join(SERVICE_NAME))
}

fn home() -> Result<PathBuf> {
    std::env::var_os("HOME")
        .map(PathBuf::from)
        .context("$HOME is not set; cannot locate ~/.config/systemd/user")
}

fn render_unit(binary: &Path, logs_dir: &Path) -> String {
    let stdout = logs_dir.join("indexd.log");
    let stderr = logs_dir.join("indexd.err");
    // systemd unit file syntax. `Type=simple` because the daemon does
    // not double-fork; `Restart=always` per the Phase-3 spec.
    // `WantedBy=default.target` is the per-user equivalent of
    // multi-user.target — it's what `systemctl --user enable` hooks
    // the unit into for auto-start at login.
    //
    // Binary path quoting: we wrap the path in double quotes so a
    // user-supplied `--binary "/path with spaces/sourcerer-indexd"` does
    // not split into multiple args under systemd's whitespace-aware
    // ExecStart parser. Backslashes and double-quotes inside the path
    // are escaped.
    format!(
        "[Unit]\n\
         Description=Sourcerer indexer daemon (Phase 3 user agent)\n\
         Documentation=https://github.com/MikesRuthless12/Sourcerer\n\
         After=default.target\n\
         \n\
         [Service]\n\
         Type=simple\n\
         ExecStart=\"{binary}\" service\n\
         Restart=always\n\
         RestartSec=2\n\
         StandardOutput=append:{stdout}\n\
         StandardError=append:{stderr}\n\
         \n\
         [Install]\n\
         WantedBy=default.target\n",
        binary = systemd_escape(&binary.display().to_string()),
        stdout = stdout.display(),
        stderr = stderr.display(),
    )
}

/// Escapes the path for use inside `"..."` in a systemd unit's
/// `ExecStart=`. systemd uses POSIX-shell-style quoting in unit files;
/// we escape backslash and double-quote, leaving everything else
/// untouched. Newlines / control chars in a binary path are vanishingly
/// rare and we let systemd reject those at unit-parse time.
fn systemd_escape(s: &str) -> String {
    let mut out = String::with_capacity(s.len());
    for c in s.chars() {
        match c {
            '\\' => out.push_str("\\\\"),
            '"' => out.push_str("\\\""),
            other => out.push(other),
        }
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn render_unit_contains_required_keys() {
        let unit = render_unit(
            Path::new("/usr/local/bin/sourcerer-indexd"),
            Path::new("/home/me/.local/share/sourcerer/logs"),
        );
        assert!(unit.contains("Type=simple"));
        assert!(unit.contains("Restart=always"));
        assert!(unit.contains("WantedBy=default.target"));
        assert!(unit.contains("ExecStart=\"/usr/local/bin/sourcerer-indexd\" service"));
        assert!(unit.contains("indexd.log"));
        assert!(unit.contains("indexd.err"));
    }

    #[test]
    fn render_unit_quotes_paths_with_spaces() {
        // A user-supplied `--binary "/path with spaces/indexd"` must
        // produce a single-arg ExecStart value, not three.
        let unit = render_unit(
            Path::new("/path with spaces/sourcerer-indexd"),
            Path::new("/home/me/.local/share/sourcerer/logs"),
        );
        assert!(unit.contains("ExecStart=\"/path with spaces/sourcerer-indexd\" service"));
    }

    #[test]
    fn systemd_escape_handles_quotes_and_backslash() {
        assert_eq!(systemd_escape("plain"), "plain");
        assert_eq!(systemd_escape("a\\b"), "a\\\\b");
        assert_eq!(systemd_escape("a\"b"), "a\\\"b");
        assert_eq!(
            systemd_escape("/path with spaces/indexd"),
            "/path with spaces/indexd"
        );
    }

    #[test]
    fn unit_path_uses_home() {
        let prior = std::env::var_os("HOME");
        // SAFETY: tests in this crate are not parallelized to the same
        // env var; std::env::set_var is safe here. Rust 2024 marks
        // env mutation `unsafe` to flag the global-mutation hazard.
        unsafe {
            std::env::set_var("HOME", "/tmp/test-home");
        }
        let path = unit_path().unwrap();
        assert_eq!(
            path,
            PathBuf::from("/tmp/test-home/.config/systemd/user/sourcerer-indexd.service")
        );
        unsafe {
            match prior {
                Some(v) => std::env::set_var("HOME", v),
                None => std::env::remove_var("HOME"),
            }
        }
    }
}
