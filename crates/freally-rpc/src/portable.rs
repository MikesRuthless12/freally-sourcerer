//! Portable mode (SRC-M17) — every byte the app writes lives in a
//! `Data/` folder beside the executable, and nothing is registered with
//! the OS.
//!
//! Turned on either by passing `--portable` (any of the three binaries)
//! or by dropping an empty `portable.flag` file beside the executable.
//! The flag file is what makes a downloaded zip / AppImage portable
//! without the user having to remember a switch, and it is what a
//! USB-stick install relies on: double-clicking the binary is enough.
//!
//! This lives in `freally-rpc` because it is the only crate all three
//! binaries — the Tauri shell, `freally-indexd`, and the `freally` CLI —
//! already depend on, and it already owns the sibling problem of
//! resolving the socket path.
//!
//! ## What does *not* move into `Data/`
//!
//! The RPC socket. A portable install is typically on a USB stick, and
//! a stick is typically FAT32 or exFAT — filesystems that cannot hold a
//! Unix domain socket at all, and that have no permission bits for the
//! 0600-plus-peer-uid check the transport relies on. So the socket
//! stays on the host, in the temp directory, under a name derived from
//! the portable root: that keeps a portable instance from colliding
//! with an installed one, or with a second stick, while keeping the
//! auth story unchanged.

use std::path::{Path, PathBuf};
use std::sync::OnceLock;
use std::sync::atomic::{AtomicBool, Ordering};

use crate::path::SocketPath;

/// Marker file that turns portable mode on with no command line.
pub const FLAG_FILE: &str = "portable.flag";

/// The one folder a portable install writes to.
pub const DATA_DIR: &str = "Data";

static FORCED: AtomicBool = AtomicBool::new(false);
static INSTALL_DIR: OnceLock<Option<PathBuf>> = OnceLock::new();
static FLAG_PRESENT: OnceLock<bool> = OnceLock::new();

/// Turn portable mode on explicitly — the `--portable` switch. Safe to
/// call more than once, and safe to call before anything has read
/// [`is_active`].
///
/// Call this during argument parsing, before any path is resolved.
/// Nothing in this module caches the *answer* to `is_active`, only the
/// two inputs to it, so a late call still takes effect — but a path
/// already handed out will not move.
pub fn activate() {
    FORCED.store(true, Ordering::Release);
}

/// Is portable mode on for this process?
pub fn is_active() -> bool {
    if FORCED.load(Ordering::Acquire) {
        return true;
    }
    *FLAG_PRESENT.get_or_init(|| {
        install_dir()
            .map(|d| d.join(FLAG_FILE).is_file())
            .unwrap_or(false)
    })
}

/// The directory a portable install treats as "beside the executable".
///
/// Normally the executable's own directory. On macOS an app bundle puts
/// the binary three levels down in `Freally.app/Contents/MacOS/`, and
/// writing user data *inside* the bundle would break code signing and
/// fail outright on a read-only mount — so the bundle's own parent is
/// used instead, which is also where a user dragging `Freally.app` onto
/// a stick would expect `Data/` to appear.
pub fn install_dir() -> Option<PathBuf> {
    INSTALL_DIR
        .get_or_init(|| std::env::current_exe().ok().and_then(|exe| base_for(&exe)))
        .clone()
}

/// Pure form of [`install_dir`], split out so it is testable without a
/// real executable on disk.
fn base_for(exe: &Path) -> Option<PathBuf> {
    let dir = exe.parent()?;
    if let Some(outside) = bundle_parent(dir) {
        return Some(outside);
    }
    Some(dir.to_path_buf())
}

/// For `…/Freally.app/Contents/MacOS`, the directory holding
/// `Freally.app`. `None` for any other shape.
fn bundle_parent(dir: &Path) -> Option<PathBuf> {
    if dir.file_name()? != "MacOS" {
        return None;
    }
    let contents = dir.parent()?;
    if contents.file_name()? != "Contents" {
        return None;
    }
    let bundle = contents.parent()?;
    if !bundle.file_name()?.to_str()?.ends_with(".app") {
        return None;
    }
    bundle.parent().map(Path::to_path_buf)
}

/// `Data/` for this install — `None` when portable mode is off, or when
/// the executable's own path could not be read.
pub fn data_dir() -> Option<PathBuf> {
    if !is_active() {
        return None;
    }
    Some(install_dir()?.join(DATA_DIR))
}

/// `Data/index` — what `--index-root` is set to in portable mode.
pub fn index_root() -> Option<PathBuf> {
    Some(data_dir()?.join("index"))
}

/// `Data/logs` — where a portable install writes log files, since a
/// double-clicked binary has no console to write to.
pub fn log_dir() -> Option<PathBuf> {
    Some(data_dir()?.join("logs"))
}

