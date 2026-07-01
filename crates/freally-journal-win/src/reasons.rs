//! USN reason-flag → semantic kind mapping.
//!
//! USN records coalesce — a single record's `Reason` field is a bitmap of
//! every change that's happened to the file since its previous CLOSE. We
//! always wait for `USN_REASON_CLOSE` before emitting a `JournalEvent` so
//! every emitted event represents a settled state. Otherwise a single
//! create-write-write-close sequence emits four duplicate `Modify` events.

#[cfg(windows)]
use windows::Win32::System::Ioctl::{
    USN_REASON_BASIC_INFO_CHANGE, USN_REASON_CLOSE, USN_REASON_DATA_EXTEND,
    USN_REASON_DATA_OVERWRITE, USN_REASON_DATA_TRUNCATION, USN_REASON_EA_CHANGE,
    USN_REASON_FILE_CREATE, USN_REASON_FILE_DELETE, USN_REASON_NAMED_DATA_EXTEND,
    USN_REASON_NAMED_DATA_OVERWRITE, USN_REASON_NAMED_DATA_TRUNCATION, USN_REASON_RENAME_NEW_NAME,
    USN_REASON_RENAME_OLD_NAME, USN_REASON_REPARSE_POINT_CHANGE, USN_REASON_SECURITY_CHANGE,
    USN_REASON_STREAM_CHANGE,
};

// Make the constants available as plain `u32`s on every platform so unit
// tests for the classifier compile and run on macOS / Linux dev machines too.
#[cfg(not(windows))]
mod reasons_portable {
    pub const USN_REASON_BASIC_INFO_CHANGE: u32 = 0x0000_8000;
    pub const USN_REASON_CLOSE: u32 = 0x8000_0000;
    pub const USN_REASON_DATA_EXTEND: u32 = 0x0000_0002;
    pub const USN_REASON_DATA_OVERWRITE: u32 = 0x0000_0001;
    pub const USN_REASON_DATA_TRUNCATION: u32 = 0x0000_0004;
    pub const USN_REASON_EA_CHANGE: u32 = 0x0000_0400;
    pub const USN_REASON_FILE_CREATE: u32 = 0x0000_0100;
    pub const USN_REASON_FILE_DELETE: u32 = 0x0000_0200;
    pub const USN_REASON_NAMED_DATA_EXTEND: u32 = 0x0000_0020;
    pub const USN_REASON_NAMED_DATA_OVERWRITE: u32 = 0x0000_0010;
    pub const USN_REASON_NAMED_DATA_TRUNCATION: u32 = 0x0000_0040;
    pub const USN_REASON_RENAME_NEW_NAME: u32 = 0x0000_2000;
    pub const USN_REASON_RENAME_OLD_NAME: u32 = 0x0000_1000;
    pub const USN_REASON_REPARSE_POINT_CHANGE: u32 = 0x0010_0000;
    pub const USN_REASON_SECURITY_CHANGE: u32 = 0x0000_0800;
    pub const USN_REASON_STREAM_CHANGE: u32 = 0x0020_0000;
}
#[cfg(not(windows))]
use reasons_portable::*;

const DATA_CHANGE_MASK: u32 = USN_REASON_DATA_EXTEND
    | USN_REASON_DATA_OVERWRITE
    | USN_REASON_DATA_TRUNCATION
    | USN_REASON_NAMED_DATA_EXTEND
    | USN_REASON_NAMED_DATA_OVERWRITE
    | USN_REASON_NAMED_DATA_TRUNCATION
    | USN_REASON_STREAM_CHANGE
    | USN_REASON_REPARSE_POINT_CHANGE
    | USN_REASON_EA_CHANGE;

const ATTR_CHANGE_MASK: u32 = USN_REASON_BASIC_INFO_CHANGE | USN_REASON_SECURITY_CHANGE;

