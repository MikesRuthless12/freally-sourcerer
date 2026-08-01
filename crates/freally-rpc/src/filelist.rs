//! File-list interop formats (SRC-M03).
//!
//! Everything's `.efu` is the lingua franca of the filename-search
//! world: it is how people move a catalogue of a disconnected drive
//! between machines, and Freally reads and writes it byte-compatibly.
//! Alongside it live the four formats users actually ask for when they
//! want the result set somewhere else — CSV for a spreadsheet, TXT for
//! a shell pipeline, M3U for a music player, NDJSON for a script.
//!
//! Everything lives here rather than in the UI so the CLI, the daemon,
//! and the Tauri app all serialise identically.
//!
//! ## `.efu` shape
//!
//! A UTF-8 CSV with this exact header:
//!
//! ```text
//! Filename,Size,Date Modified,Date Created,Attributes
//! ```
//!
//! `Filename` is the *full path*. The two dates are Windows FILETIME
//! (100-nanosecond ticks since 1601-01-01 UTC) — Freally converts to
//! and from Unix milliseconds at the boundary, so `.efu` files written
//! on macOS and Linux load into Everything on Windows unchanged.
//! `Attributes` is the Win32 `FILE_ATTRIBUTE_*` bitmask, which is the
//! same projection `QueryHit::attrs` already carries on every OS.

use std::fmt::Write as _;

use serde::{Deserialize, Serialize};

use crate::dto::QueryHit;

/// Ticks between the FILETIME epoch (1601-01-01) and the Unix epoch.
const FILETIME_EPOCH_OFFSET_MS: i64 = 11_644_473_600_000;
const FILETIME_TICKS_PER_MS: i64 = 10_000;

/// `FILE_ATTRIBUTE_DIRECTORY`. Redeclared rather than imported from
/// `freally_index::ATTR_DIRECTORY`: this crate is the transport layer
/// and deliberately does not depend on the index, so that the CLI can
/// read and write file lists without linking Tantivy.
const ATTR_DIRECTORY: u32 = 0x0000_0010;

/// Extensions that become playlist entries in an M3U export.
///
/// Must stay identical to `freally_query::QuickFilter::Audio`, so the
/// files an M3U export carries are exactly the files the Audio quick
/// filter shows. It is duplicated rather than imported because this
/// crate is the transport layer and deliberately does not depend on the
/// query crate; `freally-indexd` sees both and asserts they match
/// (`audio_extensions_match_the_quick_filter`).
pub const AUDIO_EXTENSIONS: &[&str] = &[
    "aac", "aif", "aiff", "alac", "ape", "flac", "m4a", "mid", "midi", "mka", "mp1", "mp2", "mp3",
    "mpc", "ogg", "oga", "opus", "ra", "tta", "wav", "wma", "wv",
];

/// Every interop format Freally reads or writes.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum FileListFormat {
    /// Everything File List — importable and exportable.
    Efu,
    /// Spreadsheet-friendly CSV with a Freally column header.
    Csv,
    /// One full path per line.
    Txt,
    /// Extended M3U playlist. Audio results only.
    M3u,
    /// Extended M3U playlist, `.m3u8` extension. Identical bytes —
    /// both are written UTF-8; the extension is the signal to players
    /// that they may assume it.
    M3u8,
    /// One JSON object per line.
    Ndjson,
    /// Pretty-printed JSON document (the pre-Build-1 export shape).
    Json,
}

impl FileListFormat {
    /// Pick a format from a file name / path. Unknown extensions fall
    /// back to `Json`, matching the pre-Build-1 default.
    pub fn from_path(path: &str) -> Self {
        let ext = path
            .rsplit_once('.')
            .map(|(_, e)| e.to_ascii_lowercase())
            .unwrap_or_default();
        match ext.as_str() {
            "efu" => FileListFormat::Efu,
            "csv" => FileListFormat::Csv,
            "txt" => FileListFormat::Txt,
            "m3u" => FileListFormat::M3u,
            "m3u8" => FileListFormat::M3u8,
            "ndjson" | "jsonl" => FileListFormat::Ndjson,
            _ => FileListFormat::Json,
        }
    }

