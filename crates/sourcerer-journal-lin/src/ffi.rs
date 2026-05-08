//! Thin safe wrappers around the inotify, fanotify, getdents64, statfs,
//! and capability syscalls Phase 3 needs. Linux-only by
//! `#![cfg(target_os = "linux")]`; the rest of the crate falls back to
//! typed-but-stubbed surfaces on non-Linux so workspace builds (clippy,
//! cargo check) on Windows / macOS hosts never see this module.

#![cfg(target_os = "linux")]

use std::ffi::{CStr, CString, OsStr, OsString};
use std::io;
use std::os::fd::{AsRawFd, FromRawFd, IntoRawFd, OwnedFd, RawFd};
use std::os::unix::ffi::{OsStrExt, OsStringExt};
use std::path::{Path, PathBuf};

// =====================================================================
// Inotify
// =====================================================================

/// Owned wrapper around an inotify fd. Closed via `nix::unistd::close`
/// in `Drop` so a panic anywhere upstream still releases the fd.
pub struct InotifyFd {
    inner: OwnedFd,
}

impl InotifyFd {
    /// `inotify_init1(IN_NONBLOCK | IN_CLOEXEC)`.
    ///
    /// `IN_NONBLOCK` lets us drain the queue without blocking when no
    /// events are ready (the subscribe loop polls with a timeout via
    /// `read` returning EAGAIN). `IN_CLOEXEC` keeps the fd from leaking
    /// across `exec()` if the daemon spawns helper processes.
    pub fn init() -> io::Result<Self> {
        // SAFETY: inotify_init1 is a thin syscall wrapper that returns -1
        // on error and sets errno; the returned fd is owned by us.
        let raw = unsafe { libc::inotify_init1(libc::IN_NONBLOCK | libc::IN_CLOEXEC) };
        if raw < 0 {
            return Err(io::Error::last_os_error());
        }
        // SAFETY: raw is a fresh fd we just opened; FromRawFd takes
        // ownership and the OwnedFd's Drop closes it for us.
        let inner = unsafe { OwnedFd::from_raw_fd(raw) };
        Ok(Self { inner })
    }

    /// `inotify_add_watch(fd, path, mask)`. Returns the watch descriptor.
    pub fn add_watch(&self, path: &Path, mask: u32) -> io::Result<i32> {
        let c = CString::new(path.as_os_str().as_bytes())
            .map_err(|e| io::Error::new(io::ErrorKind::InvalidInput, e))?;
        // SAFETY: c.as_ptr() is valid for the duration of the call; the
        // kernel does not retain it past return.
        let wd = unsafe { libc::inotify_add_watch(self.inner.as_raw_fd(), c.as_ptr(), mask) };
        if wd < 0 {
            return Err(io::Error::last_os_error());
        }
        Ok(wd)
    }

    /// `inotify_rm_watch(fd, wd)`. The kernel emits a final `IN_IGNORED`
    /// event for the wd before forgetting it; the subscriber consumes it
    /// and then prunes its in-memory wd→path table.
    pub fn rm_watch(&self, wd: i32) -> io::Result<()> {
        // SAFETY: wd is a value previously returned by add_watch on this
        // fd; the kernel rejects bogus wds with EINVAL.
        let r = unsafe { libc::inotify_rm_watch(self.inner.as_raw_fd(), wd) };
        if r < 0 {
            return Err(io::Error::last_os_error());
        }
        Ok(())
    }

    /// `read(fd, buf, len)` with EAGAIN translation.
    ///
    /// Returns `Ok(Some(n))` for `n` bytes consumed, `Ok(None)` when
    /// the fd is non-blocking and no events are ready, and `Err` on any
    /// other failure. The caller iterates the buffer with
    /// [`InotifyEventIter`] to extract individual `inotify_event`s.
    pub fn read_into(&self, buf: &mut [u8]) -> io::Result<Option<usize>> {
        // SAFETY: read is a thin syscall wrapper; buf is exclusive and
        // sized correctly via .len().
        let n = unsafe {
            libc::read(
                self.inner.as_raw_fd(),
                buf.as_mut_ptr().cast::<libc::c_void>(),
                buf.len(),
            )
        };
        if n < 0 {
            let err = io::Error::last_os_error();
            if matches!(
                err.raw_os_error(),
                Some(libc::EAGAIN) | Some(libc::EWOULDBLOCK)
            ) {
                return Ok(None);
            }
            return Err(err);
        }
        Ok(Some(n as usize))
    }

    pub fn raw(&self) -> RawFd {
        self.inner.as_raw_fd()
    }
}

/// Heap-decoded `inotify_event`. The kernel's on-the-wire layout is a
/// flexible-array struct, which Rust does not model directly; we copy
/// out into an owned record so the iterator can advance past it
/// without keeping the buffer slice live.
#[derive(Debug, Clone)]
pub struct ParsedInotifyEvent {
    pub wd: i32,
    pub mask: u32,
    /// Non-zero `cookie` pairs an `IN_MOVED_FROM` with its matching
    /// `IN_MOVED_TO`. Zero on every other event.
    pub cookie: u32,
    /// Trailing path component (relative to the watched dir). Empty on
    /// events that target the watched dir itself (e.g. `IN_IGNORED`,
    /// `IN_DELETE_SELF`).
    pub name: OsString,
}

