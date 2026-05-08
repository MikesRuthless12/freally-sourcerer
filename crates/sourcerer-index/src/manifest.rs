//! Atomic commit manifest (TASK-034).
//!
//! `manifest.json` records the most recently committed Tantivy generation
//! plus the per-volume journal cursor that produced it. On reopen,
//! `Index::open` compares the manifest against:
//!
//!   1. The SQLite store (canonical, durable through a `kill -9` because of
//!      WAL).
//!   2. The Tantivy index directory (whose own meta.json is the segment
//!      manifest tantivy commits atomically).
//!
//! If either of those is *behind* the manifest, we re-derive the missing
//! state from the canonical store. If they're *ahead* (rare — only happens
//! if the manifest write was torn), we re-bind the manifest to the latest
//! consistent point. Either way, no corruption is treated as fatal in
//! Phase 4 — recovery is preferred over a hard rebuild prompt because
//! the canonical store is durable.
//!
//! The manifest itself is written via tmp-rename so a crash mid-write
//! never leaves a half-formed JSON file on disk.

use std::collections::BTreeMap;
use std::fs;
use std::path::{Path, PathBuf};

use serde::{Deserialize, Serialize};
use tracing::warn;

use crate::error::IndexError;

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct Manifest {
    /// Manifest schema version. Bumped on backwards-incompatible changes.
    #[serde(default = "default_version")]
    pub version: u32,
    /// Total events ever applied — sanity check vs. SQLite row count.
    #[serde(default)]
    pub applied_events: u64,
    /// Tantivy commit generation, monotonic per `Index::commit` call.
    #[serde(default)]
    pub tantivy_generation: u64,
    /// Per-volume journal cursors. The journal subscribers persist
    /// their own native cursors elsewhere; this is the *index-side*
    /// confirmation that we've durably absorbed up to that USN/Stream
    /// ID.
    #[serde(default)]
    pub volume_cursors: BTreeMap<String, String>,
}

fn default_version() -> u32 {
    1
}

impl Manifest {
    pub fn path(root: &Path) -> PathBuf {
        root.join("manifest.json")
    }

    pub fn load_or_default(root: &Path) -> Result<Self, IndexError> {
        let p = Self::path(root);
        match fs::read(&p) {
            Ok(bytes) => match serde_json::from_slice::<Self>(&bytes) {
                Ok(m) => Ok(m),
                Err(e) => {
                    // A torn rename or a half-flushed JSON should not
                    // brick the index. The canonical SQLite store and
                    // Tantivy meta.json are the durable record; the
                    // manifest is a convenience cache that
                    // `Index::commit` rewrites every cycle. Fall back
                    // to defaults and log so the operator can see why
                    // the cache reset.
                    warn!(
                        path = %p.display(),
                        error = %e,
                        "manifest.json parse failed; treating as missing and re-deriving",
                    );
                    Ok(Self {
                        version: default_version(),
                        ..Default::default()
                    })
                }
            },
            Err(e) if e.kind() == std::io::ErrorKind::NotFound => Ok(Self {
                version: default_version(),
                ..Default::default()
            }),
            Err(e) => Err(IndexError::io(&p, e)),
        }
    }

    pub fn save(&self, root: &Path) -> Result<(), IndexError> {
        let p = Self::path(root);
        let tmp = {
            let mut s = p.as_os_str().to_owned();
            s.push(".tmp");
            PathBuf::from(s)
        };
        let bytes = serde_json::to_vec_pretty(self)
            .map_err(|e| IndexError::Manifest(format!("serialize: {e}")))?;
        fs::write(&tmp, &bytes).map_err(|e| IndexError::io(&tmp, e))?;
        fs::rename(&tmp, &p).map_err(|e| IndexError::io(&p, e))?;
        Ok(())
    }
}
