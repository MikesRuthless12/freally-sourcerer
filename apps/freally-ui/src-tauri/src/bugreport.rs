//! TASK-BR1 — opt-in, anonymous bug reporting.
//!
//! **Charter-clean: no telemetry, nothing auto-sends, no server we run, no
//! credentials shipped.** Every path here ends at a *pre-filled draft* that
//! the user reads and then submits themselves.
//!
//! A panic hook writes a **scrubbed** crash report to a local file. On the
//! next launch the UI notices it and offers to report it — an offer, not a
//! send. The dialog shows the **exact** text that would leave the machine
//! (app version, OS/arch, and optionally the crash excerpt), and the two
//! submit buttons open a pre-filled GitHub issue or a pre-filled mail
//! draft. Both require the user's own click to actually send.
//!
//! # What is collected, and what is deliberately not
//!
//! The diagnostics line is the app version and `OS/arch`. That is all. Not
//! the hostname, not the locale, not the index contents, not a file path,
//! not a query. A crash excerpt carries a panic message, a source location
//! and a backtrace — all of which can contain the user's home path, so
//! [`scrub`] redacts the home directory and the bare username before the
//! text is ever written to disk, not merely before it is sent.
//!
//! One deliberate exception: the crash file is stamped with a UTC
//! timestamp. That is weakly identifying at the scale of "which day" and
//! it is included because a crash reported four days later is otherwise
//! impossible to order against the release it came from. The user reads it
//! before anything is sent.
//!
//! # Why the report is built in Rust rather than in the webview
//!
//! Because the scrubbing has to happen on the side that knows the real
//! home path, and because the crash file must be written by a process that
//! is in the act of dying. A panicking thread cannot ask the webview for
//! anything.
//!
//! # Test affordances
//!
//! Two, both in Settings → Logs & Debug, because the loop this implements
//! is otherwise only exercised by a real crash:
//!
//! - **Simulate a crash report** writes a clearly-labelled fake crash file
//!   and returns. The app keeps running; the report dialog can then be
//!   opened against it. This drills the *reporting* half.
//! - **Force a test crash** writes the same file and then exits with a
//!   panic-like code. This drills the *whole* loop — crash, relaunch, and
//!   the offer that should appear on the next launch.
//!
//! Both write `TEST CRASH` into the report itself, so one that reaches the
//! inbox is unmistakable.

use std::path::PathBuf;
use std::sync::OnceLock;

use serde::Serialize;

/// Named in the subject line and the body so a report landing in a shared
/// inbox is attributable to the right Havoc app without being opened.
const APP_NAME: &str = "Freally Sourcerer";
/// The project's issue tracker. A pre-filled URL the user submits while
/// signed in as themselves — no token, nothing of ours in the request.
const GITHUB_NEW_ISSUE: &str = "https://github.com/MikesRuthless12/freally-sourcerer/issues/new";
/// Where an emailed report goes. The user's own mail client sends it.
const REPORT_EMAIL: &str = "mythodikalone@gmail.com";
/// Gmail's web composer. Plain https, no API key. A signed-out user gets
/// Google's login screen and is returned to the pre-filled draft. Offered
/// *alongside* `mailto:`, which stays the path for everyone else.
const GMAIL_COMPOSE: &str = "https://mail.google.com/mail/?view=cm&fs=1";

/// Bounds on the **percent-encoded** body. A character cap cannot bound a
/// URL: one 3-byte character (`—`, `“`) encodes to nine, so 80 CJK
/// characters become 720 bytes. Browsers take ~32 k, so the https targets
/// are generous.
const MAX_GITHUB_ENCODED: usize = 6000;
const MAX_GMAIL_ENCODED: usize = 6000;
/// `mailto:` rides Windows' ShellExecute, which in practice truncates near
/// 2048 characters and then opens **nothing at all** — a blank window and
/// no error. The bound is on the whole URL, scheme and address and subject
/// included, because the subject is user text and can be 80 emoji.
const MAX_MAILTO_URL: usize = 1900;
/// …of which the subject may claim at most this, so a pathological subject
/// cannot starve the body of every byte.
const MAX_MAILTO_SUBJECT_ENCODED: usize = 300;

