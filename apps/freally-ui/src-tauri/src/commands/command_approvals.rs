//! First-run approval for custom commands (HANDOFF §2.1).
//!
//! `run_custom_command` takes a command *id* over IPC rather than a program
//! path, so the launchable set is "whatever the user saved". That argument
//! holds exactly as far as the saved set is trustworthy — and the webview is
//! what saves it. A hostile transitive npm dependency in the bundle can call
//! `settings_set` to install a command and then `run_custom_command` to
//! launch it, with no user gesture in between.
//!
//! The gate is a confirmation the **first time a given program runs**, keyed
//! on the program rather than on the command id: renaming a command, or
//! installing a second one pointing at an already-approved program, is not a
//! new program. Two properties make it worth having.
//!
//! * **The dialog is native.** The dialog plugin's blocking confirm is an OS
//!   window. A compromised webview can draw a convincing HTML dialog and
//!   click its own button; it cannot dismiss this one.
//! * **The approvals do not live in `settings.json`.** That file is writable
//!   over IPC through `settings_set`, which is the very path being defended
//!   against — an attacker who could pre-approve their own program would have
//!   gained nothing but an extra write. They live in a sibling file that the
//!   command surface never exposes.
//!
//! What this does not claim: it is a speed bump against a compromised
//! webview, not a sandbox. A user who clicks Run on a program they did not
//! author is still running it. The value is that *one click on a result row*
//! can no longer become *one silent process spawn*.

use std::collections::BTreeSet;
use std::path::{Path, PathBuf};

/// The approved-program set, as persisted.
///
/// A `BTreeSet` rather than a `Vec` so the file is stable across writes and
/// readable by anyone auditing it by hand.
#[derive(Debug, Default, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub struct Approvals {
    #[serde(default)]
    programs: BTreeSet<String>,
}

impl Approvals {
    /// Read the set, treating every failure as "nothing is approved".
    ///
    /// A missing file is the first run. A corrupt one is not worth guessing
    /// at: failing closed re-asks, which costs a click, where failing open
    /// would silently launch whatever the corruption happened to contain.
    pub fn load(path: &Path) -> Self {
        std::fs::read_to_string(path)
            .ok()
            .and_then(|s| serde_json::from_str::<Self>(&s).ok())
            .unwrap_or_default()
    }

    /// tmp+rename, so a crash mid-write cannot truncate the file into a
    /// partial set — which for this file would mean silently un-approving
    /// programs the user had already approved.
    ///
    /// The tmp name is unique per process and the tmp is removed if the
    /// rename fails. `freally-indexd`'s `write_json` already learned both the
    /// hard way: a fixed `foo.json.tmp` lets two writers write the same file
    /// and then one renames a path the other has consumed, and a failed
    /// rename leaves the tmp behind forever. The three other tmp-rename
    /// copies under `commands/` still have the fixed name and neither fix;
    /// unifying them is recorded in `HANDOFF.md` rather than done here,
    /// because it reaches well outside this change.
    pub fn save(&self, path: &Path) {
        let Ok(json) = serde_json::to_string_pretty(self) else {
            return;
        };
        let tmp = path.with_extension(format!("{}.tmp", std::process::id()));
        if std::fs::write(&tmp, &json).is_err() {
            let _ = std::fs::remove_file(&tmp);
            return;
        }
        if std::fs::rename(&tmp, path).is_err() {
            let _ = std::fs::remove_file(&tmp);
        }
    }

    pub fn contains(&self, program: &str) -> bool {
        self.programs.contains(&approval_key(program))
    }

    pub fn insert(&mut self, program: &str) {
        self.programs.insert(approval_key(program));
    }
}

/// The key a program is remembered under.
///
/// Case-insensitive on Windows and macOS, whose filesystems are: approving
/// `C:\Tools\edit.exe` must not leave `C:\tools\EDIT.EXE` unapproved, because
/// they are the same file and a second prompt would read as a bug. Linux
/// paths are case-sensitive, so they are left alone.
///
/// Deliberately *not* canonicalized. Canonicalizing resolves symlinks, so the
/// approval would follow the link target rather than the name the user was
/// shown and agreed to — and it fails outright for a bare `code` that exists
/// only on PATH, which is the documented way to write one of these.
fn approval_key(program: &str) -> String {
    if cfg!(any(windows, target_os = "macos")) {
        program.to_lowercase()
    } else {
        program.to_string()
    }
}

