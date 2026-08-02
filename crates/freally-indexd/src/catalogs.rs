//! SRC-M14 — named catalogs for volumes that are not plugged in.
//!
//! Unplugging a drive does not remove its rows from the index — nothing
//! in the daemon has ever pruned them. What was missing is the *identity*
//! that makes those rows useful: once the device is gone, a hit's path is
//! all that survives, and a path cannot answer "which drive was that file
//! on?". A catalog is the small piece of state that can: the volume id
//! the rows are stamped with, the label the device had when it was last
//! seen, and when that was.
//!
//! v1 is deliberately just the registry plus the badge and `volume:`
//! filter it feeds. The catalog *manager* — user notes, per-catalog
//! stats, export/import, merge and forget — is SRC-N69.

use std::collections::BTreeMap;

use freally_rpc::{CatalogInfo, VolumeInfo, VolumeStatus};
use serde::{Deserialize, Serialize};

/// One device Freally has indexed, online or not.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Catalog {
    /// Volume id — the same value stamped onto every row from this
    /// device, and what `volume:` matches against.
    pub id: String,
    /// Display name. Seeded from the volume label the first time the
    /// device is seen; SRC-N69 lets the user change it.
    pub name: String,
    /// Where it was mounted when last seen. Kept for display only — a
    /// removable device can come back on a different mount point.
    pub mount_point: String,
    pub fs_kind: String,
    pub first_seen_ms: u64,
    pub last_seen_ms: u64,
    /// Whether the device was mounted at the last reconcile.
    pub online: bool,
}

/// Every catalog the daemon knows about, keyed by volume id.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct CatalogRegistry {
    #[serde(default)]
    pub catalogs: BTreeMap<String, Catalog>,
}

impl freally_query::VolumeCatalogs for CatalogRegistry {
    fn resolve(&self, needle: &str) -> Vec<String> {
        self.resolve_filter(needle)
    }
}

impl CatalogRegistry {
    /// Fold the currently-mounted volumes in: anything detected is
    /// online and has its label and mount point refreshed, anything
    /// previously known but absent is marked offline and kept.
    ///
    /// Returns the ids that went offline during *this* call, so the
    /// caller can log the transition rather than re-deriving it.
    pub fn reconcile(&mut self, detected: &[VolumeInfo], now_ms: u64) -> Vec<String> {
        let mut went_offline = Vec::new();

        for v in detected {
            // A volume the OS reports as offline is mounted-but-unreadable
            // (an empty card reader, a disconnected network share). It is
            // not evidence the device is present.
            if v.status == VolumeStatus::Offline {
                continue;
            }
            self.catalogs
                .entry(v.id.clone())
                .and_modify(|c| {
                    c.name = pick_name(&c.name, &v.label, &v.id);
                    c.mount_point = v.mount_point.clone();
                    c.fs_kind = v.fs_kind.clone();
                    c.last_seen_ms = now_ms;
                    c.online = true;
                })
                .or_insert_with(|| Catalog {
                    id: v.id.clone(),
                    name: pick_name("", &v.label, &v.id),
                    mount_point: v.mount_point.clone(),
                    fs_kind: v.fs_kind.clone(),
                    first_seen_ms: now_ms,
                    last_seen_ms: now_ms,
                    online: true,
                });
        }

        let present: std::collections::BTreeSet<&str> = detected
            .iter()
            .filter(|v| v.status != VolumeStatus::Offline)
            .map(|v| v.id.as_str())
            .collect();
        for c in self.catalogs.values_mut() {
            if c.online && !present.contains(c.id.as_str()) {
                c.online = false;
                went_offline.push(c.id.clone());
            }
        }
        went_offline
    }

    /// Badge data for a row stamped with `volume_id`: the catalog's
    /// display name and whether the device is currently attached.
    /// `None` for rows indexed before SRC-M14, whose volume is empty.
    pub fn badge(&self, volume_id: &str) -> Option<(&str, bool)> {
        if volume_id.is_empty() {
            return None;
        }
        self.catalogs
            .get(volume_id)
            .map(|c| (c.name.as_str(), !c.online))
    }

    /// Resolve a user-typed `volume:` value to the volume ids it means.
    /// Matches a catalog's display name or its id, case-insensitively
    /// and by substring, so `volume:orange` finds "Orange WD 4TB".
    pub fn resolve_filter(&self, needle: &str) -> Vec<String> {
        let needle = needle.trim().to_lowercase();
        if needle.is_empty() {
            return Vec::new();
        }
        self.catalogs
            .values()
            .filter(|c| {
                c.name.to_lowercase().contains(&needle) || c.id.to_lowercase().contains(&needle)
            })
            .map(|c| c.id.clone())
            .collect()
    }

    pub fn to_dto(&self) -> Vec<CatalogInfo> {
        self.catalogs
            .values()
            .map(|c| CatalogInfo {
                id: c.id.clone(),
                name: c.name.clone(),
                mount_point: c.mount_point.clone(),
                fs_kind: c.fs_kind.clone(),
                first_seen_ms: c.first_seen_ms,
                last_seen_ms: c.last_seen_ms,
                online: c.online,
            })
            .collect()
    }
}

