//! Daemon-wide shared state.
//!
//! Holds:
//!
//! - The opened `freally_index::Index`. Open at the standard per-OS
//!   path or a caller-provided override (used by smoke tests).
//! - The `freally_extractors::Pipeline` (with live settings).
//! - The `freally_audio::AudioCache`.
//! - The `freally_similarity::SimilarityIndex` (when one has been
//!   built — Phase 6 only writes it on full rebuild; the daemon reads
//!   it on open).
//! - The `freally-extractor-host::Registry` for community extractors.
//! - The volume / folder / exclude config (persisted to a TOML file).
//! - The HTTP/HTTPS server handle (when started).
//!
//! All fields are wrapped in `tokio::sync::RwLock` or `parking_lot::Mutex`-
//! shaped sync primitives so the RPC service can hand off `&self` clones
//! between dispatched calls without contention bottlenecks.

use std::path::PathBuf;
use std::sync::Arc;

use serde::{Deserialize, Serialize};
use freally_audio::AudioCache;
use freally_extractor_host::Registry as CustomExtractorRegistry;
use freally_extractors::{Pipeline, PipelineSettings, extractors as ext};
use freally_index::Index;
use freally_rpc::{ExcludeRules, RescanSchedule, WatchedFolder};
use tokio::sync::RwLock;

#[derive(Debug, Clone, Default)]
pub struct DaemonOptions {
    pub index_root: Option<PathBuf>,
    pub audio_cache_path: Option<PathBuf>,
    pub extractor_registry_root: Option<PathBuf>,
}

