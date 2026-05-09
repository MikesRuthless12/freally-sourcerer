//! Structured-data extractor — JSON / CSV / YAML.
//!
//! Build Guide Phase 8: "serde_json/csv/serde_yaml; flatten to
//! key=value text for full-text search." We use `serde_yaml_ng`
//! (actively-maintained MIT fork of serde_yaml) instead of the
//! upstream serde_yaml, which RustSec marked unmaintained in
//! mid-2024. The flattening shape is:
//!
//!   * JSON / YAML — recursive `key.subkey[3]=value` keys, one per
//!     line. Arrays use `[idx]`; objects use dotted keys. Scalars
//!     (string / number / bool / null) become the right-hand side of
//!     the `=`. Search snippets read naturally: `address.zip=10001`.
//!   * CSV — header-driven `column=value` keys, one per non-header
//!     row. Files without a header degrade to `col0=value col1=value`.
//!
//! Each scalar / row writes a separate line so a `content:zip=10001`
//! query matches without false positives across rows.

use std::fs::File;
use std::io::{BufReader, Read};
use std::path::Path;

use serde::Deserialize;

use crate::sink::TextSink;
use crate::{ExtractError, ExtractionStats, Extractor, ExtractorId};

/// Hard cap on bytes the structured-data extractor reads from disk.
/// 16 MiB matches the sink's default cap; structured-data files larger
/// than that are usually not human-curated and the long tail rarely
/// pays for itself in search-relevance terms.
pub const STRUCTURED_DATA_CAP_BYTES: usize = 16 * 1024 * 1024;

/// Coarse cancel-check granularity. Big inputs (10MB+) get checked
/// every N rows / values so the supervisor can interrupt within a
/// sub-second budget without per-byte overhead.
const CANCEL_CHECK_EVERY: usize = 256;

#[derive(Debug, Default, Clone)]
pub struct StructuredDataExtractor {
    pub max_bytes: Option<usize>,
}

impl StructuredDataExtractor {
    pub fn with_max_bytes(max_bytes: usize) -> Self {
        Self {
            max_bytes: Some(max_bytes),
        }
    }

    fn cap(&self) -> usize {
        self.max_bytes.unwrap_or(STRUCTURED_DATA_CAP_BYTES)
    }

    fn detect_format(path: &Path, magic: &[u8]) -> Option<Format> {
        match path.extension().and_then(|s| s.to_str()) {
            Some(ext) => {
                let lower = ext.to_ascii_lowercase();
                match lower.as_str() {
                    "json" | "jsonc" | "ndjson" | "json5" => Some(Format::Json),
                    "yaml" | "yml" => Some(Format::Yaml),
                    "csv" | "tsv" => Some(Format::Csv {
                        tab: lower == "tsv",
                    }),
                    _ => Self::detect_by_magic(magic),
                }
            }
            None => Self::detect_by_magic(magic),
        }
    }

    fn detect_by_magic(magic: &[u8]) -> Option<Format> {
        // Strip BOM if present so JSON/YAML detection sees the real
        // first non-whitespace byte.
        let bytes = magic.strip_prefix(b"\xEF\xBB\xBF").unwrap_or(magic);
        let head = bytes
            .iter()
            .copied()
            .find(|b| !b.is_ascii_whitespace())
            .unwrap_or(0);
        match head {
            b'{' | b'[' => Some(Format::Json),
            // No reliable single-byte YAML magic; rely on extension.
            _ => None,
        }
    }
}

#[derive(Debug, Clone, Copy)]
enum Format {
    Json,
    Yaml,
    Csv { tab: bool },
}

impl Extractor for StructuredDataExtractor {
    fn id(&self) -> ExtractorId {
        ExtractorId::new("structured")
    }

    fn matches(&self, path: &Path, magic: &[u8]) -> bool {
        Self::detect_format(path, magic).is_some()
    }

    fn extract(&self, path: &Path, sink: &mut TextSink) -> Result<ExtractionStats, ExtractError> {
        let format = Self::detect_format(path, &[]).ok_or_else(|| {
            ExtractError::Unsupported("path does not look like JSON/YAML/CSV".into())
        })?;

        let cap = self.cap();
        let file = File::open(path).map_err(|e| ExtractError::io(path, e))?;
        let mut reader = BufReader::new(file).take(cap as u64 + 1);
        let mut buf = Vec::with_capacity(cap.min(64 * 1024));
        reader
            .read_to_end(&mut buf)
            .map_err(|e| ExtractError::io(path, e))?;

        if buf.len() > cap {
            buf.truncate(cap);
        }

        if sink.is_cancelled() {
            return Err(ExtractError::Cancelled);
        }

        let bytes_in = buf.len() as u64;
        match format {
            Format::Json => extract_json(&buf, sink)?,
            Format::Yaml => extract_yaml(&buf, sink)?,
            Format::Csv { tab } => extract_csv(&buf, tab, sink)?,
        }

        Ok(ExtractionStats {
            bytes_in,
            bytes_out: sink.len() as u64,
            elapsed: std::time::Duration::ZERO,
        })
    }
}

