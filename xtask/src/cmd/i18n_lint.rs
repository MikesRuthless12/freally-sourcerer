use std::collections::BTreeSet;
use std::fs;
use std::path::PathBuf;

use anyhow::{Context, Result, bail};

use crate::workspace_root;

const SOURCE_LOCALE: &str = "en";
const FILENAME: &str = "sourcerer.ftl";

pub fn run(locales: Option<PathBuf>) -> Result<()> {
    let root = locales.unwrap_or_else(|| workspace_root().join("locales"));

    let source_path = root.join(SOURCE_LOCALE).join(FILENAME);
    let source = fs::read_to_string(&source_path)
        .with_context(|| format!("read source locale {}", source_path.display()))?;
    let source_keys = extract_keys(&source);

    if source_keys.is_empty() {
        bail!("source locale {} has no keys", source_path.display());
    }

    let mut errors: Vec<String> = Vec::new();
    let mut checked = 0usize;

    let entries =
        fs::read_dir(&root).with_context(|| format!("read locales dir {}", root.display()))?;
    for entry in entries {
        let entry = entry?;
        if !entry.file_type()?.is_dir() {
            continue;
        }
        let locale = entry.file_name().to_string_lossy().to_string();
        if locale == SOURCE_LOCALE {
            continue;
        }
        let path = entry.path().join(FILENAME);
        if !path.exists() {
            errors.push(format!("missing {}", path.display()));
            continue;
        }
        let body = fs::read_to_string(&path)?;
        let keys = extract_keys(&body);
        let missing: Vec<_> = source_keys.difference(&keys).cloned().collect();
        let extra: Vec<_> = keys.difference(&source_keys).cloned().collect();
        if !missing.is_empty() {
            errors.push(format!("[{}] missing keys: {}", locale, missing.join(", ")));
        }
        if !extra.is_empty() {
            errors.push(format!("[{}] unknown keys: {}", locale, extra.join(", ")));
        }
        checked += 1;
    }

    if errors.is_empty() {
        println!(
            "i18n-lint OK: {} key(s) across {} non-source locale(s) (source: {})",
            source_keys.len(),
            checked,
            SOURCE_LOCALE
        );
        Ok(())
    } else {
        for e in &errors {
            eprintln!("{e}");
        }
        bail!("i18n-lint failed: {} issue(s)", errors.len());
    }
}

/// Extract Fluent message identifiers (lines starting with `<id> = ...`).
fn extract_keys(body: &str) -> BTreeSet<String> {
    let mut keys = BTreeSet::new();
    for raw in body.lines() {
        let line = raw.trim_end();
        if line.is_empty()
            || line.starts_with('#')
            || line.starts_with(' ')
            || line.starts_with('\t')
        {
            continue;
        }
        if let Some(eq) = line.find('=') {
            let id = line[..eq].trim();
            if is_valid_ident(id) {
                keys.insert(id.to_string());
            }
        }
    }
    keys
}

fn is_valid_ident(s: &str) -> bool {
    let mut chars = s.chars();
    let Some(first) = chars.next() else {
        return false;
    };
    if !(first.is_ascii_alphabetic() || first == '-' || first == '_') {
        return false;
    }
    chars.all(|c| c.is_ascii_alphanumeric() || c == '-' || c == '_')
}
