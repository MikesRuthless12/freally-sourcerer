//! Linux fast walker — raw `getdents64` for directory enumeration plus
//! `statx` per entry for size + type. `statx` is faster than `lstat`
//! because the caller passes a "what fields do I actually care about?"
//! mask (`STATX_TYPE | STATX_SIZE | STATX_INO`) and the kernel skips
//! the rest.
//!
//! Cycle-safe via a `(dev, ino)` visited set so bind-mount loops can't
//! send us spinning forever.

#![cfg(target_os = "linux")]

use std::collections::HashSet;
use std::ffi::{CString, OsStr};
use std::os::unix::ffi::OsStrExt;
use std::os::unix::io::{AsRawFd, FromRawFd, OwnedFd};
use std::path::{Path, PathBuf};

use anyhow::{Context, Result, anyhow};

use crate::ScanStats;

/// Linux `linux_dirent64` layout. We don't pull it from libc because
/// some libc revisions gate it behind feature flags; the structure is
/// stable kernel ABI.
#[repr(C)]
#[allow(non_camel_case_types, dead_code)]
struct LinuxDirent64Hdr {
    d_ino: u64,
    d_off: i64,
    d_reclen: u16,
    d_type: u8,
    // d_name follows: NUL-terminated, up to d_reclen - 19 bytes long.
}

/// Linux `statx` structure (kernel ABI). We define it locally so the
/// build doesn't depend on libc's `statx` feature flag.
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

const AT_STATX_SYNC_AS_STAT: i32 = 0;
const AT_SYMLINK_NOFOLLOW: i32 = 0x100;
const STATX_TYPE: u32 = 0x0001;
const STATX_MODE: u32 = 0x0002;
const STATX_SIZE: u32 = 0x0200;
const STATX_INO: u32 = 0x0100;