fn extract_json(buf: &[u8], sink: &mut TextSink) -> Result<(), ExtractError> {
    let value: serde_json::Value = serde_json::from_slice(buf)
        .map_err(|e| ExtractError::Malformed(format!("invalid JSON: {e}")))?;
    let mut count = 0usize;
    flatten_json(&value, &mut String::new(), sink, &mut count)
}

fn flatten_json(
    value: &serde_json::Value,
    prefix: &mut String,
    sink: &mut TextSink,
    counter: &mut usize,
) -> Result<(), ExtractError> {
    match value {
        serde_json::Value::Null => write_kv(prefix, "null", sink, counter),
        serde_json::Value::Bool(b) => {
            write_kv(prefix, if *b { "true" } else { "false" }, sink, counter)
        }
        serde_json::Value::Number(n) => write_kv(prefix, &n.to_string(), sink, counter),
        serde_json::Value::String(s) => write_kv(prefix, s, sink, counter),
        serde_json::Value::Array(items) => {
            for (i, item) in items.iter().enumerate() {
                let saved_len = prefix.len();
                use std::fmt::Write;
                // Writing into a String never fails; ignore the unit
                // result of `write!` on a fixed buffer.
                let _ = write!(prefix, "[{i}]");
                flatten_json(item, prefix, sink, counter)?;
                prefix.truncate(saved_len);
            }
            Ok(())
        }
        serde_json::Value::Object(map) => {
            for (k, v) in map {
                let saved_len = prefix.len();
                if !prefix.is_empty() {
                    prefix.push('.');
                }
                prefix.push_str(k);
                flatten_json(v, prefix, sink, counter)?;
                prefix.truncate(saved_len);
            }
            Ok(())
        }
    }
}

fn extract_yaml(buf: &[u8], sink: &mut TextSink) -> Result<(), ExtractError> {
    let s = std::str::from_utf8(buf)
        .map_err(|e| ExtractError::Malformed(format!("YAML must be UTF-8: {e}")))?;
    // Multi-document YAML — flatten each document with a [doc=N]
    // prefix so search snippets distinguish them.
    let docs: Vec<serde_yaml_ng::Value> = serde_yaml_ng::Deserializer::from_str(s)
        .map(serde_yaml_ng::Value::deserialize)
        .collect::<Result<_, _>>()
        .map_err(|e| ExtractError::Malformed(format!("invalid YAML: {e}")))?;

    let mut counter = 0usize;
    let multi_doc = docs.len() > 1;
    for (i, value) in docs.iter().enumerate() {
        let mut prefix = if multi_doc {
            format!("[doc={i}]")
        } else {
            String::new()
        };
        flatten_yaml(value, &mut prefix, sink, &mut counter)?;
    }
    Ok(())
}

fn flatten_yaml(
    value: &serde_yaml_ng::Value,
    prefix: &mut String,
    sink: &mut TextSink,
    counter: &mut usize,
) -> Result<(), ExtractError> {
    use serde_yaml_ng::Value;
    match value {
        Value::Null => write_kv(prefix, "null", sink, counter),
        Value::Bool(b) => write_kv(prefix, if *b { "true" } else { "false" }, sink, counter),
        Value::Number(n) => write_kv(prefix, &n.to_string(), sink, counter),
        Value::String(s) => write_kv(prefix, s, sink, counter),
        Value::Sequence(items) => {
            for (i, item) in items.iter().enumerate() {
                let saved_len = prefix.len();
                use std::fmt::Write;
                let _ = write!(prefix, "[{i}]");
                flatten_yaml(item, prefix, sink, counter)?;
                prefix.truncate(saved_len);
            }
            Ok(())
        }
        Value::Mapping(map) => {
            for (k, v) in map {
                let saved_len = prefix.len();
                if !prefix.is_empty() {
                    prefix.push('.');
                }
                // YAML keys can be any value. Stringify with `to_string`
                // for non-scalars (rare) so search at least sees a path.
                match k {
                    Value::String(s) => prefix.push_str(s),
                    Value::Number(n) => prefix.push_str(&n.to_string()),
                    Value::Bool(b) => prefix.push_str(if *b { "true" } else { "false" }),
                    other => prefix.push_str(&format!("{other:?}")),
                }
                flatten_yaml(v, prefix, sink, counter)?;
                prefix.truncate(saved_len);
            }
            Ok(())
        }
        Value::Tagged(t) => flatten_yaml(&t.value, prefix, sink, counter),
    }
}

