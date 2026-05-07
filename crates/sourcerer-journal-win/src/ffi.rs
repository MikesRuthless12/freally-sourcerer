//! Thin safe wrappers around the three FSCTL ops we need + USN record
//! iteration. Windows-only; the rest of the crate falls back to stubs on
//! non-Windows so workspace builds (clippy / cargo check) stay clean.

#![cfg(windows)]

use std::ffi::{c_void, OsStr, OsString};
use std::io;
use std::os::windows::ffi::{OsStrExt, OsStringExt};
use std::path::{Path, PathBuf};

use windows::core::PCWSTR;
use windows::Win32::Foundation::{CloseHandle, GENERIC_READ, HANDLE};
use windows::Win32::Storage::FileSystem::{
    CreateFileW, GetFinalPathNameByHandleW, GetVolumeInformationW, OpenFileById,
    FILE_ATTRIBUTE_DIRECTORY, FILE_FLAGS_AND_ATTRIBUTES, FILE_FLAG_BACKUP_SEMANTICS,
    FILE_ID_DESCRIPTOR, FILE_ID_DESCRIPTOR_0, FILE_SHARE_DELETE, FILE_SHARE_READ,
    FILE_SHARE_WRITE, FileIdType, GETFINALPATHNAMEBYHANDLE_FLAGS, OPEN_EXISTING,
};
use windows::Win32::System::IO::DeviceIoControl;
use windows::Win32::System::Ioctl::{
    FSCTL_ENUM_USN_DATA, FSCTL_QUERY_USN_JOURNAL, FSCTL_READ_USN_JOURNAL, MFT_ENUM_DATA_V0,
    READ_USN_JOURNAL_DATA_V0, USN_JOURNAL_DATA_V0, USN_RECORD_V2,
};

const VOLUME_HANDLE_SHARE: u32 = FILE_SHARE_READ.0 | FILE_SHARE_WRITE.0 | FILE_SHARE_DELETE.0;

/// Owned wrapper around an NTFS volume `\\.\X:` handle so we never leak it.
pub struct VolumeHandle {
    raw: HANDLE,
    /// User-friendly volume root (`X:\`) — kept around for diagnostics so
    /// errors don't surface raw `\\?\Volume{...}` paths.
    pub root: PathBuf,
}

impl VolumeHandle {
    /// Opens `\\.\<drive_letter>:` with the access bits required for USN
    /// FSCTL ops. Backup-intent + share-everything mirrors what voidtools'
    /// Everything does on the same APIs.
    pub fn open(volume_root: &Path) -> io::Result<Self> {
        let drive_letter = drive_letter_from_root(volume_root).ok_or_else(|| {
            io::Error::new(
                io::ErrorKind::InvalidInput,
                format!(
                    "expected a Windows drive root like `C:\\`, got `{}`",
                    volume_root.display()
                ),
            )
        })?;
        let device = format!("\\\\.\\{drive_letter}:");
        let wide = to_pcwstr(&device);

        // Safety: the wide buffer outlives the call; CreateFileW does not
        // retain the pointer.
        let handle = unsafe {
            CreateFileW(
                PCWSTR(wide.as_ptr()),
                GENERIC_READ.0,
                windows::Win32::Storage::FileSystem::FILE_SHARE_MODE(VOLUME_HANDLE_SHARE),
                None,
                OPEN_EXISTING,
                FILE_FLAG_BACKUP_SEMANTICS,
                None,
            )
        }
        .map_err(io::Error::other)?;

        Ok(Self {
            raw: handle,
            root: volume_root.to_path_buf(),
        })
    }

    pub fn raw(&self) -> HANDLE {
        self.raw
    }
}

impl Drop for VolumeHandle {
    fn drop(&mut self) {
        if !self.raw.is_invalid() {
            let _ = unsafe { CloseHandle(self.raw) };
        }
    }
}

// SAFETY: Win32 HANDLE values are independent kernel handles; sharing across
// threads is sound as long as we don't double-close. Drop runs once, on the
// thread that finally drops the wrapper.
unsafe impl Send for VolumeHandle {}
unsafe impl Sync for VolumeHandle {}

/// Result of `FSCTL_QUERY_USN_JOURNAL`.
#[derive(Debug, Clone, Copy)]
pub struct JournalState {
    pub journal_id: u64,
    pub first_usn: i64,
    pub next_usn: i64,
    pub lowest_valid_usn: i64,
    pub max_usn: i64,
}

