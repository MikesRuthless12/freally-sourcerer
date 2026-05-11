//! File operations. open / reveal / copy_path / copy_name use real OS
//! handlers via Tauri plugins; thumbnail + preview now delegate to the
//! `preview` module which dispatches to OS-native preview hosts on each
//! platform (QuickLook on macOS, Shell preview on Windows, GNOME Sushi
//! / KDE KIO on Linux) and falls back to a text-head + typed-icon
//! response for unsupported types.

use serde::Serialize;
use tauri::{AppHandle, State};
use tauri_plugin_clipboard_manager::ClipboardExt;
use tauri_plugin_opener::OpenerExt;

use super::known_paths::KnownPaths;
use crate::preview;

/// Every file-ops command goes through this gate. Rejects paths the
/// daemon never returned — defense-in-depth against a compromised JS
/// layer (supply-chain on a frontend dep) asking the backend to act on
/// arbitrary filesystem paths. The error string deliberately does NOT
/// echo the supplied path (L1 fix) — caller-controlled bytes don't
/// belong in error responses that may flow into logs or UIs.
fn verify_path(path: &str, known: &KnownPaths) -> Result<(), String> {
    if !known.contains(path) {
        return Err("path not in current result set".into());
    }
    Ok(())
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum PreviewKind {
    Text,
    Image,
    Unsupported,
}

#[derive(Debug, Clone, Serialize)]
pub struct PreviewPayload {
    pub kind: PreviewKind,
    pub text: Option<String>,
    pub data_url: Option<String>,
    pub message: Option<String>,
}

#[tauri::command]
pub async fn files_open(
    path: String,
    app: AppHandle,
    known: State<'_, KnownPaths>,
) -> Result<(), String> {
    verify_path(&path, &known)?;
    app.opener()
        .open_path(&path, None::<&str>)
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn files_reveal(
    path: String,
    app: AppHandle,
    known: State<'_, KnownPaths>,
) -> Result<(), String> {
    verify_path(&path, &known)?;
    let parent = std::path::Path::new(&path)
        .parent()
        .map(|p| p.to_string_lossy().to_string())
        .unwrap_or_else(|| path.clone());
    app.opener()
        .open_path(&parent, None::<&str>)
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub fn files_copy_path(
    paths: Vec<String>,
    app: AppHandle,
    known: State<'_, KnownPaths>,
) -> Result<(), String> {
    for p in &paths {
        verify_path(p, &known)?;
    }
    app.clipboard()
        .write_text(paths.join("\n"))
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub fn files_copy_name(
    paths: Vec<String>,
    app: AppHandle,
    known: State<'_, KnownPaths>,
) -> Result<(), String> {
    for p in &paths {
        verify_path(p, &known)?;
    }
    let names: Vec<String> = paths
        .iter()
        .map(|p| {
            std::path::Path::new(p)
                .file_name()
                .map(|n| n.to_string_lossy().to_string())
                .unwrap_or_else(|| p.clone())
        })
        .collect();
    app.clipboard()
        .write_text(names.join("\n"))
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub fn files_delete(paths: Vec<String>, known: State<'_, KnownPaths>) -> Result<(), String> {
    for p in &paths {
        verify_path(p, &known)?;
    }
    // Sends to OS trash / recycle bin (recoverable). UI confirms before
    // calling. The `trash` crate routes to the right native API on each
    // OS — Recycle Bin on Windows, Trash on macOS, ~/.local/share/Trash
    // (XDG) on Linux.
    for p in &paths {
        trash::delete(p).map_err(|e| format!("failed to trash {p}: {e}"))?;
    }
    Ok(())
}

#[tauri::command]
pub async fn files_thumbnail(
    path: String,
    size: u32,
    known: State<'_, KnownPaths>,
) -> Result<String, String> {
    verify_path(&path, &known)?;
    // Clamp size so a hostile caller can't request a 4-billion-pixel SVG.
    let size = size.clamp(16, 512);
    let path_for_task = path.clone();
    let host_result: Option<String> =
        tokio::task::spawn_blocking(move || preview::thumbnail(&path_for_task, size))
            .await
            .map_err(|e| e.to_string())?
            .map_err(|e: anyhow::Error| e.to_string())?;
    Ok(host_result.unwrap_or_else(|| typed_icon_svg(&path, size)))
}

#[tauri::command]
pub async fn files_preview(
    path: String,
    known: State<'_, KnownPaths>,
) -> Result<PreviewPayload, String> {
    let t0 = std::time::Instant::now();
    tracing::info!(target: "sourcerer::preview", path = %path, "files_preview ENTER");
    if let Err(e) = verify_path(&path, &known) {
        tracing::warn!(target: "sourcerer::preview", path = %path, error = %e,
            "files_preview verify_path REJECTED");
        return Err(e);
    }
    // The actual preview work (file read + base64 encode for images,
    // text decode for the fallback) is CPU/IO heavy. Run it on a
    // blocking thread so Tauri's IPC dispatcher stays responsive —
    // otherwise multi-MB images freeze the entire UI while encoding.
    let path_for_task = path.clone();
    let res = tokio::task::spawn_blocking(move || preview::preview(&path_for_task))
        .await
        .map_err(|e| {
            tracing::error!(target: "sourcerer::preview", error = %e,
                "files_preview spawn_blocking JOIN FAILED");
            e.to_string()
        })?;
    tracing::info!(target: "sourcerer::preview", path = %path,
        ms = t0.elapsed().as_millis() as u64, "files_preview EXIT");
    match res {
        Ok(Some(p)) => Ok(p),
        Ok(None) => Ok(PreviewPayload {
            kind: PreviewKind::Unsupported,
            text: None,
            data_url: None,
            message: Some("Preview not available for this file type".into()),
        }),
        Err(e) => Ok(PreviewPayload {
            kind: PreviewKind::Unsupported,
            text: None,
            data_url: None,
            message: Some(e.to_string()),
        }),
    }
}

/// Typed-icon SVG fallback used when the OS-native thumbnail provider
/// declines to handle the file (e.g. on Linux where Sushi / KIO is
/// unavailable, or on Windows for an extension without a registered
/// `IThumbnailProvider`). Color matches the extension family.
fn typed_icon_svg(path: &str, size: u32) -> String {
    let ext = std::path::Path::new(path)
        .extension()
        .map(|e| e.to_string_lossy().to_ascii_lowercase())
        .unwrap_or_default();
    let color = match ext.as_str() {
        "flac" | "mp3" | "wav" | "aiff" | "aac" | "ogg" => "#8E6BD9",
        "pdf" | "docx" | "xlsx" | "pptx" | "txt" | "md" => "#39C5CF",
        "zip" | "7z" | "tar" => "#FF6A00",
        _ => "#C77DFF",
    };
    let svg = format!(
        "<svg xmlns='http://www.w3.org/2000/svg' width='{size}' height='{size}'><rect width='{size}' height='{size}' rx='6' fill='{color}'/></svg>"
    );
    format!(
        "data:image/svg+xml;base64,{}",
        base64_encode(svg.as_bytes())
    )
}

pub(crate) fn base64_encode(bytes: &[u8]) -> String {
    // Tiny inline base64 encoder. Avoids pulling in `base64` for one call.
    const ALPHABET: &[u8; 64] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/";
    let mut out = String::with_capacity(bytes.len().div_ceil(3) * 4);
    let mut i = 0;
    while i + 3 <= bytes.len() {
        let n = ((bytes[i] as u32) << 16) | ((bytes[i + 1] as u32) << 8) | (bytes[i + 2] as u32);
        out.push(ALPHABET[((n >> 18) & 63) as usize] as char);
        out.push(ALPHABET[((n >> 12) & 63) as usize] as char);
        out.push(ALPHABET[((n >> 6) & 63) as usize] as char);
        out.push(ALPHABET[(n & 63) as usize] as char);
        i += 3;
    }
    let rem = bytes.len() - i;
    if rem == 1 {
        let n = (bytes[i] as u32) << 16;
        out.push(ALPHABET[((n >> 18) & 63) as usize] as char);
        out.push(ALPHABET[((n >> 12) & 63) as usize] as char);
        out.push('=');
        out.push('=');
    } else if rem == 2 {
        let n = ((bytes[i] as u32) << 16) | ((bytes[i + 1] as u32) << 8);
        out.push(ALPHABET[((n >> 18) & 63) as usize] as char);
        out.push(ALPHABET[((n >> 12) & 63) as usize] as char);
        out.push(ALPHABET[((n >> 6) & 63) as usize] as char);
        out.push('=');
    }
    out
}

#[tauri::command]
pub fn files_copy_text(text: String, app: AppHandle) -> Result<(), String> {
    // Typed counterpart to copy_path: writes arbitrary text (not paths) to
    // the OS clipboard. Used by edit.advanced.copy_as_json /
    // copy_with_metadata so we don't abuse the path-shaped `copy_path`.
    // Cap to bound clipboard pressure.
    if text.len() > 4 * 1024 * 1024 {
        return Err("text too large for clipboard".into());
    }
    app.clipboard().write_text(text).map_err(|e| e.to_string())
}

/// Records a user-chosen path (e.g. from `view.go_to`'s native folder
/// dialog or `file.export_results`'s save dialog) as known so subsequent
/// file-ops commands accept it. The OS-native dialog is the trust
/// boundary for these paths.
#[tauri::command]
pub fn files_whitelist_user_chosen(path: String, known: State<'_, KnownPaths>) {
    known.whitelist_user_chosen(&path);
}