fn extract_csv(buf: &[u8], tab_separated: bool, sink: &mut TextSink) -> Result<(), ExtractError> {
    let mut rdr = csv::ReaderBuilder::new()
        .delimiter(if tab_separated { b'\t' } else { b',' })
        .has_headers(true)
        .flexible(true)
        .from_reader(buf);
    let headers: Vec<String> = rdr
        .headers()
        .map_err(|e| ExtractError::Malformed(format!("CSV header read failed: {e}")))?
        .iter()
        .map(str::to_owned)
        .collect();
    for (row_count, result) in rdr.records().enumerate() {
        let row = result.map_err(|e| ExtractError::Malformed(format!("CSV row failed: {e}")))?;
        if row_count % CANCEL_CHECK_EVERY == 0 && sink.is_cancelled() {
            return Err(ExtractError::Cancelled);
        }
        for (i, field) in row.iter().enumerate() {
            // Quoted CSV fields can legitimately carry embedded `\n` /
            // `\r`; emitted raw they would split one row into many in
            // the search blob. Escape line-break controls so the
            // line-per-record contract holds.
            let safe_field = super::util::sanitize_inline(field);
            let line = match headers.get(i) {
                Some(key) => {
                    let safe_key = super::util::sanitize_inline(key);
                    format!("{safe_key}={safe_field}\n")
                }
                // Past-header column: synthesize a stable `colN=` key so
                // the row stays searchable when a flexible CSV widens
                // mid-file.
                None => format!("col{i}={safe_field}\n"),
            };
            sink.push_str(&line)
                .map_err(|_| ExtractError::OutputTooLarge { cap: sink.cap() })?;
        }
    }
    Ok(())
}

