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
//! Build 1 adds the missing dimension. Two different facts used to share
//! one set: "the daemon returned this as a search hit" and "the user
//! picked this in an OS save dialog". That was fine while every gated
//! command only *read*, but `file_list_export` writes — and with one
//! undifferentiated set it would happily overwrite any file in the
//! current result set. [`Provenance`] keeps the two apart so a
//! write-side command can demand the narrower one.

use std::collections::HashMap;
use std::path::PathBuf;
use std::sync::Mutex;

const MAX_KNOWN_PATHS: usize = 16_384;

/// How a path earned its place in the registry.
///
/// Ordered weakest-first: a grant satisfies any requirement at or below
/// its own level.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum Provenance {
    /// The **frontend** asserted this path, via
    /// `files_whitelist_user_chosen`. That command takes an arbitrary
    /// string from the webview, so this level attests to nothing beyond
    /// "some JS asked for it" — it is what the untrusted layer can mint
    /// for itself.
    ///
    /// Enough to read, preview, reveal, or open, which is what the two
    /// real callers need (the preview pane, and Go To after a JS-opened
    /// folder dialog). Deliberately **not** enough to delete or rename:
    /// those are irreversible, and a compromised frontend dependency
    /// must not be able to authorize them by naming a path.
    FrontendAsserted,
    /// The daemon returned it as a search hit, registered by the Rust
    /// notification re-emitter as `query:batch` passes through. Nothing
    /// reachable from JS can mint this, which is what makes it a real
    /// attestation and the right gate for destructive commands.
    QueryHit,
    /// The user picked it in an OS-native dialog **opened in Rust**.
    /// That dialog is the strongest trust boundary available, so this is
    /// what a command that writes new content to a caller-named path
    /// requires.
    UserChosen,
}

#[derive(Default)]
pub struct KnownPaths {
    paths: Mutex<HashMap<String, Provenance>>,
}

impl KnownPaths {
    pub fn new() -> Self {
        Self {
            paths: Mutex::new(HashMap::new()),
        }
    }

    /// Register a path the **daemon** returned as a search hit. Only the
    /// notification re-emitter calls this; it is not reachable from JS.
    pub fn add(&self, path: &str) {
        self.record(path, Provenance::QueryHit);
    }

    pub fn add_many<I: IntoIterator<Item = String>>(&self, paths: I) {
        for p in paths {
            self.add(&p);
        }
    }

    /// Register a path the **frontend** named. Grants the weakest level,
    /// which cannot satisfy a destructive command's gate.
    pub fn add_frontend_asserted(&self, path: &str) {
        self.record(path, Provenance::FrontendAsserted);
    }

    /// User selected via OS-native dialog — the dialog itself is the
    /// trust boundary, so this grants the wider `UserChosen` provenance.
    pub fn whitelist_user_chosen(&self, path: &str) {
        self.record(path, Provenance::UserChosen);
    }

    fn record(&self, path: &str, provenance: Provenance) {
        let mut g = self.paths.lock().unwrap();
        if g.len() >= MAX_KNOWN_PATHS {
            // LRU is overkill for a defense-in-depth registry; just clear
            // the oldest half once the cap is hit.
            let to_drop: Vec<String> = g.keys().take(g.len() / 2).cloned().collect();
            for p in to_drop {
                g.remove(&p);
            }
        }
        // A path can be both a search hit and a user pick; keep the
        // wider grant so re-selecting a result in a save dialog doesn't
        // demote it.
        g.entry(path.to_string())
            .and_modify(|p| *p = (*p).max(provenance))
            .or_insert(provenance);
    }

    /// Drop a grant. Used when a path stops naming the file it was
    /// granted for — after a rename, the old name is just a free slot,
    /// and a later unrelated file created there must not inherit a
    /// permission the daemon never gave it.
    pub fn revoke(&self, path: &str) {
        self.paths.lock().unwrap().remove(path);
    }

    pub fn contains(&self, path: &str) -> bool {
        self.paths.lock().unwrap().contains_key(path)
    }

    /// The single gate every path-taking command goes through. Returns
    /// the path only when the registry holds it with at least `required`
    /// provenance.
    ///
    /// The error deliberately does NOT echo the supplied path —
    /// caller-controlled bytes don't belong in responses that may flow
    /// into logs or UIs.
    pub fn verify(&self, path: &str, required: Provenance) -> Result<PathBuf, String> {
        let held = self.paths.lock().unwrap().get(path).copied();
        match held {
            Some(p) if p >= required => Ok(PathBuf::from(path)),
            Some(_) => Err("path was not chosen through a file dialog".into()),
            None => Err("path not in current result set".into()),
        }
    }

