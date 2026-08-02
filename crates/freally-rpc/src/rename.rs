//! SRC-M15 — the bulk-rename engine.
//!
//! Name arithmetic: given the selected paths and a rule, produce the new
//! *file name* for each one, plus everything the preview table needs to
//! warn before anything touches the disk. Planning is pure, which is what
//! makes the interesting parts — collision detection, template expansion,
//! name validation — testable without a filesystem. Applying a plan
//! (`destination_for` / `perform_moves`) lives at the bottom of the file
//! and is the only part that touches the disk.
//!
//! ## Why the engine returns names, never paths
//!
//! Build 1's threat model established that write-side IPC must not accept
//! caller-chosen paths. A rename has a source *and* a destination, and
//! while the source is a search hit the user selected, the destination is
//! a path that was never in any result set. If the frontend supplied it,
//! a compromised JS layer could rename any search hit to any path on the
//! system — into a startup folder, over a config file, anywhere.
//!
//! So the destination is never transmitted. The backend receives the
//! rule, derives a bare file name, and [`RenamePlan`] refuses anything
//! that could escape the source's own directory. The same derivation runs
//! for the preview and for the apply, so what the user approved is what
//! executes.

use std::path::{Path, PathBuf};

use serde::{Deserialize, Serialize};

/// Which part of the file name the find/replace applies to.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum NamePart {
    /// Everything before the final dot. The common case — renaming a
    /// batch of photos should not have to preserve `.jpg` by hand, and
    /// must not let a typo silently change the file's type.
    Stem,
    /// The whole name including the extension.
    Full,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CaseTransform {
    None,
    Lower,
    Upper,
    /// First letter of each word capitalized, the rest lowered.
    Title,
}

/// One bulk-rename rule, as the dialog collects it.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RenameRule {
    /// Literal substring, or a regex when `use_regex` is set. Empty
    /// means "match nothing" — the name passes through untouched, so a
    /// counter-only or case-only rule still works.
    #[serde(default)]
    pub find: String,
    /// Replacement template. Supports `$1` capture references when
    /// `use_regex` is set, and `{n}` / `{n:03}` counters either way.
    #[serde(default)]
    pub replace: String,
    #[serde(default)]
    pub use_regex: bool,
    /// Case-insensitive matching for `find`.
    #[serde(default)]
    pub ignore_case: bool,
    #[serde(default = "default_case")]
    pub case: CaseTransform,
    #[serde(default = "default_part")]
    pub part: NamePart,
    /// First value `{n}` takes.
    #[serde(default)]
    pub counter_start: u32,
}

fn default_case() -> CaseTransform {
    CaseTransform::None
}
fn default_part() -> NamePart {
    NamePart::Stem
}

impl Default for RenameRule {
    fn default() -> Self {
        Self {
            find: String::new(),
            replace: String::new(),
            use_regex: false,
            ignore_case: false,
            case: CaseTransform::None,
            part: NamePart::Stem,
            counter_start: 1,
        }
    }
}

/// What will happen to one file, and whether it is safe.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RenameStatus {
    /// Safe to apply.
    Ok,
    /// The rule produced the name the file already has. Skipped rather
    /// than treated as an error — a batch usually has a few of these.
    Unchanged,
    /// The derived name cannot be a file name on this system.
    Invalid,
    /// Two or more selected files would end up with the same name.
    Collision,
    /// A different file already sits at the destination. Never
    /// overwritten — the whole batch is refused instead.
    Exists,
}

impl RenameStatus {
    /// Whether this row would actually be renamed.
    pub fn will_apply(self) -> bool {
        self == RenameStatus::Ok
    }
    /// Whether this row blocks the batch.
    pub fn is_blocking(self) -> bool {
        matches!(
            self,
            RenameStatus::Invalid | RenameStatus::Collision | RenameStatus::Exists
        )
    }
}

/// One row of the preview table.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RenameItem {
    /// Absolute source path, exactly as the daemon returned it.
    pub from: String,
    /// The file's current name.
    pub from_name: String,
    /// The derived name — a bare file name, never a path.
    pub to_name: String,
    pub status: RenameStatus,
    /// Machine-readable reason when `status` is `Invalid`; the UI maps
    /// it to a localized message.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub reason: Option<InvalidReason>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum InvalidReason {
    Empty,
    /// Contains `/` or `\` — an attempt to leave the source directory.
    PathSeparator,
    /// `.` or `..`.
    DotName,
    /// A character the platform forbids in a file name.
    ForbiddenCharacter,
    /// A Windows reserved device name (`CON`, `PRN`, `LPT1`, …).
    ReservedName,
    /// The `find` pattern is not a valid regex.
    BadPattern,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RenamePlan {
    pub items: Vec<RenameItem>,
}