/// Coarse classification used by the subscriber to route a settled (CLOSE)
/// USN record to a single `JournalEvent` variant.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ReasonKind {
    /// Wait for more — record has changes but no `USN_REASON_CLOSE` yet.
    Pending,
    Create,
    Delete,
    /// Old-name half of a rename — caller must pair with a `RenameNew`
    /// for the same FRN before emitting a `JournalEvent::Rename`.
    RenameOld,
    /// New-name half of a rename — pair with the matching `RenameOld`.
    RenameNew,
    Modify,
    AttrChange,
    /// Unknown reason mask we choose not to forward (e.g. transient
    /// transactional bits without CLOSE). The subscriber drops these.
    Ignore,
}

/// Classify a USN record's `Reason` field. The classifier respects this
/// Two-tier precedence:
/// - **Inherently terminal** events (file is gone — no further
///   accumulation possible): `FILE_DELETE` and `RENAME_OLD_NAME`.
///   These emit immediately; the `CLOSE` bit is NOT required. NTFS
///   does not emit a closing record for the OLD name half of a rename
///   (the old path is terminal — nothing more will happen at that
///   path), and `FILE_DELETE` records for POSIX-style deletes can
///   arrive without a paired CLOSE record.
/// - **Settled-state** events (we want the LAST record per session):
///   `FILE_CREATE`, `RENAME_NEW_NAME`, `DATA_*`, `ATTR_*`. Gated on
///   `USN_REASON_CLOSE` so a write-write-write-close sequence emits
///   exactly one `Modify`, not three.
///
/// Within each tier, precedence is:
///
/// - Terminal tier: `FILE_DELETE > RENAME_OLD_NAME`. A
///   created-then-renamed-then-deleted-in-one-session record has
///   `FILE_DELETE | RENAME_OLD_NAME` set; net result is gone.
/// - Settled tier: `RENAME_NEW_NAME > FILE_CREATE > DATA_* > ATTR_*`.
///   NTFS accumulates reasons within a session, so a record for a
///   file that was created-then-renamed ends up with
///   `FILE_CREATE | RENAME_NEW_NAME | CLOSE` for the new-name half.
///   The user-visible truth is the rename; pairing logic needs to
///   see this as `RenameNew` so it can match the corresponding
///   `RenameOld` from the terminal tier.
pub fn classify(reason: u32) -> ReasonKind {
    // --- Terminal tier (no CLOSE required) ---
    if reason & USN_REASON_FILE_DELETE != 0 {
        return ReasonKind::Delete;
    }
    // RENAME_OLD without RENAME_NEW: the old-name session is closed
    // implicitly. With both bits set we defer to RENAME_NEW handling
    // below (the new-name session carries the close).
    if reason & USN_REASON_RENAME_OLD_NAME != 0 && reason & USN_REASON_RENAME_NEW_NAME == 0 {
        return ReasonKind::RenameOld;
    }

    // --- Settled tier (CLOSE required) ---
    if reason & USN_REASON_CLOSE == 0 {
        return ReasonKind::Pending;
    }

    if reason & USN_REASON_RENAME_NEW_NAME != 0 {
        return ReasonKind::RenameNew;
    }
    if reason & USN_REASON_FILE_CREATE != 0 {
        return ReasonKind::Create;
    }
    if reason & DATA_CHANGE_MASK != 0 {
        return ReasonKind::Modify;
    }
    if reason & ATTR_CHANGE_MASK != 0 {
        return ReasonKind::AttrChange;
    }

    ReasonKind::Ignore
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn no_close_is_pending() {
        assert_eq!(classify(USN_REASON_FILE_CREATE), ReasonKind::Pending);
        assert_eq!(classify(USN_REASON_DATA_EXTEND), ReasonKind::Pending);
    }

    #[test]
    fn create_then_close_is_create() {
        let r = USN_REASON_FILE_CREATE | USN_REASON_DATA_EXTEND | USN_REASON_CLOSE;
        assert_eq!(classify(r), ReasonKind::Create);
    }

    #[test]
    fn delete_outranks_create_in_a_close_record() {
        // Rare in practice but possible if a transactional sequence both
        // creates and deletes between two CLOSE points.
        let r = USN_REASON_FILE_CREATE | USN_REASON_FILE_DELETE | USN_REASON_CLOSE;
        assert_eq!(classify(r), ReasonKind::Delete);
    }

    #[test]
    fn rename_pair_classification() {
        let old = USN_REASON_RENAME_OLD_NAME | USN_REASON_CLOSE;
        let new = USN_REASON_RENAME_NEW_NAME | USN_REASON_CLOSE;
        assert_eq!(classify(old), ReasonKind::RenameOld);
        assert_eq!(classify(new), ReasonKind::RenameNew);
    }

    #[test]
    fn rename_outranks_create_in_a_close_record() {
        // NTFS accumulates reasons within a session: a file created then
        // immediately renamed (within the time the journal hadn't yet
        // emitted a closing record) ends up with
        // `FILE_CREATE | RENAME_OLD_NAME | CLOSE` for the old-half.
        // The user-visible truth is the rename — pairing must see this
        // as RenameOld so the matching RenameNew on the new-half closes
        // out a Rename event.
        let old = USN_REASON_FILE_CREATE | USN_REASON_RENAME_OLD_NAME | USN_REASON_CLOSE;
        assert_eq!(classify(old), ReasonKind::RenameOld);
        let new = USN_REASON_FILE_CREATE | USN_REASON_RENAME_NEW_NAME | USN_REASON_CLOSE;
        assert_eq!(classify(new), ReasonKind::RenameNew);
    }

    #[test]
    fn data_close_is_modify() {
        let r = USN_REASON_DATA_OVERWRITE | USN_REASON_CLOSE;
        assert_eq!(classify(r), ReasonKind::Modify);
    }

    #[test]
    fn attribute_close_is_attr_change() {
        let r = USN_REASON_BASIC_INFO_CHANGE | USN_REASON_CLOSE;
        assert_eq!(classify(r), ReasonKind::AttrChange);
    }

    #[test]
    fn close_only_is_ignored() {
        // Just a close-without-changes — nothing to surface.
        assert_eq!(classify(USN_REASON_CLOSE), ReasonKind::Ignore);
    }

    #[test]
    fn rename_old_without_close_is_terminal() {
        // The OLD-name half of a rename never sees a CLOSE record —
        // there's nothing more to wait for at that path. Phase 1's
        // integration test confirmed via raw USN dump on a real volume:
        // a `rename(b, b2)` produces three records:
        //   usn N    : 0x00001000 (RENAME_OLD_NAME, no CLOSE)  ← old half
        //   usn N+1  : 0x00002000 (RENAME_NEW_NAME, no CLOSE)
        //   usn N+2  : 0x80002000 (RENAME_NEW_NAME | CLOSE)    ← new half
        // Without this terminal-tier handling, the old half was dropped
        // as Pending, the pairing table was empty when the new half's
        // CLOSE record arrived, and no `JournalEvent::Rename` was ever
        // emitted.
        assert_eq!(classify(USN_REASON_RENAME_OLD_NAME), ReasonKind::RenameOld);
    }

    #[test]
    fn delete_without_close_is_terminal() {
        // POSIX-style deletes on Windows can produce FILE_DELETE records
        // without a paired CLOSE record (the close has already been
        // accounted for by the rename-to-temp). Treating the delete as
        // terminal regardless of CLOSE is consumer-correct.
        assert_eq!(classify(USN_REASON_FILE_DELETE), ReasonKind::Delete);
    }

    #[test]
    fn rename_new_without_close_is_pending() {
        // The NEW-name half of a rename DOES get a CLOSE record (the
        // file is still open and may accept further writes before close).
        // We must wait for CLOSE on the new half so a write-then-rename
        // doesn't double-emit RenameNew.
        assert_eq!(classify(USN_REASON_RENAME_NEW_NAME), ReasonKind::Pending);
    }
}
