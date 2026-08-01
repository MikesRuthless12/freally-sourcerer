//! Directory-shape statistics derived from the canonical store
//! (SRC-M08).
//!
//! `empty:`, `child-count:` and `descendant-count:` are set-shaped
//! predicates: whether a folder is empty is a property of *other* rows,
//! not of the folder's own record. Rather than walk the filesystem at
//! query time, we derive the whole shape from `files.db` in one pass:
//!
//! 1. Stream `(path, attrs)` for every live row. Each row bumps its
//!    parent's direct-child counter (and, when the row is not a
//!    directory, the parent's direct-*file* counter).
//! 2. Sort the directory paths deepest-first and roll each directory's
//!    totals into its parent. One sweep, `O(N + D log D)`.
//!
//! The result answers three questions per directory in `O(1)`:
//! direct children, all descendants, and non-directory descendants —
//! the last being what "empty subtree" means (a chain of folders that
//! holds no actual files is still empty).
//!
//! Building this is a full-table scan, so the executor only builds it
//! when the parsed query actually contains an emptiness predicate. It
//! never touches the filesystem.

use std::collections::HashMap;
use std::path::Path;

use crate::error::IndexError;
use crate::store::Store;

/// `FILE_ATTRIBUTE_DIRECTORY`. The journal subscribers project every
/// OS's directory flag onto this bit (see `freally_query::AttribFlag`),
/// so it is portable across all three platforms.
pub const ATTR_DIRECTORY: u64 = 0x0000_0010;

/// Per-directory counts. Every field is derived purely from index rows.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct DirCounts {
    /// Immediate children (files *and* folders).
    pub children: u64,
    /// Every descendant at any depth (files *and* folders).
    pub descendants: u64,
    /// Every non-directory descendant at any depth. Zero means the
    /// subtree holds no real files, however many empty folders it nests.
    pub file_descendants: u64,
}

/// Directory shape for the whole index, keyed by path.
#[derive(Debug, Clone, Default)]
pub struct DirStats {
    counts: HashMap<String, DirCounts>,
    /// Keys that came from a real directory row, as opposed to a parent
    /// key synthesised from some child's path. `counts` holds both —
    /// every row creates an entry for its parent — so without this,
    /// "is this folder's parent indexed?" is always yes, including for
    /// the scan root's own parent.
    indexed_dirs: std::collections::HashSet<String>,
}

impl DirStats {
    /// Derive the shape of every directory in the canonical store.
    pub fn build(store: &Store) -> Result<Self, IndexError> {
        let mut counts: HashMap<String, DirCounts> = HashMap::new();
        let mut dirs: Vec<String> = Vec::new();

        store.iter_path_attrs(|path, attrs| {
            let is_dir = attrs & ATTR_DIRECTORY != 0;
            if is_dir {
                // Make sure every directory has an entry even when it
                // holds nothing — that is precisely the empty case.
                let key = key_of(path);
                counts.entry(key.clone()).or_default();
                dirs.push(key);
            }
            if let Some(parent) = Path::new(path).parent() {
                let entry = counts.entry(key_of(&parent.to_string_lossy())).or_default();
                entry.children += 1;
                entry.descendants += 1;
                if !is_dir {
                    entry.file_descendants += 1;
                }
            }
        })?;

        // Deepest-first, so a directory's totals are final before they
        // roll into its parent. Depth is computed once per directory
        // and carried through the sort — recomputing it inside the
        // comparator would rescan two paths per comparison, which is
        // `2·D·log D` path scans on a half-million-directory volume.
        let mut dirs: Vec<(usize, String)> = dirs.into_iter().map(|d| (depth_of(&d), d)).collect();
        dirs.sort_unstable_by(|(da, a), (db, b)| db.cmp(da).then_with(|| a.cmp(b)));
        for (_, dir) in &dirs {
            let Some(parent) = Path::new(dir).parent() else {
                continue;
            };
            let parent_key = key_of(&parent.to_string_lossy());
            if parent_key == *dir {
                // A filesystem root is its own parent (`C:\` → `C:\`);
                // rolling it into itself would loop the totals.
                continue;
            }
            let Some(child) = counts.get(dir).copied() else {
                continue;
            };
            if let Some(entry) = counts.get_mut(&parent_key) {
                entry.descendants += child.descendants;
                entry.file_descendants += child.file_descendants;
            }
        }

        Ok(Self {
            counts,
            indexed_dirs: dirs.into_iter().map(|(_, key)| key).collect(),
        })
    }

    /// Counts for one directory. Unknown paths report all-zero, which is
    /// the honest answer for a folder the index has never seen.
    pub fn counts(&self, path: &Path) -> DirCounts {
        self.counts
            .get(&key_of(&path.to_string_lossy()))
            .copied()
            .unwrap_or_default()
    }

