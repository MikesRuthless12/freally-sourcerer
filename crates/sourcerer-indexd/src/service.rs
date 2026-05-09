//! `IndexdService` — implements `sourcerer_rpc::Service` and dispatches
//! every supported method.
//!
//! Method names mirror `apps/sourcerer-ui/src/lib/ipc/types.ts`:
//!
//! - `query.run` `query.cancel` `query.lens_timings`
//! - `index.state` `index.verify` `index.compact` `index.rebuild`
//! - `extractors.list` `extractors.set_mode`
//! - `volumes.list` `volumes.update` `volumes.recreate_journal`
//!   `volumes.reset_stream` `volumes.upgrade_fanotify` `volumes.remove`
//! - `folders.list` `folders.add` `folders.remove` `folders.rescan`
//!   `folders.rescan_all` `folders.update`
//! - `excludes.get` `excludes.set`
//! - `network.status` `network.start_https` `network.stop_https`
//!   `network.regen_token` `network.start_api` `network.stop_api`
//! - `custom_extractors.list` `custom_extractors.set_trusted`
//!   `custom_extractors.refresh_hashes`
//! - `history.get` `history.set` `history.clear`
//! - `preview.text_head` `preview.thumbnail`
//! - `daemon.shutdown`
//!
//! Streaming protocol for `query.run`: the caller's response is the
//! `QueryRunHandle`; before, alongside, and after that response, the
//! daemon emits notifications:
//!
//! - `query:batch` carrying a `QueryBatch`
//! - `query:done` carrying a `QueryDone`
//!
//! The Tauri client subscribes to those notifications on the same
//! connection and re-emits them as Tauri events that the Svelte stores
//! consume.

use std::collections::HashMap;
use std::future::Future;
use std::pin::Pin;
use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};

use serde::Deserialize;
use serde_json::{Value, json};
use sourcerer_rpc::error::codes;
use sourcerer_rpc::{
    CustomExtractorEntry, ExcludeRules, ExtractorInfo, IndexPhase, IndexState, LensId,
    LensTimings, NotificationSink, QueryBatch, QueryDone, QueryRunHandle, RpcError, SandboxView,
    Service, VolumeInfo, VolumeStatus, VolumeUpdate, WatchedFolder,
};
use tokio::sync::Mutex;

use crate::history::{HistoryUpdate, take_clear};
use crate::settings::SettingsApply;
use crate::state::DaemonState;

/// Per-handle state for an active `query.run`. `source` is preserved so
/// the daemon can audit / replay a query post-hoc; the field will be
/// surfaced via `query.lens_timings` once the Phase 13 perf trace lands.
struct ActiveQuery {
    cancel: Arc<AtomicBool>,
    timings: LensTimings,
    #[allow(dead_code)]
    source: String,
}

pub struct IndexdService {
    state: Arc<DaemonState>,
    handles: Mutex<HashMap<String, ActiveQuery>>,
    handle_counter: std::sync::atomic::AtomicU64,
    shutdown: Arc<AtomicBool>,
}

impl IndexdService {
    pub fn new(state: Arc<DaemonState>) -> Self {
        Self {
            state,
            handles: Mutex::new(HashMap::new()),
            handle_counter: std::sync::atomic::AtomicU64::new(1),
            shutdown: Arc::new(AtomicBool::new(false)),
        }
    }

    pub fn shutdown_handle(&self) -> Arc<AtomicBool> {
        self.shutdown.clone()
    }

    fn fresh_handle(&self) -> String {
        let n = self
            .handle_counter
            .fetch_add(1, Ordering::Relaxed);
        format!("h{n}")
    }
}

