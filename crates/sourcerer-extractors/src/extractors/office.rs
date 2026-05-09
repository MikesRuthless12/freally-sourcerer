//! Office extractors — xlsx (via `calamine`) and docx + pptx (via
//! `zip` + `quick-xml`).
//!
//! Build Guide Phase 8: "calamine for xlsx; ooxmlsdk-rs for docx +
//! pptx. Extract paragraph text; preserve doc structure as markdown."
//!
//! ### Dep choice (Build-Guide deviation)
//!
//! `ooxmlsdk-rs` is unmaintained on crates.io as of 2026-Q1 (last
//! release pre-0.1.0, no Rust 2024 support). Office Open XML is a
//! zip of XML, so we read it directly with `zip` + `quick-xml`. The
//! result is fewer transitive deps (no full SDK) and a smaller
//! attack surface — the indexer only needs *text*, not styling /
//! layout / embedded objects. See `docs/SECURITY.md` for the threat
//! model delta.
//!
//! ### Output shape
//!
//! * **docx** — paragraphs joined with `\n\n`. Headings (`pStyle`
//!   ∈ Heading1…Heading6) prefixed with the matching `#` count.
//!   Tables flattened to `| cell | cell |\n` rows. Each `<w:p>`
//!   element flushes one paragraph.
//! * **xlsx** — one line per non-empty cell of the form `Sheet1!A1=value`.
//!   The `Sheet!Cell=value` shape is consistent with the archive-peek
//!   `archive.zip!entry size=N` shape so search snippets read uniformly.
//! * **pptx** — `# Slide N: <title>` per slide, then the slide's text
//!   runs joined by `\n`. Slides are visited in numeric order.

use std::fs::File;
use std::io::{BufReader, Read};
use std::path::Path;

use crate::sink::TextSink;
use crate::{ExtractError, ExtractionStats, Extractor, ExtractorId};

const ZIP_MAGIC: [u8; 4] = [b'P', b'K', 0x03, 0x04];
const CANCEL_CHECK_EVERY: usize = 32;

// ---------------------------------------------------------------------
// docx
// ---------------------------------------------------------------------

#[derive(Debug, Default, Clone)]
pub struct DocxExtractor;

impl Extractor for DocxExtractor {
    fn id(&self) -> ExtractorId {
        ExtractorId::new("docx")
    }

    fn matches(&self, path: &Path, magic: &[u8]) -> bool {
        if !magic.starts_with(&ZIP_MAGIC) {
            return false;
        }
        match path.extension().and_then(|s| s.to_str()) {
            Some(ext) => ext.eq_ignore_ascii_case("docx"),
            None => false,
        }
    }

    fn extract(&self, path: &Path, sink: &mut TextSink) -> Result<ExtractionStats, ExtractError> {
        let file = File::open(path).map_err(|e| ExtractError::io(path, e))?;
        let bytes_in = file.metadata().map(|m| m.len()).unwrap_or(0);
        let mut zip = zip::ZipArchive::new(BufReader::new(file))
            .map_err(|e| ExtractError::Malformed(format!("docx zip open failed: {e}")))?;

        // The main document body lives at word/document.xml. Files
        // missing it aren't legitimately docx.
        let xml = read_zip_entry(&mut zip, "word/document.xml")?;

        if sink.is_cancelled() {
            return Err(ExtractError::Cancelled);
        }

        let body = parse_docx_body(&xml, sink)?;
        sink.push_str(&body)
            .map_err(|_| ExtractError::OutputTooLarge { cap: sink.cap() })?;

        Ok(ExtractionStats {
            bytes_in,
            bytes_out: sink.len() as u64,
            elapsed: std::time::Duration::ZERO,
        })
    }
}