pub fn query_usn_journal(volume: &VolumeHandle) -> io::Result<JournalState> {
    let mut data = USN_JOURNAL_DATA_V0::default();
    let mut bytes_returned: u32 = 0;
    // Safety: out-buffer is sized correctly; FSCTL_QUERY_USN_JOURNAL takes no
    // input buffer.
    let r = unsafe {
        DeviceIoControl(
            volume.raw,
            FSCTL_QUERY_USN_JOURNAL,
            None,
            0,
            Some(&mut data as *mut _ as *mut c_void),
            std::mem::size_of::<USN_JOURNAL_DATA_V0>() as u32,
            Some(&mut bytes_returned),
            None,
        )
    };
    r.map_err(io::Error::other)?;
    Ok(JournalState {
        journal_id: data.UsnJournalID,
        first_usn: data.FirstUsn,
        next_usn: data.NextUsn,
        lowest_valid_usn: data.LowestValidUsn,
        max_usn: data.MaxUsn,
    })
}

/// One round-trip of `FSCTL_ENUM_USN_DATA`. Returns the next file-reference
/// the caller should pass back in the *next* call (or `None` if exhausted),
/// plus the byte count written to `out`. The caller iterates the buffer with
/// [`UsnRecordIter::after_initial_frn`].
pub fn enum_usn_data(
    volume: &VolumeHandle,
    start_frn: u64,
    journal: &JournalState,
    out: &mut [u8],
) -> io::Result<Option<(u64, usize)>> {
    let input = MFT_ENUM_DATA_V0 {
        StartFileReferenceNumber: start_frn,
        LowUsn: 0,
        HighUsn: journal.next_usn,
    };
    let mut bytes_returned: u32 = 0;
    let r = unsafe {
        DeviceIoControl(
            volume.raw,
            FSCTL_ENUM_USN_DATA,
            Some(&input as *const _ as *const c_void),
            std::mem::size_of::<MFT_ENUM_DATA_V0>() as u32,
            Some(out.as_mut_ptr() as *mut c_void),
            out.len() as u32,
            Some(&mut bytes_returned),
            None,
        )
    };
    match r {
        Ok(()) => {
            if (bytes_returned as usize) < std::mem::size_of::<u64>() {
                return Ok(None);
            }
            let next_frn = u64::from_le_bytes(out[0..8].try_into().unwrap());
            // ENUM_USN_DATA returns 0 in the leading u64 once the MFT is
            // exhausted (NextStartFileReferenceNumber == 0).
            if next_frn == 0 {
                Ok(Some((0, bytes_returned as usize)))
            } else {
                Ok(Some((next_frn, bytes_returned as usize)))
            }
        }
        Err(e) => {
            // ERROR_HANDLE_EOF (38) = MFT walk complete; treat as exhausted.
            if e.code().0 as u32 == 0x8007_0026 {
                Ok(None)
            } else {
                Err(io::Error::other(e))
            }
        }
    }
}

/// One round-trip of `FSCTL_READ_USN_JOURNAL` with a short timeout. Returns
/// `(next_usn, bytes_returned)`. A `bytes_returned <= 8` indicates the
/// journal is idle (only the leading next-USN cursor was returned).
pub fn read_usn_journal(
    volume: &VolumeHandle,
    journal_id: u64,
    start_usn: i64,
    out: &mut [u8],
    timeout_100ns: u64,
) -> io::Result<(i64, usize)> {
    let input = READ_USN_JOURNAL_DATA_V0 {
        StartUsn: start_usn,
        ReasonMask: u32::MAX,
        ReturnOnlyOnClose: 0,
        Timeout: timeout_100ns,
        BytesToWaitFor: 1,
        UsnJournalID: journal_id,
    };
    let mut bytes_returned: u32 = 0;
    let r = unsafe {
        DeviceIoControl(
            volume.raw,
            FSCTL_READ_USN_JOURNAL,
            Some(&input as *const _ as *const c_void),
            std::mem::size_of::<READ_USN_JOURNAL_DATA_V0>() as u32,
            Some(out.as_mut_ptr() as *mut c_void),
            out.len() as u32,
            Some(&mut bytes_returned),
            None,
        )
    };
    r.map_err(io::Error::other)?;
    if (bytes_returned as usize) < std::mem::size_of::<i64>() {
        return Ok((start_usn, 0));
    }
    let next_usn = i64::from_le_bytes(out[0..8].try_into().unwrap());
    Ok((next_usn, bytes_returned as usize))
}

/// Iterator over `USN_RECORD_V2` records inside a buffer returned by either
/// `FSCTL_ENUM_USN_DATA` or `FSCTL_READ_USN_JOURNAL`. Both ops prefix the
/// buffer with a leading u64 (next FRN / next USN) — the caller must skip
/// it via [`UsnRecordIter::after_initial_frn`].
pub struct UsnRecordIter<'a> {
    buf: &'a [u8],
    offset: usize,
}

