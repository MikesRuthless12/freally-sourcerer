//! Archive-peek extractor — list entries in zip / 7z / tar without
//! extracting bytes to disk.
//!
//! Build Guide Phase 8: "for zip / 7z / tar, list entries + per-entry
//! name + size, do NOT extract contents to disk; treat archive-entry
//! filenames as searchable virtual paths (myfile.zip!path/to/inner.txt)."
//!
//! Output shape (one line per entry):
//!
//! ```text
//! archive.zip!path/to/inner.txt size=1234
//! ```
//!
//! The `archive.ext!inner/path` syntax is the "virtual path" the
//! daemon hands the indexer; a `content:` query that includes
//! `inner.txt` matches it just like any other filename token. The
//! `size=1234` tail is queryable via the `size:` modifier in Phase 10.
//!
//! ### Dep choice (Build-Guide deviation)
//!
//! The Build Guide names `compress-tools` (libarchive bindings).
//! libarchive is a system dep — Windows CI hosts don't ship it, and a
//! pure-Rust path keeps `cargo build` working everywhere without
//! `vcpkg` orchestration. We use `zip` + `sevenz-rust2` + `tar`
//! (all permissive). The trade-off: tar.zst / tar.xz handling
//! requires the host decompressor crate; we accept that as out-of-
//! scope for Phase 8 and surface unsupported variants as
//! [`ExtractError::Unsupported`].

use std::fs::File;
use std::io::BufReader;
use std::path::Path;

use crate::sink::TextSink;
use crate::{ExtractError, ExtractionStats, Extractor, ExtractorId};

/// Maximum entries the extractor lists before it stops. A tar of
/// 1M files would otherwise burn the sink and the time budget on a
/// single archive; the cap is high enough to cover legitimate
/// project archives (~100k entries) without becoming a DoS vector.
pub const MAX_ENTRIES: usize = 100_000;

/// Cancel-flag check interval (every N entries).
const CANCEL_CHECK_EVERY: usize = 256;

#[derive(Debug, Clone, Copy)]
enum Format {
    Zip,
    SevenZ,
    Tar,
}

#[derive(Debug, Default, Clone)]
pub struct ArchivePeekExtractor {
    pub max_entries: Option<usize>,
}

impl ArchivePeekExtractor {
    pub fn with_max_entries(max_entries: usize) -> Self {
        Self {
            max_entries: Some(max_entries),
        }
    }

    fn cap(&self) -> usize {
        self.max_entries.unwrap_or(MAX_ENTRIES)
    }

    fn detect(path: &Path, magic: &[u8]) -> Option<Format> {
        // Zip: PK\x03\x04 (local file header). 7z: 6-byte signature
        // 0x37 0x7A 0xBC 0xAF 0x27 0x1C. Tar magic ("ustar") sits at
        // offset 257, which is past our 32-byte head — fall back to
        // the .tar / .tgz / .tar.gz / .tar.bz2 / .tar.xz extensions.
        if magic.starts_with(b"PK\x03\x04") {
            return Some(Format::Zip);
        }
        if magic.starts_with(&[0x37, 0x7A, 0xBC, 0xAF, 0x27, 0x1C]) {
            return Some(Format::SevenZ);
        }
        if let Some(ext) = path.extension().and_then(|s| s.to_str()) {
            let lower = ext.to_ascii_lowercase();
            if lower == "tar" {
                return Some(Format::Tar);
            }
            if lower == "zip" || lower == "epub" || lower == "jar" {
                return Some(Format::Zip);
            }
            if lower == "7z" {
                return Some(Format::SevenZ);
            }
        }
        // .tar.gz / .tgz etc. — handled in a future pass; ignore for now.
        None
    }
}

impl Extractor for ArchivePeekExtractor {
    fn id(&self) -> ExtractorId {
        ExtractorId::new("archive-peek")
    }

    fn matches(&self, path: &Path, magic: &[u8]) -> bool {
        Self::detect(path, magic).is_some()
    }

