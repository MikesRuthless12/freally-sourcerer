//! Linux fast walker — Phase 13 optimised path.
//!
//! Three optimisations over the textbook `getdents64 + statx`:
//!
//! 1. **Parallel descent via `rayon::scope`** — each directory we
//!    discover is spawned as a fresh task on the rayon pool, so a
//!    16-core runner walks 16 subtrees concurrently instead of one
//!    DFS line. Shared state (file / dir / byte totals, visited set)
//!    is held in lock-free atomics + a single `Mutex<HashSet>` whose
//!    contention is bounded by the *directory* count, not the file
//!    count.
//! 2. **dirfd-relative `statx`** — we keep the open `dirfd` from the
//!    `getdents64` loop and pass `(dirfd, basename)` to `SYS_statx`
//!    instead of `(AT_FDCWD, absolute_path)`. The kernel skips the
//!    component-by-component lookup of the parent path, which on a
//!    deep tree is the largest single cost of statx.
//! 3. **`AT_STATX_DONT_SYNC`** — when stat is satisfied by the cached
//!    inode the kernel returns immediately; we never block waiting for
//!    a remote filesystem (NFS / SMB / Ceph / 9P) to confirm the
//!    canonical state. For local filesystems this is a no-op; for
//!    network mounts it's the difference between a microsecond and a
//!    round-trip.
//!
//! Additionally: the `getdents64` buffer is 256 KB (up from 64 KB) so
//! each syscall returns more dirent records before the kernel has to
//! pause us on the syscall boundary.
//!
//! Cycle-safe via a shared `(dev, ino)` visited set so bind-mount
//! loops can't send us spinning.

#![cfg(target_os = "linux")]

use std::collections::HashSet;
use std::ffi::{CString, OsStr};
use std::os::unix::ffi::OsStrExt;
use std::os::unix::io::{AsRawFd, FromRawFd, OwnedFd, RawFd};
use std::path::{Path, PathBuf};
use std::sync::Mutex;
use std::sync::atomic::{AtomicU64, Ordering};

use anyhow::Result;

use crate::ScanStats;

// ---- kernel ABI -----------------------------------------------------

#[repr(C)]
#[derive(Default)]
#[allow(non_camel_case_types, non_snake_case)]
struct Statx {
    stx_mask: u32,
    stx_blksize: u32,
    stx_attributes: u64,
    stx_nlink: u32,
    stx_uid: u32,
    stx_gid: u32,
    stx_mode: u16,
    __spare0: [u16; 1],
    stx_ino: u64,
    stx_size: u64,
    stx_blocks: u64,
    stx_attributes_mask: u64,
    stx_atime: StatxTimestamp,
    stx_btime: StatxTimestamp,
    stx_ctime: StatxTimestamp,
    stx_mtime: StatxTimestamp,
    stx_rdev_major: u32,
    stx_rdev_minor: u32,
    stx_dev_major: u32,
    stx_dev_minor: u32,
    stx_mnt_id: u64,
    stx_dio_mem_align: u32,
    stx_dio_offset_align: u32,
    __spare3: [u64; 12],
}

#[repr(C)]
#[derive(Default)]
#[allow(non_camel_case_types, non_snake_case)]
struct StatxTimestamp {
    tv_sec: i64,
    tv_nsec: u32,
    __reserved: i32,
}

const AT_SYMLINK_NOFOLLOW: i32 = 0x100;
const AT_STATX_DONT_SYNC: i32 = 0x4000;
const STATX_TYPE: u32 = 0x0001;
const STATX_MODE: u32 = 0x0002;
const STATX_SIZE: u32 = 0x0200;
const STATX_INO: u32 = 0x0100;
/// 256 KB buffer — sized so a typical directory of 5-10k entries fits
/// in 1-2 `getdents64` calls.
const DENTS_BUF: usize = 256 * 1024;

// ---- shared state ---------------------------------------------------

struct Counters {
    files: AtomicU64,
    dirs: AtomicU64,
    bytes: AtomicU64,
    visited: Mutex<HashSet<(u64, u64)>>,
}

// ---- public entrypoint ----------------------------------------------

pub fn scan(root: &Path) -> Result<ScanStats> {
    let counters = Counters {
        files: AtomicU64::new(0),
        dirs: AtomicU64::new(1), // count root itself
        bytes: AtomicU64::new(0),
        visited: Mutex::new(HashSet::new()),
    };

    // Seed visited with root inode (path-based statx, only one such call).
    if let Some(key) = statx_dev_ino_path(root) {
        counters.visited.lock().unwrap().insert(key);
    }

    rayon::scope(|s| {
        walk_dir(root.to_path_buf(), s, &counters);
    });

    Ok(ScanStats {
        files: counters.files.load(Ordering::Relaxed),
        dirs: counters.dirs.load(Ordering::Relaxed),
        bytes: counters.bytes.load(Ordering::Relaxed),
    })
}

// ---- inner walker ---------------------------------------------------

