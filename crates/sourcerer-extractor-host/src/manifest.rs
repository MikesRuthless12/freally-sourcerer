//! TOML manifest schema for custom extractors.

use std::path::Path;

use serde::{Deserialize, Serialize};
use thiserror::Error;

#[derive(Debug, Error)]
pub enum ManifestError {
    #[error("io: {0}")]
    Io(#[from] std::io::Error),

    #[error("toml: {0}")]
    Toml(#[from] toml::de::Error),

    #[error("manifest declares no formats")]
    NoFormats,

    #[error("manifest sidecar `{0}` does not exist")]
    SidecarMissing(String),

    #[error("invalid magic byte spec `{0}`")]
    InvalidMagic(String),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Manifest {
    pub id: String,
    pub display_name: String,
    pub version: String,
    pub formats: Vec<String>,
    #[serde(default)]
    pub magic: Vec<String>,
    pub sidecar: String,
    #[serde(default = "default_time_budget_ms")]
    pub time_budget_ms: u32,
    #[serde(default = "default_memory_budget_mb")]
    pub memory_budget_mb: u32,
}

fn default_time_budget_ms() -> u32 {
    1000
}
fn default_memory_budget_mb() -> u32 {
    64
}

impl Manifest {
    /// Read a `*.toml` manifest from disk + verify the sidecar exists.
    pub fn load(toml_path: &Path) -> Result<Manifest, ManifestError> {
        let raw = std::fs::read_to_string(toml_path)?;
        let mut m: Manifest = toml::from_str(&raw)?;
        // Normalize formats to lowercase.
        for f in m.formats.iter_mut() {
            *f = f.to_ascii_lowercase();
        }
        if m.formats.is_empty() {
            return Err(ManifestError::NoFormats);
        }
        let sidecar_path = toml_path
            .parent()
            .map(|p| p.join(&m.sidecar))
            .unwrap_or_else(|| std::path::PathBuf::from(&m.sidecar));
        if !sidecar_path.exists() {
            return Err(ManifestError::SidecarMissing(m.sidecar.clone()));
        }
        // Validate magic specs are parseable so the registry can reject
        // bad manifests at load time.
        for spec in &m.magic {
            parse_magic(spec)?;
        }
        Ok(m)
    }
}

/// Parse a magic-byte spec like `"0x23 0x20"` or `"PK"` into
/// a byte sequence. Hex bytes only for now (simplest secure default).
pub fn parse_magic(spec: &str) -> Result<Vec<u8>, ManifestError> {
    let mut out = Vec::new();
    for tok in spec.split_whitespace() {
        let s = tok
            .strip_prefix("0x")
            .or_else(|| tok.strip_prefix("0X"))
            .ok_or_else(|| ManifestError::InvalidMagic(spec.into()))?;
        if s.len() != 2 {
            return Err(ManifestError::InvalidMagic(spec.into()));
        }
        let b = u8::from_str_radix(s, 16).map_err(|_| ManifestError::InvalidMagic(spec.into()))?;
        out.push(b);
    }
    if out.is_empty() {
        return Err(ManifestError::InvalidMagic(spec.into()));
    }
    Ok(out)
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::TempDir;

    #[test]
    fn round_trip_minimal_manifest() {
        let tmp = TempDir::new().unwrap();
        let toml_path = tmp.path().join("ext.toml");
        let wasm_path = tmp.path().join("ext.wasm");
        std::fs::write(&wasm_path, b"\x00asm\x01\x00\x00\x00").unwrap();
        std::fs::write(
            &toml_path,
            r#"id = "ext.test"
display_name = "Test"
version = "0.1.0"
formats = ["test"]
sidecar = "ext.wasm"
"#,
        )
        .unwrap();
        let m = Manifest::load(&toml_path).unwrap();
        assert_eq!(m.id, "ext.test");
        assert_eq!(m.formats, vec!["test"]);
        assert_eq!(m.time_budget_ms, 1000);
    }

    #[test]
    fn parses_magic_hex() {
        let bs = parse_magic("0x23 0x20").unwrap();
        assert_eq!(bs, vec![0x23, 0x20]);
    }

    #[test]
    fn rejects_bad_magic() {
        assert!(parse_magic("23 20").is_err());
        assert!(parse_magic("0xZZ").is_err());
        assert!(parse_magic("").is_err());
    }
}
