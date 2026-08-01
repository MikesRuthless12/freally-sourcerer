//! Per-OS shell integration for Build 1's result verbs.
//!
//! Three features share this module because they share one problem —
//! "what can this OS do with this file, and how do I ask it to":
//!
//! * **SRC-M04** — enumerate the applications registered for a file
//!   type and launch one of them.
//! * **SRC-M05** — put a file's *contents*, or the file itself, or a
//!   list of paths, on the OS clipboard.
//! * **SRC-M06** — open a terminal in a file's folder, and run
//!   user-defined commands against a result.
//!
//! Every launch path here spawns a local process and nothing else: no
//! shell interpretation of user data, no network, no elevation.

use std::path::{Path, PathBuf};

#[cfg(target_os = "linux")]
mod linux;
#[cfg(target_os = "macos")]
mod macos;
#[cfg(windows)]
mod windows;

// One alias per target so every entry point below is a single
// delegation rather than a four-arm `#[cfg]` cascade — whose last arm
// could never be compiled for a platform this app ships to, and so was
// never tested.
#[cfg(target_os = "linux")]
use linux as sys;
#[cfg(target_os = "macos")]
use macos as sys;
#[cfg(windows)]
use windows as sys;

/// One application the OS says can open a given file.
#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub struct AppHandler {
    /// Opaque, OS-specific launch token. Windows: the executable path
    /// from the registered verb. macOS: the `.app` bundle path. Linux:
    /// the `.desktop` file's basename. Never shown to the user.
    pub id: String,
    /// Human-readable name for the menu.
    pub name: String,
}

#[derive(Debug, thiserror::Error)]
pub enum ShellError {
    #[error("{0}")]
    Os(String),
    #[error("no application is registered for this file type")]
    NoHandlers,
    #[error("`{0}` is not a known handler for this file")]
    UnknownHandler(String),
    #[error("path has no parent directory")]
    NoParent,
    #[error("no terminal emulator found — tried: {tried}")]
    NoTerminal { tried: String },
    #[error("io: {0}")]
    Io(#[from] std::io::Error),
}

pub type Result<T> = std::result::Result<T, ShellError>;

/// Applications registered to open `path`, in the OS's own preference
/// order. An empty list is not an error — plenty of extensions have no
/// registered handler at all.
pub fn open_with_candidates(path: &Path) -> Result<Vec<AppHandler>> {
    sys::candidates(path)
}

/// Launch `path` with the handler identified by `handler_id`. The id
/// must have come from [`open_with_candidates`] for this same file —
/// we re-enumerate and verify rather than trusting the caller, so a
/// compromised UI layer cannot turn this into "run arbitrary binary".
pub fn open_with(path: &Path, handler_id: &str) -> Result<()> {
    let known = open_with_candidates(path)?;
    if !known.iter().any(|h| h.id == handler_id) {
        return Err(ShellError::UnknownHandler(handler_id.to_string()));
    }
    sys::launch(path, handler_id)
}

/// Open the platform's terminal with its working directory set to
/// `path`'s folder (or to `path` itself when it is a directory).
pub fn open_terminal_here(path: &Path) -> Result<()> {
    sys::open_terminal(&working_dir_for(path)?)
}

/// The directory a "here"-style action should act in: the file's parent,
/// or the directory itself.
pub fn working_dir_for(path: &Path) -> Result<PathBuf> {
    if path.is_dir() {
        return Ok(path.to_path_buf());
    }
    path.parent()
        .map(Path::to_path_buf)
        .ok_or(ShellError::NoParent)
}

/// Put `paths` on the clipboard as real file objects, so pasting into
/// the OS file manager copies the files themselves rather than their
/// names.
pub fn copy_files_to_clipboard(paths: &[PathBuf]) -> Result<()> {
    if paths.is_empty() {
        return Ok(());
    }
    sys::copy_files(paths)
}

// ---------- custom commands (SRC-M06) ----------------------------------

/// A user-defined command. `program` is an executable path or name;
/// `args` are templates expanded per-invocation.
#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub struct CustomCommand {
    pub id: String,
    pub name: String,
    pub program: String,
    #[serde(default)]
    pub args: Vec<String>,
    /// Lowercase extensions this command applies to. Empty = every file.
    #[serde(default)]
    pub extensions: Vec<String>,
}

impl CustomCommand {
    /// Should this command appear for `path`?
    pub fn applies_to(&self, path: &Path) -> bool {
        if self.extensions.is_empty() {
            return true;
        }
        let ext = path
            .extension()
            .and_then(|s| s.to_str())
            .map(str::to_ascii_lowercase)
            .unwrap_or_default();
        self.extensions.iter().any(|e| e.eq_ignore_ascii_case(&ext))
    }
}

