//! Phase 8 extractors — six implementations that plug into the Phase 7
//! framework. Each module exposes a public extractor type that
//! implements [`crate::Extractor`]; the [`register_all`] /
//! [`default_pipeline`] helpers build a [`crate::Pipeline`] with all
//! six registered in dispatch order.
//!
//! ## Dispatch order (registration order = dispatch order)
//!
//! Eight registrations cover the six logical extractors (the Office
//! extractor expands to three sub-extractors so each format gets its
//! own claim):
//!
//!   1. [`pdf::PdfExtractor`] — `%PDF-` magic.
//!   2. [`office::XlsxExtractor`] — `.xlsx` / `.xlsm` (zip + sheets).
//!   3. [`office::DocxExtractor`] — `.docx` (zip + word/document.xml).
//!   4. [`office::PptxExtractor`] — `.pptx` (zip + ppt/slides/*.xml).
//!   5. [`archive::ArchivePeekExtractor`] — zip / 7z / tar listing only.
//!   6. [`code::CodeExtractor`] — source files by extension.
//!   7. [`structured::StructuredDataExtractor`] — JSON / CSV / YAML.
//!   8. [`plain_text::PlainTextExtractor`] — txt / md / generic UTF-8 last.
//!
//! Office variants come **before** the generic archive-peek extractor
//! because docx / xlsx / pptx are zip files and would otherwise be
//! claimed by archive-peek's magic match.
//!
//! ## Sandbox cooperation
//!
//! Every Phase 8 extractor cooperates with the Phase 7 sandbox cancel
//! flag at coarse boundaries (per-page, per-row, per-archive-entry)
//! and bails with [`ExtractError::Cancelled`](crate::ExtractError::Cancelled).
//! The PDF extractor is the one exception: `pdf-extract` blocks on
//! the whole-document parse, so non-cooperative behavior is documented
//! contract — Phase 13 evaluates subprocess isolation for it.

pub mod archive;
pub mod code;
pub mod office;
pub mod pdf;
pub mod plain_text;
pub mod structured;
mod util;

pub use archive::ArchivePeekExtractor;
pub use code::CodeExtractor;
pub use office::{DocxExtractor, PptxExtractor, XlsxExtractor};
pub use pdf::PdfExtractor;
pub use plain_text::PlainTextExtractor;
pub use structured::StructuredDataExtractor;

use crate::pipeline::PipelineBuilder;

/// Register all eight Phase 8 extractor entries on a fresh
/// [`PipelineBuilder`](crate::pipeline::PipelineBuilder) in the
/// dispatch order documented at the top of this module. The daemon
/// calls this once at startup; tests can call it to get a "every
/// Phase 8 extractor present" pipeline.
pub fn register_all(builder: PipelineBuilder) -> PipelineBuilder {
    builder
        .register(PdfExtractor::default())
        .register(XlsxExtractor)
        .register(DocxExtractor)
        .register(PptxExtractor)
        .register(ArchivePeekExtractor::default())
        .register(CodeExtractor::default())
        .register(StructuredDataExtractor::default())
        .register(PlainTextExtractor::default())
}

/// Convenience: a [`Pipeline`](crate::pipeline::Pipeline) with every
/// Phase 8 extractor registered against
/// [`PipelineSettings::default`](crate::PipelineSettings::default) —
/// every extractor is *registered* but the per-extractor mode is
/// `Lazy` (Phase 7 default), so extractions only run on first
/// relevant query rather than at index time. Phase 12's settings
/// dialog flips individual formats to `Eager` per user preference.
pub fn default_pipeline() -> crate::pipeline::Pipeline {
    register_all(crate::pipeline::Pipeline::builder()).build()
}
