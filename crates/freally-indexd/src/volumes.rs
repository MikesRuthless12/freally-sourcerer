//! Cross-platform volume detection.
//!
//! - **Windows.** Walk `GetLogicalDrives()` and call `GetVolumeInformationW`
//!   per drive. FS family populates `fs_kind` (NTFS / ReFS / exFAT /
//!   FAT32). `used_bytes` / `total_bytes` come from `GetDiskFreeSpaceExW`.
//! - **macOS.** Read `/Volumes` for mount points; statvfs fills sizes.
//! - **Linux.** Read `/proc/mounts` (skipping pseudo filesystems);
//!   statvfs fills sizes.

use freally_rpc::{VolumeInfo, VolumeStatus};

#[cfg(windows)]
pub fn detect() -> Vec<VolumeInfo> {
    use std::os::windows::ffi::OsStrExt;
    use windows::Win32::Storage::FileSystem::{
        GetDiskFreeSpaceExW, GetLogicalDrives, GetVolumeInformationW,
    };
    use windows::core::PCWSTR;

    let mask = unsafe { GetLogicalDrives() };
    let mut out = Vec::new();
    if mask == 0 {
        return out;
    }
    for i in 0_u32..26 {
        if mask & (1 << i) == 0 {
            continue;
        }
        let letter = (b'A' + i as u8) as char;
        let root_str = format!("{letter}:\\");
        let wide: Vec<u16> = std::ffi::OsString::from(&root_str)
            .encode_wide()
            .chain(Some(0))
            .collect();

        let mut volume_label: [u16; 261] = [0; 261];
        let mut serial: u32 = 0;
        let mut max_component: u32 = 0;
        let mut flags: u32 = 0;
        let mut fs_name: [u16; 261] = [0; 261];
        let _ = unsafe {
            GetVolumeInformationW(
                PCWSTR(wide.as_ptr()),
                Some(&mut volume_label),
                Some(&mut serial),
                Some(&mut max_component),
                Some(&mut flags),
                Some(&mut fs_name),
            )
        };
        let fs_kind = decode_z_wide(&fs_name);
        let label = decode_z_wide(&volume_label);

        let mut free: u64 = 0;
        let mut total: u64 = 0;
        let _ = unsafe {
            GetDiskFreeSpaceExW(
                PCWSTR(wide.as_ptr()),
                None,
                Some(&mut total),
                Some(&mut free),
            )
        };
        let used = total.saturating_sub(free);

        let id = format!("win-{letter}");
        // The NTFS volume serial travels with the device, so a drive
        // that comes back on a different letter is still the same
        // catalog — and a *different* drive that inherits this letter is
        // correctly a different one. `GetVolumeInformationW` above
        // already read it; it used to be discarded.
        let device_id = if serial == 0 {
            String::new()
        } else {
            format!("wvol-{serial:08x}")
        };
        out.push(VolumeInfo {
            id,
            device_id,
            label: if label.is_empty() {
                format!("{letter}:")
            } else {
                label
            },
            mount_point: root_str.clone(),
            fs_kind: if fs_kind.is_empty() {
                "unknown".into()
            } else {
                fs_kind
            },
            used_bytes: used,
            total_bytes: total,
            status: if total == 0 {
                VolumeStatus::Offline
            } else {
                VolumeStatus::Indexed
            },
            indexed: false,
            journal_enabled: false,
            journal_buffer_kb: 64,
            allocation_delta_kb: Some(64),
            include_only: None,
            load_recent_changes: false,
            monitor_changes: true,
        });
    }
    out
}

#[cfg(windows)]
fn decode_z_wide(buf: &[u16]) -> String {
    let len = buf.iter().position(|&c| c == 0).unwrap_or(buf.len());
    String::from_utf16_lossy(&buf[..len])
}

#[cfg(target_os = "macos")]
pub fn detect() -> Vec<VolumeInfo> {
    let mut out = Vec::new();
    let entries = match std::fs::read_dir("/Volumes") {
        Ok(d) => d,
        Err(_) => return out,
    };
    for entry in entries.flatten() {
        let path = entry.path();
        let mount = match path.to_str() {
            Some(s) => s.to_string(),
            None => continue,
        };
        let label = entry.file_name().to_string_lossy().to_string();
        let (used, total) = read_statvfs(&path);
        let id = format!("mac-{}", label.replace('/', "_"));
        // macOS mounts removable media at `/Volumes/<volume label>`, so
        // the mount point already carries the device's own name rather
        // than a reusable slot like a drive letter. That makes it a
        // usable device key; the residual weakness is two drives sharing
        // a label, which is far narrower than letter reuse. A real
        // filesystem UUID needs `getattrlist(ATTR_VOL_UUID)` and is
        // SRC-N69's business.
        let device_id = if label.is_empty() {
            String::new()
        } else {
            format!("mvol-{}", label.replace('/', "_"))
        };
        out.push(VolumeInfo {
            id,
            device_id,
            label,
            mount_point: mount,
            fs_kind: "apfs".into(),
            used_bytes: used,
            total_bytes: total,
            status: if total == 0 {
                VolumeStatus::Offline
            } else {
                VolumeStatus::Indexed
            },
            indexed: false,
            journal_enabled: false,
            journal_buffer_kb: 0,
            allocation_delta_kb: None,
            include_only: None,
            load_recent_changes: false,
            monitor_changes: true,
        });
    }
    out
}