/// Where crash files live. Resolved once, at setup, because the panic hook
/// runs on a dying thread that cannot ask Tauri for a path.
static CRASH_DIR: OnceLock<PathBuf> = OnceLock::new();

fn crash_dir() -> Option<&'static PathBuf> {
    CRASH_DIR.get()
}

/// The OS user's home directory, without pulling in a crate for it.
///
/// `cargo deny` runs on this workspace and the dependency tree is kept
/// deliberately lean; two environment variables are not worth a crate.
fn home_dir() -> Option<String> {
    #[cfg(windows)]
    let raw = std::env::var_os("USERPROFILE");
    #[cfg(not(windows))]
    let raw = std::env::var_os("HOME");
    raw.map(|s| s.to_string_lossy().into_owned())
        .filter(|s| !s.is_empty())
}

/// Redact the home path and the bare username so a report carries no
/// personal identifier.
///
/// Applied when the crash file is **written**, not when it is sent: a
/// scrubbed file on disk cannot be leaked by some later path that forgets
/// to scrub. That matters more than "the user reads it first" implies —
/// `bug_report_context` hands the crash text to the webview on every
/// launch, so under this project's threat model the text is reachable
/// before any human sees it.
///
/// **Case-insensitive, and over both separator forms.** Windows paths are
/// case-insensitive and this app produces lowercased copies of them all
/// over (`to_lowercase` in the query executor, the volume map, the
/// index); a byte-exact `replace` left every one of those intact.
/// `{:?}`-formatted paths also double their backslashes, and a search
/// term or a `freally://` link can arrive in any casing at all.
pub fn scrub(text: &str) -> String {
    // No home to redact against means no way to know what is personal.
    // Fail closed — [`write_crash`] refuses rather than writing a report
    // that may name the user.
    let Some(home) = home_dir() else {
        return String::new();
    };
    let mut out = text.to_string();
    for form in home_forms(&home) {
        out = replace_ignore_case(&out, &form, "<home>");
    }
    // Then the bare username, which appears in paths the home prefix does
    // not cover — a temp dir, a UNC share, another tool's log line. Only
    // as a whole path component: redacting a bare "jo" everywhere would
    // shred the surrounding backtrace.
    if let Some(user) = std::path::Path::new(&home)
        .file_name()
        .and_then(|n| n.to_str())
        .filter(|n| !n.is_empty())
    {
        for sep in ['\\', '/'] {
            out = replace_ignore_case(
                &out,
                &format!("{sep}{user}{sep}"),
                &format!("{sep}<user>{sep}"),
            );
        }
    }
    out
}

/// Every spelling of the home path a report might carry: as the OS gives
/// it, with forward slashes, and with the doubled backslashes `{:?}`
/// produces. Longest first, so the doubled form is redacted before the
/// single one can consume half of it.
fn home_forms(home: &str) -> Vec<String> {
    let mut forms = vec![
        home.replace('\\', "\\\\"),
        home.replace('\\', "/"),
        home.to_string(),
    ];
    forms.sort_by_key(|f| std::cmp::Reverse(f.len()));
    forms.dedup();
    forms.retain(|f| !f.is_empty());
    forms
}

/// `str::replace`, matching without regard to case.
///
/// Offsets are found on lowercased copies and spliced out of the
/// original, so the surrounding text keeps its casing. Both sides are
/// lowercased with the same function, so a needle and a haystack that
/// differ only in case line up byte for byte for ASCII — which is what
/// every path separator and drive letter is.
fn replace_ignore_case(haystack: &str, needle: &str, with: &str) -> String {
    if needle.is_empty() || needle.len() > haystack.len() {
        return haystack.to_string();
    }
    let (hay_lower, needle_lower) = (haystack.to_lowercase(), needle.to_lowercase());
    // A non-ASCII fold can change byte length, which would make offsets
    // into the lowercased copy meaningless against the original. Paths
    // that fold that way are rare; a byte-exact pass is still correct for
    // them, just less thorough.
    if hay_lower.len() != haystack.len() {
        return haystack.replace(needle, with);
    }
    let mut out = String::with_capacity(haystack.len());
    let mut at = 0usize;
    while let Some(found) = hay_lower[at..].find(&needle_lower) {
        let start = at + found;
        out.push_str(&haystack[at..start]);
        out.push_str(with);
        at = start + needle_lower.len();
    }
    out.push_str(&haystack[at..]);
    out
}

