//! Hit-in-context content viewer (SRC-M01).
//!
//! The preview pane shows one 200-character fragment. This shows the
//! whole extracted document with every match located, so the question
//! "which of the eleven places does it say that" has an answer without
//! leaving Freally.
//!
//! Extraction reuses the Phase-7/8 pipeline, so a PDF, a `.docx`, a
//! source file, and a plain `.txt` all arrive here as text the same
//! way the content lens indexes them — the viewer never re-implements
//! a format reader.

use freally_extractors::{Pipeline, TextSink, extractors::register_all};
use serde::Serialize;
use tauri::State;

use super::known_paths::{KnownPaths, Provenance};

/// Extraction cap for the viewer. The content lens indexes more than
/// this, but a document longer than 4 MB of text is not something a
/// human scrolls — and the whole thing crosses the IPC boundary.
const VIEWER_TEXT_CAP: usize = 4 * 1024 * 1024;

/// Lines rendered at most. Keeps a minified 30 MB single-line JSON or a
/// million-line log from locking up the webview.
const MAX_LINES: usize = 20_000;

/// Characters kept per line. Long lines are truncated for display; a
/// match beyond the cutoff still counts, it just isn't highlighted.
const MAX_LINE_CHARS: usize = 2_000;

/// One match, located by line and by character offset within that
/// line's *rendered* text.
#[derive(Debug, Clone, Serialize)]
pub struct HitSpan {
    /// Zero-based index into `ContentDocument::lines`.
    pub line: u32,
    /// Character (not byte) offsets, so the frontend can slice the
    /// string it received without re-decoding UTF-8.
    pub start: u32,
    pub end: u32,
}

#[derive(Debug, Clone, Serialize)]
pub struct ContentDocument {
    pub lines: Vec<String>,
    pub hits: Vec<HitSpan>,
    /// True when `lines` stops short of the document.
    pub truncated: bool,
    /// Extractor that produced the text, for the viewer's footer.
    pub extractor: String,
}

/// Extract `path` and locate every occurrence of every term.
///
/// `terms` come from the query's literal atoms. Matching is
/// case-insensitive substring — the same thing the inline snippet
/// highlights, so the viewer and the snippet never disagree about what
/// counts as a hit.
#[tauri::command]
pub async fn content_document(
    path: String,
    terms: Vec<String>,
    known: State<'_, KnownPaths>,
) -> Result<ContentDocument, String> {
    let target = known.verify(&path, Provenance::QueryHit)?;
    let p = target.as_path();
    let pipeline = register_all(Pipeline::builder()).build();
    let extractor = pipeline
        .dispatch_path(p)
        .ok_or_else(|| "no extractor handles this file type".to_string())?;
    let extractor_id = extractor.id().as_str().to_string();

    let mut sink = TextSink::new(VIEWER_TEXT_CAP);
    extractor
        .extract(p, &mut sink)
        .map_err(|e| format!("extraction failed: {e}"))?;
    let text = String::from_utf8_lossy(sink.as_bytes()).into_owned();

    let (lines, truncated) = split_lines(&text);
    let hits = locate(&lines, &terms);
    Ok(ContentDocument {
        lines,
        hits,
        truncated,
        extractor: extractor_id,
    })
}

/// Split into display lines, bounding both the count and each line's
/// length. Returns `(lines, truncated)`.
fn split_lines(text: &str) -> (Vec<String>, bool) {
    let mut lines: Vec<String> = Vec::new();
    let mut truncated = false;
    for raw in text.lines() {
        if lines.len() >= MAX_LINES {
            truncated = true;
            break;
        }
        if raw.chars().count() > MAX_LINE_CHARS {
            truncated = true;
            lines.push(raw.chars().take(MAX_LINE_CHARS).collect());
        } else {
            lines.push(raw.to_string());
        }
    }
    (lines, truncated)
}

