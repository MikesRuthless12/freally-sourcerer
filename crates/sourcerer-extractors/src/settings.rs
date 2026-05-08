//! Per-extractor mode + framework-wide budgets.
//!
//! Phase 7 ships [`ExtractorMode`] (Lazy / Eager / Disabled), a
//! [`PipelineSettings`] struct that holds the global default mode plus
//! per-extractor overrides, and the time / memory / sink-cap budgets
//! the sandbox honors.
//!
//! The settings are JSON-serializable. Phase 12's settings dialog
//! round-trips them through `~/.config/sourcerer/extractors.json`
//! (XDG_CONFIG_HOME on Linux, `%APPDATA%\Sourcerer\` on Windows,
//! `~/Library/Application Support/Sourcerer/` on macOS); the daemon
//! re-reads on SIGHUP / file-change and calls
//! [`Pipeline::replace_settings`](crate::pipeline::Pipeline::replace_settings).

use std::collections::HashMap;
use std::time::Duration;

use serde::{Deserialize, Serialize};

use crate::ExtractorId;

/// How the dispatcher should treat an extractor.
///
/// Lazy is the safe Phase-7 default — eager extraction would burn CPU
/// on a fresh-bootstrap index before the user has expressed any
/// interest in content search. Phase 12 lets the user flip individual
/// formats to Eager via the settings dialog.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ExtractorMode {
    /// Run on `Create` / `Modify` events, ahead of any query. Burns
    /// CPU at index time; the trade-off is sub-millisecond `content:`
    /// queries.
    Eager,
    /// Defer extraction until the first relevant query lands. Cheap at
    /// index time; first query pays the extraction cost.
    #[default]
    Lazy,
    /// Never run. The dispatcher skips disabled extractors entirely.
    Disabled,
}

/// Default time budget per extraction (Build Guide Phase 7 prompt).
pub const DEFAULT_TIME_BUDGET: Duration = Duration::from_secs(5);

/// Default in-process memory ceiling (Build Guide Phase 7 prompt). The
/// sandbox watches the daemon's RSS and aborts a running extraction
/// when RSS rises above this value; on platforms without a usable RSS
/// API (notably non-Linux non-Windows Unix), the time budget is the
/// sole guard.
pub const DEFAULT_MEMORY_CEILING_BYTES: usize = 256 * 1024 * 1024;

/// Default per-extraction text-output cap (matches `sink::DEFAULT_TEXT_CAP_BYTES`).
pub const DEFAULT_TEXT_CAP_BYTES: usize = crate::sink::DEFAULT_TEXT_CAP_BYTES;

/// Default queue capacity. Mirrors `sourcerer-index::DEFAULT_CAPACITY`
/// so a stalled extractor pipeline pushes back-pressure into the
/// indexd at the same scale the journal subscribers do.
pub const DEFAULT_QUEUE_CAPACITY: usize = 10_000;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PipelineSettings {
    /// Mode applied when no per-extractor override exists.
    pub default_mode: ExtractorMode,
    /// Per-extractor overrides keyed by `ExtractorId::as_str()` so the
    /// JSON file stays stable across crate-version bumps.
    pub overrides: HashMap<String, ExtractorMode>,
    /// Per-extraction time budget. Sandbox enforces. Stored as
    /// milliseconds in JSON (sub-second budgets are useful for tests
    /// and for the eventual Phase 12 settings dialog; serializing as
    /// seconds would silently round them to zero on round-trip).
    #[serde(with = "humantime_ms")]
    pub time_budget: Duration,
    /// Process-wide RSS ceiling. Sandbox watches.
    pub memory_ceiling_bytes: usize,
    /// Hard cap on bytes a single extraction can write to its
    /// `TextSink`.
    pub text_cap_bytes: usize,
    /// Bounded queue capacity for the [`ExtractionQueue`](crate::ExtractionQueue).
    pub queue_capacity: usize,
}

impl Default for PipelineSettings {
    fn default() -> Self {
        Self {
            default_mode: ExtractorMode::default(),
            overrides: HashMap::new(),
            time_budget: DEFAULT_TIME_BUDGET,
            memory_ceiling_bytes: DEFAULT_MEMORY_CEILING_BYTES,
            text_cap_bytes: DEFAULT_TEXT_CAP_BYTES,
            queue_capacity: DEFAULT_QUEUE_CAPACITY,
        }
    }
}

impl PipelineSettings {
    pub fn set_mode(&mut self, id: ExtractorId, mode: ExtractorMode) {
        self.overrides.insert(id.as_str().to_string(), mode);
    }

    pub fn clear_override(&mut self, id: ExtractorId) {
        self.overrides.remove(id.as_str());
    }

    pub fn effective_mode(&self, id: ExtractorId) -> ExtractorMode {
        self.overrides
            .get(id.as_str())
            .copied()
            .unwrap_or(self.default_mode)
    }