/// Iterator over `inotify_event` records inside a `read()` buffer.
pub struct InotifyEventIter<'a> {
    buf: &'a [u8],
    offset: usize,
}

impl<'a> InotifyEventIter<'a> {
    pub fn new(buf: &'a [u8]) -> Self {
        Self { buf, offset: 0 }
    }
}

impl Iterator for InotifyEventIter<'_> {
    type Item = ParsedInotifyEvent;

    fn next(&mut self) -> Option<ParsedInotifyEvent> {
        // Layout per <sys/inotify.h>:
        //   int      wd;       // 4
        //   uint32_t mask;     // 4
        //   uint32_t cookie;   // 4
        //   uint32_t len;      // 4
        //   char     name[len];
        const HDR: usize = 16;
        if self.offset + HDR > self.buf.len() {
            return None;
        }
        let hdr = &self.buf[self.offset..self.offset + HDR];
        let wd = i32::from_ne_bytes(hdr[0..4].try_into().ok()?);
        let mask = u32::from_ne_bytes(hdr[4..8].try_into().ok()?);
        let cookie = u32::from_ne_bytes(hdr[8..12].try_into().ok()?);
        let len = u32::from_ne_bytes(hdr[12..16].try_into().ok()?) as usize;

        let total = HDR + len;
        if self.offset + total > self.buf.len() {
            return None;
        }
        let name = if len == 0 {
            OsString::new()
        } else {
            // Name is NUL-terminated and zero-padded to align the next
            // record. Strip everything from the first NUL.
            let bytes = &self.buf[self.offset + HDR..self.offset + HDR + len];
            let nul = bytes.iter().position(|&b| b == 0).unwrap_or(bytes.len());
            OsString::from_vec(bytes[..nul].to_vec())
        };

        self.offset += total;
        Some(ParsedInotifyEvent {
            wd,
            mask,
            cookie,
            name,
        })
    }
}

// =====================================================================
// Fanotify
// =====================================================================

/// Owned wrapper around a fanotify fd plus the per-mark "mount fd" we
/// later pass to `open_by_handle_at` for FID resolution.
pub struct FanotifyFd {
    inner: OwnedFd,
    /// Open fd on the watched root, used as the `mount_fd` argument to
    /// `open_by_handle_at`. Required by the kernel: the file_handle is
    /// only meaningful in the context of a filesystem reachable via
    /// some directory fd on that mount.
    mount: OwnedFd,
}

impl FanotifyFd {
    /// `fanotify_init(FAN_CLASS_NOTIF | FAN_REPORT_DFID_NAME |
    ///                FAN_NONBLOCK | FAN_CLOEXEC, O_RDONLY)`.
    ///
    /// `FAN_REPORT_DFID_NAME` is the request for parent-dir-FID + name
    /// info records on every event — this is what gives us proper
    /// rename tracking on Btrfs subvolumes and overlayfs (where inotify
    /// loses track of cross-subvolume renames). Returns `io::Error`
    /// with `EPERM` if the caller lacks `CAP_SYS_ADMIN`; the subscriber
    /// translates that into a graceful fall-through to the inotify
    /// path rather than a hard failure.
    pub fn init(root: &Path) -> io::Result<Self> {
        let init_flags =
            libc::FAN_CLASS_NOTIF | FAN_REPORT_DFID_NAME | libc::FAN_NONBLOCK | libc::FAN_CLOEXEC;
        // SAFETY: thin syscall wrapper; flag values are stable kernel ABI.
        let raw = unsafe { libc::fanotify_init(init_flags, libc::O_RDONLY as u32) };
        if raw < 0 {
            return Err(io::Error::last_os_error());
        }
        // SAFETY: raw is a fresh fd we just opened.
        let inner = unsafe { OwnedFd::from_raw_fd(raw) };

        // Open the root directory so we can use it as the dirfd anchor
        // for `open_by_handle_at`. O_PATH | O_DIRECTORY | O_CLOEXEC keeps
        // the fd cheap (no I/O permission, no fd leak across exec).
        let c = CString::new(root.as_os_str().as_bytes())
            .map_err(|e| io::Error::new(io::ErrorKind::InvalidInput, e))?;
        // SAFETY: c.as_ptr() is valid for the duration of the call.
        let mount_raw = unsafe {
            libc::open(
                c.as_ptr(),
                libc::O_PATH | libc::O_DIRECTORY | libc::O_CLOEXEC,
            )
        };
        if mount_raw < 0 {
            // inner's Drop closes the fanotify fd for us.
            return Err(io::Error::last_os_error());
        }
        // SAFETY: mount_raw is a fresh fd we just opened.
        let mount = unsafe { OwnedFd::from_raw_fd(mount_raw) };
        Ok(Self { inner, mount })
    }