/// The anonymous system line. App version and OS/arch — nothing else.
///
/// The version is the running app's, not `CARGO_PKG_VERSION`:
/// `tauri.conf.json` carries its own and the two have drifted before (see
/// `commands::app_env`). A report that names a version nobody shipped is
/// worse than one with no version at all.
pub fn diagnostics(version: &str) -> String {
    format!(
        "App: {APP_NAME} {version}\nOS: {}\n",
        format_args!("{} / {}", std::env::consts::OS, std::env::consts::ARCH),
    )
}

/// The running app's version, which is not `CARGO_PKG_VERSION`:
/// `tauri.conf.json` carries its own and the two have drifted before (see
/// `commands::app_env`). A report naming a version nobody shipped is
/// worse than one with no version at all.
fn app_version(app: &tauri::AppHandle) -> String {
    app.package_info().version.to_string()
}

/// Point the crash writer at a directory and chain a crash-capturing hook
/// onto whatever panic hook is already installed. Call once, from setup,
/// where an `AppHandle` exists to resolve the path.
pub fn install_panic_hook(dir: PathBuf) {
    let _ = CRASH_DIR.set(dir);
    let previous = std::panic::take_hook();
    std::panic::set_hook(Box::new(move |info| {
        let location = info
            .location()
            .map(|loc| format!("{}:{}", loc.file(), loc.line()))
            .unwrap_or_else(|| "unknown".to_string());
        let message = panic_message(info);
        let backtrace = std::backtrace::Backtrace::force_capture();
        let raw = format!("Panic at {location}\nMessage: {message}\n\nBacktrace:\n{backtrace}\n");
        write_crash(&raw);
        // Chain rather than replace: the existing hook logs the panic
        // through `tracing`, which is what a developer with a console
        // reads. Dropping it would trade one diagnostic for another.
        previous(info);
    }));
}

/// The panic payload as a string. `panic!("literal")` gives `&str`;
/// `panic!("{x}")` and `.expect()` give `String`. Missing either one
/// yields "(no message)" for half of all real panics.
fn panic_message(info: &std::panic::PanicHookInfo<'_>) -> String {
    info.payload()
        .downcast_ref::<&str>()
        .map(|s| (*s).to_string())
        .or_else(|| info.payload().downcast_ref::<String>().cloned())
        .unwrap_or_else(|| "(no message)".to_string())
}

/// `YYYY-MM-DD HH:MM:SS UTC` from the wall clock.
///
/// UTC, with no timezone database and no `chrono` dependency — `cargo
/// deny` runs on this workspace and the tree is kept lean on purpose. The
/// date conversion is `freally_query::parser::epoch_day_to_civil`, which
/// the `datemodified:` suite already exercises; a second copy of that
/// arithmetic here would be the trickiest duplicated code in the tree.
fn utc_stamp() -> String {
    let secs = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .map(|d| d.as_secs() as i64)
        .unwrap_or(0);
    let (y, mth, d) = freally_query::parser::epoch_day_to_civil(secs.div_euclid(86_400));
    let tod = secs.rem_euclid(86_400);
    let (h, m, s) = (tod / 3600, (tod % 3600) / 60, tod % 60);
    format!("Crashed: {y:04}-{mth:02}-{d:02} {h:02}:{m:02}:{s:02} UTC")
}

/// Write one crash file, scrubbing on the way in.
///
/// The redaction happens **here**, not in the callers: this is the layer
/// that owns "nothing personal reaches disk", and a caller that forgets
/// leaves PII in a file with nothing to notice it. Best-effort otherwise
/// — this runs while the process is dying, and failing to record a crash
/// must never become a second panic inside the panic hook.
fn write_crash(raw: &str) {
    let scrubbed = &scrub(raw);
    // `scrub` returns empty when it could not establish what to redact.
    // Writing the raw text would be the one outcome worse than losing the
    // report.
    if scrubbed.is_empty() {
        return;
    }
    let Some(dir) = crash_dir() else { return };
    if std::fs::create_dir_all(dir).is_err() {
        return;
    }
    // Owner-only. These are on a per-user path already, but the fallback
    // when `app_data_dir()` fails is the system temp dir — mode 1777 on
    // Linux, where the default umask would leave each report readable by
    // every local account.
    restrict_to_owner(dir, 0o700);
    let ts = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .map(|d| d.as_millis())
        .unwrap_or(0);
    let stamped = format!("{}\n{scrubbed}", utc_stamp());
    let path = dir.join(format!("crash-{ts}.txt"));
    if std::fs::write(&path, stamped).is_ok() {
        restrict_to_owner(&path, 0o600);
    }
}

