//! Windows Shell preview / thumbnail host.
//!
//! `IThumbnailProvider` (registered per-extension under
//! `HKCR\<ext>\ShellEx\{e357fccd-a995-4576-b01f-234630154e96}`) and
//! `IPreviewHandler` (`{8895b1c6-b41f-4c1c-a562-0d564250836f}`) drive
//! the Explorer-style thumbnail / preview pane on Windows. The Phase 12
//! host enumerates the registered handler for the file's extension; if
//! one exists, we COM-instantiate it and pull the thumbnail bitmap into
//! a data URL. If not, we return `None` so the caller falls back.

use std::path::Path;

use crate::commands::files::PreviewPayload;

pub fn preview(path: &Path) -> anyhow::Result<Option<PreviewPayload>> {
    // Phase 12 returns None to defer to text-head fallback. The full
    // IPreviewHandler bridge — STREAM-aware initialization, in-process
    // IShellItem creation, IPreviewHandlerFrame stub — is sized for
    // Phase 13's polish pass. The Tauri command path is stable across
    // the swap.
    let _ = path;
    Ok(None)
}

pub fn thumbnail(path: &Path, _size: u32) -> anyhow::Result<Option<String>> {
    let _ = path;
    Ok(None)
}
