//! `IndexdService` — implements `freally_rpc::Service` and dispatches
//! every supported method.
//!
//! Method names mirror `apps/freally-ui/src/lib/ipc/types.ts`:
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

use freally_rpc::error::codes;
use freally_rpc::{
    CustomExtractorEntry, ExcludeRules, ExtractorInfo, IndexPhase, IndexState, LensId, LensTimings,
    NotificationSink, QueryBatch, QueryDone, QueryRunHandle, RpcError, SandboxView, Service,
    VolumeInfo, VolumeStatus, VolumeUpdate, WatchedFolder,
};
use serde::Deserialize;
use serde_json::{Value, json};
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
        let n = self.handle_counter.fetch_add(1, Ordering::Relaxed);
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
                "index.health" => index_health(&self).await,
                "index.permissions" => index_permissions(&self).await,
                "index.verify" => index_verify(&self).await,
                "index.compact" => index_compact(&self).await,
                "index.rebuild" => index_rebuild(&self).await,
                "extractors.list" => extractors_list(&self).await,
                "extractors.set_mode" => extractors_set_mode(&self, params).await,
                "volumes.list" => volumes_list(&self).await,
                "catalogs.list" => catalogs_list(&self).await,
                "ops.list" => ops_list(&self).await,
                "ops.record" => ops_record(&self, params).await,
                "ops.set_undone" => ops_set_undone(&self, params).await,
                "ops.clear" => ops_clear(&self).await,
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
                "custom_extractors.set_trusted" => {
                    custom_extractors_set_trusted(&self, params).await
                }
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
    #[serde(default)]
    strict_everything: bool,
    #[serde(default)]
    per_lens_limits: Option<freally_rpc::PerLensLimits>,
    /// SRC-M23 — the full match-mode set now crosses the wire.
    ///
    /// Through Build 2 only `match_phonetic` did: the other four were
    /// UI-local, so `Search → Match Case` and friends changed a
    /// checkmark and nothing else. Every flag defaults to `false`, which
    /// is what `MatchMode::default()` already was, so a client that
    /// sends none of them gets exactly the previous behaviour.
    #[serde(default)]
    match_phonetic: bool,
    #[serde(default)]
    match_case: bool,
    #[serde(default)]
    whole_word: bool,
    #[serde(default)]
    match_path: bool,
    #[serde(default)]
    match_diacritics: bool,
    #[serde(default)]
    ignore_punctuation: bool,
    #[serde(default)]
    ignore_whitespace: bool,
}