    fn extract(&self, path: &Path, sink: &mut TextSink) -> Result<ExtractionStats, ExtractError> {
        let format = Self::detect(path, &[]).ok_or_else(|| {
            ExtractError::Unsupported("path does not look like zip/7z/tar".into())
        })?;
        let cap = self.cap();
        let archive_name = path
            .file_name()
            .and_then(|s| s.to_str())
            .unwrap_or("archive");

        let bytes_in = match format {
            Format::Zip => list_zip(path, archive_name, cap, sink)?,
            Format::SevenZ => list_7z(path, archive_name, cap, sink)?,
            Format::Tar => list_tar(path, archive_name, cap, sink)?,
        };

        Ok(ExtractionStats {
            bytes_in,
            bytes_out: sink.len() as u64,
            elapsed: std::time::Duration::ZERO,
        })
    }
}

fn list_zip(
    path: &Path,
    archive_name: &str,
    cap: usize,
    sink: &mut TextSink,
) -> Result<u64, ExtractError> {
    let file = File::open(path).map_err(|e| ExtractError::io(path, e))?;
    let bytes_in = file.metadata().map(|m| m.len()).unwrap_or(0);
    let mut zip = zip::ZipArchive::new(BufReader::new(file))
        .map_err(|e| ExtractError::Malformed(format!("zip open failed: {e}")))?;

    let entry_count = zip.len();
    let listed = entry_count.min(cap);
    for i in 0..listed {
        if i % CANCEL_CHECK_EVERY == 0 && sink.is_cancelled() {
            return Err(ExtractError::Cancelled);
        }
        let entry = zip
            .by_index(i)
            .map_err(|e| ExtractError::Malformed(format!("zip index {i}: {e}")))?;
        // Per Build Guide: `myfile.zip!path/to/inner.txt`. Append
        // size= so the daemon can later wire it to the `size:`
        // modifier without re-opening the archive.
        // Phase 8 review pass: a hostile zip can declare an entry
        // name containing `\n` / `\r` / `\0`, which would inject
        // synthetic line breaks into the search blob and let one
        // entry impersonate many. Sanitise before format!.
        let safe_name = super::util::sanitize_inline(entry.name());
        let line = format!(
            "{archive_name}!{safe_name} size={size}\n",
            size = entry.size()
        );
        sink.push_str(&line)
            .map_err(|_| ExtractError::OutputTooLarge { cap: sink.cap() })?;
    }
    if entry_count > listed {
        write_truncation_marker(archive_name, listed, entry_count, sink)?;
    }
    Ok(bytes_in)
}

fn list_7z(
    path: &Path,
    archive_name: &str,
    cap: usize,
    sink: &mut TextSink,
) -> Result<u64, ExtractError> {
    let file = File::open(path).map_err(|e| ExtractError::io(path, e))?;
    let bytes_in = file.metadata().map(|m| m.len()).unwrap_or(0);
    // sevenz-rust2 0.21: `Archive::read` walks the central index
    // without extracting bytes; we never call any of the
    // `decompress_*` helpers, so the archive contents stay on disk.
    let mut reader = BufReader::new(file);
    let archive = sevenz_rust2::Archive::read(&mut reader, &sevenz_rust2::Password::empty())
        .map_err(|e| ExtractError::Malformed(format!("7z open failed: {e}")))?;

    let total = archive.files.len();
    let listed = total.min(cap);
    for (i, entry) in archive.files.iter().take(listed).enumerate() {
        if i % CANCEL_CHECK_EVERY == 0 && sink.is_cancelled() {
            return Err(ExtractError::Cancelled);
        }
        // Same hostile-name sanitization as the zip path — a 7z entry
        // name with embedded `\n` would otherwise inject phantom rows
        // into the search blob.
        let safe_name = super::util::sanitize_inline(entry.name());
        let line = format!(
            "{archive_name}!{safe_name} size={size}\n",
            size = entry.size()
        );
        sink.push_str(&line)
            .map_err(|_| ExtractError::OutputTooLarge { cap: sink.cap() })?;
    }
    if total > listed {
        write_truncation_marker(archive_name, listed, total, sink)?;
    }
    Ok(bytes_in)
}