pub struct DaemonState {
    pub index: Arc<Index>,
    pub audio_cache: Arc<AudioCache>,
    pub pipeline: RwLock<Pipeline>,
    pub custom_extractors: RwLock<CustomExtractorRegistry>,
    pub volumes: RwLock<VolumesConfig>,
    pub folders: RwLock<Vec<WatchedFolder>>,
    pub excludes: RwLock<ExcludeRules>,
    pub network: RwLock<NetworkState>,
    pub history: RwLock<HistoryConfig>,
    pub config_dir: PathBuf,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VolumesConfig {
    pub auto_include_fixed: bool,
    pub auto_include_removable: bool,
    pub auto_remove_offline: bool,
    /// Per-volume overrides keyed by the cross-OS canonical id
    /// (`<fs-kind>-<mount-point>` after normalization).
    pub overrides: std::collections::BTreeMap<String, VolumeOverride>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct VolumeOverride {
    pub indexed: Option<bool>,
    pub journal_enabled: Option<bool>,
    pub journal_buffer_kb: Option<u32>,
    pub allocation_delta_kb: Option<u32>,
    pub include_only: Option<String>,
    pub load_recent_changes: Option<bool>,
    pub monitor_changes: Option<bool>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct NetworkState {
    pub https_running: bool,
    pub https_bind: Option<String>,
    pub https_port: Option<u16>,
    pub https_token_fingerprint: Option<String>,
    pub api_running: bool,
    pub api_port: Option<u16>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HistoryConfig {
    pub search_history_enabled: bool,
    pub search_history_keep_days: u32,
    pub run_history_enabled: bool,
    pub run_history_keep_days: u32,
    pub privacy_mode: bool,
    pub per_lens: PerLensHistory,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct PerLensHistory {
    pub filename: bool,
    pub content: bool,
    pub audio: bool,
    pub similarity: bool,
}

impl Default for HistoryConfig {
    fn default() -> Self {
        Self {
            search_history_enabled: true,
            search_history_keep_days: 90,
            run_history_enabled: true,
            run_history_keep_days: 90,
            privacy_mode: false,
            per_lens: PerLensHistory {
                filename: true,
                content: true,
                audio: true,
                similarity: true,
            },
        }
    }
}

impl Default for VolumesConfig {
    fn default() -> Self {
        Self {
            auto_include_fixed: true,
            auto_include_removable: false,
            auto_remove_offline: true,
            overrides: Default::default(),
        }
    }
}

impl DaemonState {
    pub fn open(opts: DaemonOptions) -> anyhow::Result<Arc<Self>> {
        let index = if let Some(p) = opts.index_root.as_ref() {
            Index::open(p)?
        } else {
            Index::open_default()?
        };
        let audio_cache_path = opts
            .audio_cache_path
            .clone()
            .unwrap_or_else(|| index.root().join("audio-cache.json"));
        let audio_cache = Arc::new(AudioCache::open(&audio_cache_path)?);
        let extractor_registry_root = opts
            .extractor_registry_root
            .clone()
            .unwrap_or_else(|| index.root().join("extractors"));
        let custom_extractors = CustomExtractorRegistry::open(&extractor_registry_root)?;
        let pipeline = ext::default_pipeline();
        pipeline.replace_settings(PipelineSettings::default());
        let config_dir = index.root().join("config");
        std::fs::create_dir_all(&config_dir)?;
        // Daemon config files round-trip as JSON — TOML's serializer
        // refuses `Option::None` (`unsupported rust type`), and the
        // VolumeOverride / ExcludeRules / NetworkState DTOs all carry
        // sparse Option fields where None has meaning. JSON serializes
        // them cleanly and is the same on-disk shape every other
        // store in the project uses (audio cache, settings.json,
        // bookmarks.json).
        let volumes = load_or_default::<VolumesConfig>(&config_dir.join("volumes.json"));
        let folders = load_or_default::<Vec<WatchedFolder>>(&config_dir.join("folders.json"));
        let excludes = load_or_default::<ExcludeRules>(&config_dir.join("excludes.json"));
        let network = load_or_default::<NetworkState>(&config_dir.join("network.json"));
        let history = load_or_default::<HistoryConfig>(&config_dir.join("history.json"));
        Ok(Arc::new(Self {
            index,
            audio_cache,
            pipeline: RwLock::new(pipeline),
            custom_extractors: RwLock::new(custom_extractors),
            volumes: RwLock::new(volumes),
            folders: RwLock::new(folders),
            excludes: RwLock::new(excludes),
            network: RwLock::new(network),
            history: RwLock::new(history),
            config_dir,
        }))
    }

    pub async fn persist(&self) -> anyhow::Result<()> {
        write_json(
            &self.config_dir.join("volumes.json"),
            &*self.volumes.read().await,
        )?;
        write_json(
            &self.config_dir.join("folders.json"),
            &*self.folders.read().await,
        )?;
        write_json(
            &self.config_dir.join("excludes.json"),
            &*self.excludes.read().await,
        )?;
        write_json(
            &self.config_dir.join("network.json"),
            &*self.network.read().await,
        )?;
        write_json(
            &self.config_dir.join("history.json"),
            &*self.history.read().await,
        )?;
        Ok(())
    }
}

fn load_or_default<T: Default + for<'de> Deserialize<'de>>(p: &std::path::Path) -> T {
    match std::fs::read_to_string(p) {
        Ok(s) => match serde_json::from_str::<T>(&s) {
            Ok(v) => v,
            Err(e) => {
                tracing::warn!(error = %e, path = %p.display(), "config parse failed; using default");
                T::default()
            }
        },
        Err(_) => T::default(),
    }
}

fn write_json<T: Serialize>(p: &std::path::Path, v: &T) -> anyhow::Result<()> {
    if let Some(parent) = p.parent() {
        std::fs::create_dir_all(parent)?;
    }
    let s = serde_json::to_string_pretty(v)?;
    // Tmp-rename so a crash mid-write can't truncate the file.
    let tmp = p.with_extension("json.tmp");
    std::fs::write(&tmp, s)?;
    std::fs::rename(&tmp, p)?;
    Ok(())
}

// `RescanSchedule` doesn't `Default`, but we still want `Vec<WatchedFolder>::default()`
// to work via `load_or_default`. The blanket `Default` for `Vec<T>` requires no
// default on the contained type, so we don't need one here. This compile-time
// assertion documents that — if a future field on `WatchedFolder` adds a
// `Default` requirement, we want to know.
#[allow(dead_code)]
fn _assert_vec_default() {
    let _: Vec<WatchedFolder> = Vec::new();
    let _: RescanSchedule = RescanSchedule::Never;
}