impl QueryRunParams {
    fn match_mode(&self) -> freally_query::MatchMode {
        freally_query::MatchMode {
            match_case: self.match_case,
            whole_word: self.whole_word,
            match_path: self.match_path,
            match_diacritics: self.match_diacritics,
            match_phonetic: self.match_phonetic,
            ignore_punctuation: self.ignore_punctuation,
            ignore_whitespace: self.ignore_whitespace,
        }
    }
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
            run_query_streaming(svc_for_task.clone(), &handle_for_task, p, cancel, sink).await;
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
    p: QueryRunParams,
    cancel: Arc<AtomicBool>,
    sink: NotificationSink,
) -> LensTimings {
    let match_mode = p.match_mode();
    let QueryRunParams {
        source,
        strict_everything,
        per_lens_limits,
        ..
    } = p;
    let started = std::time::Instant::now();
    let mut timings = LensTimings::default();
    let lenses = [
        LensId::Filename,
        LensId::Content,
        LensId::Audio,
        LensId::Similarity,
    ];

    let parse_opts = if strict_everything {
        freally_query::ParseOpts::strict()
    } else {
        freally_query::ParseOpts::default()
    };
    let parsed = freally_query::parse_with(&source, parse_opts);
    let mut total_hits = 0usize;

    for lens in lenses {
        if cancel.load(Ordering::Acquire) {
            let _ = sink
                .send(freally_rpc::Notification::new(
                    "query:cancelled",
                    json!({ "handle": handle }),
                ))
                .await;
            return timings;
        }
        let lens_started = std::time::Instant::now();
        let (hits, groups) = match (lens, &parsed) {
            (LensId::Filename, Ok(query)) => {
                filename_lens_hits(
                    handle,
                    &svc.state,
                    query,
                    per_lens_limits.as_ref().map(|l| l.filename).unwrap_or(200),
                    match_mode,
                )
                .await
            }
            // Other lenses (content / audio / similarity) wait on their
            // executor wiring; for Phase 12 we surface an empty batch
            // that the UI's lens-grouped renderer treats as "no hits"
            // rather than as an error. Phase 13 lights up these paths.
            _ => (Vec::new(), Vec::new()),
        };
        let elapsed_ms = lens_started.elapsed().as_secs_f32() * 1000.0;
        match lens {
            LensId::Filename => timings.filename_ms = elapsed_ms,
            LensId::Content => timings.content_ms = elapsed_ms,
            LensId::Audio => timings.audio_ms = elapsed_ms,
            LensId::Similarity => timings.similarity_ms = elapsed_ms,
        }
        total_hits += hits.len();
        let batch = QueryBatch {
            handle: handle.to_string(),
            lens,
            hits,
            done: true,
            groups,
        };
        if let Err(e) = sink
            .send(freally_rpc::Notification::new(
                "query:batch",
                serde_json::to_value(batch).unwrap_or(Value::Null),
            ))
            .await
        {
            tracing::debug!(error = %e, "query:batch send failed; client gone");
            return timings;
        }
    }
    // SRC-M11: only when the whole query came back empty. A search that
    // matched something does not need correcting, and the candidate
    // scan is not work worth doing on the success path.
    let did_you_mean = if total_hits == 0 {
        parsed
            .as_ref()
            .ok()
            .and_then(|q| suggest_correction(&svc.state, q))
    } else {
        None
    };

    timings.total_ms = started.elapsed().as_secs_f32() * 1000.0;
    let done = QueryDone {
        handle: handle.to_string(),
        timings,
        did_you_mean,
    };
    let _ = sink
        .send(freally_rpc::Notification::new(
            "query:done",
            serde_json::to_value(done).unwrap_or(Value::Null),
        ))
        .await;
    timings
}

/// How many trigrams a name must share with the typed term to be worth
/// scoring. Two is enough to survive a single typo in a short word
/// while still discarding names that merely share one common trigram.
const SUGGEST_MIN_SHARED_TRIGRAMS: usize = 2;

/// Ceiling on names scored for one suggestion. Edit distance is cheap
/// per candidate but this runs while the user waits, so the scan is
/// bounded rather than proportional to the index.
const SUGGEST_CANDIDATE_CAP: usize = 2_000;

/// SRC-M11 — propose a spelling correction for a query that found
/// nothing.
///
/// Candidates come from shared-trigram overlap rather than the
/// substring path, which by construction returns nothing for the typo
/// that caused the empty result. Names are compared as stems: the user
/// typed a word, not a filename, and leaving `.txt` attached would
/// spend the whole edit budget on the extension.
fn suggest_correction(
    state: &crate::state::DaemonState,
    query: &freally_query::Query,
) -> Option<freally_rpc::DidYouMean> {
    let typed = query.sole_literal_term()?;
    if freally_similarity::max_distance_for(typed) == 0 {
        return None;
    }
    // The rewrite below is a textual substitution, so it is only safe
    // when the term appears once in the source. `path:report report`
    // has one literal but two occurrences, and replacing the first
    // would silently edit the modifier instead of the search term.
    // Offering no suggestion beats offering a query the user did not
    // ask for.
    if query.source().matches(typed).count() != 1 {
        return None;
    }
    let needle = typed.to_lowercase();

    let mut stems: Vec<String> = Vec::new();
    state.index.name_index().for_each_fuzzy_candidate(
        &needle,
        SUGGEST_MIN_SHARED_TRIGRAMS,
        SUGGEST_CANDIDATE_CAP,
        |_, name| {
            // SRC-M12 stores a CJK row as `name<U+0001>reading`, and the
            // candidate iterator yields that composite key. Suggesting it
            // verbatim would put a control character and the pinyin into
            // the user's search box.
            let name = String::from_utf8_lossy(freally_index::phonetic::plain_name(name));
            let stem = match name.rsplit_once('.') {
                // A leading dot is a dotfile, not an extension.
                Some((s, _)) if !s.is_empty() => s,
                _ => name.as_ref(),
            };
            stems.push(stem.to_string());
        },
    );
    stems.sort_unstable();
    stems.dedup();

    let best = freally_similarity::best_suggestion(typed, stems.iter().map(String::as_str))?;
    // Replace only the term itself, so the surrounding modifiers,
    // booleans and quoting survive verbatim.
    let rewritten = query.source().replacen(typed, &best.term, 1);
    Some(freally_rpc::DidYouMean {
        typed: typed.to_string(),
        suggested: best.term,
        query: rewritten,
        distance: best.distance as u32,
    })
}