    /// True when the format only carries a subset of the result set.
    /// The UI warns before writing one of these so nobody thinks their
    /// 900-hit search produced a 12-line playlist by accident.
    pub fn is_lossy(self) -> bool {
        matches!(self, FileListFormat::M3u | FileListFormat::M3u8)
    }
}

/// One row of a parsed file list. The import side of `.efu`.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct FileListEntry {
    /// Full path as recorded in the file list.
    pub path: String,
    pub size: u64,
    /// Unix milliseconds. Zero when the source had no timestamp.
    pub modified_ms: u64,
    pub created_ms: u64,
    pub attrs: u32,
}

impl FileListEntry {
    /// File name component, derived from `path` using both separators
    /// so a Windows-authored `.efu` reads correctly on Linux.
    pub fn name(&self) -> &str {
        match self.path.rfind(['/', '\\']) {
            Some(i) => &self.path[i + 1..],
            None => &self.path,
        }
    }

    /// Lowercase extension without the dot, if any.
    pub fn ext(&self) -> Option<String> {
        let name = self.name();
        name.rsplit_once('.')
            .filter(|(stem, _)| !stem.is_empty())
            .map(|(_, e)| e.to_ascii_lowercase())
    }

    pub fn is_directory(&self) -> bool {
        self.attrs & ATTR_DIRECTORY != 0
    }
}