fn parse_docx_body(xml: &[u8], sink: &TextSink) -> Result<String, ExtractError> {
    use quick_xml::events::Event;
    use quick_xml::name::QName;
    use quick_xml::reader::Reader;

    let mut reader = Reader::from_reader(xml);
    reader.config_mut().trim_text(false);

    let mut out = String::new();
    let mut text_buf = String::new();
    let mut in_text = false;
    let mut current_style: Option<u8> = None; // heading level 1-6
    let mut table_row: Vec<String> = Vec::new();
    let mut table_cell: String = String::new();
    let mut in_cell = false;
    let mut event_count = 0usize;
    let mut buf = Vec::new();

    loop {
        // Phase 8 review pass: the previous version checked cancel
        // every CANCEL_CHECK_EVERY *paragraphs*. A hostile docx with
        // one giant `<w:p>` (thousands of `<w:t>` runs concatenated)
        // could spend seconds inside the inner text-event loop without
        // touching the flag. Counting *every* event makes the check
        // fire on a bounded budget no matter how the XML is shaped;
        // the load itself is a Relaxed atomic, sub-nanosecond cost.
        event_count = event_count.wrapping_add(1);
        if event_count % CANCEL_CHECK_EVERY == 0 && sink.is_cancelled() {
            return Err(ExtractError::Cancelled);
        }
        match reader.read_event_into(&mut buf) {
            Err(e) => return Err(ExtractError::Malformed(format!("docx XML: {e}"))),
            Ok(Event::Eof) => break,
            Ok(Event::Start(e)) => match e.name() {
                QName(b"w:t") => in_text = true,
                QName(b"w:tc") => {
                    in_cell = true;
                    table_cell.clear();
                }
                QName(b"w:pStyle") => {
                    // pStyle on its own block — read the attribute val
                    // for heading detection.
                    for a in e.attributes().with_checks(false).flatten() {
                        if a.key.as_ref() == b"w:val" {
                            if let Ok(s) = std::str::from_utf8(&a.value) {
                                current_style = parse_heading_level(s);
                            }
                        }
                    }
                }
                _ => {}
            },
            Ok(Event::Empty(e)) => {
                if e.name() == QName(b"w:pStyle") {
                    for a in e.attributes().with_checks(false).flatten() {
                        if a.key.as_ref() == b"w:val" {
                            if let Ok(s) = std::str::from_utf8(&a.value) {
                                current_style = parse_heading_level(s);
                            }
                        }
                    }
                }
            }
            Ok(Event::End(e)) => match e.name() {
                QName(b"w:t") => in_text = false,
                QName(b"w:p") => {
                    let para = std::mem::take(&mut text_buf);
                    // Skip empty paragraphs even when a heading style
                    // was set — a `# \n\n` line is just noise in the
                    // search blob and counts against the sink cap for
                    // no recall benefit. The previous behaviour emitted
                    // the bare hash + blank line.
                    if !para.is_empty() {
                        if let Some(level) = current_style {
                            for _ in 0..level {
                                out.push('#');
                            }
                            out.push(' ');
                        }
                        out.push_str(&para);
                        out.push_str("\n\n");
                    }
                    current_style = None;
                }
                QName(b"w:tc") => {
                    in_cell = false;
                    table_row.push(std::mem::take(&mut table_cell));
                }
                QName(b"w:tr") => {
                    if !table_row.is_empty() {
                        out.push('|');
                        for cell in table_row.drain(..) {
                            out.push(' ');
                            out.push_str(&cell);
                            out.push(' ');
                            out.push('|');
                        }
                        out.push('\n');
                    }
                }
                QName(b"w:tbl") => {
                    out.push('\n');
                }
                _ => {}
            },
            Ok(Event::Text(t)) if in_text => {
                let txt = decode_xml_text(t.as_ref())?;
                if in_cell {
                    table_cell.push_str(&txt);
                } else {
                    text_buf.push_str(&txt);
                }
            }
            _ => {}
        }
        buf.clear();
    }
    Ok(out)
}

/// Decode a quick-xml text payload to UTF-8 + unescape XML entities.
/// quick-xml 0.39 moved the per-entity unescape off `BytesText`; we
/// route through `quick_xml::escape::unescape` so encoded sequences
/// like `&amp;lt;` correctly round-trip to the literal text `&lt;`
/// (the previous hand-rolled multi-pass `replace` chain over-decoded
/// nested entities).
fn decode_xml_text(bytes: &[u8]) -> Result<String, ExtractError> {
    let raw = std::str::from_utf8(bytes)
        .map_err(|e| ExtractError::Malformed(format!("xml utf-8: {e}")))?;
    let cow = quick_xml::escape::unescape(raw)
        .map_err(|e| ExtractError::Malformed(format!("xml unescape: {e}")))?;
    Ok(cow.into_owned())
}