impl RenamePlan {
    pub fn will_apply_count(&self) -> usize {
        self.items.iter().filter(|i| i.status.will_apply()).count()
    }

    /// True when any row would block the batch. The UI disables Apply on
    /// this; the backend refuses on it too, so a stale preview cannot
    /// talk it into a partial rename.
    pub fn has_blocking(&self) -> bool {
        self.items.iter().any(|i| i.status.is_blocking())
    }
}

/// Build the plan. `paths` are absolute source paths in the order the
/// user sees them — `{n}` counts in that order, so the preview and the
/// apply agree.
pub fn plan(paths: &[String], rule: &RenameRule) -> RenamePlan {
    let matcher = Matcher::build(rule);

    let mut items: Vec<RenameItem> = Vec::with_capacity(paths.len());
    for (i, from) in paths.iter().enumerate() {
        let from_name = file_name_of(from);
        let counter = rule.counter_start.saturating_add(i as u32);

        let (to_name, reason) = match &matcher {
            Err(_) => (from_name.clone(), Some(InvalidReason::BadPattern)),
            Ok(m) => {
                let n = derive_name(&from_name, rule, m, counter);
                let r = validate_name(&n);
                (n, r)
            }
        };

        let status = if reason.is_some() {
            RenameStatus::Invalid
        } else if to_name == from_name {
            RenameStatus::Unchanged
        } else {
            RenameStatus::Ok
        };

        items.push(RenameItem {
            from: from.clone(),
            from_name,
            to_name,
            status,
            reason,
        });
    }

    mark_collisions(&mut items);
    RenamePlan { items }
}

/// Two files in the same directory that would end up sharing a name are
/// both marked — the user has to see both halves of the clash, not just
/// the second one.
///
/// Directory-scoped, because a bulk rename can span folders and
/// `a/report.txt` and `b/report.txt` do not collide.
fn mark_collisions(items: &mut [RenameItem]) {
    use std::collections::HashMap;
    let mut seen: HashMap<(String, String), Vec<usize>> = HashMap::new();
    for (i, it) in items.iter().enumerate() {
        // Invalid rows are already blocking; `Collision` is what this
        // function assigns and `Exists` is assigned later, so at this
        // point `is_blocking` means exactly "invalid".
        if it.status.is_blocking() {
            continue;
        }
        let dir = parent_of(&it.from);
        let key = (dir, fold_for_comparison(&it.to_name));
        seen.entry(key).or_default().push(i);
    }
    for (_, group) in seen {
        if group.len() > 1 {
            for i in group {
                items[i].status = RenameStatus::Collision;
                items[i].reason = None;
            }
        }
    }
}

/// Name comparison for collision purposes. Windows and macOS both treat
/// file names case-insensitively by default, so `Report.txt` and
/// `report.txt` are a clash there. Folding unconditionally is the safe
/// direction: on a case-sensitive filesystem it reports a collision that
/// technically is not one, which costs the user a rename they can redo —
/// the other way silently destroys a file.
fn fold_for_comparison(name: &str) -> String {
    name.to_lowercase()
}

enum Matcher {
    Literal {
        find: String,
        ignore_case: bool,
    },
    Regex(regex::Regex),
    /// Empty `find` — nothing is replaced.
    Passthrough,
}

impl Matcher {
    fn build(rule: &RenameRule) -> Result<Self, InvalidReason> {
        if rule.find.is_empty() {
            return Ok(Matcher::Passthrough);
        }
        if rule.use_regex {
            regex::RegexBuilder::new(&rule.find)
                .case_insensitive(rule.ignore_case)
                .size_limit(1 << 20)
                .build()
                .map(Matcher::Regex)
                .map_err(|_| InvalidReason::BadPattern)
        } else {
            Ok(Matcher::Literal {
                find: rule.find.clone(),
                ignore_case: rule.ignore_case,
            })
        }
    }
}

