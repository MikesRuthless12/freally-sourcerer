//! Cross-crate table lockstep.
//!
//! `freally-rpc` is the transport layer and deliberately depends on
//! neither `freally-query` nor `freally-index`, so a couple of small
//! tables are duplicated there rather than imported. `freally-indexd`
//! depends on all three, which makes it the one place that can assert
//! the copies still agree — a drifted table is otherwise invisible
//! until a user notices the wrong files in an export.

use freally_query::QuickFilter;
use freally_rpc::filelist::AUDIO_EXTENSIONS;

#[test]
fn audio_extensions_match_the_quick_filter() {
    let mut from_rpc: Vec<&str> = AUDIO_EXTENSIONS.to_vec();
    let mut from_query: Vec<&str> = QuickFilter::Audio.extensions().to_vec();
    from_rpc.sort_unstable();
    from_query.sort_unstable();
    assert_eq!(
        from_rpc, from_query,
        "freally_rpc::filelist::AUDIO_EXTENSIONS has drifted from \
         QuickFilter::Audio — an M3U export would carry a different set \
         of files than the Audio quick filter shows"
    );
}

#[test]
fn the_directory_attribute_bit_agrees_across_crates() {
    // `freally_index::ATTR_DIRECTORY` is the canonical home; the query
    // crate spells it as an `AttribFlag`, and `freally-rpc` keeps its
    // own `u32` copy for the `.efu` attribute column.
    assert_eq!(
        freally_query::AttribFlag::Directory.bit(),
        freally_index::ATTR_DIRECTORY
    );
}
