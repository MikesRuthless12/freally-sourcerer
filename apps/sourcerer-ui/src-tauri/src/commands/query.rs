//! Query commands routed through the real `sourcerer-indexd` RPC.
//!
//! - `query_parse` stays in-process — tokenization runs at keystroke
//!   rate and the daemon round-trip would dominate the budget.
//! - `query_run` returns a handle synchronously and streams results
//!   via Tauri events `query:batch` / `query:done` (the daemon emits
//!   them; `daemon.rs` re-emits each notification onto the Tauri
//!   bus).
//! - `query_cancel` and `query_lens_timings` route to the daemon.

use serde::{Deserialize, Serialize};
use sourcerer_query::{ParseOpts as RealParseOpts, ParseReport, parse_to_report};

use crate::daemon;

#[derive(Debug, Clone, Deserialize)]
pub struct ParseOpts {
    pub strict_everything: bool,
}

/// Hard cap on query source length. Anything larger is rejected at the
/// boundary so a hostile deep-link or paste cannot DoS the parser.
pub const MAX_QUERY_SOURCE_LEN: usize = 64_000;

#[tauri::command]
pub fn query_parse(source: String, opts: ParseOpts) -> ParseReport {
    let bounded = if source.len() > MAX_QUERY_SOURCE_LEN {
        let mut end = MAX_QUERY_SOURCE_LEN;
        while !source.is_char_boundary(end) {
            end -= 1;
        }
        &source[..end]
    } else {
        source.as_str()
    };
    let real_opts = if opts.strict_everything {
        RealParseOpts::strict()
    } else {
        RealParseOpts::default()
    };
    parse_to_report(bounded, real_opts)
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QueryRunHandle {
    pub handle: String,
}

#[derive(Debug, Clone, Deserialize, Default)]
pub struct PerLensLimits {
    pub filename: u32,
    pub content: u32,
    pub audio: u32,
    pub similarity: u32,
}

#[tauri::command]
pub fn query_run(
    source: String,
    strict_everything: Option<bool>,
    per_lens_limits: Option<PerLensLimits>,
) -> Result<QueryRunHandle, String> {
    let bounded = if source.len() > MAX_QUERY_SOURCE_LEN {
        let mut end = MAX_QUERY_SOURCE_LEN;
        while !source.is_char_boundary(end) {
            end -= 1;
        }
        source[..end].to_string()
    } else {
        source
    };
    let daemon = daemon::get().ok_or_else(|| "daemon not initialized".to_string())?;
    let mut params = serde_json::json!({
        "source": bounded,
        "strict_everything": strict_everything.unwrap_or(false),
    });
    if let Some(limits) = per_lens_limits {
        params["per_lens_limits"] = serde_json::json!({
            "filename": limits.filename,
            "content": limits.content,
            "audio": limits.audio,
            "similarity": limits.similarity,
        });
    }
    let res: sourcerer_rpc::QueryRunHandle =
        daemon.call("query.run", params).map_err(|e| e.to_string())?;
    Ok(QueryRunHandle { handle: res.handle })
}

#[tauri::command]
pub fn query_cancel(handle: String) -> Result<(), String> {
    let daemon = daemon::get().ok_or_else(|| "daemon not initialized".to_string())?;
    daemon
        .call_void("query.cancel", serde_json::json!({ "handle": handle }))
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub fn query_lens_timings(handle: String) -> Result<sourcerer_rpc::LensTimings, String> {
    let daemon = daemon::get().ok_or_else(|| "daemon not initialized".to_string())?;
    daemon
        .call("query.lens_timings", serde_json::json!({ "handle": handle }))
        .map_err(|e| e.to_string())
}
