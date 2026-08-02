//! SRC-M13 — the rules behind `index.health`.
//!
//! `build` turns raw watcher telemetry into the `IndexHealth` document the
//! Index Health panel renders, and `advise` is the rules engine that turns
//! that document into the panel's actionable advice.
//!
//! `advise` is deliberately a pure function over already-collected
//! numbers: it is the part that decides whether the app tells the user to
//! rebuild their index, and that decision is worth testing without
//! standing up a daemon, a volume, or a filesystem.

use freally_index::VolumeMap;
use freally_rpc::{
    AdvisoryFix, AdvisoryId, AdvisorySeverity, HealthAdvisory, IndexHealth, VolumeHealth,
};

use crate::catalogs::CatalogRegistry;
use crate::watcher::{WatcherSnapshot, WatcherState};

/// Lag above which changes no longer feel live. Everything the app does
/// is built on results being current, so a whole second of staleness is
/// already worth surfacing; five is worth complaining about.
const HIGH_LAG_MS: u64 = 5_000;

/// Queue occupancy that means drops are imminent. Past this the producer
/// is outrunning Tantivy and only a lull will save the difference.
const QUEUE_SATURATION_PCT: u64 = 80;

/// Assemble the health document.
///
/// Labels come from the already-reconciled catalog registry, resolved
/// through the same `VolumeMap` the index stamps rows with — rather than
/// from a fresh `volumes::detect()`. The panel polls every two seconds,
/// and on Windows a detect walks all 26 drive letters with blocking
/// Win32 calls that can spin up removable media; that is far too heavy a
/// thing to hang off a telemetry read, and re-deriving "is this the same
/// root" here would be a third answer to a question `VolumeMap` already
/// answers.
pub fn build(
    snapshots: Vec<WatcherSnapshot>,
    catalogs: &CatalogRegistry,
    volume_map: &VolumeMap,
) -> IndexHealth {
    let volumes: Vec<VolumeHealth> = snapshots
        .into_iter()
        .map(|s| {
            let root = s.root.to_string_lossy().to_string();
            let label = catalogs
                .badge(&volume_map.resolve(&s.root))
                .map(|(name, _)| name.to_string())
                .unwrap_or_else(|| root.clone());
            let (monitoring, unavailable_reason) = match &s.state {
                WatcherState::Running => (true, None),
                WatcherState::Unavailable(why) => (false, Some(why.clone())),
            };
            VolumeHealth {
                root,
                label,
                monitoring,
                unavailable_reason,
                events_seen: s.events_seen,
                events_applied: s.events_applied,
                events_dropped: s.events_dropped,
                events_coalesced: s.events_coalesced,
                last_event_ms: s.last_event_ms,
                last_drop_ms: s.last_drop_ms,
                last_apply_ms: s.last_apply_ms,
                last_lag_ms: s.last_lag_ms,
                max_lag_ms: s.max_lag_ms,
                queue_depth: s.queue_depth,
                queue_capacity: s.queue_capacity,
                stream_reset: s.stream_reset,
            }
        })
        .collect();

    let advisories = advise(&volumes);
    IndexHealth {
        volumes,
        // The daemon runs no eager-extraction worker, so there is no
        // backlog to measure. Reported as absent rather than as zero so
        // the panel can say "not tracked" instead of implying "idle".
        extraction_backlog: None,
        advisories,
    }
}

/// Run every rule over every volume, worst first.
pub fn advise(volumes: &[VolumeHealth]) -> Vec<HealthAdvisory> {
    let mut out = Vec::new();

    for v in volumes {
        // A discarded stream and dropped events are the same class of
        // problem — the index is missing changes it will never see again
        // — and the same rescan fixes both.
        if v.stream_reset {
            out.push(HealthAdvisory {
                id: AdvisoryId::JournalStreamReset,
                severity: AdvisorySeverity::Critical,
                root: Some(v.root.clone()),
                count: 0,
                fix: AdvisoryFix::RebuildIndex,
            });
        }
        if v.events_dropped > 0 {
            out.push(HealthAdvisory {
                id: AdvisoryId::EventsDropped,
                severity: AdvisorySeverity::Critical,
                root: Some(v.root.clone()),
                count: v.events_dropped,
                fix: AdvisoryFix::RebuildIndex,
            });
        }
        if !v.monitoring {
            // No automatic fix: this is a privilege or filesystem
            // limitation, and a rebuild would not change it.
            out.push(HealthAdvisory {
                id: AdvisoryId::NotMonitoring,
                severity: AdvisorySeverity::Warning,
                root: Some(v.root.clone()),
                count: 0,
                fix: AdvisoryFix::None,
            });
            // The remaining rules all describe a live pipeline. Reporting
            // "0 ms lag" on a root nothing is watching would read as
            // healthy.
            continue;
        }
        if v.max_lag_ms > HIGH_LAG_MS {
            out.push(HealthAdvisory {
                id: AdvisoryId::HighLag,
                severity: AdvisorySeverity::Warning,
                root: Some(v.root.clone()),
                count: v.max_lag_ms,
                fix: AdvisoryFix::None,
            });
        }
        if v.queue_capacity > 0
            && v.queue_depth.saturating_mul(100) / v.queue_capacity >= QUEUE_SATURATION_PCT
        {
            out.push(HealthAdvisory {
                id: AdvisoryId::QueueSaturated,
                severity: AdvisorySeverity::Warning,
                root: Some(v.root.clone()),
                count: v.queue_depth,
                fix: AdvisoryFix::None,
            });
        }
    }

    // Worst first, stable within a severity so the panel does not reorder
    // itself between polls.
    out.sort_by_key(|a| std::cmp::Reverse(a.severity));
    out
}

