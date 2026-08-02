//! Live change journaling — the piece that keeps the index fresh between
//! scans, and the telemetry SRC-M13's Index Health panel reports on.
//!
//! Until this module landed, the daemon only ever updated the index from a
//! full scan (`folders.add` / `folders.rescan` / `index.rebuild`). Every
//! OS subscriber already implemented `subscribe()` and `freally-index`
//! already shipped the bounded `EventQueue`, but nothing wired the two
//! together, so a file created after a scan stayed invisible until the
//! next one.
//!
//! Shape, one watcher per watched root:
//!
//! ```text
//!   subscribe() stream ──producer thread──▶ EventQueue ──consumer thread──▶ Index::apply + commit
//!                                              │                                    │
//!                                        drop ledger                          lag / applied
//! ```
//!
//! **Why the producer drops instead of blocking.** `EventQueue` offers
//! `push_blocking`, which never loses an event. Using it here would just
//! move the loss somewhere we cannot see it: the OS-side journal buffer
//! keeps filling while we block, and when *it* overflows the events are
//! gone with no record that they existed. Dropping at our own boundary
//! costs the same events but leaves a counted, timestamped ledger — which
//! is what lets the advisor say "3,412 events dropped on C: — rebuild
//! recommended" instead of silently serving a stale index.
//!
//! **Lag.** `oldest_pending_us` is stamped by the producer on the first
//! event of an idle stretch and cleared by the consumer after the commit
//! that makes it query-visible. The difference is exactly the
//! event → query-visible latency M13 asks for, in one atomic, with no
//! side-queue of timestamps to keep in lockstep.

use std::collections::BTreeMap;
use std::path::{Path, PathBuf};
use std::sync::Arc;
use std::sync::atomic::{AtomicBool, AtomicU64, Ordering};
use std::time::{Duration, SystemTime, UNIX_EPOCH};

use freally_index::{EventQueue, Index};
use freally_journal::{JournalEvent, JournalSubscriber};
use parking_lot::Mutex;
use tracing::{info, warn};

/// Per-watcher queue capacity. Deliberately larger than
/// `freally_index::DEFAULT_CAPACITY` (10k): a burst on a system volume
/// (an installer, a `git checkout`, a build) routinely exceeds 10k events
/// faster than Tantivy can commit them, and every event past the cap is a
/// drop the user has to resolve with a rebuild.
const QUEUE_CAPACITY: usize = 65_536;

/// How many events one `Index::apply` + `commit` cycle handles.
const APPLY_BATCH: usize = 4_096;

/// How long the consumer parks waiting for events before re-checking the
/// stop flag.
const CONSUMER_POLL: Duration = Duration::from_millis(250);

/// Longest a batch waits before being published. Bounds the
/// event → query-visible lag during a sustained burst, where the queue
/// never drains and the "burst died down" condition never fires.
const COMMIT_INTERVAL: Duration = Duration::from_millis(1_000);

/// What a single watcher is doing right now.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum WatcherState {
    /// Subscriber opened; producer and consumer threads are live.
    Running,
    /// The OS refused to open a change stream here (no privileges, an
    /// unsupported filesystem, a platform with no subscriber). The root
    /// is still searchable from its last scan — it just won't self-update.
    Unavailable(String),
}

/// Live counters for one watched root. Every field is an atomic so
/// `index.health` can read a snapshot without ever blocking the ingest
/// path.
#[derive(Debug, Default)]
pub struct WatcherMetrics {
    /// Events the OS handed us.
    pub events_seen: AtomicU64,
    /// Events that reached the index.
    pub events_applied: AtomicU64,
    /// Events refused because the queue was full. Each one is a hole in
    /// the index that only a rescan can fill.
    pub events_dropped: AtomicU64,
    /// Redundant events folded into a later one for the same path.
    pub events_coalesced: AtomicU64,
    /// Unix ms of the most recent event from the OS. 0 = none yet.
    pub last_event_ms: AtomicU64,
    /// Unix ms of the most recent drop. 0 = none yet.
    pub last_drop_ms: AtomicU64,
    /// Unix ms of the most recent commit.
    pub last_apply_ms: AtomicU64,
    /// event → query-visible latency of the last committed batch, ms.
    pub last_lag_ms: AtomicU64,
    /// Worst latency seen since the watcher started, ms.
    pub max_lag_ms: AtomicU64,
    /// Unix µs of the oldest event still waiting. 0 = queue drained.
    oldest_pending_us: AtomicU64,
}

