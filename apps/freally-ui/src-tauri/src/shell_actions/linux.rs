//! Linux shell integration, XDG-only.
//!
//! Handler discovery reads the desktop-entry spec directly: walk
//! `$XDG_DATA_HOME` + `$XDG_DATA_DIRS`, parse each `applications/*.desktop`,
//! and keep the ones whose `MimeType=` mentions the file's type. That
//! is exactly what a desktop environment does, needs no D-Bus, and
//! works the same on GNOME, KDE, Sway, and a bare WM.
//!
//! MIME type comes from the extension via the shared MIME database's
//! `globs` file when it is present, falling back to a small built-in
//! table so a container without `shared-mime-info` still lists handlers.

use std::collections::BTreeMap;
use std::path::{Path, PathBuf};

use super::{AppHandler, Result, ShellError, detach};

/// Enough of the shared MIME database to cover the types people search
/// for, for systems that ship without `shared-mime-info`.
const FALLBACK_MIME: &[(&str, &str)] = &[
    ("txt", "text/plain"),
    ("md", "text/markdown"),
    ("pdf", "application/pdf"),
    ("png", "image/png"),
    ("jpg", "image/jpeg"),
    ("jpeg", "image/jpeg"),
    ("gif", "image/gif"),
    ("webp", "image/webp"),
    ("svg", "image/svg+xml"),
    ("mp3", "audio/mpeg"),
    ("flac", "audio/flac"),
    ("wav", "audio/x-wav"),
    ("ogg", "audio/ogg"),
    ("opus", "audio/opus"),
    ("mp4", "video/mp4"),
    ("mkv", "video/x-matroska"),
    ("zip", "application/zip"),
    ("html", "text/html"),
    ("json", "application/json"),
    ("csv", "text/csv"),
];

fn data_dirs() -> Vec<PathBuf> {
    let mut dirs = Vec::new();
    if let Ok(home) = std::env::var("XDG_DATA_HOME") {
        dirs.push(PathBuf::from(home));
    } else if let Ok(home) = std::env::var("HOME") {
        dirs.push(PathBuf::from(home).join(".local/share"));
    }
    let system =
        std::env::var("XDG_DATA_DIRS").unwrap_or_else(|_| "/usr/local/share:/usr/share".into());
    dirs.extend(
        system
            .split(':')
            .filter(|s| !s.is_empty())
            .map(PathBuf::from),
    );
    dirs
}

/// MIME type for a path's extension.
fn mime_for(path: &Path) -> Option<String> {
    let ext = path.extension()?.to_str()?.to_ascii_lowercase();
    for dir in data_dirs() {
        let globs = dir.join("mime/globs");
        let Ok(text) = std::fs::read_to_string(&globs) else {
            continue;
        };
        for line in text.lines() {
            // Format is `mime/type:*.ext`.
            let Some((mime, pattern)) = line.split_once(':') else {
                continue;
            };
            if line.starts_with('#') {
                continue;
            }
            if pattern
                .strip_prefix("*.")
                .is_some_and(|p| p.eq_ignore_ascii_case(&ext))
            {
                return Some(mime.to_string());
            }
        }
    }
    FALLBACK_MIME
        .iter()
        .find(|(e, _)| *e == ext)
        .map(|(_, m)| (*m).to_string())
}

/// The fields we need out of a `.desktop` entry.
struct DesktopEntry {
    name: String,
    exec: String,
    mime_types: Vec<String>,
    no_display: bool,
    terminal: bool,
}

/// Parse the `[Desktop Entry]` group. Later groups (`[Desktop Action …]`)
/// are ignored — their `Exec` is for a secondary action, not for opening
/// a file.
fn parse_desktop(text: &str) -> Option<DesktopEntry> {
    let mut in_main = false;
    let mut fields: BTreeMap<&str, &str> = BTreeMap::new();
    for line in text.lines() {
        let line = line.trim();
        if line.starts_with('[') {
            in_main = line == "[Desktop Entry]";
            continue;
        }
        if !in_main || line.is_empty() || line.starts_with('#') {
            continue;
        }
        if let Some((k, v)) = line.split_once('=') {
            // Localised keys look like `Name[de]`; the unlocalised key
            // wins because we have no locale negotiation here.
            if !k.contains('[') {
                fields.insert(k.trim(), v.trim());
            }
        }
    }
    if fields.get("Type").copied().unwrap_or("Application") != "Application" {
        return None;
    }
    let exec = (*fields.get("Exec")?).to_string();
    Some(DesktopEntry {
        name: fields.get("Name").copied().unwrap_or("").to_string(),
        exec,
        mime_types: fields
            .get("MimeType")
            .copied()
            .unwrap_or("")
            .split(';')
            .filter(|s| !s.is_empty())
            .map(str::to_string)
            .collect(),
        no_display: fields.get("NoDisplay").copied() == Some("true"),
        terminal: fields.get("Terminal").copied() == Some("true"),
    })
}

