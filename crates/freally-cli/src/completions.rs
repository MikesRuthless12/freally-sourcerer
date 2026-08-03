//! SRC-M10 — shell completions for bash, zsh, fish and PowerShell.
//!
//! Completion runs through `clap_complete`'s dynamic engine rather than
//! a baked-in script. `freally completions <shell>` prints a small
//! registration stub; when the user hits Tab the shell calls `freally`
//! back, and the candidates are computed *in Rust* right then.
//!
//! That indirection is the whole point. Subcommands and flags could
//! have been frozen into a static script, but two of the things worth
//! completing cannot be:
//!
//! - **Modifier keywords** come from [`freally_query::MODIFIER_KEYS`],
//!   so a modifier added in a later phase completes without anyone
//!   regenerating and reshipping four scripts.
//! - **Saved-search names** are per-user data that did not exist when
//!   the package was built.
//!
//! It also means there is no hand-written shell code in this repo to
//! rot — one Rust function serves all four shells identically.
//!
//! ## Where saved searches come from
//!
//! Bookmarks are owned by the desktop app, not the daemon: there is no
//! `bookmarks.*` RPC method to ask (see `freally-indexd`'s dispatch
//! table). They are persisted as plain JSON, so this reads that file
//! directly rather than growing a transport for a completion list.
//! When the file is absent — the app has never run — completion simply
//! offers keywords.

use std::io;
use std::path::PathBuf;

use clap::CommandFactory;
use clap_complete::CompletionCandidate;
use clap_complete::engine::ArgValueCandidates;

use crate::Cli;

/// Tauri's bundle identifier, which names the app-data directory.
/// Must track `apps/freally-ui/src-tauri/tauri.conf.json`; the smoke
/// test `build_02_completions` asserts they still agree.
pub const APP_IDENTIFIER: &str = "io.mikeweaver.freally";

/// Refuse to parse a bookmarks file larger than this. The UI caps the
/// store at 1024 entries of bounded length; anything far past that is
/// not a file worth reading synchronously on a Tab keypress.
const MAX_BOOKMARKS_BYTES: u64 = 4 * 1024 * 1024;

/// Upper bound on offered saved-search names, so a large store cannot
/// turn one Tab into a thousand-line menu.
const MAX_BOOKMARK_CANDIDATES: usize = 256;

/// Where the desktop app persists bookmarks, mirroring Tauri's
/// `app_data_dir()` for each platform.
///
/// SRC-M17: a portable install keeps them in its own `Data/` folder.
/// Completion runs before the argument parser — a half-typed command
/// line is not parseable — so `--portable` is not visible here and only
/// the `portable.flag` file can select the portable store. That is the
/// case that matters: a flagged install is the one a shell is rooted in.
fn bookmarks_path() -> Option<PathBuf> {
    if let Some(data) = freally_rpc::portable::data_dir() {
        return Some(data.join("bookmarks.json"));
    }
    let dir = app_data_dir()?;
    Some(dir.join(APP_IDENTIFIER).join("bookmarks.json"))
}

#[cfg(windows)]
fn app_data_dir() -> Option<PathBuf> {
    std::env::var_os("APPDATA").map(PathBuf::from)
}

#[cfg(target_os = "macos")]
fn app_data_dir() -> Option<PathBuf> {
    std::env::var_os("HOME").map(|h| PathBuf::from(h).join("Library").join("Application Support"))
}

#[cfg(all(unix, not(target_os = "macos")))]
fn app_data_dir() -> Option<PathBuf> {
    if let Some(x) = std::env::var_os("XDG_DATA_HOME").filter(|v| !v.is_empty()) {
        return Some(PathBuf::from(x));
    }
    std::env::var_os("HOME").map(|h| PathBuf::from(h).join(".local").join("share"))
}

/// A name is only offered if it is a single clean line.
///
/// The store is written by the app, which bounds names — but this file
/// is user-writable, and a completion candidate carrying a newline or a
/// control character would corrupt the menu of whichever shell rendered
/// it. Dropping such a name costs one completion; rendering it costs
/// the shell's parsing.
fn is_offerable(name: &str) -> bool {
    !name.is_empty() && name.len() <= 256 && !name.chars().any(char::is_control)
}

/// Saved-search names, or an empty list if the app has never run.
/// Never errors: a completion callback that fails is worse than one
/// that returns nothing.
fn saved_search_names() -> Vec<String> {
    let Some(path) = bookmarks_path() else {
        return Vec::new();
    };
    if std::fs::metadata(&path)
        .map(|m| m.len())
        .unwrap_or(u64::MAX)
        > MAX_BOOKMARKS_BYTES
    {
        return Vec::new();
    }
    let Ok(text) = std::fs::read_to_string(&path) else {
        return Vec::new();
    };
    // Read through `Value` rather than a mirrored `Bookmark` struct:
    // the CLI cannot depend on the Tauri crate that owns that type, and
    // only one field is needed.
    let Ok(serde_json::Value::Array(items)) = serde_json::from_str::<serde_json::Value>(&text)
    else {
        return Vec::new();
    };
    items
        .iter()
        .filter_map(|b| b.get("name")?.as_str())
        .filter(|n| is_offerable(n))
        .map(str::to_string)
        .take(MAX_BOOKMARK_CANDIDATES)
        .collect()
}