    /// `fanotify_mark(fd, FAN_MARK_ADD | FAN_MARK_FILESYSTEM, mask, AT_FDCWD, root)`.
    ///
    /// `FAN_MARK_FILESYSTEM` registers a single mark covering every
    /// path on the volume that hosts `root` — far cheaper than the
    /// per-directory `inotify_add_watch` calls the unprivileged path
    /// has to do, and immune to the `IN_Q_OVERFLOW` storms a recursive
    /// inotify watcher hits on a busy filesystem.
    pub fn mark_filesystem(&self, root: &Path, mask: u64) -> io::Result<()> {
        let c = CString::new(root.as_os_str().as_bytes())
            .map_err(|e| io::Error::new(io::ErrorKind::InvalidInput, e))?;
        let flags: libc::c_uint = libc::FAN_MARK_ADD | libc::FAN_MARK_FILESYSTEM;
        // SAFETY: thin syscall wrapper; the kernel does not retain c past return.
        let r = unsafe {
            libc::fanotify_mark(
                self.inner.as_raw_fd(),
                flags,
                mask,
                libc::AT_FDCWD,
                c.as_ptr(),
            )
        };
        if r < 0 {
            return Err(io::Error::last_os_error());
        }
        Ok(())
    }

    /// `read(fd, buf, len)` with EAGAIN translation. Same shape as
    /// `InotifyFd::read_into`.
    pub fn read_into(&self, buf: &mut [u8]) -> io::Result<Option<usize>> {
        // SAFETY: read is a thin syscall wrapper; buf is exclusive.
        let n = unsafe {
            libc::read(
                self.inner.as_raw_fd(),
                buf.as_mut_ptr().cast::<libc::c_void>(),
                buf.len(),
            )
        };
        if n < 0 {
            let err = io::Error::last_os_error();
            if matches!(
                err.raw_os_error(),
                Some(libc::EAGAIN) | Some(libc::EWOULDBLOCK)
            ) {
                return Ok(None);
            }
            return Err(err);
        }
        Ok(Some(n as usize))
    }

    pub fn raw(&self) -> RawFd {
        self.inner.as_raw_fd()
    }

    /// Returns the underlying fd as an `i32` for use as `pollfd.fd`.
    /// Routed through this rather than `inner.as_raw_fd()` so the
    /// subscriber can poll the fanotify fd without the ffi module
    /// having to expose `OwnedFd`.
    pub fn poll_fd(&self) -> RawFd {
        self.inner.as_raw_fd()
    }

    /// Resolves an `(file_handle, name)` pair from a fanotify info
    /// record into a full `PathBuf` via `open_by_handle_at` +
    /// `readlink("/proc/self/fd/N")`. Returns `Ok(None)` when the
    /// handle can no longer be resolved (rename-out-of-watch + delete
    /// happens fast enough on a busy fs that the dir is gone by the
    /// time we look up the parent).
    pub fn resolve_dfid_name(
        &self,
        handle_bytes: &[u8],
        handle_type: i32,
        name: &OsStr,
    ) -> io::Result<Option<PathBuf>> {
        // The userspace `struct file_handle` is a flexible-array type:
        //   __u32 handle_bytes;
        //   int   handle_type;
        //   unsigned char f_handle[handle_bytes];
        // We construct it on the heap so the trailing array carries
        // the bytes the kernel sent us in the fanotify info record.
        let total = std::mem::size_of::<u32>() + std::mem::size_of::<i32>() + handle_bytes.len();
        let mut buf = vec![0u8; total];
        buf[0..4].copy_from_slice(&(handle_bytes.len() as u32).to_ne_bytes());
        buf[4..8].copy_from_slice(&handle_type.to_ne_bytes());
        buf[8..].copy_from_slice(handle_bytes);

        // SAFETY: open_by_handle_at reads `handle_bytes` bytes after the
        // 8-byte header; we sized buf exactly to that.
        let fd = unsafe {
            libc::open_by_handle_at(
                self.mount.as_raw_fd(),
                buf.as_ptr() as *mut _,
                libc::O_PATH | libc::O_CLOEXEC,
            )
        };
        if fd < 0 {
            let err = io::Error::last_os_error();
            // ENOENT / ESTALE: the file or dir went away between the
            // event and our resolve. Caller falls back to a cache lookup.
            match err.raw_os_error() {
                Some(libc::ENOENT) | Some(libc::ESTALE) => return Ok(None),
                _ => return Err(err),
            }
        }
        // SAFETY: fd is a fresh fd we just opened.
        let handle_fd = unsafe { OwnedFd::from_raw_fd(fd) };

        // readlink("/proc/self/fd/N") is the canonical Linux idiom for
        // turning an open fd into the path that opened it. O_PATH fds
        // are valid arguments here per `proc(5)`.
        let dir = read_link_proc_self_fd(handle_fd.as_raw_fd())?;
        // The fanotify DFID_NAME info record's `name` is the trailing
        // path component (NUL-terminated). Empty-name events target the
        // dir itself (e.g. ATTRIB on a dir), but we already filter those
        // out at the classifier layer.
        if name.is_empty() {
            return Ok(Some(dir));
        }
        Ok(Some(dir.join(name)))
    }
}

