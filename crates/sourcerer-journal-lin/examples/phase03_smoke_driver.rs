//! Phase 3 smoke driver — invoked by `tests/smoke/phase_03_journal_lin.sh`.
//!
//! Performs the spec workload (1000 creates, 200 modifies, 100 renames,
//! 100 deletes by default) on a scratch directory under the user's
//! HOME, runs the inotify (or fanotify, when CAP_SYS_ADMIN is held)
//! subscriber concurrently, and asserts overlap with the workload at
//! the same 99% gate the Phase 2 smoke uses on macOS.
//!
//! On non-Linux hosts, `main` exits 0 immediately so the workspace's
//! `cargo build --workspace --all-targets` still passes; the smoke
//! shell-script is gated separately.

#[cfg(not(target_os = "linux"))]
fn main() {
    eprintln!("phase03_smoke_driver is Linux-only; exiting cleanly on non-Linux host.");
}

#[cfg(target_os = "linux")]
use std::collections::{HashMap, HashSet};
#[cfg(target_os = "linux")]
use std::fs::OpenOptions;
#[cfg(target_os = "linux")]
use std::io::Write;
#[cfg(target_os = "linux")]
use std::path::PathBuf;
#[cfg(target_os = "linux")]
use std::sync::{Arc, Mutex};
#[cfg(target_os = "linux")]
use std::time::{Duration, Instant};

#[cfg(target_os = "linux")]
use futures::StreamExt;
#[cfg(target_os = "linux")]
use sourcerer_journal_lin::{JournalEvent, open_with_cursor_root};

#[cfg(target_os = "linux")]
#[derive(Debug)]
struct Args {
    scratch: PathBuf,
    creates: usize,
    modifies: usize,
    renames: usize,
    deletes: usize,
    timeout: Duration,
    out_events: Option<PathBuf>,
}

#[cfg(target_os = "linux")]
fn parse_args() -> Args {
    let mut scratch: Option<PathBuf> = None;
    let mut creates = 1000usize;
    let mut modifies = 200usize;
    let mut renames = 100usize;
    let mut deletes = 100usize;
    let mut timeout_secs = 30u64;
    let mut out_events: Option<PathBuf> = None;

    let mut iter = std::env::args().skip(1);
    while let Some(flag) = iter.next() {
        let value = iter
            .next()
            .unwrap_or_else(|| panic!("flag `{flag}` is missing a value"));
        match flag.as_str() {
            "--scratch" => scratch = Some(PathBuf::from(value)),
            "--creates" => creates = value.parse().expect("--creates"),
            "--modifies" => modifies = value.parse().expect("--modifies"),
            "--renames" => renames = value.parse().expect("--renames"),
            "--deletes" => deletes = value.parse().expect("--deletes"),
            "--timeout-secs" => timeout_secs = value.parse().expect("--timeout-secs"),
            "--out-events" => out_events = Some(PathBuf::from(value)),
            other => panic!("unknown flag `{other}`"),
        }
    }
    Args {
        scratch: scratch.expect("--scratch is required"),
        creates,
        modifies,
        renames,
        deletes,
        timeout: Duration::from_secs(timeout_secs),
        out_events,
    }
}

