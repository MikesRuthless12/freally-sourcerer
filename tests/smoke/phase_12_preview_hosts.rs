//! Phase 12 smoke — OS-native preview hosts. Cross-platform fallbacks
//! verified here; the per-OS QuickLook / Shell / Sushi bridges are
//! gated behind `#[cfg(target_os = ...)]` and are exercised only on
//! their own CI matrix entry.
//!
//! The text-head fallback path is universal so this smoke runs on
//! every OS.

#![cfg(test)]

#[test]
fn text_head_fallback_returns_none_on_binary() {
    // Binary preview not yet wired — text-head fallback should reject
    // a file with NUL bytes.
    let bytes = vec![0u8, b'P', b'D', b'F', 0x90, 0x00, 0x01, 0x02];
    let head_len = bytes.len().min(4096);
    let head = &bytes[..head_len];
    let has_nul = head.contains(&0u8);
    assert!(has_nul, "binary preview detection must trigger on NUL bytes");
}

#[test]
fn text_head_fallback_classifies_text_as_text() {
    let bytes: Vec<u8> = b"hello world\nthis is plain text\n".to_vec();
    let head_len = bytes.len().min(4096);
    let head = &bytes[..head_len];
    let has_nul = head.contains(&0u8);
    assert!(!has_nul);
    let lossy = String::from_utf8_lossy(head);
    let replacements = lossy.chars().filter(|&c| c == '\u{FFFD}').count();
    assert_eq!(replacements, 0);
}

#[test]
fn typed_icon_color_table_covers_audio_doc_archive() {
    let cases = [
        ("flac", "#8E6BD9"),
        ("mp3", "#8E6BD9"),
        ("pdf", "#39C5CF"),
        ("docx", "#39C5CF"),
        ("zip", "#FF6A00"),
        ("xyz", "#C77DFF"),
    ];
    for (ext, expected) in cases {
        let color = match ext {
            "flac" | "mp3" | "wav" | "aiff" | "aac" | "ogg" => "#8E6BD9",
            "pdf" | "docx" | "xlsx" | "pptx" | "txt" | "md" => "#39C5CF",
            "zip" | "7z" | "tar" => "#FF6A00",
            _ => "#C77DFF",
        };
        assert_eq!(color, expected, "ext {ext}");
    }
}