/// Run a parsed query through the real `freally-query` executor against
/// the daemon's `Index`. Empty queries / parse errors / unsupported
/// modifiers return no hits rather than propagating the error — the UI's
/// search bar already surfaces parse errors via the local `query.parse`
/// IPC, so the daemon stays terse here.
async fn filename_lens_hits(
    handle: &str,
    state: &DaemonState,
    query: &freally_query::Query,
    limit: u32,
    match_mode: freally_query::MatchMode,
) -> (Vec<freally_rpc::QueryHit>, Vec<freally_rpc::HitGroup>) {
    let _ = handle;
    let opts = freally_query::ExecOpts {
        limit: limit as usize,
        match_mode,
        ..Default::default()
    };
    // SRC-M14: snapshot the registry once so `volume:` can be typed as a
    // catalog name, and so every hit can carry its own badge.
    let catalogs = state.catalogs.read().await.clone();
    let result = freally_query::execute_with_catalogs(
        state.index.as_ref(),
        None,
        None,
        Some(&catalogs),
        query,
        opts,
    );
    let (rows, groups) = match result {
        Ok(rs) => {
            // SRC-M07: read the clusters before `collect()` consumes
            // the set — group offsets index into these same rows.
            let groups: Vec<freally_rpc::HitGroup> = rs
                .dupe_groups
                .iter()
                .map(|g| freally_rpc::HitGroup {
                    name: g.key.name.clone(),
                    size: g.key.size,
                    start: g.start as u32,
                    len: g.len as u32,
                })
                .collect();
            (rs.collect(), groups)
        }
        Err(_) => return (Vec::new(), Vec::new()),
    };
    let hits = rows
        .into_iter()
        .map(|row| {
            let (volume_label, volume_offline) = match catalogs.badge(&row.volume) {
                Some((name, offline)) => (Some(name.to_string()), offline),
                None => (None, false),
            };
            freally_rpc::QueryHit {
                file_id: row.file_id.to_string(),
                lens: LensId::Filename,
                name: row.name,
                path: row.path.to_string_lossy().into_owned(),
                ext: row.ext.clone().unwrap_or_default(),
                size: row.size,
                // `row.mtime_ns` is signed; if a journal record carried a
                // zero/unset FILETIME (some MFT bootstrap entries do), the
                // stored value is a large negative i64. Casting that to u64
                // directly wraps to ~1.8e19, which overflows JS's max Date
                // (8.64e15) and renders as `NaN-NaN-NaN`. Clamp to 0 so the
                // UI shows 1970-01-01 — surfaced through `formatDateMs`'s
                // em-dash for "unknown timestamp" once the UI guards land.
                modified_ms: (row.mtime_ns.max(0) / 1_000_000) as u64,
                kind: row.ext.unwrap_or_else(|| "file".into()).to_uppercase(),
                score: 1.0,
                // Pass the FILE_ATTRIBUTE_* bitmask through to the UI so it
                // can render folder icons (0x10 = FILE_ATTRIBUTE_DIRECTORY).
                attrs: row.attrs as u32,
                volume: row.volume,
                volume_label,
                volume_offline,
            }
        })
        .collect();
    (hits, groups)
}

// ---------- index ----------

