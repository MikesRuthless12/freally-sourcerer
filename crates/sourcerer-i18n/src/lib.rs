//! Sourcerer Fluent i18n loader.
//!
//! Phase 0: scaffold. Loader lands in Phase 12.

/// The 18 locale codes Sourcerer ships in v0.19.84.
pub const LOCALES: &[&str] = &[
    "en", "es", "zh-CN", "hi", "ar", "pt-BR", "ru", "ja", "de", "fr", "ko", "it", "tr", "vi", "pl",
    "nl", "id", "uk",
];

pub fn placeholder() -> &'static str {
    "sourcerer-i18n Phase 0 scaffold"
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ships_eighteen_locales() {
        assert_eq!(LOCALES.len(), 18);
    }

    #[test]
    fn english_is_first() {
        assert_eq!(LOCALES[0], "en");
    }

    #[test]
    fn arabic_is_present() {
        assert!(LOCALES.contains(&"ar"));
    }
}