fn find_desktop_file(id: &str) -> Option<DesktopEntry> {
    // A desktop-file id is its basename; reject anything with a
    // separator so a caller cannot walk out of the applications dirs.
    if id.contains('/') || id.contains('\\') || !id.ends_with(".desktop") {
        return None;
    }
    for dir in data_dirs() {
        let candidate = dir.join("applications").join(id);
        if let Ok(text) = std::fs::read_to_string(&candidate)
            && let Some(entry) = parse_desktop(&text)
        {
            return Some(entry);
        }
    }
    None
}

pub fn candidates(path: &Path) -> Result<Vec<AppHandler>> {
    let Some(mime) = mime_for(path) else {
        return Ok(Vec::new());
    };
    let mut out: Vec<AppHandler> = Vec::new();
    let mut seen: Vec<String> = Vec::new();
    for dir in data_dirs() {
        let apps = dir.join("applications");
        let Ok(read) = std::fs::read_dir(&apps) else {
            continue;
        };
        for entry in read.flatten() {
            let file_name = entry.file_name();
            let Some(id) = file_name.to_str() else {
                continue;
            };
            if !id.ends_with(".desktop") || seen.iter().any(|s| s == id) {
                continue;
            }
            let Ok(text) = std::fs::read_to_string(entry.path()) else {
                continue;
            };
            let Some(desktop) = parse_desktop(&text) else {
                continue;
            };
            if desktop.no_display || !desktop.mime_types.iter().any(|m| m == &mime) {
                continue;
            }
            seen.push(id.to_string());
            out.push(AppHandler {
                id: id.to_string(),
                name: if desktop.name.is_empty() {
                    id.trim_end_matches(".desktop").to_string()
                } else {
                    desktop.name
                },
            });
            if out.len() >= 24 {
                return Ok(out);
            }
        }
    }
    out.sort_by_key(|a| a.name.to_lowercase());
    Ok(out)
}

/// Turn a desktop-entry `Exec=` line into a program plus arguments.
///
/// Field codes are substituted per the desktop-entry spec: `%f`/`%u`
/// take the file, `%F`/`%U` take the (single-element) file list, and
/// the deprecated `%d %D %n %N %v %m` are dropped. Anything else is
/// passed through. Splitting is on whitespace outside quotes — the spec
/// permits quoted arguments and some entries use them.
fn exec_to_command(exec: &str, path: &Path) -> (String, Vec<String>) {
    let mut tokens: Vec<String> = Vec::new();
    let mut current = String::new();
    let mut quote: Option<char> = None;
    for c in exec.chars() {
        match (quote, c) {
            (Some(q), ch) if ch == q => quote = None,
            (Some(_), ch) => current.push(ch),
            (None, '"') | (None, '\'') => quote = Some(c),
            (None, ch) if ch.is_whitespace() => {
                if !current.is_empty() {
                    tokens.push(std::mem::take(&mut current));
                }
            }
            (None, ch) => current.push(ch),
        }
    }
    if !current.is_empty() {
        tokens.push(current);
    }

    let file = path.to_string_lossy().into_owned();
    let mut args: Vec<String> = Vec::new();
    let mut took_file = false;
    for token in tokens.iter().skip(1) {
        match token.as_str() {
            "%f" | "%F" | "%u" | "%U" => {
                args.push(file.clone());
                took_file = true;
            }
            "%d" | "%D" | "%n" | "%N" | "%v" | "%m" | "%i" | "%c" | "%k" => {}
            // A field code can also sit *inside* a token —
            // `Exec=myapp --input=%f` is common. Substituting only
            // standalone tokens would hand the app a literal `%f` and
            // then append the path again as a stray positional.
            other if other.contains('%') => {
                let (expanded, consumed) = expand_field_codes(other, &file);
                took_file |= consumed;
                args.push(expanded);
            }
            other => args.push(other.to_string()),
        }
    }
    if !took_file {
        args.push(file);
    }
    (tokens.first().cloned().unwrap_or_default(), args)
}

/// Substitute desktop-entry field codes inside one argument. Returns the
/// expanded string and whether it consumed the file. `%%` is the spec's
/// escape for a literal `%`.
fn expand_field_codes(token: &str, file: &str) -> (String, bool) {
    let mut out = String::with_capacity(token.len());
    let mut took_file = false;
    let mut chars = token.chars();
    while let Some(c) = chars.next() {
        if c != '%' {
            out.push(c);
            continue;
        }
        match chars.next() {
            Some('%') => out.push('%'),
            Some('f' | 'F' | 'u' | 'U') => {
                out.push_str(file);
                took_file = true;
            }
            // Deprecated codes expand to nothing, per the spec.
            Some('d' | 'D' | 'n' | 'N' | 'v' | 'm' | 'i' | 'c' | 'k') => {}
            Some(other) => {
                out.push('%');
                out.push(other);
            }
            None => out.push('%'),
        }
    }
    (out, took_file)
}