fn walk_dir<'scope>(dir: PathBuf, scope: &rayon::Scope<'scope>, counters: &'scope Counters) {
    let cdir = match CString::new(dir.as_os_str().as_bytes()) {
        Ok(c) => c,
        Err(_) => return,
    };
    // SAFETY: thin syscall wrapper. O_NOFOLLOW + O_DIRECTORY refuses
    // symlinked dirs and non-directories.
    let raw_fd = unsafe {
        libc::open(
            cdir.as_ptr(),
            libc::O_RDONLY | libc::O_DIRECTORY | libc::O_CLOEXEC | libc::O_NOFOLLOW,
        )
    };
    if raw_fd < 0 {
        // EACCES / ENOENT — best-effort, skip subtree.
        return;
    }
    // SAFETY: fresh fd we just opened.
    let dirfd = unsafe { OwnedFd::from_raw_fd(raw_fd) };
    let dirfd_raw = dirfd.as_raw_fd();

    let mut buf = vec![0u8; DENTS_BUF];
    loop {
        // SAFETY: SYS_getdents64 writes ≤ buf.len() bytes into buf and
        // returns the byte count (or 0 = EOF, -1 = error).
        let n =
            unsafe { libc::syscall(libc::SYS_getdents64, dirfd_raw, buf.as_mut_ptr(), buf.len()) };
        if n <= 0 {
            break;
        }
        let bytes = n as usize;
        let mut off = 0usize;
        while off + 19 <= bytes {
            let reclen = u16::from_ne_bytes(buf[off + 16..off + 18].try_into().unwrap()) as usize;
            if reclen == 0 || off + reclen > bytes {
                break;
            }
            let d_type = buf[off + 18];
            let name_start = off + 19;
            let name_end = off + reclen;
            let nul = buf[name_start..name_end]
                .iter()
                .position(|&b| b == 0)
                .map(|p| name_start + p)
                .unwrap_or(name_end);
            // Copy the name out of `buf` so we don't keep a borrow
            // across the spawn-into-rayon boundary below.
            let name_bytes: Vec<u8> = buf[name_start..nul].to_vec();
            off += reclen;
            if name_bytes == b"." || name_bytes == b".." {
                continue;
            }

            match d_type {
                libc::DT_DIR => {
                    // Cycle-guard via dirfd-relative statx (avoids
                    // re-walking the parent path).
                    if let Some(key) = statx_dev_ino_relative(dirfd_raw, &name_bytes)
                        && counters.visited.lock().unwrap().insert(key)
                    {
                        counters.dirs.fetch_add(1, Ordering::Relaxed);
                        let full = dir.join(OsStr::from_bytes(&name_bytes));
                        scope.spawn(move |s| walk_dir(full, s, counters));
                    }
                }
                libc::DT_REG => {
                    counters.files.fetch_add(1, Ordering::Relaxed);
                    if let Some(size) = statx_size_relative(dirfd_raw, &name_bytes) {
                        counters.bytes.fetch_add(size, Ordering::Relaxed);
                    }
                }
                libc::DT_LNK => { /* symlink — don't follow, don't count */ }
                libc::DT_UNKNOWN => {
                    // Some FSes (older XFS, network FSes) return
                    // DT_UNKNOWN; resolve via a full statx.
                    let Some(stx) = statx_full_relative(dirfd_raw, &name_bytes) else {
                        continue;
                    };
                    let mode = stx.stx_mode & 0o170000;
                    if mode == 0o040000 {
                        // S_IFDIR
                        let key = pack_dev_ino(&stx);
                        if counters.visited.lock().unwrap().insert(key) {
                            counters.dirs.fetch_add(1, Ordering::Relaxed);
                            let full = dir.join(OsStr::from_bytes(&name_bytes));
                            scope.spawn(move |s| walk_dir(full, s, counters));
                        }
                    } else if mode == 0o100000 {
                        // S_IFREG
                        counters.files.fetch_add(1, Ordering::Relaxed);
                        counters.bytes.fetch_add(stx.stx_size, Ordering::Relaxed);
                    }
                }
                _ => { /* sockets, fifos, devs — skip */ }
            }
        }
    }
}

// ---- statx wrappers -------------------------------------------------

fn statx_size_relative(dirfd: RawFd, name: &[u8]) -> Option<u64> {
    statx_relative(dirfd, name, STATX_SIZE).map(|s| s.stx_size)
}

fn statx_dev_ino_relative(dirfd: RawFd, name: &[u8]) -> Option<(u64, u64)> {
    statx_relative(dirfd, name, STATX_INO).map(|s| pack_dev_ino(&s))
}

fn statx_full_relative(dirfd: RawFd, name: &[u8]) -> Option<Statx> {
    statx_relative(
        dirfd,
        name,
        STATX_TYPE | STATX_MODE | STATX_SIZE | STATX_INO,
    )
}

fn statx_relative(dirfd: RawFd, name: &[u8], mask: u32) -> Option<Statx> {
    let cname = CString::new(name).ok()?;
    let mut buf = Statx::default();
    // SAFETY: SYS_statx writes to buf; the kernel does not retain
    // cname or buf past return.
    let rc = unsafe {
        libc::syscall(
            libc::SYS_statx,
            dirfd,
            cname.as_ptr(),
            AT_SYMLINK_NOFOLLOW | AT_STATX_DONT_SYNC,
            mask,
            &mut buf as *mut Statx,
        )
    };
    if rc == 0 { Some(buf) } else { None }
}

fn statx_dev_ino_path(path: &Path) -> Option<(u64, u64)> {
    let cpath = CString::new(path.as_os_str().as_bytes()).ok()?;
    let mut buf = Statx::default();
    let rc = unsafe {
        libc::syscall(
            libc::SYS_statx,
            libc::AT_FDCWD,
            cpath.as_ptr(),
            AT_SYMLINK_NOFOLLOW | AT_STATX_DONT_SYNC,
            STATX_INO,
            &mut buf as *mut Statx,
        )
    };
    if rc == 0 {
        Some(pack_dev_ino(&buf))
    } else {
        None
    }
}

fn pack_dev_ino(s: &Statx) -> (u64, u64) {
    (
        ((s.stx_dev_major as u64) << 32) | s.stx_dev_minor as u64,
        s.stx_ino,
    )
}
