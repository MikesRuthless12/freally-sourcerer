//! SRC-M09 — machine-readable `freally search` output.
//!
//! `freally search` streams lens-grouped hits for a human to read. That
//! shape is hostile to a shell script: the lens headers, the em dash
//! between name and path, and the timing footer all have to be parsed
//! back out. These formats exist so nothing ever has to — `--json` for
//! one document, `--ndjson` for a stream, `--csv` for a spreadsheet,
//! `-0` for `xargs -0`.
//!
//! Three rules hold across every format:
//!
//! - **`--fields` selects columns**, defaulting to the whole record.
//!   The names match the JSON keys on the wire, so `--fields type`
//!   yields `"type"` and not `"kind"`.
//! - **`--offset` / `--limit` window the stream**, counted in hits
//!   offered rather than hits written, so paging is stable.
//! - **The exit code answers "did anything match?"** without reading
//!   stdout at all — see [`EXIT_HITS`].
//!
//! Only `--json` buffers; the rest write as batches arrive, so a query
//! over a million rows stays flat in memory.

use std::io::{self, Write};

use clap::ValueEnum;
use freally_rpc::filelist::write_csv_field;
use freally_rpc::{LensId, LensTimings, QueryHit};

/// Matched at least one hit.
pub const EXIT_HITS: u8 = 0;
/// Ran to completion and matched nothing. Distinct from [`EXIT_ERROR`]
/// on purpose: `freally search q || echo none` must not fire because
/// the daemon was down, and must fire when the index simply had no
/// answer.
pub const EXIT_NO_HITS: u8 = 1;
/// Parse error, no daemon, or a write that failed.
pub const EXIT_ERROR: u8 = 2;

/// Which serialisation `freally search` writes to stdout.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum Format {
    /// Lens-grouped, em-dash separated, with a timing footer.
    #[default]
    Human,
    /// One document: `{ hits: [...], count, timings }`.
    Json,
    /// One JSON object per line.
    Ndjson,
    /// RFC-4180, CRLF, with a header row.
    Csv,
    /// NUL-separated paths and nothing else, for `xargs -0`.
    Null,
}

/// One selectable column. The variant names are the JSON keys, so a
/// `--fields` list and the object it produces never disagree.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, ValueEnum)]
#[value(rename_all = "snake_case")]
pub enum Field {
    Name,
    Path,
    Ext,
    Size,
    /// Modification time, Unix epoch milliseconds.
    Mtime,
    /// Kind label, e.g. `Text` or `Folder`.
    // Spelled `type` on the wire and on the command line; `Kind` is
    // only the Rust field name, `type` being a keyword.
    #[value(name = "type")]
    Kind,
    Lens,
    Score,
    FileId,
    Attrs,
}

impl Field {
    /// Default column set: the whole record. `--fields` narrows it.
    pub const ALL: &'static [Field] = &[
        Field::Name,
        Field::Path,
        Field::Ext,
        Field::Size,
        Field::Mtime,
        Field::Kind,
        Field::Lens,
        Field::Score,
        Field::FileId,
        Field::Attrs,
    ];

    /// JSON key and CSV header for this column.
    pub fn key(self) -> &'static str {
        match self {
            Field::Name => "name",
            Field::Path => "path",
            Field::Ext => "ext",
            Field::Size => "size",
            Field::Mtime => "mtime",
            Field::Kind => "type",
            Field::Lens => "lens",
            Field::Score => "score",
            Field::FileId => "file_id",
            Field::Attrs => "attrs",
        }
    }

    /// Typed value, so numbers stay numbers in JSON rather than
    /// arriving at `jq` as strings.
    fn json(self, h: &QueryHit) -> serde_json::Value {
        match self {
            Field::Name => h.name.clone().into(),
            Field::Path => h.path.clone().into(),
            Field::Ext => h.ext.clone().into(),
            Field::Size => h.size.into(),
            Field::Mtime => h.modified_ms.into(),
            Field::Kind => h.kind.clone().into(),
            Field::Lens => lens_key(h.lens).into(),
            Field::Score => h.score.into(),
            Field::FileId => h.file_id.clone().into(),
            Field::Attrs => h.attrs.into(),
        }
    }

    /// Flat text, for CSV cells.
    fn text(self, h: &QueryHit) -> String {
        match self {
            Field::Name => h.name.clone(),
            Field::Path => h.path.clone(),
            Field::Ext => h.ext.clone(),
            Field::Size => h.size.to_string(),
            Field::Mtime => h.modified_ms.to_string(),
            Field::Kind => h.kind.clone(),
            Field::Lens => lens_key(h.lens).to_string(),
            Field::Score => h.score.to_string(),
            Field::FileId => h.file_id.clone(),
            Field::Attrs => h.attrs.to_string(),
        }
    }
}