impl<'a> UsnRecordIter<'a> {
    pub fn after_initial_frn(buf: &'a [u8]) -> Self {
        Self {
            buf,
            offset: std::mem::size_of::<u64>().min(buf.len()),
        }
    }
}

#[derive(Debug, Clone)]
pub struct ParsedUsnRecord {
    pub usn: i64,
    pub file_ref: u64,
    pub parent_file_ref: u64,
    pub reason: u32,
    pub timestamp_filetime: i64,
    pub file_attributes: u32,
    pub major_version: u16,
    pub file_name: OsString,
}

impl ParsedUsnRecord {
    pub fn is_directory(&self) -> bool {
        self.file_attributes & FILE_ATTRIBUTE_DIRECTORY.0 != 0
    }
}

impl Iterator for UsnRecordIter<'_> {
    type Item = ParsedUsnRecord;

    fn next(&mut self) -> Option<ParsedUsnRecord> {
        if self.offset + std::mem::size_of::<u32>() > self.buf.len() {
            return None;
        }
        let record_len = u32::from_le_bytes(
            self.buf[self.offset..self.offset + 4].try_into().ok()?,
        ) as usize;
        if record_len == 0 || self.offset + record_len > self.buf.len() {
            return None;
        }
        // We consume V2; V3/V4 records (which use FILE_ID_128 — only present
        // on ReFS at the moment) are skipped. This is correct on NTFS.
        let major =
            u16::from_le_bytes(self.buf[self.offset + 4..self.offset + 6].try_into().ok()?);
        if major != 2 {
            self.offset += record_len;
            return self.next();
        }

        let rec_bytes = &self.buf[self.offset..self.offset + record_len];
        // Safety: we've bounds-checked record_len above; USN_RECORD_V2 has a
        // C layout with a trailing variable-length name, so we read fixed
        // fields from the prefix and the name from the byte slice directly.
        let header = unsafe { &*(rec_bytes.as_ptr() as *const USN_RECORD_V2) };

        let file_name_offset = header.FileNameOffset as usize;
        let file_name_length = header.FileNameLength as usize; // bytes
        let name = if file_name_offset + file_name_length <= record_len && file_name_length > 0 {
            let name_bytes = &rec_bytes[file_name_offset..file_name_offset + file_name_length];
            // FileNameLength is in bytes; the buffer is u16 wide chars.
            let mut wide = Vec::with_capacity(file_name_length / 2);
            for chunk in name_bytes.chunks_exact(2) {
                wide.push(u16::from_le_bytes([chunk[0], chunk[1]]));
            }
            OsString::from_wide(&wide)
        } else {
            OsString::new()
        };

        let parsed = ParsedUsnRecord {
            usn: header.Usn,
            file_ref: header.FileReferenceNumber,
            parent_file_ref: header.ParentFileReferenceNumber,
            reason: header.Reason,
            timestamp_filetime: header.TimeStamp,
            file_attributes: header.FileAttributes,
            major_version: header.MajorVersion,
            file_name: name,
        };

        self.offset += record_len;
        Some(parsed)
    }
}

/// Resolves an NTFS file reference number to its full path on the volume by
/// `OpenFileById` + `GetFinalPathNameByHandleW`. Returns `None` if the file
/// no longer exists (deleted) — caller should fall back to its FRN-cache.
pub fn resolve_path_by_frn(volume: &VolumeHandle, frn: u64) -> io::Result<Option<PathBuf>> {
    let descriptor = FILE_ID_DESCRIPTOR {
        dwSize: std::mem::size_of::<FILE_ID_DESCRIPTOR>() as u32,
        Type: FileIdType,
        Anonymous: FILE_ID_DESCRIPTOR_0 {
            FileId: frn as i64,
        },
    };
    let result = unsafe {
        OpenFileById(
            volume.raw,
            &descriptor,
            0, // 0 desired access — sufficient for GetFinalPathNameByHandleW
            windows::Win32::Storage::FileSystem::FILE_SHARE_MODE(VOLUME_HANDLE_SHARE),
            None,
            FILE_FLAGS_AND_ATTRIBUTES(FILE_FLAG_BACKUP_SEMANTICS.0),
        )
    };
    let h = match result {
        Ok(h) => h,
        Err(e) => {
            // ERROR_FILE_NOT_FOUND / ERROR_PATH_NOT_FOUND / ERROR_INVALID_PARAMETER
            // all surface here when the file's been deleted between the USN
            // event and our resolve. Bubble Ok(None) so the subscriber can
            // fall back to its FRN cache.
            let code = e.code().0 as u32 & 0xFFFF;
            if matches!(code, 2 | 3 | 21 | 87) {
                return Ok(None);
            }
            return Err(io::Error::other(e));
        }
    };

    let mut buf = [0u16; 1024];
    // Safety: out-buffer length is in u16s.
    let len = unsafe {
        GetFinalPathNameByHandleW(h, &mut buf, GETFINALPATHNAMEBYHANDLE_FLAGS(0))
    };
    let _ = unsafe { CloseHandle(h) };
    if len == 0 {
        return Err(io::Error::last_os_error());
    }
    let len = len as usize;
    if len >= buf.len() {
        // Path too long — extremely rare on test paths. Caller can retry
        // with a larger buffer; for Phase 1 we surface an error.
        return Err(io::Error::new(
            io::ErrorKind::InvalidData,
            "resolved path exceeds 1024 wide chars",
        ));
    }
    let os = OsString::from_wide(&buf[..len]);
    let path = strip_extended_prefix(&os);
    Ok(Some(path))
}

