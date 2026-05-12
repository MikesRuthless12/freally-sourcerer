//! macOS fast walker — `getattrlistbulk(2)`, the syscall Spotlight
//! itself uses. Each call returns *multiple* dir entries plus their
//! attributes (name, type, size, inode) in a single trip through the
//! kernel, batching what would otherwise be `readdir` + N × `lstat`.
//!
//! Cycle-safe via a `(dev, ino)` visited set so a recursive bind-mount
//! can't send us spinning.

#![cfg(target_os = "macos")]

use std::collections::HashSet;
use std::ffi::CString;
use std::os::unix::ffi::OsStrExt;
use std::os::unix::io::{AsRawFd, FromRawFd, OwnedFd};
use std::path::{Path, PathBuf};

use anyhow::Result;

use crate::ScanStats;

// ---- attrlist bits (sys/attr.h) --------------------------------------

#[repr(C)]
#[allow(non_camel_case_types, non_snake_case)]
struct attrlist {
    bitmapcount: u16,
    reserved: u16,
    commonattr: u32,
    volattr: u32,
    dirattr: u32,
    fileattr: u32,
    forkattr: u32,
}

const ATTR_BIT_MAP_COUNT: u16 = 5;

// commonattr bits
const ATTR_CMN_NAME: u32 = 0x0000_0001;
const ATTR_CMN_OBJTYPE: u32 = 0x0000_0008;
const ATTR_CMN_FILEID: u32 = 0x0200_0000;
const ATTR_CMN_RETURNED_ATTRS: u32 = 0x8000_0000;

// fileattr bits
const ATTR_FILE_TOTALSIZE: u32 = 0x0000_0002;

// getattrlistbulk options
const FSOPT_PACK_INVAL_ATTRS: u64 = 0x0000_0008;
const FSOPT_NOFOLLOW: u64 = 0x0000_0001;

// vnode types (from sys/vnode.h)
const VREG: u32 = 1;
const VDIR: u32 = 2;
const VLNK: u32 = 5;

// libc FFI — getattrlistbulk isn't in the libc crate's stable surface
// (as of 0.2.x), so declare it manually.
unsafe extern "C" {
    fn getattrlistbulk(
        dirfd: libc::c_int,
        attrlist: *mut attrlist,
        attribuf: *mut libc::c_void,
        bufsize: libc::size_t,
        options: u64,
    ) -> libc::c_int;
}

#[repr(C)]
#[allow(non_camel_case_types, non_snake_case)]
struct attrreference_t {
    attr_dataoffset: i32,
    attr_length: u32,
}

/// One bulk request's worth of attribute data. The kernel writes back
/// a stream of variable-length entries — each starts with a u32 entry
/// length, followed by a `returned_attrs` `attribute_set_t`, then the
/// requested attributes in the order they were asked for.
#[repr(C)]
#[allow(non_camel_case_types, non_snake_case)]
struct attribute_set_t {
    commonattr: u32,
    volattr: u32,
    dirattr: u32,
    fileattr: u32,
    forkattr: u32,
}

pub fn scan(root: &Path) -> Result<ScanStats> {
    let mut stats = ScanStats::default();
    let mut stack: Vec<PathBuf> = vec![root.to_path_buf()];
    let mut visited: HashSet<(u64, u64)> = HashSet::new();
    // Seed with the root.
    if let Some((dev, ino)) = stat_dev_ino(root) {
        visited.insert((dev, ino));
        stats.dirs += 1;
    }

    let mut buf = vec![0u8; 64 * 1024];

    // Attribute list — we request: returned-attrs bitmap, name,
    // objtype, fileid, file totalsize. FSOPT_PACK_INVAL_ATTRS lets the
    // kernel emit a zero-length field for attributes that don't apply
    // (e.g. fileattr on a directory).
    let mut al = attrlist {
        bitmapcount: ATTR_BIT_MAP_COUNT,
        reserved: 0,
        commonattr: ATTR_CMN_RETURNED_ATTRS | ATTR_CMN_NAME | ATTR_CMN_OBJTYPE | ATTR_CMN_FILEID,
        volattr: 0,
        dirattr: 0,
        fileattr: ATTR_FILE_TOTALSIZE,
        forkattr: 0,
    };
    let options: u64 = FSOPT_PACK_INVAL_ATTRS | FSOPT_NOFOLLOW;

    while let Some(dir) = stack.pop() {
        let cdir = match CString::new(dir.as_os_str().as_bytes()) {
            Ok(c) => c,
            Err(_) => continue,
        };
        // SAFETY: thin syscall wrapper. O_NOFOLLOW prevents traversing
        // a symlinked dir.
        let fd = unsafe {
            libc::open(
                cdir.as_ptr(),
                libc::O_RDONLY | libc::O_CLOEXEC | libc::O_NOFOLLOW,
            )
        };
        if fd < 0 {
            continue;
        }
        // SAFETY: fresh fd.
        let dirfd = unsafe { OwnedFd::from_raw_fd(fd) };

        loop {
            // SAFETY: thin syscall wrapper; the kernel writes at most
            // `buf.len()` bytes into `buf`. Returns # of entries packed
            // into the buffer, 0 at EOF, -1 on error.
            let rc = unsafe {
                getattrlistbulk(
                    dirfd.as_raw_fd(),
                    &mut al as *mut attrlist,
                    buf.as_mut_ptr().cast::<libc::c_void>(),
                    buf.len(),
                    options,
                )
            };
            if rc <= 0 {
                // 0 = EOF, -1 = error (perm denied / etc.) — give up on
                // this dir but keep walking the rest.
                break;
            }
            let mut cursor: usize = 0;
            for _ in 0..(rc as usize) {
                let entry_start = cursor;
                if entry_start + 4 > buf.len() {
                    break;
                }
                let entry_len =
                    u32::from_ne_bytes(buf[entry_start..entry_start + 4].try_into().unwrap())
                        as usize;
                if entry_len == 0 || entry_start + entry_len > buf.len() {
                    break;
                }
                cursor = entry_start + entry_len;

                // The payload starts after the u32 length field.
                let payload_start = entry_start + 4;
                let body = &buf[payload_start..entry_start + entry_len];

                // Returned-attrs set is the first attribute (we asked
                // for it via ATTR_CMN_RETURNED_ATTRS). It tells us
                // which of the others were actually filled in.
                let Some(parsed) = parse_entry(body, &dir, entry_start, payload_start, &buf) else {
                    continue;
                };

                match parsed.objtype {
                    VDIR => {
                        let key = (0u64, parsed.fileid); // dev unknown per-entry; treat
                        // root inode as already in visited; child dirs
                        // get traversed and re-stat-deduped via stat call
                        let _ = key;
                        if let Some((dev, ino)) = stat_dev_ino(&parsed.path)
                            && visited.insert((dev, ino))
                        {
                            stats.dirs += 1;
                            stack.push(parsed.path);
                        }
                    }
                    VREG => {
                        stats.files += 1;
                        stats.bytes += parsed.size;
                    }
                    VLNK => { /* symlink — don't follow, don't count */ }
                    _ => {}
                }
            }
        }
    }
    Ok(stats)
}