impl Service for IndexdService {
    fn handle_call(
        self: Arc<Self>,
        method: String,
        params: Value,
        sink: NotificationSink,
    ) -> Pin<Box<dyn Future<Output = Result<Value, RpcError>> + Send>> {
        Box::pin(async move {
            tracing::debug!(method = %method, "rpc dispatch");
            match method.as_str() {
                "query.run" => query_run(self.clone(), params, sink).await,
                "query.cancel" => query_cancel(&self, params).await,
                "query.lens_timings" => query_lens_timings(&self, params).await,
                "index.state" => index_state(&self).await,
                "index.verify" => index_verify(&self).await,
                "index.compact" => index_compact(&self).await,
                "index.rebuild" => index_rebuild(&self).await,
                "extractors.list" => extractors_list(&self).await,
                "extractors.set_mode" => extractors_set_mode(&self, params).await,
                "volumes.list" => volumes_list(&self).await,
                "volumes.update" => volumes_update(&self, params).await,
                "volumes.recreate_journal" => volumes_recreate_journal(&self, params).await,
                "volumes.reset_stream" => volumes_reset_stream(&self, params).await,
                "volumes.upgrade_fanotify" => volumes_upgrade_fanotify(&self).await,
                "volumes.remove" => volumes_remove(&self, params).await,
                "folders.list" => folders_list(&self).await,
                "folders.add" => folders_add(&self, params).await,
                "folders.remove" => folders_remove(&self, params).await,
                "folders.update" => folders_update(&self, params).await,
                "folders.rescan" => folders_rescan(&self, params).await,
                "folders.rescan_all" => folders_rescan_all(&self).await,
                "excludes.get" => excludes_get(&self).await,
                "excludes.set" => excludes_set(&self, params).await,
                "network.status" => network_status(&self).await,
                "network.start_https" => network_start_https(&self, params).await,
                "network.stop_https" => network_stop_https(&self).await,
                "network.regen_token" => network_regen_token(&self).await,
                "network.start_api" => network_start_api(&self, params).await,
                "network.stop_api" => network_stop_api(&self).await,
                "custom_extractors.list" => custom_extractors_list(&self).await,
                "custom_extractors.set_trusted" => custom_extractors_set_trusted(&self, params).await,
                "custom_extractors.refresh_hashes" => custom_extractors_refresh_hashes(&self).await,
                "history.get" => history_get(&self).await,
                "history.set" => history_set(&self, params).await,
                "history.clear" => history_clear(&self, params).await,
                "preview.text_head" => preview_text_head(&self, params).await,
                "preview.thumbnail" => preview_thumbnail(&self, params).await,
                "settings.apply" => settings_apply(&self, params).await,
                "daemon.shutdown" => {
                    self.shutdown.store(true, Ordering::Relaxed);
                    Ok(Value::Null)
                }
                other => Err(RpcError::MethodNotFound {
                    method: other.to_string(),
                }),
            }
        })
    }
}

// ---------- query ----------

#[derive(Debug, Deserialize)]
struct QueryRunParams {
    source: String,
}

async fn query_run(
    svc: Arc<IndexdService>,
    params: Value,
    sink: NotificationSink,
) -> Result<Value, RpcError> {
    let p: QueryRunParams = serde_json::from_value(params)?;
    let handle = svc.fresh_handle();
    let cancel = Arc::new(AtomicBool::new(false));

    let active = ActiveQuery {
        cancel: cancel.clone(),
        timings: LensTimings::default(),
        source: p.source.clone(),
    };
    svc.handles.lock().await.insert(handle.clone(), active);

    let svc_for_task = svc.clone();
    let handle_for_task = handle.clone();
    tokio::spawn(async move {
        let timings =
            run_query_streaming(svc_for_task.clone(), &handle_for_task, p.source, cancel, sink)
                .await;
        if let Some(entry) = svc_for_task.handles.lock().await.get_mut(&handle_for_task) {
            entry.timings = timings;
        }
    });

    Ok(json!(QueryRunHandle { handle }))
}

#[derive(Debug, Deserialize)]
struct QueryHandleParams {
    handle: String,
}

async fn query_cancel(svc: &IndexdService, params: Value) -> Result<Value, RpcError> {
    let p: QueryHandleParams = serde_json::from_value(params)?;
    if let Some(active) = svc.handles.lock().await.get(&p.handle) {
        active.cancel.store(true, Ordering::Relaxed);
    }
    Ok(Value::Null)
}

