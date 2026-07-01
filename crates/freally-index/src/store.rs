//! SQLite canonical `FileRecord` store (TASK-032).
//!
//! `files.db` is the source of truth: Tantivy is rebuildable from it, the
//! custom name index is rebuildable from it, and crash recovery falls
//! back to it. WAL mode + `synchronous=NORMAL` is the durability /
//! throughput point per the Build Guide.

use std::path::{Path, PathBuf};
use std::sync::Arc;

use parking_lot::Mutex;
use rusqlite::{Connection, OpenFlags, params};

use crate::error::IndexError;

const SCHEMA_SQL: &str = r#"
CREATE TABLE IF NOT EXISTS files (
    file_id     INTEGER PRIMARY KEY,
    path        TEXT NOT NULL UNIQUE,
    name        TEXT NOT NULL,
    name_lower  TEXT NOT NULL,
    ext         TEXT,
    size        INTEGER NOT NULL DEFAULT 0,
    mtime_ns    INTEGER NOT NULL DEFAULT 0,
    ctime_ns    INTEGER NOT NULL DEFAULT 0,
    attrs       INTEGER NOT NULL DEFAULT 0,
    volume      TEXT NOT NULL DEFAULT ''
);

CREATE INDEX IF NOT EXISTS files_name_lower_idx ON files(name_lower);
CREATE INDEX IF NOT EXISTS files_ext_idx ON files(ext);
CREATE INDEX IF NOT EXISTS files_volume_idx ON files(volume);
"#;

/// Single-connection SQLite façade. Wrapped in a `parking_lot::Mutex`
/// because all writers serialize through one connection — simpler than
/// fighting SQLite's per-connection locking semantics, and good enough
/// for the indexer's MPSC-batched apply loop.
#[derive(Clone)]
pub struct Store {
    inner: Arc<Mutex<Connection>>,
    path: PathBuf,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FileRow {
    pub file_id: i64,
    pub path: PathBuf,
    pub name: String,
    pub name_lower: String,
    pub ext: Option<String>,
    pub size: u64,
    pub mtime_ns: i64,
    pub ctime_ns: i64,
    pub attrs: u64,
    pub volume: String,
}

impl Store {
    pub fn open(db_path: &Path) -> Result<Self, IndexError> {
        let conn = Connection::open_with_flags(
            db_path,
            OpenFlags::SQLITE_OPEN_READ_WRITE | OpenFlags::SQLITE_OPEN_CREATE,
        )?;
        // WAL + NORMAL is the recommended durability point for indexers
        // that batch-commit through Tantivy. WAL also unblocks readers
        // during long writes.
        conn.execute_batch(
            "
            PRAGMA journal_mode=WAL;
            PRAGMA synchronous=NORMAL;
            PRAGMA temp_store=MEMORY;
            PRAGMA foreign_keys=ON;
            ",
        )?;
        conn.execute_batch(SCHEMA_SQL)?;
        Ok(Self {
            inner: Arc::new(Mutex::new(conn)),
            path: db_path.to_path_buf(),
        })
    }

    pub fn path(&self) -> &Path {
        &self.path
    }

    /// Bulk variant of [`Store::upsert`] — wraps every row in a single
    /// transaction and reuses one prepared statement. ~100× faster than
    /// looping `upsert()` when the per-statement autocommit dominates
    /// (the common case during bootstrap, where N is 100K-1M+).
    pub fn bulk_upsert(&self, rows: &[FileRow]) -> Result<(), IndexError> {
        if rows.is_empty() {
            return Ok(());
        }
        let mut conn = self.inner.lock();
        let tx = conn.transaction()?;
        {
            let mut stmt = tx.prepare(
                "INSERT INTO files (file_id, path, name, name_lower, ext, size, mtime_ns, ctime_ns, attrs, volume)
                 VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10)
                 ON CONFLICT(file_id) DO UPDATE SET
                    path=excluded.path,
                    name=excluded.name,
                    name_lower=excluded.name_lower,
                    ext=excluded.ext,
                    size=excluded.size,
                    mtime_ns=excluded.mtime_ns,
                    ctime_ns=excluded.ctime_ns,
                    attrs=excluded.attrs,
                    volume=excluded.volume",
            )?;
            for row in rows {
                let path_str = row.path.to_string_lossy();
                stmt.execute(params![
                    row.file_id,
                    path_str,
                    row.name,
                    row.name_lower,
                    row.ext,
                    row.size as i64,
                    row.mtime_ns,
                    row.ctime_ns,
                    row.attrs as i64,
                    row.volume,
                ])?;
            }
        }
        tx.commit()?;
        Ok(())
    }

