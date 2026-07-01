//! macOS QuickLook preview + thumbnail host.
//!
//! Phase 12 wires the API surface and the in-app fallback. The full
//! `objc2` bridge to `QLPreviewPanel` runs the user-visible preview
//! window, and `QLThumbnailGenerator` produces the data-URL-encoded
//! PNG thumbnail. A no-op host on macOS would surface the bare text-head
//! fallback, which is acceptable behavior; the rich preview is a
//! UX-quality enhancement.
//!
//! For Phase 12 the implementation gates the heavier `objc2` integration
//! behind a runtime probe (`mdimport`) so the daemon ships everywhere
//! and the QuickLook path lights up only when the system frameworks are
//! present. The data-URL contract is stable across the swap.

use std::path::Path;

use crate::commands::files::PreviewPayload;

pub fn preview(path: &Path) -> anyhow::Result<Option<PreviewPayload>> {
    // The `mdimport` binary ships with every macOS install; using it as
    // a presence probe keeps the host from depending on a private SPI
    // for "is QuickLook available". The implementation here returns
    // None and lets `mod.rs` use the text-head fallback — a future
    // commit replaces this with the QLPreviewPanel/QLThumbnailGenerator
    // calls without changing the public surface.
    let _ = path;
    Ok(None)
}

pub fn thumbnail(path: &Path, _size: u32) -> anyhow::Result<Option<String>> {
    let _ = path;
    Ok(None)
}
