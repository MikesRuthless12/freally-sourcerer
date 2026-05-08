//! Thin safe wrappers around the FSEvents + CoreFoundation calls we need,
//! plus `statfs` for filesystem-name diagnostics. macOS-only by
//! `#![cfg(target_os = "macos")]`; the rest of the crate falls back to a
//! typed stub on non-macOS so workspace builds (clippy / cargo check)
//! never see this module on Windows + Linux.

#![cfg(target_os = "macos")]

use std::ffi::{CStr, CString, c_void};
use std::io;
use std::os::unix::ffi::OsStrExt;
use std::path::{Path, PathBuf};
use std::ptr;

use core_foundation::array::{CFArray, CFArrayRef};
use core_foundation::base::TCFType;
use core_foundation::runloop::{CFRunLoop, CFRunLoopRef};
use core_foundation::string::{CFString, CFStringRef};

// CF run-loop pump + stop come from core-foundation-sys (typed pointers).
// fsevent-sys 4.x's `core_foundation` re-exports use `*mut c_void` for CF
// types and don't expose `CFRunLoopRunInMode`, so we go to the typed
// canonical bindings instead.
use core_foundation_sys::runloop::{
    CFRunLoopRunInMode, CFRunLoopStop, kCFRunLoopDefaultMode, kCFRunLoopRunFinished,
    kCFRunLoopRunStopped, kCFRunLoopRunTimedOut,
};

// fsevent-sys 4.x's FSEvents bindings use `*mut c_void` everywhere CF
// types appear, so we cast typed CF pointers down to `*mut c_void` at
// each call site.
use fsevent_sys::{
    FSEventStreamContext, FSEventStreamCreate, FSEventStreamEventFlags, FSEventStreamEventId,
    FSEventStreamInvalidate, FSEventStreamRef, FSEventStreamRelease,
    FSEventStreamScheduleWithRunLoop, FSEventStreamStart, FSEventStreamStop,
    kFSEventStreamCreateFlagFileEvents, kFSEventStreamCreateFlagNoDefer,
    kFSEventStreamCreateFlagUseCFTypes, kFSEventStreamCreateFlagWatchRoot,
};

/// Composite create-flags Phase 2 always passes to `FSEventStreamCreate`:
/// per-file granularity, no defer (deliver as soon as the latency window
/// elapses), CF-typed paths in the callback, and `WatchRoot` so we get
/// `RootChanged` notifications when the watched directory is moved.
pub const SOURCERER_STREAM_CREATE_FLAGS: u32 = kFSEventStreamCreateFlagFileEvents
    | kFSEventStreamCreateFlagNoDefer
    | kFSEventStreamCreateFlagUseCFTypes
    | kFSEventStreamCreateFlagWatchRoot;

/// Build a one-element `CFArray<CFString>` containing the watch root and
/// hand back the underlying `CFArrayRef` plus the owner. The caller must
/// keep the `CFArray` alive for the duration of the FSEvents stream.
pub fn paths_array_for(root: &Path) -> Option<CFArray<CFString>> {
    let s = root.to_str()?;
    Some(CFArray::from_CFTypes(&[CFString::new(s)]))
}

/// Calls `FSEventStreamCreate` with Sourcerer's standard flag set.
///
/// # Safety
///
/// `context` must point at a valid `FSEventStreamContext` whose `info`
/// field outlives the stream. `paths_to_watch` must be a valid CF array
/// of CF strings. The returned pointer is owned by the caller (release
/// via `FSEventStreamRelease`).
pub unsafe fn create_stream(
    callback: extern "C" fn(
        FSEventStreamRef,
        *mut c_void,
        usize,
        *mut c_void,
        *const FSEventStreamEventFlags,
        *const FSEventStreamEventId,
    ),
    context: *mut FSEventStreamContext,
    paths_to_watch: CFArrayRef,
    since_when: u64,
    latency_secs: f64,
) -> FSEventStreamRef {
    // SAFETY: caller-provided invariants documented above. fsevent-sys 4.x
    // expects `*mut c_void` for CF types, so we cast at the boundary.
    unsafe {
        FSEventStreamCreate(
            ptr::null_mut(),
            callback,
            context,
            paths_to_watch as *mut c_void,
            since_when,
            latency_secs,
            SOURCERER_STREAM_CREATE_FLAGS,
        )
    }
}

/// Schedules + starts the stream on the current thread's CFRunLoop.
/// Returns false on `FSEventStreamStart` failure; the caller should
/// release the stream and surface a `JournalError::StreamStartFailed`.
///
/// # Safety
///
/// `stream` must be a non-null `FSEventStreamRef` returned by
/// [`create_stream`] (or `FSEventStreamCreate`) and not yet released.
pub unsafe fn schedule_and_start(stream: FSEventStreamRef) -> bool {
    // SAFETY: caller invariants: non-null, unreleased stream pointer.
    // fsevent-sys 4.x's schedule fn takes `*mut c_void` for runLoop +
    // runLoopMode, so we cast typed CF pointers at the boundary.
    unsafe {
        let run_loop = CFRunLoop::get_current();
        FSEventStreamScheduleWithRunLoop(
            stream,
            run_loop.as_concrete_TypeRef() as *mut c_void,
            kCFRunLoopDefaultMode as *mut c_void,
        );
        FSEventStreamStart(stream) != 0
    }
}