/// Where the set lives. Beside `settings.json`, but not inside it.
pub fn approvals_path(app: &tauri::AppHandle) -> PathBuf {
    super::settings::app_data_root(app).join("custom_command_approvals.json")
}

/// The text the native dialog shows.
///
/// It names the program **and** the arguments, because the arguments are
/// where a plausible-looking command hides its payload: a user who reads
/// "Photoshop" and nothing else learns nothing about a trailing
/// download-and-run flag. Kept here so the wording is testable without a
/// window server.
pub fn prompt_body(name: &str, program: &str, args: &[String]) -> String {
    let mut body = format!("{name} runs this program for the first time:\n\n{program}");
    if !args.is_empty() {
        body.push_str("\n\nwith arguments:\n");
        for a in args {
            body.push_str("\n  ");
            body.push_str(a);
        }
    }
    body.push_str(
        "\n\nRun it only if you set this command up yourself. \
         Freally will not ask again for this program.",
    );
    body
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn an_unknown_program_is_not_approved() {
        assert!(!Approvals::default().contains("/usr/bin/code"));
    }

    #[test]
    fn insert_then_contains() {
        let mut a = Approvals::default();
        a.insert("/usr/bin/code");
        assert!(a.contains("/usr/bin/code"));
        assert!(!a.contains("/usr/bin/vim"), "approval is per program");
    }

    #[test]
    fn case_folds_only_where_the_filesystem_does() {
        let mut a = Approvals::default();
        a.insert("C:\\Tools\\Edit.exe");
        let same_file_different_case = a.contains("c:\\tools\\edit.EXE");
        if cfg!(any(windows, target_os = "macos")) {
            assert!(
                same_file_different_case,
                "same file — a second prompt here would read as a bug"
            );
        } else {
            assert!(
                !same_file_different_case,
                "two different files on a case-sensitive filesystem"
            );
        }
    }

    #[test]
    fn round_trips_through_disk() {
        let dir = std::env::temp_dir().join("freally-approvals-roundtrip");
        let _ = std::fs::remove_dir_all(&dir);
        std::fs::create_dir_all(&dir).unwrap();
        let path = dir.join("a.json");

        let mut a = Approvals::default();
        a.insert("/usr/bin/code");
        a.save(&path);
        assert_eq!(Approvals::load(&path), a);

        let _ = std::fs::remove_dir_all(&dir);
    }

    #[test]
    fn a_missing_or_corrupt_file_approves_nothing() {
        let dir = std::env::temp_dir().join("freally-approvals-corrupt");
        let _ = std::fs::remove_dir_all(&dir);
        std::fs::create_dir_all(&dir).unwrap();

        assert_eq!(
            Approvals::load(&dir.join("nope.json")),
            Approvals::default()
        );

        // Failing closed costs a click; failing open launches whatever the
        // damaged bytes happened to decode to.
        let corrupt = dir.join("bad.json");
        std::fs::write(&corrupt, "{ not json").unwrap();
        assert_eq!(Approvals::load(&corrupt), Approvals::default());

        let _ = std::fs::remove_dir_all(&dir);
    }

    #[test]
    fn the_prompt_shows_the_arguments_not_just_the_name() {
        let body = prompt_body(
            "Tidy up",
            "powershell.exe",
            &["-c".into(), "download-and-run".into()],
        );
        assert!(body.contains("powershell.exe"));
        assert!(
            body.contains("download-and-run"),
            "the payload lives in the arguments; hiding them defeats the prompt"
        );
    }

    #[test]
    fn the_prompt_omits_the_argument_block_when_there_are_none() {
        let body = prompt_body("Open", "code", &[]);
        assert!(body.contains("code"));
        assert!(!body.contains("with arguments"));
    }
}