#[cfg(target_os = "linux")]
fn main() {
    let args = parse_args();

    if !args.scratch.exists() {
        std::fs::create_dir_all(&args.scratch).expect("create scratch dir");
    }
    let canonical_scratch = args
        .scratch
        .canonicalize()
        .expect("canonicalize scratch path");

    // Cursor MUST live outside the watched tree; otherwise every
    // `cursor.save()` writes a `<file>.tmp` then renames it over
    // `<file>.json` — that's two events the subscriber would observe
    // inside its own watched root, polluting the smoke's coverage gate.
    let cursor_root = canonical_scratch
        .parent()
        .map(|p| {
            p.join(format!(
                "{}_cursors",
                canonical_scratch
                    .file_name()
                    .unwrap_or_default()
                    .to_string_lossy()
            ))
        })
        .unwrap_or_else(|| std::env::temp_dir().join("sourcerer-phase03-cursors"));

    let subscriber = match open_with_cursor_root(&canonical_scratch, &cursor_root) {
        Ok(s) => s,
        Err(e) => {
            eprintln!(
                "FAIL: open journal subscriber on `{}`: {e}",
                canonical_scratch.display()
            );
            std::process::exit(2);
        }
    };

    let cursor = subscriber.cursor();
    println!(
        "subscriber opened on {} (fs={}, device={}, backend={:?})",
        canonical_scratch.display(),
        cursor.fs_name,
        cursor.device,
        cursor.backend,
    );

    let collected: Arc<Mutex<Vec<JournalEvent>>> = Arc::new(Mutex::new(Vec::new()));
    let collector = collected.clone();
    let _drain = std::thread::spawn(move || {
        let mut stream = Box::pin(subscriber.subscribe());
        futures::executor::block_on(async move {
            while let Some(ev) = stream.next().await {
                collector.lock().unwrap().push(ev);
            }
        });
    });

    // Give the subscribe thread time to install its watches before the
    // workload starts. Without this the first ~100 creates can race the
    // recursive `add_watch` and miss events.
    std::thread::sleep(Duration::from_millis(500));

    // -- Workload --
    let workload_start = Instant::now();
    let mut created_paths = Vec::with_capacity(args.creates);
    for i in 0..args.creates {
        let p = canonical_scratch.join(format!("file-{i:05}.txt"));
        std::fs::write(&p, b"sourcerer phase 3 smoke").expect("write create");
        created_paths.push(p);
    }
    println!(
        "create {} files in {:?}",
        args.creates,
        workload_start.elapsed()
    );

    // Pause to let the Create batch settle before we issue Modifies.
    // Inotify is closer to real-time than FSEvents, but a short pause
    // here keeps the smoke comparable across platforms.
    std::thread::sleep(Duration::from_millis(300));

    for p in created_paths.iter().take(args.modifies) {
        std::fs::write(p, b"sourcerer phase 3 smoke - modified").expect("write modify");
    }

    let mut rename_pairs: Vec<(PathBuf, PathBuf)> = Vec::with_capacity(args.renames);
    for p in created_paths.iter().skip(args.modifies).take(args.renames) {
        let new = p.with_file_name(format!(
            "{}.renamed",
            p.file_name().unwrap().to_string_lossy()
        ));
        std::fs::rename(p, &new).expect("rename");
        rename_pairs.push((p.clone(), new));
    }

    let renamed: HashSet<&std::path::Path> =
        rename_pairs.iter().map(|(old, _)| old.as_path()).collect();
    let delete_targets: Vec<PathBuf> = created_paths
        .iter()
        .rev()
        .filter(|p| !renamed.contains(p.as_path()))
        .take(args.deletes)
        .cloned()
        .collect();
    for p in &delete_targets {
        std::fs::remove_file(p).expect("delete");
    }

    println!("workload finished in {:?}", workload_start.elapsed());

    // -- Wait for events --
    let want = HashMap::from([
        ("Create", args.creates),
        ("Modify", args.modifies),
        ("Rename", args.renames),
        ("Delete", args.deletes),
    ]);

    let mut last_log = Instant::now();
    let assertion_start = Instant::now();
    let deadline = assertion_start + args.timeout;
    let scratch_lower = canonical_scratch.to_string_lossy().to_lowercase();

    loop {
        let mut counts: HashMap<&'static str, usize> = HashMap::new();
        let total = {
            let evs = collected.lock().unwrap();
            for ev in evs.iter() {
                if !path_in_scope(ev, &scratch_lower) {
                    continue;
                }
                let key = match ev {
                    JournalEvent::Create { .. } => "Create",
                    JournalEvent::Modify { .. } => "Modify",
                    JournalEvent::Rename { .. } => "Rename",
                    JournalEvent::Delete { .. } => "Delete",
                    JournalEvent::AttrChange { .. } => continue,
                };
                *counts.entry(key).or_insert(0) += 1;
            }
            evs.len()
        };

        // Phase-3 acceptance gate: 99 % of expected counts must show.
        let met = want.iter().all(|(k, v)| {
            let got = counts.get(k).copied().unwrap_or(0);
            got * 100 >= *v * 99
        });
        if met {
            println!(
                "PASS: all event counts >= 99% met within {:?} ({total} events total)",
                assertion_start.elapsed()
            );
            if let Some(out) = &args.out_events {
                write_events(out, &collected.lock().unwrap()).ok();
            }
            return;
        }

        if Instant::now() > deadline {
            eprintln!(
                "FAIL: did not observe expected event counts within {:?}",
                args.timeout
            );
            for (k, want_n) in &want {
                let got = counts.get(k).copied().unwrap_or(0);
                eprintln!("  {k}: want {want_n}, got {got}");
            }
            if let Some(out) = &args.out_events {
                write_events(out, &collected.lock().unwrap()).ok();
            }
            std::process::exit(1);
        }

        if last_log.elapsed() >= Duration::from_secs(2) {
            print!("  progress:");
            for k in ["Create", "Modify", "Rename", "Delete"] {
                let got = counts.get(k).copied().unwrap_or(0);
                let want_n = want.get(k).unwrap();
                print!(" {k}={got}/{want_n}");
            }
            println!();
            last_log = Instant::now();
        }

        std::thread::sleep(Duration::from_millis(100));
    }
}

#[cfg(target_os = "linux")]
fn path_in_scope(ev: &JournalEvent, scratch_lower: &str) -> bool {
    let path_lower = match ev {
        JournalEvent::Rename { new_path, .. } => new_path.to_string_lossy().to_lowercase(),
        JournalEvent::Create { path, .. }
        | JournalEvent::Modify { path, .. }
        | JournalEvent::Delete { path }
        | JournalEvent::AttrChange { path, .. } => path.to_string_lossy().to_lowercase(),
    };
    path_lower.starts_with(scratch_lower)
}

#[cfg(target_os = "linux")]
fn write_events(out: &PathBuf, events: &[JournalEvent]) -> std::io::Result<()> {
    let mut f = OpenOptions::new()
        .create(true)
        .write(true)
        .truncate(true)
        .open(out)?;
    for ev in events {
        let line = serde_json::to_string(ev).unwrap_or_default();
        writeln!(f, "{line}")?;
    }
    Ok(())
}
