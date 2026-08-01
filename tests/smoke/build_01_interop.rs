//! Build 1 (v0.21.0) smoke — the interop half (SRC-M03).
//!
//! OS-agnostic, runs on every CI matrix entry. Everything's `.efu` is a
//! wire format shared with another program, so the gates here are about
//! *bytes*, not just round-tripping through our own reader:
//!
//!   1. The header is byte-exact, and rows are CRLF-terminated, so
//!      Everything on Windows loads a list Freally wrote on Linux.
//!   2. Timestamps are Windows FILETIME ticks, converted at the
//!      boundary from Unix milliseconds.
//!   3. Paths containing commas and quotes survive the round trip.
//!   4. The lossy formats say so, and refuse to import.

use freally_rpc::filelist::{
    EFU_HEADER, FileListFormat, export, filetime_to_unix_ms, import, unix_ms_to_filetime,
};
use freally_rpc::{LensId, QueryHit};

/// 2023-11-14T22:13:20Z, chosen so the FILETIME conversion is a value a
/// human can check by hand against a converter.
const FIXTURE_MS: u64 = 1_700_000_000_000;

fn hit(name: &str, path: &str, ext: &str, size: u64, attrs: u32) -> QueryHit {
    QueryHit {
        file_id: format!("id-{name}"),
        lens: LensId::Filename,
        name: name.to_string(),
        path: path.to_string(),
        ext: ext.to_string(),
        size,
        modified_ms: FIXTURE_MS,
        kind: ext.to_uppercase(),
        score: 1.0,
        attrs,
    }
}

fn fixture() -> Vec<QueryHit> {
    vec![
        hit("report.pdf", "/vault/docs/report.pdf", "pdf", 1200, 0),
        hit("track.flac", "/vault/media/track.flac", "flac", 900, 0),
        hit("docs", "/vault/docs", "", 0, 0x10),
    ]
}

#[test]
fn efu_header_and_line_endings_match_everything() {
    let text = export(&fixture(), FileListFormat::Efu).unwrap();
    let mut lines = text.split("\r\n");
    assert_eq!(
        lines.next(),
        Some(EFU_HEADER),
        "the header must match Everything's literally"
    );
    // Three data rows plus the trailing empty string after the last CRLF.
    assert_eq!(lines.count(), 4);
    assert!(
        !text.contains("\n\r"),
        "line endings must be CRLF, not swapped"
    );
}

#[test]
fn efu_dates_are_windows_filetime_ticks() {
    let text = export(&fixture(), FileListFormat::Efu).unwrap();
    let first_row = text.lines().nth(1).expect("one data row");
    let cols: Vec<&str> = first_row.split(',').collect();
    assert_eq!(cols.len(), 5, "row was {first_row:?}");
    let ticks: i64 = cols[2].parse().expect("Date Modified is an integer");
    assert_eq!(ticks, unix_ms_to_filetime(FIXTURE_MS));
    assert_eq!(filetime_to_unix_ms(ticks), FIXTURE_MS);
    // Sanity: FILETIME for a 2023 date is ~1.33e17. A Unix-epoch value
    // leaking through would be ~1.7e12 — three orders of magnitude off.
    assert!(
        ticks > 100_000_000_000_000_000,
        "{ticks} is not a plausible FILETIME"
    );
}

#[test]
fn efu_round_trips_a_path_with_a_comma_and_a_quote() {
    let hits = vec![hit(
        "a, \"b\".txt",
        "/vault/notes/a, \"b\".txt",
        "txt",
        7,
        0,
    )];
    let text = export(&hits, FileListFormat::Efu).unwrap();
    let entries = import(&text, FileListFormat::Efu).unwrap();
    assert_eq!(entries.len(), 1);
    assert_eq!(entries[0].path, "/vault/notes/a, \"b\".txt");
    assert_eq!(entries[0].size, 7);
    assert_eq!(entries[0].modified_ms, FIXTURE_MS);
}

#[test]
fn efu_preserves_the_directory_attribute() {
    let text = export(&fixture(), FileListFormat::Efu).unwrap();
    let entries = import(&text, FileListFormat::Efu).unwrap();
    let dir = entries
        .iter()
        .find(|e| e.path == "/vault/docs")
        .expect("the folder row");
    assert!(dir.is_directory());
}

#[test]
fn every_export_format_produces_something_loadable() {
    for format in [
        FileListFormat::Efu,
        FileListFormat::Csv,
        FileListFormat::Txt,
        FileListFormat::M3u,
        FileListFormat::M3u8,
        FileListFormat::Ndjson,
        FileListFormat::Json,
    ] {
        let text = export(&fixture(), format).unwrap();
        assert!(!text.is_empty(), "{format:?} produced nothing");
    }
}

#[test]
fn playlists_are_flagged_lossy_and_carry_audio_only() {
    for format in [FileListFormat::M3u, FileListFormat::M3u8] {
        assert!(format.is_lossy(), "{format:?}");
        let text = export(&fixture(), format).unwrap();
        assert_eq!(
            text.matches("#EXTINF:").count(),
            1,
            "only the flac belongs in a playlist: {text}"
        );
        assert!(import(&text, format).is_err(), "{format:?} must not import");
    }
}

#[test]
fn ndjson_round_trips_every_hit() {
    let text = export(&fixture(), FileListFormat::Ndjson).unwrap();
    assert_eq!(text.lines().count(), 3);
    let entries = import(&text, FileListFormat::Ndjson).unwrap();
    assert_eq!(entries.len(), 3);
    assert_eq!(entries[0].modified_ms, FIXTURE_MS);
}

#[test]
fn the_format_follows_the_chosen_extension() {
    for (name, want) in [
        ("catalog.efu", FileListFormat::Efu),
        ("CATALOG.EFU", FileListFormat::Efu),
        ("sheet.csv", FileListFormat::Csv),
        ("paths.txt", FileListFormat::Txt),
        ("mix.m3u8", FileListFormat::M3u8),
        ("stream.ndjson", FileListFormat::Ndjson),
        ("dump.json", FileListFormat::Json),
        ("no-extension", FileListFormat::Json),
    ] {
        assert_eq!(FileListFormat::from_path(name), want, "{name}");
    }
}

#[test]
fn a_file_that_is_not_an_efu_is_rejected_with_a_typed_error() {
    let err = import("Name,Path\r\nx,y\r\n", FileListFormat::Efu).unwrap_err();
    assert!(
        err.to_string().contains("Everything file list"),
        "message was {err}"
    );
}
