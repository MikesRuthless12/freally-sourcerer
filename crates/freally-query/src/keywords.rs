//! The DSL's advertised vocabulary — the keyword lists anything that
//! *suggests* query text reads from.
//!
//! SRC-M10's shell completions are the first consumer. The lists live
//! here rather than inside the completion generator so there is exactly
//! one place to update when a modifier lands, and so a test in this
//! crate can hold them against the parser itself
//! ([`every_advertised_modifier_is_known_to_the_parser`]).
//!
//! Two deliberate exclusions:
//!
//! - **Aliases.** `attrib` is listed; `attr` and `attributes` are not.
//!   Offering three spellings of one modifier makes a completion menu
//!   worse, not better.
//! - **Reserved-but-unimplemented keys.** `content:`, `count:`,
//!   `lang:`, `type:`, `channels:`, `wfn:`, `case:` and
//!   `nodiacritics:` parse (Standing Rule #8) but fail at execute with
//!   `UnsupportedModifier`. Completing them would walk the user into
//!   an error, so they stay out until their owning phase ships.

/// Canonical, executable modifier keys. Written without the trailing
/// `:` so a caller can decide how to present them.
pub const MODIFIER_KEYS: &[&str] = &[
    "attrib",
    "child",
    "child-count",
    "codec",
    "date",
    "descendant-count",
    "dr",
    "dupe",
    "empty",
    "ext",
    "length",
    "lufs",
    "name",
    "name-dupe",
    "parent",
    "path",
    "rate",
    "silence",
    "similar",
    "size",
    "size-dupe",
];

/// Quick-filter aliases, usable bare (`audio:`) or as a lens prefix.
pub const QUICK_FILTER_KEYS: &[&str] = &[
    "archive",
    "audio",
    "document",
    "executable",
    "image",
    "video",
];

#[cfg(test)]
mod tests {
    use super::*;
    use crate::error::ParseError;
    use crate::parser::parse;

    /// The whole point of this module: if a modifier is renamed or
    /// dropped in the parser, the advertised list must not keep
    /// offering it. `UnknownModifier` is the exact failure that means
    /// "this list has drifted".
    #[test]
    fn every_advertised_modifier_is_known_to_the_parser() {
        for key in MODIFIER_KEYS {
            // A bare `key:` may legitimately be an invalid *value* —
            // `lufs:` needs a comparator. Only `UnknownModifier` means
            // the key itself is gone.
            let source = format!("{key}:");
            if let Err(ParseError::UnknownModifier { name, .. }) = parse(&source) {
                panic!("advertised modifier `{name}:` is not known to the parser");
            }
        }
    }

    #[test]
    fn every_advertised_quick_filter_is_known_to_the_parser() {
        for key in QUICK_FILTER_KEYS {
            let source = format!("{key}:");
            if let Err(ParseError::UnknownModifier { name, .. }) = parse(&source) {
                panic!("advertised quick filter `{name}:` is not known to the parser");
            }
        }
    }

    /// SRC-M11 relies on `sole_literal_term` to decide whether there is
    /// anything to correct; these pin the cases where it must decline.
    #[test]
    fn sole_literal_term_finds_the_one_correctable_word() {
        let q = parse("freallly").unwrap();
        assert_eq!(q.sole_literal_term(), Some("freallly"));

        // One literal plus modifiers is still one correctable word.
        let q = parse("freallly ext:txt size:>1mb").unwrap();
        assert_eq!(q.sole_literal_term(), Some("freallly"));
    }

    #[test]
    fn sole_literal_term_declines_when_there_is_nothing_to_correct() {
        // Two literals — no way to know which was mistyped.
        assert_eq!(parse("alpha beta").unwrap().sole_literal_term(), None);
        // A wildcard is a pattern the user meant to write.
        assert_eq!(parse("*.txt").unwrap().sole_literal_term(), None);
        // Modifiers only — an empty result is a real answer.
        assert_eq!(parse("ext:txt").unwrap().sole_literal_term(), None);
        // Correcting an exclusion cannot add results.
        assert_eq!(parse("!freallly").unwrap().sole_literal_term(), None);
    }

    #[test]
    fn lists_are_sorted_and_free_of_duplicates() {
        for list in [MODIFIER_KEYS, QUICK_FILTER_KEYS] {
            let mut sorted = list.to_vec();
            sorted.sort_unstable();
            sorted.dedup();
            assert_eq!(
                sorted.as_slice(),
                list,
                "keyword list must be sorted + unique"
            );
        }
    }
}
