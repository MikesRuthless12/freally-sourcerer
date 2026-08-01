//! Result-row verbs added by Build 1 — SRC-M04 (Open with…),
//! SRC-M05 (advanced copy), SRC-M06 (terminal-here + custom commands).
//!
//! Every command here re-checks the path against the known-path
//! registry, exactly like `commands::files`: the daemon's result set is
//! the only source of paths this backend will act on.

use std::path::{Path, PathBuf};

use tauri::{AppHandle, State};
use tauri_plugin_clipboard_manager::ClipboardExt;

use super::files::MAX_CLIPBOARD_BYTES;
use super::known_paths::{KnownPaths, Provenance};
use super::settings::SettingsStore;
use crate::shell_actions::{self, AppHandler, CustomCommand};

// ---------- SRC-M04 ----------------------------------------------------

#[tauri::command]
pub async fn open_with_candidates(
    path: String,
    known: State<'_, KnownPaths>,
) -> Result<Vec<AppHandler>, String> {
    let p = known.verify(&path, Provenance::QueryHit)?;
    shell_actions::open_with_candidates(&p).map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn open_with(
    path: String,
    handler_id: String,
    known: State<'_, KnownPaths>,
) -> Result<(), String> {
    let p = known.verify(&path, Provenance::QueryHit)?;
    shell_actions::open_with(&p, &handler_id).map_err(|e| e.to_string())
}

// ---------- SRC-M05 ----------------------------------------------------

/// Copy a text file's *contents* to the clipboard. Binary files are
/// refused rather than pasted as replacement characters.
#[tauri::command]
pub async fn copy_file_contents(
    path: String,
    app: AppHandle,
    known: State<'_, KnownPaths>,
) -> Result<(), String> {
    let p = known.verify(&path, Provenance::QueryHit)?;
    let meta = std::fs::metadata(&p).map_err(|e| e.to_string())?;
    if meta.is_dir() {
        return Err("a folder has no contents to copy".into());
    }
    if meta.len() > MAX_CLIPBOARD_BYTES as u64 {
        return Err(format!(
            "file is {} MB — larger than the {} MB clipboard limit",
            meta.len() / (1024 * 1024),
            MAX_CLIPBOARD_BYTES / (1024 * 1024)
        ));
    }
    let bytes = std::fs::read(&p).map_err(|e| e.to_string())?;
    let text = String::from_utf8(bytes)
        .map_err(|_| "file is not UTF-8 text — use Copy as File instead".to_string())?;
    app.clipboard().write_text(text).map_err(|e| e.to_string())
}

/// Copy the files themselves, as OS clipboard file objects, so pasting
/// into the file manager copies the files.
#[tauri::command]
pub async fn copy_files_as_objects(
    paths: Vec<String>,
    known: State<'_, KnownPaths>,
) -> Result<(), String> {
    let verified = known.verify_all(&paths, Provenance::QueryHit)?;
    shell_actions::copy_files_to_clipboard(&verified).map_err(|e| e.to_string())
}

/// How a multi-selection path list is written to the clipboard.
#[derive(Debug, Clone, Copy, PartialEq, Eq, serde::Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum QuoteStyle {
    /// One path per line, unquoted. The default.
    Lines,
    /// One path per line, each wrapped in double quotes.
    Quoted,
    /// Space-separated on one line, quoted only where needed.
    SpaceSeparated,
    /// Space-separated on one line with POSIX backslash escaping.
    Escaped,
}

/// Render a path list in the requested style. Pure so the quoting rules
/// are testable without a clipboard.
pub fn format_path_list(paths: &[PathBuf], style: QuoteStyle) -> String {
    let render = |s: &str| -> String {
        match style {
            QuoteStyle::Lines => s.to_string(),
            QuoteStyle::Quoted => double_quote(s),
            QuoteStyle::SpaceSeparated if needs_quoting(s) => double_quote(s),
            QuoteStyle::SpaceSeparated => s.to_string(),
            QuoteStyle::Escaped => escape_posix(s),
        }
    };
    let separator = match style {
        QuoteStyle::Lines | QuoteStyle::Quoted => "\n",
        QuoteStyle::SpaceSeparated | QuoteStyle::Escaped => " ",
    };
    paths
        .iter()
        .map(|p| render(&p.to_string_lossy()))
        .collect::<Vec<_>>()
        .join(separator)
}

/// Wrap in double quotes, escaping the characters that stay special
/// inside them. `$` and `` ` `` are the ones that bite: an unescaped
/// `/vault/$USER report.txt` pastes into bash as a different path, and
/// backticks would execute.
fn double_quote(s: &str) -> String {
    let mut out = String::with_capacity(s.len() + 2);
    out.push('"');
    for c in s.chars() {
        if matches!(c, '"' | '\\' | '$' | '`') {
            out.push('\\');
        }
        out.push(c);
    }
    out.push('"');
    out
}

/// Characters that make a bare argument unsafe to paste. Whitespace is
/// not the only one — `/vault/a&b.txt` has none and still backgrounds
/// the command at the `&`.
fn needs_quoting(s: &str) -> bool {
    s.chars()
        .any(|c| c.is_whitespace() || " \"'\\$`&|;<>()*?[]{}~!#".contains(c))
}