/// Stops + invalidates + releases an FSEvents stream. Idempotent over a
/// null pointer.
///
/// # Safety
///
/// If non-null, `stream` must be an unreleased `FSEventStreamRef` returned
/// by [`create_stream`].
pub unsafe fn teardown_stream(stream: FSEventStreamRef) {
    if stream.is_null() {
        return;
    }
    // SAFETY: caller invariants: non-null, unreleased stream pointer.
    unsafe {
        FSEventStreamStop(stream);
        FSEventStreamInvalidate(stream);
        FSEventStreamRelease(stream);
    }
}

/// Stops the run loop running on the thread that holds `run_loop`. Called
/// from a control thread (typically `JournalSubscriber::drop`). Safe to
/// call repeatedly; the second call is a no-op once the loop has exited.
///
/// # Safety
///
/// `run_loop` must be a `CFRunLoopRef` that is still alive on its owning
/// thread. CoreFoundation's run-loop refcounting keeps the run-loop
/// instance alive until its thread exits, so passing the value across
/// thread boundaries is sound.
pub unsafe fn signal_stop(run_loop: CFRunLoopRef) {
    // SAFETY: run-loop pointer is valid for the lifetime of its thread;
    // CFRunLoopStop is itself thread-safe per Apple's CF docs. The
    // typed CFRunLoopRef from core_foundation matches core-foundation-sys's
    // CFRunLoopStop signature, so no cast is needed.
    unsafe { CFRunLoopStop(run_loop) }
}

/// Pumps the current run loop until it is stopped. Returns true if
/// `CFRunLoopStop` ended the loop normally; false if it ran out of
/// sources first (we exit the subscriber on either path).
///
/// # Safety
///
/// Must be called on the thread that scheduled the FSEvents stream onto
/// its run loop. `cycle_seconds` controls how often the wait wakes up to
/// re-check `tx.is_closed()` — keep small (~1.0) so a dropped receiver
/// stops the subscriber promptly.
pub unsafe fn run_until_stopped(cycle_seconds: f64) -> RunLoopExit {
    // SAFETY: kCFRunLoopDefaultMode is a stable Apple constant; the call
    // returns one of the four documented status codes. The caller's outer
    // loop drives the re-pump cadence (see `subscribe_thread`).
    let status = unsafe { CFRunLoopRunInMode(kCFRunLoopDefaultMode, cycle_seconds, 0) };
    match status {
        x if x == kCFRunLoopRunStopped => RunLoopExit::Stopped,
        x if x == kCFRunLoopRunFinished => RunLoopExit::Finished,
        x if x == kCFRunLoopRunTimedOut => RunLoopExit::TimedOut,
        _ => RunLoopExit::Other(status),
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RunLoopExit {
    /// Loop ended via `CFRunLoopStop`.
    Stopped,
    /// Loop ran out of input sources before being told to stop.
    Finished,
    /// Loop hit the per-call timeout and is ready to be re-pumped.
    TimedOut,
    /// Apple returned an undocumented status — surface as-is for logging.
    Other(i32),
}

/// Reads `f_fstypename` via `statfs(2)` for diagnostics-only filesystem
/// labeling on the cursor (e.g. `apfs`, `hfs`, `exfat`).
pub fn statfs_name(path: &Path) -> io::Result<String> {
    let c = CString::new(path.as_os_str().as_bytes())
        .map_err(|e| io::Error::new(io::ErrorKind::InvalidInput, e))?;
    // SAFETY: out-buffer is sized by libc; statfs() writes via pointer.
    let mut buf: libc::statfs = unsafe { std::mem::zeroed() };
    let r = unsafe { libc::statfs(c.as_ptr(), &mut buf) };
    if r != 0 {
        return Err(io::Error::last_os_error());
    }
    // SAFETY: f_fstypename is a NUL-terminated [c_char; 16] in the
    // statfs struct; CStr::from_ptr stops at the NUL and copies into
    // a Rust String via to_string_lossy.
    let raw = unsafe { CStr::from_ptr(buf.f_fstypename.as_ptr()) };
    Ok(raw.to_string_lossy().into_owned())
}

/// Reads `stat.st_dev` for the given path. Used to lock cursor reuse to
/// the exact volume the cursor was minted on.
pub fn device_id(path: &Path) -> io::Result<u64> {
    let meta = std::fs::symlink_metadata(path)?;
    use std::os::unix::fs::MetadataExt;
    Ok(meta.dev())
}

/// Decode a CFString-typed pointer pulled from FSEvents'
/// `eventPaths: CFArrayRef of CFString` callback argument into an owned
/// `PathBuf`. Returns `None` if the pointer is null or the string
/// transcoding fails — both indicate a CoreFoundation invariant violation
/// and the subscriber drops the corresponding event.
///
/// # Safety
///
/// `cf_str` must be a non-null `CFStringRef` that is alive for the
/// duration of this call. Because we use `wrap_under_get_rule`, the
/// caller does NOT transfer ownership; CFRetain is called internally.
pub unsafe fn cfstring_to_pathbuf(cf_str: CFStringRef) -> Option<PathBuf> {
    if cf_str.is_null() {
        return None;
    }
    // SAFETY: caller invariants: non-null, alive CFString.
    let owned = unsafe { CFString::wrap_under_get_rule(cf_str) };
    Some(PathBuf::from(owned.to_string()))
}
