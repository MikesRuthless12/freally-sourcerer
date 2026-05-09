//! OS-native preview hosts.
//!
//! Each platform module owns its preview / thumbnail strategy:
//!
//! - **macOS** (`macos.rs`): QuickLook via `objc2` —
//!   `QLPreviewPanel` for full preview, `QLThumbnailGenerator` for
//!   thumbnails.
//! - **Windows** (`windows_host.rs`): Shell preview handlers via
//!   `IPreviewHandler`, thumbnails via `IThumbnailProvider` (windows-rs).
//! - **Linux** (`linux.rs`): GNOME Sushi via DBus when present, KDE KIO
//!   via `kioclient5` shell-out when present, in-app text-head and
//!   typed-icon SVG fallbacks otherwise.
//!
//! Each platform implements two functions:
//!
//! - `preview(path) -> Result<Option<PreviewPayload>>` — `Some(...)` if
//!   the platform handler returned a preview, `None` if it declined and
//!   we should fall back. Errors propagate.
//! - `thumbnail(path, size) -> Result<Option<String>>` — same shape, the
//!   `String` is a `data:` URL.

use std::path::Path;

use crate::commands::files::PreviewPayload;

#[cfg(target_os = "macos")]
mod macos;
#[cfg(target_os = "macos")]
use macos as host;

#[cfg(windows)]
mod windows_host;
#[cfg(windows)]
use windows_host as host;

#[cfg(target_os = "linux")]
mod linux;
#[cfg(target_os = "linux")]
use linux as host;

#[cfg(all(not(target_os = "macos"), not(windows), not(target_os = "linux")))]
mod fallback;
#[cfg(all(not(target_os = "macos"), not(windows), not(target_os = "linux")))]
use fallback as host;

/// Try the platform host first; if it declines, fall back to the
/// universal text-head probe.
pub fn preview(path: &str) -> anyhow::Result<Option<PreviewPayload>> {
    if let Some(p) = host::preview(Path::new(path))? {
        return Ok(Some(p));
    }
    Ok(text_head_fallback(path))
}

/// Thumbnail. Platform host returns `Some(data_url)` when it handled
/// the file; on `None`, the caller substitutes a typed-icon SVG.
pub fn thumbnail(path: &str, size: u32) -> anyhow::Result<Option<String>> {
    host::thumbnail(Path::new(path), size)
}

/// Universal fallback — read up to 4 KiB and lossy-decode as UTF-8.
/// Classify as `Text` iff the head has no NUL bytes and the decode
/// produced few replacement characters.
pub fn text_head_fallback(path: &str) -> Option<PreviewPayload> {
    use crate::commands::files::PreviewKind;
    let bytes = std::fs::read(path).ok()?;
    let head_len = bytes.len().min(4096);
    let head = &bytes[..head_len];
    if head.contains(&0u8) {
        return Some(PreviewPayload {
            kind: PreviewKind::Unsupported,
            text: None,
            data_url: None,
            message: Some("Binary preview not yet wired".into()),
        });
    }
    let lossy = String::from_utf8_lossy(head);
    let replacements = lossy.chars().filter(|&c| c == '\u{FFFD}').count();
    if replacements * 100 > head_len {
        return Some(PreviewPayload {
            kind: PreviewKind::Unsupported,
            text: None,
            data_url: None,
            message: Some("Binary preview not yet wired".into()),
        });
    }
    Some(PreviewPayload {
        kind: PreviewKind::Text,
        text: Some(lossy.into_owned()),
        data_url: None,
        message: None,
    })
}