fn write_kv(
    key: &str,
    value: &str,
    sink: &mut TextSink,
    counter: &mut usize,
) -> Result<(), ExtractError> {
    *counter += 1;
    if *counter % CANCEL_CHECK_EVERY == 0 && sink.is_cancelled() {
        return Err(ExtractError::Cancelled);
    }
    // JSON / YAML scalar string values can legitimately contain
    // `\n` / `\r` — emitting them raw would split one logical record
    // into multiple lines in the search blob and let one value
    // impersonate many. Sanitise key + value before format!.
    let safe_value = super::util::sanitize_inline(value);
    let line = if key.is_empty() {
        format!("{safe_value}\n")
    } else {
        let safe_key = super::util::sanitize_inline(key);
        format!("{safe_key}={safe_value}\n")
    };
    sink.push_str(&line)
        .map_err(|_| ExtractError::OutputTooLarge { cap: sink.cap() })?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Write;
    use tempfile::tempdir;

    fn write_fixture(name: &str, contents: &[u8]) -> std::path::PathBuf {
        let dir = tempdir().unwrap().keep();
        let path = dir.join(name);
        let mut f = File::create(&path).unwrap();
        f.write_all(contents).unwrap();
        path
    }

    #[test]
    fn matches_json_by_extension() {
        let ext = StructuredDataExtractor::default();
        assert!(ext.matches(Path::new("/tmp/data.json"), b""));
        assert!(ext.matches(Path::new("/tmp/data.yaml"), b""));
        assert!(ext.matches(Path::new("/tmp/data.csv"), b""));
        assert!(!ext.matches(Path::new("/tmp/notes.txt"), b""));
    }

    #[test]
    fn matches_json_by_magic() {
        let ext = StructuredDataExtractor::default();
        assert!(ext.matches(Path::new("/tmp/no-ext"), b"   {\"a\":1}"));
        assert!(ext.matches(Path::new("/tmp/no-ext"), b"[1,2,3]"));
    }

    #[test]
    fn flattens_json_object() {
        let path = write_fixture("d.json", br#"{"a":1,"b":{"c":"hi"}}"#);
        let ext = StructuredDataExtractor::default();
        let mut sink = TextSink::new(256);
        ext.extract(&path, &mut sink).unwrap();
        let out = std::str::from_utf8(sink.as_bytes()).unwrap();
        assert!(out.contains("a=1"));
        assert!(out.contains("b.c=hi"));
    }

    #[test]
    fn flattens_json_array_with_indices() {
        let path = write_fixture("d.json", br#"{"items":["x","y"]}"#);
        let ext = StructuredDataExtractor::default();
        let mut sink = TextSink::new(256);
        ext.extract(&path, &mut sink).unwrap();
        let out = std::str::from_utf8(sink.as_bytes()).unwrap();
        assert!(out.contains("items[0]=x"));
        assert!(out.contains("items[1]=y"));
    }

    #[test]
    fn flattens_yaml() {
        let path = write_fixture("d.yaml", b"name: alice\naddress:\n  zip: 10001\n");
        let ext = StructuredDataExtractor::default();
        let mut sink = TextSink::new(256);
        ext.extract(&path, &mut sink).unwrap();
        let out = std::str::from_utf8(sink.as_bytes()).unwrap();
        assert!(out.contains("name=alice"));
        assert!(out.contains("address.zip=10001"));
    }

    #[test]
    fn flattens_csv_with_headers() {
        let path = write_fixture("d.csv", b"name,zip\nalice,10001\nbob,90210\n");
        let ext = StructuredDataExtractor::default();
        let mut sink = TextSink::new(256);
        ext.extract(&path, &mut sink).unwrap();
        let out = std::str::from_utf8(sink.as_bytes()).unwrap();
        assert!(out.contains("name=alice"));
        assert!(out.contains("zip=10001"));
        assert!(out.contains("name=bob"));
        assert!(out.contains("zip=90210"));
    }

    #[test]
    fn rejects_invalid_json() {
        let path = write_fixture("bad.json", b"{not json");
        let ext = StructuredDataExtractor::default();
        let mut sink = TextSink::new(64);
        let err = ext.extract(&path, &mut sink).unwrap_err();
        assert!(matches!(err, ExtractError::Malformed(_)));
    }

    /// Phase 8 review pass — the multi-doc YAML path uses `[doc=N]`
    /// prefixes per document. Locks the prefix logic behind a test so
    /// a future refactor can't silently drop document boundaries.
    #[test]
    fn flattens_multi_document_yaml_with_doc_prefix() {
        let path = write_fixture("multi.yaml", b"---\nname: alice\n---\nname: bob\n");
        let ext = StructuredDataExtractor::default();
        let mut sink = TextSink::new(256);
        ext.extract(&path, &mut sink).unwrap();
        let out = std::str::from_utf8(sink.as_bytes()).unwrap();
        assert!(out.contains("[doc=0].name=alice"));
        assert!(out.contains("[doc=1].name=bob"));
    }

    /// Phase 8 review pass — regression for the CSV value-injection
    /// bug. A quoted CSV field with an embedded `\n` used to emit two
    /// lines in the sink, letting one row impersonate two in the
    /// search index. `sanitize_inline` escapes `\n` / `\r` / `\0` so
    /// the line-per-record contract holds even on hostile CSVs.
    #[test]
    fn csv_field_with_newline_is_sanitized() {
        let path = write_fixture(
            "evil.csv",
            b"name,note\nalice,\"line1\nfake-row=injected\"\n",
        );
        let ext = StructuredDataExtractor::default();
        let mut sink = TextSink::new(256);
        ext.extract(&path, &mut sink).unwrap();
        let out = std::str::from_utf8(sink.as_bytes()).unwrap();
        // Two key=value lines for the single row (one per column).
        assert_eq!(out.lines().count(), 2);
        assert!(out.contains("name=alice"));
        assert!(out.contains(r"note=line1\nfake-row=injected"));
        // No raw line-break inside a key=value line.
        assert!(!out.contains("note=line1\n"));
    }

    /// Phase 8 review pass — same fix as CSV, but at the JSON / YAML
    /// scalar surface. A JSON string value that contains `\n` is
    /// legitimate JSON; emitting it raw would inject phantom rows.
    #[test]
    fn json_string_value_with_newline_is_sanitized() {
        let path = write_fixture("evil.json", br#"{"note": "line1\nfake-row=injected"}"#);
        let ext = StructuredDataExtractor::default();
        let mut sink = TextSink::new(256);
        ext.extract(&path, &mut sink).unwrap();
        let out = std::str::from_utf8(sink.as_bytes()).unwrap();
        assert_eq!(out.lines().count(), 1);
        assert!(out.contains(r"note=line1\nfake-row=injected"));
    }
}