pub fn launch(path: &Path, handler_id: &str) -> Result<()> {
    let entry = find_desktop_file(handler_id)
        .ok_or_else(|| ShellError::UnknownHandler(handler_id.to_string()))?;
    let (program, args) = exec_to_command(&entry.exec, path);
    if program.is_empty() {
        return Err(ShellError::Os(format!("{handler_id} has an empty Exec=")));
    }
    if entry.terminal {
        // A Terminal=true entry expects a terminal to host it. Reuse the
        // same emulator search the "open terminal here" verb uses.
        return spawn_in_terminal(&program, &args, path);
    }
    let mut cmd = std::process::Command::new(&program);
    cmd.args(&args);
    detach(&mut cmd);
    cmd.spawn()?;
    Ok(())
}

/// Terminal emulators in preference order. `x-terminal-emulator` is the
/// Debian alternatives symlink and comes first so a user's own choice
/// wins over our list.
const TERMINALS: &[&str] = &[
    "x-terminal-emulator",
    "gnome-terminal",
    "konsole",
    "xfce4-terminal",
    "alacritty",
    "kitty",
    "wezterm",
    "foot",
    "xterm",
];

pub fn open_terminal(dir: &Path) -> Result<()> {
    spawn_terminal(dir, None)
}

fn spawn_in_terminal(program: &str, args: &[String], path: &Path) -> Result<()> {
    let dir = super::working_dir_for(path)?;
    spawn_terminal(&dir, Some((program, args)))
}

/// Launch the first emulator that starts, optionally hosting a command
/// (`-e <program> <args...>`) rather than an interactive shell.
fn spawn_terminal(dir: &Path, hosting: Option<(&str, &[String])>) -> Result<()> {
    for terminal in TERMINALS {
        let mut cmd = std::process::Command::new(terminal);
        if let Some((program, args)) = hosting {
            cmd.arg("-e").arg(program).args(args);
        }
        cmd.current_dir(dir);
        detach(&mut cmd);
        if cmd.spawn().is_ok() {
            return Ok(());
        }
    }
    Err(ShellError::NoTerminal {
        tried: TERMINALS.join(", "),
    })
}

/// GNOME/KDE-compatible file copy. `x-special/gnome-copied-files` is
/// what Nautilus, Nemo, Caja, and Dolphin all read; `text/uri-list` is
/// the fallback every other consumer understands.
///
/// Both go on the clipboard via `wl-copy` (Wayland) or `xclip` (X11).
/// There is no in-process clipboard API that survives this process
/// exiting on Linux — the clipboard is owned by a live client — so
/// handing ownership to a purpose-built helper is the correct shape,
/// not a shortcut.
pub fn copy_files(paths: &[PathBuf]) -> Result<()> {
    let uris: Vec<String> = paths
        .iter()
        .map(|p| format!("file://{}", percent_encode_path(&p.to_string_lossy())))
        .collect();
    // `x-special/gnome-copied-files` is what Nautilus, Nemo, Caja, and
    // Dolphin read; `text/uri-list` is the fallback everything else
    // understands. Try the richer flavour first on each helper.
    let payloads = [
        (
            "x-special/gnome-copied-files",
            format!(
                "copy
{}",
                uris.join(
                    "
"
                )
            ),
        ),
        (
            "text/uri-list",
            format!(
                "{}
",
                uris.join(
                    "
"
                )
            ),
        ),
    ];
    for helper in CLIPBOARD_HELPERS {
        for (mime, payload) in &payloads {
            if write_to_stdin(helper.program, &helper.args(mime), payload).is_ok() {
                return Ok(());
            }
        }
    }
    Err(ShellError::Os(
        "no clipboard helper found — install wl-clipboard (Wayland) or xclip (X11)".into(),
    ))
}

/// A clipboard helper and how it takes a MIME type. There is no
/// in-process clipboard on Linux that survives the writing process
/// exiting — the selection is owned by a live client — so handing
/// ownership to a purpose-built helper is the correct shape, not a
/// shortcut.
struct ClipboardHelper {
    program: &'static str,
    /// Arguments before the `--type`/`-t` flag, and the flag itself.
    prefix: &'static [&'static str],
    type_flag: &'static str,
}

impl ClipboardHelper {
    fn args(&self, mime: &str) -> Vec<String> {
        let mut out: Vec<String> = self.prefix.iter().map(|s| (*s).to_string()).collect();
        out.push(self.type_flag.to_string());
        out.push(mime.to_string());
        out
    }
}