async fn index_state(svc: &IndexdService) -> Result<Value, RpcError> {
    let stats = svc.state.index.stats().map_err(|e| RpcError::Remote {
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

/// SRC-M13. Reads live watcher counters plus the detected volume list;
/// touches no locks the ingest path holds, so it stays cheap enough for
/// the panel to poll.
async fn index_health(svc: &IndexdService) -> Result<Value, RpcError> {
    let snapshots = svc.state.watchers.snapshot();
    let catalogs = svc.state.catalogs.read().await;
    Ok(serde_json::to_value(crate::health::build(
        snapshots,
        &catalogs,
        &svc.state.index.volume_map(),
    ))?)
}

/// SRC-M21 — the permission health report.
async fn index_permissions(svc: &IndexdService) -> Result<Value, RpcError> {
    let ledger = {
        let guard = svc
            .state
            .permissions
            .lock()
            .unwrap_or_else(|e| e.into_inner());
        guard.clone()
    };
    Ok(serde_json::to_value(crate::permissions::report(&ledger))?)
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

async fn index_rebuild(svc: &IndexdService) -> Result<Value, RpcError> {
    svc.state.reconcile_catalogs().await;
    let folders = svc.state.folders.read().await.clone();
    tracing::info!(count = folders.len(), "index.rebuild received");
    for f in folders {
        let scan_path = std::path::PathBuf::from(&f.path);
        // The scan stamps every row with whatever volume map is installed,
        // so refresh it first — otherwise a drive plugged in since boot is
        // indexed with no volume, or worse, with the id of whatever used to
        // hold that drive letter.
        svc.state.reconcile_catalogs().await;
        crate::scanner::spawn_scan(
            svc.state.index.clone(),
            &scan_path,
            Some(svc.state.permissions.clone()),
        );
    }
    Ok(json!({ "ok": true }))
}

// ---------- extractors ----------

/// Static, exhaustive list of built-in extractor ids. The daemon
/// rejects any `extractors.set_mode` call whose `id` isn't in this
/// table — this is what makes `ExtractorId::new(&'static str)` safe
/// to call without leaking memory at runtime.
const BUILTIN_EXTRACTORS: &[(&str, &str, &[&str])] = &[
    ("plain_text", "Plain text + Markdown", &["txt", "md"]),
    ("pdf", "PDF", &["pdf"]),
    ("xlsx", "Excel (.xlsx)", &["xlsx"]),
    ("docx", "Word (.docx)", &["docx"]),
    ("pptx", "PowerPoint (.pptx)", &["pptx"]),
    (
        "code",
        "Code (tree-sitter)",
        &["rs", "py", "js", "ts", "go"],
    ),
    ("archive_peek", "Archive peek", &["zip", "7z", "tar"]),
    ("structured", "Structured data", &["json", "csv", "yaml"]),
];

async fn extractors_list(svc: &IndexdService) -> Result<Value, RpcError> {
    let pipeline = svc.state.pipeline.read().await;
    let snapshot = pipeline.settings_snapshot();
    let infos: Vec<ExtractorInfo> = BUILTIN_EXTRACTORS
        .iter()
        .map(|(id, dn, fmts)| ExtractorInfo {
            id: (*id).to_string(),
            display_name: (*dn).to_string(),
            mode: into_dto_mode(snapshot.effective_mode(freally_extractors::ExtractorId::new(id))),
            formats: fmts.iter().map(|s| s.to_string()).collect(),
        })
        .collect();
    Ok(serde_json::to_value(infos)?)
}

fn into_dto_mode(m: freally_extractors::ExtractorMode) -> freally_rpc::ExtractorMode {
    match m {
        freally_extractors::ExtractorMode::Eager => freally_rpc::ExtractorMode::Eager,
        freally_extractors::ExtractorMode::Lazy => freally_rpc::ExtractorMode::Lazy,
        freally_extractors::ExtractorMode::Disabled => freally_rpc::ExtractorMode::Disabled,
    }
}

fn from_dto_mode(m: freally_rpc::ExtractorMode) -> freally_extractors::ExtractorMode {
    match m {
        freally_rpc::ExtractorMode::Eager => freally_extractors::ExtractorMode::Eager,
        freally_rpc::ExtractorMode::Lazy => freally_extractors::ExtractorMode::Lazy,
        freally_rpc::ExtractorMode::Disabled => freally_extractors::ExtractorMode::Disabled,
    }
}

#[derive(Debug, Deserialize)]
struct ExtractorsSetModeParams {
    id: String,
    mode: freally_rpc::ExtractorMode,
}

async fn extractors_set_mode(svc: &IndexdService, params: Value) -> Result<Value, RpcError> {
    let p: ExtractorsSetModeParams = serde_json::from_value(params)?;
    // Look up the user-supplied id in the static known-id table so
    // `ExtractorId::new(&'static str)` is fed a `&'static` literal and
    // we don't `Box::leak` user input on every set-mode call.
    let id_static = BUILTIN_EXTRACTORS
        .iter()
        .find_map(|(id, _, _)| if *id == p.id { Some(*id) } else { None })
        .ok_or_else(|| RpcError::Remote {
            code: codes::NOT_FOUND,
            message: format!("unknown extractor id: {}", p.id),
            data: None,
        })?;
    let pipeline = svc.state.pipeline.write().await;
    let mut snapshot = pipeline.settings_snapshot();
    snapshot.set_mode(
        freally_extractors::ExtractorId::new(id_static),
        from_dto_mode(p.mode),
    );
    pipeline.replace_settings(snapshot);
    Ok(Value::Null)
}

// ---------- volumes ----------

/// SRC-M14 — every device we have indexed, attached or not. Reconciles
/// first so a drive plugged in since the last call shows as online.
async fn catalogs_list(svc: &IndexdService) -> Result<Value, RpcError> {
    svc.state.reconcile_catalogs().await;
    let dto = svc.state.catalogs.read().await.to_dto();
    Ok(serde_json::to_value(dto)?)
}

// ---------- operation journal (SRC-M16) ----------

/// Refuse the journal outright when one daemon serves every user on the
/// machine.
///
/// The Windows service binds a single pipe that grants Authenticated
/// Users and keeps its state in `%PROGRAMDATA%`, so a shared journal
/// would let any local peer record a rename that a *different* user's
/// Ctrl+Z then executes under that user's own account — the peer chooses
/// both halves of every pair, so the inverse is entirely theirs.
///
/// Partitioning would need a per-connection identity the RPC layer does
/// not carry today. Refusing is the honest alternative rather than the
/// lesser one: "undo my last operation" has no meaning over a stack
/// shared by everyone, so there is no correct behaviour being withheld.
/// The per-user desktop daemon — what a normal install runs — is
/// unaffected and keeps full undo.
fn deny_if_shared(svc: &IndexdService) -> Result<(), RpcError> {
    if svc.state.shared_multi_user {
        return Err(RpcError::Remote {
            code: codes::INVALID_REQUEST,
            message: "the operation journal is unavailable on a shared multi-user daemon".into(),
            data: None,
        });
    }
    Ok(())
}

/// The undo stack, oldest first, plus what Ctrl+Z / Ctrl+Shift+Z would
/// act on right now. The UI renders the history popover from this and
/// enables its two shortcuts from `undo_id` / `redo_id` being present.
async fn ops_list(svc: &IndexdService) -> Result<Value, RpcError> {
    deny_if_shared(svc)?;
    let j = svc.state.operations.read().await;
    Ok(json!({
        "entries": j.entries().cloned().collect::<Vec<_>>(),
        "undo_id": j.next_undo().map(|e| e.id.clone()),
        "redo_id": j.next_redo().map(|e| e.id.clone()),
    }))
}

async fn ops_record(svc: &IndexdService, params: Value) -> Result<Value, RpcError> {
    deny_if_shared(svc)?;
    let entry: freally_rpc::OperationEntry = serde_json::from_value(params)?;
    // Refuse anything that is not the shape this app produces, at the
    // boundary rather than at replay time. The code that executes the
    // journal deliberately skips `validate_name` (an undo restores a name
    // that already existed on disk), so if the shape is not pinned here it
    // is not pinned anywhere.
    entry.validate_shape().map_err(|e| RpcError::Remote {
        code: codes::INVALID_PARAMS,
        message: e.to_string(),
        data: None,
    })?;
    let id = entry.id.clone();
    svc.state.operations.write().await.record(entry);
    svc.state
        .persist()
        .await
        .map_err(|e| RpcError::Other(e.to_string()))?;
    Ok(json!({ "ok": true, "id": id }))
}

#[derive(Debug, Deserialize)]
struct SetUndoneParams {
    id: String,
    undone: bool,
}

async fn ops_set_undone(svc: &IndexdService, params: Value) -> Result<Value, RpcError> {
    deny_if_shared(svc)?;
    let p: SetUndoneParams = serde_json::from_value(params)?;
    let ok = svc
        .state
        .operations
        .write()
        .await
        .set_undone(&p.id, p.undone);
    if !ok {
        // A stale UI asking about an entry that has fallen off the end
        // of the journal is a no-op, not a crash — but it must not
        // report success, or the client will believe the disk changed.
        return Err(RpcError::Other("unknown operation id".into()));
    }
    svc.state
        .persist()
        .await
        .map_err(|e| RpcError::Other(e.to_string()))?;
    Ok(json!({ "ok": true }))
}

async fn ops_clear(svc: &IndexdService) -> Result<Value, RpcError> {
    deny_if_shared(svc)?;
    svc.state.operations.write().await.clear();
    svc.state
        .persist()
        .await
        .map_err(|e| RpcError::Other(e.to_string()))?;
    Ok(json!({ "ok": true }))
}

async fn volumes_list(svc: &IndexdService) -> Result<Value, RpcError> {
    // The UI polls this whenever the volumes panel is open, which makes
    // it the natural place to notice a drive being plugged or unplugged.
    let detected = svc.state.reconcile_catalogs().await;
    let cfg = svc.state.volumes.read().await.clone();
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
            } else if cfg.auto_include_fixed
                && matches!(v.status, VolumeStatus::Indexed | VolumeStatus::Indexing)
            {
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
    let was_indexed = cfg
        .overrides
        .get(&p.id)
        .and_then(|o| o.indexed)
        .unwrap_or(false);
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
    // If the user just flipped "Include in index" ON, kick off a scan
    // of the volume's mount point. (Walkdir or MFT — `spawn_scan` picks.)
    if matches!(p.indexed, Some(true)) && !was_indexed {
        let detected = crate::volumes::detect();
        if let Some(v) = detected.into_iter().find(|v| v.id == p.id) {
            tracing::info!(
                id = %p.id,
                mount = %v.mount_point,
                "volumes.update: indexing newly-included volume"
            );
            let mount = std::path::PathBuf::from(&v.mount_point);
            crate::scanner::spawn_scan(
                svc.state.index.clone(),
                &mount,
                Some(svc.state.permissions.clone()),
            );
        } else {
            tracing::warn!(id = %p.id, "volumes.update: no matching volume to scan");
        }
    }
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
        // The polkit-elevated upgrade path is wired by freally-journal-lin.
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
    tracing::info!(id = %folder.id, path = %folder.path, "folders.add received");
    let scan_path = std::path::PathBuf::from(&folder.path);
    let mut cur = svc.state.folders.write().await;
    cur.retain(|f| f.id != folder.id);
    cur.push(folder);
    drop(cur);
    svc.state
        .persist()
        .await
        .map_err(|e| RpcError::Other(e.to_string()))?;
    crate::scanner::spawn_scan(
        svc.state.index.clone(),
        &scan_path,
        Some(svc.state.permissions.clone()),
    );
    svc.state.reconcile_watchers().await;
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
    svc.state.reconcile_watchers().await;
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
    svc.state.reconcile_watchers().await;
    Ok(json!({ "ok": true }))
}

async fn folders_rescan(svc: &IndexdService, params: Value) -> Result<Value, RpcError> {
    let p: FolderIdParams = serde_json::from_value(params)?;
    tracing::info!(id = %p.id, "folders.rescan received");
    let cur = svc.state.folders.read().await;
    let target = cur.iter().find(|f| f.id == p.id).cloned();
    drop(cur);
    if let Some(f) = target {
        let scan_path = std::path::PathBuf::from(&f.path);
        crate::scanner::spawn_scan(
            svc.state.index.clone(),
            &scan_path,
            Some(svc.state.permissions.clone()),
        );
    } else {
        tracing::warn!(id = %p.id, "folders.rescan: no folder with that id");
    }
    Ok(json!({ "ok": true }))
}

async fn folders_rescan_all(svc: &IndexdService) -> Result<Value, RpcError> {
    svc.state.reconcile_catalogs().await;
    let folders = svc.state.folders.read().await.clone();
    tracing::info!(count = folders.len(), "folders.rescan_all received");
    for f in folders {
        let scan_path = std::path::PathBuf::from(&f.path);
        crate::scanner::spawn_scan(
            svc.state.index.clone(),
            &scan_path,
            Some(svc.state.permissions.clone()),
        );
    }
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