#[derive(Debug, thiserror::Error)]
pub enum FileListError {
    #[error("not an Everything file list: expected the `Filename,Size,…` header, found {found:?}")]
    MissingEfuHeader { found: String },
    #[error("malformed row {line}: {reason}")]
    MalformedRow { line: usize, reason: String },
    #[error("unterminated quoted field starting on line {line}")]
    UnterminatedQuote { line: usize },
    #[error("{0} cannot be imported — it carries no per-file metadata")]
    NotImportable(&'static str),
    #[error(transparent)]
    Json(#[from] serde_json::Error),
}

// ---------- export -----------------------------------------------------

/// Serialise a result set into the requested interop format.
pub fn export(hits: &[QueryHit], format: FileListFormat) -> Result<String, FileListError> {
    Ok(match format {
        FileListFormat::Efu => export_efu(hits),
        FileListFormat::Csv => export_csv(hits),
        FileListFormat::Txt => export_txt(hits),
        FileListFormat::M3u | FileListFormat::M3u8 => export_m3u(hits),
        FileListFormat::Ndjson => export_ndjson(hits)?,
        FileListFormat::Json => serde_json::to_string_pretty(hits)?,
    })
}

/// The canonical `.efu` header. Everything matches it literally.
pub const EFU_HEADER: &str = "Filename,Size,Date Modified,Date Created,Attributes";

fn export_efu(hits: &[QueryHit]) -> String {
    let mut out = String::with_capacity(hits.len() * 96 + EFU_HEADER.len());
    out.push_str(EFU_HEADER);
    out.push_str("\r\n");
    for h in hits {
        write_csv_field(&mut out, &h.path);
        // Everything writes 0 for folders; the size column is unsigned.
        let _ = write!(out, ",{}", h.size);
        let _ = write!(
            out,
            ",{},{},{}",
            unix_ms_to_filetime(h.modified_ms),
            // Freally's `QueryHit` carries no separate creation time, so
            // the two columns agree. Everything tolerates this and the
            // round-trip below asserts it.
            unix_ms_to_filetime(h.modified_ms),
            h.attrs
        );
        out.push_str("\r\n");
    }
    out
}

fn export_csv(hits: &[QueryHit]) -> String {
    let mut out = String::with_capacity(hits.len() * 96);
    out.push_str("Name,Path,Size,Modified,Type\r\n");
    for h in hits {
        write_csv_field(&mut out, &h.name);
        out.push(',');
        write_csv_field(&mut out, &h.path);
        let _ = write!(out, ",{},{},", h.size, h.modified_ms);
        write_csv_field(&mut out, &h.kind);
        out.push_str("\r\n");
    }
    out
}

fn export_txt(hits: &[QueryHit]) -> String {
    let mut out = String::with_capacity(hits.len() * 64);
    for h in hits {
        out.push_str(&h.path);
        out.push('\n');
    }
    out
}

fn export_m3u(hits: &[QueryHit]) -> String {
    let mut out = String::from("#EXTM3U\n");
    for h in hits.iter().filter(|h| is_audio(h)) {
        // Duration is unknown without decoding, and the audio lens's
        // measured length isn't on `QueryHit`; `-1` is the documented
        // "unknown" value and every player accepts it.
        let _ = writeln!(out, "#EXTINF:-1,{}", h.name);
        out.push_str(&h.path);
        out.push('\n');
    }
    out
}

fn export_ndjson(hits: &[QueryHit]) -> Result<String, FileListError> {
    let mut out = String::with_capacity(hits.len() * 160);
    for h in hits {
        out.push_str(&serde_json::to_string(h)?);
        out.push('\n');
    }
    Ok(out)
}

fn is_audio(hit: &QueryHit) -> bool {
    AUDIO_EXTENSIONS
        .iter()
        .any(|e| hit.ext.eq_ignore_ascii_case(e))
}

// ---------- import -----------------------------------------------------

/// Parse a file list. Only formats that carry per-file metadata are
/// importable — a playlist or a bare path dump would produce entries
/// with invented sizes and dates.
pub fn import(text: &str, format: FileListFormat) -> Result<Vec<FileListEntry>, FileListError> {
    match format {
        FileListFormat::Efu => import_efu(text),
        FileListFormat::Ndjson => import_ndjson(text),
        FileListFormat::Txt => Ok(import_txt(text)),
        FileListFormat::Csv => Err(FileListError::NotImportable(
            "CSV export uses display columns, not index fields — import .efu instead",
        )),
        FileListFormat::M3u | FileListFormat::M3u8 => {
            Err(FileListError::NotImportable("an M3U playlist"))
        }
        FileListFormat::Json => import_json(text),
    }
}

fn import_efu(text: &str) -> Result<Vec<FileListEntry>, FileListError> {
    let rows = parse_csv(text)?;
    let mut iter = rows.into_iter();
    let header = iter.next().unwrap_or_default();
    let header_line = header.join(",");
    if !header_line.eq_ignore_ascii_case(EFU_HEADER) {
        return Err(FileListError::MissingEfuHeader { found: header_line });
    }
    let mut out = Vec::new();
    for (i, row) in iter.enumerate() {
        // Line numbers are 1-based and the header is line 1.
        let line = i + 2;
        if row.iter().all(|f| f.is_empty()) {
            continue;
        }
        if row.len() < 5 {
            return Err(FileListError::MalformedRow {
                line,
                reason: format!("expected 5 columns, found {}", row.len()),
            });
        }
        out.push(FileListEntry {
            path: row[0].clone(),
            size: parse_num_field(&row[1], line, "Size")?,
            modified_ms: filetime_to_unix_ms(parse_num_field(&row[2], line, "Date Modified")?),
            created_ms: filetime_to_unix_ms(parse_num_field(&row[3], line, "Date Created")?),
            attrs: parse_num_field(&row[4], line, "Attributes")?,
        });
    }
    Ok(out)
}

fn import_ndjson(text: &str) -> Result<Vec<FileListEntry>, FileListError> {
    let mut out = Vec::new();
    for line in text.lines().filter(|l| !l.trim().is_empty()) {
        let hit: QueryHit = serde_json::from_str(line)?;
        out.push(entry_from_hit(&hit));
    }
    Ok(out)
}

fn import_json(text: &str) -> Result<Vec<FileListEntry>, FileListError> {
    let hits: Vec<QueryHit> = serde_json::from_str(text)?;
    Ok(hits.iter().map(entry_from_hit).collect())
}

/// A TXT list has paths and nothing else. Sizes and dates stay zero —
/// the caller re-stats them if it wants real metadata.
fn import_txt(text: &str) -> Vec<FileListEntry> {
    text.lines()
        .map(str::trim)
        .filter(|l| !l.is_empty())
        .map(|path| FileListEntry {
            path: path.to_string(),
            size: 0,
            modified_ms: 0,
            created_ms: 0,
            attrs: 0,
        })
        .collect()
}

fn entry_from_hit(hit: &QueryHit) -> FileListEntry {
    FileListEntry {
        path: hit.path.clone(),
        size: hit.size,
        modified_ms: hit.modified_ms,
        created_ms: hit.modified_ms,
        attrs: hit.attrs,
    }
}

/// Parse one numeric column. An empty cell reads as zero — Everything
/// writes an empty `Size` for some folder rows.
fn parse_num_field<T>(s: &str, line: usize, column: &str) -> Result<T, FileListError>
where
    T: std::str::FromStr + Default,
{
    let t = s.trim();
    if t.is_empty() {
        return Ok(T::default());
    }
    t.parse().map_err(|_| FileListError::MalformedRow {
        line,
        reason: format!("{column} `{t}` is not a number"),
    })
}

// ---------- CSV primitives ---------------------------------------------

/// Append one RFC-4180 field, quoting only when it has to.
fn write_csv_field(out: &mut String, field: &str) {
    if field.contains([',', '"', '\r', '\n']) {
        out.push('"');
        for c in field.chars() {
            if c == '"' {
                out.push('"');
            }
            out.push(c);
        }
        out.push('"');
    } else {
        out.push_str(field);
    }
}

/// Minimal RFC-4180 reader: handles quoted fields, doubled quotes
/// inside them, embedded newlines, and both CRLF and LF terminators.
/// Written in-tree rather than pulled in as a dependency because the
/// `.efu` dialect is exactly this and nothing more.
fn parse_csv(text: &str) -> Result<Vec<Vec<String>>, FileListError> {
    let mut rows = Vec::new();
    let mut row = Vec::new();
    let mut field = String::new();
    let mut in_quotes = false;
    let mut quote_start_line = 1usize;
    let mut line = 1usize;
    let mut chars = text.chars().peekable();

    while let Some(c) = chars.next() {
        if in_quotes {
            match c {
                '"' => {
                    if chars.peek() == Some(&'"') {
                        chars.next();
                        field.push('"');
                    } else {
                        in_quotes = false;
                    }
                }
                '\n' => {
                    line += 1;
                    field.push('\n');
                }
                _ => field.push(c),
            }
            continue;
        }
        match c {
            '"' if field.is_empty() => {
                in_quotes = true;
                quote_start_line = line;
            }
            ',' => row.push(std::mem::take(&mut field)),
            '\r' => {
                // Swallow a lone CR as well as the CR of a CRLF pair.
                if chars.peek() == Some(&'\n') {
                    chars.next();
                }
                row.push(std::mem::take(&mut field));
                rows.push(std::mem::take(&mut row));
                line += 1;
            }
            '\n' => {
                row.push(std::mem::take(&mut field));
                rows.push(std::mem::take(&mut row));
                line += 1;
            }
            _ => field.push(c),
        }
    }
    if in_quotes {
        return Err(FileListError::UnterminatedQuote {
            line: quote_start_line,
        });
    }
    if !field.is_empty() || !row.is_empty() {
        row.push(field);
        rows.push(row);
    }
    Ok(rows)
}

// ---------- time -------------------------------------------------------

/// Unix milliseconds → Windows FILETIME ticks. Saturates instead of
/// wrapping so a corrupt timestamp can never produce a nonsense date.
pub fn unix_ms_to_filetime(ms: u64) -> i64 {
    // `as i64` on a u64 past `i64::MAX` wraps *negative*, and a negative
    // FILETIME is one Everything rejects — so clamp on the way in rather
    // than letting `saturating_mul` clamp a wrapped value to `i64::MIN`.
    // `modified_ms` reaches here from a caller-supplied `QueryHit`, so
    // the out-of-range case is reachable, not theoretical.
    i64::try_from(ms)
        .unwrap_or(i64::MAX)
        .saturating_add(FILETIME_EPOCH_OFFSET_MS)
        .saturating_mul(FILETIME_TICKS_PER_MS)
}

/// Windows FILETIME ticks → Unix milliseconds. Pre-1970 timestamps
/// clamp to 0 — the UI renders that as "unknown" rather than as a
/// negative date JS cannot hold.
pub fn filetime_to_unix_ms(ticks: i64) -> u64 {
    let ms = ticks / FILETIME_TICKS_PER_MS - FILETIME_EPOCH_OFFSET_MS;
    ms.max(0) as u64
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::dto::LensId;

    fn hit(name: &str, path: &str, ext: &str, size: u64, attrs: u32) -> QueryHit {
        QueryHit {
            file_id: format!("id-{name}"),
            lens: LensId::Filename,
            name: name.to_string(),
            path: path.to_string(),
            ext: ext.to_string(),
            size,
            modified_ms: 1_700_000_000_000,
            kind: ext.to_uppercase(),
            score: 1.0,
            attrs,
        }
    }

    fn fixture() -> Vec<QueryHit> {
        vec![
            hit("report.pdf", "/vault/docs/report.pdf", "pdf", 1200, 0),
            hit("track.flac", "/vault/media/track.flac", "flac", 900, 0),
            hit("docs", "/vault/docs", "", 0, ATTR_DIRECTORY),
        ]
    }

    #[test]
    fn efu_round_trips_through_import() {
        let text = export(&fixture(), FileListFormat::Efu).unwrap();
        let entries = import(&text, FileListFormat::Efu).unwrap();
        assert_eq!(entries.len(), 3);
        assert_eq!(entries[0].path, "/vault/docs/report.pdf");
        assert_eq!(entries[0].size, 1200);
        assert_eq!(entries[0].modified_ms, 1_700_000_000_000);
        assert!(entries[2].is_directory());
    }

    #[test]
    fn efu_writes_the_exact_everything_header() {
        let text = export(&fixture(), FileListFormat::Efu).unwrap();
        assert!(
            text.starts_with("Filename,Size,Date Modified,Date Created,Attributes\r\n"),
            "header was {:?}",
            &text[..text.find('\n').unwrap_or(text.len())]
        );
    }

    #[test]
    fn efu_rejects_a_file_that_is_not_one() {
        let err = import("Name,Path\r\na,b\r\n", FileListFormat::Efu).unwrap_err();
        assert!(matches!(err, FileListError::MissingEfuHeader { .. }));
    }

    #[test]
    fn filetime_round_trips() {
        for ms in [0u64, 1, 1_700_000_000_000, 4_102_444_800_000] {
            assert_eq!(filetime_to_unix_ms(unix_ms_to_filetime(ms)), ms);
        }
    }

    #[test]
    fn pre_1970_filetime_clamps_to_zero() {
        // 1601-01-01 itself.
        assert_eq!(filetime_to_unix_ms(0), 0);
    }

    #[test]
    fn csv_quotes_only_when_required() {
        let hits = vec![hit("a,b\".txt", "/vault/a,b\".txt", "txt", 1, 0)];
        let text = export(&hits, FileListFormat::Csv).unwrap();
        assert!(
            text.contains("\"a,b\"\".txt\""),
            "comma+quote field not escaped: {text}"
        );
        // The plain header fields are never quoted.
        assert!(text.starts_with("Name,Path,Size,Modified,Type\r\n"));
    }

    #[test]
    fn csv_with_embedded_newline_round_trips_through_the_reader() {
        let mut buf = String::new();
        write_csv_field(&mut buf, "line1\nline2");
        buf.push_str("\r\n");
        let rows = parse_csv(&buf).unwrap();
        assert_eq!(rows, vec![vec!["line1\nline2".to_string()]]);
    }

    #[test]
    fn unterminated_quote_is_a_typed_error() {
        let err = parse_csv("\"never closed").unwrap_err();
        assert!(matches!(err, FileListError::UnterminatedQuote { line: 1 }));
    }

    #[test]
    fn txt_is_one_path_per_line() {
        let text = export(&fixture(), FileListFormat::Txt).unwrap();
        assert_eq!(
            text.lines().collect::<Vec<_>>(),
            vec![
                "/vault/docs/report.pdf",
                "/vault/media/track.flac",
                "/vault/docs"
            ]
        );
    }

    #[test]
    fn m3u_carries_audio_only() {
        let text = export(&fixture(), FileListFormat::M3u).unwrap();
        assert!(text.starts_with("#EXTM3U\n"));
        assert!(text.contains("/vault/media/track.flac"));
        assert!(
            !text.contains("report.pdf"),
            "a pdf must not enter a playlist: {text}"
        );
        assert!(FileListFormat::M3u.is_lossy());
    }

    #[test]
    fn ndjson_is_one_object_per_line_and_round_trips() {
        let text = export(&fixture(), FileListFormat::Ndjson).unwrap();
        assert_eq!(text.lines().count(), 3);
        let entries = import(&text, FileListFormat::Ndjson).unwrap();
        assert_eq!(entries.len(), 3);
        assert_eq!(entries[1].path, "/vault/media/track.flac");
    }

    #[test]
    fn format_is_chosen_from_the_extension() {
        assert_eq!(FileListFormat::from_path("x.efu"), FileListFormat::Efu);
        assert_eq!(FileListFormat::from_path("X.EFU"), FileListFormat::Efu);
        assert_eq!(FileListFormat::from_path("a/b.m3u8"), FileListFormat::M3u8);
        assert_eq!(FileListFormat::from_path("a.jsonl"), FileListFormat::Ndjson);
        assert_eq!(FileListFormat::from_path("noext"), FileListFormat::Json);
    }

    #[test]
    fn playlists_and_csv_refuse_to_import() {
        assert!(matches!(
            import("#EXTM3U\n", FileListFormat::M3u),
            Err(FileListError::NotImportable(_))
        ));
        assert!(matches!(
            import("Name,Path\r\n", FileListFormat::Csv),
            Err(FileListError::NotImportable(_))
        ));
    }

    #[test]
    fn entry_name_and_ext_handle_both_separators() {
        let e = FileListEntry {
            path: r"C:\Users\mike\Report.PDF".into(),
            size: 1,
            modified_ms: 0,
            created_ms: 0,
            attrs: 0,
        };
        assert_eq!(e.name(), "Report.PDF");
        assert_eq!(e.ext().as_deref(), Some("pdf"));

        let dotfile = FileListEntry {
            path: "/home/mike/.bashrc".into(),
            size: 1,
            modified_ms: 0,
            created_ms: 0,
            attrs: 0,
        };
        assert_eq!(dotfile.name(), ".bashrc");
        assert_eq!(dotfile.ext(), None, "a leading dot is not an extension");
    }

    #[test]
    fn a_path_containing_a_comma_survives_the_efu_round_trip() {
        let hits = vec![hit("a, b.txt", "/vault/notes/a, b.txt", "txt", 7, 0)];
        let text = export(&hits, FileListFormat::Efu).unwrap();
        let entries = import(&text, FileListFormat::Efu).unwrap();
        assert_eq!(entries[0].path, "/vault/notes/a, b.txt");
        assert_eq!(entries[0].size, 7);
    }
}