fn parse_heading_level(style: &str) -> Option<u8> {
    let s = style.to_ascii_lowercase();
    let prefix = "heading";
    let rest = s.strip_prefix(prefix)?;
    let n: u8 = rest.parse().ok()?;
    if (1..=6).contains(&n) { Some(n) } else { None }
}

// ---------------------------------------------------------------------
// pptx
// ---------------------------------------------------------------------

#[derive(Debug, Default, Clone)]
pub struct PptxExtractor;

impl Extractor for PptxExtractor {
    fn id(&self) -> ExtractorId {
        ExtractorId::new("pptx")
    }

    fn matches(&self, path: &Path, magic: &[u8]) -> bool {
        if !magic.starts_with(&ZIP_MAGIC) {
            return false;
        }
        match path.extension().and_then(|s| s.to_str()) {
            Some(ext) => ext.eq_ignore_ascii_case("pptx"),
            None => false,
        }
    }

    fn extract(&self, path: &Path, sink: &mut TextSink) -> Result<ExtractionStats, ExtractError> {
        let file = File::open(path).map_err(|e| ExtractError::io(path, e))?;
        let bytes_in = file.metadata().map(|m| m.len()).unwrap_or(0);
        let mut zip = zip::ZipArchive::new(BufReader::new(file))
            .map_err(|e| ExtractError::Malformed(format!("pptx zip open failed: {e}")))?;

        // Walk slides in numeric order. ppt/slides/slide{N}.xml.
        let mut slide_paths: Vec<String> = (0..zip.len())
            .filter_map(|i| {
                let entry = zip.by_index(i).ok()?;
                let name = entry.name();
                if name.starts_with("ppt/slides/slide") && name.ends_with(".xml") {
                    Some(name.to_owned())
                } else {
                    None
                }
            })
            .collect();
        slide_paths.sort_by_key(|s| {
            // Numeric sort by the digits between "slide" and ".xml".
            let stem = s
                .trim_start_matches("ppt/slides/slide")
                .trim_end_matches(".xml");
            stem.parse::<u32>().unwrap_or(u32::MAX)
        });

        if slide_paths.is_empty() {
            return Err(ExtractError::Malformed(
                "pptx contains no ppt/slides/slide*.xml entries".into(),
            ));
        }

        let mut out = String::new();
        for (idx, slide_path) in slide_paths.iter().enumerate() {
            if sink.is_cancelled() {
                return Err(ExtractError::Cancelled);
            }
            let xml = read_zip_entry(&mut zip, slide_path)?;
            let runs = parse_pptx_slide(&xml, sink)?;
            // Heuristic: first non-empty run becomes the slide title;
            // remaining non-empty runs become body lines. `title_idx`
            // is computed once so the body loop's "skip the title"
            // check stays in step with the title we actually emitted.
            let title_idx = runs.iter().position(|r| !r.trim().is_empty());
            out.push_str(&format!("# Slide {n}", n = idx + 1));
            if let Some(i) = title_idx {
                out.push_str(": ");
                out.push_str(runs[i].trim());
            }
            out.push_str("\n\n");
            for (j, run) in runs.iter().enumerate() {
                if Some(j) == title_idx {
                    continue;
                }
                if !run.trim().is_empty() {
                    out.push_str(run);
                    out.push('\n');
                }
            }
            out.push('\n');
        }

        sink.push_str(&out)
            .map_err(|_| ExtractError::OutputTooLarge { cap: sink.cap() })?;

        Ok(ExtractionStats {
            bytes_in,
            bytes_out: sink.len() as u64,
            elapsed: std::time::Duration::ZERO,
        })
    }
}

