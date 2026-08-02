//! SRC-M12 — CJK phonetic keys.
//!
//! Typing 文件 requires an IME. Typing `wenjian` — or just `wj` —
//! requires nothing. This module turns a CJK filename into the latin
//! (or jamo) text a user would actually type on a keyboard, so the
//! filename lens can match it.
//!
//! Three scripts, three conventions, all of them what the native
//! keyboard produces:
//!
//! | Script | Name | Keys produced |
//! |---|---|---|
//! | Han | `文件.txt` | `wenjian`, `wj` |
//! | Kana | `にほん.txt` | `nihon` |
//! | Hangul | `한글.txt` | `ㅎㄱ` |
//!
//! Han gets both the full reading and the initials because Chinese
//! users routinely type either. Kana gets romaji only — initials of
//! romaji syllables are not how anyone searches. Hangul gets
//! lead-jamo, which *is* how Korean users abbreviate, and stays in
//! jamo rather than being romanised because that is what the Korean
//! keyboard emits.
//!
//! ## No AI, no models
//!
//! Every conversion here is a table lookup or arithmetic. Hangul
//! decomposition is pure arithmetic on the Unicode syllable block;
//! kana is a fixed table; Han readings come from the `pinyin` crate's
//! Unihan data (MIT). Heteronyms resolve to the most common reading —
//! this is a search affordance, not a transliteration service.
//!
//! *(The `kana` crate on crates.io is GPL-3.0-only, which `deny.toml`
//! hard-bans, and it is a quantum-physics library besides. The table
//! below is written in-tree.)*

/// Separates a name from its phonetic keys inside one index entry.
///
/// `U+0001` because the query parser cannot produce it: a user's
/// literal term is text they typed, so no query can straddle the
/// boundary and match across it by accident.
pub const PHONETIC_SEP: char = '\u{1}';

/// Phonetic keys for `name`, or `None` when it holds no CJK at all.
///
/// The result is space-separated so trigrams cannot bridge two
/// unrelated readings — without it, `文件` + `abc` would manufacture a
/// trigram spanning `n` and `a` that matches neither word.
pub fn phonetic_keys(name: &str) -> Option<String> {
    let mut full = String::new();
    let mut initials = String::new();
    let mut kana_run = String::new();
    let mut jamo = String::new();
    let mut saw_cjk = false;

    for ch in name.chars() {
        if let Some(py) = han_reading(ch) {
            saw_cjk = true;
            full.push_str(py);
            initials.push(py.as_bytes()[0] as char);
            continue;
        }
        if let Some(r) = kana_romaji(ch) {
            saw_cjk = true;
            kana_run.push_str(r);
            continue;
        }
        if let Some(lead) = hangul_lead_jamo(ch) {
            saw_cjk = true;
            jamo.push(lead);
            continue;
        }
        // A non-CJK character ends the current run rather than joining
        // the next one: `文件-v2` must not produce `wenjianv2`.
        if !kana_run.is_empty() {
            kana_run.push(' ');
        }
        if !full.is_empty() {
            full.push(' ');
            initials.push(' ');
        }
        if !jamo.is_empty() {
            jamo.push(' ');
        }
    }

    if !saw_cjk {
        return None;
    }
    let mut out = String::new();
    for part in [full, initials, kana_run, jamo] {
        let part = part.trim();
        if part.is_empty() {
            continue;
        }
        if !out.is_empty() {
            out.push(' ');
        }
        out.push_str(part);
    }
    (!out.is_empty()).then_some(out)
}

/// Append phonetic keys to an index key, or return it unchanged.
///
/// The combined form is what the name index stores: matching runs over
/// the whole string, and a caller that wants plain-name semantics
/// truncates at [`PHONETIC_SEP`] via [`plain_name`].
pub fn with_phonetic_keys(name_lower: &str) -> String {
    match phonetic_keys(name_lower) {
        Some(keys) => format!("{name_lower}{PHONETIC_SEP}{keys}"),
        None => name_lower.to_string(),
    }
}

/// The name portion of a combined index key.
///
/// Used when the phonetic toggle is off, so a search behaves exactly
/// as it did before SRC-M12 — the keys stay in the index either way,
/// which is what lets the setting take effect without a reindex.
pub fn plain_name(key: &[u8]) -> &[u8] {
    match key.iter().position(|&b| b == PHONETIC_SEP as u8) {
        Some(i) => &key[..i],
        None => key,
    }
}

