//! `JournalSubscriber` — the Phase-2 macOS public entry point.
//!
//! `open()` resolves the watched root, loads the persisted cursor, and
//! readies (without starting) the subscriber. `bootstrap()` walks the tree
//! and emits synthetic `Create` events. `subscribe()` spins an FSEvents
//! stream on a dedicated thread's CFRunLoop and emits classified, settled
//! events.
//!
//! ## Threading model
//!
//! `subscribe()` spawns one thread per call. The thread:
//! 1. Stashes its own `CFRunLoopRef` so `JournalSubscriber::drop()` can
//!    stop it from another thread.
//! 2. Builds an FSEvents stream rooted at the watched directory with
//!    `latency = 0.5s`, `FileEvents | NoDefer | UseCFTypes | WatchRoot`.
//! 3. Schedules the stream on the run loop and starts it.
//! 4. Pumps the run loop in 1-second slices, polling `tx.is_closed()`
//!    between pumps so a dropped receiver stops us promptly.
//! 5. Tears down the stream on exit.
//!
//! The FSEvents callback runs on the run-loop thread. Inside the callback
//! we classify each event, do **per-batch rename pairing** (FSEvents emits
//! both halves of a rename pair within the same coalesce window in the
//! overwhelming majority of cases), handle `MustScanSubDirs` by rescanning
//! the affected subtree inline, and forward `JournalEvent`s through the
//! shared `mpsc::UnboundedSender`.
//!
//! ## Phase-2 trade-offs (documented for Phase 13's perf pass)
//!
//! - **Bootstrap walker** uses `std::fs::read_dir` recursively rather than
//!   `fts_open`. The Build Guide flags `fts_open` as a perf preference;
//!   on the Phase 11 5M-file dataset this is on the bench list. The
//!   walker emits the same `JournalEvent::Create` shape either way, so
//!   swapping is contained.
//! - **Rename pairing** is per-batch only. A rename split across two
//!   FSEvents batches (rare — both halves carry consecutive event IDs and
//!   typically arrive together) renders as `Delete` + `Create` instead of
//!   `Rename`. This is correctness-preserving for the index but loses
//!   the rename-link in the rare cross-batch case.

#![cfg(target_os = "macos")]

use std::collections::{HashMap, HashSet};
use std::ffi::c_void;
use std::path::{Path, PathBuf};
use std::sync::atomic::{AtomicBool, AtomicUsize, Ordering};
use std::sync::{Arc, Mutex};

use core_foundation::base::TCFType;
use core_foundation::string::CFStringRef;
use core_foundation_sys::array::{
    CFArrayGetCount, CFArrayGetValueAtIndex, CFArrayRef as CfArrayRefRaw,
};

use fsevent_sys::{
    FSEventStreamContext, FSEventStreamEventFlags, FSEventStreamEventId, FSEventStreamRef,
    kFSEventStreamEventIdSinceNow,
};

use futures::Stream;
use futures::channel::mpsc;

use crate::cursor::StreamCursor;
use crate::event::{JournalError, JournalEvent};
use crate::ffi::{
    RunLoopExit, cfstring_to_pathbuf, create_stream, device_id, paths_array_for, run_until_stopped,
    schedule_and_start, signal_stop, statfs_name, teardown_stream,
};
use crate::flags::{self, FlagKind};

/// 0.5 s coalesce window — Phase-2 spec value.
const FSEVENTS_LATENCY_SECS: f64 = 0.5;

/// Run-loop pump cycle. Short enough that a dropped receiver stops the
/// subscriber within ~1 second.
const RUN_LOOP_CYCLE_SECS: f64 = 1.0;

