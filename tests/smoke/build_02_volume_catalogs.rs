//! Build 2 (v0.22.0) smoke — SRC-M14 offline removable-volume catalogs.
//!
//! OS-agnostic: hand-built journal events into a `tempfile` index with an
//! explicit `VolumeMap`, so the same gates run on every CI matrix entry
//! without needing a real removable drive.
//!
//! Gates:
//!   1. Rows are stamped with the volume they came from, resolved
//!      through real mount points rather than guessed from the path.
//!   2. `volume:` filters to one device, and does so by the catalog's
//!      display name — what the user actually knows the drive as — not
//!      only by the internal id.
//!   3. Rows indexed before M14 (empty volume) never match `volume:`,
//!      rather than matching everything.
//!   4. `volume:` with no value is a parse error, not a query that
//!      quietly returns the whole index.
//!   5. Unplugging a device keeps its rows searchable — the point of the
//!      feature — and the answer to "which drive was that on?" survives.

#![cfg(test)]

use std::path::PathBuf;

use freally_index::{Index, VolumeMap};
use freally_indexd::catalogs::CatalogRegistry;
use freally_journal::JournalEvent;
use freally_query::{ExecOpts, execute_with_catalogs, parse};
use freally_rpc::{VolumeInfo, VolumeStatus};
use tempfile::tempdir;

fn file(path: &str) -> JournalEvent {
    JournalEvent::Create {
        path: PathBuf::from(path),
        size: 1,
        mtime_ns: 0,
        ctime_ns: 0,
        attrs: 0,
    }
}

fn vol(id: &str, label: &str, mount: &str) -> VolumeInfo {
    VolumeInfo {
        id: id.to_string(),
        label: label.to_string(),
        mount_point: mount.to_string(),
        fs_kind: "exfat".into(),
        used_bytes: 0,
        total_bytes: 1,
        status: VolumeStatus::Indexed,
        indexed: true,
        journal_enabled: false,
        journal_buffer_kb: 0,
        allocation_delta_kb: None,
        include_only: None,
        load_recent_changes: false,
        monitor_changes: true,
    }
}

/// An index holding one file per device, plus one legacy row with no
/// volume, and the registry that names those devices.
fn fixture() -> (tempfile::TempDir, std::sync::Arc<Index>, CatalogRegistry) {
    let dir = tempdir().unwrap();
    let idx = Index::open(dir.path()).unwrap();

    let detected = vec![
        vol("ext-orange", "Orange WD 4TB", "/mnt/orange"),
        vol("ext-blue", "Blue Backup", "/mnt/blue"),
    ];
    idx.set_volume_map(VolumeMap::new(
        detected
            .iter()
            .map(|v| (PathBuf::from(&v.mount_point), v.id.clone())),
    ));
    idx.apply(&[
        file("/mnt/orange/holiday-photo.jpg"),
        file("/mnt/blue/holiday-photo.jpg"),
    ])
    .unwrap();

    // A row written before any map existed — the shape every pre-M14
    // index is full of.
    idx.set_volume_map(VolumeMap::default());
    idx.apply(&[file("/elsewhere/holiday-photo.jpg")]).unwrap();
    idx.commit().unwrap();

    let mut registry = CatalogRegistry::default();
    registry.reconcile(&detected, 1_000);

    (dir, idx, registry)
}

fn run(idx: &Index, registry: &CatalogRegistry, source: &str) -> Vec<String> {
    let q = parse(source).expect("query parses");
    let rs = execute_with_catalogs(idx, None, None, Some(registry), &q, ExecOpts::default())
        .expect("query executes");
    rs.into_all_rows()
        .into_iter()
        .map(|r| r.path.to_string_lossy().to_string())
        .collect()
}

#[test]
fn rows_are_stamped_with_the_device_they_came_from() {
    let (_d, idx, _r) = fixture();
    let q = parse("holiday-photo").unwrap();
    let rs = execute_with_catalogs(&idx, None, None, None, &q, ExecOpts::default()).unwrap();
    let mut got: Vec<(String, String)> = rs
        .into_all_rows()
        .into_iter()
        .map(|r| (r.path.to_string_lossy().to_string(), r.volume))
        .collect();
    got.sort();

    assert_eq!(
        got,
        vec![
            ("/elsewhere/holiday-photo.jpg".to_string(), String::new()),
            ("/mnt/blue/holiday-photo.jpg".to_string(), "ext-blue".into()),
            (
                "/mnt/orange/holiday-photo.jpg".to_string(),
                "ext-orange".into()
            ),
        ]
    );
}

#[test]
fn volume_filters_by_the_name_the_user_knows_the_drive_as() {
    let (_d, idx, registry) = fixture();
    assert_eq!(
        run(&idx, &registry, "holiday-photo volume:orange"),
        vec!["/mnt/orange/holiday-photo.jpg"]
    );
    // Case-insensitive, and one word out of a multi-word device name is
    // enough — which is how anyone actually refers to "Blue Backup".
    // (A quoted value with a space is not expressible here: the
    // tokenizer splits `volume:"Blue Backup"` on the space, the same
    // pre-existing limitation `path:` and `parent:` have.)
    assert_eq!(
        run(&idx, &registry, "holiday-photo volume:BACKUP"),
        vec!["/mnt/blue/holiday-photo.jpg"]
    );
    // The internal id keeps working for scripts and the CLI.
    assert_eq!(
        run(&idx, &registry, "holiday-photo volume:ext-blue"),
        vec!["/mnt/blue/holiday-photo.jpg"]
    );
}

#[test]
fn a_volume_that_matches_no_catalog_returns_nothing() {
    let (_d, idx, registry) = fixture();
    assert!(run(&idx, &registry, "holiday-photo volume:nosuchdrive").is_empty());
}

#[test]
fn rows_indexed_before_m14_never_match_a_volume_filter() {
    let (_d, idx, registry) = fixture();
    // The legacy row is findable by name...
    assert_eq!(run(&idx, &registry, "holiday-photo").len(), 3);
    // ...but has no device, so it cannot belong to one. Matching it
    // would tell the user a file is on a drive it is not on.
    for q in ["volume:orange", "volume:blue", "volume:ext-orange"] {
        let hits = run(&idx, &registry, &format!("holiday-photo {q}"));
        assert!(
            !hits.iter().any(|p| p.starts_with("/elsewhere")),
            "{q} matched a row with no volume"
        );
    }
}

#[test]
fn an_empty_volume_value_is_a_parse_error() {
    // Answering `volume:` with the whole index would hide the typo.
    assert!(parse("holiday-photo volume:").is_err());
}

#[test]
fn unplugging_a_drive_keeps_its_files_findable_and_still_named() {
    let (_d, idx, mut registry) = fixture();

    // Orange is detached; only Blue is still attached.
    let gone = registry.reconcile(&[vol("ext-blue", "Blue Backup", "/mnt/blue")], 2_000);
    assert_eq!(gone, vec!["ext-orange"]);

    // The whole point: the files are still there.
    assert_eq!(
        run(&idx, &registry, "holiday-photo volume:orange"),
        vec!["/mnt/orange/holiday-photo.jpg"],
        "an unplugged drive's rows must stay searchable"
    );

    // And "which drive was that file on?" is still answerable.
    assert_eq!(
        registry.badge("ext-orange"),
        Some(("Orange WD 4TB", true)),
        "offline, but still named"
    );
    assert_eq!(registry.badge("ext-blue"), Some(("Blue Backup", false)));
}
