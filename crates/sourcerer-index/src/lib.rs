//! Sourcerer index core (Phase 4).
//!
//! Orchestrates three persistent stores behind one façade:
//!
//!   * **Tantivy** — full-text + faceted search; segment-committed.
//!   * **SQLite** `files.db` — canonical FileRecord store, WAL mode,
//!     authoritative across crash recovery.
//!   * **Custom name index** (`name.idx` + `name.suf`) — trigram +
//!     suffix array, mmap-backed.
//!
//! Public surface (per the Build Guide's Phase 4 prompt):
//!
//! ```ignore
//! let idx = Index::open(root)?;
//! idx.apply(&events)?;
//! idx.commit()?;
//! let hits = idx.candidates_for_substring("report")?;
//! ```
//!
//! The `JournalEvent` type comes from `sourcerer-journal` and is the
//! same shape on every OS.

#![deny(rust_2018_idioms)]

pub mod error;
pub mod location;
pub mod manifest;
pub mod name_index;
pub mod pipeline;
pub mod schema;
pub mod store;

use std::path::{Path, PathBuf};
use std::sync::Arc;

use parking_lot::Mutex;
use sourcerer_journal::JournalEvent;
use tantivy::{
    Term,
    collector::TopDocs,
    doc,
    query::{BooleanQuery, Occur, Query, TermQuery},
    schema::IndexRecordOption,
    schema::Value,
};
use tracing::{debug, warn};

pub use error::IndexError;
pub use location::{default_index_root, service_index_root};
pub use manifest::Manifest;
pub use pipeline::{DEFAULT_CAPACITY, EventQueue};
pub use store::FileRow;

/// Tantivy writer heap. Bumped to 256 MiB so the bulk-bootstrap path
/// can ingest a full NTFS MFT walk (~1M files) without forcing
/// intermediate segment commits — those are the dominant cost in the
/// per-batch commit loop, and one final commit at end-of-bootstrap is
/// what gets the indexer near Everything-tier fps.
const TANTIVY_WRITER_HEAP_BYTES: usize = 256 * 1024 * 1024;

#[derive(Debug, Clone)]
pub struct IndexStats {
    pub files: u64,
    pub manifest_generation: u64,
    pub applied_events: u64,
    pub trigram_buckets: usize,
}

pub struct Index {
    root: PathBuf,
    schema: tantivy::schema::Schema,
    fields: schema::Fields,
    tantivy: tantivy::Index,
    writer: Mutex<tantivy::IndexWriter>,
    reader: tantivy::IndexReader,
    store: store::Store,
    name_index: name_index::NameIndex,
    manifest: Mutex<Manifest>,
}

impl Index {
    /// Open or create an index rooted at `root`. Creates the per-OS
    /// directory tree if needed.
    pub fn open(root: &Path) -> Result<Arc<Self>, IndexError> {
        std::fs::create_dir_all(root).map_err(|e| IndexError::io(root, e))?;
        let tantivy_dir = root.join("index.tantivy");
        std::fs::create_dir_all(&tantivy_dir).map_err(|e| IndexError::io(&tantivy_dir, e))?;
        let extracted_dir = root.join("extracted");
        std::fs::create_dir_all(&extracted_dir).map_err(|e| IndexError::io(&extracted_dir, e))?;

        let (schema, fields) = schema::build();
        let mmap_dir = tantivy::directory::MmapDirectory::open(&tantivy_dir)?;
        let tantivy = tantivy::Index::open_or_create(mmap_dir, schema.clone())?;
        let writer = tantivy.writer(TANTIVY_WRITER_HEAP_BYTES)?;
        let reader = tantivy
            .reader_builder()
            .reload_policy(tantivy::ReloadPolicy::OnCommitWithDelay)
            .try_into()?;

        let store = store::Store::open(&root.join("files.db"))?;
        let name_idx = name_index::NameIndex::open(root)?;

        let manifest = Manifest::load_or_default(root)?;

        let s = Arc::new(Self {
            root: root.to_path_buf(),
            schema,
            fields,
            tantivy,
            writer: Mutex::new(writer),
            reader,
            store,
            name_index: name_idx,
            manifest: Mutex::new(manifest),
        });

        s.recover_if_needed()?;
        Ok(s)
    }

    /// Convenience: open at the per-OS default index root.
    pub fn open_default() -> Result<Arc<Self>, IndexError> {
        let root = default_index_root()?;
        Self::open(&root)
    }