const CLIPBOARD_HELPERS: &[ClipboardHelper] = &[
    ClipboardHelper {
        program: "wl-copy",
        prefix: &[],
        type_flag: "--type",
    },
    ClipboardHelper {
        program: "xclip",
        prefix: &["-selection", "clipboard"],
        type_flag: "-t",
    },
];

/// Feed `payload` to a clipboard helper and report whether it actually
/// took it.
///
/// Both helpers fork and let the parent exit as soon as they own the
/// selection, so waiting is fast and cannot hang. Not waiting was a bug:
/// `wl-copy` on an X11 session exits non-zero after failing to reach a
/// Wayland display, and skipping the status made that look like success
/// — the toast said "copied", the clipboard was untouched, and `xclip`
/// was never tried. It also leaked a zombie per attempt.
fn write_to_stdin(program: &str, args: &[String], payload: &str) -> std::io::Result<()> {
    use std::io::Write;
    let mut child = std::process::Command::new(program)
        .args(args)
        .stdin(std::process::Stdio::piped())
        .stdout(std::process::Stdio::null())
        .stderr(std::process::Stdio::null())
        .spawn()?;
    if let Some(stdin) = child.stdin.as_mut() {
        stdin.write_all(payload.as_bytes())?;
    }
    drop(child.stdin.take());
    let status = child.wait()?;
    if status.success() {
        Ok(())
    } else {
        Err(std::io::Error::other(format!(
            "{program} exited with {}",
            status.code().unwrap_or(-1)
        )))
    }
}

/// Percent-encode the bytes a `file://` URI may not carry literally.
/// Deliberately conservative: everything outside the unreserved set
/// plus `/` is escaped.
fn percent_encode_path(path: &str) -> String {
    let mut out = String::with_capacity(path.len());
    for b in path.bytes() {
        match b {
            b'A'..=b'Z' | b'a'..=b'z' | b'0'..=b'9' | b'-' | b'.' | b'_' | b'~' | b'/' => {
                out.push(b as char)
            }
            _ => out.push_str(&format!("%{b:02X}")),
        }
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn desktop_entry_parses_the_main_group_only() {
        let text = "\
[Desktop Entry]
Type=Application
Name=Text Editor
Name[de]=Texteditor
Exec=gedit %U
MimeType=text/plain;text/markdown;
Terminal=false

[Desktop Action new-window]
Exec=gedit --new-window
";
        let e = parse_desktop(text).unwrap();
        assert_eq!(e.name, "Text Editor", "localised keys must not win");
        assert_eq!(e.exec, "gedit %U");
        assert_eq!(e.mime_types, vec!["text/plain", "text/markdown"]);
        assert!(!e.terminal);
        assert!(!e.no_display);
    }

    #[test]
    fn non_application_entries_are_skipped() {
        assert!(parse_desktop("[Desktop Entry]\nType=Link\nURL=x\n").is_none());
        assert!(
            parse_desktop("[Desktop Entry]\nType=Application\n").is_none(),
            "an entry with no Exec cannot launch anything"
        );
    }

    #[test]
    fn exec_field_codes_expand_to_the_file() {
        let path = Path::new("/vault/a b.txt");
        let (p, a) = exec_to_command("gedit --new %U", path);
        assert_eq!(p, "gedit");
        assert_eq!(a, vec!["--new", "/vault/a b.txt"]);
    }

    #[test]
    fn exec_without_a_field_code_still_gets_the_file() {
        let (p, a) = exec_to_command("mousepad", Path::new("/x.txt"));
        assert_eq!(p, "mousepad");
        assert_eq!(a, vec!["/x.txt"]);
    }

    #[test]
    fn exec_drops_deprecated_field_codes() {
        let (_, a) = exec_to_command("app %i %c %f", Path::new("/x.txt"));
        assert_eq!(a, vec!["/x.txt"]);
    }

    #[test]
    fn exec_honours_quoted_arguments() {
        let (p, a) = exec_to_command(
            "\"/opt/My App/run\" --flag \"two words\" %f",
            Path::new("/x"),
        );
        assert_eq!(p, "/opt/My App/run");
        assert_eq!(a, vec!["--flag", "two words", "/x"]);
    }

    #[test]
    fn desktop_ids_cannot_escape_the_applications_directory() {
        assert!(find_desktop_file("../../etc/passwd").is_none());
        assert!(find_desktop_file("evil").is_none(), "must end in .desktop");
    }

    #[test]
    fn uris_percent_encode_spaces_and_specials() {
        assert_eq!(
            percent_encode_path("/vault/a b&c.txt"),
            "/vault/a%20b%26c.txt"
        );
        assert_eq!(percent_encode_path("/plain/path"), "/plain/path");
    }
}
