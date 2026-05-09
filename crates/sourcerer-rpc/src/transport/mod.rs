//! Per-OS local transport primitives.

#[cfg(unix)]
pub mod unix;

#[cfg(windows)]
pub mod windows;

#[cfg(all(not(unix), not(windows)))]
compile_error!("sourcerer-rpc requires Unix or Windows");
