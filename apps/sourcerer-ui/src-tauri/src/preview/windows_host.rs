//! Windows Shell preview / thumbnail host.
//!
//! Full IPreviewHandler / IThumbnailProvider COM bridges are deferred —
//! the simpler path that covers the most common case (images) is to
//! read the file directly and emit a `data:image/...` URL the WebView
//! renders natively. Non-image formats fall back to the universal
//! text-head probe in `mod.rs`.

use std::path::Path;

use crate::commands::files::{PreviewKind, PreviewPayload};

// 4 MiB cap: keeps the base64 IPC payload under ~6 MiB so the WebView
// doesn't choke on a giant string. Real photos are typically 1-3 MiB;
// anything larger probably wants the (future) shell-thumbnail path.
const MAX_PREVIEW_BYTES: u64 = 4 * 1024 * 1024;

pub fn preview(path: &Path) -> anyhow::Result<Option<PreviewPayload>> {
    let ext = path
        .extension()
        .and_then(|e| e.to_str())
        .map(|s| s.to_ascii_lowercase())
        .unwrap_or_default();
    let mime = match ext.as_str() {
        "png" => "image/png",
        "jpg" | "jpeg" | "jfif" => "image/jpeg",
        "gif" => "image/gif",
        "webp" => "image/webp",
        "bmp" => "image/bmp",
        "svg" => "image/svg+xml",
        "ico" => "image/x-icon",
        "avif" => "image/avif",
        _ => return Ok(None),
    };
    let metadata = match std::fs::metadata(path) {
        Ok(m) => m,
        Err(_) => return Ok(None),
    };
    if metadata.len() > MAX_PREVIEW_BYTES {
        return Ok(Some(PreviewPayload {
            kind: PreviewKind::Unsupported,
            text: None,
            data_url: None,
            message: Some(format!(
                "Image too large to inline-preview ({} bytes; limit {MAX_PREVIEW_BYTES})",
                metadata.len()
            )),
        }));
    }
    let bytes = match std::fs::read(path) {
        Ok(b) => b,
        Err(e) => {
            return Ok(Some(PreviewPayload {
                kind: PreviewKind::Unsupported,
                text: None,
                data_url: None,
                message: Some(format!("Read failed: {e}")),
            }));
        }
    };
    let b64 = crate::commands::files::base64_encode(&bytes);
    Ok(Some(PreviewPayload {
        kind: PreviewKind::Image,
        text: None,
        data_url: Some(format!("data:{mime};base64,{b64}")),
        message: None,
    }))
}

pub fn thumbnail(path: &Path, _size: u32) -> anyhow::Result<Option<String>> {
    let _ = path;
    Ok(None)
}