/// `readlink("/proc/self/fd/N")` — turns an open fd into its path.
fn read_link_proc_self_fd(fd: RawFd) -> io::Result<PathBuf> {
    let link = format!("/proc/self/fd/{fd}");
    let c = CString::new(link.as_bytes())
        .map_err(|e| io::Error::new(io::ErrorKind::InvalidInput, e))?;
    // PATH_MAX on Linux is 4096; readlink returns the byte count read
    // (not NUL-terminated). Loop-grow if the path turns out longer.
    let mut buf = vec![0u8; 4096];
    loop {
        // SAFETY: thin syscall wrapper; buf.len() bounds the write.
        // `c_char` is i8 on x86_64-linux-gnu and u8 on some musl arches —
        // route through `*mut libc::c_char` so the cast is portable.
        let n =
            unsafe { libc::readlink(c.as_ptr(), buf.as_mut_ptr() as *mut libc::c_char, buf.len()) };
        if n < 0 {
            return Err(io::Error::last_os_error());
        }
        let n = n as usize;
        if n < buf.len() {
            buf.truncate(n);
            return Ok(PathBuf::from(OsString::from_vec(buf)));
        }
        // Path was longer than the buffer — grow and retry.
        buf.resize(buf.len() * 2, 0);
    }
}

/// `FAN_REPORT_DFID_NAME` is fairly recent (Linux 5.17+); some libc
/// versions don't expose it as a constant. Define it locally so the
/// build doesn't depend on libc-version coverage of every fanotify
/// flag we use. Value taken from `<linux/fanotify.h>`.
const FAN_REPORT_DFID_NAME: libc::c_uint = 0x0000_0c00;

/// Info-record `info_type` values from `<linux/fanotify.h>`.
pub const FAN_EVENT_INFO_TYPE_FID: u8 = 1;
pub const FAN_EVENT_INFO_TYPE_DFID_NAME: u8 = 2;
pub const FAN_EVENT_INFO_TYPE_DFID: u8 = 3;
pub const FAN_EVENT_INFO_TYPE_OLD_DFID_NAME: u8 = 10;
pub const FAN_EVENT_INFO_TYPE_NEW_DFID_NAME: u8 = 12;

/// Decoded fanotify event metadata + the FIRST DFID_NAME info record we
/// found. Phase 3 only needs the parent-FID + filename pair to resolve
/// a rename or create — Phase 13 will widen this for full info-record
/// chain handling (e.g. xattr-change events).
#[derive(Debug)]
pub struct ParsedFanotifyEvent {
    pub mask: u64,
    pub fd: i32,
    pub pid: i32,
    pub handle_bytes: Vec<u8>,
    pub handle_type: i32,
    pub name: OsString,
    /// `OLD_DFID_NAME`-typed info record (rename old half), if present
    /// in the same event. The kernel emits both halves of a rename in
    /// a single event when `FAN_REPORT_DFID_NAME` is set, so pairing
    /// is implicit here unlike the inotify cookie dance.
    pub old: Option<(Vec<u8>, i32, OsString)>,
}

/// Iterator over fanotify events in a `read()` buffer.
pub struct FanotifyEventIter<'a> {
    buf: &'a [u8],
    offset: usize,
}

impl<'a> FanotifyEventIter<'a> {
    pub fn new(buf: &'a [u8]) -> Self {
        Self { buf, offset: 0 }
    }
}