impl WatcherMetrics {
    fn note_event(&self) {
        self.events_seen.fetch_add(1, Ordering::Relaxed);
        self.last_event_ms.store(now_ms(), Ordering::Relaxed);
        // Only the *first* event of an idle stretch sets the clock; the
        // consumer clears it after the commit that publishes this batch.
        let _ = self.oldest_pending_us.compare_exchange(
            0,
            now_us(),
            Ordering::Relaxed,
            Ordering::Relaxed,
        );
    }

    fn note_drop(&self) {
        self.events_dropped.fetch_add(1, Ordering::Relaxed);
        self.last_drop_ms.store(now_ms(), Ordering::Relaxed);
    }

    /// Called after the commit that makes a batch query-visible.
    fn note_applied(&self, applied: u64, coalesced: u64) {
        self.events_applied.fetch_add(applied, Ordering::Relaxed);
        self.events_coalesced
            .fetch_add(coalesced, Ordering::Relaxed);
        self.last_apply_ms.store(now_ms(), Ordering::Relaxed);
        let oldest = self.oldest_pending_us.swap(0, Ordering::Relaxed);
        if oldest != 0 {
            let lag_ms = now_us().saturating_sub(oldest) / 1_000;
            self.last_lag_ms.store(lag_ms, Ordering::Relaxed);
            self.max_lag_ms.fetch_max(lag_ms, Ordering::Relaxed);
        }
    }
}

/// Immutable snapshot of one watcher, handed to the RPC layer.
#[derive(Debug, Clone)]
pub struct WatcherSnapshot {
    pub root: PathBuf,
    pub state: WatcherState,
    pub events_seen: u64,
    pub events_applied: u64,
    pub events_dropped: u64,
    pub events_coalesced: u64,
    pub last_event_ms: u64,
    pub last_drop_ms: u64,
    pub last_apply_ms: u64,
    pub last_lag_ms: u64,
    pub max_lag_ms: u64,
    pub queue_depth: u64,
    pub queue_capacity: u64,
    /// True once the OS threw the change stream away and re-seated us —
    /// a wrapped USN journal, a recreated FSEvents stream. Everything
    /// that happened in the gap is missing from the index.
    pub stream_reset: bool,
}

struct VolumeWatcher {
    root: PathBuf,
    /// Path prefixes this watcher cares about. On Windows the USN journal
    /// is per-volume, so one watcher covers every watched folder on that
    /// drive and filters by prefix. Empty means "everything under root".
    prefixes: Vec<String>,
    queue: EventQueue,
    metrics: Arc<WatcherMetrics>,
    state: WatcherState,
    stop: Arc<AtomicBool>,
    /// `None` when the subscriber could not be opened.
    subscriber: Option<Arc<JournalSubscriber>>,
    /// Stream generation observed at open. A later mismatch means the OS
    /// discarded the stream we were reading.
    opened_generation: u64,
    threads: Vec<std::thread::JoinHandle<()>>,
}

/// Owns every live watcher and reconciles them against the daemon's
/// watched-folder set.
pub struct WatcherSupervisor {
    index: Arc<Index>,
    watchers: Mutex<BTreeMap<PathBuf, VolumeWatcher>>,
}

impl WatcherSupervisor {
    pub fn new(index: Arc<Index>) -> Self {
        Self {
            index,
            watchers: Mutex::new(BTreeMap::new()),
        }
    }

    /// Start watchers for `folders`, stop any whose folder went away.
    /// Idempotent — safe to call on every `folders.*` mutation.
    pub fn reconcile(&self, folders: &[String]) {
        let wanted = watch_roots(folders);
        let mut live = self.watchers.lock();

        live.retain(|root, w| {
            if wanted.contains_key(root) {
                true
            } else {
                info!(root = %root.display(), "stopping watcher; root no longer indexed");
                w.shutdown();
                false
            }
        });

        for (root, prefixes) in wanted {
            match live.get_mut(&root) {
                // The producer thread captured its prefix list at spawn,
                // so a changed list only takes effect on a fresh
                // watcher. Restarting is cheap and only happens when the
                // folder set actually changed — assigning the field
                // in place would silently keep filtering on the old one.
                Some(w) if w.prefixes != prefixes => {
                    w.shutdown();
                    *w = self.start_watcher(root, prefixes);
                }
                Some(_) => {}
                None => {
                    let w = self.start_watcher(root.clone(), prefixes);
                    live.insert(root, w);
                }
            }
        }
    }