/// Per-watch subscriber. Cheap to hold; expensive operations only happen
/// when the caller invokes `bootstrap()` or `subscribe()`.
pub struct JournalSubscriber {
    root: PathBuf,
    cursor_root: PathBuf,
    cursor: Arc<Mutex<StreamCursor>>,
    /// Set by the subscribe-thread once it has captured its run loop;
    /// `Drop` reads it back to call `CFRunLoopStop` from a control thread.
    run_loop_ptr: Arc<AtomicUsize>,
    /// Belt-and-braces stop flag. `Drop` sets it; the subscribe-thread's
    /// run-loop slice polls it every cycle. We rely primarily on
    /// `CFRunLoopStop` for fast shutdown, but a host where the stop
    /// signal lands before the subscribe thread has captured its run
    /// loop (or where CFRunLoopStop drops a wakeup under load) would
    /// otherwise hang the subscribe thread until process exit; the
    /// AtomicBool-driven fallback bounds shutdown latency at one cycle
    /// (see `RUN_LOOP_CYCLE_SECS`).
    stop_flag: Arc<AtomicBool>,
}

impl JournalSubscriber {
    pub fn root(&self) -> &Path {
        &self.root
    }

    pub fn cursor(&self) -> StreamCursor {
        self.cursor.lock().expect("cursor mutex poisoned").clone()
    }
}

impl Drop for JournalSubscriber {
    fn drop(&mut self) {
        // Set the flag first so a subscribe-thread that observes it on
        // the next slice tick exits even if `signal_stop` below is a
        // no-op (race: thread hasn't yet captured its run loop).
        self.stop_flag.store(true, Ordering::SeqCst);
        let raw = self.run_loop_ptr.load(Ordering::SeqCst);
        if raw == 0 {
            return;
        }
        // SAFETY: the subscribe-thread sets `run_loop_ptr` before pumping
        // the loop; CFRunLoopStop is documented as thread-safe.
        unsafe { signal_stop(raw as _) }
    }
}

/// Opens a subscriber rooted at `root`. The path must be an absolute,
/// existing directory. Cursor lives under
/// `~/Library/Application Support/Sourcerer/cursors/`.
pub fn open(root: &Path) -> Result<JournalSubscriber, JournalError> {
    open_with_cursor_root(root, &StreamCursor::default_root())
}

/// `open()` variant that lets tests redirect cursor persistence to a
/// scratch directory.
pub fn open_with_cursor_root(
    root: &Path,
    cursor_root: &Path,
) -> Result<JournalSubscriber, JournalError> {
    if !root.is_absolute() {
        return Err(JournalError::InvalidRoot(root.to_path_buf()));
    }
    let canonical =
        std::fs::canonicalize(root).map_err(|e| JournalError::OpenRoot(root.to_path_buf(), e))?;
    if !canonical.is_dir() {
        return Err(JournalError::InvalidRoot(canonical));
    }

    let dev = device_id(&canonical).map_err(|e| JournalError::OpenRoot(canonical.clone(), e))?;
    let fs_name =
        statfs_name(&canonical).map_err(|e| JournalError::Statfs(canonical.clone(), e))?;

    let persisted = StreamCursor::load(cursor_root, &canonical)?;
    let cursor = match persisted {
        Some(c) if c.device == dev => StreamCursor {
            // Always refresh fs_name + root in case a remount changed the
            // user-visible label. Keep last_event_id + bootstrap_complete.
            root: canonical.clone(),
            device: c.device,
            last_event_id: c.last_event_id,
            fs_name: fs_name.clone(),
            bootstrap_complete: c.bootstrap_complete,
        },
        Some(stale) => {
            tracing::info!(
                root = %canonical.display(),
                old_device = stale.device,
                new_device = dev,
                "persisted FSEvents cursor is on a different device; resetting to SinceNow",
            );
            StreamCursor {
                root: canonical.clone(),
                device: dev,
                last_event_id: 0,
                fs_name: fs_name.clone(),
                bootstrap_complete: false,
            }
        }
        None => StreamCursor {
            root: canonical.clone(),
            device: dev,
            last_event_id: 0,
            fs_name: fs_name.clone(),
            bootstrap_complete: false,
        },
    };
    cursor.save(cursor_root)?;

    Ok(JournalSubscriber {
        root: canonical,
        cursor_root: cursor_root.to_path_buf(),
        cursor: Arc::new(Mutex::new(cursor)),
        run_loop_ptr: Arc::new(AtomicUsize::new(0)),
        stop_flag: Arc::new(AtomicBool::new(false)),
    })
}