impl Iterator for FanotifyEventIter<'_> {
    type Item = ParsedFanotifyEvent;

    fn next(&mut self) -> Option<ParsedFanotifyEvent> {
        // struct fanotify_event_metadata {
        //   __u32 event_len;     // 0
        //   __u8  vers;          // 4
        //   __u8  reserved;      // 5
        //   __u16 metadata_len;  // 6
        //   __u64 mask;          // 8
        //   __s32 fd;            // 16
        //   __s32 pid;           // 20
        // }                       // 24 bytes
        const META: usize = 24;
        if self.offset + META > self.buf.len() {
            return None;
        }
        let meta = &self.buf[self.offset..self.offset + META];
        let event_len = u32::from_ne_bytes(meta[0..4].try_into().ok()?) as usize;
        let metadata_len = u16::from_ne_bytes(meta[6..8].try_into().ok()?) as usize;
        let mask = u64::from_ne_bytes(meta[8..16].try_into().ok()?);
        let fd = i32::from_ne_bytes(meta[16..20].try_into().ok()?);
        let pid = i32::from_ne_bytes(meta[20..24].try_into().ok()?);

        if event_len == 0 || self.offset + event_len > self.buf.len() {
            return None;
        }

        let mut handle_bytes: Vec<u8> = Vec::new();
        let mut handle_type: i32 = 0;
        let mut name = OsString::new();
        let mut old: Option<(Vec<u8>, i32, OsString)> = None;

        // Walk info records starting at offset+metadata_len.
        let mut info_off = self.offset + metadata_len;
        let event_end = self.offset + event_len;
        while info_off + 4 <= event_end {
            let info_hdr = &self.buf[info_off..info_off + 4];
            let info_type = info_hdr[0];
            let info_len = u16::from_ne_bytes(info_hdr[2..4].try_into().ok()?) as usize;
            if info_len == 0 || info_off + info_len > event_end {
                break;
            }
            // Body of a (D)FID info record:
            //   __kernel_fsid_t fsid;          // 8 bytes
            //   struct file_handle handle;     // flexible
            //   ... optional NUL-terminated name (DFID_NAME variants)
            const FSID: usize = 8;
            const FH_HDR: usize = 8; // handle_bytes (u32) + handle_type (i32)
            if matches!(
                info_type,
                FAN_EVENT_INFO_TYPE_FID
                    | FAN_EVENT_INFO_TYPE_DFID
                    | FAN_EVENT_INFO_TYPE_DFID_NAME
                    | FAN_EVENT_INFO_TYPE_OLD_DFID_NAME
                    | FAN_EVENT_INFO_TYPE_NEW_DFID_NAME
            ) && info_len >= 4 + FSID + FH_HDR
            {
                let body_start = info_off + 4 + FSID;
                let hbytes =
                    u32::from_ne_bytes(self.buf[body_start..body_start + 4].try_into().ok()?)
                        as usize;
                let htype =
                    i32::from_ne_bytes(self.buf[body_start + 4..body_start + 8].try_into().ok()?);
                let handle_payload_start = body_start + FH_HDR;
                let handle_payload_end = handle_payload_start + hbytes;
                if handle_payload_end <= info_off + info_len {
                    let payload = self.buf[handle_payload_start..handle_payload_end].to_vec();
                    // DFID_NAME variants carry a trailing NUL-terminated
                    // filename; FID / DFID alone do not.
                    let n = if matches!(
                        info_type,
                        FAN_EVENT_INFO_TYPE_DFID_NAME
                            | FAN_EVENT_INFO_TYPE_OLD_DFID_NAME
                            | FAN_EVENT_INFO_TYPE_NEW_DFID_NAME
                    ) && handle_payload_end < info_off + info_len
                    {
                        let name_bytes = &self.buf[handle_payload_end..info_off + info_len];
                        let nul = name_bytes
                            .iter()
                            .position(|&b| b == 0)
                            .unwrap_or(name_bytes.len());
                        OsString::from_vec(name_bytes[..nul].to_vec())
                    } else {
                        OsString::new()
                    };

                    if info_type == FAN_EVENT_INFO_TYPE_OLD_DFID_NAME {
                        old = Some((payload, htype, n));
                    } else {
                        // First DFID_NAME / DFID / FID we see wins as
                        // "the" event location; later NEW_DFID_NAME
                        // records (which sometimes accompany renames)
                        // are treated as the new-half via `name`.
                        if handle_bytes.is_empty() {
                            handle_bytes = payload;
                            handle_type = htype;
                            name = n;
                        }
                    }
                }
            }
            info_off += info_len;
        }

        self.offset += event_len;
        Some(ParsedFanotifyEvent {
            mask,
            fd,
            pid,
            handle_bytes,
            handle_type,
            name,
            old,
        })
    }
}

// =====================================================================
// getdents64 walker
// =====================================================================