    /// Stop every watcher. Called on daemon shutdown.
    pub fn shutdown(&self) {
        let mut live = self.watchers.lock();
        for w in live.values_mut() {
            w.shutdown();
        }
        live.clear();
    }

    /// Snapshot every watcher for `index.health`.
    pub fn snapshot(&self) -> Vec<WatcherSnapshot> {
        let live = self.watchers.lock();
        live.values()
            .map(|w| {
                let m = &w.metrics;
                // Generation drift is checked here rather than polled on a
                // timer: it is sticky (once the OS drops a stream it stays
                // dropped until we re-open), so a lazy read loses only the
                // moment it happened, not the fact.
                let stream_reset = w
                    .subscriber
                    .as_ref()
                    .map(|s| {
                        let g = s.position().generation;
                        g != 0 && w.opened_generation != 0 && g != w.opened_generation
                    })
                    .unwrap_or(false);
                WatcherSnapshot {
                    root: w.root.clone(),
                    state: w.state.clone(),
                    events_seen: m.events_seen.load(Ordering::Relaxed),
                    events_applied: m.events_applied.load(Ordering::Relaxed),
                    events_dropped: m.events_dropped.load(Ordering::Relaxed),
                    events_coalesced: m.events_coalesced.load(Ordering::Relaxed),
                    last_event_ms: m.last_event_ms.load(Ordering::Relaxed),
                    last_drop_ms: m.last_drop_ms.load(Ordering::Relaxed),
                    last_apply_ms: m.last_apply_ms.load(Ordering::Relaxed),
                    last_lag_ms: m.last_lag_ms.load(Ordering::Relaxed),
                    max_lag_ms: m.max_lag_ms.load(Ordering::Relaxed),
                    queue_depth: w.queue.len() as u64,
                    queue_capacity: w.queue.capacity() as u64,
                    stream_reset,
                }
            })
            .collect()
    }

    fn start_watcher(&self, root: PathBuf, prefixes: Vec<String>) -> VolumeWatcher {
        let queue = EventQueue::new(QUEUE_CAPACITY);
        let metrics = Arc::new(WatcherMetrics::default());
        let stop = Arc::new(AtomicBool::new(false));

        let opened = freally_journal::open(&root);
        let (state, subscriber, opened_generation) = match opened {
            Ok(sub) => {
                let sub = Arc::new(sub);
                let gen_ = sub.position().generation;
                info!(root = %root.display(), "live journal watcher started");
                (WatcherState::Running, Some(sub), gen_)
            }
            Err(e) => {
                // Not an error the user must act on: an unwatchable root is
                // still fully searchable from its last scan.
                warn!(root = %root.display(), error = %e, "no live journal here; scans only");
                (WatcherState::Unavailable(e.to_string()), None, 0)
            }
        };

        let mut threads = Vec::new();
        if let Some(sub) = subscriber.clone() {
            threads.push(spawn_producer(
                root.clone(),
                prefixes.clone(),
                sub,
                queue.clone(),
                metrics.clone(),
                stop.clone(),
            ));
            threads.push(spawn_consumer(
                root.clone(),
                self.index.clone(),
                queue.clone(),
                metrics.clone(),
                stop.clone(),
            ));
        }

        VolumeWatcher {
            root,
            prefixes,
            queue,
            metrics,
            state,
            stop,
            subscriber,
            opened_generation,
            threads,
        }
    }
}

impl VolumeWatcher {
    fn shutdown(&mut self) {
        self.stop.store(true, Ordering::SeqCst);
        // Wakes the consumer's `wait_for_events` and makes the producer's
        // next `try_push` fail, so both loops observe the stop flag.
        self.queue.close();
        for t in self.threads.drain(..) {
            let _ = t.join();
        }
    }
}

impl Drop for WatcherSupervisor {
    fn drop(&mut self) {
        self.shutdown();
    }
}