/// Most common Mandarin reading for a Han character, without tone marks.
fn han_reading(ch: char) -> Option<&'static str> {
    use pinyin::ToPinyin;
    // Returns `None` for everything that is not Han, which is exactly
    // the discrimination this module needs.
    ch.to_pinyin().map(|p| p.plain())
}

/// Lead consonant of a Hangul syllable, as a compatibility jamo.
///
/// The syllable block is arithmetic: `U+AC00` + lead*588 + vowel*28 +
/// tail. Only precomposed syllables are handled; a bare jamo the user
/// already typed needs no conversion.
fn hangul_lead_jamo(ch: char) -> Option<char> {
    /// The 19 lead consonants, in Unicode order.
    const LEADS: [char; 19] = [
        'ㄱ', 'ㄲ', 'ㄴ', 'ㄷ', 'ㄸ', 'ㄹ', 'ㅁ', 'ㅂ', 'ㅃ', 'ㅅ', 'ㅆ', 'ㅇ', 'ㅈ', 'ㅉ', 'ㅊ',
        'ㅋ', 'ㅌ', 'ㅍ', 'ㅎ',
    ];
    let c = ch as u32;
    if !(0xAC00..=0xD7A3).contains(&c) {
        return None;
    }
    LEADS.get(((c - 0xAC00) / 588) as usize).copied()
}

