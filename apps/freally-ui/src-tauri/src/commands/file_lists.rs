//! File-list interop commands (SRC-M03).
//!
//! Serialisation itself lives in `freally_rpc::filelist` so the CLI,
//! the daemon, and this app all write byte-identical files.
//!
//! **These commands open their own dialogs.** The write target and the
//! import source are values *Rust produced*, never values the webview
//! handed it. That distinction is the whole security boundary here:
//! `file_list_export` truncates and overwrites whatever it is pointed
//! at, so "the user picked this in an OS-native dialog" has to be a fact
//! the backend witnessed, not a claim the frontend made. A path that
//! merely round-tripped through the frontend would let a compromised
//! dependency nominate any file on disk.

use freally_rpc::QueryHit;
use freally_rpc::filelist::{self, FileListEntry, FileListFormat};
use tauri::{AppHandle, State};
use tauri_plugin_dialog::DialogExt;

use super::known_paths::KnownPaths;

/// Refuse to import a file larger than this. An `.efu` of a full 5M-file
/// volume is roughly 500 MB; anything past this cap is not a file list
/// someone meant to open, and parsing it would balloon the UI process.
const MAX_IMPORT_BYTES: u64 = 512 * 1024 * 1024;

/// Formats offered in the save dialog, in menu order. Kept here rather
/// than in the frontend so the dialog and the serialiser cannot drift.
const EXPORT_FILTERS: &[(&str, &[&str])] = &[
    ("Everything File List", &["efu"]),
    ("CSV", &["csv"]),
    ("Text", &["txt"]),
    ("Playlist", &["m3u8", "m3u"]),
    ("NDJSON", &["ndjson"]),
    ("JSON", &["json"]),
];

/// Formats the open dialog accepts — the ones carrying enough per-file
/// metadata to reconstruct rows.
const IMPORT_FILTERS: &[(&str, &[&str])] = &[
    ("Everything File List", &["efu"]),
    ("NDJSON", &["ndjson", "jsonl"]),
    ("JSON", &["json"]),
    ("Text", &["txt"]),
];

#[derive(Debug, Clone, serde::Serialize)]
pub struct ExportSummary {
    /// False when the user dismissed the save dialog. Every other field
    /// is meaningless in that case.
    pub saved: bool,
    /// Entries actually written. Lower than the hit count for the
    /// playlist formats, which carry audio only.
    pub written: u32,
    /// Format chosen from the path's extension.
    pub format: FileListFormat,
    /// True when the format dropped rows the result set contained.
    pub lossy: bool,
}

#[derive(Debug, Clone, serde::Serialize)]
pub struct ImportSummary {
    /// False when the user dismissed the open dialog.
    pub opened: bool,
    /// File name of the opened list, for the status bar.
    pub name: String,
    pub entries: Vec<FileListEntry>,
}

/// Ask the user where to save, then write the supplied hits there in the
/// format their chosen extension names (`.efu`, `.csv`, `.txt`, `.m3u`,
/// `.m3u8`, `.ndjson`, `.json` as the fallback).
#[tauri::command]
pub async fn file_list_export(
    hits: Vec<QueryHit>,
    app: AppHandle,
    known: State<'_, KnownPaths>,
) -> Result<ExportSummary, String> {
    let mut dialog = app.dialog().file().set_file_name("freally-results.efu");
    for (name, extensions) in EXPORT_FILTERS {
        dialog = dialog.add_filter(*name, extensions);
    }
    // Safe to block: `#[tauri::command] async fn` runs off the main
    // thread, which is the plugin's requirement for the blocking API.
    let Some(chosen) = dialog.blocking_save_file() else {
        return Ok(ExportSummary {
            saved: false,
            written: 0,
            format: FileListFormat::Json,
            lossy: false,
        });
    };
    let target = chosen
        .into_path()
        .map_err(|e| format!("dialog returned a path we cannot use: {e}"))?;
    let path_str = target.to_string_lossy().into_owned();

    // Record the dialog result so the rest of the app can act on this
    // path too. This is the one place `UserChosen` is granted, and it is
    // granted to a path the backend just watched the user pick.
    known.whitelist_user_chosen(&path_str);

    let format = FileListFormat::from_path(&path_str);
    let text = filelist::export(&hits, format).map_err(|e| e.to_string())?;
    let written = written_count(&text, format, hits.len());
    std::fs::write(&target, text.as_bytes()).map_err(|e| e.to_string())?;
    Ok(ExportSummary {
        saved: true,
        written,
        format,
        lossy: written as usize != hits.len(),
    })
}