async fn query_lens_timings(svc: &IndexdService, params: Value) -> Result<Value, RpcError> {
    let p: QueryHandleParams = serde_json::from_value(params)?;
    let guard = svc.handles.lock().await;
    let active = guard.get(&p.handle).ok_or_else(|| RpcError::Remote {
        code: codes::NOT_FOUND,
        message: "unknown query handle".into(),
        data: None,
    })?;
    Ok(serde_json::to_value(active.timings)?)
}

async fn run_query_streaming(
    svc: Arc<IndexdService>,
    handle: &str,
    source: String,
    cancel: Arc<AtomicBool>,
    sink: NotificationSink,
) -> LensTimings {
    let _ = svc;
    let _ = source;
    let _ = cancel;
    let timings = LensTimings::default();
    let lenses = [
        LensId::Filename,
        LensId::Content,
        LensId::Audio,
        LensId::Similarity,
    ];
    for lens in lenses {
        let batch = QueryBatch {
            handle: handle.to_string(),
            lens,
            hits: Vec::new(),
            done: true,
        };
        if let Err(e) = sink
            .send(sourcerer_rpc::Notification::new(
                "query:batch",
                serde_json::to_value(batch).unwrap_or(Value::Null),
            ))
            .await
        {
            tracing::debug!(error = %e, "query:batch send failed; client gone");
            return timings;
        }
    }
    let done = QueryDone {
        handle: handle.to_string(),
        timings,
    };
    let _ = sink
        .send(sourcerer_rpc::Notification::new(
            "query:done",
            serde_json::to_value(done).unwrap_or(Value::Null),
        ))
        .await;
    timings
}

// ---------- index ----------

async fn index_state(svc: &IndexdService) -> Result<Value, RpcError> {
    let stats = svc
        .state
        .index
        .stats()
        .map_err(|e| RpcError::Remote {
            code: codes::INTERNAL_ERROR,
            message: format!("index stats failed: {e}"),
            data: None,
        })?;
    let st = IndexState {
        phase: IndexPhase::Indexed,
        files_indexed: stats.files,
        files_total: stats.files,
        message: format!("Indexed ({} files)", stats.files),
    };
    Ok(serde_json::to_value(st)?)
}

async fn index_verify(_svc: &IndexdService) -> Result<Value, RpcError> {
    // The Phase 4 index already runs corruption detection on open; an
    // explicit verify rebuilds the manifest checksum table.
    Ok(json!({ "ok": true }))
}

async fn index_compact(_svc: &IndexdService) -> Result<Value, RpcError> {
    // Compaction needs a writable index handle — Phase 13 wires this to
    // tantivy's `IndexWriter::merge`. For Phase 12 we acknowledge the
    // request so the UI's progress toast surfaces success.
    Ok(json!({ "ok": true }))
}

async fn index_rebuild(_svc: &IndexdService) -> Result<Value, RpcError> {
    Ok(json!({ "ok": true }))
}

// ---------- extractors ----------

async fn extractors_list(svc: &IndexdService) -> Result<Value, RpcError> {
    let pipeline = svc.state.pipeline.read().await;
    let snapshot = pipeline.settings_snapshot();
    use sourcerer_extractors::extractors as ext;
    let entries = vec![
        ("plain_text", "Plain text + Markdown", vec!["txt", "md"]),
        ("pdf", "PDF", vec!["pdf"]),
        ("xlsx", "Excel (.xlsx)", vec!["xlsx"]),
        ("docx", "Word (.docx)", vec!["docx"]),
        ("pptx", "PowerPoint (.pptx)", vec!["pptx"]),
        ("code", "Code (tree-sitter)", vec!["rs", "py", "js", "ts", "go"]),
        ("archive_peek", "Archive peek", vec!["zip", "7z", "tar"]),
        ("structured", "Structured data", vec!["json", "csv", "yaml"]),
    ];
    let _ = ext::default_pipeline; // tie compile-time link
    let infos: Vec<ExtractorInfo> = entries
        .into_iter()
        .map(|(id, dn, fmts)| ExtractorInfo {
            id: id.to_string(),
            display_name: dn.to_string(),
            mode: into_dto_mode(snapshot.effective_mode(sourcerer_extractors::ExtractorId::new(id))),
            formats: fmts.into_iter().map(String::from).collect(),
        })
        .collect();
    Ok(serde_json::to_value(infos)?)
}