/// Lowercase lens name, matching `LensId`'s serde representation.
fn lens_key(lens: LensId) -> &'static str {
    match lens {
        LensId::Filename => "filename",
        LensId::Content => "content",
        LensId::Audio => "audio",
        LensId::Similarity => "similarity",
    }
}

fn row_json(fields: &[Field], h: &QueryHit) -> serde_json::Value {
    let mut map = serde_json::Map::with_capacity(fields.len());
    for f in fields {
        map.insert(f.key().to_string(), f.json(h));
    }
    serde_json::Value::Object(map)
}

fn encode(value: &serde_json::Value) -> io::Result<String> {
    serde_json::to_string(value).map_err(io::Error::other)
}

/// Streams hits to `W` in the chosen [`Format`], applying `--offset` /
/// `--limit` as it goes.
pub struct Writer<W: Write> {
    out: W,
    format: Format,
    fields: Vec<Field>,
    offset: u64,
    limit: Option<u64>,
    /// Hits offered, including those skipped by `--offset`.
    seen: u64,
    /// Hits actually written.
    emitted: u64,
    csv_header_done: bool,
    /// `--json` is the one format that cannot stream: a single document
    /// has to know its own `count` before it closes the array.
    json_buf: Vec<serde_json::Value>,
    last_lens: Option<LensId>,
}

impl<W: Write> Writer<W> {
    pub fn new(
        out: W,
        format: Format,
        fields: Vec<Field>,
        offset: u64,
        limit: Option<u64>,
    ) -> Self {
        Self {
            out,
            format,
            fields,
            offset,
            limit,
            seen: 0,
            emitted: 0,
            csv_header_done: false,
            json_buf: Vec::new(),
            last_lens: None,
        }
    }

    /// False once `--limit` is satisfied — the caller stops draining the
    /// notification stream instead of formatting rows it will discard.
    pub fn wants_more(&self) -> bool {
        self.limit.is_none_or(|l| self.emitted < l)
    }

    /// Offer one hit. Returns whether the caller should keep going.
    pub fn push(&mut self, hit: &QueryHit) -> io::Result<bool> {
        if !self.wants_more() {
            return Ok(false);
        }
        self.seen += 1;
        if self.seen <= self.offset {
            return Ok(true);
        }
        match self.format {
            Format::Human => self.push_human(hit)?,
            Format::Json => self.json_buf.push(row_json(&self.fields, hit)),
            Format::Ndjson => {
                let line = encode(&row_json(&self.fields, hit))?;
                self.out.write_all(line.as_bytes())?;
                self.out.write_all(b"\n")?;
            }
            Format::Csv => self.push_csv(hit)?,
            // Paths only — a NUL-separated stream exists to be fed
            // straight to `xargs -0`, so columns would be meaningless.
            Format::Null => {
                self.out.write_all(hit.path.as_bytes())?;
                self.out.write_all(&[0])?;
            }
        }
        self.emitted += 1;
        Ok(self.wants_more())
    }

    fn push_human(&mut self, hit: &QueryHit) -> io::Result<()> {
        // The header prints when the lens *changes* rather than once per
        // batch: under `--offset` / `--limit` a batch boundary is no
        // longer where a group starts.
        if self.last_lens != Some(hit.lens) {
            writeln!(self.out)?;
            writeln!(self.out, "[{:?}]", hit.lens)?;
            self.last_lens = Some(hit.lens);
        }
        writeln!(self.out, "  {} — {}", hit.name, hit.path)
    }