fn spawn_producer(
    root: PathBuf,
    prefixes: Vec<String>,
    subscriber: Arc<JournalSubscriber>,
    queue: EventQueue,
    metrics: Arc<WatcherMetrics>,
    stop: Arc<AtomicBool>,
) -> std::thread::JoinHandle<()> {
    std::thread::Builder::new()
        .name("freally-indexd/journal-producer".into())
        .spawn(move || {
            use futures::StreamExt;
            use futures::executor::block_on;

            let stream = subscriber.subscribe();
            let mut stream = Box::pin(stream);
            while let Some(event) = block_on(stream.next()) {
                if stop.load(Ordering::Relaxed) {
                    break;
                }
                if !matches_prefixes(&event, &prefixes) {
                    continue;
                }
                metrics.note_event();
                if queue.try_push(event).is_err() {
                    metrics.note_drop();
                }
            }
            info!(root = %root.display(), "journal producer exited");
        })
        .expect("spawn journal producer")
}

fn spawn_consumer(
    root: PathBuf,
    index: Arc<Index>,
    queue: EventQueue,
    metrics: Arc<WatcherMetrics>,
    stop: Arc<AtomicBool>,
) -> std::thread::JoinHandle<()> {
    std::thread::Builder::new()
        .name("freally-indexd/journal-consumer".into())
        .spawn(move || {
            // Applied but not yet committed. `apply` is cheap; `commit`
            // is a Tantivy commit plus a full re-serialize of the name
            // index, so committing once per drained batch would rewrite
            // the whole index every time a single log line was written.
            let mut pending_applied: u64 = 0;
            let mut pending_coalesced: u64 = 0;
            let mut last_commit = std::time::Instant::now();

            loop {
                if stop.load(Ordering::Relaxed) {
                    // Deferring the commit means shutdown can arrive with
                    // work already handed to the writer but not yet
                    // published. Committing here is what keeps "applied"
                    // and "durable" the same set — dropping out of the
                    // loop would silently lose everything since the last
                    // commit.
                    if pending_applied > 0 {
                        match index.commit() {
                            Ok(()) => metrics.note_applied(pending_applied, pending_coalesced),
                            Err(e) => {
                                warn!(root = %root.display(), error = %e,
                                    "final commit failed; changes since the last commit are lost")
                            }
                        }
                    }
                    break;
                }
                if queue.wait_for_events(CONSUMER_POLL) {
                    let batch = queue.drain(APPLY_BATCH);
                    if !batch.is_empty() {
                        let raw = batch.len() as u64;
                        let events = coalesce(batch);
                        pending_coalesced += raw - events.len() as u64;
                        match index.apply(&events) {
                            Ok(()) => pending_applied += events.len() as u64,
                            Err(e) => {
                                warn!(root = %root.display(), error = %e, "apply failed; batch lost")
                            }
                        }
                    }
                }

                // Publish either when enough has piled up to be worth a
                // commit, or when the burst has died down. Either way the
                // wait is bounded by COMMIT_INTERVAL, so a lone change
                // still becomes searchable promptly.
                if pending_applied == 0 {
                    continue;
                }
                let due = pending_applied >= APPLY_BATCH as u64
                    || last_commit.elapsed() >= COMMIT_INTERVAL
                    || queue.is_empty();
                if !due {
                    continue;
                }
                if let Err(e) = index.commit() {
                    warn!(root = %root.display(), error = %e, "commit failed; batch not yet visible");
                    continue;
                }
                last_commit = std::time::Instant::now();
                metrics.note_applied(pending_applied, pending_coalesced);
                pending_applied = 0;
                pending_coalesced = 0;
            }
            info!(root = %root.display(), "journal consumer exited");
        })
        .expect("spawn journal consumer")
}