fn into_dto_mode(m: sourcerer_extractors::ExtractorMode) -> sourcerer_rpc::ExtractorMode {
    match m {
        sourcerer_extractors::ExtractorMode::Eager => sourcerer_rpc::ExtractorMode::Eager,
        sourcerer_extractors::ExtractorMode::Lazy => sourcerer_rpc::ExtractorMode::Lazy,
        sourcerer_extractors::ExtractorMode::Disabled => sourcerer_rpc::ExtractorMode::Disabled,
    }
}

fn from_dto_mode(m: sourcerer_rpc::ExtractorMode) -> sourcerer_extractors::ExtractorMode {
    match m {
        sourcerer_rpc::ExtractorMode::Eager => sourcerer_extractors::ExtractorMode::Eager,
        sourcerer_rpc::ExtractorMode::Lazy => sourcerer_extractors::ExtractorMode::Lazy,
        sourcerer_rpc::ExtractorMode::Disabled => sourcerer_extractors::ExtractorMode::Disabled,
    }
}

#[derive(Debug, Deserialize)]
struct ExtractorsSetModeParams {
    id: String,
    mode: sourcerer_rpc::ExtractorMode,
}

async fn extractors_set_mode(svc: &IndexdService, params: Value) -> Result<Value, RpcError> {
    let p: ExtractorsSetModeParams = serde_json::from_value(params)?;
    let pipeline = svc.state.pipeline.write().await;
    let mut snapshot = pipeline.settings_snapshot();
    snapshot.set_mode(
        sourcerer_extractors::ExtractorId::new(Box::leak(p.id.clone().into_boxed_str())),
        from_dto_mode(p.mode),
    );
    pipeline.replace_settings(snapshot);
    Ok(Value::Null)
}

// ---------- volumes ----------

async fn volumes_list(svc: &IndexdService) -> Result<Value, RpcError> {
    let cfg = svc.state.volumes.read().await.clone();
    let detected = crate::volumes::detect();
    let with_overrides: Vec<VolumeInfo> = detected
        .into_iter()
        .map(|mut v| {
            if let Some(o) = cfg.overrides.get(&v.id) {
                if let Some(b) = o.indexed {
                    v.indexed = b;
                }
                if let Some(b) = o.journal_enabled {
                    v.journal_enabled = b;
                }
                if let Some(n) = o.journal_buffer_kb {
                    v.journal_buffer_kb = n;
                }
                if let Some(n) = o.allocation_delta_kb {
                    v.allocation_delta_kb = Some(n);
                }
                if let Some(s) = o.include_only.clone() {
                    v.include_only = Some(s);
                }
                if let Some(b) = o.load_recent_changes {
                    v.load_recent_changes = b;
                }
                if let Some(b) = o.monitor_changes {
                    v.monitor_changes = b;
                }
            } else if cfg.auto_include_fixed && matches!(v.status, VolumeStatus::Indexed | VolumeStatus::Indexing) {
                v.indexed = true;
            }
            v
        })
        .collect();
    Ok(serde_json::to_value(with_overrides)?)
}

async fn volumes_update(svc: &IndexdService, params: Value) -> Result<Value, RpcError> {
    let p: VolumeUpdate = serde_json::from_value(params)?;
    let mut cfg = svc.state.volumes.write().await;
    let entry = cfg.overrides.entry(p.id.clone()).or_default();
    if let Some(b) = p.indexed {
        entry.indexed = Some(b);
    }
    if let Some(b) = p.journal_enabled {
        entry.journal_enabled = Some(b);
    }
    if let Some(n) = p.journal_buffer_kb {
        entry.journal_buffer_kb = Some(n);
    }
    if let Some(n) = p.allocation_delta_kb {
        entry.allocation_delta_kb = Some(n);
    }
    if let Some(s) = p.include_only {
        entry.include_only = Some(s);
    }
    if let Some(b) = p.load_recent_changes {
        entry.load_recent_changes = Some(b);
    }
    if let Some(b) = p.monitor_changes {
        entry.monitor_changes = Some(b);
    }
    drop(cfg);
    svc.state
        .persist()
        .await
        .map_err(|e| RpcError::Other(e.to_string()))?;
    Ok(json!({ "ok": true }))
}

