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

use super::known_paths::{KnownPaths, Provenance};
use crate::preview;

/// Every file-ops command goes through this gate. Rejects paths the
/// daemon never returned — defense-in-depth against a compromised JS
/// layer (supply-chain on a frontend dep) asking the backend to act on
/// arbitrary filesystem paths.
///
/// These commands all *read*, so the weakest grant is enough — the
/// preview pane and Go To both name a path the user just picked in a
/// JS-opened dialog. Destructive commands ask for [`Provenance::QueryHit`]
/// (daemon-attested) and content-writing ones for
/// [`Provenance::UserChosen`].
pub(super) fn verify_readable(path: &str, known: &KnownPaths) -> Result<(), String> {
    known.verify(path, Provenance::FrontendAsserted).map(|_| ())
}

/// Largest payload any command will put on the OS clipboard. Past this,
/// the clipboard stops being the right tool and the OS starts thrashing.
pub const MAX_CLIPBOARD_BYTES: usize = 4 * 1024 * 1024;

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
    verify_readable(&path, &known)?;
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
    verify_readable(&path, &known)?;
    let parent = std::path::Path::new(&path)
        .parent()
        .map(|p| p.to_string_lossy().to_string())
        .unwrap_or_else(|| path.clone());
    app.opener()
        .open_path(&parent, None::<&str>)
        .map_err(|e| e.to_string())
}

/// SRC-M19 — hand the file to macOS's own Quick Look.
///
/// The in-app modal is the cross-platform surface and is what the
/// arrow-key flow drives; this is the escape hatch for a format macOS
/// can render and Freally's preview host cannot (Keynote, Sketch, a
/// signed PDF with an embedded form). It opens a separate OS window, so
/// it is a deliberate action rather than the default: `qlmanage` cannot
/// be driven from Freally's key handling, and silently replacing the
/// modal with it would break the flow the feature exists for.
///
/// The path is gated by the same registry as every other file command,
/// and passed as an argument to `Command` — never through a shell — so
/// a name containing spaces or shell metacharacters is inert.
#[tauri::command]
pub fn quick_look_native(path: String, known: State<'_, KnownPaths>) -> Result<(), String> {
    verify_readable(&path, &known)?;
    #[cfg(target_os = "macos")]
    {
        std::process::Command::new("qlmanage")
            .arg("-p")
            .arg(&path)
            .stdout(std::process::Stdio::null())
            .stderr(std::process::Stdio::null())
            .spawn()
            .map(|_| ())
            .map_err(|e| format!("could not start Quick Look: {e}"))
    }
    #[cfg(not(target_os = "macos"))]
    {
        Err("native Quick Look is a macOS feature".to_string())
    }
}

#[tauri::command]
pub fn files_copy_path(
    paths: Vec<String>,
    app: AppHandle,
    known: State<'_, KnownPaths>,
) -> Result<(), String> {
    for p in &paths {
        verify_readable(p, &known)?;
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
        verify_readable(p, &known)?;
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
    // Destructive, so it demands a **daemon-attested** hit rather than the
    // read gate above. `files_whitelist_user_chosen` is reachable from the
    // webview with an arbitrary string, so accepting the weaker level here
    // would let a compromised frontend dependency trash any path it names,
    // with no user interaction.
    known.verify_all(&paths, Provenance::QueryHit)?;
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
    verify_readable(&path, &known)?;
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
    tracing::info!(target: "freally::preview", path = %path, "files_preview ENTER");
    if let Err(e) = verify_readable(&path, &known) {
        tracing::warn!(target: "freally::preview", path = %path, error = %e,
            "files_preview verify_readable REJECTED");
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
            tracing::error!(target: "freally::preview", error = %e,
                "files_preview spawn_blocking JOIN FAILED");
            e.to_string()
        })?;
    tracing::info!(target: "freally::preview", path = %path,
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
    if text.len() > MAX_CLIPBOARD_BYTES {
        return Err("text too large for clipboard".into());
    }
    app.clipboard().write_text(text).map_err(|e| e.to_string())
}

/// Records a path the user pointed at *inside the app* — a selected
/// result row, a folder picked for `view.go_to` — so the read-side
/// file-ops commands accept it.
///
/// Grants only [`Provenance::QueryHit`], deliberately. This command is
/// reachable from the webview with an arbitrary string, so whatever it
/// grants is in practice caller-chosen; it must therefore never be able
/// to hand out the write-capable `UserChosen` level. That level is
/// granted in exactly one place — `commands::file_lists`, after a
/// dialog the *backend* ran and whose result it read directly.
#[tauri::command]
pub fn files_whitelist_user_chosen(path: String, known: State<'_, KnownPaths>) {
    known.add_frontend_asserted(&path);
}
