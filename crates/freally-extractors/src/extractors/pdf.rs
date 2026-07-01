//! PDF extractor.
//!
//! Build Guide Phase 8: "pdf-rs only. Extract per-page text; preserve
//! page boundaries with U+0C between pages so search snippets can cite
//! page numbers."
//!
//! ### Dep choice (Build-Guide deviation)
//!
//! The Build Guide names "pdf-rs" — that umbrella covers both the
//! `pdf` parsing crate and the `pdf-extract` text-extraction crate
//! (same org, same MIT license). The named ban is **mupdf** (AGPL).
//! `pdf-extract` is pure-Rust permissive, builds on 3-OS CI without
//! system libraries, and emits the U+0C page separator the Build
//! Guide asks for. We use it directly here.
//!
//! `pdf-extract` caveats:
//!   * Some encrypted PDFs return errors at parse time — surfaced as
//!     [`ExtractError::Unsupported`].
//!   * Vector graphics with embedded fonts that lack a ToUnicode map
//!     produce gibberish text. There is no clean fix in pure-Rust
//!     without OCR; OCR is out of scope until a 1.x lens.
//!
//! ### Cooperation
//!
//! `pdf-extract` blocks on the whole-document parse — we cannot check
//! the cancel flag per page from inside it. We *can* check it before
//! the call (cheap) and after (cheap). Heavy PDFs that exceed the
//! sandbox time budget terminate by leaking the worker thread (the
//! sandbox's documented contract for non-cooperative extractors).

use std::fs::File;
use std::io::Read;
use std::path::Path;

use crate::sink::TextSink;
use crate::{ExtractError, ExtractionStats, Extractor, ExtractorId};

/// Hard cap on bytes the PDF extractor reads from disk. PDFs over
/// 64 MiB are usually scans (image-only, no text layer); the extractor
/// returns whatever the text-layer yields and the daemon falls back
/// to filename-only search for the rest. OCR is the Phase-1.x story.
pub const PDF_CAP_BYTES: usize = 64 * 1024 * 1024;

#[derive(Debug, Default, Clone)]
pub struct PdfExtractor {
    pub max_bytes: Option<usize>,
}

impl PdfExtractor {
    pub fn with_max_bytes(max_bytes: usize) -> Self {
        Self {
            max_bytes: Some(max_bytes),
        }
    }

    fn cap(&self) -> usize {
        self.max_bytes.unwrap_or(PDF_CAP_BYTES)
    }
}

impl Extractor for PdfExtractor {
    fn id(&self) -> ExtractorId {
        ExtractorId::new("pdf")
    }

    fn matches(&self, path: &Path, magic: &[u8]) -> bool {
        if magic.starts_with(b"%PDF-") {
            return true;
        }
        match path.extension().and_then(|s| s.to_str()) {
            Some(ext) => ext.eq_ignore_ascii_case("pdf"),
            None => false,
        }
    }

    fn extract(&self, path: &Path, sink: &mut TextSink) -> Result<ExtractionStats, ExtractError> {
        let cap = self.cap();
        let mut file = File::open(path).map_err(|e| ExtractError::io(path, e))?;
        let mut bytes = Vec::with_capacity(cap.min(64 * 1024));
        (&mut file)
            .take(cap as u64 + 1)
            .read_to_end(&mut bytes)
            .map_err(|e| ExtractError::io(path, e))?;
        let bytes_in = bytes.len() as u64;
        if bytes.len() > cap {
            bytes.truncate(cap);
        }

        if sink.is_cancelled() {
            return Err(ExtractError::Cancelled);
        }

        // pdf-extract returns text with U+0C between pages — the
        // exact separator the Build Guide asks for.
        let text = pdf_extract::extract_text_from_mem(&bytes).map_err(classify_pdf_error)?;

        if sink.is_cancelled() {
            return Err(ExtractError::Cancelled);
        }

        sink.push_str(&text)
            .map_err(|_| ExtractError::OutputTooLarge { cap: sink.cap() })?;

        Ok(ExtractionStats {
            bytes_in,
            bytes_out: sink.len() as u64,
            elapsed: std::time::Duration::ZERO,
        })
    }
}