fn derive_name(from_name: &str, rule: &RenameRule, m: &Matcher, counter: u32) -> String {
    let (stem, ext) = split_name(from_name);
    let target = match rule.part {
        NamePart::Stem => stem,
        NamePart::Full => from_name,
    };

    // The counter is expanded into the *replacement text only*, never
    // over the finished name. Expanding afterwards would rewrite a
    // literal `{n}` that was already part of the file's own name, so a
    // folder of templates (`invoice_{n}.pdf`) would be silently
    // renumbered by a rule that matches nothing at all.
    let replacement = expand_counter(&rule.replace, counter);
    let replaced = match m {
        Matcher::Passthrough => target.to_string(),
        Matcher::Literal { find, ignore_case } => {
            replace_literal(target, find, &replacement, *ignore_case)
        }
        // `$1` capture references are expanded by the regex crate.
        Matcher::Regex(re) => re.replace_all(target, replacement.as_str()).into_owned(),
    };

    let cased = apply_case(&replaced, rule.case);

    match rule.part {
        // Re-attach the extension the rule was never allowed to touch.
        NamePart::Stem => match ext {
            Some(e) => format!("{cased}.{e}"),
            None => cased,
        },
        NamePart::Full => cased,
    }
}

/// Split into (stem, extension). A leading dot is part of the stem, so
/// `.gitignore` is a hidden file with no extension rather than an empty
/// name with the extension `gitignore`.
fn split_name(name: &str) -> (&str, Option<&str>) {
    match name.rfind('.') {
        Some(i) if i > 0 && i + 1 < name.len() => (&name[..i], Some(&name[i + 1..])),
        _ => (name, None),
    }
}

fn replace_literal(haystack: &str, find: &str, replace: &str, ignore_case: bool) -> String {
    if !ignore_case {
        return haystack.replace(find, replace);
    }
    // Case-insensitive literal replace, walking the original bytes so
    // the untouched parts keep their original casing.
    let hay_l = haystack.to_lowercase();
    let find_l = find.to_lowercase();
    if find_l.is_empty() {
        return haystack.to_string();
    }
    let mut out = String::with_capacity(haystack.len());
    let mut cursor = 0usize;
    // Lowercasing can change byte lengths (e.g. `İ`), which would make
    // offsets from the folded string unsafe to slice the original with.
    // Fall back to the exact-case replace in that case rather than
    // risking a panic or a mangled name.
    if hay_l.len() != haystack.len() || find_l.len() != find.len() {
        return haystack.replace(find, replace);
    }
    while let Some(rel) = hay_l[cursor..].find(&find_l) {
        let at = cursor + rel;
        out.push_str(&haystack[cursor..at]);
        out.push_str(replace);
        cursor = at + find_l.len();
    }
    out.push_str(&haystack[cursor..]);
    out
}

/// Expand `{n}` and `{n:0N}` counter tokens.
fn expand_counter(s: &str, counter: u32) -> String {
    if !s.contains("{n") {
        return s.to_string();
    }
    let mut out = String::with_capacity(s.len());
    let bytes = s.as_bytes();
    let mut i = 0usize;
    while i < bytes.len() {
        if bytes[i] == b'{' {
            if let Some(close) = s[i..].find('}') {
                let token = &s[i + 1..i + close];
                if let Some(width) = parse_counter_token(token) {
                    out.push_str(&format!("{counter:0width$}"));
                    i += close + 1;
                    continue;
                }
            }
        }
        // Not a counter token — copy the character whole so multi-byte
        // codepoints survive.
        let ch_len = s[i..].chars().next().map(|c| c.len_utf8()).unwrap_or(1);
        out.push_str(&s[i..i + ch_len]);
        i += ch_len;
    }
    out
}

/// `n` → width 1, `n:03` → width 3. `None` for anything else, so
/// `{name}` in a replacement stays literal text.
fn parse_counter_token(token: &str) -> Option<usize> {
    if token == "n" {
        return Some(1);
    }
    let rest = token.strip_prefix("n:")?;
    if rest.is_empty() || !rest.bytes().all(|b| b.is_ascii_digit()) {
        return None;
    }
    // `03` means pad to 3. Cap the width so `{n:999999999}` cannot be
    // used to allocate an enormous string.
    Some(rest.parse::<usize>().ok()?.min(32))
}