    /// Reconcile in-memory + on-disk state (TASK-034 corruption gate).
    /// SQLite is canonical: if the name index lost rows or tantivy is
    /// behind, we re-derive what's missing. The simplest correct loop
    /// is to walk the canonical store and re-upsert into the name
    /// index — this is also what makes the v0 file format
    /// forward-compatible.
    fn recover_if_needed(&self) -> Result<(), IndexError> {
        let want_rows = self.store.count()?;
        let have_rows = self.name_index.live_row_count() as u64;
        if want_rows == have_rows {
            return Ok(());
        }
        warn!(
            want_rows,
            have_rows, "name-index row count drift; replaying canonical store"
        );
        // Walk the canonical store and re-upsert. The name index
        // tombstones the old row and writes a fresh one — idempotent
        // by file_id.
        self.store.iter_all(|row| {
            if let Err(e) = self.name_index.upsert(row.file_id as u64, &row.name_lower) {
                warn!(?e, file_id = row.file_id, "name-index recovery upsert");
            }
        })?;
        Ok(())
    }

    pub fn root(&self) -> &Path {
        &self.root
    }

    pub fn schema(&self) -> &tantivy::schema::Schema {
        &self.schema
    }

    pub fn fields(&self) -> &schema::Fields {
        &self.fields
    }

    pub fn store(&self) -> &store::Store {
        &self.store
    }

    pub fn name_index(&self) -> &name_index::NameIndex {
        &self.name_index
    }

    pub fn stats(&self) -> Result<IndexStats, IndexError> {
        let m = self.manifest.lock().clone();
        Ok(IndexStats {
            files: self.store.count()?,
            manifest_generation: m.tantivy_generation,
            applied_events: m.applied_events,
            trigram_buckets: self.name_index.trigram_buckets(),
        })
    }

    /// Fast-path apply for the initial MFT/walkdir bootstrap. Only
    /// emits Tantivy `add_document` calls — no `delete_term` (no
    /// duplicates on a fresh scan), no SQLite write, no name_index
    /// write. Caller must finish with [`Index::finalize_bootstrap`]
    /// to rebuild the SQLite + name_index from Tantivy in one bulk
    /// pass. This is ~10× faster than `apply()` for `Create`-heavy
    /// workloads (initial scan of `C:\`).
    pub fn bootstrap_apply(&self, events: &[JournalEvent]) -> Result<(), IndexError> {
        if events.is_empty() {
            return Ok(());
        }
        let writer = self.writer.lock();
        for ev in events {
            if let JournalEvent::Create {
                path,
                size,
                mtime_ns,
                ctime_ns,
                attrs,
            } = ev
            {
                let row = path_to_row(
                    path,
                    *size,
                    clamp_i64(*mtime_ns),
                    clamp_i64(*ctime_ns),
                    *attrs as u64,
                );
                let path_str = row.path.to_string_lossy().into_owned();
                writer.add_document(doc!(
                    self.fields.file_id => row.file_id as u64,
                    self.fields.name => row.name.clone(),
                    self.fields.name_lower => row.name_lower.clone(),
                    self.fields.path => path_str,
                    self.fields.ext => row.ext.clone().unwrap_or_default(),
                    self.fields.size => row.size,
                    self.fields.mtime_ns => row.mtime_ns,
                    self.fields.ctime_ns => row.ctime_ns,
                    self.fields.attrs => row.attrs,
                    self.fields.volume => row.volume.clone(),
                ))?;
            }
            // Bootstrap mode only handles Create; other variants are
            // routed through the regular `apply()` path post-bootstrap.
        }
        let mut m = self.manifest.lock();
        m.applied_events = m.applied_events.saturating_add(events.len() as u64);
        Ok(())
    }

