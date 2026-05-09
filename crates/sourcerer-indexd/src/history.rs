//! History config + clear-all command.

use serde::{Deserialize, Serialize};
use sourcerer_rpc::error::RpcError;

use crate::state::{DaemonState, HistoryConfig};

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct HistoryUpdate {
    #[serde(default)]
    pub search_history_enabled: Option<bool>,
    #[serde(default)]
    pub search_history_keep_days: Option<u32>,
    #[serde(default)]
    pub run_history_enabled: Option<bool>,
    #[serde(default)]
    pub run_history_keep_days: Option<u32>,
    #[serde(default)]
    pub privacy_mode: Option<bool>,
    #[serde(default)]
    pub per_lens: Option<crate::state::PerLensHistory>,
}

impl HistoryUpdate {
    pub fn apply(self, cfg: &mut HistoryConfig) {
        if let Some(b) = self.search_history_enabled {
            cfg.search_history_enabled = b;
        }
        if let Some(n) = self.search_history_keep_days {
            cfg.search_history_keep_days = n;
        }
        if let Some(b) = self.run_history_enabled {
            cfg.run_history_enabled = b;
        }
        if let Some(n) = self.run_history_keep_days {
            cfg.run_history_keep_days = n;
        }
        if let Some(b) = self.privacy_mode {
            cfg.privacy_mode = b;
        }
        if let Some(p) = self.per_lens {
            cfg.per_lens = p;
        }
    }
}

/// Wipe the on-disk history files. The configuration itself (whether to
/// keep recording) is unchanged — `Clear Now` only blanks accumulated
/// entries, per Everything's behavior.
pub async fn take_clear(state: &DaemonState) -> Result<(), RpcError> {
    let _ = state;
    // History entries themselves are local-side state in
    // `apps/sourcerer-ui/src/lib/stores/`. The daemon-side equivalent
    // lands here in Phase 13 when we move history persistence into the
    // canonical files.db. For Phase 12 we simply ack the call.
    Ok(())
}