/// Walk `root` once via getdents64 + statx and return aggregate stats.
pub fn scan(root: &Path) -> Result<ScanStats> {
    let mut stats = ScanStats::default();
    let mut stack: Vec<PathBuf> = vec![root.to_path_buf()];
    let mut visited: HashSet<(u64, u64)> = HashSet::new();
    // Seed the visited set with the root inode so a bind-mount of root
    // back under itself doesn't double-count.
    if let Some((dev, ino)) = statx_dev_ino(root) {
        visited.insert((dev, ino));
        stats.dirs += 1;
    }
    let mut buf = vec![0u8; 64 * 1024];

    while let Some(dir) = stack.pop() {
        let cdir = match CString::new(dir.as_os_str().as_bytes()) {
            Ok(c) => c,
            Err(_) => continue,
        };
        // SAFETY: thin syscall wrapper; the kernel does not retain
        // `cdir` past the call. O_NOFOLLOW prevents following a
        // symlinked directory.
        let fd = unsafe {
            libc::open(
                cdir.as_ptr(),
                libc::O_RDONLY | libc::O_DIRECTORY | libc::O_CLOEXEC | libc::O_NOFOLLOW,
            )
        };
        if fd < 0 {
            // EACCES / ENOENT / etc. on a single subtree — keep walking
            // the rest of the volume. This mirrors the journal crate's
            // best-effort stance and matches `find`'s default behaviour.
            continue;
        }
        // SAFETY: fresh fd we just opened.
        let dirfd = unsafe { OwnedFd::from_raw_fd(fd) };

        loop {
            // SAFETY: SYS_getdents64 writes at most `buf.len()` bytes
            // into `buf` starting at offset 0 and returns the number of
            // bytes filled (or -1 on error, 0 on EOF).
            let n = unsafe {
                libc::syscall(
                    libc::SYS_getdents64,
                    dirfd.as_raw_fd(),
                    buf.as_mut_ptr(),
                    buf.len(),
                )
            };
            if n <= 0 {
                break;
            }
            let bytes = n as usize;
            let mut off = 0usize;
            while off + 19 <= bytes {
                // Manual struct walk — d_reclen at offset 16, d_type at
                // offset 18, d_name at offset 19.
                let reclen =
                    u16::from_ne_bytes(buf[off + 16..off + 18].try_into().unwrap()) as usize;
                if reclen == 0 || off + reclen > bytes {
                    break;
                }
                let d_type = buf[off + 18];
                let name_start = off + 19;
                let name_end_max = off + reclen;
                let nul = buf[name_start..name_end_max]
                    .iter()
                    .position(|&b| b == 0)
                    .map(|p| name_start + p)
                    .unwrap_or(name_end_max);
                let name_bytes = &buf[name_start..nul];
                off += reclen;
                if name_bytes == b"." || name_bytes == b".." {
                    continue;
                }
                let name_os = OsStr::from_bytes(name_bytes);
                let full = dir.join(name_os);

                match d_type {
                    libc::DT_DIR => {
                        // Confirm dev+ino (cycle guard) and recurse.
                        let key = statx_dev_ino(&full);
                        if let Some(k) = key
                            && !visited.insert(k)
                        {
                            continue;
                        }
                        stats.dirs += 1;
                        stack.push(full);
                    }
                    libc::DT_REG => {
                        // Single statx pulls the file size + inode in
                        // one syscall, skipping uid/gid/perms/timestamps
                        // we don't need.
                        if let Some(size) = statx_size(&full) {
                            stats.files += 1;
                            stats.bytes += size;
                        } else {
                            // Race with deletion — count the file
                            // (the dirent existed) but not its bytes.
                            stats.files += 1;
                        }
                    }
                    libc::DT_LNK => {
                        // Don't follow symlinks; don't count them as
                        // files. Matches Sourcerer's index policy.
                    }
                    libc::DT_UNKNOWN => {
                        // Some filesystems (older XFS, network FSes)
                        // return DT_UNKNOWN — fall through to statx
                        // to figure out what we're looking at.
                        let Some(stx) = statx_full(&full) else {
                            continue;
                        };
                        let mode = stx.stx_mode & 0o170000;
                        if mode == 0o040000 {
                            // S_IFDIR
                            if visited.insert((
                                ((stx.stx_dev_major as u64) << 32) | stx.stx_dev_minor as u64,
                                stx.stx_ino,
                            )) {
                                stats.dirs += 1;
                                stack.push(full);
                            }
                        } else if mode == 0o100000 {
                            // S_IFREG
                            stats.files += 1;
                            stats.bytes += stx.stx_size;
                        }
                    }
                    _ => { /* sockets, fifos, char/block devices — skip */ }
                }
            }
        }
    }
    Ok(stats)
}

fn statx_size(path: &Path) -> Option<u64> {
    statx_full(path).map(|s| s.stx_size)
}

fn statx_dev_ino(path: &Path) -> Option<(u64, u64)> {
    let s = statx_full(path)?;
    Some((
        ((s.stx_dev_major as u64) << 32) | s.stx_dev_minor as u64,
        s.stx_ino,
    ))
}

fn statx_full(path: &Path) -> Option<Statx> {
    let cpath = CString::new(path.as_os_str().as_bytes()).ok()?;
    let mut buf = Statx::default();
    let mask = STATX_TYPE | STATX_MODE | STATX_SIZE | STATX_INO;
    // SAFETY: thin syscall wrapper. statx(2) writes to buf, doesn't
    // retain pointers, and returns 0 on success.
    let rc = unsafe {
        libc::syscall(
            libc::SYS_statx,
            libc::AT_FDCWD,
            cpath.as_ptr(),
            AT_SYMLINK_NOFOLLOW | AT_STATX_SYNC_AS_STAT,
            mask,
            &mut buf as *mut Statx,
        )
    };
    if rc == 0 { Some(buf) } else { None }
}

#[allow(dead_code)]
fn err_msg(prefix: &str) -> anyhow::Error {
    let e = std::io::Error::last_os_error();
    anyhow!("{prefix}: {e}")
}

#[allow(dead_code)]
fn context<T>(r: Result<T>, what: impl Into<String>) -> Result<T> {
    r.with_context(|| what.into())
}