/// Expand `{path}` / `{dir}` / `{name}` / `{stem}` / `{ext}` in one
/// argument template.
///
/// Substitution happens on the *argument vector*, never on a shell
/// command line — the expanded values are passed to `Command::arg`
/// verbatim, so a filename containing `&&` or `$(...)` is inert.
pub fn expand_placeholders(template: &str, path: &Path) -> String {
    let dir = path.parent().unwrap_or(path).to_string_lossy().into_owned();
    let name = path
        .file_name()
        .map(|s| s.to_string_lossy().into_owned())
        .unwrap_or_default();
    let stem = path
        .file_stem()
        .map(|s| s.to_string_lossy().into_owned())
        .unwrap_or_default();
    let ext = path
        .extension()
        .map(|s| s.to_string_lossy().into_owned())
        .unwrap_or_default();
    template
        .replace("{path}", &path.to_string_lossy())
        .replace("{dir}", &dir)
        .replace("{name}", &name)
        .replace("{stem}", &stem)
        .replace("{ext}", &ext)
}

/// Run a user-defined command against `path`.
pub fn run_custom_command(cmd: &CustomCommand, path: &Path) -> Result<()> {
    let mut c = std::process::Command::new(&cmd.program);
    for a in &cmd.args {
        c.arg(expand_placeholders(a, path));
    }
    // Templates that name no placeholder still want the file; appending
    // it unconditionally would double it up for the ones that do.
    if !cmd.args.iter().any(|a| a.contains('{')) {
        c.arg(path);
    }
    c.current_dir(working_dir_for(path)?);
    detach(&mut c);
    c.spawn()?;
    Ok(())
}

/// Don't hold a console window (Windows) or the child's stdio open.
fn detach(cmd: &mut std::process::Command) {
    cmd.stdin(std::process::Stdio::null())
        .stdout(std::process::Stdio::null())
        .stderr(std::process::Stdio::null());
    #[cfg(windows)]
    {
        use std::os::windows::process::CommandExt;
        // CREATE_NO_WINDOW — a GUI app spawning a console child would
        // otherwise flash a black box.
        cmd.creation_flags(0x0800_0000);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn p(s: &str) -> PathBuf {
        PathBuf::from(s)
    }

    #[test]
    fn placeholders_expand_every_token() {
        let path = p("/vault/docs/report.final.pdf");
        assert_eq!(
            expand_placeholders("{name}", &path),
            "report.final.pdf",
            "name keeps every dot"
        );
        assert_eq!(expand_placeholders("{stem}", &path), "report.final");
        assert_eq!(expand_placeholders("{ext}", &path), "pdf");
        assert_eq!(expand_placeholders("{dir}", &path), "/vault/docs");
        assert_eq!(
            expand_placeholders("--in={path}", &path),
            "--in=/vault/docs/report.final.pdf"
        );
    }

    #[test]
    fn a_template_without_placeholders_is_left_alone() {
        assert_eq!(
            expand_placeholders("--verbose", &p("/a/b.txt")),
            "--verbose"
        );
    }

    #[test]
    fn shell_metacharacters_in_a_filename_are_not_expanded() {
        // The value lands in one argv slot; nothing re-parses it.
        let path = p("/vault/$(rm -rf ~) && evil.txt");
        let arg = expand_placeholders("{path}", &path);
        assert_eq!(arg, "/vault/$(rm -rf ~) && evil.txt");
    }

    #[test]
    fn extension_scoping_filters_commands() {
        let scoped = CustomCommand {
            id: "1".into(),
            name: "Edit".into(),
            program: "code".into(),
            args: vec!["{path}".into()],
            extensions: vec!["rs".into(), "TOML".into()],
        };
        assert!(scoped.applies_to(&p("/a/lib.rs")));
        assert!(
            scoped.applies_to(&p("/a/Cargo.toml")),
            "match is case-insensitive"
        );
        assert!(!scoped.applies_to(&p("/a/notes.md")));

        let unscoped = CustomCommand {
            extensions: vec![],
            ..scoped
        };
        assert!(unscoped.applies_to(&p("/a/anything.xyz")));
    }

    #[test]
    fn working_dir_of_a_file_is_its_parent() {
        let dir = tempfile::tempdir().unwrap();
        let file = dir.path().join("x.txt");
        std::fs::write(&file, b"hi").unwrap();
        assert_eq!(working_dir_for(&file).unwrap(), dir.path());
        assert_eq!(working_dir_for(dir.path()).unwrap(), dir.path());
    }
}