fn parse_pptx_slide(xml: &[u8], sink: &TextSink) -> Result<Vec<String>, ExtractError> {
    use quick_xml::events::Event;
    use quick_xml::name::QName;
    use quick_xml::reader::Reader;

    let mut reader = Reader::from_reader(xml);
    reader.config_mut().trim_text(false);

    let mut runs: Vec<String> = Vec::new();
    let mut buf = Vec::new();
    let mut in_text = false;
    let mut current = String::new();
    let mut in_paragraph = false;
    let mut event_count = 0usize;

    loop {
        // Phase 8 review pass: per-event cancel check — the previous
        // version had no cancel check inside slide parsing, so a
        // hostile slide with one massive `<a:t>` run could dodge the
        // outer per-slide check entirely.
        event_count = event_count.wrapping_add(1);
        if event_count % CANCEL_CHECK_EVERY == 0 && sink.is_cancelled() {
            return Err(ExtractError::Cancelled);
        }
        match reader.read_event_into(&mut buf) {
            Err(e) => return Err(ExtractError::Malformed(format!("pptx XML: {e}"))),
            Ok(Event::Eof) => break,
            Ok(Event::Start(e)) => match e.name() {
                QName(b"a:t") => in_text = true,
                QName(b"a:p") => {
                    in_paragraph = true;
                    current.clear();
                }
                _ => {}
            },
            Ok(Event::End(e)) => match e.name() {
                QName(b"a:t") => in_text = false,
                QName(b"a:p") => {
                    if in_paragraph {
                        runs.push(std::mem::take(&mut current));
                        in_paragraph = false;
                    }
                }
                _ => {}
            },
            Ok(Event::Text(t)) if in_text => {
                let txt = decode_xml_text(t.as_ref())?;
                current.push_str(&txt);
            }
            _ => {}
        }
        buf.clear();
    }
    Ok(runs)
}

// ---------------------------------------------------------------------
// xlsx
// ---------------------------------------------------------------------

#[derive(Debug, Default, Clone)]
pub struct XlsxExtractor;

impl Extractor for XlsxExtractor {
    fn id(&self) -> ExtractorId {
        ExtractorId::new("xlsx")
    }

    fn matches(&self, path: &Path, magic: &[u8]) -> bool {
        if !magic.starts_with(&ZIP_MAGIC) {
            return false;
        }
        match path.extension().and_then(|s| s.to_str()) {
            Some(ext) => {
                let lower = ext.to_ascii_lowercase();
                lower == "xlsx" || lower == "xlsm"
            }
            None => false,
        }
    }

    fn extract(&self, path: &Path, sink: &mut TextSink) -> Result<ExtractionStats, ExtractError> {
        use calamine::{Reader, Xlsx};
        let bytes_in = std::fs::metadata(path).map(|m| m.len()).unwrap_or(0);
        let file = File::open(path).map_err(|e| ExtractError::io(path, e))?;
        let mut workbook: Xlsx<_> = calamine::open_workbook_from_rs(BufReader::new(file))
            .map_err(|e| ExtractError::Malformed(format!("xlsx open failed: {e}")))?;

        let sheet_names: Vec<String> = workbook.sheet_names().to_vec();
        let mut cell_count = 0usize;
        for sheet_name in sheet_names {
            if sink.is_cancelled() {
                return Err(ExtractError::Cancelled);
            }
            let range = workbook
                .worksheet_range(&sheet_name)
                .map_err(|e| ExtractError::Malformed(format!("xlsx sheet load: {e}")))?;
            let start = range.start().unwrap_or((0, 0));
            for (r, row) in range.rows().enumerate() {
                if cell_count % CANCEL_CHECK_EVERY == 0 && sink.is_cancelled() {
                    return Err(ExtractError::Cancelled);
                }
                for (c, cell) in row.iter().enumerate() {
                    if matches!(cell, calamine::Data::Empty) {
                        continue;
                    }
                    let abs_row = start.0 as usize + r + 1; // 1-indexed in Excel
                    let abs_col = start.1 as usize + c;
                    let col_letters = column_letters(abs_col as u32);
                    let value = render_cell(cell);
                    let line = format!("{sheet_name}!{col_letters}{abs_row}={value}\n");
                    sink.push_str(&line)
                        .map_err(|_| ExtractError::OutputTooLarge { cap: sink.cap() })?;
                    cell_count += 1;
                }
            }
        }

        Ok(ExtractionStats {
            bytes_in,
            bytes_out: sink.len() as u64,
            elapsed: std::time::Duration::ZERO,
        })
    }
}