/// Recursive directory walker built on raw `getdents64(2)`. Faster than
/// `std::fs::read_dir` on huge trees because each syscall returns
/// thousands of entries packed into a single buffer instead of one
/// `getdents` call per entry.
///
/// Calls `visit` for every regular file encountered. Symlinks are
/// skipped (not followed) — same policy as Phase 2 macOS. Permission
/// errors on a single subdir are logged and skipped, not propagated;
/// this matches the spec's "best-effort indexing" stance for trees
/// the user doesn't fully own.
pub fn walk_getdents64<F>(root: &Path, mut visit: F) -> io::Result<()>
where
    F: FnMut(&Path, &std::fs::Metadata),
{
    use std::collections::HashSet;
    use std::os::unix::fs::MetadataExt;

    // Iterative DFS so deeply-nested trees can't blow the stack.
    // `visited` dedupes by (st_dev, st_ino) so a bind-mount loop
    // (mount --bind /a /a/sub) doesn't spin forever.
    let mut stack: Vec<PathBuf> = vec![root.to_path_buf()];
    let mut visited: HashSet<(u64, u64)> = HashSet::new();
    if let Ok(meta) = std::fs::symlink_metadata(root) {
        visited.insert((meta.dev(), meta.ino()));
    }
    let mut buf = vec![0u8; 64 * 1024];

    while let Some(dir) = stack.pop() {
        let cdir = match CString::new(dir.as_os_str().as_bytes()) {
            Ok(c) => c,
            Err(_) => continue,
        };
        // SAFETY: thin syscall wrapper; the kernel does not retain cdir
        // past return.
        let dirfd = unsafe {
            libc::open(
                cdir.as_ptr(),
                libc::O_RDONLY | libc::O_DIRECTORY | libc::O_CLOEXEC | libc::O_NOFOLLOW,
            )
        };
        if dirfd < 0 {
            // EACCES (perm denied) / ENOENT (raced delete) — skip.
            tracing::trace!(dir = %dir.display(), error = %io::Error::last_os_error(),
                "getdents64: skipping unreadable dir");
            continue;
        }
        // SAFETY: dirfd is a fresh fd we just opened.
        let dirfd = unsafe { OwnedFd::from_raw_fd(dirfd) };

        loop {
            // SAFETY: SYS_getdents64 fills buf with linux_dirent64 records
            // up to the buffer length, returning bytes written.
            let n = unsafe {
                libc::syscall(
                    libc::SYS_getdents64,
                    dirfd.as_raw_fd(),
                    buf.as_mut_ptr(),
                    buf.len(),
                )
            };
            if n < 0 {
                return Err(io::Error::last_os_error());
            }
            if n == 0 {
                break;
            }
            let bytes = n as usize;
            let mut off = 0usize;
            while off + 19 <= bytes {
                // struct linux_dirent64 {
                //   __u64 d_ino;        // 0
                //   __s64 d_off;        // 8
                //   __u16 d_reclen;     // 16
                //   __u8  d_type;       // 18
                //   char  d_name[];     // 19
                // }
                let reclen =
                    u16::from_ne_bytes(buf[off + 16..off + 18].try_into().unwrap()) as usize;
                if reclen == 0 || off + reclen > bytes {
                    break;
                }
                let d_type = buf[off + 18];
                // d_name is NUL-terminated; we trust the NUL even though
                // d_reclen pads to 8-byte alignment past it.
                let name_start = off + 19;
                let name_max_end = off + reclen;
                let nul = buf[name_start..name_max_end]
                    .iter()
                    .position(|&b| b == 0)
                    .map(|p| name_start + p)
                    .unwrap_or(name_max_end);
                let name_bytes = &buf[name_start..nul];
                off += reclen;

                // Skip "." and ".." — every dir has them and we don't
                // want to descend into the parent.
                if name_bytes == b"." || name_bytes == b".." {
                    continue;
                }
                let name_os = OsStr::from_bytes(name_bytes);
                let full = dir.join(name_os);

                match d_type {
                    libc::DT_DIR => match std::fs::symlink_metadata(&full) {
                        Ok(meta) => {
                            let key = (meta.dev(), meta.ino());
                            if visited.insert(key) {
                                stack.push(full);
                            }
                        }
                        Err(_) => stack.push(full),
                    },
                    libc::DT_REG => {
                        // The only metadata bit we need at bootstrap is
                        // size + mtime + ctime + mode; symlink-metadata
                        // (which doesn't follow symlinks) gets us all of
                        // them in one statx-equivalent call.
                        if let Ok(meta) = std::fs::symlink_metadata(&full) {
                            if meta.is_file() {
                                visit(&full, &meta);
                            }
                        }
                    }
                    libc::DT_UNKNOWN => {
                        // Some filesystems (e.g. some FUSE backends, old
                        // ZFS-on-Linux versions) don't fill in d_type;
                        // fall back to a stat() to disambiguate.
                        if let Ok(meta) = std::fs::symlink_metadata(&full) {
                            if meta.is_dir() {
                                let key = (meta.dev(), meta.ino());
                                if visited.insert(key) {
                                    stack.push(full);
                                }
                            } else if meta.is_file() {
                                visit(&full, &meta);
                            }
                        }
                    }
                    _ => {} // symlinks, sockets, fifos, devices — drop.
                }
            }
        }
    }
    Ok(())
}

// =====================================================================
// statfs / stat
// =====================================================================

/// Reads the device id (`stat.st_dev`) for the given path. Used to
/// pin cursor reuse to the exact volume the cursor was minted on.
pub fn device_id(path: &Path) -> io::Result<u64> {
    let meta = std::fs::symlink_metadata(path)?;
    use std::os::unix::fs::MetadataExt;
    Ok(meta.dev())
}

/// Reads `statfs(2)` and maps `f_type` (a fs magic number) to a human
/// label. Linux's statfs doesn't carry a textual filesystem name the
/// way macOS's does, so we maintain a small lookup table for the
/// filesystems Phase 3's smoke matrix exercises (ext4, Btrfs, ZFS,
/// XFS, F2FS, tmpfs). Unknown magic numbers degrade to a hex string
/// — better than panicking, and the cursor's `fs_name` is diagnostic
/// only so a wrong label is non-fatal.
pub fn statfs_name(path: &Path) -> io::Result<String> {
    let c = CString::new(path.as_os_str().as_bytes())
        .map_err(|e| io::Error::new(io::ErrorKind::InvalidInput, e))?;
    // SAFETY: out-buffer is sized by libc; statfs() writes via pointer.
    let mut buf: libc::statfs = unsafe { std::mem::zeroed() };
    let r = unsafe { libc::statfs(c.as_ptr(), &mut buf) };
    if r != 0 {
        return Err(io::Error::last_os_error());
    }
    Ok(fs_magic_name(buf.f_type as i64))
}