    /// True when `path` is the top of a subtree that holds no files —
    /// its parent must hold at least one file somewhere, otherwise the
    /// parent is the real root and this is just a link in the chain.
    pub fn is_empty_subtree_root(&self, path: &Path) -> bool {
        if self.counts(path).file_descendants != 0 {
            return false;
        }
        let Some(parent) = path.parent().filter(|p| *p != path) else {
            return true;
        };
        let parent_key = key_of(&parent.to_string_lossy());
        // Only a parent the index actually holds as a directory can be
        // the higher root. Every row synthesises a `counts` entry for
        // its parent, so checking `counts` alone would treat the scan
        // root's own parent as indexed and report no roots at all when
        // the whole scanned tree is file-free.
        if !self.indexed_dirs.contains(&parent_key) {
            return true;
        }
        match self.counts.get(&parent_key) {
            // Parent is also file-free — it is the higher root, so this
            // one collapses into it.
            Some(c) => c.file_descendants != 0,
            None => true,
        }
    }
}

/// Normalise a path into a map key. Windows and macOS default to
/// case-insensitive filesystems, so a child's `C:\Users\...` parent must
/// key-match the folder row stored as `C:\users\...`. Linux is
/// case-sensitive and keeps the path verbatim.
fn key_of(path: &str) -> String {
    let trimmed = trim_trailing_separator(path);
    if cfg!(any(windows, target_os = "macos")) {
        trimmed.to_lowercase()
    } else {
        trimmed.to_string()
    }
}

/// Drop a trailing separator so `C:\Users\` and `C:\Users` key alike.
/// A bare root (`/`, `C:\`) keeps its separator — stripping it would
/// collapse `/` to the empty string.
fn trim_trailing_separator(path: &str) -> &str {
    let stripped = path.trim_end_matches(['/', '\\']);
    if stripped.is_empty() || stripped.ends_with(':') {
        path
    } else {
        stripped
    }
}

fn depth_of(path: &str) -> usize {
    // Both separators are ASCII, so bytes suffice and this avoids
    // UTF-8-decoding every path on a full-volume build.
    path.bytes().filter(|b| *b == b'/' || *b == b'\\').count()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::store::FileRow;
    use std::path::PathBuf;
    use tempfile::tempdir;

    fn row(path: &str, is_dir: bool, size: u64) -> FileRow {
        let p = PathBuf::from(path);
        let name = p
            .file_name()
            .map(|s| s.to_string_lossy().into_owned())
            .unwrap_or_default();
        FileRow {
            file_id: path.len() as i64 * 7 + path.bytes().map(i64::from).sum::<i64>(),
            path: p,
            name_lower: name.to_lowercase(),
            name,
            ext: None,
            size,
            mtime_ns: 0,
            ctime_ns: 0,
            attrs: if is_dir { ATTR_DIRECTORY } else { 0 },
            volume: String::new(),
        }
    }

    fn stats_for(rows: &[FileRow]) -> DirStats {
        let dir = tempdir().unwrap();
        let store = Store::open(&dir.path().join("files.db")).unwrap();
        store.bulk_upsert(rows).unwrap();
        DirStats::build(&store).unwrap()
    }

    #[test]
    fn counts_direct_children_and_descendants() {
        let stats = stats_for(&[
            row("/root", true, 0),
            row("/root/a", true, 0),
            row("/root/a/one.txt", false, 10),
            row("/root/a/two.txt", false, 20),
            row("/root/b.txt", false, 5),
        ]);
        let root = stats.counts(Path::new("/root"));
        assert_eq!(root.children, 2, "a/ and b.txt");
        assert_eq!(root.descendants, 4, "a/, b.txt, one.txt, two.txt");
        assert_eq!(root.file_descendants, 3);

        let a = stats.counts(Path::new("/root/a"));
        assert_eq!(a.children, 2);
        assert_eq!(a.descendants, 2);
        assert_eq!(a.file_descendants, 2);
    }

    #[test]
    fn empty_folder_has_zero_children() {
        let stats = stats_for(&[row("/root", true, 0), row("/root/hollow", true, 0)]);
        assert_eq!(stats.counts(Path::new("/root/hollow")).children, 0);
        assert_eq!(stats.counts(Path::new("/root")).children, 1);
    }

    #[test]
    fn nested_empty_chain_collapses_to_its_top() {
        // /keep holds a file, so /keep/a is the top of the empty chain.
        let stats = stats_for(&[
            row("/keep", true, 0),
            row("/keep/real.txt", false, 1),
            row("/keep/a", true, 0),
            row("/keep/a/b", true, 0),
            row("/keep/a/b/c", true, 0),
        ]);
        assert!(stats.is_empty_subtree_root(Path::new("/keep/a")));
        assert!(!stats.is_empty_subtree_root(Path::new("/keep/a/b")));
        assert!(!stats.is_empty_subtree_root(Path::new("/keep/a/b/c")));
        assert!(!stats.is_empty_subtree_root(Path::new("/keep")));
    }

    #[test]
    fn unknown_path_reports_zeroes() {
        let stats = stats_for(&[row("/root", true, 0)]);
        assert_eq!(stats.counts(Path::new("/nowhere")), DirCounts::default());
    }
}