impl JournalSubscriber {
    /// One-shot stream of synthetic `Create` events for every regular file
    /// under the watched root. Skips dotfile-system entries the caller has
    /// no interest in (e.g. `.DS_Store`, `.Trashes`). After the walk
    /// finishes, sets `cursor.bootstrap_complete = true`.
    pub fn bootstrap(&self) -> impl Stream<Item = JournalEvent> + Send + 'static {
        let (tx, rx) = mpsc::unbounded::<JournalEvent>();
        let root = self.root.clone();
        let cursor = self.cursor.clone();
        let cursor_root = self.cursor_root.clone();

        std::thread::Builder::new()
            .name("sourcerer-journal-mac/bootstrap".into())
            .spawn(move || {
                if let Err(err) = bootstrap_thread(&root, &tx) {
                    tracing::warn!(error = %err, "bootstrap walk failed");
                    return;
                }
                if let Ok(mut c) = cursor.lock() {
                    c.bootstrap_complete = true;
                    let _ = c.save(&cursor_root);
                }
            })
            .expect("spawn bootstrap thread");

        rx
    }

    /// Long-running stream of incremental events. Drop the receiver to
    /// stop the subscriber.
    pub fn subscribe(&self) -> impl Stream<Item = JournalEvent> + Send + 'static {
        let (tx, rx) = mpsc::unbounded::<JournalEvent>();
        let root = self.root.clone();
        let cursor = self.cursor.clone();
        let cursor_root = self.cursor_root.clone();
        let run_loop_ptr = self.run_loop_ptr.clone();
        let stop_flag = self.stop_flag.clone();

        std::thread::Builder::new()
            .name("sourcerer-journal-mac/subscribe".into())
            .spawn(move || {
                if let Err(err) = subscribe_thread(
                    &root,
                    cursor,
                    &cursor_root,
                    run_loop_ptr,
                    stop_flag,
                    tx.clone(),
                ) {
                    tracing::warn!(error = %err, "FSEvents subscribe loop exited");
                }
                drop(tx);
            })
            .expect("spawn subscribe thread");

        rx
    }
}

fn bootstrap_thread(
    root: &Path,
    tx: &mpsc::UnboundedSender<JournalEvent>,
) -> Result<(), JournalError> {
    walk_dir(root, tx).map_err(|e| JournalError::WalkFailed(root.to_path_buf(), e))
}

/// Recursive directory walk. Emits `JournalEvent::Create` for every
/// regular file. Skips symlinks (we don't follow them — that's a Phase 13
/// follow-up if a user opts in to symlink-traversal).
fn walk_dir(dir: &Path, tx: &mpsc::UnboundedSender<JournalEvent>) -> std::io::Result<()> {
    // BFS-ish iterative walk so a deeply-nested tree can't blow the stack.
    let mut stack: Vec<PathBuf> = vec![dir.to_path_buf()];
    while let Some(current) = stack.pop() {
        let read = match std::fs::read_dir(&current) {
            Ok(r) => r,
            Err(e) if e.kind() == std::io::ErrorKind::PermissionDenied => {
                tracing::trace!(dir = %current.display(), "skipping unreadable dir");
                continue;
            }
            Err(e) => return Err(e),
        };
        for entry in read {
            let entry = match entry {
                Ok(e) => e,
                Err(e) => {
                    tracing::trace!(error = %e, "skipping unreadable entry");
                    continue;
                }
            };
            let path = entry.path();
            // Use `file_type()` (lstat-equivalent — does NOT follow
            // symlinks) for the symlink check; `metadata()` would follow
            // and report the target's type, masking symlinks-to-dir as
            // regular dirs and risking traversal escape from the watch
            // root.
            let file_type = match entry.file_type() {
                Ok(ft) => ft,
                Err(_) => continue,
            };
            if file_type.is_symlink() {
                continue;
            }
            if file_type.is_dir() {
                stack.push(path);
                continue;
            }
            if !file_type.is_file() {
                continue;
            }
            let meta = match entry.metadata() {
                Ok(m) => m,
                Err(_) => continue,
            };
            let ev = file_create_event(&path, &meta);
            if tx.unbounded_send(ev).is_err() {
                return Ok(()); // receiver dropped — exit cleanly
            }
        }
    }
    Ok(())
}