    /// Insert-or-replace a row keyed by `file_id`.
    pub fn upsert(&self, row: &FileRow) -> Result<(), IndexError> {
        let conn = self.inner.lock();
        // SQLite columns are UTF-8; non-UTF-8 path bytes get the
        // standard Rust replacement-character treatment. Phase 13's
        // perf pass swaps in an os-string aware bytes column.
        let path_str = row.path.to_string_lossy();
        conn.execute(
            "INSERT INTO files (file_id, path, name, name_lower, ext, size, mtime_ns, ctime_ns, attrs, volume)
             VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10)
             ON CONFLICT(file_id) DO UPDATE SET
                path=excluded.path,
                name=excluded.name,
                name_lower=excluded.name_lower,
                ext=excluded.ext,
                size=excluded.size,
                mtime_ns=excluded.mtime_ns,
                ctime_ns=excluded.ctime_ns,
                attrs=excluded.attrs,
                volume=excluded.volume",
            params![
                row.file_id,
                path_str,
                row.name,
                row.name_lower,
                row.ext,
                row.size as i64,
                row.mtime_ns,
                row.ctime_ns,
                row.attrs as i64,
                row.volume,
            ],
        )?;
        Ok(())
    }

    /// Update fields touched by a `Modify` event.
    pub fn update_modify(
        &self,
        file_id: i64,
        size: u64,
        mtime_ns: i64,
        attrs: u64,
    ) -> Result<(), IndexError> {
        let conn = self.inner.lock();
        conn.execute(
            "UPDATE files SET size=?1, mtime_ns=?2, attrs=?3 WHERE file_id=?4",
            params![size as i64, mtime_ns, attrs as i64, file_id],
        )?;
        Ok(())
    }

    /// Update fields touched by an `AttrChange` event.
    pub fn update_attrs(&self, file_id: i64, attrs: u64) -> Result<(), IndexError> {
        let conn = self.inner.lock();
        conn.execute(
            "UPDATE files SET attrs=?1 WHERE file_id=?2",
            params![attrs as i64, file_id],
        )?;
        Ok(())
    }

    /// Rewrite the path-derived columns for a `Rename` event.
    pub fn update_rename(
        &self,
        old_id: i64,
        new_id: i64,
        new_path: &str,
        new_name: &str,
        new_name_lower: &str,
        new_ext: Option<&str>,
    ) -> Result<(), IndexError> {
        let conn = self.inner.lock();
        conn.execute(
            "UPDATE files SET file_id=?1, path=?2, name=?3, name_lower=?4, ext=?5 WHERE file_id=?6",
            params![new_id, new_path, new_name, new_name_lower, new_ext, old_id],
        )?;
        Ok(())
    }

    pub fn delete(&self, file_id: i64) -> Result<(), IndexError> {
        let conn = self.inner.lock();
        conn.execute("DELETE FROM files WHERE file_id=?1", params![file_id])?;
        Ok(())
    }

    pub fn count(&self) -> Result<u64, IndexError> {
        let conn = self.inner.lock();
        let n: i64 = conn.query_row("SELECT COUNT(*) FROM files", [], |r| r.get(0))?;
        Ok(n as u64)
    }

    pub fn get(&self, file_id: i64) -> Result<Option<FileRow>, IndexError> {
        let conn = self.inner.lock();
        let mut stmt = conn.prepare(
            "SELECT file_id, path, name, name_lower, ext, size, mtime_ns, ctime_ns, attrs, volume
             FROM files WHERE file_id=?1",
        )?;
        let mut rows = stmt.query(params![file_id])?;
        if let Some(r) = rows.next()? {
            let path_str: String = r.get(1)?;
            let size_i: i64 = r.get(5)?;
            let attrs_i: i64 = r.get(8)?;
            Ok(Some(FileRow {
                file_id: r.get(0)?,
                path: PathBuf::from(path_str),
                name: r.get(2)?,
                name_lower: r.get(3)?,
                ext: r.get(4)?,
                size: size_i.max(0) as u64,
                mtime_ns: r.get(6)?,
                ctime_ns: r.get(7)?,
                attrs: attrs_i.max(0) as u64,
                volume: r.get(9)?,
            }))
        } else {
            Ok(None)
        }
    }

    /// Force a checkpoint of the WAL into the main DB. Used after a
    /// successful Tantivy commit so a crash between commits leaves the
    /// canonical store ahead of (never behind) the search index.
    pub fn checkpoint(&self) -> Result<(), IndexError> {
        let conn = self.inner.lock();
        conn.execute_batch("PRAGMA wal_checkpoint(TRUNCATE);")?;
        Ok(())
    }

    /// Phase-5 batch hydration: fetch FileRows for a list of
    /// `file_id`s in one round-trip per ~250 ids. The caller is the
    /// query executor; per-row `get()` would burn the 16ms budget on
    /// large candidate sets. Order of returned rows is unspecified —
    /// the caller sorts by their `SortSpec`.
    pub fn get_many(&self, file_ids: &[i64]) -> Result<Vec<FileRow>, IndexError> {
        if file_ids.is_empty() {
            return Ok(Vec::new());
        }
        let conn = self.inner.lock();
        let mut out = Vec::with_capacity(file_ids.len());
        // SQLite's default SQLITE_MAX_VARIABLE_NUMBER is 999 (older) /
        // 32766 (newer). Stay well under the conservative bound.
        for chunk in file_ids.chunks(250) {
            let placeholders = std::iter::repeat_n("?", chunk.len())
                .collect::<Vec<_>>()
                .join(",");
            let sql = format!(
                "SELECT file_id, path, name, name_lower, ext, size, mtime_ns, ctime_ns, attrs, volume \
                 FROM files WHERE file_id IN ({placeholders})"
            );
            let mut stmt = conn.prepare(&sql)?;
            let params = rusqlite::params_from_iter(chunk.iter());
            let mut rows = stmt.query(params)?;
            while let Some(r) = rows.next()? {
                let path_str: String = r.get(1)?;
                let size_i: i64 = r.get(5)?;
                let attrs_i: i64 = r.get(8)?;
                out.push(FileRow {
                    file_id: r.get(0)?,
                    path: PathBuf::from(path_str),
                    name: r.get(2)?,
                    name_lower: r.get(3)?,
                    ext: r.get(4)?,
                    size: size_i.max(0) as u64,
                    mtime_ns: r.get(6)?,
                    ctime_ns: r.get(7)?,
                    attrs: attrs_i.max(0) as u64,
                    volume: r.get(9)?,
                });
            }
        }
        Ok(out)
    }

    /// Scan all rows — used by recovery to rebuild the in-memory name
    /// index after a crash.
    pub fn iter_all<F>(&self, mut f: F) -> Result<(), IndexError>
    where
        F: FnMut(FileRow),
    {
        let conn = self.inner.lock();
        let mut stmt = conn.prepare(
            "SELECT file_id, path, name, name_lower, ext, size, mtime_ns, ctime_ns, attrs, volume
             FROM files",
        )?;
        let mut rows = stmt.query([])?;
        while let Some(r) = rows.next()? {
            let path_str: String = r.get(1)?;
            let size_i: i64 = r.get(5)?;
            let attrs_i: i64 = r.get(8)?;
            f(FileRow {
                file_id: r.get(0)?,
                path: PathBuf::from(path_str),
                name: r.get(2)?,
                name_lower: r.get(3)?,
                ext: r.get(4)?,
                size: size_i.max(0) as u64,
                mtime_ns: r.get(6)?,
                ctime_ns: r.get(7)?,
                attrs: attrs_i.max(0) as u64,
                volume: r.get(9)?,
            });
        }
        Ok(())
    }
}