fn list_tar(
    path: &Path,
    archive_name: &str,
    cap: usize,
    sink: &mut TextSink,
) -> Result<u64, ExtractError> {
    let file = File::open(path).map_err(|e| ExtractError::io(path, e))?;
    let bytes_in = file.metadata().map(|m| m.len()).unwrap_or(0);
    let mut archive = tar::Archive::new(BufReader::new(file));
    let mut listed = 0usize;
    let mut hit_cap = false;
    for (i, entry_result) in archive
        .entries()
        .map_err(|e| ExtractError::Malformed(format!("tar open failed: {e}")))?
        .enumerate()
    {
        if i >= cap {
            hit_cap = true;
            break;
        }
        listed = i + 1;
        if i % CANCEL_CHECK_EVERY == 0 && sink.is_cancelled() {
            return Err(ExtractError::Cancelled);
        }
        let entry = entry_result.map_err(|e| ExtractError::Malformed(format!("tar entry: {e}")))?;
        let path_in_archive = entry
            .path()
            .map_err(|e| ExtractError::Malformed(format!("tar path: {e}")))?;
        // tar paths can be non-UTF-8 (legacy archives). `to_string_lossy`
        // substitutes U+FFFD for invalid bytes so search recall doesn't
        // drop the entry; entry names never reach a filesystem path
        // argument so the lossy round-trip is search-only. The follow-on
        // `sanitize_inline` strips embedded `\n` / `\r` / `\0` so a
        // hostile tar entry name cannot inject phantom rows into the
        // search blob.
        let lossy = path_in_archive.to_string_lossy();
        let safe_name = super::util::sanitize_inline(&lossy);
        let size = entry.header().size().unwrap_or(0);
        let line = format!("{archive_name}!{safe_name} size={size}\n");
        sink.push_str(&line)
            .map_err(|_| ExtractError::OutputTooLarge { cap: sink.cap() })?;
    }
    if hit_cap {
        // tar's iterator doesn't yield a total count up front, so we
        // can't render "listed/total"; the marker just notes the cap
        // tripped. Search recall is preserved for the first `cap`
        // entries, which is the contract.
        sink.push_str(&format!(
            "{archive_name}!_truncated_=true entries_listed={listed}\n"
        ))
        .map_err(|_| ExtractError::OutputTooLarge { cap: sink.cap() })?;
    }
    Ok(bytes_in)
}