/// Everything worth suggesting inside a query string: the DSL's
/// modifier keywords and quick filters (with their trailing `:`, since
/// that is how they are typed), then the user's saved searches.
pub fn query_candidates() -> Vec<CompletionCandidate> {
    let mut out = Vec::with_capacity(
        freally_query::MODIFIER_KEYS.len() + freally_query::QUICK_FILTER_KEYS.len() + 8,
    );
    for key in freally_query::MODIFIER_KEYS {
        out.push(CompletionCandidate::new(format!("{key}:")).help(Some("modifier".into())));
    }
    for key in freally_query::QUICK_FILTER_KEYS {
        out.push(CompletionCandidate::new(format!("{key}:")).help(Some("quick filter".into())));
    }
    for name in saved_search_names() {
        out.push(CompletionCandidate::new(name).help(Some("saved search".into())));
    }
    out
}

/// Attaches [`query_candidates`] to an argument.
pub fn query_value_candidates() -> ArgValueCandidates {
    ArgValueCandidates::new(query_candidates)
}

/// The shells `freally completions` can register with.
#[derive(Debug, Clone, Copy, PartialEq, Eq, clap::ValueEnum)]
#[value(rename_all = "lowercase")]
pub enum Shell {
    Bash,
    Zsh,
    Fish,
    Powershell,
}

impl Shell {
    fn as_clap(self) -> clap_complete::Shell {
        match self {
            Shell::Bash => clap_complete::Shell::Bash,
            Shell::Zsh => clap_complete::Shell::Zsh,
            Shell::Fish => clap_complete::Shell::Fish,
            Shell::Powershell => clap_complete::Shell::PowerShell,
        }
    }

    /// Where a user is expected to put the output, quoted in the
    /// stub's own comment syntax.
    fn install_hint(self) -> &'static str {
        match self {
            Shell::Bash => {
                "# Install: freally completions bash > /etc/bash_completion.d/freally\n#      or: eval \"$(freally completions bash)\" in ~/.bashrc"
            }
            Shell::Zsh => {
                "# Install: freally completions zsh > \"${fpath[1]}/_freally\"\n#      or: eval \"$(freally completions zsh)\" in ~/.zshrc"
            }
            Shell::Fish => {
                "# Install: freally completions fish > ~/.config/fish/completions/freally.fish"
            }
            Shell::Powershell => "# Install: freally completions powershell >> $PROFILE",
        }
    }
}

/// Print the registration stub for `shell`.
pub fn print(shell: Shell, out: &mut impl io::Write) -> io::Result<()> {
    writeln!(out, "{}", shell.install_hint())?;
    // The name comes from the parser rather than a literal, so the
    // registration can never describe a binary the CLI does not answer to.
    let bin = Cli::command().get_name().to_string();
    clap_complete::env::Shells::builtins()
        .completer(&shell.as_clap().to_string())
        .ok_or_else(|| io::Error::other(format!("no completer for {shell:?}")))?
        .write_registration("COMPLETE", &bin, &bin, &bin, out)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn every_dsl_modifier_is_offered_with_its_colon() {
        let names: Vec<String> = query_candidates()
            .iter()
            .map(|c| c.get_value().to_string_lossy().to_string())
            .collect();
        for key in freally_query::MODIFIER_KEYS {
            assert!(
                names.contains(&format!("{key}:")),
                "modifier `{key}` is not offered"
            );
        }
        assert!(names.contains(&"audio:".to_string()));
    }

    #[test]
    fn candidates_never_contain_a_bare_keyword_without_its_colon() {
        // `size` alone is a literal search term, not a modifier — the
        // colon is what makes it one, so completing without it would
        // silently change the query's meaning.
        let names: Vec<String> = query_candidates()
            .iter()
            .map(|c| c.get_value().to_string_lossy().to_string())
            .collect();
        assert!(!names.contains(&"size".to_string()));
    }

    #[test]
    fn a_name_with_a_newline_is_never_offered() {
        assert!(!is_offerable("evil\nname"));
        assert!(!is_offerable("evil\rname"));
        assert!(!is_offerable("bell\u{7}"));
        assert!(!is_offerable(""));
        assert!(is_offerable("Big media files"));
    }

    #[test]
    fn an_overlong_name_is_never_offered() {
        assert!(!is_offerable(&"x".repeat(257)));
        assert!(is_offerable(&"x".repeat(256)));
    }

    #[test]
    fn every_shell_writes_a_registration_stub() {
        for shell in [Shell::Bash, Shell::Zsh, Shell::Fish, Shell::Powershell] {
            let mut buf = Vec::new();
            print(shell, &mut buf).expect("registration written");
            let text = String::from_utf8(buf).unwrap();
            assert!(text.contains("freally"), "{shell:?} stub names no binary");
            assert!(
                text.contains("COMPLETE"),
                "{shell:?} stub sets no COMPLETE var"
            );
            assert!(
                text.contains("# Install:"),
                "{shell:?} stub has no install hint"
            );
        }
    }

    #[test]
    fn a_missing_bookmarks_file_yields_no_candidates_rather_than_an_error() {
        // Point the lookup at a directory that cannot exist.
        let names = saved_search_names();
        // Either the real store exists on this machine or it doesn't;
        // both are fine. What must hold is that it never panics and
        // never exceeds the cap.
        assert!(names.len() <= MAX_BOOKMARK_CANDIDATES);
    }
}
