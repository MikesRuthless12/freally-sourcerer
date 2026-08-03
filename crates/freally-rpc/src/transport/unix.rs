//! Unix-domain-socket transport with peer-uid auth.
//!
//! On `accept()`, we call `peer_cred()` and reject any peer whose UID
//! does not match the current process. Combined with file-mode `0600`
//! on the socket file, this ensures only the same user can connect.

use std::io;
use std::os::unix::fs::MetadataExt;
use std::os::unix::fs::PermissionsExt;
use std::path::{Path, PathBuf};

use tokio::net::{UnixListener, UnixStream};

use crate::error::{RpcError, RpcResult};

pub trait UnixListenerExt {
    fn accept_authenticated(
        &self,
    ) -> impl std::future::Future<Output = RpcResult<UnixStream>> + Send;
}

impl UnixListenerExt for UnixListener {
    async fn accept_authenticated(&self) -> RpcResult<UnixStream> {
        loop {
            let (stream, _addr) = self.accept().await?;
            match stream.peer_cred() {
                Ok(cred) => {
                    let our_uid =
                        // SAFETY: `getuid` is always-safe; it returns the current real UID.
                        unsafe { libc::getuid() };
                    if cred.uid() != our_uid {
                        tracing::warn!(
                            peer_uid = cred.uid(),
                            our_uid,
                            "rejecting connection from foreign uid"
                        );
                        drop(stream);
                        continue;
                    }
                    return Ok(stream);
                }
                Err(e) => {
                    tracing::warn!(error = %e, "failed to get peer cred; dropping connection");
                    drop(stream);
                    continue;
                }
            }
        }
    }
}

/// Bind a UDS at `path`, removing any stale file first, then chmod it
/// `0600` so even other users on the system can't `connect()` it.
pub fn listen(path: &Path) -> RpcResult<UnixListener> {
    if let Some(parent) = path.parent() {
        std::fs::create_dir_all(parent)?;
        // 0700 on the parent directory belt-and-suspenders the socket's
        // own 0600 permission — but only when we can actually set it.
        // A caller may legitimately hand us a path whose parent is not
        // ours to chmod (`/tmp`, a smoke test's scratch dir), and
        // `chmod 0700 /tmp` fails with EPERM for every non-root user.
        // Hard-failing there took the whole daemon down rather than the
        // hardening step.
        let mut perms = std::fs::metadata(parent)?.permissions();
        perms.set_mode(0o700);
        if let Err(e) = std::fs::set_permissions(parent, perms) {
            tracing::debug!(
                parent = %parent.display(), error = %e,
                "could not tighten the socket directory; relying on the socket's own 0600"
            );
        }
    }
    // Remove any stale socket left over from a previous run.
    match std::fs::remove_file(path) {
        Ok(_) => {}
        Err(e) if e.kind() == io::ErrorKind::NotFound => {}
        Err(e) => return Err(RpcError::Io(e)),
    }
    let listener = UnixListener::bind(path)?;
    let mut perms = std::fs::metadata(path)?.permissions();
    perms.set_mode(0o600);
    std::fs::set_permissions(path, perms)?;
    Ok(listener)
}

/// Connect to a UDS, refusing one this user does not own.
///
/// The counterpart of [`accept_authenticated`]'s peer-uid check. That
/// one stops a foreign process reaching *our* listener; this one stops
/// *us* reaching a foreign listener. Without it, any socket path an
/// attacker can pre-create is an impersonation of the daemon — and the
/// shell trusts whatever comes back over that connection enough to mint
/// `Provenance::QueryHit` from it, which is the gate on every
/// destructive file command.
pub async fn connect(path: &Path) -> RpcResult<UnixStream> {
    // `symlink_metadata`, not `metadata`: a symlink pointing at a
    // socket we *do* own would otherwise pass while the link itself is
    // attacker-controlled.
    let md = std::fs::symlink_metadata(path)?;
    let me = unsafe { libc::getuid() };
    if md.uid() != me {
        return Err(RpcError::Io(io::Error::new(
            io::ErrorKind::PermissionDenied,
            format!(
                "refusing to connect to {}: owned by uid {}, not {me}",
                path.display(),
                md.uid()
            ),
        )));
    }
    if md.mode() & 0o077 != 0 {
        return Err(RpcError::Io(io::Error::new(
            io::ErrorKind::PermissionDenied,
            format!(
                "refusing to connect to {}: mode {:o} is reachable by other users",
                path.display(),
                md.mode() & 0o777
            ),
        )));
    }
    let stream = UnixStream::connect(path).await?;
    Ok(stream)
}

/// Resolve a typed `SocketPath::Path` for the listener / connect helpers.
pub fn extract_path(p: &crate::path::SocketPath) -> PathBuf {
    match p {
        crate::path::SocketPath::Path(p) => p.clone(),
        crate::path::SocketPath::Pipe(_) => PathBuf::from("/tmp/freally-indexd.sock"),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::TempDir;

    #[tokio::test]
    async fn round_trip_accept_authenticated() {
        let tmp = TempDir::new().unwrap();
        let sock = tmp.path().join("test.sock");
        let listener = listen(&sock).unwrap();
        let listener = std::sync::Arc::new(listener);
        let listener_c = listener.clone();
        let server = tokio::spawn(async move {
            let mut stream = listener_c.accept_authenticated().await.unwrap();
            use tokio::io::AsyncWriteExt;
            stream.write_all(b"hi").await.unwrap();
        });
        let mut client = connect(&sock).await.unwrap();
        use tokio::io::AsyncReadExt;
        let mut buf = [0_u8; 2];
        client.read_exact(&mut buf).await.unwrap();
        assert_eq!(&buf, b"hi");
        server.await.unwrap();
    }

    #[tokio::test]
    async fn socket_file_is_0600() {
        let tmp = TempDir::new().unwrap();
        let sock = tmp.path().join("test.sock");
        let _listener = listen(&sock).unwrap();
        let mode = std::fs::metadata(&sock).unwrap().permissions().mode();
        assert_eq!(mode & 0o777, 0o600);
    }
}
