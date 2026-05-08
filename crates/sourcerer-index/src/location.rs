//! Per-OS canonical index-root resolution (TASK-033).
//!
//! Windows: `%LOCALAPPDATA%\Sourcerer\index`
//! macOS:   `~/Library/Application Support/Sourcerer/index`
//! Linux:   `${XDG_DATA_HOME:-~/.local/share}/sourcerer/index`
//!
//! Other Unix targets fall back to the Linux convention so the workspace
//! still compiles cross-OS without `cfg`-fences in callers.

use std::path::PathBuf;

use crate::error::IndexError;

/// Returns the default index root for the current OS, honoring the
/// per-platform conventions called out in the Build Guide. Resolution is
/// done by environment variable rather than `dirs`-style crates so the
/// crate has zero non-permissive dependency exposure.
pub fn default_index_root() -> Result<PathBuf, IndexError> {
    let mut p = platform_data_dir()?;
    p.push("Sourcerer");
    p.push("index");
    Ok(p)
}

#[cfg(windows)]
fn platform_data_dir() -> Result<PathBuf, IndexError> {
    if let Some(v) = std::env::var_os("LOCALAPPDATA") {
        return Ok(PathBuf::from(v));
    }
    let mut user = std::env::var_os("USERPROFILE")
        .map(PathBuf::from)
        .ok_or_else(|| {
            IndexError::InvalidRoot(PathBuf::from(
                "neither %LOCALAPPDATA% nor %USERPROFILE% is set",
            ))
        })?;
    user.push("AppData");
    user.push("Local");
    Ok(user)
}

#[cfg(target_os = "macos")]
fn platform_data_dir() -> Result<PathBuf, IndexError> {
    let mut home = std::env::var_os("HOME")
        .map(PathBuf::from)
        .ok_or_else(|| IndexError::InvalidRoot(PathBuf::from("$HOME is not set")))?;
    home.push("Library");
    home.push("Application Support");
    Ok(home)
}

#[cfg(all(unix, not(target_os = "macos")))]
fn platform_data_dir() -> Result<PathBuf, IndexError> {
    if let Some(v) = std::env::var_os("XDG_DATA_HOME") {
        let p = PathBuf::from(v);
        if p.is_absolute() {
            return Ok(p);
        }
    }
    let mut home = std::env::var_os("HOME")
        .map(PathBuf::from)
        .ok_or_else(|| IndexError::InvalidRoot(PathBuf::from("$HOME is not set")))?;
    home.push(".local");
    home.push("share");
    Ok(home)
}

#[cfg(all(not(unix), not(windows)))]
fn platform_data_dir() -> Result<PathBuf, IndexError> {
    Err(IndexError::InvalidRoot(PathBuf::from(
        "unsupported platform — set the index root explicitly",
    )))
}