/// Prefer a real label over a synthesized one. Detection falls back to
/// the mount point when a device reports no label, and an unlabelled USB
/// stick that later reports a name should adopt it — but a name the user
/// is already seeing should not be replaced by a worse one.
fn pick_name(current: &str, label: &str, id: &str) -> String {
    let label = label.trim();
    if !label.is_empty() {
        return label.to_string();
    }
    if !current.is_empty() {
        return current.to_string();
    }
    id.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn vol(id: &str, label: &str, status: VolumeStatus) -> VolumeInfo {
        VolumeInfo {
            id: id.to_string(),
            label: label.to_string(),
            mount_point: format!("/mnt/{id}"),
            fs_kind: "exfat".into(),
            used_bytes: 0,
            total_bytes: 1,
            status,
            indexed: true,
            journal_enabled: false,
            journal_buffer_kb: 0,
            allocation_delta_kb: None,
            include_only: None,
            load_recent_changes: false,
            monitor_changes: true,
        }
    }

    #[test]
    fn a_new_device_is_registered_with_its_label() {
        let mut r = CatalogRegistry::default();
        r.reconcile(&[vol("win-D", "Orange WD 4TB", VolumeStatus::Indexed)], 100);
        let c = &r.catalogs["win-D"];
        assert_eq!(c.name, "Orange WD 4TB");
        assert!(c.online);
        assert_eq!(c.first_seen_ms, 100);
    }

    #[test]
    fn unplugging_keeps_the_catalog_and_marks_it_offline() {
        let mut r = CatalogRegistry::default();
        r.reconcile(&[vol("win-D", "Orange WD 4TB", VolumeStatus::Indexed)], 100);
        let gone = r.reconcile(&[], 200);

        assert_eq!(gone, vec!["win-D"]);
        let c = &r.catalogs["win-D"];
        assert!(!c.online, "the device is gone");
        assert_eq!(c.name, "Orange WD 4TB", "but it keeps its name");
        assert_eq!(c.last_seen_ms, 100, "and when we last saw it");
    }

    #[test]
    fn going_offline_is_reported_once_not_on_every_poll() {
        let mut r = CatalogRegistry::default();
        r.reconcile(&[vol("win-D", "D", VolumeStatus::Indexed)], 100);
        assert_eq!(r.reconcile(&[], 200).len(), 1);
        assert!(
            r.reconcile(&[], 300).is_empty(),
            "already-offline catalogs are not a new transition"
        );
    }

    #[test]
    fn replugging_brings_the_same_catalog_back_online() {
        let mut r = CatalogRegistry::default();
        r.reconcile(&[vol("win-D", "Orange WD 4TB", VolumeStatus::Indexed)], 100);
        r.reconcile(&[], 200);
        // Removable media routinely comes back on a different letter or
        // mount point; the id is what keeps it the same catalog.
        let mut back = vol("win-D", "Orange WD 4TB", VolumeStatus::Indexed);
        back.mount_point = "/mnt/elsewhere".into();
        r.reconcile(&[back], 300);

        assert_eq!(r.catalogs.len(), 1, "not a second catalog for one device");
        let c = &r.catalogs["win-D"];
        assert!(c.online);
        assert_eq!(c.mount_point, "/mnt/elsewhere");
        assert_eq!(c.first_seen_ms, 100, "still the device we met first");
    }

    #[test]
    fn a_mounted_but_unreadable_volume_is_not_evidence_the_device_is_present() {
        let mut r = CatalogRegistry::default();
        r.reconcile(&[vol("win-E", "Card Reader", VolumeStatus::Indexed)], 100);
        r.reconcile(&[vol("win-E", "Card Reader", VolumeStatus::Offline)], 200);
        assert!(!r.catalogs["win-E"].online);
    }

    #[test]
    fn an_unlabelled_device_adopts_a_label_when_it_reports_one() {
        let mut r = CatalogRegistry::default();
        r.reconcile(&[vol("win-D", "", VolumeStatus::Indexed)], 100);
        assert_eq!(r.catalogs["win-D"].name, "win-D");
        r.reconcile(&[vol("win-D", "Orange WD 4TB", VolumeStatus::Indexed)], 200);
        assert_eq!(r.catalogs["win-D"].name, "Orange WD 4TB");
    }

    #[test]
    fn badges_mark_only_the_detached_devices() {
        let mut r = CatalogRegistry::default();
        r.reconcile(
            &[
                vol("win-C", "System", VolumeStatus::Indexed),
                vol("win-D", "Orange WD 4TB", VolumeStatus::Indexed),
            ],
            100,
        );
        r.reconcile(&[vol("win-C", "System", VolumeStatus::Indexed)], 200);

        assert_eq!(r.badge("win-C"), Some(("System", false)));
        assert_eq!(r.badge("win-D"), Some(("Orange WD 4TB", true)));
        assert_eq!(r.badge("win-Z"), None, "an unknown volume has no badge");
        assert_eq!(r.badge(""), None, "rows indexed before M14 have no badge");
    }

    #[test]
    fn volume_filter_matches_a_catalog_by_part_of_its_name() {
        let mut r = CatalogRegistry::default();
        r.reconcile(
            &[
                vol("win-D", "Orange WD 4TB", VolumeStatus::Indexed),
                vol("win-E", "Blue Backup", VolumeStatus::Indexed),
            ],
            100,
        );
        assert_eq!(r.resolve_filter("orange"), vec!["win-D"]);
        assert_eq!(r.resolve_filter("ORANGE"), vec!["win-D"]);
        assert_eq!(r.resolve_filter("win-E"), vec!["win-E"]);
        assert!(r.resolve_filter("nothing").is_empty());
        assert!(r.resolve_filter("  ").is_empty());
    }
}