fn file_create_event(path: &Path, meta: &std::fs::Metadata) -> JournalEvent {
    use std::os::unix::fs::MetadataExt;
    let mtime_ns = (i128::from(meta.mtime())) * 1_000_000_000 + i128::from(meta.mtime_nsec());
    let ctime_ns = (i128::from(meta.ctime())) * 1_000_000_000 + i128::from(meta.ctime_nsec());
    JournalEvent::Create {
        path: path.to_path_buf(),
        size: meta.len(),
        mtime_ns,
        ctime_ns,
        attrs: meta.mode(),
    }
}

fn modify_event_for(path: PathBuf) -> JournalEvent {
    let meta = std::fs::metadata(&path).ok();
    let (size, mtime_ns, attrs) = match meta {
        Some(m) => {
            use std::os::unix::fs::MetadataExt;
            let mtime_ns = (i128::from(m.mtime())) * 1_000_000_000 + i128::from(m.mtime_nsec());
            (m.len(), mtime_ns, m.mode())
        }
        None => (0, 0, 0),
    };
    JournalEvent::Modify {
        path,
        size,
        mtime_ns,
        attrs,
    }
}

/// RAII reclaim guard for `Box<CallbackContext>`. Holds the raw pointer
/// the FSEvents stream sees in its `clientCallBackInfo`; on drop (normal
/// exit *or* panic unwind) the box is reconstructed and freed. The
/// `release()` method moves out of the guard and returns the raw pointer
/// without freeing — used at the explicit teardown spot in
/// `subscribe_thread` once we know the stream has been stopped and
/// invalidated.
struct CtxBoxGuard {
    ptr: *mut CallbackContext,
}

impl CtxBoxGuard {
    fn new(ctx: Box<CallbackContext>) -> Self {
        Self {
            ptr: Box::into_raw(ctx),
        }
    }
    fn raw(&self) -> *mut CallbackContext {
        self.ptr
    }
}

impl Drop for CtxBoxGuard {
    fn drop(&mut self) {
        if !self.ptr.is_null() {
            // SAFETY: ptr came from `Box::into_raw` in `new()`, has not
            // been aliased with another Box, and the FSEvents stream
            // (which previously aliased the same pointer in its callback
            // context) is guaranteed-stopped by the teardown ordering in
            // `subscribe_thread` before the guard goes out of scope.
            let _ = unsafe { Box::from_raw(self.ptr) };
            self.ptr = std::ptr::null_mut();
        }
    }
}