    /// Commit the bootstrap pass: flush Tantivy once, then walk the
    /// committed index and bulk-rebuild SQLite + name_index from it.
    /// One SQLite transaction wraps the whole insert pass — that's
    /// where the speed comes from (per-row autocommit is what made
    /// the legacy path ~7K fps).
    ///
    /// Returns the number of documents rebuilt into the canonical
    /// stores.
    pub fn finalize_bootstrap(&self) -> Result<u64, IndexError> {
        // 1) Single Tantivy commit.
        {
            let mut writer = self.writer.lock();
            writer.commit()?;
        }
        if let Err(e) = self.reader.reload() {
            warn!(?e, "tantivy reader reload (non-fatal)");
        }

        // 2) Walk every committed doc and collect FileRows.
        let searcher = self.reader.searcher();
        let mut rebuilt: u64 = 0;
        let mut batch: Vec<FileRow> = Vec::with_capacity(8192);
        for segment_reader in searcher.segment_readers() {
            let store_reader = segment_reader
                .get_store_reader(0)
                .map_err(|e| IndexError::Tantivy(format!("get_store_reader: {e}")))?;
            let max_doc = segment_reader.max_doc();
            for doc_id in 0..max_doc {
                if segment_reader.is_deleted(doc_id) {
                    continue;
                }
                let doc: tantivy::TantivyDocument = store_reader
                    .get(doc_id)
                    .map_err(|e| IndexError::Tantivy(format!("store_reader.get: {e}")))?;
                if let Some(row) = doc_to_file_row(&doc, &self.fields) {
                    batch.push(row);
                    if batch.len() >= 8192 {
                        self.store.bulk_upsert(&batch)?;
                        for r in &batch {
                            self.name_index.upsert(r.file_id as u64, &r.name_lower)?;
                        }
                        rebuilt += batch.len() as u64;
                        batch.clear();
                    }
                }
            }
        }
        if !batch.is_empty() {
            self.store.bulk_upsert(&batch)?;
            for r in &batch {
                self.name_index.upsert(r.file_id as u64, &r.name_lower)?;
            }
            rebuilt += batch.len() as u64;
        }

        // 3) Flush name_index + SQLite checkpoint, persist manifest.
        self.name_index.flush()?;
        self.store.checkpoint()?;
        let mut m = self.manifest.lock();
        m.tantivy_generation = m.tantivy_generation.saturating_add(1);
        m.save(&self.root)?;
        Ok(rebuilt)
    }

    /// Apply a batch of events. Tantivy mutations and SQLite mutations
    /// are issued sequentially; commit() durably flushes both.
    pub fn apply(&self, events: &[JournalEvent]) -> Result<(), IndexError> {
        if events.is_empty() {
            return Ok(());
        }
        let mut writer = self.writer.lock();
        for ev in events {
            match ev {
                JournalEvent::Create {
                    path,
                    size,
                    mtime_ns,
                    ctime_ns,
                    attrs,
                } => {
                    self.apply_create(&mut writer, path, *size, *mtime_ns, *ctime_ns, *attrs)?;
                }
                JournalEvent::Modify {
                    path,
                    size,
                    mtime_ns,
                    attrs,
                } => {
                    self.apply_modify(&mut writer, path, *size, *mtime_ns, *attrs)?;
                }
                JournalEvent::Delete { path } => {
                    self.apply_delete(&mut writer, path)?;
                }
                JournalEvent::Rename { old_path, new_path } => {
                    self.apply_rename(&mut writer, old_path, new_path)?;
                }
                JournalEvent::AttrChange { path, attrs } => {
                    self.apply_attrchange(&mut writer, path, *attrs)?;
                }
            }
        }
        let mut m = self.manifest.lock();
        m.applied_events = m.applied_events.saturating_add(events.len() as u64);
        Ok(())
    }

    fn apply_create(
        &self,
        writer: &mut tantivy::IndexWriter,
        path: &Path,
        size: u64,
        mtime_ns: i128,
        ctime_ns: i128,
        attrs: u32,
    ) -> Result<(), IndexError> {
        let row = path_to_row(
            path,
            size,
            clamp_i64(mtime_ns),
            clamp_i64(ctime_ns),
            attrs as u64,
        );
        let path_str = row.path.to_string_lossy().into_owned();
        // Tantivy: delete-then-add to keep the index in sync if a Create
        // is replayed during recovery.
        writer.delete_term(Term::from_field_u64(
            self.fields.file_id,
            row.file_id as u64,
        ));
        writer.add_document(doc!(
            self.fields.file_id => row.file_id as u64,
            self.fields.name => row.name.clone(),
            self.fields.name_lower => row.name_lower.clone(),
            self.fields.path => path_str.clone(),
            self.fields.ext => row.ext.clone().unwrap_or_default(),
            self.fields.size => row.size,
            self.fields.mtime_ns => row.mtime_ns,
            self.fields.ctime_ns => row.ctime_ns,
            self.fields.attrs => row.attrs,
            self.fields.volume => row.volume.clone(),
        ))?;
        self.store.upsert(&row)?;
        self.name_index
            .upsert(row.file_id as u64, &row.name_lower)?;
        Ok(())
    }

