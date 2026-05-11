//! Named-pipe transport on Windows.
//!
//! The pipe is created with a SECURITY_DESCRIPTOR that grants
//! `GENERIC_READ | GENERIC_WRITE` only to the current user's SID and
//! `GENERIC_ALL` to the system account. No `Everyone`, no `Authenticated
//! Users`. Combined with the per-user pipe name (SID-tagged in
//! `path::default_pipe_name`), this prevents any other user on the
//! system from connecting.

use std::ffi::OsStr;
use std::os::windows::ffi::OsStrExt;
use std::ptr::null_mut;

use tokio::net::windows::named_pipe::{
    ClientOptions, NamedPipeClient, NamedPipeServer, ServerOptions,
};
use windows::Win32::Foundation::{CloseHandle, HLOCAL, LocalFree};
use windows::Win32::Security::Authorization::{
    ConvertStringSecurityDescriptorToSecurityDescriptorW, SDDL_REVISION_1,
};
use windows::Win32::Security::{
    GetTokenInformation, PSECURITY_DESCRIPTOR, PSID, SECURITY_ATTRIBUTES, TOKEN_QUERY, TOKEN_USER,
    TokenUser,
};
use windows::Win32::System::Threading::{GetCurrentProcess, OpenProcessToken};
use windows::core::{PCWSTR, PWSTR};

use crate::error::{RpcError, RpcResult};

/// Create a NamedPipeServer instance with a DACL that admits only the
/// current user (and SYSTEM). Subsequent calls to `create` re-use the
/// same security descriptor for additional pipe instances.
pub fn listen(pipe_name: &str) -> RpcResult<NamedPipeServer> {
    listen_with_sddl(pipe_name, None)
}

/// Create one additional pipe-instance with the same DACL. Used by the
/// server's accept loop to keep at least one pending instance available
/// while previous ones service connections.
pub fn create_next_instance(pipe_name: &str) -> RpcResult<NamedPipeServer> {
    create_next_instance_with_sddl(pipe_name, None)
}

/// `listen` variant that lets the caller pin the DACL (used by the
/// Windows service so logged-in users can connect to the elevated
/// pipe). When `sddl_override` is `None`, falls back to per-user SDDL.
pub fn listen_with_sddl(
    pipe_name: &str,
    sddl_override: Option<&str>,
) -> RpcResult<NamedPipeServer> {
    let sd_string = match sddl_override {
        Some(s) => s.to_string(),
        None => current_user_sddl()
            .ok_or_else(|| RpcError::Other("could not resolve current-user SID".into()))?,
    };
    let pipe = unsafe { create_first_instance(pipe_name, &sd_string) }?;
    Ok(pipe)
}

/// `create_next_instance` variant that pins the DACL.
pub fn create_next_instance_with_sddl(
    pipe_name: &str,
    sddl_override: Option<&str>,
) -> RpcResult<NamedPipeServer> {
    let sd_string = match sddl_override {
        Some(s) => s.to_string(),
        None => current_user_sddl()
            .ok_or_else(|| RpcError::Other("could not resolve current-user SID".into()))?,
    };
    let pipe = unsafe { create_subsequent_instance(pipe_name, &sd_string) }?;
    Ok(pipe)
}

pub async fn connect(pipe_name: &str) -> RpcResult<NamedPipeClient> {
    // Retry on PIPE_BUSY (server saturated) and on FILE_NOT_FOUND
    // (sidecar hasn't created the pipe yet) so a client that wins the
    // race against a freshly-spawned server still connects. Bounded so
    // a truly absent daemon surfaces the error instead of hanging.
    let deadline = std::time::Instant::now() + std::time::Duration::from_secs(5);
    loop {
        match ClientOptions::new().open(pipe_name) {
            Ok(c) => return Ok(c),
            Err(e)
                if matches!(
                    e.raw_os_error(),
                    Some(231) /* ERROR_PIPE_BUSY */ | Some(2) /* ERROR_FILE_NOT_FOUND */
                ) && std::time::Instant::now() < deadline =>
            {
                tokio::time::sleep(std::time::Duration::from_millis(50)).await;
                continue;
            }
            Err(e) => return Err(RpcError::Io(e)),
        }
    }
}

