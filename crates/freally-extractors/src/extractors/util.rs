//! Shared helpers for Phase 8 extractors.
//!
//! Two concerns are repeated across PDF / Office / archive / code /
//! structured / plain-text and live here so each extractor can pull
//! the same sanitised behaviour:
//!
//!   * [`sanitize_inline`] — escape line-breaking control characters
//!     (`\n` / `\r` / `\0`) so a hostile input cannot poison the
//!     line-per-record contract the extractors emit. Borrowing fast
//!     path keeps clean inputs zero-allocation.
//!   * [`trim_to_utf8_boundary`] — pop at most 4 bytes off the tail
//!     of a `Vec<u8>` so the buffer ends on a complete UTF-8
//!     codepoint. The naive `while !std::str::from_utf8(&buf).is_ok()
//!     { buf.pop(); }` scan re-validates the entire buffer per pop and
//!     is O(N²); the continuation-byte backtrack here is O(1).
//!
//! Phase 8 review pass: fixes for archive-name / CSV-value
//! search-index poisoning and the UTF-16 truncation bug in
//! [`super::plain_text::PlainTextExtractor`].

use std::borrow::Cow;

/// Replace `\n` / `\r` / `\0` in `s` with their literal escape forms
/// (`\\n`, `\\r`, `\\0`) so a hostile input cannot break the
/// "one record per line" contract every Phase 8 extractor emits into
/// the sink. Other Unicode (including U+0085 NEXT LINE and the
/// rare line-separator codepoints U+2028 / U+2029) passes through —
/// the sink is consumed downstream by the search index, which
/// tokenises on ASCII whitespace, not on every Unicode line break.
///
/// Borrowed when `s` is already clean — zero-allocation fast path.
pub(super) fn sanitize_inline(s: &str) -> Cow<'_, str> {
    if !s.bytes().any(|b| b == b'\n' || b == b'\r' || b == 0) {
        return Cow::Borrowed(s);
    }
    let mut out = String::with_capacity(s.len() + 4);
    for ch in s.chars() {
        match ch {
            '\n' => out.push_str("\\n"),
            '\r' => out.push_str("\\r"),
            '\0' => out.push_str("\\0"),
            other => out.push(other),
        }
    }
    Cow::Owned(out)
}