    fn push_csv(&mut self, hit: &QueryHit) -> io::Result<()> {
        if !self.csv_header_done {
            let mut head = String::new();
            for (i, f) in self.fields.iter().enumerate() {
                if i > 0 {
                    head.push(',');
                }
                write_csv_field(&mut head, f.key());
            }
            self.out.write_all(head.as_bytes())?;
            self.out.write_all(b"\r\n")?;
            self.csv_header_done = true;
        }
        let mut line = String::new();
        for (i, f) in self.fields.iter().enumerate() {
            if i > 0 {
                line.push(',');
            }
            write_csv_field(&mut line, &f.text(hit));
        }
        self.out.write_all(line.as_bytes())?;
        self.out.write_all(b"\r\n")
    }

    /// Close the output. `--json` emits its document here; the streaming
    /// formats have already written everything and only flush.
    pub fn finish(&mut self, timings: &LensTimings) -> io::Result<()> {
        match self.format {
            Format::Human => {
                writeln!(
                    self.out,
                    "\n# {} hit(s); filename {}ms · content {}ms · audio {}ms · similarity {}ms · total {}ms",
                    self.emitted,
                    timings.filename_ms,
                    timings.content_ms,
                    timings.audio_ms,
                    timings.similarity_ms,
                    timings.total_ms
                )?;
            }
            Format::Json => {
                let doc = serde_json::json!({
                    "hits": std::mem::take(&mut self.json_buf),
                    "count": self.emitted,
                    "timings": timings,
                });
                let text = serde_json::to_string_pretty(&doc).map_err(io::Error::other)?;
                self.out.write_all(text.as_bytes())?;
                self.out.write_all(b"\n")?;
            }
            // A CSV with no rows still owes the consumer its header, so
            // a downstream reader sees an empty table rather than an
            // empty file.
            Format::Csv if !self.csv_header_done => {
                let mut head = String::new();
                for (i, f) in self.fields.iter().enumerate() {
                    if i > 0 {
                        head.push(',');
                    }
                    write_csv_field(&mut head, f.key());
                }
                self.out.write_all(head.as_bytes())?;
                self.out.write_all(b"\r\n")?;
                self.csv_header_done = true;
            }
            Format::Csv | Format::Ndjson | Format::Null => {}
        }
        self.out.flush()
    }