/// Append a "truncation marker" line so a search snippet against a
/// listing-capped archive is honest about the missing tail. Format:
///
/// ```text
/// archive!_truncated_=true entries_listed=N total=M
/// ```
fn write_truncation_marker(
    archive_name: &str,
    listed: usize,
    total: usize,
    sink: &mut TextSink,
) -> Result<(), ExtractError> {
    let line = format!("{archive_name}!_truncated_=true entries_listed={listed} total={total}\n");
    sink.push_str(&line)
        .map_err(|_| ExtractError::OutputTooLarge { cap: sink.cap() })
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::{Cursor, Write};
    use tempfile::tempdir;

    fn write_fixture(name: &str, bytes: &[u8]) -> std::path::PathBuf {
        let dir = tempdir().unwrap().keep();
        let path = dir.join(name);
        let mut f = File::create(&path).unwrap();
        f.write_all(bytes).unwrap();
        path
    }

    fn make_zip(entries: &[(&str, &[u8])]) -> Vec<u8> {
        let buf = Cursor::new(Vec::<u8>::new());
        let mut zip = zip::ZipWriter::new(buf);
        let opts: zip::write::SimpleFileOptions = zip::write::SimpleFileOptions::default()
            .compression_method(zip::CompressionMethod::Stored);
        for (name, data) in entries {
            zip.start_file(*name, opts).unwrap();
            zip.write_all(data).unwrap();
        }
        zip.finish().unwrap().into_inner()
    }

    fn make_tar(entries: &[(&str, &[u8])]) -> Vec<u8> {
        let mut buf = Vec::new();
        {
            let mut t = tar::Builder::new(&mut buf);
            for (name, data) in entries {
                let mut h = tar::Header::new_gnu();
                h.set_size(data.len() as u64);
                h.set_cksum();
                t.append_data(&mut h, name, *data).unwrap();
            }
            t.finish().unwrap();
        }
        buf
    }

    #[test]
    fn matches_zip_by_magic_and_extension() {
        let ext = ArchivePeekExtractor::default();
        assert!(ext.matches(Path::new("/tmp/file.zip"), b""));
        assert!(ext.matches(Path::new("/tmp/no-ext"), b"PK\x03\x04..."));
        assert!(!ext.matches(Path::new("/tmp/notes.txt"), b"hi"));
    }

    #[test]
    fn lists_zip_entries_with_size_and_virtual_path() {
        let bytes = make_zip(&[("README.md", b"# Hello"), ("src/main.rs", b"fn main() {}")]);
        let path = write_fixture("test.zip", &bytes);
        let ext = ArchivePeekExtractor::default();
        let mut sink = TextSink::new(1024);
        ext.extract(&path, &mut sink).unwrap();
        let out = std::str::from_utf8(sink.as_bytes()).unwrap();
        assert!(out.contains("test.zip!README.md size=7"));
        assert!(out.contains("test.zip!src/main.rs size=12"));
    }

    #[test]
    fn lists_tar_entries() {
        let bytes = make_tar(&[("a.txt", b"a"), ("nested/b.txt", b"bb")]);
        let path = write_fixture("test.tar", &bytes);
        let ext = ArchivePeekExtractor::default();
        let mut sink = TextSink::new(1024);
        ext.extract(&path, &mut sink).unwrap();
        let out = std::str::from_utf8(sink.as_bytes()).unwrap();
        assert!(out.contains("test.tar!a.txt size=1"));
        assert!(out.contains("test.tar!nested/b.txt size=2"));
    }

    #[test]
    fn caps_listed_entries_and_emits_truncation_marker() {
        let names: Vec<String> = (0..20).map(|i| format!("file{i}.txt")).collect();
        let entries: Vec<(&str, &[u8])> = names.iter().map(|n| (n.as_str(), &b"x"[..])).collect();
        let bytes = make_zip(&entries);
        let path = write_fixture("many.zip", &bytes);
        let ext = ArchivePeekExtractor::with_max_entries(5);
        let mut sink = TextSink::new(2048);
        ext.extract(&path, &mut sink).unwrap();
        let out = std::str::from_utf8(sink.as_bytes()).unwrap();
        // 5 entry lines + 1 truncation-marker line.
        assert_eq!(out.lines().count(), 6);
        assert!(out.contains("_truncated_=true entries_listed=5 total=20"));
    }

    #[test]
    fn does_not_match_plain_text() {
        let ext = ArchivePeekExtractor::default();
        assert!(!ext.matches(Path::new("/tmp/notes.md"), b"# hello"));
    }

    /// Phase 8 review pass — regression for the archive-name search-
    /// index poisoning bug. A hostile zip can declare an entry name
    /// containing `\n`, which would split the listing line into two
    /// rows and let one entry impersonate two in the search blob.
    /// `sanitize_inline` escapes the line-break before format!, so
    /// the listing keeps exactly one line per entry.
    #[test]
    fn entry_name_with_newline_is_sanitized() {
        let bytes = make_zip(&[("evil\nfake-row.txt", b"x")]);
        let path = write_fixture("hostile.zip", &bytes);
        let mut sink = TextSink::new(1024);
        ArchivePeekExtractor::default()
            .extract(&path, &mut sink)
            .unwrap();
        let out = std::str::from_utf8(sink.as_bytes()).unwrap();
        // Exactly one line per entry — the embedded `\n` was escaped.
        assert_eq!(out.lines().count(), 1);
        assert!(out.contains("evil\\nfake-row.txt"));
        assert!(!out.contains("evil\nfake-row.txt"));
    }
}
