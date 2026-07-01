//! On-disk registry of installed custom extractors.
//!
//! `<index_root>/extractors/` holds one subdirectory per extractor. Each
//! contains `manifest.toml` + the `*.wasm` sidecar. The registry-level
//! `registry.toml` records the user's per-extractor trust state plus any
//! cached blake3 hashes used to detect tampering between launches.

use std::collections::BTreeMap;
use std::path::{Path, PathBuf};

use serde::{Deserialize, Serialize};
use thiserror::Error;

use crate::manifest::{Manifest, ManifestError};

#[derive(Debug, Error)]
pub enum RegistryError {
    #[error("io: {0}")]
    Io(#[from] std::io::Error),

    #[error("toml: {0}")]
    Toml(#[from] toml::de::Error),

    #[error("manifest: {0}")]
    Manifest(#[from] ManifestError),

    #[error("toml encode: {0}")]
    TomlEncode(#[from] toml::ser::Error),

    #[error("extractor `{0}` not found")]
    NotFound(String),
}

/// Per-extractor user state stored in `registry.toml`.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct EntryState {
    pub trusted: bool,
    pub disabled: bool,
    pub last_blake3_hash: Option<String>,
    /// Crash counter; the host disables an extractor that crashes 3+
    /// times in a row until the user re-trusts it.
    pub crash_count: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct RegistrySettings {
    /// When true, freshly added extractors are trusted automatically.
    /// Default false — per Phase 12 trust model, new extractors must be
    /// explicitly trusted by the user.
    #[serde(default)]
    pub trust_on_install: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
struct RegistryFile {
    #[serde(default)]
    settings: RegistrySettings,
    #[serde(default)]
    state: BTreeMap<String, EntryState>,
}

#[derive(Debug, Clone)]
pub struct RegistryEntry {
    pub manifest: Manifest,
    pub state: EntryState,
    pub manifest_path: PathBuf,
    pub sidecar_path: PathBuf,
}

pub struct Registry {
    root: PathBuf,
    file_path: PathBuf,
    entries: Vec<RegistryEntry>,
    file: RegistryFile,
}

impl Registry {
    /// Open (or create) the extractors registry at `<index_root>/
    /// extractors/`. Each subdirectory is scanned for a `manifest.toml`.
    pub fn open(root: &Path) -> Result<Self, RegistryError> {
        std::fs::create_dir_all(root)?;
        let file_path = root.join("registry.toml");
        let file: RegistryFile = if file_path.exists() {
            let raw = std::fs::read_to_string(&file_path)?;
            toml::from_str(&raw)?
        } else {
            RegistryFile::default()
        };
        let mut entries: Vec<RegistryEntry> = Vec::new();
        for de in std::fs::read_dir(root)? {
            let de = de?;
            let p = de.path();
            if !p.is_dir() {
                continue;
            }
            let manifest_path = p.join("manifest.toml");
            if !manifest_path.exists() {
                continue;
            }
            match Manifest::load(&manifest_path) {
                Ok(m) => {
                    let sidecar_path = p.join(&m.sidecar);
                    let state = file.state.get(&m.id).cloned().unwrap_or_default();
                    entries.push(RegistryEntry {
                        manifest: m,
                        state,
                        manifest_path,
                        sidecar_path,
                    });
                }
                Err(e) => {
                    tracing::warn!(error = %e, path = %manifest_path.display(), "skipping bad manifest");
                }
            }
        }
        Ok(Self {
            root: root.to_path_buf(),
            file_path,
            entries,
            file,
        })
    }

    pub fn settings(&self) -> &RegistrySettings {
        &self.file.settings
    }

    pub fn set_settings(&mut self, s: RegistrySettings) -> Result<(), RegistryError> {
        self.file.settings = s;
        self.persist()
    }

    pub fn entries(&self) -> &[RegistryEntry] {
        &self.entries
    }

    pub fn entry(&self, id: &str) -> Option<&RegistryEntry> {
        self.entries.iter().find(|e| e.manifest.id == id)
    }

    pub fn set_trusted(&mut self, id: &str, trusted: bool) -> Result<(), RegistryError> {
        let mut found = false;
        for e in self.entries.iter_mut() {
            if e.manifest.id == id {
                e.state.trusted = trusted;
                if trusted {
                    e.state.crash_count = 0;
                    e.state.disabled = false;
                }
                self.file.state.insert(id.to_string(), e.state.clone());
                found = true;
                break;
            }
        }
        if !found {
            return Err(RegistryError::NotFound(id.into()));
        }
        self.persist()
    }

    pub fn record_crash(&mut self, id: &str) -> Result<(), RegistryError> {
        for e in self.entries.iter_mut() {
            if e.manifest.id == id {
                e.state.crash_count = e.state.crash_count.saturating_add(1);
                if e.state.crash_count >= 3 {
                    e.state.disabled = true;
                }
                self.file.state.insert(id.to_string(), e.state.clone());
                return self.persist();
            }
        }
        Err(RegistryError::NotFound(id.into()))
    }

    /// Compute and cache the blake3 hash of each entry's sidecar so the
    /// UI can surface "modified since trusted" without recomputing on
    /// every render.
    pub fn refresh_hashes(&mut self) -> Result<(), RegistryError> {
        for e in self.entries.iter_mut() {
            let bytes = std::fs::read(&e.sidecar_path)?;
            let h = blake3::hash(&bytes);
            e.state.last_blake3_hash = Some(h.to_hex().to_string());
            self.file
                .state
                .insert(e.manifest.id.clone(), e.state.clone());
        }
        self.persist()
    }

    fn persist(&self) -> Result<(), RegistryError> {
        let toml_str = toml::to_string_pretty(&self.file)?;
        std::fs::write(&self.file_path, toml_str)?;
        Ok(())
    }

    pub fn root(&self) -> &Path {
        &self.root
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::TempDir;

    fn write_min_extractor(root: &Path, id: &str) {
        let dir = root.join(id);
        std::fs::create_dir_all(&dir).unwrap();
        std::fs::write(dir.join("ext.wasm"), b"\x00asm\x01\x00\x00\x00").unwrap();
        std::fs::write(
            dir.join("manifest.toml"),
            format!(
                r#"id = "{id}"
display_name = "{id}"
version = "0.1.0"
formats = ["x"]
sidecar = "ext.wasm"
"#
            ),
        )
        .unwrap();
    }

    #[test]
    fn loads_entries() {
        let tmp = TempDir::new().unwrap();
        write_min_extractor(tmp.path(), "ext.a");
        write_min_extractor(tmp.path(), "ext.b");
        let reg = Registry::open(tmp.path()).unwrap();
        assert_eq!(reg.entries().len(), 2);
    }

    #[test]
    fn trust_round_trip() {
        let tmp = TempDir::new().unwrap();
        write_min_extractor(tmp.path(), "ext.a");
        let mut reg = Registry::open(tmp.path()).unwrap();
        reg.set_trusted("ext.a", true).unwrap();
        let reg2 = Registry::open(tmp.path()).unwrap();
        assert!(reg2.entry("ext.a").unwrap().state.trusted);
    }

    #[test]
    fn crash_counter_disables_at_three() {
        let tmp = TempDir::new().unwrap();
        write_min_extractor(tmp.path(), "ext.a");
        let mut reg = Registry::open(tmp.path()).unwrap();
        reg.record_crash("ext.a").unwrap();
        reg.record_crash("ext.a").unwrap();
        reg.record_crash("ext.a").unwrap();
        let e = reg.entry("ext.a").unwrap();
        assert_eq!(e.state.crash_count, 3);
        assert!(e.state.disabled);
    }
}