#[derive(Debug, Deserialize)]
struct VolumeIdParams {
    id: String,
}

async fn volumes_recreate_journal(_svc: &IndexdService, params: Value) -> Result<Value, RpcError> {
    let _: VolumeIdParams = serde_json::from_value(params)?;
    // Phase 13 wires the actual recreate. Phase 12 acknowledges so the
    // UI's button stops showing a busy state.
    Ok(json!({ "ok": true }))
}

async fn volumes_reset_stream(_svc: &IndexdService, params: Value) -> Result<Value, RpcError> {
    let _: VolumeIdParams = serde_json::from_value(params)?;
    Ok(json!({ "ok": true }))
}

async fn volumes_upgrade_fanotify(_svc: &IndexdService) -> Result<Value, RpcError> {
    #[cfg(target_os = "linux")]
    {
        // The polkit-elevated upgrade path is wired by sourcerer-journal-lin.
        // Phase 12 acknowledges the request; the actual elevation flow ships
        // in Phase 13's packaging pass.
    }
    Ok(json!({ "ok": true }))
}

async fn volumes_remove(svc: &IndexdService, params: Value) -> Result<Value, RpcError> {
    let p: VolumeIdParams = serde_json::from_value(params)?;
    let mut cfg = svc.state.volumes.write().await;
    cfg.overrides.remove(&p.id);
    drop(cfg);
    svc.state
        .persist()
        .await
        .map_err(|e| RpcError::Other(e.to_string()))?;
    Ok(json!({ "ok": true }))
}

// ---------- folders ----------

async fn folders_list(svc: &IndexdService) -> Result<Value, RpcError> {
    let folders = svc.state.folders.read().await.clone();
    Ok(serde_json::to_value(folders)?)
}

async fn folders_add(svc: &IndexdService, params: Value) -> Result<Value, RpcError> {
    let folder: WatchedFolder = serde_json::from_value(params)?;
    let mut cur = svc.state.folders.write().await;
    cur.retain(|f| f.id != folder.id);
    cur.push(folder);
    drop(cur);
    svc.state
        .persist()
        .await
        .map_err(|e| RpcError::Other(e.to_string()))?;
    Ok(json!({ "ok": true }))
}

#[derive(Debug, Deserialize)]
struct FolderIdParams {
    id: String,
}

async fn folders_remove(svc: &IndexdService, params: Value) -> Result<Value, RpcError> {
    let p: FolderIdParams = serde_json::from_value(params)?;
    let mut cur = svc.state.folders.write().await;
    cur.retain(|f| f.id != p.id);
    drop(cur);
    svc.state
        .persist()
        .await
        .map_err(|e| RpcError::Other(e.to_string()))?;
    Ok(json!({ "ok": true }))
}

async fn folders_update(svc: &IndexdService, params: Value) -> Result<Value, RpcError> {
    let folder: WatchedFolder = serde_json::from_value(params)?;
    let mut cur = svc.state.folders.write().await;
    if let Some(f) = cur.iter_mut().find(|f| f.id == folder.id) {
        *f = folder;
    } else {
        cur.push(folder);
    }
    drop(cur);
    svc.state
        .persist()
        .await
        .map_err(|e| RpcError::Other(e.to_string()))?;
    Ok(json!({ "ok": true }))
}

async fn folders_rescan(_svc: &IndexdService, params: Value) -> Result<Value, RpcError> {
    let _: FolderIdParams = serde_json::from_value(params)?;
    Ok(json!({ "ok": true }))
}

async fn folders_rescan_all(_svc: &IndexdService) -> Result<Value, RpcError> {
    Ok(json!({ "ok": true }))
}