    fn apply_modify(
        &self,
        writer: &mut tantivy::IndexWriter,
        path: &Path,
        size: u64,
        mtime_ns: i128,
        attrs: u32,
    ) -> Result<(), IndexError> {
        let file_id = derive_file_id(path);
        // Tantivy doesn't have an in-place update — delete-then-add.
        writer.delete_term(Term::from_field_u64(self.fields.file_id, file_id as u64));
        // Re-add with current SQLite-known fields, modulating the
        // mutated columns in.
        if let Some(prev) = self.store.get(file_id)? {
            let row = FileRow {
                size,
                mtime_ns: clamp_i64(mtime_ns),
                attrs: attrs as u64,
                ..prev
            };
            writer.add_document(doc!(
                self.fields.file_id => row.file_id as u64,
                self.fields.name => row.name.clone(),
                self.fields.name_lower => row.name_lower.clone(),
                self.fields.path => row.path.to_string_lossy().into_owned(),
                self.fields.ext => row.ext.clone().unwrap_or_default(),
                self.fields.size => row.size,
                self.fields.mtime_ns => row.mtime_ns,
                self.fields.ctime_ns => row.ctime_ns,
                self.fields.attrs => row.attrs,
                self.fields.volume => row.volume.clone(),
            ))?;
            self.store
                .update_modify(row.file_id, row.size, row.mtime_ns, row.attrs)?;
        } else {
            // Modify before Create — synthesize a Create. Only happens
            // when the index opened mid-stream.
            self.apply_create(writer, path, size, mtime_ns, 0, attrs)?;
        }
        Ok(())
    }

    fn apply_delete(
        &self,
        writer: &mut tantivy::IndexWriter,
        path: &Path,
    ) -> Result<(), IndexError> {
        let file_id = derive_file_id(path);
        writer.delete_term(Term::from_field_u64(self.fields.file_id, file_id as u64));
        self.store.delete(file_id)?;
        self.name_index.remove(file_id as u64)?;
        Ok(())
    }

    fn apply_rename(
        &self,
        writer: &mut tantivy::IndexWriter,
        old_path: &Path,
        new_path: &Path,
    ) -> Result<(), IndexError> {
        let old_id = derive_file_id(old_path);
        let new_id = derive_file_id(new_path);
        let Some(prev) = self.store.get(old_id)? else {
            // Rename-without-prior-Create: the journal subscribers' own
            // contract degrades cross-batch rename pairs to Delete +
            // Create (see CHANGELOG.md "Phase 3 Linux journal" entry).
            // Mirror that here so the canonical store, Tantivy index,
            // and name index stay coherent — never write a Tantivy or
            // name-index row that has no `files.db` row of truth.
            self.apply_delete(writer, old_path)?;
            return self.apply_create(writer, new_path, 0, 0, 0, 0);
        };
        writer.delete_term(Term::from_field_u64(self.fields.file_id, old_id as u64));
        let new_path_str = new_path.to_string_lossy().into_owned();
        let new_name = file_name_str(new_path);
        let new_lower = new_name.to_lowercase();
        let new_ext = file_ext_lower(new_path);
        self.store.update_rename(
            old_id,
            new_id,
            &new_path_str,
            &new_name,
            &new_lower,
            new_ext.as_deref(),
        )?;
        let new_row = FileRow {
            file_id: new_id,
            path: new_path.to_path_buf(),
            name: new_name,
            name_lower: new_lower,
            ext: new_ext,
            ..prev
        };
        // Mirror the change into Tantivy.
        writer.add_document(doc!(
            self.fields.file_id => new_row.file_id as u64,
            self.fields.name => new_row.name.clone(),
            self.fields.name_lower => new_row.name_lower.clone(),
            self.fields.path => new_row.path.to_string_lossy().into_owned(),
            self.fields.ext => new_row.ext.clone().unwrap_or_default(),
            self.fields.size => new_row.size,
            self.fields.mtime_ns => new_row.mtime_ns,
            self.fields.ctime_ns => new_row.ctime_ns,
            self.fields.attrs => new_row.attrs,
            self.fields.volume => new_row.volume.clone(),
        ))?;
        // Update the name index: remove old file_id, upsert under new.
        self.name_index.remove(old_id as u64)?;
        self.name_index
            .upsert(new_row.file_id as u64, &new_row.name_lower)?;
        Ok(())
    }