/// Fold a batch down to the events that still matter.
///
/// Only `Modify` and `AttrChange` are foldable: they carry the file's
/// whole current state, so an earlier one for the same path is fully
/// superseded by a later one. `Create` / `Delete` / `Rename` change
/// whether the row exists at all, and dropping any of them would corrupt
/// the index — they pass through untouched and act as barriers, so a
/// modify before a delete is never folded past it.
fn coalesce(batch: Vec<JournalEvent>) -> Vec<JournalEvent> {
    let mut keep = vec![true; batch.len()];
    {
        // Keys borrow from `batch`, so folding costs no allocation — the
        // scope ends before `batch` is consumed below.
        let mut last_foldable: std::collections::HashMap<&Path, usize> =
            std::collections::HashMap::new();
        for (i, ev) in batch.iter().enumerate() {
            match ev {
                JournalEvent::Modify { path, .. } | JournalEvent::AttrChange { path, .. } => {
                    if let Some(prev) = last_foldable.insert(path.as_path(), i) {
                        keep[prev] = false;
                    }
                }
                JournalEvent::Create { path, .. } | JournalEvent::Delete { path } => {
                    last_foldable.remove(path.as_path());
                }
                JournalEvent::Rename { old_path, new_path } => {
                    last_foldable.remove(old_path.as_path());
                    last_foldable.remove(new_path.as_path());
                }
            }
        }
    }

    batch
        .into_iter()
        .zip(keep)
        .filter_map(|(ev, k)| k.then_some(ev))
        .collect()
}

/// True when the event touches a path under one of `prefixes`. An empty
/// prefix list means the watcher covers its whole root.
fn matches_prefixes(event: &JournalEvent, prefixes: &[String]) -> bool {
    if prefixes.is_empty() {
        return true;
    }
    // On Windows one watcher covers a whole volume, so this runs for
    // every USN record on the drive — including the vast majority that
    // are filtered out. Compare in place rather than lowercasing a copy
    // of each path first.
    let under_any = |p: &Path| {
        let bytes = p.as_os_str().as_encoded_bytes();
        prefixes.iter().any(|pre| {
            bytes.len() >= pre.len() && bytes[..pre.len()].eq_ignore_ascii_case(pre.as_bytes())
        })
    };
    match event {
        JournalEvent::Create { path, .. }
        | JournalEvent::Modify { path, .. }
        | JournalEvent::Delete { path }
        | JournalEvent::AttrChange { path, .. } => under_any(path),
        // A rename *out* of the tree still matters: the old path has to
        // be removed from the index.
        JournalEvent::Rename { old_path, new_path } => under_any(old_path) || under_any(new_path),
    }
}

/// Map watched folders onto the roots a subscriber can actually be opened
/// on, plus the prefixes each root should filter to.
///
/// The one place the OS model genuinely differs: Windows' USN journal is
/// per-volume and can only be opened on a drive root, so several watched
/// folders on `C:` collapse into one watcher that filters by prefix.
/// FSEvents and inotify/fanotify watch an arbitrary directory subtree, so
/// each folder gets its own watcher and needs no filtering.
fn watch_roots(folders: &[String]) -> BTreeMap<PathBuf, Vec<String>> {
    let mut out: BTreeMap<PathBuf, Vec<String>> = BTreeMap::new();
    for f in folders {
        if f.trim().is_empty() {
            continue;
        }
        #[cfg(windows)]
        {
            if let Some(vol) = volume_root_of(Path::new(f)) {
                let mut prefix = f.clone();
                if !prefix.ends_with('\\') && !prefix.ends_with('/') {
                    prefix.push('\\');
                }
                out.entry(vol).or_default().push(prefix);
                continue;
            }
        }
        out.entry(PathBuf::from(f)).or_default();
    }
    out
}

/// Volume root (`C:\`) for a drive-letter path. `None` for UNC paths and
/// anything else the USN journal cannot be opened on.
#[cfg(windows)]
fn volume_root_of(p: &Path) -> Option<PathBuf> {
    let s = p.to_string_lossy();
    let b = s.as_bytes();
    (b.len() >= 2 && b[1] == b':' && b[0].is_ascii_alphabetic())
        .then(|| PathBuf::from(format!("{}:\\", b[0].to_ascii_uppercase() as char)))
}

pub(crate) fn now_ms() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|d| d.as_millis() as u64)
        .unwrap_or(0)
}

