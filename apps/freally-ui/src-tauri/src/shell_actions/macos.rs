//! macOS shell integration.
//!
//! Handler discovery reads what applications themselves declare: each
//! `.app` bundle's `Info.plist` lists `CFBundleDocumentTypes`, and each
//! of those lists the extensions it opens. Walking the standard
//! application directories and reading those declarations gives the
//! same answer Finder's "Open With" does for the common case, in safe
//! Rust, with no Objective-C bridge and no private LaunchServices
//! tooling.
//!
//! Launching goes through `/usr/bin/open -a`, which hands the request
//! to LaunchServices — so the app opens in the user's session with the
//! right activation policy, not as a child of this process.

use std::path::{Path, PathBuf};

use super::{AppHandler, Result, ShellError, detach};

/// Where macOS keeps applications. `~/Applications` is scanned last so
/// a user-installed copy does not shadow the system one in the list.
fn application_dirs() -> Vec<PathBuf> {
    let mut dirs = vec![
        PathBuf::from("/Applications"),
        PathBuf::from("/System/Applications"),
        PathBuf::from("/Applications/Utilities"),
        PathBuf::from("/System/Applications/Utilities"),
    ];
    if let Ok(home) = std::env::var("HOME") {
        dirs.push(PathBuf::from(home).join("Applications"));
    }
    dirs
}

/// Extensions an `Info.plist` declares, lowercased.
///
/// `Info.plist` is usually a binary plist, so rather than depend on a
/// plist parser we scan for the extension strings directly: both the
/// XML and binary encodings store `CFBundleTypeExtensions` values as
/// plain UTF-8 runs. We look for the marker key, then collect the short
/// ASCII tokens that follow it. This over-collects slightly — a bundle
/// that declares `pdf` will match `pdf` — which is the safe direction:
/// a wrong entry shows an app that then declines the file, whereas a
/// missing entry hides an app the user wanted.
fn declared_extensions(plist: &[u8]) -> Vec<String> {
    const MARKER: &[u8] = b"CFBundleTypeExtensions";
    let mut out: Vec<String> = Vec::new();
    let mut from = 0usize;
    while let Some(rel) = plist[from..]
        .windows(MARKER.len())
        .position(|w| w == MARKER)
    {
        let start = from + rel + MARKER.len();
        // Extensions cluster right after the key in both encodings;
        // 2 KiB covers the largest real document-type array without
        // running into the next unrelated key.
        let end = (start + 2048).min(plist.len());
        for token in ascii_tokens(&plist[start..end]) {
            if is_extension_like(&token) {
                let lower = token.to_ascii_lowercase();
                if !out.contains(&lower) {
                    out.push(lower);
                }
            }
        }
        from = start;
    }
    out
}

/// Runs of printable ASCII, which is what both plist encodings store
/// short keys and values as.
fn ascii_tokens(bytes: &[u8]) -> Vec<String> {
    let mut out = Vec::new();
    let mut current = String::new();
    for &b in bytes {
        if b.is_ascii_alphanumeric() || b == b'-' || b == b'_' {
            current.push(b as char);
        } else if !current.is_empty() {
            out.push(std::mem::take(&mut current));
        }
    }
    if !current.is_empty() {
        out.push(current);
    }
    out
}

/// File extensions are short and alphanumeric. The bound rejects the
/// surrounding plist keys (`CFBundleTypeRole`, `LSItemContentTypes`, …)
/// without needing to enumerate them.
fn is_extension_like(token: &str) -> bool {
    (1..=8).contains(&token.len())
        && token.chars().all(|c| c.is_ascii_alphanumeric())
        && token.chars().any(|c| c.is_ascii_alphabetic())
}

/// Display name for a bundle: its directory name minus `.app`.
fn bundle_name(bundle: &Path) -> String {
    bundle
        .file_stem()
        .map(|s| s.to_string_lossy().into_owned())
        .unwrap_or_default()
}

pub fn candidates(path: &Path) -> Result<Vec<AppHandler>> {
    let Some(ext) = path
        .extension()
        .and_then(|s| s.to_str())
        .map(str::to_ascii_lowercase)
    else {
        return Ok(Vec::new());
    };
    let mut out: Vec<AppHandler> = Vec::new();
    for dir in application_dirs() {
        let Ok(read) = std::fs::read_dir(&dir) else {
            continue;
        };
        for entry in read.flatten() {
            let bundle = entry.path();
            if bundle.extension().and_then(|s| s.to_str()) != Some("app") {
                continue;
            }
            let plist_path = bundle.join("Contents/Info.plist");
            let Ok(plist) = std::fs::read(&plist_path) else {
                continue;
            };
            if !declared_extensions(&plist).contains(&ext) {
                continue;
            }
            let id = bundle.to_string_lossy().into_owned();
            if out.iter().any(|h| h.id == id) {
                continue;
            }
            out.push(AppHandler {
                name: bundle_name(&bundle),
                id,
            });
            if out.len() >= 24 {
                return Ok(out);
            }
        }
    }
    out.sort_by_key(|a| a.name.to_lowercase());
    Ok(out)
}