/// Owner-only permissions. A no-op off Unix, where the per-user app-data
/// path already carries the right ACL.
#[allow(unused_variables)]
fn restrict_to_owner(path: &std::path::Path, mode: u32) {
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        let _ = std::fs::set_permissions(path, std::fs::Permissions::from_mode(mode));
    }
}

/// The newest pending crash report, already scrubbed, if any.
pub fn pending_crash() -> Option<String> {
    let dir = crash_dir()?;
    let mut newest: Option<(u128, PathBuf)> = None;
    for entry in std::fs::read_dir(dir).ok()?.flatten() {
        let path = entry.path();
        if path.extension().and_then(|e| e.to_str()) != Some("txt") {
            continue;
        }
        let mtime = entry
            .metadata()
            .ok()
            .and_then(|m| m.modified().ok())
            .and_then(|t| t.duration_since(std::time::UNIX_EPOCH).ok())
            .map(|d| d.as_millis())
            .unwrap_or(0);
        if newest.as_ref().is_none_or(|(t, _)| mtime > *t) {
            newest = Some((mtime, path));
        }
    }
    std::fs::read_to_string(newest?.1).ok()
}

/// Delete the pending crash reports — the user sent or dismissed them.
pub fn clear_crashes() {
    if let Some(dir) = crash_dir() {
        let _ = std::fs::remove_dir_all(dir);
    }
}

/// Percent-encode a query component, keeping RFC 3986's unreserved set.
fn urlencode(s: &str) -> String {
    let mut out = String::with_capacity(s.len() * 3);
    for byte in s.as_bytes() {
        match byte {
            b'A'..=b'Z' | b'a'..=b'z' | b'0'..=b'9' | b'-' | b'_' | b'.' | b'~' => {
                out.push(*byte as char)
            }
            other => out.push_str(&format!("%{other:02X}")),
        }
    }
    out
}

fn truncate_chars(s: &str, max: usize) -> String {
    if s.chars().count() <= max {
        return s.to_string();
    }
    s.chars().take(max).collect()
}

fn encode_bounded(s: &str, max_encoded: usize) -> String {
    encode_bounded_with(
        s,
        max_encoded,
        "\n… (truncated — use “Copy report” for the full text)",
    )
}

/// Percent-encode `s` so the result never exceeds `max_encoded` bytes,
/// appending `note` (itself encoded, and reserved out of the budget) when
/// anything was cut. Cuts on whole encoded characters — never half of a
/// `%E2%80%94`, which would produce a URL the OS silently refuses.
fn encode_bounded_with(s: &str, max_encoded: usize, note: &str) -> String {
    let full = urlencode(s);
    if full.len() <= max_encoded {
        return full;
    }
    let note = urlencode(note);
    let budget = max_encoded.saturating_sub(note.len());
    let mut out = String::with_capacity(max_encoded);
    let mut buf = [0u8; 4];
    for ch in s.chars() {
        let piece = urlencode(ch.encode_utf8(&mut buf));
        if out.len() + piece.len() > budget {
            break;
        }
        out.push_str(&piece);
    }
    out.push_str(&note);
    out
}

/// A pre-filled GitHub "new issue" URL. The user submits it signed in as
/// themselves; nothing of ours rides along.
pub fn github_url(title: &str, body: &str) -> String {
    format!(
        "{GITHUB_NEW_ISSUE}?labels=bug&title={}&body={}",
        urlencode(&truncate_chars(title, 200)),
        encode_bounded(body, MAX_GITHUB_ENCODED),
    )
}