/// Maps a `statfs.f_type` magic number to a human-readable filesystem
/// name. Values from `<linux/magic.h>` and the userspace `statfs(2)`
/// man page. Anything unknown renders as `"unknown:<hex>"` so the
/// cursor JSON still round-trips cleanly.
pub fn fs_magic_name(magic: i64) -> String {
    match magic as u64 {
        0xEF53 => "ext4".into(),
        0x9123_683E => "btrfs".into(),
        0x2FC1_2FC1 => "zfs".into(),
        0x5846_5342 => "xfs".into(),
        0xF2F5_2010 => "f2fs".into(),
        0x0102_1994 => "tmpfs".into(),
        0x6969 => "nfs".into(),
        0x4d44 => "fat".into(),
        0x5346_4e54 => "ntfs".into(),
        0x6E73_6663 | 0x6573_5546 => "fuse".into(),
        0x794C_7630 => "overlayfs".into(),
        0x9FA0 => "proc".into(),
        0x6273_6473 => "btrfs-test".into(),
        other => format!("unknown:0x{other:x}"),
    }
}

// =====================================================================
// Capability detection
// =====================================================================

/// Returns true when this process has `CAP_SYS_ADMIN` in its effective
/// set. Reads `/proc/self/status`'s `CapEff:` line and tests bit 21
/// (CAP_SYS_ADMIN). A failed read defaults to `false` so a stripped-
/// down container without `/proc` mounted falls cleanly into the
/// inotify path instead of attempting a fanotify upgrade that would
/// EPERM at the syscall.
pub fn has_cap_sys_admin() -> bool {
    has_capability_bit(CAP_SYS_ADMIN_BIT)
}

/// CAP_SYS_ADMIN's bit number per `<linux/capability.h>`.
const CAP_SYS_ADMIN_BIT: u32 = 21;

