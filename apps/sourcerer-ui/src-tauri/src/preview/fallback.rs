//! Fallback host for non-macOS / non-Windows / non-Linux targets
//! (BSDs, niche Unix). Always returns None so the caller drops to the
//! universal text-head + typed-icon path.

use std::path::Path;

use crate::commands::files::PreviewPayload;

pub fn preview(_path: &Path) -> anyhow::Result<Option<PreviewPayload>> {
    Ok(None)
}

pub fn thumbnail(_path: &Path, _size: u32) -> anyhow::Result<Option<String>> {
    Ok(None)
}