/// Socket / pipe for this portable install, kept off the stick and
/// namespaced by the portable root so two instances never collide.
///
/// See the module note: the socket deliberately does not live in
/// `Data/`.
///
/// It also deliberately does not live *directly* in the temp directory.
/// `std::env::temp_dir()` is `/tmp` on Linux — mode 1777, writable by
/// every local account — and the tag is a plain hash of a guessable
/// path, so any local user could pre-bind the exact name and have the
/// shell connect to them instead of to the real daemon. The transport
/// authenticates the *peer* of a listener it owns; a client connecting
/// out checks nothing, so squatting the path is impersonation, and a
/// forged `query:batch` mints `Provenance::QueryHit` for paths of the
/// attacker's choosing.
///
/// So: an owner-only directory, and the socket inside it.
/// `$XDG_RUNTIME_DIR` is already per-user and 0700 where it exists;
/// otherwise a uid-tagged subdirectory that this process creates itself.
#[cfg(not(windows))]
pub fn socket_path() -> Option<SocketPath> {
    let tag = tag_for(&data_dir()?);
    let dir = private_runtime_dir(&tag)?;
    Some(SocketPath::Path(dir.join("indexd.sock")))
}

#[cfg(windows)]
pub fn socket_path() -> Option<SocketPath> {
    // Named pipes are not filesystem objects and the pipe's DACL is set
    // by the server, so the squatting problem above does not apply.
    let tag = tag_for(&data_dir()?);
    let base = crate::path::default_pipe_name();
    Some(SocketPath::Pipe(format!("{base}-{tag}")))
}

/// A directory only this user can write to, created if absent.
#[cfg(not(windows))]
fn private_runtime_dir(tag: &str) -> Option<PathBuf> {
    let base = match std::env::var_os("XDG_RUNTIME_DIR") {
        Some(v) if !v.is_empty() => PathBuf::from(v),
        // No XDG_RUNTIME_DIR (macOS, or a bare login shell): fall back
        // to the temp dir but never to a *shared* name inside it — the
        // uid tag means another account's directory is a different
        // directory, and the 0700 mode below means they cannot enter
        // ours.
        _ => std::env::temp_dir(),
    };
    let dir = base.join(format!("freally-{}-{tag}", current_uid()));
    std::fs::create_dir_all(&dir).ok()?;
    // Set the mode after creating rather than trusting the umask.
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        std::fs::set_permissions(&dir, std::fs::Permissions::from_mode(0o700)).ok()?;
        // Refuse a directory we do not own: under a sticky /tmp another
        // account can create the name first, and `create_dir_all`
        // succeeds against an existing directory whoever owns it.
        use std::os::unix::fs::MetadataExt;
        let md = std::fs::metadata(&dir).ok()?;
        if md.uid() != current_uid() {
            return None;
        }
    }
    Some(dir)
}

#[cfg(all(unix, not(windows)))]
fn current_uid() -> u32 {
    // SAFETY: `getuid` is always safe — it takes no arguments, reads
    // process credentials, and cannot fail.
    unsafe { libc::getuid() }
}

#[cfg(all(not(unix), not(windows)))]
fn current_uid() -> u32 {
    0
}

/// Short, stable tag for a portable root.
///
/// FNV-1a rather than `DefaultHasher`: the CLI derives this
/// independently of the UI, and `DefaultHasher`'s output is explicitly
/// not guaranteed stable across Rust releases. FNV is eight lines and
/// stable forever.
fn tag_for(root: &Path) -> String {
    const OFFSET: u64 = 0xcbf2_9ce4_8422_2325;
    const PRIME: u64 = 0x0000_0100_0000_01b3;
    let mut h = OFFSET;
    for b in root.to_string_lossy().as_bytes() {
        h ^= u64::from(*b);
        h = h.wrapping_mul(PRIME);
    }
    format!("{h:016x}")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn plain_executable_uses_its_own_directory() {
        let base = base_for(Path::new("/opt/freally/freally-ui")).unwrap();
        assert_eq!(base, Path::new("/opt/freally"));
    }

    #[test]
    fn mac_bundle_writes_beside_the_bundle_not_inside_it() {
        let exe = Path::new("/Volumes/STICK/Freally.app/Contents/MacOS/freally-ui");
        let base = base_for(exe).unwrap();
        // Not `.../Freally.app/Contents/MacOS` — user data inside a
        // bundle breaks signing and fails on a read-only mount.
        assert_eq!(base, Path::new("/Volumes/STICK"));
    }

    #[test]
    fn a_directory_merely_named_macos_is_not_a_bundle() {
        let base = base_for(Path::new("/srv/MacOS/freally-ui")).unwrap();
        assert_eq!(base, Path::new("/srv/MacOS"));
    }

    #[test]
    fn tag_is_stable_and_root_specific() {
        let a = tag_for(Path::new("/Volumes/STICK/Data"));
        let b = tag_for(Path::new("/Volumes/OTHER/Data"));
        assert_eq!(a, tag_for(Path::new("/Volumes/STICK/Data")));
        assert_ne!(a, b);
        assert_eq!(a.len(), 16);
    }
}
