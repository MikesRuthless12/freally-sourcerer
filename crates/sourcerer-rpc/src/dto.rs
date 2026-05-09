//! DTOs that mirror `apps/sourcerer-ui/src/lib/ipc/types.ts` byte-for-
//! byte. Field names use `serde(rename = "...")` only where the TS shape
//! demands it; everything else is the natural `snake_case` from Rust.
//!
//! Whenever the TS contract changes, keep these in lockstep — the
//! `Phase 12 → Phase 13` parity audit asserts byte-stable JSON output
//! against checked-in fixtures.

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum LensId {
    Filename,
    Content,
    Audio,
    Similarity,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QueryHit {
    pub file_id: String,
    pub lens: LensId,
    pub name: String,
    pub path: String,
    pub ext: String,
    pub size: u64,
    pub modified_ms: u64,
    #[serde(rename = "type")]
    pub kind: String,
    pub score: f32,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct PerLensLimits {
    pub filename: u32,
    pub content: u32,
    pub audio: u32,
    pub similarity: u32,
}

impl Default for PerLensLimits {
    fn default() -> Self {
        Self {
            filename: 200,
            content: 50,
            audio: 20,
            similarity: 10,
        }
    }
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, Default)]
pub struct LensTimings {
    pub filename_ms: f32,
    pub content_ms: f32,
    pub audio_ms: f32,
    pub similarity_ms: f32,
    pub total_ms: f32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QueryRunHandle {
    pub handle: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QueryBatch {
    pub handle: String,
    pub lens: LensId,
    pub hits: Vec<QueryHit>,
    pub done: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QueryDone {
    pub handle: String,
    pub timings: LensTimings,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum IndexPhase {
    Indexing,
    Indexed,
    Paused,
    Error,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IndexState {
    pub phase: IndexPhase,
    pub files_indexed: u64,
    pub files_total: u64,
    pub message: String,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum ExtractorMode {
    Eager,
    Lazy,
    Disabled,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExtractorInfo {
    pub id: String,
    pub display_name: String,
    pub mode: ExtractorMode,
    pub formats: Vec<String>,
}

/// One detected volume / mount point on the host. Cross-platform: every
/// supported FS on every OS surfaces here, with the FS family in
/// `fs_kind`.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VolumeInfo {
    pub id: String,
    pub label: String,
    pub mount_point: String,
    pub fs_kind: String,
    pub used_bytes: u64,
    pub total_bytes: u64,
    pub status: VolumeStatus,
    pub indexed: bool,
    pub journal_enabled: bool,
    pub journal_buffer_kb: u32,
    pub allocation_delta_kb: Option<u32>,
    pub include_only: Option<String>,
    pub load_recent_changes: bool,
    pub monitor_changes: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum VolumeStatus {
    Indexed,
    Indexing,
    Paused,
    Offline,
    Error,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VolumeUpdate {
    pub id: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub indexed: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub journal_enabled: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub journal_buffer_kb: Option<u32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub allocation_delta_kb: Option<u32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub include_only: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub load_recent_changes: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub monitor_changes: Option<bool>,
}

/// One folder in `Indexes → Folders` (additional watched roots beyond
/// the auto-detected volumes).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WatchedFolder {
    pub id: String,
    pub path: String,
    pub monitor_changes: bool,
    pub buffer_kb: u32,
    pub rescan_on_full_buffer: bool,
    pub rescan_schedule: RescanSchedule,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case", tag = "kind")]
pub enum RescanSchedule {
    AtTime { hour: u8, minute: u8 },
    EveryHours { hours: u32 },
    Never,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExcludeRules {
    pub exclude_hidden: bool,
    pub exclude_system: bool,
    pub list_enabled: bool,
    pub folders: Vec<String>,
    pub include_only_files: Option<String>,
    pub exclude_files: Option<String>,
}

impl Default for ExcludeRules {
    fn default() -> Self {
        Self {
            exclude_hidden: false,
            exclude_system: false,
            list_enabled: true,
            folders: Vec::new(),
            include_only_files: None,
            exclude_files: None,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct CustomExtractorEntry {
    pub id: String,
    pub display_name: String,
    pub version: String,
    pub hash_blake3: String,
    pub formats: Vec<String>,
    pub time_budget_ms: u32,
    pub memory_budget_mb: u32,
    pub trusted: bool,
    pub sandbox_view: SandboxView,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct SandboxView {
    pub network: bool,
    pub filesystem_write: bool,
    pub clock: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PreviewPayload {
    pub kind: PreviewKind,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub text: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub data_url: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum PreviewKind {
    Text,
    Image,
    Unsupported,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServerStartParams {
    pub bind: String,
    pub port: u16,
    pub force_https: bool,
    pub legacy_auth: bool,
    pub cors_allowlist: Vec<String>,
    pub rate_limit_per_sec: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServerStatus {
    pub running: bool,
    pub bind: Option<String>,
    pub port: Option<u16>,
    pub token_fingerprint: Option<String>,
}