fn apply_case(s: &str, case: CaseTransform) -> String {
    match case {
        CaseTransform::None => s.to_string(),
        CaseTransform::Lower => s.to_lowercase(),
        CaseTransform::Upper => s.to_uppercase(),
        CaseTransform::Title => {
            let mut out = String::with_capacity(s.len());
            let mut at_word_start = true;
            for c in s.chars() {
                if at_word_start {
                    out.extend(c.to_uppercase());
                } else {
                    out.extend(c.to_lowercase());
                }
                at_word_start = !c.is_alphanumeric();
            }
            out
        }
    }
}

/// `None` when the name is usable. This is the security-critical half of
/// the module: it is what stops a derived name from addressing anything
/// outside the source file's own directory.
pub fn validate_name(name: &str) -> Option<InvalidReason> {
    if name.trim().is_empty() {
        return Some(InvalidReason::Empty);
    }
    if name.contains('/') || name.contains('\\') {
        return Some(InvalidReason::PathSeparator);
    }
    if name == "." || name == ".." {
        return Some(InvalidReason::DotName);
    }
    // A colon would create an NTFS alternate data stream, and a drive
    // prefix (`C:`) is a path, not a name. The rest are rejected by
    // Windows outright; rejecting them everywhere keeps a batch that
    // works on Linux from breaking when the same drive is read on
    // Windows.
    if name.contains(['<', '>', ':', '"', '|', '?', '*']) || name.contains('\0') {
        return Some(InvalidReason::ForbiddenCharacter);
    }
    if name.chars().any(|c| (c as u32) < 0x20) {
        return Some(InvalidReason::ForbiddenCharacter);
    }
    // Windows refuses these regardless of extension, and a trailing dot
    // or space is silently stripped there — which would make the applied
    // name differ from the previewed one.
    if name.ends_with('.') || name.ends_with(' ') {
        return Some(InvalidReason::ForbiddenCharacter);
    }
    let stem_upper = split_name(name).0.to_uppercase();
    const RESERVED: &[&str] = &[
        "CON", "PRN", "AUX", "NUL", "COM1", "COM2", "COM3", "COM4", "COM5", "COM6", "COM7", "COM8",
        "COM9", "LPT1", "LPT2", "LPT3", "LPT4", "LPT5", "LPT6", "LPT7", "LPT8", "LPT9",
    ];
    if RESERVED.contains(&stem_upper.as_str()) {
        return Some(InvalidReason::ReservedName);
    }
    None
}

// ---------- applying a plan ----------
//
// The two functions below are the only part of the rename path that
// touches the disk. They live here rather than in the Tauri app because
// `apps/freally-ui/src-tauri` is excluded from the cargo workspace, so
// anything defined there cannot be reached by `cargo test --workspace` —
// and a rollback rule that is only covered by a re-typed copy of itself
// is not covered at all. The app keeps what it uniquely owns: the
// provenance gate and the daemon RPC.

/// Absolute destination for one row: the source's own parent joined with
/// a validated bare name.
///
/// This is the last line of defence against a derived name escaping the
/// directory it came from — see the module docs. It is deliberately not
/// given the destination by its caller.
pub fn destination_for(from: &str, to_name: &str) -> Result<PathBuf, RenameApplyError> {
    if validate_name(to_name).is_some() {
        return Err(RenameApplyError::InvalidName);
    }
    let parent = Path::new(from).parent().ok_or(RenameApplyError::NoParent)?;
    let dest = parent.join(to_name);
    // Joining a validated bare name onto the parent cannot escape it;
    // assert the invariant rather than assume it.
    if dest.parent() != Some(parent) {
        return Err(RenameApplyError::EscapesDirectory);
    }
    Ok(dest)
}

#[derive(Debug, thiserror::Error, PartialEq, Eq)]
pub enum RenameApplyError {
    #[error("derived name is not a valid file name")]
    InvalidName,
    #[error("source path has no parent directory")]
    NoParent,
    #[error("derived name would leave the source directory")]
    EscapesDirectory,
    #[error("rename failed, batch rolled back: {0}")]
    Failed(String),
    /// The batch failed *and* undoing what had already moved failed too,
    /// so some files are sitting under their new names with no journal
    /// entry. Named explicitly because telling the user "nothing
    /// changed" here would leave them unable to find those files.
    #[error("rename failed ({error}), and {} file(s) could not be moved back: {}", stranded.len(), stranded.join(", "))]
    PartiallyRolledBack {
        error: String,
        stranded: Vec<String>,
    },
}