    /// Reject settings whose budgets / capacities would degenerate the
    /// framework into a no-op (a zero time budget, for example,
    /// time-budget-fails *every* extraction immediately). Daemon
    /// callers should run this after deserializing user-edited JSON.
    pub fn validate(&self) -> Result<(), SettingsError> {
        if self.time_budget.is_zero() {
            return Err(SettingsError::Zero("time_budget"));
        }
        if self.memory_ceiling_bytes == 0 {
            return Err(SettingsError::Zero("memory_ceiling_bytes"));
        }
        if self.text_cap_bytes == 0 {
            return Err(SettingsError::Zero("text_cap_bytes"));
        }
        if self.queue_capacity == 0 {
            return Err(SettingsError::Zero("queue_capacity"));
        }
        Ok(())
    }
}

#[derive(Debug, thiserror::Error, PartialEq, Eq)]
pub enum SettingsError {
    #[error("`{0}` must be greater than zero")]
    Zero(&'static str),
}

mod humantime_ms {
    //! Plain `Duration` serializes verbosely (`{ secs: 5, nanos: 0 }`).
    //! For human-edited config files we serialize milliseconds-as-u64
    //! so sub-second budgets survive a JSON round-trip.

    use std::time::Duration;

    use serde::{Deserialize, Deserializer, Serialize, Serializer};

    pub fn serialize<S>(d: &Duration, ser: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        // u64::MAX ms is ~584 million years; any realistic Duration
        // fits without truncation. The cast is the documented serde
        // contract — humans editing JSON expect a plain integer.
        let ms: u64 = d.as_millis().try_into().unwrap_or(u64::MAX);
        ms.serialize(ser)
    }

    pub fn deserialize<'de, D>(de: D) -> Result<Duration, D::Error>
    where
        D: Deserializer<'de>,
    {
        let ms = u64::deserialize(de)?;
        Ok(Duration::from_millis(ms))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn override_wins_over_default() {
        let mut s = PipelineSettings::default();
        let id = ExtractorId::new("pdf");
        assert_eq!(s.effective_mode(id), ExtractorMode::Lazy);
        s.set_mode(id, ExtractorMode::Eager);
        assert_eq!(s.effective_mode(id), ExtractorMode::Eager);
        s.clear_override(id);
        assert_eq!(s.effective_mode(id), ExtractorMode::Lazy);
    }

    #[test]
    fn sub_second_time_budget_survives_json_round_trip() {
        // Regression for the seconds-as-u64 truncation: a 750ms budget
        // used to round-trip as Duration::ZERO, which would
        // time-budget-fail every extraction immediately. The current
        // serializer stores milliseconds.
        let s = PipelineSettings {
            time_budget: Duration::from_millis(750),
            ..Default::default()
        };
        let json = serde_json::to_string(&s).unwrap();
        let back: PipelineSettings = serde_json::from_str(&json).unwrap();
        assert_eq!(back.time_budget, Duration::from_millis(750));
    }

    #[test]
    fn validate_rejects_zero_budgets() {
        let s = PipelineSettings {
            time_budget: Duration::ZERO,
            ..Default::default()
        };
        assert_eq!(s.validate(), Err(SettingsError::Zero("time_budget")));

        let s = PipelineSettings {
            memory_ceiling_bytes: 0,
            ..Default::default()
        };
        assert_eq!(
            s.validate(),
            Err(SettingsError::Zero("memory_ceiling_bytes"))
        );

        let s = PipelineSettings {
            text_cap_bytes: 0,
            ..Default::default()
        };
        assert_eq!(s.validate(), Err(SettingsError::Zero("text_cap_bytes")));

        let s = PipelineSettings {
            queue_capacity: 0,
            ..Default::default()
        };
        assert_eq!(s.validate(), Err(SettingsError::Zero("queue_capacity")));
    }

    #[test]
    fn validate_accepts_defaults() {
        assert!(PipelineSettings::default().validate().is_ok());
    }

    #[test]
    fn settings_round_trip_json() {
        let mut s = PipelineSettings::default();
        s.set_mode(ExtractorId::new("pdf"), ExtractorMode::Eager);
        s.set_mode(ExtractorId::new("docx"), ExtractorMode::Disabled);
        s.time_budget = Duration::from_secs(10);
        let j = serde_json::to_string(&s).unwrap();
        let r: PipelineSettings = serde_json::from_str(&j).unwrap();
        assert_eq!(r.default_mode, ExtractorMode::Lazy);
        assert_eq!(
            r.effective_mode(ExtractorId::new("pdf")),
            ExtractorMode::Eager
        );
        assert_eq!(
            r.effective_mode(ExtractorId::new("docx")),
            ExtractorMode::Disabled
        );
        assert_eq!(r.time_budget, Duration::from_secs(10));
    }
}
