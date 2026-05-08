//! Tantivy schema for `FileRecord` rows (PRD §3, TASK-029).
//!
//! Field roles:
//! * `file_id` — stable u64 derived from blake3(canonical path). Stored +
//!   indexed (FAST + STORED) so we can round-trip from a Tantivy hit
//!   back into the SQLite canonical store.
//! * `name` — filename (raw + lowercase tokenized) for the filename lens
//!   and for trigram pre-filter refinement in Phase 5.
//! * `path` — canonicalized full path, indexed so the `path:` modifier
//!   from Phase 5's DSL has a backing field.
//! * `ext` — lowercased extension; STRING-indexed for exact-match `ext:`
//!   queries.
//! * `size` / `mtime_ns` / `ctime_ns` — numeric FAST fields.
//! * `attrs` — packed `AttrFlags` bitset.
//! * `volume` — volume identifier (lower-case canonical).
//!
//! Phase-7's content lens, Phase-9's audio lens, and Phase-6's
//! similarity lens extend this schema in their own crates; the
//! filename-lens shape lives here so Phase 4 has the canonical core.

use tantivy::schema::{
    FAST, INDEXED, STORED, STRING, Schema, SchemaBuilder, TEXT, TextFieldIndexing, TextOptions,
};

/// Field handles bound to a built [`Schema`]. Owners hold this alongside
/// the schema so callers don't have to look fields up by name on every
/// write.
#[derive(Debug, Clone)]
pub struct Fields {
    pub file_id: tantivy::schema::Field,
    pub name: tantivy::schema::Field,
    pub name_lower: tantivy::schema::Field,
    pub path: tantivy::schema::Field,
    pub ext: tantivy::schema::Field,
    pub size: tantivy::schema::Field,
    pub mtime_ns: tantivy::schema::Field,
    pub ctime_ns: tantivy::schema::Field,
    pub attrs: tantivy::schema::Field,
    pub volume: tantivy::schema::Field,
}

/// Build the Phase-4 Tantivy schema and the matching `Fields` handles.
pub fn build() -> (Schema, Fields) {
    let mut sb: SchemaBuilder = Schema::builder();

    // Stable identifier: STORED so we can recover the FileRecord from a hit;
    // INDEXED so we can delete-by-term during apply().
    let file_id = sb.add_u64_field("file_id", FAST | STORED | INDEXED);

    // Filename — TEXT (default tokenizer) for substring-style queries.
    // `raw` keeps original casing; `lower` is normalised for case-insensitive.
    let name_options = TextOptions::default()
        .set_indexing_options(
            TextFieldIndexing::default()
                .set_tokenizer("default")
                .set_index_option(tantivy::schema::IndexRecordOption::WithFreqsAndPositions),
        )
        .set_stored();
    let name = sb.add_text_field("name", name_options.clone());
    let name_lower = sb.add_text_field("name_lower", name_options);

    let path = sb.add_text_field("path", TEXT | STORED);

    // Extension is exact-match: STRING (no tokenizing) is right here.
    let ext = sb.add_text_field("ext", STRING | STORED);

    let size = sb.add_u64_field("size", FAST | STORED);
    let mtime_ns = sb.add_i64_field("mtime_ns", FAST | STORED);
    let ctime_ns = sb.add_i64_field("ctime_ns", FAST | STORED);
    let attrs = sb.add_u64_field("attrs", FAST | STORED);
    let volume = sb.add_text_field("volume", STRING | STORED);

    let schema = sb.build();
    let fields = Fields {
        file_id,
        name,
        name_lower,
        path,
        ext,
        size,
        mtime_ns,
        ctime_ns,
        attrs,
        volume,
    };
    (schema, fields)
}