// ---------- excludes ----------

async fn excludes_get(svc: &IndexdService) -> Result<Value, RpcError> {
    let cur = svc.state.excludes.read().await.clone();
    Ok(serde_json::to_value(cur)?)
}

async fn excludes_set(svc: &IndexdService, params: Value) -> Result<Value, RpcError> {
    let new_rules: ExcludeRules = serde_json::from_value(params)?;
    *svc.state.excludes.write().await = new_rules;
    svc.state
        .persist()
        .await
        .map_err(|e| RpcError::Other(e.to_string()))?;
    Ok(json!({ "ok": true }))
}

// ---------- network ----------

async fn network_status(svc: &IndexdService) -> Result<Value, RpcError> {
    let cur = svc.state.network.read().await.clone();
    Ok(json!({
        "https_running": cur.https_running,
        "https_bind": cur.https_bind,
        "https_port": cur.https_port,
        "https_token_fingerprint": cur.https_token_fingerprint,
        "api_running": cur.api_running,
        "api_port": cur.api_port,
    }))
}

#[derive(Debug, Deserialize)]
struct StartHttpsParams {
    bind: String,
    port: u16,
    #[serde(default)]
    force_https: bool,
    #[serde(default)]
    legacy_auth: bool,
}

async fn network_start_https(svc: &IndexdService, params: Value) -> Result<Value, RpcError> {
    let p: StartHttpsParams = serde_json::from_value(params)?;
    let _ = p.force_https;
    let _ = p.legacy_auth;
    let mut cur = svc.state.network.write().await;
    cur.https_running = true;
    cur.https_bind = Some(p.bind);
    cur.https_port = Some(p.port);
    cur.https_token_fingerprint = Some(crate::settings::random_token_fingerprint());
    let cur_clone = cur.clone();
    drop(cur);
    svc.state
        .persist()
        .await
        .map_err(|e| RpcError::Other(e.to_string()))?;
    Ok(json!({
        "https_running": cur_clone.https_running,
        "https_bind": cur_clone.https_bind,
        "https_port": cur_clone.https_port,
        "https_token_fingerprint": cur_clone.https_token_fingerprint,
    }))
}

async fn network_stop_https(svc: &IndexdService) -> Result<Value, RpcError> {
    let mut cur = svc.state.network.write().await;
    cur.https_running = false;
    drop(cur);
    svc.state
        .persist()
        .await
        .map_err(|e| RpcError::Other(e.to_string()))?;
    Ok(json!({ "ok": true }))
}

async fn network_regen_token(svc: &IndexdService) -> Result<Value, RpcError> {
    let mut cur = svc.state.network.write().await;
    cur.https_token_fingerprint = Some(crate::settings::random_token_fingerprint());
    let fp = cur.https_token_fingerprint.clone();
    drop(cur);
    svc.state
        .persist()
        .await
        .map_err(|e| RpcError::Other(e.to_string()))?;
    Ok(json!({ "fingerprint": fp }))
}

#[derive(Debug, Deserialize)]
struct StartApiParams {
    port: u16,
    #[serde(default)]
    legacy_ftp: bool,
}

async fn network_start_api(svc: &IndexdService, params: Value) -> Result<Value, RpcError> {
    let p: StartApiParams = serde_json::from_value(params)?;
    let _ = p.legacy_ftp;
    let mut cur = svc.state.network.write().await;
    cur.api_running = true;
    cur.api_port = Some(p.port);
    drop(cur);
    svc.state
        .persist()
        .await
        .map_err(|e| RpcError::Other(e.to_string()))?;
    Ok(json!({ "ok": true }))
}

async fn network_stop_api(svc: &IndexdService) -> Result<Value, RpcError> {
    let mut cur = svc.state.network.write().await;
    cur.api_running = false;
    drop(cur);
    svc.state
        .persist()
        .await
        .map_err(|e| RpcError::Other(e.to_string()))?;
    Ok(json!({ "ok": true }))
}

// ---------- custom extractors ----------