    fn apply_attrchange(
        &self,
        writer: &mut tantivy::IndexWriter,
        path: &Path,
        attrs: u32,
    ) -> Result<(), IndexError> {
        let file_id = derive_file_id(path);
        if let Some(prev) = self.store.get(file_id)? {
            writer.delete_term(Term::from_field_u64(self.fields.file_id, file_id as u64));
            let row = FileRow {
                attrs: attrs as u64,
                ..prev
            };
            writer.add_document(doc!(
                self.fields.file_id => row.file_id as u64,
                self.fields.name => row.name.clone(),
                self.fields.name_lower => row.name_lower.clone(),
                self.fields.path => row.path.to_string_lossy().into_owned(),
                self.fields.ext => row.ext.clone().unwrap_or_default(),
                self.fields.size => row.size,
                self.fields.mtime_ns => row.mtime_ns,
                self.fields.ctime_ns => row.ctime_ns,
                self.fields.attrs => row.attrs,
                self.fields.volume => row.volume.clone(),
            ))?;
            self.store.update_attrs(file_id, attrs as u64)?;
        }
        Ok(())
    }

    /// Atomically commit Tantivy + flush the name index + checkpoint
    /// SQLite WAL + persist the manifest. Order matters for crash
    /// recovery: SQLite-checkpoint last so it's the *only* store that
    /// can be ahead of the manifest, never behind.
    pub fn commit(&self) -> Result<(), IndexError> {
        let mut writer = self.writer.lock();
        writer.commit()?;
        self.name_index.flush()?;
        self.store.checkpoint()?;
        let mut m = self.manifest.lock();
        m.tantivy_generation = m.tantivy_generation.saturating_add(1);
        m.save(&self.root)?;
        // Reload the reader so subsequent search() calls see the new
        // segment generation.
        if let Err(e) = self.reader.reload() {
            debug!(?e, "tantivy reader reload (non-fatal)");
        }
        Ok(())
    }

    /// Bookkeeping for a per-volume cursor — recorded into the manifest
    /// the next time `commit()` runs.
    pub fn record_cursor(&self, volume: &str, cursor: &str) {
        let mut m = self.manifest.lock();
        m.volume_cursors
            .insert(volume.to_string(), cursor.to_string());
    }

    /// Convenience search used by Phase 4's smoke test. Phase 5 layers
    /// the full DSL on top.
    pub fn search_name(&self, q: &str, limit: usize) -> Result<Vec<u64>, IndexError> {
        let searcher = self.reader.searcher();
        // Filename queries are membership-style — build a `BooleanQuery::Should`
        // over individual tokens rather than going through `QueryParser`, which
        // synthesises phrase queries for hyphenated input ("mid-stream" →
        // phrase "mid stream"). The schema indexes `name_lower` with the
        // `Basic` IndexRecordOption (no positions) for ingest perf, so any
        // phrase query throws `field 'name_lower' does not have positions
        // indexed`. Tokenising the query the same way as the field and
        // OR-ing the term queries gives us substring-style matches with
        // zero position-table lookups.
        let mut tokenizer = self.tantivy.tokenizer_for_field(self.fields.name_lower)?;
        let normalized = q.to_lowercase();
        let mut stream = tokenizer.token_stream(&normalized);
        let mut clauses: Vec<(Occur, Box<dyn Query>)> = Vec::new();
        while let Some(token) = stream.next() {
            let term = Term::from_field_text(self.fields.name_lower, &token.text);
            clauses.push((
                Occur::Should,
                Box::new(TermQuery::new(term, IndexRecordOption::Basic)),
            ));
        }
        if clauses.is_empty() {
            return Ok(Vec::new());
        }
        let query = BooleanQuery::new(clauses);
        let top: Vec<(tantivy::Score, tantivy::DocAddress)> =
            searcher.search(&query, &TopDocs::with_limit(limit).order_by_score())?;
        let mut out = Vec::with_capacity(top.len());
        for (_, addr) in top {
            let doc: tantivy::TantivyDocument = searcher.doc(addr)?;
            if let Some(v) = doc.get_first(self.fields.file_id)
                && let Some(id) = v.as_u64()
            {
                out.push(id);
            }
        }
        Ok(out)
    }

