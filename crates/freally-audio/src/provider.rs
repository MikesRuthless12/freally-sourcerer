//! Provider abstraction the query executor talks to. Lets us pass a
//! cache-backed implementation in production and a stub in tests.

use std::path::Path;

use crate::attributes::AudioAttributes;
use crate::error::AudioError;

/// Look up audio attributes for a file by path. The `mtime_ns` is the
/// expected modification time at the moment the lookup is requested —
/// the cache layer compares it to the cached value to decide whether
/// to serve from cache or re-extract.
pub trait AudioAttributesProvider: Send + Sync {
    /// Return cached or freshly-computed attributes. `Ok(None)` is
    /// reserved for the "not an audio file" / "extractor disabled"
    /// case so the executor can short-circuit cleanly.
    fn get(&self, path: &Path, mtime_ns: i64) -> Result<Option<AudioAttributes>, AudioError>;
}

/// No-op provider. Returns `Ok(None)` for every path. Wired for the
/// "user disabled the audio lens" path so the executor still produces
/// (empty) results instead of failing.
#[derive(Debug, Default, Clone, Copy)]
pub struct NullProvider;

impl AudioAttributesProvider for NullProvider {
    fn get(&self, _path: &Path, _mtime_ns: i64) -> Result<Option<AudioAttributes>, AudioError> {
        Ok(None)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::PathBuf;

    #[test]
    fn null_provider_returns_none() {
        let p = NullProvider;
        let path = PathBuf::from("/tmp/whatever.flac");
        assert!(p.get(&path, 0).unwrap().is_none());
    }
}
