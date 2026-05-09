//! Plain-text + Markdown extractor.
//!
//! Reads up to [`PLAIN_TEXT_CAP_BYTES`] from disk and pushes UTF-8
//! decoded text into the sink. Recognizes UTF-8 / UTF-16 LE / UTF-16 BE
//! byte-order marks; on UTF-16 the decoder surrogate-pairs through and
//! re-encodes as UTF-8 because the framework's sink is UTF-8 only.
//!
//! Build Guide Phase 8: "read whole file (cap at 5MB), UTF-8 decode
//! with BOM detection."
//!
//! This extractor is the dispatcher's last line of defense — it claims
//! any path with a `.txt`, `.md`, `.markdown`, `.text`, `.log`, `.rst`,
//! or `.adoc` extension regardless of magic, plus any file whose head
//! parses as valid UTF-8 with no embedded NULs (the magic-byte
//! heuristic for "this is human-readable text"). Source code files are
//! caught earlier by the [code extractor](super::code); structured data
//! files by the [structured-data extractor](super::structured); office
//! docs by the [office extractors](super::office). Plain-text only sees
//! files no other extractor claimed.

use std::fs::File;
use std::io::Read;
use std::path::Path;

use crate::sink::TextSink;
use crate::{ExtractError, ExtractionStats, Extractor, ExtractorId};

/// Hard cap on bytes the plain-text extractor reads from disk. Build
/// Guide pegs this at 5 MiB; a runaway log file shouldn't fill the
/// sink (which has its own 16 MiB cap, but we'd rather fail fast on
/// the source-side read than let a 100 MB log churn through the
/// decoder).
pub const PLAIN_TEXT_CAP_BYTES: usize = 5 * 1024 * 1024;

const UTF8_BOM: [u8; 3] = [0xEF, 0xBB, 0xBF];
const UTF16_LE_BOM: [u8; 2] = [0xFF, 0xFE];
const UTF16_BE_BOM: [u8; 2] = [0xFE, 0xFF];

/// Extensions the plain-text extractor claims by name. Any other path
/// is claimed only when its head bytes parse as UTF-8 / UTF-16 with
/// no embedded NULs (heuristic for "this is text").
const TEXT_EXTENSIONS: &[&str] = &[
    "txt", "text", "md", "markdown", "mkd", "log", "rst", "adoc", "asciidoc", "rtf",
];

#[derive(Debug, Clone, Copy)]
enum Encoding {
    Utf8,
    Utf16Le,
    Utf16Be,
}

#[derive(Debug, Default, Clone)]
pub struct PlainTextExtractor {
    /// Override the default 5 MiB read cap. Tests use this to verify
    /// the truncation contract without writing a 5 MiB fixture.
    pub max_bytes: Option<usize>,
}

impl PlainTextExtractor {
    pub fn with_max_bytes(max_bytes: usize) -> Self {
        Self {
            max_bytes: Some(max_bytes),
        }
    }

    fn cap(&self) -> usize {
        self.max_bytes.unwrap_or(PLAIN_TEXT_CAP_BYTES)
    }

    /// Detect encoding by leading BOM. Files with no BOM default to
    /// UTF-8 — a UTF-16 file without a BOM looks like text-with-NULs
    /// to us and the heuristic in [`looks_like_text`] rejects it.
    fn detect_encoding(buf: &[u8]) -> (Encoding, usize) {
        if buf.starts_with(&UTF8_BOM) {
            (Encoding::Utf8, UTF8_BOM.len())
        } else if buf.starts_with(&UTF16_LE_BOM) {
            (Encoding::Utf16Le, UTF16_LE_BOM.len())
        } else if buf.starts_with(&UTF16_BE_BOM) {
            (Encoding::Utf16Be, UTF16_BE_BOM.len())
        } else {
            (Encoding::Utf8, 0)
        }
    }

    fn decode(buf: &[u8], encoding: Encoding) -> Result<String, ExtractError> {
        match encoding {
            Encoding::Utf8 => std::str::from_utf8(buf)
                .map(str::to_owned)
                .map_err(|e| ExtractError::Malformed(format!("UTF-8 decode failed: {e}"))),
            Encoding::Utf16Le | Encoding::Utf16Be => {
                if buf.len() % 2 != 0 {
                    return Err(ExtractError::Malformed(
                        "UTF-16 input has odd byte length".into(),
                    ));
                }
                let units: Vec<u16> = buf
                    .chunks_exact(2)
                    .map(|c| {
                        let bytes = [c[0], c[1]];
                        match encoding {
                            Encoding::Utf16Le => u16::from_le_bytes(bytes),
                            Encoding::Utf16Be => u16::from_be_bytes(bytes),
                            // Unreachable: outer match already filtered to UTF-16.
                            Encoding::Utf8 => unreachable!(),
                        }
                    })
                    .collect();
                String::from_utf16(&units)
                    .map_err(|e| ExtractError::Malformed(format!("UTF-16 decode failed: {e}")))
            }
        }
    }
}