/// POSIX single-quote wrapping: everything inside `'...'` is literal,
/// and an embedded `'` is closed, escaped, and reopened.
///
/// Backslash-escaping each metacharacter individually cannot express a
/// literal newline — in POSIX sh a backslash-newline is a *line
/// continuation*, so both characters vanish and a legal Linux filename
/// containing a newline pastes as a different path.
fn escape_posix(s: &str) -> String {
    let mut out = String::with_capacity(s.len() + 2);
    out.push('\'');
    for c in s.chars() {
        if c == '\'' {
            out.push_str("'\\''");
        } else {
            out.push(c);
        }
    }
    out.push('\'');
    out
}

#[tauri::command]
pub async fn copy_path_list(
    paths: Vec<String>,
    style: QuoteStyle,
    app: AppHandle,
    known: State<'_, KnownPaths>,
) -> Result<(), String> {
    let verified = known.verify_all(&paths, Provenance::QueryHit)?;
    let text = format_path_list(&verified, style);
    app.clipboard().write_text(text).map_err(|e| e.to_string())
}

// ---------- SRC-M06 ----------------------------------------------------

#[tauri::command]
pub async fn open_terminal_here(path: String, known: State<'_, KnownPaths>) -> Result<(), String> {
    let p = known.verify(&path, Provenance::QueryHit)?;
    shell_actions::open_terminal_here(&p).map_err(|e| e.to_string())
}

/// Run one of the user's configured commands against a result.
///
/// Takes the command's **id**, not its body. The set of programs this
/// can launch is therefore fixed by what the user saved in Settings —
/// resolved here from the backend's own persisted copy. Accepting the
/// `program` field over IPC would make this an arbitrary-process-
/// execution primitive: argv-safety is no protection when the caller
/// also chooses the executable, and `/bin/sh -c …` is one call away.
#[tauri::command]
pub async fn run_custom_command(
    command_id: String,
    path: String,
    known: State<'_, KnownPaths>,
    settings: State<'_, SettingsStore>,
) -> Result<(), String> {
    let p = known.verify(&path, Provenance::QueryHit)?;
    let command = configured_command(&settings, &command_id)
        .ok_or_else(|| "no such custom command".to_string())?;
    if command.program.trim().is_empty() {
        return Err("custom command has no program set".into());
    }
    if !command.applies_to(Path::new(&path)) {
        return Err(format!(
            "`{}` is not configured for this file type",
            command.name
        ));
    }
    shell_actions::run_custom_command(&command, &p).map_err(|e| e.to_string())
}

/// Look one command up in the persisted settings. Returns `None` for an
/// unknown id, and for a `custom_commands` value that has been
/// hand-edited into a shape we can't read.
fn configured_command(settings: &SettingsStore, id: &str) -> Option<CustomCommand> {
    let raw = settings
        .state
        .lock()
        .ok()?
        .extras
        .get("custom_commands")
        .cloned()?;
    serde_json::from_value::<Vec<CustomCommand>>(raw)
        .ok()?
        .into_iter()
        .find(|c| c.id == id)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn paths() -> Vec<PathBuf> {
        vec![
            PathBuf::from("/vault/plain.txt"),
            PathBuf::from("/vault/two words.txt"),
        ]
    }

    #[test]
    fn lines_style_is_bare_paths() {
        assert_eq!(
            format_path_list(&paths(), QuoteStyle::Lines),
            "/vault/plain.txt\n/vault/two words.txt"
        );
    }

    #[test]
    fn quoted_style_wraps_every_path() {
        assert_eq!(
            format_path_list(&paths(), QuoteStyle::Quoted),
            "\"/vault/plain.txt\"\n\"/vault/two words.txt\""
        );
    }

    #[test]
    fn space_separated_quotes_only_what_needs_it() {
        assert_eq!(
            format_path_list(&paths(), QuoteStyle::SpaceSeparated),
            "/vault/plain.txt \"/vault/two words.txt\""
        );
    }

    #[test]
    fn escaped_style_neutralises_shell_metacharacters() {
        let p = vec![PathBuf::from("/vault/a b&c$(x).txt")];
        assert_eq!(
            format_path_list(&p, QuoteStyle::Escaped),
            "'/vault/a b&c$(x).txt'"
        );
    }

    #[test]
    fn escaped_style_survives_an_embedded_newline_and_quote() {
        // Backslash-newline is a line continuation in POSIX sh, so the
        // per-character escape used to delete the newline outright.
        let p = vec![PathBuf::from("/vault/we\nird's.txt")];
        assert_eq!(
            format_path_list(&p, QuoteStyle::Escaped),
            "'/vault/we\nird'\\''s.txt'"
        );
    }

    #[test]
    fn quoted_style_neutralises_dollar_and_backtick() {
        let p = vec![PathBuf::from("/vault/$USER `id`.txt")];
        assert_eq!(
            format_path_list(&p, QuoteStyle::Quoted),
            "\"/vault/\\$USER \\`id\\`.txt\""
        );
    }

    #[test]
    fn space_separated_quotes_metacharacters_not_just_whitespace() {
        let p = vec![PathBuf::from("/vault/a&b.txt")];
        assert_eq!(
            format_path_list(&p, QuoteStyle::SpaceSeparated),
            "\"/vault/a&b.txt\"",
            "a bare & would background the command at paste time"
        );
    }

    #[test]
    fn embedded_quotes_are_escaped_not_dropped() {
        let p = vec![PathBuf::from("/vault/say \"hi\".txt")];
        assert_eq!(
            format_path_list(&p, QuoteStyle::Quoted),
            "\"/vault/say \\\"hi\\\".txt\""
        );
    }
}