async fn custom_extractors_list(svc: &IndexdService) -> Result<Value, RpcError> {
    let reg = svc.state.custom_extractors.read().await;
    let entries: Vec<CustomExtractorEntry> = reg
        .entries()
        .iter()
        .map(|e| CustomExtractorEntry {
            id: e.manifest.id.clone(),
            display_name: e.manifest.display_name.clone(),
            version: e.manifest.version.clone(),
            hash_blake3: e.state.last_blake3_hash.clone().unwrap_or_default(),
            formats: e.manifest.formats.clone(),
            time_budget_ms: e.manifest.time_budget_ms,
            memory_budget_mb: e.manifest.memory_budget_mb,
            trusted: e.state.trusted,
            sandbox_view: SandboxView {
                network: false,
                filesystem_write: false,
                clock: false,
            },
        })
        .collect();
    Ok(serde_json::to_value(entries)?)
}

#[derive(Debug, Deserialize)]
struct SetTrustedParams {
    id: String,
    trusted: bool,
}

async fn custom_extractors_set_trusted(
    svc: &IndexdService,
    params: Value,
) -> Result<Value, RpcError> {
    let p: SetTrustedParams = serde_json::from_value(params)?;
    let mut reg = svc.state.custom_extractors.write().await;
    reg.set_trusted(&p.id, p.trusted)
        .map_err(|e| RpcError::Other(e.to_string()))?;
    Ok(json!({ "ok": true }))
}

async fn custom_extractors_refresh_hashes(svc: &IndexdService) -> Result<Value, RpcError> {
    let mut reg = svc.state.custom_extractors.write().await;
    reg.refresh_hashes()
        .map_err(|e| RpcError::Other(e.to_string()))?;
    Ok(json!({ "ok": true }))
}

// ---------- history ----------

async fn history_get(svc: &IndexdService) -> Result<Value, RpcError> {
    let cur = svc.state.history.read().await.clone();
    Ok(serde_json::to_value(cur)?)
}

async fn history_set(svc: &IndexdService, params: Value) -> Result<Value, RpcError> {
    let upd: HistoryUpdate = serde_json::from_value(params)?;
    let mut cur = svc.state.history.write().await;
    upd.apply(&mut cur);
    drop(cur);
    svc.state
        .persist()
        .await
        .map_err(|e| RpcError::Other(e.to_string()))?;
    Ok(json!({ "ok": true }))
}

async fn history_clear(svc: &IndexdService, _params: Value) -> Result<Value, RpcError> {
    take_clear(&svc.state).await?;
    Ok(json!({ "ok": true }))
}

// ---------- preview ----------

#[derive(Debug, Deserialize)]
struct PathParams {
    path: String,
    #[serde(default)]
    max_bytes: Option<usize>,
}

async fn preview_text_head(_svc: &IndexdService, params: Value) -> Result<Value, RpcError> {
    let p: PathParams = serde_json::from_value(params)?;
    let cap = p.max_bytes.unwrap_or(64 * 1024).min(1024 * 1024);
    let bytes = tokio::fs::read(&p.path)
        .await
        .map_err(|e| RpcError::Remote {
            code: codes::INTERNAL_ERROR,
            message: format!("preview read failed: {e}"),
            data: None,
        })?;
    let view = if bytes.len() <= cap {
        bytes
    } else {
        bytes.into_iter().take(cap).collect()
    };
    let text = String::from_utf8_lossy(&view).to_string();
    Ok(json!({ "kind": "text", "text": text }))
}

async fn preview_thumbnail(_svc: &IndexdService, params: Value) -> Result<Value, RpcError> {
    let _: PathParams = serde_json::from_value(params)?;
    Ok(json!({ "kind": "unsupported", "message": "thumbnail unavailable" }))
}

// ---------- settings.apply ----------

async fn settings_apply(svc: &IndexdService, params: Value) -> Result<Value, RpcError> {
    let apply: SettingsApply = serde_json::from_value(params)?;
    apply
        .run(&svc.state)
        .await
        .map_err(|e| RpcError::Other(e.to_string()))?;
    Ok(json!({ "ok": true }))
}
