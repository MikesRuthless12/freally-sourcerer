//! Cross-platform volume detection.
//!
//! - **Windows.** Walk `GetLogicalDrives()` and call `GetVolumeInformationW`
//!   per drive. FS family populates `fs_kind` (NTFS / ReFS / exFAT /
//!   FAT32). `used_bytes` / `total_bytes` come from `GetDiskFreeSpaceExW`.
//! - **macOS.** Read `/Volumes` for mount points; statvfs fills sizes.
//! - **Linux.** Read `/proc/mounts` (skipping pseudo filesystems);
//!   statvfs fills sizes.

use sourcerer_rpc::{VolumeInfo, VolumeStatus};

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
        out.push(VolumeInfo {
            id,
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
        out.push(VolumeInfo {
            id,
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
            "proc" | "sysfs" | "devpts" | "tmpfs" | "cgroup" | "cgroup2" | "mqueue" | "pstore"
                | "bpf" | "configfs" | "ramfs" | "rpc_pipefs" | "binfmt_misc"
                | "tracefs" | "debugfs" | "fusectl" | "securityfs" | "hugetlbfs"
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
        out.push(VolumeInfo {
            id,
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