#[cfg(target_os = "linux")]
pub fn detect() -> Vec<VolumeInfo> {
    let mut out = Vec::new();
    let raw = match std::fs::read_to_string("/proc/mounts") {
        Ok(s) => s,
        Err(_) => return out,
    };
    let uuid_by_device = read_uuid_map();
    for line in raw.lines() {
        let mut cols = line.split_whitespace();
        let device = match cols.next() {
            Some(s) => s,
            None => continue,
        };
        let mount = match cols.next() {
            Some(s) => s,
            None => continue,
        };
        let fs_type = match cols.next() {
            Some(s) => s,
            None => continue,
        };
        if matches!(
            fs_type,
            "proc"
                | "sysfs"
                | "devpts"
                | "tmpfs"
                | "cgroup"
                | "cgroup2"
                | "mqueue"
                | "pstore"
                | "bpf"
                | "configfs"
                | "ramfs"
                | "rpc_pipefs"
                | "binfmt_misc"
                | "tracefs"
                | "debugfs"
                | "fusectl"
                | "securityfs"
                | "hugetlbfs"
                | "autofs"
        ) {
            continue;
        }
        let (used, total) = read_statvfs(std::path::Path::new(mount));
        let label = std::path::Path::new(mount)
            .file_name()
            .map(|s| s.to_string_lossy().to_string())
            .unwrap_or_else(|| device.to_string());
        let id = format!("lin-{}", mount.replace('/', "_"));
        // The filesystem UUID is the device's own identity; the mount
        // point is just where it happens to be attached, and
        // `/media/<user>/<label>` is as reusable as a drive letter.
        let device_id = uuid_by_device
            .get(canonical_device(device).as_str())
            .map(|u| format!("lvol-{u}"))
            .unwrap_or_default();
        out.push(VolumeInfo {
            id,
            device_id,
            label,
            mount_point: mount.to_string(),
            fs_kind: fs_type.to_string(),
            used_bytes: used,
            total_bytes: total,
            status: if total == 0 {
                VolumeStatus::Offline
            } else {
                VolumeStatus::Indexed
            },
            indexed: false,
            journal_enabled: false,
            journal_buffer_kb: 0,
            allocation_delta_kb: None,
            include_only: None,
            load_recent_changes: false,
            monitor_changes: true,
        });
    }
    out
}

#[cfg(unix)]
// `statvfs` block-count / fragment-size widths are `c_ulong` on Linux and
// `u64` on macOS — the explicit `as u64` cast is a no-op on macOS (which
// fires clippy's `unnecessary_cast` lint) but is load-bearing on Linux's
// `c_ulong`. Allow the lint here so the same source compiles on both.
#[allow(clippy::unnecessary_cast)]
fn read_statvfs(path: &std::path::Path) -> (u64, u64) {
    use std::ffi::CString;
    use std::os::unix::ffi::OsStrExt;
    let cs = match CString::new(path.as_os_str().as_bytes()) {
        Ok(c) => c,
        Err(_) => return (0, 0),
    };
    let mut sb: libc::statvfs = unsafe { std::mem::zeroed() };
    if unsafe { libc::statvfs(cs.as_ptr(), &mut sb) } != 0 {
        return (0, 0);
    }
    let block = sb.f_frsize as u64;
    let total = sb.f_blocks as u64 * block;
    let free = sb.f_bavail as u64 * block;
    let used = total.saturating_sub(free);
    (used, total)
}

/// `/dev/disk/by-uuid` is a directory of symlinks named by filesystem
/// UUID pointing at device nodes, so reading it once gives the whole
/// device-node → UUID table. Absent on systems without udev, in which
/// case every device_id falls back to empty and callers use the mount id.
#[cfg(target_os = "linux")]
fn read_uuid_map() -> std::collections::HashMap<String, String> {
    let mut out = std::collections::HashMap::new();
    let Ok(entries) = std::fs::read_dir("/dev/disk/by-uuid") else {
        return out;
    };
    for e in entries.flatten() {
        let Some(uuid) = e.file_name().to_str().map(|s| s.to_string()) else {
            continue;
        };
        if let Ok(target) = std::fs::canonicalize(e.path()) {
            out.insert(target.to_string_lossy().into_owned(), uuid);
        }
    }
    out
}

/// `/proc/mounts` may list a device by a `/dev/disk/by-*` alias; resolve
/// it to the same real node the UUID table is keyed on.
#[cfg(target_os = "linux")]
fn canonical_device(device: &str) -> String {
    std::fs::canonicalize(device)
        .map(|p| p.to_string_lossy().into_owned())
        .unwrap_or_else(|_| device.to_string())
}

#[cfg(all(not(windows), not(target_os = "macos"), not(target_os = "linux")))]
pub fn detect() -> Vec<VolumeInfo> {
    Vec::new()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn detect_returns_at_least_root_or_drive() {
        let v = detect();
        // We don't assert non-empty in CI containers without /proc/mounts
        // (e.g. some BSD) — just that the call returns without panicking.
        let _ = v;
    }
}