/// A pre-filled `mailto:` draft, bounded as a whole URL — see
/// [`MAX_MAILTO_URL`].
pub fn mailto_url(subject: &str, body: &str) -> String {
    let head = format!(
        "mailto:{REPORT_EMAIL}?subject={}&body=",
        // No truncation note on a subject line — a trailing "(truncated)"
        // sentence there would be absurd.
        encode_bounded_with(subject, MAX_MAILTO_SUBJECT_ENCODED, ""),
    );
    let budget = MAX_MAILTO_URL.saturating_sub(head.len());
    format!("{head}{}", encode_bounded(body, budget))
}

/// A pre-filled Gmail web-compose draft. Unlike `mailto:` this does not
/// depend on a registered mail handler, which many Linux desktops and
/// fresh Windows installs simply do not have.
pub fn gmail_url(subject: &str, body: &str) -> String {
    format!(
        "{GMAIL_COMPOSE}&to={}&su={}&body={}",
        urlencode(REPORT_EMAIL),
        urlencode(&truncate_chars(subject, 200)),
        encode_bounded(body, MAX_GMAIL_ENCODED),
    )
}

/// Open a URL we built ourselves with the OS default handler.
///
/// The allowlist is not ceremony. This opens whatever it is handed with
/// the user's own credentials and permissions, so a `file:` URL would read
/// their disk and a `javascript:` one would run in whatever opened it.
/// Only the two schemes this module constructs are permitted, and the URL
/// is passed as a single argv entry — never through a shell.
#[cfg(windows)]
pub(crate) fn system32(exe: &str) -> std::path::PathBuf {
    // `CreateProcessW` with a null application name searches the launching
    // image's directory and the **current directory** before System32.
    // Portable mode is a supported shape here and puts this executable in
    // user-writable places with a CWD the app never chose (a `freally://`
    // launch, for one), so a planted `rundll32.exe` beside it would run
    // as the user the moment they clicked "Report a Bug".
    let root = std::env::var_os("SystemRoot")
        .map(std::path::PathBuf::from)
        .unwrap_or_else(|| std::path::PathBuf::from(r"C:\Windows"));
    root.join("System32").join(exe)
}

fn open_url(url: &str) -> Result<(), String> {
    if !(url.starts_with("https://") || url.starts_with("mailto:")) {
        return Err("refusing to open a non-https/mailto URL".into());
    }
    if url.chars().any(char::is_control) {
        return Err("invalid URL".into());
    }
    #[cfg(windows)]
    let mut cmd = {
        let mut c = std::process::Command::new(system32("rundll32.exe"));
        c.args(["url.dll,FileProtocolHandler", url]);
        c
    };
    #[cfg(target_os = "macos")]
    let mut cmd = {
        let mut c = std::process::Command::new("/usr/bin/open");
        c.arg(url);
        c
    };
    #[cfg(all(unix, not(target_os = "macos")))]
    let mut cmd = {
        let mut c = std::process::Command::new("xdg-open");
        c.arg(url);
        c
    };
    cmd.spawn()
        .map(|_| ())
        .map_err(|err| format!("could not open the link: {err}"))
}

/// One line naming what went wrong, for the subject.
///
/// The panic message when there is a crash, else the first line the user
/// typed, else a generic label. This is what makes a subject triageable
/// from an inbox list without opening it.
fn error_summary(crash: Option<&str>, description: &str) -> String {
    let raw = crash
        .and_then(|c| {
            c.lines()
                .find_map(|line| line.strip_prefix("Message: "))
                .map(str::to_string)
        })
        .filter(|s| !s.trim().is_empty())
        .or_else(|| {
            description
                .lines()
                .map(str::trim)
                .find(|line| !line.is_empty())
                .map(str::to_string)
        })
        .unwrap_or_else(|| {
            if crash.is_some() {
                "crash report".to_string()
            } else {
                "bug report".to_string()
            }
        });
    let one_line = raw.split_whitespace().collect::<Vec<_>>().join(" ");
    if one_line.chars().count() > 80 {
        format!("{}…", one_line.chars().take(80).collect::<String>())
    } else {
        one_line
    }
}

/// `[Freally Sourcerer] <what went wrong>` — the app **and** the error, so
/// a report is attributable at a glance in a shared inbox.
fn subject(crash: Option<&str>, description: &str) -> String {
    format!("[{APP_NAME}] {}", error_summary(crash, description))
}