/// Pop bytes off the tail of `buf` until it ends on a complete UTF-8
/// codepoint. UTF-8 codepoints are at most 4 bytes, so this scans at
/// most 4 bytes — true O(1) in the buffer length, unlike the naive
/// `from_utf8`-in-a-loop pattern that re-scans the entire buffer per
/// pop.
///
/// Algorithm: count trailing continuation bytes (`0b10xx_xxxx`), then
/// look at the byte before them. That byte is either ASCII (clean
/// boundary, no work) or a multi-byte leader. If the leader's encoded
/// length matches the continuation count, the codepoint is complete
/// and we leave the buffer alone; otherwise the codepoint is partial
/// and we drop the leader plus its half-formed continuations.
///
/// Assumes `buf` was a valid UTF-8 prefix before truncation — the
/// only invalid suffix shape this needs to handle is the partial
/// codepoint at the cut point.
pub(super) fn trim_to_utf8_boundary(buf: &mut Vec<u8>) {
    let mut continuations = 0usize;
    while continuations < 3 && continuations < buf.len() {
        let b = buf[buf.len() - 1 - continuations];
        if b & 0xC0 == 0x80 {
            continuations += 1;
        } else {
            break;
        }
    }
    let leader_idx = match buf.len().checked_sub(continuations + 1) {
        Some(i) => i,
        // Buffer was all continuations (pathological — implies the
        // input was not a valid UTF-8 prefix). Drop everything.
        None => {
            buf.clear();
            return;
        }
    };
    let leader = buf[leader_idx];
    let expected_continuations = if leader < 0x80 {
        0
    } else if leader & 0xE0 == 0xC0 {
        1
    } else if leader & 0xF0 == 0xE0 {
        2
    } else if leader & 0xF8 == 0xF0 {
        3
    } else {
        // The byte before the trailing continuations is itself a
        // continuation byte (0xC0 == 0x80) or one of the disallowed
        // 0xFE/0xFF values. The buffer's tail is malformed UTF-8 —
        // drop the bad byte and the trailing continuations.
        buf.truncate(leader_idx);
        return;
    };
    if continuations < expected_continuations {
        // Partial codepoint — the file was cut mid-codepoint. Drop the
        // leader and any continuations we did read so the caller sees
        // only complete codepoints.
        buf.truncate(leader_idx);
    }
    // continuations == expected_continuations: complete codepoint, no
    // work. continuations > expected_continuations cannot happen on a
    // previously-valid UTF-8 prefix (the prior codepoint would have
    // claimed those bytes), so we don't bother handling it.
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sanitize_inline_passes_through_clean_text() {
        let s = "hello world / café / 漢字";
        let out = sanitize_inline(s);
        assert!(matches!(out, Cow::Borrowed(_)));
        assert_eq!(out, s);
    }

    #[test]
    fn sanitize_inline_escapes_line_breaks() {
        let dirty = "line1\nline2\rline3\0end";
        let out = sanitize_inline(dirty);
        assert_eq!(out, "line1\\nline2\\rline3\\0end");
    }

    #[test]
    fn sanitize_inline_keeps_tab_and_unicode_separators() {
        // Tabs are legitimate (TSV) and Unicode line separators don't
        // break our line-per-record contract — the sink consumer
        // tokenises on ASCII whitespace.
        let s = "col1\tcol2\u{2028}rest";
        let out = sanitize_inline(s);
        assert!(matches!(out, Cow::Borrowed(_)));
        assert_eq!(out, s);
    }

    #[test]
    fn trim_to_utf8_boundary_no_op_on_clean_ascii() {
        let mut v = b"hello".to_vec();
        trim_to_utf8_boundary(&mut v);
        assert_eq!(v, b"hello");
    }

    #[test]
    fn trim_to_utf8_boundary_pops_dangling_leader() {
        // 0xE6 starts a 3-byte sequence (e.g. `漢` = E6 BC A2). With
        // only the leader we have an incomplete codepoint.
        let mut v = vec![b'a', 0xE6];
        trim_to_utf8_boundary(&mut v);
        assert_eq!(v, b"a");
    }

    #[test]
    fn trim_to_utf8_boundary_pops_continuation_then_leader() {
        // Truncated `漢` (E6 BC A2) → keep only `E6 BC`. Trim should
        // pop the continuation BC, then the leader E6.
        let mut v = vec![b'a', 0xE6, 0xBC];
        trim_to_utf8_boundary(&mut v);
        assert_eq!(v, b"a");
    }

    #[test]
    fn trim_to_utf8_boundary_keeps_complete_codepoint() {
        // Complete `漢` (E6 BC A2) — must NOT be touched.
        let mut v = vec![b'a', 0xE6, 0xBC, 0xA2];
        trim_to_utf8_boundary(&mut v);
        assert_eq!(v, vec![b'a', 0xE6, 0xBC, 0xA2]);
    }

    #[test]
    fn trim_to_utf8_boundary_handles_empty_buffer() {
        let mut v: Vec<u8> = Vec::new();
        trim_to_utf8_boundary(&mut v);
        assert!(v.is_empty());
    }

    #[test]
    fn trim_to_utf8_boundary_keeps_complete_two_byte_codepoint() {
        // Complete `é` (C3 A9). leader expects 1 continuation, we
        // counted 1 → keep.
        let mut v = vec![b'a', 0xC3, 0xA9];
        trim_to_utf8_boundary(&mut v);
        assert_eq!(v, vec![b'a', 0xC3, 0xA9]);
    }

    #[test]
    fn trim_to_utf8_boundary_keeps_complete_four_byte_codepoint() {
        // Complete `🦀` (F0 9F A6 80). leader expects 3, counted 3 → keep.
        let mut v = vec![b'a', 0xF0, 0x9F, 0xA6, 0x80];
        trim_to_utf8_boundary(&mut v);
        assert_eq!(v, vec![b'a', 0xF0, 0x9F, 0xA6, 0x80]);
    }

    #[test]
    fn trim_to_utf8_boundary_drops_partial_four_byte_codepoint() {
        // Partial `🦀` — leader + 1 continuation. Drop leader + cont.
        let mut v = vec![b'a', 0xF0, 0x9F];
        trim_to_utf8_boundary(&mut v);
        assert_eq!(v, vec![b'a']);
    }
}