/// Heuristic for "this raw byte slice is human-readable text". Used
/// when the path doesn't have a recognized extension. Rules:
///
///   * Up to 1 NUL is tolerated (very rare BOM-less UTF-16 happens to
///     have NULs); 2+ NULs flips us to "binary".
///   * The remainder must be valid UTF-8 once any leading BOM is
///     stripped.
///
/// Cheap: one pass of the head bytes, no allocation.
fn looks_like_text(magic: &[u8]) -> bool {
    if magic.is_empty() {
        return false;
    }
    let body = if magic.starts_with(&UTF8_BOM) {
        &magic[UTF8_BOM.len()..]
    } else {
        magic
    };
    let nul_count = body.iter().filter(|b| **b == 0).count();
    if nul_count >= 2 {
        return false;
    }
    std::str::from_utf8(body).is_ok()
}

impl Extractor for PlainTextExtractor {
    fn id(&self) -> ExtractorId {
        ExtractorId::new("plain-text")
    }

    fn matches(&self, path: &Path, magic: &[u8]) -> bool {
        // Extension match wins fast.
        if let Some(ext) = path.extension().and_then(|s| s.to_str()) {
            let lower = ext.to_ascii_lowercase();
            if TEXT_EXTENSIONS.iter().any(|e| *e == lower) {
                return true;
            }
        }
        // Fallback: bytes look text-shaped. UTF-16 BOMs implicitly
        // claim the file too — the [`detect_encoding`] path will pick
        // them up after the dispatcher hands us the file.
        magic.starts_with(&UTF16_LE_BOM)
            || magic.starts_with(&UTF16_BE_BOM)
            || magic.starts_with(&UTF8_BOM)
            || looks_like_text(magic)
    }

    fn extract(&self, path: &Path, sink: &mut TextSink) -> Result<ExtractionStats, ExtractError> {
        let cap = self.cap();
        let mut file = File::open(path).map_err(|e| ExtractError::io(path, e))?;
        let mut buf = Vec::with_capacity(cap.min(64 * 1024));
        // `take` truncates the underlying read at `cap` bytes — anything
        // past that is silently ignored, which is the documented Phase 8
        // contract for plain text. We use `+ 1` so a file exactly at the
        // cap reads cleanly without flagging truncation; truncation is
        // observable via the file's actual on-disk size.
        let mut limited = (&mut file).take(cap as u64 + 1);
        limited
            .read_to_end(&mut buf)
            .map_err(|e| ExtractError::io(path, e))?;

        if sink.is_cancelled() {
            return Err(ExtractError::Cancelled);
        }

        // Detect encoding *before* truncating so the truncation can
        // honor the encoding's codepoint boundary. The previous version
        // used a UTF-8 from_utf8 loop on the truncated buffer regardless
        // of encoding — for UTF-16 input that overshot the cap, the
        // loop popped bytes until it found a "valid UTF-8 prefix" which
        // could leave the buffer at an odd byte length and surface as
        // ExtractError::Malformed at decode time. Detecting first means
        // a UTF-16 file is trimmed to an even-byte boundary, a UTF-8
        // file to a complete-codepoint boundary, and the BOM at the
        // start is preserved across truncation either way (the BOM
        // lives in the first 2-3 bytes of the buffer; truncation only
        // ever removes from the tail).
        let (encoding, skip) = Self::detect_encoding(&buf);

        if buf.len() > cap {
            buf.truncate(cap);
            match encoding {
                Encoding::Utf8 => super::util::trim_to_utf8_boundary(&mut buf),
                Encoding::Utf16Le | Encoding::Utf16Be => {
                    // Trim to even byte count so chunks_exact(2) sees a
                    // whole number of code units. A truncated trailing
                    // high surrogate (no matching low) surfaces from the
                    // String::from_utf16 decoder as Malformed; that is
                    // the right surface — the file genuinely lost its
                    // tail when we hit the cap.
                    if buf.len() % 2 != 0 {
                        buf.truncate(buf.len() - 1);
                    }
                }
            }
        }

        let bytes_in = buf.len() as u64;
        let body = &buf[skip..];
        let text = Self::decode(body, encoding)?;

        sink.push_str(&text)
            .map_err(|_| ExtractError::OutputTooLarge { cap: sink.cap() })?;

        Ok(ExtractionStats {
            bytes_in,
            bytes_out: sink.len() as u64,
            elapsed: std::time::Duration::ZERO,
        })
    }
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
    fn matches_txt_by_extension() {
        let ext = PlainTextExtractor::default();
        assert!(ext.matches(Path::new("/tmp/notes.txt"), b"hi"));
        assert!(ext.matches(Path::new("/tmp/README.md"), b"# hi"));
        assert!(ext.matches(Path::new("/tmp/CHANGELOG.markdown"), b"# hi"));
    }

    #[test]
    fn matches_text_shaped_bytes_without_extension() {
        let ext = PlainTextExtractor::default();
        assert!(ext.matches(Path::new("/tmp/anything"), b"plain text content"));
    }

    #[test]
    fn rejects_binary_bytes_without_extension() {
        let ext = PlainTextExtractor::default();
        assert!(!ext.matches(Path::new("/tmp/blob"), b"\x00\x00\x00binary"));
    }

