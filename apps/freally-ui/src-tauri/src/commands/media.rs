//! SRC-M18 — inline media playback for the preview pane.
//!
//! Two commands: the waveform envelope plus the technical badges for an
//! audio file, and the bytes themselves for the `<audio>` / `<video>`
//! element to play.
//!
//! ## Why the bytes go through IPC instead of the asset protocol
//!
//! Tauri's asset protocol is the conventional way to feed a media
//! element, and it was rejected here. Enabling it means declaring a
//! filesystem scope the *webview* may read directly, and any scope wide
//! enough to cover "wherever the user's media happens to live" is wide
//! enough to undo the [`KnownPaths`] gate the rest of this surface is
//! built on — the property that a compromised frontend dependency
//! cannot ask for a file the daemon never returned. Routing the bytes
//! through a command keeps every read behind the same check as every
//! other file operation.
//!
//! The cost is that playback is buffered rather than streamed, so there
//! is a size ceiling. That is stated in the UI rather than hidden: past
//! the cap the pane offers to open the file in the system player.

use serde::Serialize;
use tauri::State;

use super::known_paths::{KnownPaths, Provenance};

/// Largest file served inline.
///
/// The whole file lands in the webview's memory, so this is a real
/// budget rather than a formality: a 4K video would be gigabytes. 192 MB
/// comfortably covers an hour of lossless audio and short video clips,
/// which is what a preview pane is for.
pub const MAX_INLINE_BYTES: u64 = 192 * 1024 * 1024;

/// Waveform resolution. The canvas is a few hundred CSS pixels wide;
/// 800 buckets survives a 2× device-pixel-ratio display with headroom
/// and still costs about 3 KB on the wire.
const WAVEFORM_BUCKETS: usize = 800;

#[derive(Debug, Serialize)]
pub struct Waveform {
    /// Peak amplitude per bucket, each `0.0..=1.0`.
    pub peaks: Vec<f32>,
    pub duration_ms: u64,
    pub codec: String,
    pub sample_rate: u32,
    pub channels: u16,
    /// Integrated loudness. `None` when the measurement came back
    /// non-finite — a clip shorter than one gating block has no
    /// meaningful integrated value, and rendering `-inf LUFS` as a badge
    /// would be worse than rendering nothing.
    pub lufs_integrated: Option<f32>,
}

#[tauri::command]
pub async fn media_waveform(
    path: String,
    known: State<'_, KnownPaths>,
) -> Result<Waveform, String> {
    // Daemon-attested, not the frontend-asserted read level: this hands
    // the *complete* contents of a named file to the webview, and
    // `files_whitelist_user_chosen` lets the webview mint
    // `FrontendAsserted` for any string it likes. `content_view` and
    // `copy_file_contents` — the other two commands that return file
    // contents — already demand this level, for this reason.
    known.verify(&path, Provenance::QueryHit)?;
    let p = std::path::PathBuf::from(&path);
    // Decoding is CPU-bound and can run for seconds on a long file;
    // holding the async runtime's worker for that would stall every
    // other command.
    let (attrs, peaks) = tokio::task::spawn_blocking(move || {
        freally_audio::analyze_with_peaks(&p, WAVEFORM_BUCKETS)
    })
    .await
    .map_err(|e| format!("waveform task failed: {e}"))?
    .map_err(|e| e.to_string())?;

    Ok(Waveform {
        peaks,
        duration_ms: attrs.duration_ns / 1_000_000,
        codec: attrs.codec.to_string(),
        sample_rate: attrs.sample_rate,
        channels: attrs.channels,
        lufs_integrated: attrs
            .lufs_integrated
            .is_finite()
            .then_some(attrs.lufs_integrated),
    })
}

/// The file's bytes, for a media element to play.
///
/// Returns a raw byte response rather than a base64 data URL: base64
/// inflates the payload by a third and costs a full re-encode on both
/// sides of the bridge for a file that can be a hundred megabytes.
#[tauri::command]
pub async fn media_bytes(
    path: String,
    known: State<'_, KnownPaths>,
) -> Result<tauri::ipc::Response, String> {
    // Daemon-attested, not the frontend-asserted read level: this hands
    // the *complete* contents of a named file to the webview, and
    // `files_whitelist_user_chosen` lets the webview mint
    // `FrontendAsserted` for any string it likes. `content_view` and
    // `copy_file_contents` — the other two commands that return file
    // contents — already demand this level, for this reason.
    known.verify(&path, Provenance::QueryHit)?;
    let p = std::path::PathBuf::from(&path);

    // Check the size before reading, not after: `read` on an oversized
    // file would allocate the whole thing first and only then let us
    // refuse it.
    let len = tokio::fs::metadata(&p)
        .await
        .map_err(|e| e.to_string())?
        .len();
    if len > MAX_INLINE_BYTES {
        return Err(format!(
            "file is {len} bytes; inline playback is capped at {MAX_INLINE_BYTES}"
        ));
    }

    let bytes = tokio::fs::read(&p).await.map_err(|e| e.to_string())?;
    Ok(tauri::ipc::Response::new(bytes))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn the_inline_cap_is_a_real_budget_not_a_formality() {
        // Guards against someone "simplifying" this to a token value:
        // the whole file lands in webview memory, so the number has to
        // stay in a range a renderer can actually hold.
        //
        // `const` blocks, so a bad value fails the build rather than
        // waiting for someone to run the tests.
        const { assert!(MAX_INLINE_BYTES >= 64 * 1024 * 1024) };
        const { assert!(MAX_INLINE_BYTES <= 512 * 1024 * 1024) };
    }

    #[test]
    fn waveform_resolution_fits_a_hidpi_canvas() {
        const { assert!(WAVEFORM_BUCKETS >= 400) };
        const { assert!(WAVEFORM_BUCKETS <= freally_audio::peaks::MAX_BUCKETS) };
    }
}