/// Body of the subscribe thread: builds + runs the FSEvents stream until
/// the receiver is dropped.
fn subscribe_thread(
    root: &Path,
    cursor: Arc<Mutex<StreamCursor>>,
    cursor_root: &Path,
    run_loop_ptr: Arc<AtomicUsize>,
    stop_flag: Arc<AtomicBool>,
    tx: mpsc::UnboundedSender<JournalEvent>,
) -> Result<(), JournalError> {
    let paths =
        paths_array_for(root).ok_or_else(|| JournalError::InvalidRoot(root.to_path_buf()))?;

    // RAII guard for the heap-allocated callback context. The FSEvents
    // stream sees the raw pointer; the guard owns the box and reclaims it
    // on Drop — including a panic unwind anywhere below this point.
    let ctx_guard = CtxBoxGuard::new(Box::new(CallbackContext::new(tx.clone(), cursor.clone())));

    let mut stream_context = FSEventStreamContext {
        version: 0,
        info: ctx_guard.raw().cast::<c_void>(),
        retain: None,
        release: None,
        copy_description: None,
    };

    let since_when = match cursor.lock() {
        Ok(c) if c.last_event_id != 0 => c.last_event_id,
        _ => kFSEventStreamEventIdSinceNow,
    };

    // Capture the current thread's run loop so Drop can stop us.
    let run_loop_raw =
        core_foundation::runloop::CFRunLoop::get_current().as_concrete_TypeRef() as usize;
    run_loop_ptr.store(run_loop_raw, Ordering::SeqCst);

    // SAFETY: `paths` outlives the stream (we keep it on the stack until
    // teardown). `stream_context.info` is the guard-owned raw pointer.
    let stream: FSEventStreamRef = unsafe {
        create_stream(
            fsevents_callback,
            &mut stream_context,
            paths.as_concrete_TypeRef(),
            since_when,
            FSEVENTS_LATENCY_SECS,
        )
    };
    if stream.is_null() {
        run_loop_ptr.store(0, Ordering::SeqCst);
        // ctx_guard's Drop reclaims the box on return.
        return Err(JournalError::StreamCreateFailed(root.to_path_buf()));
    }

    // SAFETY: stream is non-null per the check above.
    let started = unsafe { schedule_and_start(stream) };
    if !started {
        // SAFETY: stream is non-null + unreleased; teardown is idempotent.
        unsafe { teardown_stream(stream) };
        run_loop_ptr.store(0, Ordering::SeqCst);
        return Err(JournalError::StreamStartFailed(root.to_path_buf()));
    }

    // Pump the run loop in slices so we can poll `tx.is_closed()` and
    // `stop_flag` between batches. The FSEvents callback itself can also
    // stop the loop when it observes a closed receiver, and Drop on
    // JournalSubscriber sets `stop_flag` + calls CFRunLoopStop.
    loop {
        // SAFETY: we are on the run-loop's owning thread.
        let exit = unsafe { run_until_stopped(RUN_LOOP_CYCLE_SECS) };
        match exit {
            RunLoopExit::Stopped | RunLoopExit::Finished => break,
            RunLoopExit::TimedOut => {
                if stop_flag.load(Ordering::SeqCst) {
                    break;
                }
                if tx.is_closed() {
                    break;
                }
                // Persist the latest seen event id at most once per slice,
                // so a long quiet period still flushes the cursor to disk
                // periodically. We release the mutex before the I/O so a
                // slow disk doesn't block other holders.
                if let Ok(c) = cursor.lock() {
                    let snapshot = c.clone();
                    drop(c);
                    let _ = snapshot.save(cursor_root);
                }
            }
            RunLoopExit::Other(code) => {
                tracing::warn!(code, "CFRunLoopRunInMode returned an unexpected status");
                break;
            }
        }
    }

    // SAFETY: stream is non-null + unreleased; teardown sequences stop /
    // invalidate / release per Apple's docs. After teardown returns, the
    // FSEvents framework has stopped invoking our callback, so the
    // `ctx_guard`'s Drop (which fires on return below) is now safe to run
    // with no aliasing concern.
    unsafe { teardown_stream(stream) };
    run_loop_ptr.store(0, Ordering::SeqCst);

    // Final cursor flush. We release the lock before the I/O so a slow
    // disk doesn't block other holders of the cursor mutex.
    if let Ok(c) = cursor.lock() {
        let snapshot = c.clone();
        drop(c);
        let _ = snapshot.save(cursor_root);
    }

    drop(ctx_guard); // explicit for clarity; Drop reclaims the box.
    Ok(())
}

/// Per-stream callback state. Lives in a `Box` for the duration of the
/// stream and is reclaimed on subscribe-thread exit.
struct CallbackContext {
    tx: mpsc::UnboundedSender<JournalEvent>,
    /// Shared cursor handle so each callback batch can advance
    /// `last_event_id` for the next subscribe() session. The on-disk
    /// flush happens in `subscribe_thread`'s run-loop slice.
    cursor: Arc<Mutex<StreamCursor>>,
    /// inode → last-known-path cache. Used by per-batch rename pairing
    /// when an `ItemRenamed` arrives whose pair-half is not in the same
    /// batch but matches a previously-seen inode.
    known_inodes: HashMap<u64, PathBuf>,
    /// Paths we've already emitted a `Create` for. FSEvents on
    /// macOS 10.13+ keeps `kFSEventStreamEventFlagItemCreated` set in
    /// every subsequent event for the same path while it remains on
    /// disk — see Apple's CoreServices/FSEvents.h. Without de-dup, a
    /// post-creation `ItemModified` arrives with `Created | Modified`
    /// in the bitmask and our flag classifier (precedence: Create >
    /// Modify) renders it as Create, swallowing the Modify the consumer
    /// is waiting for. We track seen paths here and demote second-and-
    /// subsequent Create-flagged events to Modify.
    seen_paths: HashSet<PathBuf>,
}