/// How the body is rendered. The content is identical; only the syntax
/// around it changes.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BodyStyle {
    /// GitHub renders it — headings and a fenced diagnostics block.
    Markdown,
    /// Mail clients do not; they show `###` and ``` as literal noise.
    Plain,
}

/// Assemble the report from the user's note, the diagnostics, and the
/// crash excerpt when one is being included.
pub fn compose_body(
    version: &str,
    description: &str,
    crash: Option<&str>,
    style: BodyStyle,
) -> String {
    let markdown = style == BodyStyle::Markdown;
    let mut body = String::new();

    body.push_str(if markdown {
        "### What happened\n"
    } else {
        "WHAT HAPPENED\n"
    });
    body.push_str(if description.trim().is_empty() {
        "(no description provided)"
    } else {
        description.trim()
    });

    body.push_str(if markdown {
        "\n\n### Anonymous diagnostics (no personal data)\n```\n"
    } else {
        "\n\nANONYMOUS DIAGNOSTICS (no personal data)\n"
    });
    body.push_str(&diagnostics(version));
    if let Some(crash) = crash {
        body.push_str("\n--- crash excerpt ---\n");
        body.push_str(crash);
    }
    body.push_str(if markdown { "\n```\n" } else { "\n" });

    // Belt and braces. The crash text was scrubbed on the way to disk and
    // the description is the user's own words, but this is the last point
    // before a URL is built and it costs one pass.
    scrub(&body)
}

// --- Tauri commands --------------------------------------------------------

/// Whether the last run left a crash behind. The dialog needs this to
/// decide whether to offer the crash excerpt; everything it *displays*
/// comes from [`bug_report_preview`], which builds it with the same code
/// the submit path uses. Nothing here leaves the machine.
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct BugReportContextDto {
    /// The scrubbed crash text from the previous run, when the app
    /// crashed. `None` is the ordinary case.
    pub pending_crash: Option<String>,
}

#[tauri::command]
pub fn bug_report_context() -> BugReportContextDto {
    BugReportContextDto {
        pending_crash: pending_crash(),
    }
}

/// The exact text that would be sent, so the dialog can show it rather
/// than describe it. Same builder the submit path uses — a preview that
/// came from a second implementation would be a preview of nothing.
#[tauri::command]
pub fn bug_report_preview(
    app: tauri::AppHandle,
    description: String,
    include_crash: bool,
) -> String {
    let crash = if include_crash { pending_crash() } else { None };
    let subject = subject(crash.as_deref(), &description);
    format!(
        "Subject: {subject}\n\n{}",
        compose_body(
            &app_version(&app),
            &description,
            crash.as_deref(),
            BodyStyle::Plain
        )
    )
}

/// Open a pre-filled draft. `target` is `"github"`, `"gmail"` or
/// `"email"`. **This opens a draft and stops.** The user still clicks send.
#[tauri::command]
pub fn bug_report_submit(
    app: tauri::AppHandle,
    target: String,
    description: String,
    include_crash: bool,
) -> Result<(), String> {
    let crash = if include_crash { pending_crash() } else { None };
    let subject = subject(crash.as_deref(), &description);
    let crash = crash.as_deref();
    let version = app_version(&app);
    let url = match target.as_str() {
        "github" => github_url(
            &subject,
            &compose_body(&version, &description, crash, BodyStyle::Markdown),
        ),
        "gmail" => gmail_url(
            &subject,
            &compose_body(&version, &description, crash, BodyStyle::Plain),
        ),
        "email" => mailto_url(
            &subject,
            &compose_body(&version, &description, crash, BodyStyle::Plain),
        ),
        other => return Err(format!("unknown report target: {other}")),
    };
    open_url(&url)
}

/// Dismiss and delete the pending crash report(s).
#[tauri::command]
pub fn bug_report_clear_crash() {
    clear_crashes();
}

/// The text both test affordances write, so a drill report is
/// unmistakable in the inbox.
fn test_crash_text(kind: &str) -> String {
    format!(
        "Panic at src/bugreport.rs:0\nMessage: TEST CRASH — {kind} from Settings → Logs & \
         Debug. No real fault occurred.\n\nBacktrace:\n(test)\n"
    )
}