/// True when both paths name the same file on disk — which is what a
/// case-only rename looks like on Windows and macOS, where the
/// filesystem is case-insensitive.
///
/// Needed because "does the destination already exist?" and "would this
/// rename destroy something?" are different questions: renaming
/// `report.txt` to `REPORT.txt` finds an existing destination that *is*
/// the source, and refusing it would make every case transform
/// impossible on the two platforms most users are on.
pub fn is_same_file(a: &Path, b: &Path) -> bool {
    match (std::fs::canonicalize(a), std::fs::canonicalize(b)) {
        (Ok(a), Ok(b)) => a == b,
        _ => false,
    }
}

/// `std::fs::rename` that refuses to replace an existing destination.
///
/// The standard library's rename maps to `MOVEFILE_REPLACE_EXISTING` on
/// Windows and plain `rename(2)` on Unix, both of which **silently
/// destroy** whatever is at the destination. Every guard above this is a
/// `try_exists` check, and a check is not a guarantee: between the check
/// and the move, a sync client, an editor, or the user's own file
/// manager can create that name. This is the only place the guarantee
/// can actually be made, because only the kernel can make it atomically.
fn rename_no_replace(from: &Path, to: &Path) -> std::io::Result<()> {
    #[cfg(windows)]
    {
        use std::os::windows::ffi::OsStrExt;
        use windows::Win32::Storage::FileSystem::{MOVE_FILE_FLAGS, MoveFileExW};
        use windows::core::PCWSTR;

        let wide = |p: &Path| -> Vec<u16> { p.as_os_str().encode_wide().chain(Some(0)).collect() };
        let (f, t) = (wide(from), wide(to));
        // No MOVEFILE_REPLACE_EXISTING: the call fails with
        // ERROR_ALREADY_EXISTS rather than clobbering.
        unsafe { MoveFileExW(PCWSTR(f.as_ptr()), PCWSTR(t.as_ptr()), MOVE_FILE_FLAGS(0)) }
            .map_err(|e| std::io::Error::other(e.to_string()))
    }
    #[cfg(target_os = "linux")]
    {
        use std::ffi::CString;
        use std::os::unix::ffi::OsStrExt;
        let c = |p: &Path| CString::new(p.as_os_str().as_bytes()).ok();
        let (Some(f), Some(t)) = (c(from), c(to)) else {
            return Err(std::io::Error::other("path contains an interior NUL"));
        };
        // RENAME_NOREPLACE (1) needs Linux 3.15+ and filesystem support.
        let rc = unsafe {
            libc::syscall(
                libc::SYS_renameat2,
                libc::AT_FDCWD,
                f.as_ptr(),
                libc::AT_FDCWD,
                t.as_ptr(),
                1u32,
            )
        };
        if rc == 0 {
            return Ok(());
        }
        let err = std::io::Error::last_os_error();
        match err.raw_os_error() {
            // Old kernel or a filesystem without the flag. Fall back to
            // the racy rename rather than refusing to work at all — the
            // caller's existence check still covers the common case.
            Some(libc::ENOSYS) | Some(libc::EINVAL) | Some(libc::ENOTTY) => {
                std::fs::rename(from, to)
            }
            _ => Err(err),
        }
    }
    #[cfg(target_os = "macos")]
    {
        use std::ffi::CString;
        use std::os::unix::ffi::OsStrExt;
        let c = |p: &Path| CString::new(p.as_os_str().as_bytes()).ok();
        let (Some(f), Some(t)) = (c(from), c(to)) else {
            return Err(std::io::Error::other("path contains an interior NUL"));
        };
        // RENAME_EXCL == 0x04.
        let rc = unsafe { libc::renamex_np(f.as_ptr(), t.as_ptr(), 0x04) };
        if rc == 0 {
            return Ok(());
        }
        let err = std::io::Error::last_os_error();
        match err.raw_os_error() {
            Some(libc::ENOTSUP) | Some(libc::EINVAL) => std::fs::rename(from, to),
            _ => Err(err),
        }
    }
    #[cfg(not(any(windows, target_os = "linux", target_os = "macos")))]
    {
        std::fs::rename(from, to)
    }
}