/// Every occurrence of every term, ordered by position so `F3` walks
/// the document top-to-bottom regardless of which term matched.
///
/// Overlapping matches from different terms are kept separately — the
/// viewer renders them as two highlights, which is honest: both terms
/// really did match there.
fn locate(lines: &[String], terms: &[String]) -> Vec<HitSpan> {
    // Fold each needle per character, the same way the haystack is
    // compared below. Folding the needle as a *string* instead would
    // apply Unicode's final-sigma rule — `"ΟΔΟΣ".to_lowercase()` ends
    // in ς while a per-char fold yields σ — so a Greek term would never
    // match the line it was copied from.
    let needles: Vec<Vec<char>> = terms
        .iter()
        .map(|t| t.trim().chars().flat_map(char::to_lowercase).collect())
        .filter(|n: &Vec<char>| !n.is_empty())
        .collect();
    if needles.is_empty() {
        return Vec::new();
    }
    let mut out: Vec<HitSpan> = Vec::new();
    // Character offsets into the ORIGINAL line, which is what the
    // frontend slices. Searching a lowercased *copy* and reporting
    // offsets into it would drift on any line containing a character
    // whose lowercase form is longer — `İ` (U+0130) folds to two chars,
    // shifting every later highlight one column right.
    let mut haystack: Vec<char> = Vec::new();
    for (i, line) in lines.iter().enumerate() {
        haystack.clear();
        haystack.extend(line.chars());
        for needle in &needles {
            let mut from = 0usize;
            while let Some(rel) = find_folded(&haystack[from..], needle) {
                let start = from + rel;
                out.push(HitSpan {
                    line: i as u32,
                    start: start as u32,
                    end: (start + needle.len()) as u32,
                });
                // Advance one character so overlapping occurrences
                // ("aa" in "aaa") are all found.
                from = start + 1;
            }
        }
    }
    out.sort_by_key(|h| (h.line, h.start, h.end));
    merge_overlapping(out)
}

/// Index of the first case-insensitive occurrence of `needle` in
/// `haystack`, in characters.
///
/// Compares character by character with a per-char fold on the haystack
/// side, so the result indexes the original text directly — no parallel
/// lowercased buffer, and no offset mapping to get wrong.
///
/// Characters whose lowercase form is more than one character (only
/// `İ` in current Unicode) compare against their first folded char.
/// That is a deliberate simplification: it keeps offsets exact, and the
/// alternative — a folded copy — is the bug this replaced.
fn find_folded(haystack: &[char], needle: &[char]) -> Option<usize> {
    if needle.len() > haystack.len() {
        return None;
    }
    let folded = |c: char| c.to_lowercase().next().unwrap_or(c);
    (0..=haystack.len() - needle.len()).find(|&start| {
        haystack[start..start + needle.len()]
            .iter()
            .zip(needle)
            .all(|(h, n)| folded(*h) == *n)
    })
}