fn now_us() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|d| d.as_micros() as u64)
        .unwrap_or(0)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn create(p: &str) -> JournalEvent {
        JournalEvent::Create {
            path: PathBuf::from(p),
            size: 1,
            mtime_ns: 0,
            ctime_ns: 0,
            attrs: 0,
        }
    }

    fn modify(p: &str, size: u64) -> JournalEvent {
        JournalEvent::Modify {
            path: PathBuf::from(p),
            size,
            mtime_ns: 0,
            attrs: 0,
        }
    }

    #[test]
    fn coalesce_folds_repeated_modifies_keeping_the_last() {
        let out = coalesce(vec![modify("/a", 1), modify("/a", 2), modify("/a", 3)]);
        assert_eq!(out, vec![modify("/a", 3)]);
    }

    #[test]
    fn coalesce_keeps_modifies_for_distinct_paths() {
        let out = coalesce(vec![modify("/a", 1), modify("/b", 1)]);
        assert_eq!(out.len(), 2);
    }

    #[test]
    fn coalesce_never_folds_across_a_delete() {
        // Folding the first modify away would be harmless, but folding the
        // *second* one into it would resurrect a deleted file.
        let out = coalesce(vec![
            modify("/a", 1),
            JournalEvent::Delete {
                path: PathBuf::from("/a"),
            },
            create("/a"),
            modify("/a", 2),
        ]);
        assert_eq!(
            out,
            vec![
                modify("/a", 1),
                JournalEvent::Delete {
                    path: PathBuf::from("/a")
                },
                create("/a"),
                modify("/a", 2),
            ]
        );
    }

    #[test]
    fn coalesce_leaves_creates_and_renames_alone() {
        let evs = vec![
            create("/a"),
            create("/a"),
            JournalEvent::Rename {
                old_path: PathBuf::from("/a"),
                new_path: PathBuf::from("/b"),
            },
        ];
        assert_eq!(coalesce(evs.clone()), evs);
    }

    #[test]
    fn prefix_filter_accepts_descendants_and_rejects_siblings() {
        let prefixes = vec!["c:\\users\\me\\".to_string()];
        assert!(matches_prefixes(
            &create("C:\\Users\\Me\\deep\\file.txt"),
            &prefixes
        ));
        assert!(!matches_prefixes(&create("C:\\Windows\\x.dll"), &prefixes));
        // A rename *out* of the tree still matters: the old path has to be
        // removed from the index.
        assert!(matches_prefixes(
            &JournalEvent::Rename {
                old_path: PathBuf::from("C:\\Users\\Me\\a.txt"),
                new_path: PathBuf::from("C:\\Temp\\a.txt"),
            },
            &prefixes
        ));
    }

    #[test]
    fn empty_prefix_list_matches_everything() {
        assert!(matches_prefixes(&create("/anywhere"), &[]));
    }

    #[test]
    fn lag_is_measured_from_the_first_pending_event() {
        let m = WatcherMetrics::default();
        m.note_event();
        m.note_event();
        assert_ne!(m.oldest_pending_us.load(Ordering::Relaxed), 0);
        std::thread::sleep(Duration::from_millis(5));
        m.note_applied(2, 0);
        assert_eq!(m.oldest_pending_us.load(Ordering::Relaxed), 0);
        assert!(m.last_lag_ms.load(Ordering::Relaxed) >= 4);
        assert_eq!(m.events_applied.load(Ordering::Relaxed), 2);
    }

    #[test]
    fn drops_are_counted_and_timestamped() {
        let m = WatcherMetrics::default();
        m.note_drop();
        assert_eq!(m.events_dropped.load(Ordering::Relaxed), 1);
        assert_ne!(m.last_drop_ms.load(Ordering::Relaxed), 0);
    }

    #[cfg(windows)]
    #[test]
    fn windows_folders_collapse_onto_one_volume_watcher() {
        let roots = watch_roots(&[
            "C:\\Users\\Me\\Documents".into(),
            "C:\\Users\\Me\\Pictures".into(),
            "D:\\Media".into(),
        ]);
        assert_eq!(roots.len(), 2);
        assert_eq!(roots[&PathBuf::from("C:\\")].len(), 2);
        assert!(roots[&PathBuf::from("C:\\")].contains(&"C:\\Users\\Me\\Documents\\".to_string()));
        assert_eq!(roots[&PathBuf::from("D:\\")].len(), 1);
    }

    #[cfg(not(windows))]
    #[test]
    fn unix_folders_each_get_their_own_watcher() {
        let roots = watch_roots(&["/home/me/docs".into(), "/home/me/pics".into()]);
        assert_eq!(roots.len(), 2);
        assert!(roots[&PathBuf::from("/home/me/docs")].is_empty());
    }

    #[test]
    fn blank_folder_entries_are_ignored() {
        assert!(watch_roots(&["".into(), "   ".into()]).is_empty());
    }
}