#[cfg(test)]
mod tests {
    use super::*;

    fn healthy(root: &str) -> VolumeHealth {
        VolumeHealth {
            root: root.to_string(),
            label: root.to_string(),
            monitoring: true,
            unavailable_reason: None,
            events_seen: 100,
            events_applied: 100,
            events_dropped: 0,
            events_coalesced: 0,
            last_event_ms: 1,
            last_drop_ms: 0,
            last_apply_ms: 1,
            last_lag_ms: 12,
            max_lag_ms: 40,
            queue_depth: 0,
            queue_capacity: 1000,
            stream_reset: false,
        }
    }

    #[test]
    fn a_healthy_volume_produces_no_advice() {
        assert!(advise(&[healthy("C:\\")]).is_empty());
    }

    #[test]
    fn dropped_events_are_critical_and_offer_a_rebuild() {
        let mut v = healthy("C:\\");
        v.events_dropped = 3412;
        let out = advise(&[v]);
        assert_eq!(out.len(), 1);
        assert_eq!(out[0].id, AdvisoryId::EventsDropped);
        assert_eq!(out[0].severity, AdvisorySeverity::Critical);
        assert_eq!(out[0].count, 3412);
        assert_eq!(out[0].fix, AdvisoryFix::RebuildIndex);
    }

    #[test]
    fn a_discarded_stream_is_critical_and_offers_a_rebuild() {
        let mut v = healthy("C:\\");
        v.stream_reset = true;
        let out = advise(&[v]);
        assert_eq!(out[0].id, AdvisoryId::JournalStreamReset);
        assert_eq!(out[0].fix, AdvisoryFix::RebuildIndex);
    }

    #[test]
    fn an_unwatched_volume_warns_once_and_says_nothing_about_its_idle_pipeline() {
        let mut v = healthy("D:\\");
        v.monitoring = false;
        v.unavailable_reason = Some("access denied".into());
        // Would otherwise trip the lag rule too.
        v.max_lag_ms = 60_000;
        let out = advise(&[v]);
        assert_eq!(out.len(), 1);
        assert_eq!(out[0].id, AdvisoryId::NotMonitoring);
        assert_eq!(out[0].fix, AdvisoryFix::None);
    }

    #[test]
    fn lag_only_fires_above_the_threshold() {
        let mut v = healthy("C:\\");
        v.max_lag_ms = HIGH_LAG_MS;
        assert!(advise(&[v.clone()]).is_empty());
        v.max_lag_ms = HIGH_LAG_MS + 1;
        assert_eq!(advise(&[v]).len(), 1);
    }

    #[test]
    fn queue_saturation_fires_at_the_threshold_and_survives_a_zero_capacity() {
        let mut v = healthy("C:\\");
        v.queue_depth = 800;
        v.queue_capacity = 1000;
        assert_eq!(advise(&[v.clone()])[0].id, AdvisoryId::QueueSaturated);
        v.queue_depth = 799;
        assert!(advise(&[v.clone()]).is_empty());
        // A watcher that never opened reports capacity 0 — must not divide.
        v.queue_capacity = 0;
        v.queue_depth = 0;
        assert!(advise(&[v]).is_empty());
    }

    #[test]
    fn the_worst_advisory_sorts_first() {
        let mut a = healthy("C:\\");
        a.max_lag_ms = 90_000;
        let mut b = healthy("D:\\");
        b.events_dropped = 5;
        let out = advise(&[a, b]);
        assert_eq!(out[0].severity, AdvisorySeverity::Critical);
        assert_eq!(out[0].root.as_deref(), Some("D:\\"));
    }

    #[test]
    fn every_rule_can_fire_for_one_volume_at_once() {
        let mut v = healthy("C:\\");
        v.stream_reset = true;
        v.events_dropped = 10;
        v.max_lag_ms = 99_000;
        v.queue_depth = 1000;
        let out = advise(&[v]);
        assert_eq!(out.len(), 4);
        assert_eq!(out[0].severity, AdvisorySeverity::Critical);
        assert_eq!(out[1].severity, AdvisorySeverity::Critical);
    }
}