/// Fuse spans that overlap into one.
///
/// The viewer renders one `<mark>` per span and skips any span starting
/// inside the previous one, but the match-count badge and `F3` count
/// every span — so an unmerged overlap makes `F3` step to a highlight
/// that was never drawn. Merging keeps "match 3 of 7" and the seven
/// highlights the user can actually see in agreement.
fn merge_overlapping(spans: Vec<HitSpan>) -> Vec<HitSpan> {
    let mut out: Vec<HitSpan> = Vec::with_capacity(spans.len());
    for span in spans {
        match out.last_mut() {
            Some(prev) if prev.line == span.line && span.start < prev.end => {
                prev.end = prev.end.max(span.end);
            }
            _ => out.push(span),
        }
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;

    fn lines_of(s: &str) -> Vec<String> {
        s.lines().map(str::to_string).collect()
    }

    #[test]
    fn matches_are_case_insensitive_and_ordered() {
        let lines = lines_of("Report draft\nnothing here\nfinal REPORT\n");
        let hits = locate(&lines, &["report".into()]);
        assert_eq!(hits.len(), 2);
        assert_eq!((hits[0].line, hits[0].start, hits[0].end), (0, 0, 6));
        assert_eq!((hits[1].line, hits[1].start, hits[1].end), (2, 6, 12));
    }

    #[test]
    fn every_term_contributes_and_results_stay_sorted() {
        let lines = lines_of("alpha beta\nbeta alpha\n");
        let hits = locate(&lines, &["alpha".into(), "beta".into()]);
        let positions: Vec<(u32, u32)> = hits.iter().map(|h| (h.line, h.start)).collect();
        assert_eq!(positions, vec![(0, 0), (0, 6), (1, 0), (1, 5)]);
    }

    #[test]
    fn overlapping_occurrences_merge_into_one_span() {
        // `aa` occurs at 0, 1, and 2 — but they are one contiguous run
        // of highlight, and the badge must not promise three F3 stops
        // the viewer never draws.
        let lines = lines_of("aaaa");
        let hits = locate(&lines, &["aa".into()]);
        assert_eq!(hits.len(), 1);
        assert_eq!((hits[0].start, hits[0].end), (0, 4));
    }

    #[test]
    fn overlaps_from_different_terms_also_merge() {
        let lines = lines_of("final report");
        let hits = locate(&lines, &["report".into(), "rep".into()]);
        assert_eq!(hits.len(), 1, "one highlight, so one F3 stop");
        assert_eq!((hits[0].start, hits[0].end), (6, 12));
    }

    #[test]
    fn offsets_index_the_original_line_not_a_lowercased_copy() {
        // U+0130 lowercases to TWO characters, so searching a folded
        // copy would report `report` one column right of where it is.
        let lines = lines_of("\u{130}stanbul report");
        let hits = locate(&lines, &["report".into()]);
        assert_eq!(hits.len(), 1);
        assert_eq!((hits[0].start, hits[0].end), (9, 15));
        let chars: Vec<char> = lines[0].chars().collect();
        let slice: String = chars[hits[0].start as usize..hits[0].end as usize]
            .iter()
            .collect();
        assert_eq!(slice, "report", "the frontend slices exactly this");
    }

    #[test]
    fn greek_final_sigma_still_matches_the_line_it_came_from() {
        // A string-level fold applies the final-sigma rule and would
        // leave this term unable to match its own source line.
        let lines = lines_of("\u{39F}\u{394}\u{39F}\u{3A3}");
        let hits = locate(&lines, &["\u{39F}\u{394}\u{39F}\u{3A3}".into()]);
        assert_eq!(hits.len(), 1);
        assert_eq!((hits[0].start, hits[0].end), (0, 4));
    }

    #[test]
    fn offsets_are_character_based_not_byte_based() {
        // "héllo" is 6 bytes but 5 characters; a byte offset would put
        // the highlight one column right of where it belongs.
        let lines = lines_of("héllo world");
        let hits = locate(&lines, &["world".into()]);
        assert_eq!(hits.len(), 1);
        assert_eq!((hits[0].start, hits[0].end), (6, 11));
    }

    #[test]
    fn a_match_at_end_of_line_resolves_its_end_offset() {
        let lines = lines_of("find me");
        let hits = locate(&lines, &["me".into()]);
        assert_eq!((hits[0].start, hits[0].end), (5, 7));
    }

    #[test]
    fn empty_and_whitespace_terms_are_ignored() {
        let lines = lines_of("anything");
        assert!(locate(&lines, &[]).is_empty());
        assert!(locate(&lines, &["".into(), "   ".into()]).is_empty());
    }

    #[test]
    fn line_count_is_bounded_and_reported() {
        let text = "x\n".repeat(MAX_LINES + 50);
        let (lines, truncated) = split_lines(&text);
        assert_eq!(lines.len(), MAX_LINES);
        assert!(truncated);
    }

    #[test]
    fn a_very_long_line_is_clipped_and_reported() {
        let text = "y".repeat(MAX_LINE_CHARS + 10);
        let (lines, truncated) = split_lines(&text);
        assert_eq!(lines[0].chars().count(), MAX_LINE_CHARS);
        assert!(truncated);
    }

    #[test]
    fn a_short_document_is_not_reported_as_truncated() {
        let (lines, truncated) = split_lines("one\ntwo\n");
        assert_eq!(lines, vec!["one", "two"]);
        assert!(!truncated);
    }
}