/// 0-indexed column → A, B, …, Z, AA, AB, … . Excel uses 1-based column
/// numbering; our caller passes the 0-indexed value.
fn column_letters(col: u32) -> String {
    let mut n = col;
    let mut s = String::new();
    loop {
        let r = (n % 26) as u8;
        s.insert(0, (b'A' + r) as char);
        if n < 26 {
            break;
        }
        n = n / 26 - 1;
    }
    s
}

fn render_cell(cell: &calamine::Data) -> String {
    use calamine::Data;
    match cell {
        Data::String(s) => s.clone(),
        Data::Float(f) => f.to_string(),
        Data::Int(i) => i.to_string(),
        Data::Bool(b) => b.to_string(),
        Data::DateTime(d) => d.to_string(),
        Data::DateTimeIso(s) => s.clone(),
        Data::DurationIso(s) => s.clone(),
        Data::Error(e) => format!("#ERR:{e:?}"),
        Data::Empty => String::new(),
    }
}

// ---------------------------------------------------------------------
// shared helpers
// ---------------------------------------------------------------------

/// Maximum bytes [`read_zip_entry`] reads from a single archive entry.
/// Caps adversarial size declarations: a docx that claims a 4 GiB
/// `word/document.xml` body would otherwise drive a 4 GiB
/// `Vec::with_capacity` allocation regardless of actual content size.
/// 64 MiB is well above any plausible legitimate OOXML body and well
/// below the 256 MiB sandbox RSS ceiling.
const ZIP_ENTRY_CAP_BYTES: u64 = 64 * 1024 * 1024;