/// Perform the moves, rolling back on the first failure.
///
/// A bulk rename that dies halfway leaves the user with a directory in
/// two naming schemes and no record of which files moved. The rollback
/// is best-effort, and when it *also* fails the error says so rather
/// than claiming a clean rollback — a caller that is told "nothing
/// changed" when files did move has no way to find them again.
pub fn perform_moves(moves: &[(PathBuf, PathBuf)]) -> Result<usize, RenameApplyError> {
    let mut done: Vec<&(PathBuf, PathBuf)> = Vec::with_capacity(moves.len());
    for pair in moves {
        // A case-only rename has a destination that *is* the source, so
        // the no-replace call would refuse it. Nothing can be destroyed
        // in that case by definition.
        let res = if is_same_file(&pair.0, &pair.1) {
            std::fs::rename(&pair.0, &pair.1)
        } else {
            rename_no_replace(&pair.0, &pair.1)
        };
        match res {
            Ok(()) => done.push(pair),
            Err(e) => {
                let mut stranded = Vec::new();
                for (undo_from, undo_to) in done.iter().rev() {
                    if std::fs::rename(undo_to, undo_from).is_err() {
                        stranded.push(undo_to.to_string_lossy().into_owned());
                    }
                }
                return Err(if stranded.is_empty() {
                    RenameApplyError::Failed(e.to_string())
                } else {
                    RenameApplyError::PartiallyRolledBack {
                        error: e.to_string(),
                        stranded,
                    }
                });
            }
        }
    }
    Ok(done.len())
}

fn file_name_of(path: &str) -> String {
    let trimmed = path.trim_end_matches(['/', '\\']);
    match trimmed.rfind(['/', '\\']) {
        Some(i) => trimmed[i + 1..].to_string(),
        None => trimmed.to_string(),
    }
}