    /// [`KnownPaths::verify`] over a list, failing on the first path
    /// that doesn't qualify.
    pub fn verify_all(
        &self,
        paths: &[String],
        required: Provenance,
    ) -> Result<Vec<PathBuf>, String> {
        paths.iter().map(|p| self.verify(p, required)).collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a_query_hit_can_be_read_but_not_written() {
        let k = KnownPaths::new();
        k.add("/vault/report.pdf");
        assert!(k.verify("/vault/report.pdf", Provenance::QueryHit).is_ok());
        let err = k
            .verify("/vault/report.pdf", Provenance::UserChosen)
            .unwrap_err();
        assert!(err.contains("file dialog"), "message was {err}");
    }

    #[test]
    fn a_user_chosen_path_satisfies_both() {
        let k = KnownPaths::new();
        k.whitelist_user_chosen("/vault/out.efu");
        assert!(k.verify("/vault/out.efu", Provenance::QueryHit).is_ok());
        assert!(k.verify("/vault/out.efu", Provenance::UserChosen).is_ok());
    }

    #[test]
    fn an_unknown_path_is_rejected_without_echoing_it() {
        let k = KnownPaths::new();
        let err = k.verify("/etc/shadow", Provenance::QueryHit).unwrap_err();
        assert!(
            !err.contains("/etc/shadow"),
            "message leaked the path: {err}"
        );
    }

    #[test]
    fn picking_a_result_in_a_save_dialog_widens_it_and_never_narrows() {
        let k = KnownPaths::new();
        k.add("/vault/a.txt");
        k.whitelist_user_chosen("/vault/a.txt");
        assert!(k.verify("/vault/a.txt", Provenance::UserChosen).is_ok());
        // A later query hit for the same path must not demote it.
        k.add("/vault/a.txt");
        assert!(k.verify("/vault/a.txt", Provenance::UserChosen).is_ok());
    }

    #[test]
    fn what_the_frontend_asserts_can_be_read_but_never_deleted_or_renamed() {
        // The whole point of the third tier. `files_whitelist_user_chosen`
        // takes an arbitrary string from the webview, so a compromised
        // frontend dependency could otherwise name any path on disk and
        // then have it renamed or trashed with no user interaction.
        let k = KnownPaths::new();
        k.add_frontend_asserted("/etc/shadow");

        // Reads still work — the preview pane and Go To depend on it.
        assert!(
            k.verify("/etc/shadow", Provenance::FrontendAsserted).is_ok(),
            "preview / reveal must still accept a path the user just picked"
        );
        // Destructive commands do not.
        assert!(k.verify("/etc/shadow", Provenance::QueryHit).is_err());
        assert!(k.verify("/etc/shadow", Provenance::UserChosen).is_err());
    }

    #[test]
    fn a_daemon_returned_hit_satisfies_the_destructive_gate() {
        let k = KnownPaths::new();
        k.add("/vault/report.pdf");
        assert!(k.verify("/vault/report.pdf", Provenance::QueryHit).is_ok());
        // ...but still not the dialog-only level.
        assert!(k.verify("/vault/report.pdf", Provenance::UserChosen).is_err());
    }

    #[test]
    fn a_frontend_assertion_cannot_downgrade_a_daemon_attested_path() {
        let k = KnownPaths::new();
        k.add("/vault/report.pdf");
        k.add_frontend_asserted("/vault/report.pdf");
        assert!(
            k.verify("/vault/report.pdf", Provenance::QueryHit).is_ok(),
            "record() keeps the wider grant"
        );
    }

    #[test]
    fn a_revoked_path_stops_qualifying() {
        let k = KnownPaths::new();
        k.add("/vault/a.txt");
        assert!(k.verify("/vault/a.txt", Provenance::QueryHit).is_ok());
        // After a rename, /vault/a.txt names nothing. A file later
        // created there was never in any result set.
        k.revoke("/vault/a.txt");
        assert!(k.verify("/vault/a.txt", Provenance::QueryHit).is_err());
    }

    #[test]
    fn verify_all_fails_on_the_first_unqualified_path() {
        let k = KnownPaths::new();
        k.add("/vault/a.txt");
        let paths = vec!["/vault/a.txt".to_string(), "/vault/b.txt".to_string()];
        assert!(k.verify_all(&paths, Provenance::QueryHit).is_err());
        k.add("/vault/b.txt");
        assert_eq!(k.verify_all(&paths, Provenance::QueryHit).unwrap().len(), 2);
    }
}