fn read_zip_entry<R: Read + std::io::Seek>(
    zip: &mut zip::ZipArchive<R>,
    name: &str,
) -> Result<Vec<u8>, ExtractError> {
    let mut entry = zip
        .by_name(name)
        .map_err(|e| ExtractError::Malformed(format!("missing entry {name}: {e}")))?;
    let declared = entry.size();
    if declared > ZIP_ENTRY_CAP_BYTES {
        return Err(ExtractError::Malformed(format!(
            "entry {name} declares {declared} bytes, exceeds {ZIP_ENTRY_CAP_BYTES}-byte cap"
        )));
    }
    // Cap the pre-allocation at the smaller of the declared size and
    // the per-entry cap. The +1 byte on the limited reader is the
    // sentinel for "we read past the cap" — never trust the declared
    // size; verify against the actual stream length.
    let initial = declared.min(ZIP_ENTRY_CAP_BYTES) as usize;
    let mut buf = Vec::with_capacity(initial);
    let mut limited = (&mut entry).take(ZIP_ENTRY_CAP_BYTES + 1);
    limited
        .read_to_end(&mut buf)
        .map_err(|e| ExtractError::Malformed(format!("read {name}: {e}")))?;
    if buf.len() as u64 > ZIP_ENTRY_CAP_BYTES {
        return Err(ExtractError::Malformed(format!(
            "entry {name} streamed past {ZIP_ENTRY_CAP_BYTES}-byte cap"
        )));
    }
    Ok(buf)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::{Cursor, Write};
    use tempfile::tempdir;

    fn write_fixture(name: &str, bytes: &[u8]) -> std::path::PathBuf {
        let dir = tempdir().unwrap().keep();
        let path = dir.join(name);
        let mut f = File::create(&path).unwrap();
        f.write_all(bytes).unwrap();
        path
    }

    fn make_docx(document_xml: &str) -> Vec<u8> {
        let buf = Cursor::new(Vec::<u8>::new());
        let mut zip = zip::ZipWriter::new(buf);
        let opts: zip::write::SimpleFileOptions = zip::write::SimpleFileOptions::default()
            .compression_method(zip::CompressionMethod::Stored);
        zip.start_file("word/document.xml", opts).unwrap();
        zip.write_all(document_xml.as_bytes()).unwrap();
        // Minimal Content_Types to feel like a real docx.
        zip.start_file("[Content_Types].xml", opts).unwrap();
        zip.write_all(b"<?xml version=\"1.0\"?><Types/>").unwrap();
        zip.finish().unwrap().into_inner()
    }

    fn make_pptx(slides: &[&str]) -> Vec<u8> {
        let buf = Cursor::new(Vec::<u8>::new());
        let mut zip = zip::ZipWriter::new(buf);
        let opts: zip::write::SimpleFileOptions = zip::write::SimpleFileOptions::default()
            .compression_method(zip::CompressionMethod::Stored);
        for (i, xml) in slides.iter().enumerate() {
            let name = format!("ppt/slides/slide{}.xml", i + 1);
            zip.start_file(&name, opts).unwrap();
            zip.write_all(xml.as_bytes()).unwrap();
        }
        zip.finish().unwrap().into_inner()
    }

    #[test]
    fn docx_matches_extension_and_magic() {
        let ext = DocxExtractor;
        assert!(ext.matches(Path::new("/x/file.docx"), b"PK\x03\x04..."));
        assert!(!ext.matches(Path::new("/x/file.docx"), b"not a zip"));
        assert!(!ext.matches(Path::new("/x/file.zip"), b"PK\x03\x04..."));
    }

    #[test]
    fn docx_extracts_paragraphs() {
        let xml = r#"<?xml version="1.0"?>
<w:document xmlns:w="http://schemas.openxmlformats.org/wordprocessingml/2006/main">
<w:body>
<w:p><w:r><w:t>Hello</w:t></w:r><w:r><w:t> world</w:t></w:r></w:p>
<w:p><w:r><w:t>second paragraph</w:t></w:r></w:p>
</w:body></w:document>"#;
        let bytes = make_docx(xml);
        let path = write_fixture("doc.docx", &bytes);
        let mut sink = TextSink::new(1024);
        DocxExtractor.extract(&path, &mut sink).unwrap();
        let out = std::str::from_utf8(sink.as_bytes()).unwrap();
        assert!(out.contains("Hello world"));
        assert!(out.contains("second paragraph"));
    }

    #[test]
    fn docx_renders_headings_as_markdown() {
        let xml = r#"<?xml version="1.0"?>
<w:document xmlns:w="http://schemas.openxmlformats.org/wordprocessingml/2006/main">
<w:body>
<w:p><w:pPr><w:pStyle w:val="Heading1"/></w:pPr><w:r><w:t>Title</w:t></w:r></w:p>
<w:p><w:pPr><w:pStyle w:val="Heading2"/></w:pPr><w:r><w:t>Sub</w:t></w:r></w:p>
<w:p><w:r><w:t>body</w:t></w:r></w:p>
</w:body></w:document>"#;
        let bytes = make_docx(xml);
        let path = write_fixture("h.docx", &bytes);
        let mut sink = TextSink::new(1024);
        DocxExtractor.extract(&path, &mut sink).unwrap();
        let out = std::str::from_utf8(sink.as_bytes()).unwrap();
        assert!(out.contains("# Title"));
        assert!(out.contains("## Sub"));
        assert!(out.contains("body"));
    }

    #[test]
    fn pptx_matches_extension_and_magic() {
        let ext = PptxExtractor;
        assert!(ext.matches(Path::new("/x/deck.pptx"), b"PK\x03\x04..."));
        assert!(!ext.matches(Path::new("/x/deck.docx"), b"PK\x03\x04..."));
    }

    #[test]
    fn pptx_extracts_slides() {
        let s1 = r#"<?xml version="1.0"?>
<p:sld xmlns:a="http://schemas.openxmlformats.org/drawingml/2006/main"
       xmlns:p="http://schemas.openxmlformats.org/presentationml/2006/main">
<p:cSld><p:spTree>
<a:p><a:r><a:t>Slide One Title</a:t></a:r></a:p>
<a:p><a:r><a:t>bullet 1</a:t></a:r></a:p>
</p:spTree></p:cSld></p:sld>"#;
        let s2 = r#"<?xml version="1.0"?>
<p:sld xmlns:a="http://schemas.openxmlformats.org/drawingml/2006/main"
       xmlns:p="http://schemas.openxmlformats.org/presentationml/2006/main">
<p:cSld><p:spTree>
<a:p><a:r><a:t>Slide Two</a:t></a:r></a:p>
</p:spTree></p:cSld></p:sld>"#;
        let bytes = make_pptx(&[s1, s2]);
        let path = write_fixture("d.pptx", &bytes);
        let mut sink = TextSink::new(2048);
        PptxExtractor.extract(&path, &mut sink).unwrap();
        let out = std::str::from_utf8(sink.as_bytes()).unwrap();
        assert!(out.contains("# Slide 1: Slide One Title"));
        assert!(out.contains("bullet 1"));
        assert!(out.contains("# Slide 2: Slide Two"));
    }

    #[test]
    fn xlsx_matches_extension_and_magic() {
        let ext = XlsxExtractor;
        assert!(ext.matches(Path::new("/x/sheet.xlsx"), b"PK\x03\x04..."));
        assert!(!ext.matches(Path::new("/x/sheet.zip"), b"PK\x03\x04..."));
    }

    #[test]
    fn column_letters_round_trip() {
        assert_eq!(column_letters(0), "A");
        assert_eq!(column_letters(25), "Z");
        assert_eq!(column_letters(26), "AA");
        assert_eq!(column_letters(701), "ZZ");
        assert_eq!(column_letters(702), "AAA");
    }

    /// Phase 8 review pass — `parse_docx_body` handles `w:tc` / `w:tr`
    /// / `w:tbl` tags but no test exercised the table path. A
    /// regression here would silently strip table contents from
    /// search recall on every docx with a table.
    #[test]
    fn docx_renders_tables_as_pipe_rows() {
        let xml = r#"<?xml version="1.0"?>
<w:document xmlns:w="http://schemas.openxmlformats.org/wordprocessingml/2006/main">
<w:body>
<w:tbl>
<w:tr>
<w:tc><w:p><w:r><w:t>Header A</w:t></w:r></w:p></w:tc>
<w:tc><w:p><w:r><w:t>Header B</w:t></w:r></w:p></w:tc>
</w:tr>
<w:tr>
<w:tc><w:p><w:r><w:t>row1-a</w:t></w:r></w:p></w:tc>
<w:tc><w:p><w:r><w:t>row1-b</w:t></w:r></w:p></w:tc>
</w:tr>
</w:tbl>
</w:body></w:document>"#;
        let bytes = make_docx(xml);
        let path = write_fixture("table.docx", &bytes);
        let mut sink = TextSink::new(2048);
        DocxExtractor.extract(&path, &mut sink).unwrap();
        let out = std::str::from_utf8(sink.as_bytes()).unwrap();
        assert!(
            out.contains("| Header A | Header B |"),
            "header row missing in {out:?}"
        );
        assert!(
            out.contains("| row1-a | row1-b |"),
            "data row missing in {out:?}"
        );
    }

    /// Phase 8 review pass — heading-styled but text-empty paragraphs
    /// used to emit a bare `# \n\n` (or deeper hash level) into the
    /// search blob. Locked out via the empty-paragraph skip.
    #[test]
    fn docx_skips_empty_heading_paragraphs() {
        let xml = r#"<?xml version="1.0"?>
<w:document xmlns:w="http://schemas.openxmlformats.org/wordprocessingml/2006/main">
<w:body>
<w:p><w:pPr><w:pStyle w:val="Heading1"/></w:pPr></w:p>
<w:p><w:r><w:t>real body</w:t></w:r></w:p>
</w:body></w:document>"#;
        let bytes = make_docx(xml);
        let path = write_fixture("empty_h.docx", &bytes);
        let mut sink = TextSink::new(1024);
        DocxExtractor.extract(&path, &mut sink).unwrap();
        let out = std::str::from_utf8(sink.as_bytes()).unwrap();
        assert!(out.contains("real body"));
        assert!(!out.contains("# \n"), "bare-hash heading leaked: {out:?}");
    }
}