    /// Look up a file_id directly (Tantivy term query — exercise the
    /// index path the smoke test relies on).
    pub fn search_by_file_id(&self, file_id: u64) -> Result<Option<u64>, IndexError> {
        let searcher = self.reader.searcher();
        let term = Term::from_field_u64(self.fields.file_id, file_id);
        let query = TermQuery::new(term, IndexRecordOption::Basic);
        let top: Vec<(tantivy::Score, tantivy::DocAddress)> =
            searcher.search(&query, &TopDocs::with_limit(1).order_by_score())?;
        let Some((_, addr)) = top.into_iter().next() else {
            return Ok(None);
        };
        let doc: tantivy::TantivyDocument = searcher.doc(addr)?;
        Ok(doc.get_first(self.fields.file_id).and_then(|v| v.as_u64()))
    }
}

fn file_name_str(path: &Path) -> String {
    path.file_name()
        .and_then(|s| s.to_str())
        .map(|s| s.to_string())
        .unwrap_or_else(|| path.to_string_lossy().into_owned())
}

fn file_ext_lower(path: &Path) -> Option<String> {
    path.extension()
        .and_then(|s| s.to_str())
        .map(|s| s.to_ascii_lowercase())
}

fn derive_file_id(path: &Path) -> i64 {
    // Stable 64-bit identifier derived from the first 8 bytes of
    // `blake3(OsStr::as_encoded_bytes())`. We hash the OS-string bytes
    // directly (not `to_string_lossy()`) so two paths that differ only
    // in invalid-UTF-8 byte sequences don't collapse to the same id —
    // rare in practice but possible on Linux ext4 / Btrfs where
    // filenames are arbitrary byte sequences.
    //
    // Truncating blake3's 256-bit output to 64 bits keeps the SQLite
    // INTEGER column tight, but the birthday bound is ~2^32 paths.
    // 5M files (the perf-target dataset) sit ~600× below that — Phase
    // 13 widens to a full 128-bit ULID once the perf pass runs.
    let bytes = path.as_os_str().as_encoded_bytes();
    let h = blake3::hash(bytes);
    let head: [u8; 8] = h.as_bytes()[..8]
        .try_into()
        .expect("blake3 output is 32 bytes");
    i64::from_le_bytes(head)
}

fn clamp_i64(v: i128) -> i64 {
    v.clamp(i128::from(i64::MIN), i128::from(i64::MAX)) as i64
}

/// Reconstruct a `FileRow` from a fully-stored Tantivy doc. Used by
/// `Index::finalize_bootstrap` to rebuild the SQLite + name_index
/// after the Tantivy-only fast scan. Returns `None` if any required
/// field is missing.
fn doc_to_file_row(doc: &tantivy::TantivyDocument, fields: &schema::Fields) -> Option<FileRow> {
    use tantivy::schema::Value;
    let file_id = doc.get_first(fields.file_id)?.as_u64()? as i64;
    let name = doc.get_first(fields.name)?.as_str()?.to_string();
    let name_lower = doc.get_first(fields.name_lower)?.as_str()?.to_string();
    let path = doc.get_first(fields.path)?.as_str()?.to_string();
    let ext_str = doc.get_first(fields.ext).and_then(|v| v.as_str());
    let ext = ext_str.filter(|s| !s.is_empty()).map(|s| s.to_string());
    let size = doc.get_first(fields.size)?.as_u64()?;
    let mtime_ns = doc.get_first(fields.mtime_ns)?.as_i64()?;
    let ctime_ns = doc.get_first(fields.ctime_ns)?.as_i64()?;
    let attrs = doc.get_first(fields.attrs)?.as_u64()?;
    let volume = doc
        .get_first(fields.volume)
        .and_then(|v| v.as_str())
        .unwrap_or("")
        .to_string();
    Some(FileRow {
        file_id,
        path: PathBuf::from(path),
        name,
        name_lower,
        ext,
        size,
        mtime_ns,
        ctime_ns,
        attrs,
        volume,
    })
}