/// **Simulate a crash report** — write a fake crash file and keep running.
/// Drills the reporting half of the loop without losing the session.
#[tauri::command]
pub fn bug_report_simulate_crash() {
    write_crash(&test_crash_text("simulated"));
}

/// **Force a test crash** — write the file, then exit. Drills the whole
/// loop: crash, relaunch, and the offer that should be waiting on the
/// next launch.
///
/// Exits through Tauri rather than `std::process::exit`. What the drill
/// needs to prove is that the report survives to the next launch, and a
/// raw `exit` would additionally skip `Daemon::drop` — abandoning the
/// indexer child and whatever the watchers had pending. That is real data
/// loss for a *test* button, and it is reachable from the webview, which
/// under this project's threat model is the attacker.
#[tauri::command]
pub fn bug_report_force_crash(app: tauri::AppHandle) {
    write_crash(&test_crash_text("forced"));
    app.exit(101);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn scrub_redacts_the_home_path_and_the_username() {
        let Some(home) = home_dir() else { return };
        let text = format!("failed to open {home}/Documents/taxes.pdf");
        let scrubbed = scrub(&text);
        assert!(!scrubbed.contains(&home), "home path survived: {scrubbed}");
        if let Some(user) = std::path::Path::new(&home)
            .file_name()
            .and_then(|n| n.to_str())
            .filter(|n| n.len() >= 3)
        {
            assert!(!scrubbed.contains(user), "username survived: {scrubbed}");
        }
    }

    #[test]
    fn scrub_catches_a_windows_home_written_with_forward_slashes() {
        // Rust's `Path::display` and `format!("{:?}")` disagree about
        // separators, and a backtrace carries both. Redacting only the
        // form the OS reports would leak the other.
        let Some(home) = home_dir() else { return };
        if !home.contains('\\') {
            return;
        }
        let flipped = home.replace('\\', "/");
        assert!(!scrub(&format!("at {flipped}/x")).contains(&flipped));
    }

    #[test]
    fn scrub_ignores_case_because_windows_paths_do() {
        // The app produces lowercased copies of paths all over — the query
        // executor, the volume map, the index — and a search term or a
        // `freally://` link can arrive in any casing at all. A byte-exact
        // replace left every one of those intact, which is a leak of the
        // full home path and the account name.
        let Some(home) = home_dir() else { return };
        let shouted = home.to_uppercase();
        let hushed = home.to_lowercase();
        for spelling in [shouted, hushed] {
            let scrubbed = scrub(&format!("failed to open {spelling}/notes.txt"));
            assert!(
                scrubbed.contains("<home>"),
                "{spelling} was not redacted: {scrubbed}"
            );
        }
    }

    #[test]
    fn scrub_catches_the_doubled_backslashes_that_debug_formatting_produces() {
        // `format!("{:?}", path)` doubles every separator, so a backtrace
        // carries a spelling that matches neither the OS's form nor the
        // forward-slash one.
        let Some(home) = home_dir() else { return };
        if !home.contains('\\') {
            return;
        }
        let doubled = home.replace('\\', "\\\\");
        let scrubbed = scrub(&format!("at \"{doubled}\\\\x\""));
        assert!(
            !scrubbed.contains(&doubled),
            "doubled form survived: {scrubbed}"
        );
    }

    #[test]
    fn scrub_redacts_a_username_as_a_path_component_however_short() {
        // The old rule skipped usernames under three characters entirely,
        // to avoid shredding a backtrace by redacting every "jo". Matching
        // a whole path component gets both: the name goes, the prose stays.
        assert_eq!(
            replace_ignore_case("/Users/jo/x and a job listing", "/jo/", "/<user>/"),
            "/Users/<user>/x and a job listing"
        );
    }

    #[test]
    fn scrub_fails_closed_when_there_is_no_home_to_redact_against() {
        // With no home directory there is no way to know what is personal,
        // so an empty result tells `write_crash` to write nothing at all.
        // A report that may name the user is worse than no report.
        let saved = std::env::var_os(if cfg!(windows) { "USERPROFILE" } else { "HOME" });
        // SAFETY: single-threaded test; the var is restored below.
        unsafe {
            std::env::remove_var(if cfg!(windows) { "USERPROFILE" } else { "HOME" });
        }
        let out = scrub("anything at all");
        unsafe {
            if let Some(v) = saved {
                std::env::set_var(if cfg!(windows) { "USERPROFILE" } else { "HOME" }, v);
            }
        }
        assert!(
            out.is_empty(),
            "expected a fail-closed empty scrub, got {out:?}"
        );
    }

    #[test]
    fn replace_ignore_case_keeps_the_surrounding_text_as_written() {
        assert_eq!(replace_ignore_case("A/B/C", "/b/", "/<x>/"), "A/<x>/C");
        assert_eq!(
            replace_ignore_case("no match here", "zzz", "!"),
            "no match here"
        );
        assert_eq!(replace_ignore_case("aaa", "a", "b"), "bbb");
        // An empty needle would otherwise loop forever.
        assert_eq!(replace_ignore_case("abc", "", "!"), "abc");
    }

    #[test]
    fn diagnostics_carry_the_version_and_the_platform_and_nothing_else() {
        let d = diagnostics("9.9.9");
        assert!(d.contains("9.9.9"));
        assert!(d.contains(std::env::consts::OS));
        assert!(d.contains(std::env::consts::ARCH));
        // Three lines' worth at most: two fields and a trailing newline.
        assert_eq!(d.lines().count(), 2, "diagnostics grew a field: {d}");
    }

    #[test]
    fn the_subject_names_the_app_and_the_panic_message() {
        let crash =
            "Crashed: 2026-08-22 01:00:00 UTC\nPanic at src/x.rs:9\nMessage: index out of bounds\n";
        let s = subject(Some(crash), "");
        assert_eq!(s, "[Freally Sourcerer] index out of bounds");
    }

    #[test]
    fn the_subject_falls_back_to_the_users_first_line() {
        let s = subject(None, "  \n\nSearch hangs on a network drive\nmore detail\n");
        assert_eq!(s, "[Freally Sourcerer] Search hangs on a network drive");
    }

    #[test]
    fn a_mailto_url_stays_under_the_shellexecute_cliff() {
        // The failure this bounds is silent: past ~2048 characters
        // ShellExecute opens a blank window and reports success.
        let huge = "\u{1F600}".repeat(4000);
        let url = mailto_url(&format!("[App] {huge}"), &huge);
        assert!(
            url.len() <= MAX_MAILTO_URL,
            "mailto URL was {} bytes",
            url.len()
        );
        assert!(url.starts_with("mailto:"));
    }

    #[test]
    fn truncation_never_splits_an_encoded_character() {
        // A half-written `%E2%80` is a URL the OS refuses without saying
        // why, which would look exactly like "the button does nothing".
        let s = "—".repeat(100);
        let out = encode_bounded_with(&s, 40, "");
        assert!(out.len() <= 40);
        assert_eq!(out.len() % 9, 0, "an em dash encodes to 9 bytes: {out}");
    }

    #[test]
    fn open_url_refuses_anything_but_https_and_mailto() {
        for bad in [
            "file:///etc/passwd",
            "javascript:alert(1)",
            "ftp://example.com",
            "https://example.com/\u{7}",
        ] {
            assert!(open_url(bad).is_err(), "{bad} should have been refused");
        }
    }

    #[test]
    fn the_body_says_what_it_carries_and_carries_nothing_else() {
        let body = compose_body("9.9.9", "it broke", None, BodyStyle::Plain);
        assert!(body.contains("it broke"));
        assert!(body.contains("ANONYMOUS DIAGNOSTICS"));
        assert!(body.contains("9.9.9"));
        assert!(!body.contains("crash excerpt"), "no crash was included");
    }

    #[test]
    fn the_crash_excerpt_is_included_only_when_asked_for() {
        let with = compose_body("9.9.9", "x", Some("Message: boom"), BodyStyle::Markdown);
        assert!(with.contains("crash excerpt") && with.contains("boom"));
    }

    #[test]
    fn utc_stamp_is_a_real_date() {
        let s = utc_stamp();
        assert!(s.starts_with("Crashed: 20"), "{s}");
        assert!(s.ends_with(" UTC"), "{s}");
        // `Crashed: YYYY-MM-DD HH:MM:SS UTC`
        assert_eq!(s.len(), "Crashed: 2026-08-22 01:23:45 UTC".len(), "{s}");
    }
}