fn parent_of(path: &str) -> String {
    let trimmed = path.trim_end_matches(['/', '\\']);
    match trimmed.rfind(['/', '\\']) {
        Some(i) => trimmed[..i].to_string(),
        None => String::new(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn rule() -> RenameRule {
        RenameRule::default()
    }

    fn names(plan: &RenamePlan) -> Vec<String> {
        plan.items.iter().map(|i| i.to_name.clone()).collect()
    }

    #[test]
    fn a_literal_replace_leaves_the_extension_alone() {
        let r = RenameRule {
            find: "draft".into(),
            replace: "final".into(),
            ..rule()
        };
        let p = plan(&["/a/draft-report.draft".into()], &r);
        // The `.draft` extension is not the stem and must survive.
        assert_eq!(names(&p), vec!["final-report.draft"]);
    }

    #[test]
    fn renaming_the_full_name_can_change_the_extension() {
        let r = RenameRule {
            find: "draft".into(),
            replace: "final".into(),
            part: NamePart::Full,
            ..rule()
        };
        let p = plan(&["/a/draft-report.draft".into()], &r);
        assert_eq!(names(&p), vec!["final-report.final"]);
    }

    #[test]
    fn regex_captures_are_expanded() {
        let r = RenameRule {
            find: r"(\d{4})-(\d{2})-(\d{2})".into(),
            replace: "$3.$2.$1".into(),
            use_regex: true,
            ..rule()
        };
        let p = plan(&["/a/2026-08-02-notes.txt".into()], &r);
        assert_eq!(names(&p), vec!["02.08.2026-notes.txt"]);
    }

    #[test]
    fn an_invalid_regex_is_reported_not_panicked() {
        let r = RenameRule {
            find: "(unclosed".into(),
            use_regex: true,
            ..rule()
        };
        let p = plan(&["/a/x.txt".into()], &r);
        assert_eq!(p.items[0].status, RenameStatus::Invalid);
        assert_eq!(p.items[0].reason, Some(InvalidReason::BadPattern));
        assert!(p.has_blocking());
    }

    #[test]
    fn counters_advance_in_the_order_the_user_sees() {
        let r = RenameRule {
            find: "img".into(),
            replace: "photo-{n:03}".into(),
            counter_start: 1,
            ..rule()
        };
        let p = plan(
            &[
                "/a/img1.jpg".into(),
                "/a/img2.jpg".into(),
                "/a/img3.jpg".into(),
            ],
            &r,
        );
        assert_eq!(
            names(&p),
            vec!["photo-0011.jpg", "photo-0022.jpg", "photo-0033.jpg"]
        );
    }

    #[test]
    fn a_bare_counter_token_has_no_padding() {
        let r = RenameRule {
            find: "x".into(),
            replace: "{n}".into(),
            counter_start: 9,
            ..rule()
        };
        let p = plan(&["/a/x.txt".into(), "/a/x2.txt".into()], &r);
        assert_eq!(names(&p), vec!["9.txt", "102.txt"]);
    }

    #[test]
    fn a_counter_token_already_in_the_file_name_is_left_alone() {
        // A folder of templates (`invoice_{n}.pdf`) must survive a rule
        // that does not target them. Expanding the counter over the
        // finished name renumbered every one of them, under a rule that
        // matches nothing at all.
        // (A padded token like `{n:04}` cannot appear in a real file
        // name — the colon is rejected outright — so the bare form is
        // the only one that can actually sit on disk.)
        let p = plan(
            &["/d/invoice_{n}.pdf".into(), "/d/label_{n}.svg".into()],
            &rule(),
        );
        assert_eq!(names(&p), vec!["invoice_{n}.pdf", "label_{n}.svg"]);
        assert!(p.items.iter().all(|i| i.status == RenameStatus::Unchanged));
    }

    #[test]
    fn a_counter_in_the_replacement_still_expands_over_such_a_name() {
        let r = RenameRule {
            find: "invoice".into(),
            replace: "bill-{n}".into(),
            counter_start: 7,
            ..rule()
        };
        // The literal `{n}` from the original name survives; only the
        // one the rule introduced is numbered.
        assert_eq!(
            names(&plan(&["/d/invoice_{n}.pdf".into()], &r)),
            vec!["bill-7_{n}.pdf"]
        );
    }

    #[test]
    fn a_non_counter_brace_token_stays_literal() {
        let r = RenameRule {
            find: "x".into(),
            replace: "{name}".into(),
            ..rule()
        };
        assert_eq!(names(&plan(&["/a/x.txt".into()], &r)), vec!["{name}.txt"]);
    }

    #[test]
    fn case_transforms_apply_to_the_chosen_part_only() {
        let r = RenameRule {
            case: CaseTransform::Upper,
            ..rule()
        };
        // Stem is uppercased; the extension is re-attached untouched.
        assert_eq!(
            names(&plan(&["/a/report.TxT".into()], &r)),
            vec!["REPORT.TxT"]
        );
    }

    #[test]
    fn title_case_capitalizes_each_word() {
        let r = RenameRule {
            case: CaseTransform::Title,
            ..rule()
        };
        assert_eq!(
            names(&plan(&["/a/my_holiday photo-2.jpg".into()], &r)),
            vec!["My_Holiday Photo-2.jpg"]
        );
    }

    #[test]
    fn an_unchanged_name_is_skipped_rather_than_applied() {
        let r = RenameRule {
            find: "nothing-matches".into(),
            replace: "x".into(),
            ..rule()
        };
        let p = plan(&["/a/report.txt".into()], &r);
        assert_eq!(p.items[0].status, RenameStatus::Unchanged);
        assert_eq!(p.will_apply_count(), 0);
        assert!(!p.has_blocking(), "a no-op is not an error");
    }

    #[test]
    fn both_halves_of_a_collision_are_marked() {
        let r = RenameRule {
            find: r"[ab]".into(),
            replace: "same".into(),
            use_regex: true,
            ..rule()
        };
        let p = plan(&["/dir/a.txt".into(), "/dir/b.txt".into()], &r);
        assert!(p.items.iter().all(|i| i.status == RenameStatus::Collision));
        assert!(p.has_blocking());
    }

    #[test]
    fn a_collision_is_scoped_to_one_directory() {
        let r = RenameRule {
            find: "x".into(),
            replace: "same".into(),
            ..rule()
        };
        // Same resulting name, different folders — not a clash.
        let p = plan(&["/one/x.txt".into(), "/two/x.txt".into()], &r);
        assert!(p.items.iter().all(|i| i.status == RenameStatus::Ok));
        assert!(!p.has_blocking());
    }

    #[test]
    fn a_case_only_difference_still_counts_as_a_collision() {
        // On Windows and macOS these are the same file. Folding is the
        // safe direction: worst case the user redoes a rename, versus
        // silently destroying a file.
        let r = RenameRule {
            find: "b".into(),
            replace: "A".into(),
            ..rule()
        };
        let p = plan(&["/dir/a.txt".into(), "/dir/b.txt".into()], &r);
        assert!(p.items.iter().all(|i| i.status == RenameStatus::Collision));
    }

    // ---- the security-critical half ----

    #[test]
    fn a_derived_name_can_never_contain_a_path_separator() {
        for replacement in ["../../etc/passwd", "sub/dir", "sub\\dir"] {
            let r = RenameRule {
                find: "x".into(),
                replace: replacement.into(),
                part: NamePart::Full,
                ..rule()
            };
            let p = plan(&["/a/x".into()], &r);
            assert_eq!(
                p.items[0].status,
                RenameStatus::Invalid,
                "{replacement} was accepted"
            );
            assert!(p.has_blocking());
        }
    }

    #[test]
    fn dot_names_are_refused() {
        for name in [".", ".."] {
            assert_eq!(validate_name(name), Some(InvalidReason::DotName));
        }
    }

    #[test]
    fn characters_no_platform_should_accept_are_refused() {
        for name in ["a:b.txt", "a<b", "a>b", "a\"b", "a|b", "a?b", "a*b"] {
            assert_eq!(
                validate_name(name),
                Some(InvalidReason::ForbiddenCharacter),
                "{name} was accepted"
            );
        }
        assert_eq!(
            validate_name("a\u{1}b"),
            Some(InvalidReason::ForbiddenCharacter)
        );
    }

    #[test]
    fn a_name_windows_would_silently_rewrite_is_refused() {
        // Windows strips these, so the applied name would differ from
        // the previewed one — the user would approve one thing and get
        // another.
        assert_eq!(
            validate_name("report."),
            Some(InvalidReason::ForbiddenCharacter)
        );
        assert_eq!(
            validate_name("report "),
            Some(InvalidReason::ForbiddenCharacter)
        );
    }

    #[test]
    fn windows_device_names_are_refused_with_or_without_an_extension() {
        assert_eq!(validate_name("CON"), Some(InvalidReason::ReservedName));
        assert_eq!(validate_name("con.txt"), Some(InvalidReason::ReservedName));
        assert_eq!(validate_name("LPT9.log"), Some(InvalidReason::ReservedName));
        assert_eq!(validate_name("CONSOLE.txt"), None, "only exact matches");
    }

    #[test]
    fn an_empty_or_whitespace_name_is_refused() {
        assert_eq!(validate_name(""), Some(InvalidReason::Empty));
        assert_eq!(validate_name("   "), Some(InvalidReason::Empty));
        let r = RenameRule {
            find: "x".into(),
            replace: "".into(),
            part: NamePart::Full,
            ..rule()
        };
        assert_eq!(
            plan(&["/a/x".into()], &r).items[0].status,
            RenameStatus::Invalid
        );
    }

    #[test]
    fn a_hidden_file_keeps_its_leading_dot_as_part_of_the_stem() {
        let r = RenameRule {
            find: "git".into(),
            replace: "hg".into(),
            ..rule()
        };
        assert_eq!(
            names(&plan(&["/a/.gitignore".into()], &r)),
            vec![".hgignore"]
        );
    }

    #[test]
    fn case_insensitive_literal_replace_keeps_surrounding_case() {
        let r = RenameRule {
            find: "draft".into(),
            replace: "FINAL".into(),
            ignore_case: true,
            ..rule()
        };
        assert_eq!(
            names(&plan(&["/a/MyDRAFTReport.txt".into()], &r)),
            vec!["MyFINALReport.txt"]
        );
    }

    #[test]
    fn a_padding_width_cannot_be_used_to_allocate_absurd_strings() {
        let r = RenameRule {
            find: "x".into(),
            replace: "{n:999999999}".into(),
            ..rule()
        };
        let out = names(&plan(&["/a/x.txt".into()], &r));
        assert!(out[0].len() < 64, "width was not capped: {}", out[0].len());
    }

    #[test]
    fn windows_source_paths_are_split_correctly() {
        let r = RenameRule {
            find: "a".into(),
            replace: "b".into(),
            ..rule()
        };
        let p = plan(&[r"C:\Users\me\a.txt".into()], &r);
        assert_eq!(p.items[0].from_name, "a.txt");
        assert_eq!(p.items[0].to_name, "b.txt");
    }
}