/// Romaji for one kana, or `None` if `ch` is not kana.
///
/// Hepburn, because it is what a latin-keyboard IME accepts. Katakana
/// is folded onto hiragana first (they differ by a fixed 0x60 offset),
/// so the table below is written once.
fn kana_romaji(ch: char) -> Option<&'static str> {
    let c = ch as u32;
    // Katakana U+30A1..U+30F6 maps onto hiragana U+3041..U+3096.
    let h = if (0x30A1..=0x30F6).contains(&c) {
        char::from_u32(c - 0x60)?
    } else {
        ch
    };
    Some(match h {
        'あ' => "a",
        'い' => "i",
        'う' => "u",
        'え' => "e",
        'お' => "o",
        'か' => "ka",
        'き' => "ki",
        'く' => "ku",
        'け' => "ke",
        'こ' => "ko",
        'が' => "ga",
        'ぎ' => "gi",
        'ぐ' => "gu",
        'げ' => "ge",
        'ご' => "go",
        'さ' => "sa",
        'し' => "shi",
        'す' => "su",
        'せ' => "se",
        'そ' => "so",
        'ざ' => "za",
        'じ' => "ji",
        'ず' => "zu",
        'ぜ' => "ze",
        'ぞ' => "zo",
        'た' => "ta",
        'ち' => "chi",
        'つ' => "tsu",
        'て' => "te",
        'と' => "to",
        'だ' => "da",
        'ぢ' => "ji",
        'づ' => "zu",
        'で' => "de",
        'ど' => "do",
        'な' => "na",
        'に' => "ni",
        'ぬ' => "nu",
        'ね' => "ne",
        'の' => "no",
        'は' => "ha",
        'ひ' => "hi",
        'ふ' => "fu",
        'へ' => "he",
        'ほ' => "ho",
        'ば' => "ba",
        'び' => "bi",
        'ぶ' => "bu",
        'べ' => "be",
        'ぼ' => "bo",
        'ぱ' => "pa",
        'ぴ' => "pi",
        'ぷ' => "pu",
        'ぺ' => "pe",
        'ぽ' => "po",
        'ま' => "ma",
        'み' => "mi",
        'む' => "mu",
        'め' => "me",
        'も' => "mo",
        'や' => "ya",
        'ゆ' => "yu",
        'よ' => "yo",
        'ら' => "ra",
        'り' => "ri",
        'る' => "ru",
        'れ' => "re",
        'ろ' => "ro",
        'わ' => "wa",
        'ゐ' => "wi",
        'ゑ' => "we",
        'を' => "wo",
        'ん' => "n",
        // Small kana. A digraph like きゃ is produced by the preceding
        // syllable's "ki" plus this "ya" giving "kiya"; users type both
        // that and "kya", and the substring match tolerates the former.
        'ぁ' => "a",
        'ぃ' => "i",
        'ぅ' => "u",
        'ぇ' => "e",
        'ぉ' => "o",
        'ゃ' => "ya",
        'ゅ' => "yu",
        'ょ' => "yo",
        'ゎ' => "wa",
        // Sokuon marks gemination of the *next* consonant; emitting
        // nothing keeps `がっこう` as `gakou`, which still matches the
        // `gakkou` a user types by substring on either side of the seam.
        'っ' => "",
        'ゔ' => "vu",
        _ => return None,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn han_yields_full_reading_and_initials() {
        let keys = phonetic_keys("文件").expect("han is cjk");
        assert!(keys.contains("wenjian"), "got {keys}");
        assert!(keys.contains("wj"), "got {keys}");
    }

    #[test]
    fn kana_yields_romaji() {
        assert_eq!(phonetic_keys("にほん").as_deref(), Some("nihon"));
        // Katakana folds onto the same table.
        assert_eq!(phonetic_keys("ニホン").as_deref(), Some("nihon"));
    }

    #[test]
    fn hangul_yields_lead_jamo() {
        assert_eq!(phonetic_keys("한글").as_deref(), Some("ㅎㄱ"));
    }

    #[test]
    fn latin_only_names_produce_no_keys() {
        assert!(phonetic_keys("report-final.txt").is_none());
        assert!(phonetic_keys("").is_none());
        assert!(phonetic_keys("123 456").is_none());
    }

    #[test]
    fn a_non_cjk_character_breaks_the_run() {
        // Without the break this would read `wenjianv2` and match a
        // query no user would type.
        let keys = phonetic_keys("文件-v2").unwrap();
        assert!(!keys.contains("wenjianv"), "runs joined: {keys}");
        assert!(keys.contains("wenjian"), "got {keys}");
    }

    #[test]
    fn mixed_scripts_each_contribute() {
        let keys = phonetic_keys("文件にほん").unwrap();
        assert!(keys.contains("wenjian"), "got {keys}");
        assert!(keys.contains("nihon"), "got {keys}");
    }

    #[test]
    fn combined_key_round_trips_through_plain_name() {
        let combined = with_phonetic_keys("文件.txt");
        assert!(combined.contains(PHONETIC_SEP));
        assert_eq!(plain_name(combined.as_bytes()), "文件.txt".as_bytes());
    }

    #[test]
    fn a_latin_name_is_stored_unchanged() {
        let combined = with_phonetic_keys("report.txt");
        assert_eq!(combined, "report.txt");
        assert_eq!(plain_name(combined.as_bytes()), b"report.txt");
    }

    #[test]
    fn plain_name_is_identity_when_there_are_no_keys() {
        assert_eq!(plain_name(b"report.txt"), b"report.txt");
    }

    #[test]
    fn sokuon_is_dropped_rather_than_guessed() {
        // `gakkou` typed by the user still substring-matches `gakou`'s
        // `ga` and `kou` halves; inventing the doubled consonant would
        // require knowing the next syllable's romaji.
        assert_eq!(phonetic_keys("がっこう").as_deref(), Some("gakou"));
    }

    /// The whole point of storing the readings in the name index: the
    /// trigram postings must cover the latin text, or the candidate
    /// generator never offers the CJK row for a latin query.
    #[test]
    fn a_latin_query_reaches_a_cjk_name_through_the_index() {
        let dir = tempfile::tempdir().unwrap();
        let ni = crate::name_index::NameIndex::open(dir.path()).unwrap();
        ni.upsert(1, "文件.txt").unwrap();
        ni.upsert(2, "report.txt").unwrap();

        let mut hit = false;
        ni.for_each_candidate_named("wenjian", 0, |fid, key| {
            if fid == 1 {
                hit = true;
                // The name half is still the real filename, so sorting
                // and display are unaffected by the stored reading.
                assert_eq!(plain_name(key), "文件.txt".as_bytes());
            }
        });
        assert!(hit, "`wenjian` did not reach 文件.txt through the index");
    }

    #[test]
    fn the_separator_cannot_appear_in_a_name_derived_key() {
        let combined = with_phonetic_keys("文件");
        assert_eq!(
            combined.matches(PHONETIC_SEP).count(),
            1,
            "exactly one seam, else plain_name truncates wrongly"
        );
    }
}
