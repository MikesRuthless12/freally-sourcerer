//! DTOs that mirror `apps/freally-ui/src/lib/ipc/types.ts` byte-for-
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
    /// Win32 FILE_ATTRIBUTE_* bitmask. Bit `0x10` =
    /// `FILE_ATTRIBUTE_DIRECTORY`, which the UI uses to render a folder
    /// icon. `#[serde(default)]` keeps older daemons / older recorded
    /// fixtures readable without re-serialization.
    #[serde(default)]
    pub attrs: u32,
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

/// A contiguous run of hits that belong together under a header row,
/// emitted by the SRC-M07 `dupe:` family.
///
/// Carries the values the members share, not a rendered header string:
/// the UI localises the header through Fluent and formats bytes with
/// the user's chosen size format, exactly as it does for every other
/// number on screen.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct HitGroup {
    /// Shared file name, when the query grouped by name.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// Shared byte size, when the query grouped by size.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub size: Option<u64>,
    /// Index of the group's first hit within `QueryBatch::hits`.
    pub start: u32,
    /// How many hits belong to this group.
    pub len: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QueryBatch {
    pub handle: String,
    pub lens: LensId,
    pub hits: Vec<QueryHit>,
    pub done: bool,
    /// Header rows for grouped result views. Empty for ordinary
    /// queries. `#[serde(default)]` keeps pre-Build-1 daemons and
    /// recorded fixtures readable.
    #[serde(default)]
    pub groups: Vec<HitGroup>,
}

/// SRC-M11 — a spelling correction offered when a search found nothing.
///
/// Carries the rewritten query rather than only the corrected word so
/// accepting the suggestion is one click: the UI runs `query` as-is
/// instead of re-deriving where in the source the term sat.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DidYouMean {
    /// The term as the user typed it.
    pub typed: String,
    /// The indexed name to suggest instead, in its on-disk casing.
    pub suggested: String,
    /// The original query with `typed` replaced by `suggested`.
    pub query: String,
    /// Edit distance between the two, so the UI can choose to present a
    /// 1-edit correction more confidently than a 3-edit one.
    pub distance: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QueryDone {
    pub handle: String,
    pub timings: LensTimings,
    /// Present only when every lens came back empty and a plausible
    /// correction exists. `#[serde(default)]` keeps pre-Build-2 daemons
    /// and recorded fixtures readable.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub did_you_mean: Option<DidYouMean>,
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

/// SRC-M13 — everything the Index Health panel renders.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IndexHealth {
    pub volumes: Vec<VolumeHealth>,
    /// Pending eager content extractions. `None` means the daemon does
    /// not run an eager-extraction worker, so there is no backlog to
    /// report — distinct from `Some(0)`, which means "worker idle".
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub extraction_backlog: Option<u64>,
    pub advisories: Vec<HealthAdvisory>,
}

/// Live-journaling health for one watched root.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VolumeHealth {
    /// The root the change stream is opened on — a drive root on
    /// Windows, the watched directory on macOS / Linux.
    pub root: String,
    /// Volume label when the root maps onto a detected volume.
    pub label: String,
    /// False when the OS refused a change stream here; the root is still
    /// searchable from its last scan, it just will not self-update.
    pub monitoring: bool,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub unavailable_reason: Option<String>,
    pub events_seen: u64,
    pub events_applied: u64,
    /// Events refused because the queue was full — each is a hole in the
    /// index that only a rescan can fill.
    pub events_dropped: u64,
    pub events_coalesced: u64,
    /// Unix ms; 0 means "never".
    pub last_event_ms: u64,
    pub last_drop_ms: u64,
    pub last_apply_ms: u64,
    /// event → query-visible latency of the last committed batch.
    pub last_lag_ms: u64,
    pub max_lag_ms: u64,
    pub queue_depth: u64,
    pub queue_capacity: u64,
    /// The OS discarded the change stream we were reading (a wrapped USN
    /// journal, a recreated FSEvents stream). Events in the gap are lost.
    pub stream_reset: bool,
}

/// One rule the advisor fired. The daemon sends a stable `id` and a
/// single number rather than a sentence, so the UI renders it through
/// Fluent in the user's locale.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HealthAdvisory {
    pub id: AdvisoryId,
    pub severity: AdvisorySeverity,
    /// The root this concerns; `None` for index-wide advisories.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub root: Option<String>,
    /// The one figure the rule's message interpolates — dropped events,
    /// lag in ms, queue depth. 0 when the rule needs no number.
    pub count: u64,
    pub fix: AdvisoryFix,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum AdvisoryId {
    /// The change stream was discarded and re-seated; the index missed
    /// everything in the gap.
    JournalStreamReset,
    /// Events were dropped at our queue boundary under load.
    EventsDropped,
    /// No change stream on this root — scans only.
    NotMonitoring,
    /// Changes are taking a long time to become searchable.
    HighLag,
    /// The queue is close to full, so drops are imminent.
    QueueSaturated,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum AdvisorySeverity {
    Info,
    Warning,
    Critical,
}

/// The one-click action that resolves an advisory.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum AdvisoryFix {
    /// Nothing the app can do unattended — the panel explains instead.
    None,
    /// Re-scan every watched folder, which fills whatever holes the
    /// dropped or missed events left.
    RebuildIndex,
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