/// Ask the user for a file list, then read it back. Only `.efu`,
/// `.ndjson`, `.json`, and `.txt` carry enough per-file data to import;
/// the others surface a typed error the UI shows verbatim.
#[tauri::command]
pub async fn file_list_import(
    app: AppHandle,
    known: State<'_, KnownPaths>,
) -> Result<ImportSummary, String> {
    let mut dialog = app.dialog().file();
    for (name, extensions) in IMPORT_FILTERS {
        dialog = dialog.add_filter(*name, extensions);
    }
    let Some(chosen) = dialog.blocking_pick_file() else {
        return Ok(ImportSummary {
            opened: false,
            name: String::new(),
            entries: Vec::new(),
        });
    };
    let source = chosen
        .into_path()
        .map_err(|e| format!("dialog returned a path we cannot use: {e}"))?;
    let path_str = source.to_string_lossy().into_owned();
    known.whitelist_user_chosen(&path_str);

    let meta = std::fs::metadata(&source).map_err(|e| e.to_string())?;
    if meta.len() > MAX_IMPORT_BYTES {
        return Err(format!(
            "file list is {} MB — larger than the {} MB import limit",
            meta.len() / (1024 * 1024),
            MAX_IMPORT_BYTES / (1024 * 1024)
        ));
    }
    let format = FileListFormat::from_path(&path_str);
    let text = std::fs::read_to_string(&source).map_err(|e| e.to_string())?;
    let entries = filelist::import(&text, format).map_err(|e| e.to_string())?;
    Ok(ImportSummary {
        opened: true,
        name: source
            .file_name()
            .map(|s| s.to_string_lossy().into_owned())
            .unwrap_or(path_str),
        entries,
    })
}

/// How many entries the serialised export actually carries. Only the
/// playlist formats drop rows, and they drop the non-audio ones —
/// counted off the text we already produced rather than by
/// re-serialising or by duplicating the extension list here.
fn written_count(text: &str, format: FileListFormat, hit_count: usize) -> u32 {
    match format {
        // Count the directive *lines*, not every occurrence: a track
        // literally named `mix#EXTINF:live.mp3` appears in its own
        // `#EXTINF` tail and again on its path line, and would report
        // two entries for one track.
        FileListFormat::M3u | FileListFormat::M3u8 => {
            text.lines().filter(|l| l.starts_with("#EXTINF:")).count() as u32
        }
        _ => hit_count as u32,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use freally_rpc::LensId;

    fn hit(name: &str, ext: &str) -> QueryHit {
        QueryHit {
            file_id: name.into(),
            lens: LensId::Filename,
            name: name.into(),
            path: format!("/vault/{name}"),
            ext: ext.into(),
            size: 4,
            modified_ms: 1_700_000_000_000,
            kind: ext.to_uppercase(),
            score: 1.0,
            attrs: 0,
            volume: String::new(),
            volume_label: None,
            volume_offline: false,
        }
    }

    fn count_for(format: FileListFormat) -> u32 {
        let hits = vec![hit("a.txt", "txt"), hit("b.flac", "flac")];
        let text = filelist::export(&hits, format).unwrap();
        written_count(&text, format, hits.len())
    }

    #[test]
    fn written_count_matches_the_full_set_for_lossless_formats() {
        for f in [
            FileListFormat::Efu,
            FileListFormat::Csv,
            FileListFormat::Txt,
            FileListFormat::Ndjson,
            FileListFormat::Json,
        ] {
            assert_eq!(count_for(f), 2, "{f:?}");
        }
    }

    #[test]
    fn written_count_drops_non_audio_for_playlists() {
        assert_eq!(count_for(FileListFormat::M3u), 1);
        assert_eq!(count_for(FileListFormat::M3u8), 1);
    }
}
