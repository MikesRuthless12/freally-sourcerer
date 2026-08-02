//! SRC-M14 — which volume a path lives on.
//!
//! The `volume` column, its SQLite index, and the Tantivy field all
//! shipped in Phase 4, but nothing ever wrote to them: the one row
//! constructor on the ingest path hard-coded an empty string. Offline
//! catalogs need that column to actually mean something, because after a
//! drive is unplugged its *path* is the only thing left and the path
//! alone cannot tell you which device it was.
//!
//! Resolution is a longest-prefix match over real mount points rather
//! than a guess from the path shape. `C:\` is unambiguous, but
//! `/media/me/photos` could be a mount point or an ordinary directory on
//! `/`, and only the mount table knows which. `freally-indexd` owns that
//! table (`volumes::detect()`) and installs it here; an index with no map
//! resolves everything to `""`, which is exactly the pre-M14 behaviour.

use std::path::{Path, PathBuf};

/// Mount-point → volume-id table, ordered longest-prefix-first.
#[derive(Debug, Clone, Default)]
pub struct VolumeMap {
    /// `(lower-cased mount point without a trailing separator, volume id)`,
    /// longest first so the first match is the most specific one.
    entries: Vec<(String, String)>,
}

impl VolumeMap {
    /// Build from `(mount_point, volume_id)` pairs in any order.
    pub fn new(entries: impl IntoIterator<Item = (PathBuf, String)>) -> Self {
        let mut entries: Vec<(String, String)> = entries
            .into_iter()
            .map(|(mount, id)| (normalize(&mount), id))
            .filter(|(mount, id)| !id.is_empty() && !mount.is_empty())
            .collect();
        // Longest first: `/media/me/usb` has to win over `/`.
        entries.sort_by(|a, b| b.0.len().cmp(&a.0.len()).then_with(|| a.0.cmp(&b.0)));
        entries.dedup_by(|a, b| a.0 == b.0);
        Self { entries }
    }

    pub fn is_empty(&self) -> bool {
        self.entries.is_empty()
    }

    /// The volume id owning `path`, or `""` when nothing matches.
    pub fn resolve(&self, path: &Path) -> String {
        let p = normalize(path);
        for (mount, id) in &self.entries {
            if is_under(&p, mount) {
                return id.clone();
            }
        }
        String::new()
    }
}

/// Lower-case, forward-slash, no trailing separator. Case folding is
/// unconditional: a volume id has to stay stable across the case
/// variations Windows hands out for the same drive, and on Unix two
/// mount points differing only by case are not a distinction worth
/// splitting a catalog over.
fn normalize(p: &Path) -> String {
    let s = p.to_string_lossy().replace('\\', "/").to_lowercase();
    let trimmed = s.trim_end_matches('/');
    if trimmed.is_empty() && s.starts_with('/') {
        // The Unix root is a mount point whose normalized form is empty.
        "/".to_string()
    } else {
        trimmed.to_string()
    }
}

/// True when `path` is `mount` itself or sits beneath it. Compares whole
/// components, so `/media/usb2` is not treated as living under
/// `/media/usb`.
fn is_under(path: &str, mount: &str) -> bool {
    if mount == "/" {
        return path.starts_with('/');
    }
    if path == mount {
        return true;
    }
    path.len() > mount.len()
        && path.starts_with(mount)
        && path.as_bytes().get(mount.len()) == Some(&b'/')
}

#[cfg(test)]
mod tests {
    use super::*;

    fn map(pairs: &[(&str, &str)]) -> VolumeMap {
        VolumeMap::new(
            pairs
                .iter()
                .map(|(m, i)| (PathBuf::from(m), i.to_string()))
                .collect::<Vec<_>>(),
        )
    }

    #[test]
    fn an_empty_map_resolves_to_nothing() {
        assert_eq!(VolumeMap::default().resolve(Path::new("/a/b")), "");
    }

    #[test]
    fn the_most_specific_mount_point_wins() {
        let m = map(&[("/", "lin-root"), ("/media/me/usb", "lin-usb")]);
        assert_eq!(m.resolve(Path::new("/media/me/usb/photo.jpg")), "lin-usb");
        assert_eq!(m.resolve(Path::new("/home/me/notes.txt")), "lin-root");
    }

    #[test]
    fn a_mount_point_matches_itself() {
        let m = map(&[("/media/me/usb", "lin-usb")]);
        assert_eq!(m.resolve(Path::new("/media/me/usb")), "lin-usb");
        assert_eq!(m.resolve(Path::new("/media/me/usb/")), "lin-usb");
    }

    #[test]
    fn a_sibling_with_a_shared_prefix_is_not_a_match() {
        // The bug a naive `starts_with` would ship: `/media/usb2` is a
        // different device from `/media/usb`.
        let m = map(&[("/media/usb", "a")]);
        assert_eq!(m.resolve(Path::new("/media/usb2/file")), "");
    }

    #[test]
    fn windows_drive_letters_resolve_regardless_of_case_or_separator() {
        let m = map(&[("C:\\", "win-C"), ("D:\\", "win-D")]);
        assert_eq!(m.resolve(Path::new("c:\\users\\me\\a.txt")), "win-C");
        assert_eq!(m.resolve(Path::new("C:/Users/Me/a.txt")), "win-C");
        assert_eq!(m.resolve(Path::new("D:\\media\\b.mp3")), "win-D");
        assert_eq!(m.resolve(Path::new("E:\\other")), "");
    }

    #[test]
    fn entries_without_an_id_are_ignored() {
        let m = map(&[("/media/usb", "")]);
        assert!(m.is_empty());
        assert_eq!(m.resolve(Path::new("/media/usb/x")), "");
    }

    #[test]
    fn the_unix_root_covers_everything_else() {
        let m = map(&[("/", "lin-root")]);
        assert_eq!(m.resolve(Path::new("/anything/at/all")), "lin-root");
    }
}
