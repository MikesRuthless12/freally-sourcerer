//! OS-conventional socket / pipe path resolution.
//!
//! - **Windows.** `\\.\pipe\freally-indexd-<userSid>`. The SID disambiguates
//!   per-user pipes on multi-user Windows hosts.
//! - **macOS.** `$HOME/Library/Application Support/freally/indexd.sock`.
//! - **Linux.** `$XDG_RUNTIME_DIR/freally/indexd.sock` when set; otherwise
//!   `$HOME/.local/share/freally/indexd.sock`.

use std::path::PathBuf;

#[derive(Debug, Clone)]
pub enum SocketPath {
    /// Filesystem path (UDS).
    Path(PathBuf),
    /// Named-pipe path (e.g. `\\.\pipe\freally-indexd-S-1-5-21-...`).
    Pipe(String),
}

/// The per-OS directory both sockets live in, with `file` as the leaf.
/// Two endpoints share it — the daemon a user runs themselves and the one
/// the service manager runs — so the directory rule is written once.
#[cfg(target_os = "macos")]
fn unix_socket(file: &str) -> SocketPath {
    let home = std::env::var_os("HOME").unwrap_or_default();
    let mut p = PathBuf::from(home);
    p.push("Library");
    p.push("Application Support");
    p.push("freally");
    p.push(file);
    SocketPath::Path(p)
}

#[cfg(target_os = "linux")]
fn unix_socket(file: &str) -> SocketPath {
    if let Some(rt) = std::env::var_os("XDG_RUNTIME_DIR") {
        let mut p = PathBuf::from(rt);
        p.push("freally");
        p.push(file);
        return SocketPath::Path(p);
    }
    let home = std::env::var_os("HOME").unwrap_or_default();
    let mut p = PathBuf::from(home);
    p.push(".local");
    p.push("share");
    p.push("freally");
    p.push(file);
    SocketPath::Path(p)
}

#[cfg(any(target_os = "macos", target_os = "linux"))]
pub fn default_socket_path() -> SocketPath {
    unix_socket("indexd.sock")
}

/// Well-known endpoint for the **installed** daemon — the Windows
/// service, the launchd agent, or the systemd user unit — as opposed to
/// the one a user or the app starts for itself.
///
/// The two are deliberately separate names. A user can have both at once:
/// the app spawns its own child at [`default_socket_path`] whenever it
/// cannot find an installed daemon, and if the installed one later starts
/// they must not fight over one endpoint. Keeping them apart is also what
/// lets the app *prefer* the installed daemon — it probes this path first
/// and only falls back to spawning.
///
/// On Windows this is machine-wide, because the service runs as SYSTEM
/// and serves every logged-in user (`service_sddl` governs access). On
/// macOS and Linux the installed daemon is a **per-user** agent — a
/// LaunchAgent and a `systemd --user` unit, both running as the logged-in
/// user, never root — so its endpoint is per-user too.
#[cfg(any(target_os = "macos", target_os = "linux"))]
pub fn service_socket_path() -> SocketPath {
    unix_socket("indexd-service.sock")
}

/// See the macOS / Linux twin above.
#[cfg(windows)]
pub fn service_socket_path() -> SocketPath {
    SocketPath::Pipe(service_pipe_name())
}

/// The endpoint `FREALLY_RPC_SOCKET` pins, if it is set.
///
/// A smoke test or dev session that sets it is naming the daemon it
/// means, and every consumer has to agree on that — including the
/// app's installed-daemon probe, which must stand down rather than
/// silently adopt the machine's own daemon and run against the wrong
/// index. Parsing lives here because this is the crate that decides what
/// an endpoint string looks like: a value starting with a pipe prefix is
/// a named pipe, anything else a filesystem path.
pub fn socket_override() -> Option<SocketPath> {
    let raw = std::env::var("FREALLY_RPC_SOCKET").ok()?;
    if raw.is_empty() {
        return None;
    }
    Some(parse_socket(&raw))
}

/// Interpret an endpoint string the way [`socket_override`] does.
pub fn parse_socket(s: &str) -> SocketPath {
    if s.starts_with(r"\\.\pipe\") || s.starts_with(r"\\?\pipe\") {
        SocketPath::Pipe(s.to_string())
    } else {
        SocketPath::Path(PathBuf::from(s))
    }
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
        return format!(r"\\.\pipe\freally-indexd-{sid}");
    }
    let user = std::env::var("USERNAME").unwrap_or_else(|_| "user".to_string());
    format!(r"\\.\pipe\freally-indexd-{user}")
}

/// Well-known pipe name for the elevated Windows service. Unlike the
/// per-user `default_pipe_name`, this is a single shared endpoint that
/// any logged-in user can connect to (DACL governs access — see
/// `service_sddl`). The Tauri UI prefers this pipe over spawning its
/// own child daemon when the service is installed.
#[cfg(windows)]
pub fn service_pipe_name() -> String {
    r"\\.\pipe\freally-indexd".to_string()
}

/// SDDL for the service-mode pipe.
///
/// - `(A;;FRFW;;;AU)` — `FILE_GENERIC_READ | FILE_GENERIC_WRITE` for any
///   logged-in local user. This used to be `GENERIC_ALL`, which also
///   carried `WRITE_DAC` and `WRITE_OWNER` — a client that could rewrite
///   the pipe's own ACL.
/// - `(A;;GA;;;SY)` — the service itself.
///
/// The server also calls `reject_remote_clients(true)`, so none of this
/// extends across the network.
///
/// # Two gaps this DACL does not close
///
/// **Instance squatting.** `FILE_CREATE_PIPE_INSTANCE` lets a caller add
/// their own instances to an existing pipe name, and clients round-robin
/// across instances — so another local user could intercept a share of
/// connections and, because the app mints `Provenance::QueryHit` for every
/// path in a `query:batch`, forge the attestation that `files_delete` /
/// `files_rename` / `shell_verbs` gate on.
///
/// The obvious mitigation — a `(D;;CC;;;AU)` deny ACE — **does not work,
/// and this was measured, not assumed.** `FILE_CREATE_PIPE_INSTANCE` is
/// `0x0004`, which is the *same bit* as `FILE_APPEND_DATA`, and
/// `FILE_GENERIC_WRITE` includes it. Tokio's client opens with
/// `GENERIC_READ | GENERIC_WRITE`, so denying the create right denies
/// every legitimate client: `phase_13_daemon_service` fails with
/// `Access is denied. (os error 5)`. There is no access mask that admits
/// a `GENERIC_WRITE` client and refuses instance creation. The fix has to
/// be on the **client** side — `GetNamedPipeServerProcessId` and verify
/// the peer is the registered service — not in this string.
///
/// **No peer check on accept.** Unlike the Unix listener, the Windows
/// accept loop does not look at who connected.
///
/// Both are written up under TASK-102 in `docs/ROADMAP.md`. Until they
/// are closed, the installed Windows service is not safe on a machine
/// with more than one account.
#[cfg(windows)]
pub fn service_sddl() -> String {
    "D:(A;;FRFW;;;AU)(A;;GA;;;SY)".to_string()
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
    SocketPath::Path(PathBuf::from("/tmp/freally-indexd.sock"))
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