pub fn launch(path: &Path, handler_id: &str) -> Result<()> {
    let mut cmd = std::process::Command::new("/usr/bin/open");
    cmd.arg("-a").arg(handler_id).arg(path);
    detach(&mut cmd);
    let status = cmd.status()?;
    if !status.success() {
        return Err(ShellError::Os(format!(
            "open -a exited with {}",
            status.code().unwrap_or(-1)
        )));
    }
    Ok(())
}

/// iTerm2 first when installed — a user who installed it wants it —
/// then the built-in Terminal.
pub fn open_terminal(dir: &Path) -> Result<()> {
    const TERMINALS: &[&str] = &[
        "/Applications/iTerm.app",
        "/System/Applications/Utilities/Terminal.app",
    ];
    for app in TERMINALS {
        if !Path::new(app).exists() {
            continue;
        }
        let mut cmd = std::process::Command::new("/usr/bin/open");
        cmd.arg("-a").arg(app).arg(dir);
        detach(&mut cmd);
        if cmd.status().is_ok_and(|s| s.success()) {
            return Ok(());
        }
    }
    Err(ShellError::NoTerminal {
        tried: TERMINALS.join(", "),
    })
}

/// Put file URLs on `NSPasteboard` via `osascript`. AppKit is the only
/// way to write the file-URL flavour Finder pastes as files, and going
/// through `osascript` keeps this module free of an Objective-C bridge.
pub fn copy_files(paths: &[PathBuf]) -> Result<()> {
    // AppleScript string literals escape `\` and `"` and nothing else.
    let list = paths
        .iter()
        .map(|p| format!("\"{}\"", applescript_escape(&p.to_string_lossy())))
        .collect::<Vec<_>>()
        .join(", ");
    let script = format!(
        "use framework \"AppKit\"\n\
         set pb to current application's NSPasteboard's generalPasteboard()\n\
         pb's clearContents()\n\
         set urls to {{}}\n\
         repeat with p in {{{list}}}\n\
         set end of urls to (current application's NSURL's fileURLWithPath:p)\n\
         end repeat\n\
         pb's writeObjects:urls\n"
    );
    let status = std::process::Command::new("/usr/bin/osascript")
        .arg("-e")
        .arg(script)
        .stdin(std::process::Stdio::null())
        .stdout(std::process::Stdio::null())
        .stderr(std::process::Stdio::null())
        .status()?;
    if !status.success() {
        return Err(ShellError::Os(
            "could not write file URLs to the pasteboard".into(),
        ));
    }
    Ok(())
}

fn applescript_escape(s: &str) -> String {
    s.replace('\\', "\\\\").replace('"', "\\\"")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn extensions_are_read_out_of_an_xml_plist() {
        let plist = br#"<?xml version="1.0"?>
<plist version="1.0"><dict>
  <key>CFBundleDocumentTypes</key>
  <array><dict>
    <key>CFBundleTypeExtensions</key>
    <array><string>pdf</string><string>PDF</string></array>
    <key>CFBundleTypeRole</key><string>Viewer</string>
  </dict></array>
</dict></plist>"#;
        let exts = declared_extensions(plist);
        assert!(exts.contains(&"pdf".to_string()), "got {exts:?}");
    }

    #[test]
    fn a_plist_without_the_marker_declares_nothing() {
        assert!(declared_extensions(b"<plist><dict></dict></plist>").is_empty());
    }

    #[test]
    fn long_camel_case_keys_are_not_mistaken_for_extensions() {
        assert!(!is_extension_like("CFBundleTypeRole"));
        assert!(!is_extension_like("LSItemContentTypes"));
        assert!(!is_extension_like("123"), "an extension has a letter in it");
        assert!(is_extension_like("pdf"));
        assert!(is_extension_like("m4a"));
    }

    #[test]
    fn applescript_literals_escape_quotes_and_backslashes() {
        assert_eq!(
            applescript_escape(r#"/vault/say "hi"\x"#),
            r#"/vault/say \"hi\"\\x"#
        );
    }

    #[test]
    fn bundle_name_strips_the_app_suffix() {
        assert_eq!(
            bundle_name(Path::new("/Applications/Preview.app")),
            "Preview"
        );
    }
}