fn path_to_row(path: &Path, size: u64, mtime_ns: i64, ctime_ns: i64, attrs: u64) -> FileRow {
    let name = file_name_str(path);
    let lower = name.to_lowercase();
    let ext = file_ext_lower(path);
    FileRow {
        file_id: derive_file_id(path),
        path: path.to_path_buf(),
        name,
        name_lower: lower,
        ext,
        size,
        mtime_ns,
        ctime_ns,
        attrs,
        volume: String::new(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::tempdir;

    #[test]
    fn open_creates_directory_layout() {
        let dir = tempdir().unwrap();
        let _idx = Index::open(dir.path()).unwrap();
        assert!(dir.path().join("index.tantivy").exists());
        assert!(dir.path().join("files.db").exists());
        assert!(dir.path().join("extracted").exists());
    }

    #[test]
    fn apply_create_then_query_round_trips() {
        let dir = tempdir().unwrap();
        let idx = Index::open(dir.path()).unwrap();
        let p = PathBuf::from("/tmp/sourcerer-fixture/Report-Final.txt");
        idx.apply(&[JournalEvent::Create {
            path: p.clone(),
            size: 1024,
            mtime_ns: 1_700_000_000_000_000_000,
            ctime_ns: 1_700_000_000_000_000_000,
            attrs: 0,
        }])
        .unwrap();
        idx.commit().unwrap();

        let want = derive_file_id(&p) as u64;
        let hits = idx.search_name("report", 10).unwrap();
        assert!(hits.contains(&want), "expected {want} in hits {hits:?}");

        let cands = idx.name_index().candidates("report");
        assert!(cands.contains(&want), "trigram candidates miss {want}");
    }

    #[test]
    fn delete_removes_from_all_three_stores() {
        let dir = tempdir().unwrap();
        let idx = Index::open(dir.path()).unwrap();
        let p = PathBuf::from("/tmp/will-go-away.bin");
        idx.apply(&[JournalEvent::Create {
            path: p.clone(),
            size: 8,
            mtime_ns: 0,
            ctime_ns: 0,
            attrs: 0,
        }])
        .unwrap();
        idx.commit().unwrap();
        assert_eq!(idx.store().count().unwrap(), 1);

        idx.apply(&[JournalEvent::Delete { path: p.clone() }])
            .unwrap();
        idx.commit().unwrap();
        assert_eq!(idx.store().count().unwrap(), 0);
        let want = derive_file_id(&p) as u64;
        assert!(!idx.name_index().candidates("will").contains(&want));
    }

    #[test]
    fn rename_rewrites_both_indexes() {
        let dir = tempdir().unwrap();
        let idx = Index::open(dir.path()).unwrap();
        let old_p = PathBuf::from("/tmp/draft-alpha.md");
        let new_p = PathBuf::from("/tmp/draft-beta.md");
        idx.apply(&[JournalEvent::Create {
            path: old_p.clone(),
            size: 4,
            mtime_ns: 0,
            ctime_ns: 0,
            attrs: 0,
        }])
        .unwrap();
        idx.apply(&[JournalEvent::Rename {
            old_path: old_p.clone(),
            new_path: new_p.clone(),
        }])
        .unwrap();
        idx.commit().unwrap();
        let new_id = derive_file_id(&new_p) as u64;
        assert!(idx.name_index().candidates("beta").contains(&new_id));
        let old_id = derive_file_id(&old_p) as u64;
        assert!(!idx.name_index().candidates("alpha").contains(&old_id));
    }

    #[test]
    fn reopen_after_drop_recovers_name_index_from_sqlite() {
        let dir = tempdir().unwrap();
        let p = PathBuf::from("/tmp/recovery-canary.dat");
        {
            let idx = Index::open(dir.path()).unwrap();
            idx.apply(&[JournalEvent::Create {
                path: p.clone(),
                size: 1,
                mtime_ns: 0,
                ctime_ns: 0,
                attrs: 0,
            }])
            .unwrap();
            idx.commit().unwrap();
            // Drop without explicit shutdown — simulates kill -9.
            drop(idx);
        }
        // Tamper: blow away the on-disk name index files. Open() must
        // re-derive from SQLite.
        let _ = std::fs::remove_file(dir.path().join("name.idx"));
        let _ = std::fs::remove_file(dir.path().join("name.suf"));

        let idx2 = Index::open(dir.path()).unwrap();
        let want = derive_file_id(&p) as u64;
        assert!(idx2.name_index().candidates("canary").contains(&want));
    }

    #[test]
    fn back_pressure_surfaces_queue_full() {
        let q = EventQueue::new(2);
        for i in 0..2 {
            q.try_push(JournalEvent::Delete {
                path: PathBuf::from(format!("/tmp/x{i}")),
            })
            .unwrap();
        }
        let err = q
            .try_push(JournalEvent::Delete {
                path: PathBuf::from("/tmp/overflow"),
            })
            .unwrap_err();
        assert!(matches!(err, IndexError::QueueFull { .. }));
    }
}
