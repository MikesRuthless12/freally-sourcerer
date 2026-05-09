//! `settings.apply` payload — a typed container for every index-affecting
//! settings change the UI dispatches to the daemon.
//!
//! The Tauri side calls `settings.apply` with the subset of fields it
//! wants the daemon to acknowledge, and the daemon mutates the relevant
//! state slots, then persists.

use serde::{Deserialize, Serialize};

use crate::state::DaemonState;

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct SettingsApply {
    #[serde(default)]
    pub auto_include_fixed: Option<bool>,
    #[serde(default)]
    pub auto_include_removable: Option<bool>,
    #[serde(default)]
    pub auto_remove_offline: Option<bool>,
    /// Default ExtractorMode at the pipeline level.
    #[serde(default)]
    pub default_extractor_mode: Option<sourcerer_rpc::ExtractorMode>,
    /// Memory budget in MiB for in-process extraction.
    #[serde(default)]
    pub extractor_memory_mb: Option<u32>,
    /// Time budget per extraction in ms.
    #[serde(default)]
    pub extractor_time_ms: Option<u32>,
}

impl SettingsApply {
    pub async fn run(self, state: &DaemonState) -> anyhow::Result<()> {
        if let Some(b) = self.auto_include_fixed {
            state.volumes.write().await.auto_include_fixed = b;
        }
        if let Some(b) = self.auto_include_removable {
            state.volumes.write().await.auto_include_removable = b;
        }
        if let Some(b) = self.auto_remove_offline {
            state.volumes.write().await.auto_remove_offline = b;
        }
        if let Some(m) = self.default_extractor_mode {
            let pipeline = state.pipeline.write().await;
            let mut snap = pipeline.settings_snapshot();
            snap.default_mode = match m {
                sourcerer_rpc::ExtractorMode::Eager => sourcerer_extractors::ExtractorMode::Eager,
                sourcerer_rpc::ExtractorMode::Lazy => sourcerer_extractors::ExtractorMode::Lazy,
                sourcerer_rpc::ExtractorMode::Disabled => {
                    sourcerer_extractors::ExtractorMode::Disabled
                }
            };
            pipeline.replace_settings(snap);
        }
        if let Some(mb) = self.extractor_memory_mb {
            let pipeline = state.pipeline.write().await;
            let mut snap = pipeline.settings_snapshot();
            snap.memory_ceiling_bytes = (mb as usize).saturating_mul(1024 * 1024);
            pipeline.replace_settings(snap);
        }
        if let Some(ms) = self.extractor_time_ms {
            let pipeline = state.pipeline.write().await;
            let mut snap = pipeline.settings_snapshot();
            snap.time_budget = std::time::Duration::from_millis(ms as u64);
            pipeline.replace_settings(snap);
        }
        state.persist().await?;
        Ok(())
    }
}

/// Generate a short fingerprint string for a freshly-rotated HTTPS API
/// token. The fingerprint is what the UI shows; the actual token lives
/// only in `<index_root>/config/network.token` with mode 0600 / DACL
/// restricted.
pub fn random_token_fingerprint() -> String {
    use std::time::{SystemTime, UNIX_EPOCH};
    let n = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|d| d.as_nanos())
        .unwrap_or(0);
    let h = blake_hash(&n.to_le_bytes());
    h.chars().take(12).collect()
}

fn blake_hash(b: &[u8]) -> String {
    // Avoid pulling in blake3 just for this — use a tiny FNV-1a so the
    // crate stays leaner. The fingerprint is non-secret display text.
    const FNV_OFFSET: u64 = 0xcbf29ce484222325;
    const FNV_PRIME: u64 = 0x100000001b3;
    let mut h = FNV_OFFSET;
    for &c in b {
        h ^= c as u64;
        h = h.wrapping_mul(FNV_PRIME);
    }
    format!("{h:016x}")
}