impl CallbackContext {
    fn new(tx: mpsc::UnboundedSender<JournalEvent>, cursor: Arc<Mutex<StreamCursor>>) -> Self {
        Self {
            tx,
            cursor,
            known_inodes: HashMap::new(),
            seen_paths: HashSet::new(),
        }
    }
}

extern "C" fn fsevents_callback(
    _stream_ref: FSEventStreamRef,
    info: *mut c_void,
    num_events: usize,
    event_paths: *mut c_void,
    event_flags: *const FSEventStreamEventFlags,
    event_ids: *const FSEventStreamEventId,
) {
    if info.is_null() || event_paths.is_null() || num_events == 0 {
        return;
    }
    // SAFETY: `info` was set to a Box::into_raw pointer in
    // subscribe_thread; the Box is alive for the duration of the stream.
    let ctx: &mut CallbackContext = unsafe { &mut *(info.cast::<CallbackContext>()) };

    if ctx.tx.is_closed() {
        // The CFRunLoop pumping us in slices will catch this on its next
        // RunLoopExit::TimedOut tick; here we just stop emitting.
        return;
    }

    // SAFETY: with kFSEventStreamCreateFlagUseCFTypes set, `event_paths`
    // is a `CFArrayRef of CFString`. The arrays of flags + ids are
    // `num_events` long.
    let paths_arr = event_paths as CfArrayRefRaw;
    let flags_slice: &[FSEventStreamEventFlags] =
        unsafe { std::slice::from_raw_parts(event_flags, num_events) };
    let ids_slice: &[FSEventStreamEventId] =
        unsafe { std::slice::from_raw_parts(event_ids, num_events) };

    // Sanity: the array length should match num_events. CoreFoundation
    // guarantees this in practice; treat a mismatch as "trust num_events"
    // and log once.
    // SAFETY: paths_arr is a valid CFArrayRef per the create-flag.
    let array_len = unsafe { CFArrayGetCount(paths_arr as _) } as usize;
    if array_len != num_events {
        tracing::warn!(
            array_len,
            num_events,
            "FSEvents callback: paths array length disagrees with num_events"
        );
    }
    let len = num_events.min(array_len);

    // First pass: classify, decode the path, partition into rename-halves
    // vs settled events.
    let mut decoded: Vec<DecodedEvent> = Vec::with_capacity(len);
    let mut rescan_dirs: Vec<PathBuf> = Vec::new();
    let mut max_event_id: u64 = 0;

    for i in 0..len {
        let flags = flags_slice[i];
        let id = ids_slice[i];
        if id > max_event_id {
            max_event_id = id;
        }

        // SAFETY: index < array_len; CF array stores CFStringRef pointers.
        let cf_ptr = unsafe { CFArrayGetValueAtIndex(paths_arr as _, i as isize) };
        let path = match unsafe { cfstring_to_pathbuf(cf_ptr as CFStringRef) } {
            Some(p) => p,
            None => continue,
        };

        let kind = flags::classify(flags);
        match kind {
            FlagKind::MustScanSubDirs => rescan_dirs.push(path),
            FlagKind::RootChanged => {
                // The watched root moved or vanished. Phase 2 logs and
                // continues; Phase 13 will add an auto-recover path.
                tracing::warn!(path = %path.display(), "FSEvents RootChanged");
            }
            FlagKind::Ignore => {}
            other => decoded.push(DecodedEvent {
                kind: other,
                flags,
                path,
            }),
        }
    }

    // Per-batch rename pairing. We pre-stat every rename-half once so the
    // partner search is allocation-free in-memory work — original draft
    // re-stat'd on every comparison, which is O(N²) syscalls for N halves
    // in a single batch.
    //
    // Pairing rule (preserved from the original): two halves are a pair
    // when exactly one of the two paths exists at callback time, or when
    // both exist and share the same inode (the rare overwrite case where
    // the kernel coalesced events from both halves before we observed
    // them). Otherwise emit Rename; an unpaired existing half is a
    // rename-from-outside (Create); an unpaired missing half is a
    // rename-to-outside (Delete).
    let mut emit: Vec<JournalEvent> = Vec::with_capacity(decoded.len());

    // Indices of decoded events that are RenameMaybe and not directory-only.
    // FSEvents preserves event-ID order within a batch, so this Vec is
    // already in the order Apple delivered the halves.
    let rename_idxs: Vec<usize> = decoded
        .iter()
        .enumerate()
        .filter_map(|(i, d)| {
            if d.kind == FlagKind::RenameMaybe && !flags::is_dir(d.flags) {
                Some(i)
            } else {
                None
            }
        })
        .collect();

    // Pre-stat each half exactly once: (decoded-idx, path_exists, inode).
    let halves: Vec<(usize, bool, Option<u64>)> = rename_idxs
        .iter()
        .map(|&i| {
            let path = &decoded[i].path;
            let exists = path.exists();
            let inode = if exists { inode_of(path) } else { None };
            (i, exists, inode)
        })
        .collect();

    let mut consumed = vec![false; halves.len()];
    for k in 0..halves.len() {
        if consumed[k] {
            continue;
        }
        let (i, exists_i, inode_i) = halves[k];

        // Forward search for the first available partner.
        let partner = (k + 1..halves.len()).find(|&p| {
            if consumed[p] {
                return false;
            }
            let (_, exists_p, inode_p) = halves[p];
            if exists_i && exists_p {
                inode_p == inode_i && inode_i.is_some()
            } else {
                exists_i != exists_p
            }
        });

        if let Some(p) = partner {
            consumed[k] = true;
            consumed[p] = true;
            let (j, _, _) = halves[p];
            let path_i = decoded[i].path.clone();
            let path_j = decoded[j].path.clone();
            let (old_path, new_path) = if exists_i {
                (path_j, path_i)
            } else {
                (path_i, path_j)
            };
            // Refresh inode + seen-path caches: drop the old path,
            // register the new.
            ctx.known_inodes.retain(|_, p| p != &old_path);
            ctx.seen_paths.remove(&old_path);
            if let Some(ino) = inode_of(&new_path) {
                ctx.known_inodes.insert(ino, new_path.clone());
            }
            ctx.seen_paths.insert(new_path.clone());
            emit.push(JournalEvent::Rename { old_path, new_path });
            continue;
        }

        // Singleton: render as Create (rename-into-tree) or Delete
        // (rename-out-of-tree).
        consumed[k] = true;
        let path = decoded[i].path.clone();
        if exists_i {
            if let Ok(meta) = std::fs::metadata(&path) {
                if meta.is_file() {
                    if let Some(ino) = inode_i {
                        ctx.known_inodes.insert(ino, path.clone());
                    }
                    if ctx.seen_paths.insert(path.clone()) {
                        emit.push(file_create_event(&path, &meta));
                    } else {
                        // We've already emitted Create for this path. The
                        // sticky `ItemCreated` bit just rode along with a
                        // post-creation modification — render as Modify.
                        emit.push(modify_event_for(path.clone()));
                    }
                }
            }
        } else {
            ctx.known_inodes.retain(|_, p| p != &path);
            ctx.seen_paths.remove(&path);
            emit.push(JournalEvent::Delete { path });
        }
    }

    // Pass for non-rename events.
    for d in decoded.iter() {
        if d.kind == FlagKind::RenameMaybe {
            continue;
        }
        // Drop directory-level events (we only index files).
        if flags::is_dir(d.flags) && !flags::is_file(d.flags) {
            continue;
        }
        match d.kind {
            FlagKind::Create => {
                if let Ok(meta) = std::fs::metadata(&d.path) {
                    if meta.is_file() {
                        if let Some(ino) = inode_of(&d.path) {
                            ctx.known_inodes.insert(ino, d.path.clone());
                        }
                        // Demote sticky-Create-bit re-events to Modify
                        // (see `seen_paths` doc on `CallbackContext`).
                        if ctx.seen_paths.insert(d.path.clone()) {
                            emit.push(file_create_event(&d.path, &meta));
                        } else {
                            emit.push(modify_event_for(d.path.clone()));
                        }
                    }
                }
            }
            FlagKind::Modify => emit.push(modify_event_for(d.path.clone())),
            FlagKind::Delete => {
                ctx.known_inodes.retain(|_, p| p != &d.path);
                ctx.seen_paths.remove(&d.path);
                emit.push(JournalEvent::Delete {
                    path: d.path.clone(),
                });
            }
            FlagKind::AttrChange => {
                let attrs = std::fs::metadata(&d.path)
                    .map(|m| {
                        use std::os::unix::fs::MetadataExt;
                        m.mode()
                    })
                    .unwrap_or(0);
                emit.push(JournalEvent::AttrChange {
                    path: d.path.clone(),
                    attrs,
                });
            }
            // Already handled above.
            FlagKind::RenameMaybe
            | FlagKind::MustScanSubDirs
            | FlagKind::RootChanged
            | FlagKind::Ignore => {}
        }
    }

    for ev in emit {
        if ctx.tx.unbounded_send(ev).is_err() {
            return;
        }
    }

    // Inline rescan on MustScanSubDirs.
    for dir in rescan_dirs {
        if !dir.is_dir() {
            continue;
        }
        let _ = walk_dir(&dir, &ctx.tx);
    }

    // Advance the in-memory cursor; subscribe_thread's run-loop slice
    // flushes it to disk. We never go backwards on `last_event_id`.
    if max_event_id != 0 {
        if let Ok(mut c) = ctx.cursor.lock() {
            if max_event_id > c.last_event_id {
                c.last_event_id = max_event_id;
            }
        }
    }
}

