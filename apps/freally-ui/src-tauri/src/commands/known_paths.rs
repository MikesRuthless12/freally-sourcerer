//! Known-path registry — Phase 11 defense-in-depth for file-ops commands.
//!
//! Every `query_run` populates this set with the hit paths it returns;
//! every file-ops command (`files_open` / `files_reveal` / `files_delete`
//! / `files_thumbnail` / `files_preview` / `files_copy_path` /
//! `files_copy_name`) verifies the path is in the set before acting on
//! it. A compromised JS layer can no longer ask the Rust backend to act
//! on arbitrary filesystem paths — only on paths the daemon actually
//! returned in this session.
//!
//! Phase 12 keeps the same shape: `query_run` swaps from canned data to
//! the real daemon, but the per-session known-path registry stays.
//!
//! User-initiated paths (e.g. `view.go_to`, `file.export_results` save
//! dialog) flow through `whitelist_user_chosen` so they bypass the
//! per-hit registry — the user explicitly selected them via the OS
//! native dialog, which is the actual trust boundary.

use std::collections::HashSet;
use std::sync::Mutex;

const MAX_KNOWN_PATHS: usize = 16_384;

#[derive(Default)]
pub struct KnownPaths {
    pub set: Mutex<HashSet<String>>,
}

impl KnownPaths {
    pub fn new() -> Self {
        Self {
            set: Mutex::new(HashSet::new()),
        }
    }

    pub fn add(&self, path: &str) {
        let mut g = self.set.lock().unwrap();
        if g.len() >= MAX_KNOWN_PATHS {
            // LRU is overkill for a defense-in-depth registry; just clear
            // the oldest half once the cap is hit.
            let to_drop: Vec<String> = g.iter().take(g.len() / 2).cloned().collect();
            for p in to_drop {
                g.remove(&p);
            }
        }
        g.insert(path.to_string());
    }

    pub fn add_many<I: IntoIterator<Item = String>>(&self, paths: I) {
        for p in paths {
            self.add(&p);
        }
    }

    pub fn contains(&self, path: &str) -> bool {
        self.set.lock().unwrap().contains(path)
    }

    pub fn whitelist_user_chosen(&self, path: &str) {
        // User selected via OS-native dialog — trust boundary is the
        // dialog itself; record so subsequent ops on the same path pass.
        self.add(path);
    }
}