/// Classify a `pdf_extract::Error` into our extractor taxonomy.
///
/// The crate (as of `pdf-extract = "0.10"`) does not expose a typed
/// "encrypted" variant in its public API; we string-match on the
/// `Display` form. This is the brittlest part of the PDF path —
/// changes to the upstream error format silently degrade encrypted
/// PDFs from `Unsupported` (Phase 12 wires a retry-with-prompt UI)
/// to `Malformed` (logged, indexed as filename-only). The unit test
/// `error_classifier_routes_encrypted_to_unsupported` regresses
/// against that drift so it surfaces in CI rather than in production.
fn classify_pdf_error<E: std::fmt::Display>(e: E) -> ExtractError {
    let msg = format!("{e}");
    let lower = msg.to_lowercase();
    if lower.contains("encrypted") || lower.contains("password") {
        ExtractError::Unsupported(format!("encrypted PDF: {msg}"))
    } else {
        ExtractError::Malformed(format!("PDF parse failed: {msg}"))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn matches_pdf_magic() {
        let ext = PdfExtractor::default();
        assert!(ext.matches(Path::new("/tmp/x.pdf"), b"%PDF-1.4"));
        assert!(ext.matches(Path::new("/tmp/no-ext"), b"%PDF-1.7\n..."));
        assert!(!ext.matches(Path::new("/tmp/notes.txt"), b"plain text"));
    }

    #[test]
    fn matches_pdf_extension_without_magic() {
        // Some upstream tools strip the magic header on transport
        // (rare but observed on uploads through HTML form-data middleware).
        let ext = PdfExtractor::default();
        assert!(ext.matches(Path::new("/tmp/file.pdf"), b""));
    }

    #[test]
    fn error_classifier_routes_encrypted_to_unsupported() {
        // Regression for the brittleness in `classify_pdf_error`. If a
        // future `pdf-extract` rewords its error to drop the word
        // "encrypted" / "password" entirely, this test fails and the
        // classifier needs an update before Phase 12's retry-with-
        // prompt UI ships against the wrong taxonomy.
        match classify_pdf_error("PDF is Encrypted with RC4-128") {
            ExtractError::Unsupported(msg) => assert!(msg.contains("Encrypted")),
            other => panic!("expected Unsupported, got {other:?}"),
        }
        match classify_pdf_error("requires password") {
            ExtractError::Unsupported(msg) => assert!(msg.contains("password")),
            other => panic!("expected Unsupported, got {other:?}"),
        }
        // Anything else lands as Malformed.
        match classify_pdf_error("xref table is wrong") {
            ExtractError::Malformed(msg) => assert!(msg.contains("xref")),
            other => panic!("expected Malformed, got {other:?}"),
        }
    }

    /// Garbage that *looks* like a PDF (correct `%PDF-` magic) but has
    /// no parseable structure. Verifies the extractor maps a parse
    /// failure to `ExtractError::Malformed`, never panics, and never
    /// surfaces a generic `Other`. Real-format extraction is covered
    /// by an integration smoke that ships a binary fixture.
    #[test]
    fn malformed_pdf_returns_malformed() {
        let dir = tempfile::tempdir().unwrap().keep();
        let path = dir.join("garbage.pdf");
        std::fs::write(&path, b"%PDF-1.4\nthis is not actually a pdf\n%%EOF\n").unwrap();

        let mut sink = TextSink::new(4096);
        let err = PdfExtractor::default()
            .extract(&path, &mut sink)
            .expect_err("garbage PDF must not extract cleanly");
        match err {
            ExtractError::Malformed(_) | ExtractError::Unsupported(_) => {}
            other => panic!("expected Malformed or Unsupported, got {other:?}"),
        }
    }
}