    /// [`EXIT_HITS`] or [`EXIT_NO_HITS`], by whether anything was written.
    pub fn exit_code(&self) -> u8 {
        if self.emitted > 0 {
            EXIT_HITS
        } else {
            EXIT_NO_HITS
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn hit(name: &str, path: &str, size: u64) -> QueryHit {
        QueryHit {
            file_id: format!("id-{name}"),
            lens: LensId::Filename,
            name: name.to_string(),
            path: path.to_string(),
            ext: "txt".into(),
            size,
            modified_ms: 1_700_000_000_000,
            kind: "Text".into(),
            score: 1.0,
            attrs: 32,
            volume: String::new(),
            volume_label: None,
            volume_offline: false,
        }
    }

    fn render(format: Format, fields: Vec<Field>, offset: u64, limit: Option<u64>) -> String {
        let mut w = Writer::new(Vec::new(), format, fields, offset, limit);
        for h in [
            hit("a.txt", "/v/a.txt", 1),
            hit("b.txt", "/v/b.txt", 2),
            hit("c.txt", "/v/c.txt", 3),
        ] {
            let _ = w.push(&h).unwrap();
        }
        w.finish(&LensTimings::default()).unwrap();
        String::from_utf8(std::mem::take(&mut w.out)).unwrap()
    }

    #[test]
    fn ndjson_writes_one_object_per_line() {
        let out = render(Format::Ndjson, vec![Field::Name], 0, None);
        let lines: Vec<_> = out.lines().collect();
        assert_eq!(
            lines,
            [
                r#"{"name":"a.txt"}"#,
                r#"{"name":"b.txt"}"#,
                r#"{"name":"c.txt"}"#
            ]
        );
    }

    #[test]
    fn json_is_one_document_carrying_its_own_count() {
        let out = render(Format::Json, vec![Field::Name], 0, None);
        let v: serde_json::Value = serde_json::from_str(&out).unwrap();
        assert_eq!(v["count"], 3);
        assert_eq!(v["hits"].as_array().unwrap().len(), 3);
        assert!(v["timings"].is_object());
    }

    #[test]
    fn csv_emits_a_header_then_crlf_rows() {
        let out = render(Format::Csv, vec![Field::Name, Field::Size], 0, None);
        assert_eq!(out, "name,size\r\na.txt,1\r\nb.txt,2\r\nc.txt,3\r\n");
    }

    #[test]
    fn csv_with_no_rows_still_writes_its_header() {
        let mut w = Writer::new(Vec::new(), Format::Csv, vec![Field::Name], 0, None);
        w.finish(&LensTimings::default()).unwrap();
        assert_eq!(String::from_utf8(w.out).unwrap(), "name\r\n");
    }

    #[test]
    fn csv_quotes_a_comma_bearing_field() {
        let mut w = Writer::new(Vec::new(), Format::Csv, vec![Field::Name], 0, None);
        w.push(&hit("a,b.txt", "/v/x", 1)).unwrap();
        w.finish(&LensTimings::default()).unwrap();
        let out = String::from_utf8(w.out).unwrap();
        assert!(out.contains("\"a,b.txt\""), "not quoted: {out}");
    }

    #[test]
    fn null_format_writes_nul_separated_paths_only() {
        let out = render(Format::Null, Field::ALL.to_vec(), 0, None);
        assert_eq!(out, "/v/a.txt\0/v/b.txt\0/v/c.txt\0");
    }

    #[test]
    fn offset_and_limit_window_the_stream() {
        let out = render(Format::Ndjson, vec![Field::Name], 1, Some(1));
        assert_eq!(out, "{\"name\":\"b.txt\"}\n");
    }

    #[test]
    fn push_reports_false_once_the_limit_is_reached() {
        let mut w = Writer::new(Vec::new(), Format::Ndjson, vec![Field::Name], 0, Some(1));
        assert!(!w.push(&hit("a.txt", "/v/a.txt", 1)).unwrap());
        assert_eq!(w.emitted, 1);
    }

    #[test]
    fn exit_code_separates_no_hits_from_hits() {
        let mut w = Writer::new(Vec::new(), Format::Ndjson, vec![Field::Name], 0, None);
        assert_eq!(w.exit_code(), EXIT_NO_HITS);
        w.push(&hit("a.txt", "/v/a.txt", 1)).unwrap();
        assert_eq!(w.exit_code(), EXIT_HITS);
    }

    #[test]
    fn field_keys_match_the_wire_names() {
        assert_eq!(Field::Kind.key(), "type");
        assert_eq!(Field::FileId.key(), "file_id");
        // Every column must be reachable from `--fields`.
        assert_eq!(Field::ALL.len(), Field::value_variants().len());
    }

    #[test]
    fn numbers_stay_numbers_in_json() {
        let out = render(Format::Ndjson, vec![Field::Size, Field::Mtime], 0, Some(1));
        let v: serde_json::Value = serde_json::from_str(out.lines().next().unwrap()).unwrap();
        assert!(v["size"].is_number(), "size arrived as {:?}", v["size"]);
        assert!(v["mtime"].is_number());
    }

    #[test]
    fn human_prints_one_lens_header_per_run_not_per_hit() {
        let out = render(Format::Human, Field::ALL.to_vec(), 0, None);
        assert_eq!(out.matches("[Filename]").count(), 1);
        assert!(out.contains("a.txt — /v/a.txt"));
        assert!(out.contains("# 3 hit(s)"));
    }
}