fn has_capability_bit(bit: u32) -> bool {
    let bytes = match std::fs::read("/proc/self/status") {
        Ok(b) => b,
        Err(_) => return false,
    };
    let s = match std::str::from_utf8(&bytes) {
        Ok(s) => s,
        Err(_) => return false,
    };
    for line in s.lines() {
        if let Some(rest) = line.strip_prefix("CapEff:") {
            let hex = rest.trim();
            if let Ok(mask) = u64::from_str_radix(hex, 16) {
                return (mask >> bit) & 1 == 1;
            }
        }
    }
    false
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_inotify_event_with_name() {
        // Hand-craft a single inotify_event(IN_CREATE | IN_ISDIR) for
        // a watched-dir entry called "child". len rounds up to 8 (NUL
        // + padding), record total = 16 + 8 = 24.
        let mut buf = vec![0u8; 24];
        buf[0..4].copy_from_slice(&7i32.to_ne_bytes()); // wd
        buf[4..8].copy_from_slice(&0x4000_0100u32.to_ne_bytes()); // mask: ISDIR | CREATE
        buf[8..12].copy_from_slice(&0u32.to_ne_bytes()); // cookie
        buf[12..16].copy_from_slice(&8u32.to_ne_bytes()); // len
        buf[16..21].copy_from_slice(b"child");
        // 16+5..16+8 stay zero (NUL pad).

        let events: Vec<_> = InotifyEventIter::new(&buf).collect();
        assert_eq!(events.len(), 1);
        let e = &events[0];
        assert_eq!(e.wd, 7);
        assert_eq!(e.mask, 0x4000_0100);
        assert_eq!(e.cookie, 0);
        assert_eq!(e.name.to_string_lossy(), "child");
    }

    #[test]
    fn parse_inotify_event_with_no_name() {
        // IN_DELETE_SELF for the watched dir itself — len == 0, record
        // total == 16.
        let mut buf = vec![0u8; 16];
        buf[0..4].copy_from_slice(&3i32.to_ne_bytes());
        buf[4..8].copy_from_slice(&0x0000_0400u32.to_ne_bytes()); // IN_DELETE_SELF
        buf[8..12].copy_from_slice(&0u32.to_ne_bytes());
        buf[12..16].copy_from_slice(&0u32.to_ne_bytes()); // len 0

        let events: Vec<_> = InotifyEventIter::new(&buf).collect();
        assert_eq!(events.len(), 1);
        assert_eq!(events[0].name, OsString::new());
    }

    #[test]
    fn parse_two_back_to_back_inotify_events() {
        // Verifies the iterator correctly advances past the first
        // record's variable-length tail to find the second record's
        // fixed header at the right offset.
        let mut buf = Vec::with_capacity(48);

        let mut ev1 = vec![0u8; 24];
        ev1[0..4].copy_from_slice(&1i32.to_ne_bytes());
        ev1[4..8].copy_from_slice(&0x40u32.to_ne_bytes()); // IN_MOVED_FROM
        ev1[8..12].copy_from_slice(&0xCAFEu32.to_ne_bytes()); // cookie
        ev1[12..16].copy_from_slice(&8u32.to_ne_bytes());
        ev1[16..21].copy_from_slice(b"old.x");
        buf.extend_from_slice(&ev1);

        let mut ev2 = vec![0u8; 24];
        ev2[0..4].copy_from_slice(&1i32.to_ne_bytes());
        ev2[4..8].copy_from_slice(&0x80u32.to_ne_bytes()); // IN_MOVED_TO
        ev2[8..12].copy_from_slice(&0xCAFEu32.to_ne_bytes());
        ev2[12..16].copy_from_slice(&8u32.to_ne_bytes());
        ev2[16..21].copy_from_slice(b"new.x");
        buf.extend_from_slice(&ev2);

        let events: Vec<_> = InotifyEventIter::new(&buf).collect();
        assert_eq!(events.len(), 2);
        assert_eq!(events[0].cookie, 0xCAFE);
        assert_eq!(events[0].name.to_string_lossy(), "old.x");
        assert_eq!(events[1].cookie, 0xCAFE);
        assert_eq!(events[1].name.to_string_lossy(), "new.x");
    }

    #[test]
    fn parse_inotify_event_truncated_header_is_none() {
        // Half a header should not panic — iterator returns None.
        let buf = vec![0u8; 7];
        let events: Vec<_> = InotifyEventIter::new(&buf).collect();
        assert!(events.is_empty());
    }

    #[test]
    fn fs_magic_name_known_filesystems() {
        assert_eq!(fs_magic_name(0xEF53), "ext4");
        assert_eq!(fs_magic_name(0x9123_683E), "btrfs");
        assert_eq!(fs_magic_name(0x2FC1_2FC1), "zfs");
        assert_eq!(fs_magic_name(0x5846_5342), "xfs");
        assert_eq!(fs_magic_name(0x0102_1994), "tmpfs");
    }

    #[test]
    fn fs_magic_name_unknown_renders_hex() {
        let s = fs_magic_name(0xDEAD_BEEF);
        assert!(s.starts_with("unknown:0x"));
        assert!(s.contains("deadbeef"));
    }

    #[test]
    fn parse_fanotify_event_with_dfid_name() {
        // Hand-craft a single fanotify event with one DFID_NAME info
        // record. Layout:
        //
        //   metadata (24 bytes)
        //   info_hdr (4) + fsid (8) + handle_bytes_field (4) +
        //   handle_type (4) + handle_payload (8) + name "f.txt\0" (6)
        //   = info_total 34, padded to (event_len - 24) = ?
        //
        // For simplicity we pick event_len so info_total fits exactly
        // (no padding required by the kernel ABI when we control both
        // sides of the buffer in test land).
        let info_payload_len = 8usize; // 8 bytes of opaque handle bytes
        let name = b"f.txt\0";
        let info_len = 4 + 8 + 4 + 4 + info_payload_len + name.len(); // = 34
        let event_len = 24 + info_len; // = 58

        let mut buf = vec![0u8; event_len];
        buf[0..4].copy_from_slice(&(event_len as u32).to_ne_bytes());
        buf[4] = 3; // vers
        buf[5] = 0;
        buf[6..8].copy_from_slice(&24u16.to_ne_bytes()); // metadata_len
        buf[8..16].copy_from_slice(&(0x100u64).to_ne_bytes()); // mask: FAN_CREATE
        buf[16..20].copy_from_slice(&(-1i32).to_ne_bytes()); // fd: FAN_NOFD
        buf[20..24].copy_from_slice(&12345i32.to_ne_bytes()); // pid

        let info_off = 24usize;
        buf[info_off] = FAN_EVENT_INFO_TYPE_DFID_NAME;
        buf[info_off + 1] = 0;
        buf[info_off + 2..info_off + 4].copy_from_slice(&(info_len as u16).to_ne_bytes());
        // fsid
        buf[info_off + 4..info_off + 12].copy_from_slice(&[0u8; 8]);
        // file_handle: bytes + type
        buf[info_off + 12..info_off + 16].copy_from_slice(&(info_payload_len as u32).to_ne_bytes());
        buf[info_off + 16..info_off + 20].copy_from_slice(&1i32.to_ne_bytes());
        // handle payload (opaque to us)
        for i in 0..info_payload_len {
            buf[info_off + 20 + i] = 0xAA;
        }
        // name
        buf[info_off + 20 + info_payload_len..event_len].copy_from_slice(name);

        let events: Vec<_> = FanotifyEventIter::new(&buf).collect();
        assert_eq!(events.len(), 1);
        let e = &events[0];
        assert_eq!(e.mask, 0x100);
        assert_eq!(e.handle_type, 1);
        assert_eq!(e.handle_bytes.len(), info_payload_len);
        assert!(e.handle_bytes.iter().all(|&b| b == 0xAA));
        assert_eq!(e.name.to_string_lossy(), "f.txt");
        assert!(e.old.is_none());
    }
}