/// `GetFinalPathNameByHandleW` returns the `\\?\` extended prefix; strip it
/// for ergonomics so callers see ordinary `C:\foo\bar` paths.
fn strip_extended_prefix(s: &OsStr) -> PathBuf {
    let lossy = s.to_string_lossy();
    if let Some(stripped) = lossy.strip_prefix("\\\\?\\") {
        // Handle `\\?\UNC\server\share\...` → `\\server\share\...`
        if let Some(unc) = stripped.strip_prefix("UNC\\") {
            return PathBuf::from(format!("\\\\{unc}"));
        }
        return PathBuf::from(stripped.to_string());
    }
    PathBuf::from(s.to_os_string())
}

/// Reads the volume's filesystem metadata (`GetVolumeInformationW`).
pub fn volume_info(volume_root: &Path) -> io::Result<VolumeInfo> {
    let drive_letter = drive_letter_from_root(volume_root).ok_or_else(|| {
        io::Error::new(
            io::ErrorKind::InvalidInput,
            "expected a Windows drive root path like `C:\\`",
        )
    })?;
    let path = format!("{drive_letter}:\\");
    let wide = to_pcwstr(&path);

    let mut name_buf = [0u16; 32];
    let mut serial: u32 = 0;
    let mut max_component: u32 = 0;
    let mut fs_flags: u32 = 0;
    let mut fs_buf = [0u16; 32];
    unsafe {
        GetVolumeInformationW(
            PCWSTR(wide.as_ptr()),
            Some(&mut name_buf),
            Some(&mut serial),
            Some(&mut max_component),
            Some(&mut fs_flags),
            Some(&mut fs_buf),
        )
    }
    .map_err(io::Error::other)?;
    Ok(VolumeInfo {
        serial,
        fs_name: wide_str_truncated_to_string(&fs_buf),
    })
}

#[derive(Debug, Clone)]
pub struct VolumeInfo {
    pub serial: u32,
    pub fs_name: String,
}

fn drive_letter_from_root(p: &Path) -> Option<char> {
    let s = p.to_string_lossy();
    let bytes = s.as_bytes();
    if bytes.len() >= 2
        && bytes[1] == b':'
        && (bytes[0].is_ascii_alphabetic())
        && (bytes.len() == 2 || bytes[2] == b'\\' || bytes[2] == b'/')
    {
        Some(bytes[0].to_ascii_uppercase() as char)
    } else {
        None
    }
}

fn to_pcwstr(s: &str) -> Vec<u16> {
    OsStr::new(s).encode_wide().chain(std::iter::once(0)).collect()
}

fn wide_str_truncated_to_string(buf: &[u16]) -> String {
    let end = buf.iter().position(|&c| c == 0).unwrap_or(buf.len());
    String::from_utf16_lossy(&buf[..end])
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn drive_letter_parses_root_forms() {
        assert_eq!(drive_letter_from_root(Path::new("C:\\")), Some('C'));
        assert_eq!(drive_letter_from_root(Path::new("c:\\")), Some('C'));
        assert_eq!(drive_letter_from_root(Path::new("D:")), Some('D'));
        assert_eq!(drive_letter_from_root(Path::new("E:/")), Some('E'));
        assert_eq!(drive_letter_from_root(Path::new("\\\\server\\share")), None);
        assert_eq!(drive_letter_from_root(Path::new("relative")), None);
    }

    #[test]
    fn extended_prefix_stripped() {
        let p = strip_extended_prefix(OsStr::new("\\\\?\\C:\\Users\\foo"));
        assert_eq!(p, PathBuf::from("C:\\Users\\foo"));
        let p = strip_extended_prefix(OsStr::new("\\\\?\\UNC\\srv\\share\\file"));
        assert_eq!(p, PathBuf::from("\\\\srv\\share\\file"));
        let p = strip_extended_prefix(OsStr::new("C:\\plain"));
        assert_eq!(p, PathBuf::from("C:\\plain"));
    }
}