struct ParsedEntry {
    path: PathBuf,
    objtype: u32,
    fileid: u64,
    size: u64,
}

fn parse_entry(
    body: &[u8],
    parent_dir: &Path,
    entry_start: usize,
    payload_start: usize,
    full_buf: &[u8],
) -> Option<ParsedEntry> {
    let mut p: usize = 0;
    if body.len() < std::mem::size_of::<attribute_set_t>() {
        return None;
    }
    // returned_attrs — we asked for it, so it's first in the payload.
    let returned_commonattr = u32::from_ne_bytes(body[p..p + 4].try_into().unwrap());
    let returned_fileattr = u32::from_ne_bytes(body[p + 12..p + 16].try_into().unwrap());
    p += std::mem::size_of::<attribute_set_t>();

    // ATTR_CMN_NAME — variable-length, comes through as an
    // attrreference_t (relative offset + length).
    let mut name = String::new();
    if returned_commonattr & ATTR_CMN_NAME != 0 {
        if p + std::mem::size_of::<attrreference_t>() > body.len() {
            return None;
        }
        let dataoffset = i32::from_ne_bytes(body[p..p + 4].try_into().unwrap()) as isize;
        let length = u32::from_ne_bytes(body[p + 4..p + 8].try_into().unwrap()) as usize;
        // dataoffset is relative to the start of the attrreference
        // field itself in the kernel's reply. Compute the absolute
        // index into the original buffer.
        let abs_start = (payload_start + p) as isize + dataoffset;
        if abs_start >= 0 && (abs_start as usize) + length <= full_buf.len() {
            let raw = &full_buf[abs_start as usize..abs_start as usize + length];
            // The string is NUL-terminated; trim it.
            let nul = raw.iter().position(|&b| b == 0).unwrap_or(length);
            if let Ok(s) = std::str::from_utf8(&raw[..nul]) {
                name = s.to_string();
            } else {
                return None;
            }
        }
        p += std::mem::size_of::<attrreference_t>();
    }
    if name.is_empty() {
        return None;
    }

    // ATTR_CMN_OBJTYPE — fsobj_type_t (i32 actually but defined as u32-sized).
    let mut objtype: u32 = 0;
    if returned_commonattr & ATTR_CMN_OBJTYPE != 0 {
        if p + 4 > body.len() {
            return None;
        }
        objtype = u32::from_ne_bytes(body[p..p + 4].try_into().unwrap());
        p += 4;
    }

    // ATTR_CMN_FILEID — u64.
    let mut fileid: u64 = 0;
    if returned_commonattr & ATTR_CMN_FILEID != 0 {
        if p + 8 > body.len() {
            return None;
        }
        fileid = u64::from_ne_bytes(body[p..p + 8].try_into().unwrap());
        p += 8;
    }

    // ATTR_FILE_TOTALSIZE — u64. Only valid for VREG.
    let mut size: u64 = 0;
    if returned_fileattr & ATTR_FILE_TOTALSIZE != 0 && objtype == VREG {
        if p + 8 > body.len() {
            return None;
        }
        size = u64::from_ne_bytes(body[p..p + 8].try_into().unwrap());
    }

    let _ = entry_start;
    Some(ParsedEntry {
        path: parent_dir.join(&name),
        objtype,
        fileid,
        size,
    })
}

fn stat_dev_ino(path: &Path) -> Option<(u64, u64)> {
    use std::os::unix::fs::MetadataExt;
    std::fs::symlink_metadata(path)
        .ok()
        .map(|m| (m.dev(), m.ino()))
}