#[derive(Debug, Clone)]
struct DecodedEvent {
    kind: FlagKind,
    flags: FSEventStreamEventFlags,
    path: PathBuf,
}

fn inode_of(path: &Path) -> Option<u64> {
    use std::os::unix::fs::MetadataExt;
    std::fs::symlink_metadata(path).ok().map(|m| m.ino())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn modify_event_handles_missing_file() {
        let ev = modify_event_for(PathBuf::from("/nonexistent/path/that/should/not/exist"));
        match ev {
            JournalEvent::Modify {
                size,
                mtime_ns,
                attrs,
                ..
            } => {
                assert_eq!(size, 0);
                assert_eq!(mtime_ns, 0);
                assert_eq!(attrs, 0);
            }
            _ => panic!("expected Modify"),
        }
    }

    #[test]
    fn walk_dir_skips_symlinks_to_dirs() {
        // Stage:  scratch/real/inside-real.txt   (regular file)
        //         scratch/symlink -> real        (symlink to dir)
        //
        // walk_dir(scratch) must emit a Create for `inside-real.txt`
        // exactly once — under its real path. It must NOT emit a second
        // Create for `scratch/symlink/inside-real.txt`. A pre-fix
        // implementation that called `metadata()` (which follows symlinks)
        // before checking `is_symlink()` would silently traverse the
        // symlink and emit the duplicate.
        use futures::StreamExt;

        let scratch = tempfile::tempdir().expect("tempdir");
        let scratch_path = scratch
            .path()
            .canonicalize()
            .expect("canonicalize scratch tempdir");
        let real_dir = scratch_path.join("real");
        let real_file = real_dir.join("inside-real.txt");
        std::fs::create_dir(&real_dir).expect("mkdir real");
        std::fs::write(&real_file, b"content").expect("write real file");

        let symlink = scratch_path.join("symlink");
        std::os::unix::fs::symlink(&real_dir, &symlink).expect("symlink");

        let (tx, rx) = mpsc::unbounded::<JournalEvent>();
        walk_dir(&scratch_path, &tx).expect("walk_dir");
        drop(tx);

        let mut paths: Vec<PathBuf> = Vec::new();
        let mut stream = Box::pin(rx);
        while let Some(ev) = futures::executor::block_on(stream.next()) {
            if let JournalEvent::Create { path, .. } = ev {
                paths.push(path);
            }
        }

        assert!(
            paths.iter().any(|p| p == &real_file),
            "walk_dir must emit Create for the regular file under the real \
             dir; got: {paths:?}"
        );
        assert!(
            !paths.iter().any(|p| p.starts_with(&symlink)),
            "walk_dir must NOT traverse symlinks to dirs; got: {paths:?}"
        );
    }
}
