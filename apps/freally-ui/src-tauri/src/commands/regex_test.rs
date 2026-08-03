//! SRC-M20 — compile a candidate regex and report what it matches.
//!
//! The point of doing this in Rust rather than with the webview's own
//! `RegExp` is fidelity: the query executor runs the `regex` crate,
//! whose flavour differs from JavaScript's in ways that matter to
//! exactly the person composing a pattern. It has no backreferences and
//! no lookaround (and says so, rather than silently meaning something
//! else); `\d` is Unicode-aware by default; and `(?i)` is an inline flag
//! rather than a trailing one. A builder that validated against
//! `RegExp` would happily accept `(?<=foo)bar` and then fail when the
//! query ran.
//!
//! The crate's engine is finite-automaton based, so a hostile pattern
//! cannot backtrack exponentially — no ReDoS. What it *can* do is
//! compile to a very large automaton, so the size limit is pinned well
//! below the default.

use serde::{Deserialize, Serialize};

/// Cap on the compiled automaton. The default is 10 MB; a filename
/// pattern needs a tiny fraction of that, and this keeps a pathological
/// paste from allocating for a keystroke.
const SIZE_LIMIT_BYTES: usize = 256 * 1024;

/// Longest pattern accepted. Beyond this the user is pasting, not
/// composing, and the popover has nowhere to show it.
const MAX_PATTERN_CHARS: usize = 1024;

/// Most names tested. The feature line asks for the top 50; the extra
/// headroom lets the caller pass a slightly larger window without the
/// command silently truncating something it was asked to check.
const MAX_SUBJECTS: usize = 200;

/// One matched span, as character offsets into the subject.
///
/// Characters, not bytes: the UI slices these with JavaScript string
/// indices, and a byte offset would cut a multi-byte name in half.
#[derive(Debug, Serialize)]
pub struct MatchSpan {
    pub start: usize,
    pub end: usize,
}

#[derive(Debug, Serialize)]
pub struct SubjectMatches {
    /// Index into the `subjects` array the caller passed.
    pub index: usize,
    pub spans: Vec<MatchSpan>,
}

#[derive(Debug, Serialize)]
pub struct RegexTestResult {
    pub valid: bool,
    /// Compile error, verbatim from the engine — it is already a good
    /// error, with a caret pointing at the offending construct.
    pub error: Option<String>,
    /// Only subjects with at least one match, so the UI can show a
    /// "3 of 50 match" count without walking the whole list.
    pub matches: Vec<SubjectMatches>,
}

#[derive(Debug, Deserialize)]
pub struct RegexTestArgs {
    pub pattern: String,
    pub subjects: Vec<String>,
    /// Mirrors `Search → Match Case`. Off means the pattern is compiled
    /// case-insensitively, which is what the executor does too.
    #[serde(default)]
    pub match_case: bool,
}

#[tauri::command]
pub fn regex_test(args: RegexTestArgs) -> RegexTestResult {
    if args.pattern.chars().count() > MAX_PATTERN_CHARS {
        return RegexTestResult {
            valid: false,
            error: Some(format!(
                "pattern is longer than {MAX_PATTERN_CHARS} characters"
            )),
            matches: Vec::new(),
        };
    }
    // An empty pattern compiles fine and matches everything, which is a
    // useless and alarming thing to show while the user is still
    // typing the first character.
    if args.pattern.is_empty() {
        return RegexTestResult {
            valid: false,
            error: None,
            matches: Vec::new(),
        };
    }

    let compiled = regex::RegexBuilder::new(&args.pattern)
        .case_insensitive(!args.match_case)
        .size_limit(SIZE_LIMIT_BYTES)
        .build();
    let re = match compiled {
        Ok(re) => re,
        Err(e) => {
            return RegexTestResult {
                valid: false,
                error: Some(e.to_string()),
                matches: Vec::new(),
            };
        }
    };

    let mut matches = Vec::new();
    for (index, subject) in args.subjects.iter().take(MAX_SUBJECTS).enumerate() {
        let spans: Vec<MatchSpan> = re
            .find_iter(subject)
            .map(|m| MatchSpan {
                start: char_offset(subject, m.start()),
                end: char_offset(subject, m.end()),
            })
            // A zero-width pattern like `a*` matches at every position;
            // rendering those as highlights is noise, not information.
            .filter(|s| s.end > s.start)
            .collect();
        if !spans.is_empty() {
            matches.push(SubjectMatches { index, spans });
        }
    }

    RegexTestResult {
        valid: true,
        error: None,
        matches,
    }
}

/// Byte offset → character offset.
fn char_offset(s: &str, byte: usize) -> usize {
    s[..byte].chars().count()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn run(pattern: &str, subjects: &[&str]) -> RegexTestResult {
        regex_test(RegexTestArgs {
            pattern: pattern.to_string(),
            subjects: subjects.iter().map(|s| s.to_string()).collect(),
            match_case: false,
        })
    }

    #[test]
    fn reports_spans_for_matching_subjects_only() {
        let r = run(r"report", &["report-final.md", "draft.md", "myreport.txt"]);
        assert!(r.valid);
        let indices: Vec<usize> = r.matches.iter().map(|m| m.index).collect();
        assert_eq!(indices, vec![0, 2]);
        assert_eq!(r.matches[0].spans[0].start, 0);
        assert_eq!(r.matches[0].spans[0].end, 6);
    }

    #[test]
    fn spans_are_character_offsets_not_byte_offsets() {
        // `é` is two bytes; a byte offset would put the highlight one
        // character too far right and split the name in the UI.
        let r = run(r"report", &["café-report.md"]);
        assert_eq!(r.matches[0].spans[0].start, 5);
        assert_eq!(r.matches[0].spans[0].end, 11);
    }

    #[test]
    fn an_invalid_pattern_reports_the_engines_own_error() {
        let r = run(r"report(", &["report.md"]);
        assert!(!r.valid);
        assert!(r.error.is_some());
        assert!(r.matches.is_empty());
    }

    #[test]
    fn lookaround_is_rejected_because_the_executor_would_reject_it() {
        // The whole reason this runs in Rust: JavaScript's RegExp
        // accepts this and the query engine does not.
        let r = run(r"(?<=foo)bar", &["foobar"]);
        assert!(!r.valid);
    }

    #[test]
    fn an_empty_pattern_is_not_reported_as_an_error() {
        let r = run("", &["anything"]);
        assert!(!r.valid);
        assert!(r.error.is_none(), "no error text while still typing");
    }

    #[test]
    fn zero_width_matches_are_not_rendered() {
        let r = run(r"x*", &["report.md"]);
        assert!(r.valid);
        assert!(r.matches.is_empty(), "empty spans would highlight nothing");
    }

    #[test]
    fn case_insensitive_by_default_and_sensitive_on_request() {
        assert_eq!(run(r"REPORT", &["report.md"]).matches.len(), 1);
        let sensitive = regex_test(RegexTestArgs {
            pattern: "REPORT".into(),
            subjects: vec!["report.md".into()],
            match_case: true,
        });
        assert!(sensitive.matches.is_empty());
    }
}
