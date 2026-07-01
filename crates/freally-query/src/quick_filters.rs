//! Quick-filter aliases: `audio:` / `video:` / `image:` / `document:` /
//! `executable:` / `archive:`. Each expands to a predefined extension set
//! (Everything-parity). Phase 5 wires them as `ext:` modifier shortcuts;
//! Phase 12's settings UI lets users author custom filters that plug
//! into the same registry.

use std::str::FromStr;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum QuickFilter {
    Audio,
    Video,
    Image,
    Document,
    Executable,
    Archive,
}

impl QuickFilter {
    pub fn extensions(self) -> &'static [&'static str] {
        match self {
            // Everything's audio set, plus modern lossless. Phase 9
            // checks that the audio-extractor matches this list.
            QuickFilter::Audio => &[
                "aac", "aif", "aiff", "alac", "ape", "flac", "m4a", "mid", "midi", "mka", "mp1",
                "mp2", "mp3", "mpc", "ogg", "oga", "opus", "ra", "tta", "wav", "wma", "wv",
            ],
            QuickFilter::Video => &[
                "3gp", "asf", "avi", "divx", "f4v", "flv", "m2ts", "m4v", "mkv", "mov", "mp4",
                "mpe", "mpeg", "mpg", "mts", "ogm", "ogv", "rm", "rmvb", "ts", "vob", "webm",
                "wmv",
            ],
            QuickFilter::Image => &[
                "ai", "avif", "bmp", "cur", "dds", "exr", "gif", "heic", "heif", "ico", "j2k",
                "jp2", "jpe", "jpeg", "jpg", "jxl", "png", "psd", "raw", "svg", "tga", "tif",
                "tiff", "webp",
            ],
            QuickFilter::Document => &[
                "csv", "doc", "docx", "epub", "json", "md", "odp", "ods", "odt", "pdf", "ppt",
                "pptx", "rst", "rtf", "tex", "text", "toml", "txt", "wpd", "wps", "xls", "xlsx",
                "xml", "yaml", "yml",
            ],
            QuickFilter::Executable => &[
                "app", "appimage", "bat", "bin", "cmd", "com", "deb", "dmg", "elf", "exe", "jar",
                "msi", "out", "pkg", "ps1", "rpm", "run", "sh", "wasm",
            ],
            QuickFilter::Archive => &[
                "7z", "bz2", "cab", "deb", "gz", "iso", "jar", "lz", "lz4", "lzma", "rar", "rpm",
                "tar", "tbz", "tbz2", "tgz", "txz", "war", "xz", "z", "zip", "zst", "zstd",
            ],
        }
    }
}

impl FromStr for QuickFilter {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_ascii_lowercase().as_str() {
            "audio" => Ok(QuickFilter::Audio),
            "video" => Ok(QuickFilter::Video),
            "image" | "pic" | "pics" | "picture" => Ok(QuickFilter::Image),
            "document" | "doc" | "docs" => Ok(QuickFilter::Document),
            "executable" | "exec" | "exe" => Ok(QuickFilter::Executable),
            "archive" | "compressed" => Ok(QuickFilter::Archive),
            _ => Err(()),
        }
    }
}