    #[test]
    fn extracts_utf8() {
        let path = write_fixture("u8.txt", b"hello world");
        let ext = PlainTextExtractor::default();
        let mut sink = TextSink::new(64);
        let stats = ext.extract(&path, &mut sink).unwrap();
        assert_eq!(sink.as_bytes(), b"hello world");
        assert_eq!(stats.bytes_in, 11);
        assert_eq!(stats.bytes_out, 11);
    }

    #[test]
    fn strips_utf8_bom() {
        let mut bytes = UTF8_BOM.to_vec();
        bytes.extend_from_slice(b"hello");
        let path = write_fixture("u8bom.txt", &bytes);
        let ext = PlainTextExtractor::default();
        let mut sink = TextSink::new(64);
        ext.extract(&path, &mut sink).unwrap();
        assert_eq!(sink.as_bytes(), b"hello");
    }

    #[test]
    fn decodes_utf16_le_with_bom() {
        // "hi" → 0x68 0x00 0x69 0x00, prefixed with FF FE.
        let mut bytes = UTF16_LE_BOM.to_vec();
        bytes.extend_from_slice(&[0x68, 0x00, 0x69, 0x00]);
        let path = write_fixture("u16le.txt", &bytes);
        let ext = PlainTextExtractor::default();
        let mut sink = TextSink::new(64);
        ext.extract(&path, &mut sink).unwrap();
        assert_eq!(sink.as_bytes(), b"hi");
    }

    #[test]
    fn decodes_utf16_be_with_bom() {
        let mut bytes = UTF16_BE_BOM.to_vec();
        bytes.extend_from_slice(&[0x00, 0x68, 0x00, 0x69]);
        let path = write_fixture("u16be.txt", &bytes);
        let ext = PlainTextExtractor::default();
        let mut sink = TextSink::new(64);
        ext.extract(&path, &mut sink).unwrap();
        assert_eq!(sink.as_bytes(), b"hi");
    }

    #[test]
    fn truncates_at_max_bytes() {
        let big = b"x".repeat(20).to_vec();
        let path = write_fixture("big.txt", &big);
        let ext = PlainTextExtractor::with_max_bytes(8);
        let mut sink = TextSink::new(64);
        let stats = ext.extract(&path, &mut sink).unwrap();
        assert_eq!(sink.as_bytes(), &big[..8]);
        assert_eq!(stats.bytes_in, 8);
    }

    #[test]
    fn malformed_utf8_returns_malformed() {
        let bad = vec![0xC3, 0x28]; // invalid UTF-8 sequence (no NULs)
        let path = write_fixture("bad.txt", &bad);
        let ext = PlainTextExtractor::default();
        let mut sink = TextSink::new(64);
        let err = ext.extract(&path, &mut sink).unwrap_err();
        assert!(
            matches!(err, ExtractError::Malformed(_)),
            "expected Malformed, got {err:?}"
        );
    }

    #[test]
    fn output_too_large_when_sink_full() {
        let path = write_fixture("big2.txt", b"abcdefghij");
        let ext = PlainTextExtractor::default();
        let mut sink = TextSink::new(4);
        let err = ext.extract(&path, &mut sink).unwrap_err();
        assert!(
            matches!(err, ExtractError::OutputTooLarge { cap: 4 }),
            "expected OutputTooLarge(4), got {err:?}"
        );
    }

    /// Phase 8 review pass — regression for the UTF-16 truncation bug.
    /// A UTF-16 LE file that overshoots the read cap used to surface
    /// `ExtractError::Malformed` because the truncation loop ran
    /// `from_utf8` on the buffer (UTF-16 with non-ASCII codepoints
    /// fails UTF-8 validation, and the loop happily popped bytes until
    /// it found an "OK UTF-8 prefix" — often leaving an odd byte
    /// count, which `decode` then rejected). With the encoding-aware
    /// truncation, the file extracts cleanly with the tail dropped
    /// at a 2-byte boundary.
    #[test]
    fn utf16_le_overshoot_truncates_at_even_boundary_not_malformed() {
        // 32 LE-encoded `é` (U+00E9 → E9 00) = 64 bytes of UTF-16
        // payload after the 2-byte BOM. Cap at 24 bytes total — well
        // inside the payload — so the truncation path fires.
        let mut bytes = UTF16_LE_BOM.to_vec();
        for _ in 0..32 {
            bytes.extend_from_slice(&[0xE9, 0x00]);
        }
        let path = write_fixture("u16le_overshoot.txt", &bytes);
        let ext = PlainTextExtractor::with_max_bytes(24);
        let mut sink = TextSink::new(256);
        ext.extract(&path, &mut sink)
            .expect("UTF-16 cap-overshoot must extract cleanly");
        let out = std::str::from_utf8(sink.as_bytes()).unwrap();
        // 24 bytes - 2 BOM = 22 bytes payload = 11 LE code units.
        assert_eq!(out.chars().count(), 11);
        assert!(out.chars().all(|c| c == 'é'));
    }
}
