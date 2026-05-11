//! OS-conventional socket / pipe path resolution.
//!
//! - **Windows.** `\\.\pipe\sourcerer-indexd-<userSid>`. The SID disambiguates
//!   per-user pipes on multi-user Windows hosts.
//! - **macOS.** `$HOME/Library/Application Support/sourcerer/indexd.sock`.
//! - **Linux.** `$XDG_RUNTIME_DIR/sourcerer/indexd.sock` when set; otherwise
//!   `$HOME/.local/share/sourcerer/indexd.sock`.

use std::path::PathBuf;

#[derive(Debug, Clone)]
pub enum SocketPath {
    /// Filesystem path (UDS).
    Path(PathBuf),
    /// Named-pipe path (e.g. `\\.\pipe\sourcerer-indexd-S-1-5-21-...`).
    Pipe(String),
}

#[cfg(target_os = "macos")]
pub fn default_socket_path() -> SocketPath {
    let home = std::env::var_os("HOME").unwrap_or_default();
    let mut p = PathBuf::from(home);
    p.push("Library");
    p.push("Application Support");
    p.push("sourcerer");
    p.push("indexd.sock");
    SocketPath::Path(p)
}

#[cfg(target_os = "linux")]
pub fn default_socket_path() -> SocketPath {
    if let Some(rt) = std::env::var_os("XDG_RUNTIME_DIR") {
        let mut p = PathBuf::from(rt);
        p.push("sourcerer");
        p.push("indexd.sock");
        return SocketPath::Path(p);
    }
    let home = std::env::var_os("HOME").unwrap_or_default();
    let mut p = PathBuf::from(home);
    p.push(".local");
    p.push("share");
    p.push("sourcerer");
    p.push("indexd.sock");
    SocketPath::Path(p)
}

#[cfg(windows)]
pub fn default_socket_path() -> SocketPath {
    SocketPath::Pipe(default_pipe_name())
}

#[cfg(windows)]
pub fn default_pipe_name() -> String {
    // Best-effort SID-tagged name. If we can't resolve the user SID, fall
    // back to a username-tagged name (still per-user, just less robust on
    // multi-user hosts).
    if let Some(sid) = current_user_sid_string() {
        return format!(r"\\.\pipe\sourcerer-indexd-{sid}");
    }
    let user = std::env::var("USERNAME").unwrap_or_else(|_| "user".to_string());
    format!(r"\\.\pipe\sourcerer-indexd-{user}")
}

/// Well-known pipe name for the elevated Windows service. Unlike the
/// per-user `default_pipe_name`, this is a single shared endpoint that
/// any logged-in user can connect to (DACL governs access — see
/// `service_sddl`). The Tauri UI prefers this pipe over spawning its
/// own child daemon when the service is installed.
#[cfg(windows)]
pub fn service_pipe_name() -> String {
    r"\\.\pipe\sourcerer-indexd".to_string()
}

/// SDDL string for the service-mode pipe. Grants GENERIC_ALL to:
///   * Authenticated Users (AU)  — any logged-in local user can connect
///   * SYSTEM (SY)               — the service itself
///
/// The pipe server already calls `reject_remote_clients(true)`, so the
/// AU grant does not extend across the network.
#[cfg(windows)]
pub fn service_sddl() -> String {
    "D:(A;;GA;;;AU)(A;;GA;;;SY)".to_string()
}

#[cfg(windows)]
fn current_user_sid_string() -> Option<String> {
    use windows::Win32::Foundation::HANDLE;
    use windows::Win32::Security::Authorization::ConvertSidToStringSidW;
    use windows::Win32::Security::{GetTokenInformation, PSID, TOKEN_QUERY, TOKEN_USER, TokenUser};
    use windows::Win32::System::Threading::{GetCurrentProcess, OpenProcessToken};
    use windows::core::PWSTR;

    unsafe {
        let mut token: HANDLE = HANDLE::default();
        OpenProcessToken(GetCurrentProcess(), TOKEN_QUERY, &mut token).ok()?;
        // Two-call pattern: first call returns the required size in `len`.
        let mut len: u32 = 0;
        let _ = GetTokenInformation(token, TokenUser, None, 0, &mut len);
        if len == 0 {
            let _ = windows::Win32::Foundation::CloseHandle(token);
            return None;
        }
        let mut buf = vec![0_u8; len as usize];
        if GetTokenInformation(
            token,
            TokenUser,
            Some(buf.as_mut_ptr() as *mut _),
            len,
            &mut len,
        )
        .is_err()
        {
            let _ = windows::Win32::Foundation::CloseHandle(token);
            return None;
        }
        let tu = &*(buf.as_ptr() as *const TOKEN_USER);
        let mut sid_str: PWSTR = PWSTR::null();
        let psid = PSID(tu.User.Sid.0);
        let res = ConvertSidToStringSidW(psid, &mut sid_str);
        let _ = windows::Win32::Foundation::CloseHandle(token);
        if res.is_err() || sid_str.is_null() {
            return None;
        }
        // PWSTR points to a heap allocation owned by the OS; copy into a
        // Rust String, then `LocalFree` it.
        let mut len: usize = 0;
        let mut p = sid_str.0;
        while *p != 0 {
            len += 1;
            p = p.add(1);
        }
        let slice = std::slice::from_raw_parts(sid_str.0, len);
        let s = String::from_utf16_lossy(slice);
        let _ = windows::Win32::Foundation::LocalFree(Some(windows::Win32::Foundation::HLOCAL(
            sid_str.0 as _,
        )));
        Some(s)
    }
}

#[cfg(all(not(target_os = "macos"), not(target_os = "linux"), not(windows)))]
pub fn default_socket_path() -> SocketPath {
    SocketPath::Path(PathBuf::from("/tmp/sourcerer-indexd.sock"))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn default_path_is_well_formed() {
        let p = default_socket_path();
        match p {
            SocketPath::Path(pb) => assert!(pb.is_absolute() || !pb.as_os_str().is_empty()),
            SocketPath::Pipe(name) => assert!(name.starts_with(r"\\.\pipe\")),
        }
    }
}