unsafe fn create_first_instance(pipe_name: &str, sddl: &str) -> RpcResult<NamedPipeServer> {
    let (mut sa, _sd_drop) = unsafe { build_security_attrs(sddl)? };
    let server = unsafe {
        ServerOptions::new()
            .first_pipe_instance(true)
            .access_inbound(true)
            .access_outbound(true)
            .reject_remote_clients(true)
            .max_instances(254)
            .pipe_mode(tokio::net::windows::named_pipe::PipeMode::Byte)
            .create_with_security_attributes_raw(pipe_name, &mut sa as *mut _ as *mut _)?
    };
    Ok(server)
}

unsafe fn create_subsequent_instance(pipe_name: &str, sddl: &str) -> RpcResult<NamedPipeServer> {
    let (mut sa, _sd_drop) = unsafe { build_security_attrs(sddl)? };
    let server = unsafe {
        ServerOptions::new()
            .first_pipe_instance(false)
            .access_inbound(true)
            .access_outbound(true)
            .reject_remote_clients(true)
            .max_instances(254)
            .pipe_mode(tokio::net::windows::named_pipe::PipeMode::Byte)
            .create_with_security_attributes_raw(pipe_name, &mut sa as *mut _ as *mut _)?
    };
    Ok(server)
}

/// Drop guard around `LocalFree` for the security descriptor.
struct SdDrop(*mut std::ffi::c_void);
impl Drop for SdDrop {
    fn drop(&mut self) {
        if !self.0.is_null() {
            unsafe {
                let _ = LocalFree(Some(HLOCAL(self.0)));
            }
        }
    }
}

unsafe fn build_security_attrs(sddl: &str) -> RpcResult<(SECURITY_ATTRIBUTES, SdDrop)> {
    let wide: Vec<u16> = OsStr::new(sddl).encode_wide().chain(Some(0)).collect();
    let mut sd: PSECURITY_DESCRIPTOR = PSECURITY_DESCRIPTOR(null_mut());
    let res = unsafe {
        ConvertStringSecurityDescriptorToSecurityDescriptorW(
            PCWSTR(wide.as_ptr()),
            SDDL_REVISION_1,
            &mut sd,
            None,
        )
    };
    res.map_err(|e| {
        RpcError::Other(format!(
            "ConvertStringSecurityDescriptorToSecurityDescriptorW failed: {e}"
        ))
    })?;
    let sa = SECURITY_ATTRIBUTES {
        nLength: std::mem::size_of::<SECURITY_ATTRIBUTES>() as u32,
        lpSecurityDescriptor: sd.0,
        bInheritHandle: false.into(),
    };
    let drop = SdDrop(sd.0);
    Ok((sa, drop))
}

fn current_user_sddl() -> Option<String> {
    let sid = current_user_sid_string()?;
    // SDDL: D:(A;;GA;;;<sid>)(A;;GA;;;SY)
    //   D: = DACL section
    //   A = Access Allowed ACE
    //   GA = GENERIC_ALL
    //   SY = NT AUTHORITY\SYSTEM
    Some(format!("D:(A;;GA;;;{sid})(A;;GA;;;SY)"))
}

fn current_user_sid_string() -> Option<String> {
    use windows::Win32::Foundation::HANDLE;
    use windows::Win32::Security::Authorization::ConvertSidToStringSidW;

    unsafe {
        let mut token = HANDLE::default();
        OpenProcessToken(GetCurrentProcess(), TOKEN_QUERY, &mut token).ok()?;
        let mut len: u32 = 0;
        let _ = GetTokenInformation(token, TokenUser, None, 0, &mut len);
        if len == 0 {
            let _ = CloseHandle(token);
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
            let _ = CloseHandle(token);
            return None;
        }
        let tu = &*(buf.as_ptr() as *const TOKEN_USER);
        let mut sid_str: PWSTR = PWSTR::null();
        let psid = PSID(tu.User.Sid.0);
        let res = ConvertSidToStringSidW(psid, &mut sid_str);
        let _ = CloseHandle(token);
        if res.is_err() || sid_str.is_null() {
            return None;
        }
        let mut len: usize = 0;
        let mut p = sid_str.0;
        while *p != 0 {
            len += 1;
            p = p.add(1);
        }
        let slice = std::slice::from_raw_parts(sid_str.0, len);
        let s = String::from_utf16_lossy(slice);
        let _ = LocalFree(Some(HLOCAL(sid_str.0 as _)));
        Some(s)
    }
}
