//! inotify / fanotify mask → semantic kind mapping.
//!
//! The bit layout below mirrors `<sys/inotify.h>` and `<sys/fanotify.h>`
//! and is duplicated here as portable `u32` constants so the classifier
//! compiles + runs on every dev OS — Phase 0's cross-OS workspace gate
//! requires that `cargo check` on a Windows host walks the whole tree
//! including this module.
//!
//! Both event sources coalesce: an inotify record's `mask` may carry
//! several `IN_*` bits at once, and a fanotify record may carry both
//! `FAN_MODIFY` and `FAN_CLOSE_WRITE` for the same file in the same
//! batch. We use `IN_CLOSE_WRITE` (or `FAN_CLOSE_WRITE`) as the
//! settled-state proxy, mirroring the Phase-1 `USN_REASON_CLOSE` rule
//! so write-write-write-close sequences emit exactly one `Modify`,
//! not three.

#![allow(non_upper_case_globals)]

// ---------------------------------------------------------------------
// Inotify constants (mirror <sys/inotify.h>; libc on Linux exposes the
// same values, we redeclare so the classifier compiles cross-OS).
// ---------------------------------------------------------------------
pub const IN_ACCESS: u32 = 0x0000_0001;
pub const IN_MODIFY: u32 = 0x0000_0002;
pub const IN_ATTRIB: u32 = 0x0000_0004;
pub const IN_CLOSE_WRITE: u32 = 0x0000_0008;
pub const IN_CLOSE_NOWRITE: u32 = 0x0000_0010;
pub const IN_OPEN: u32 = 0x0000_0020;
pub const IN_MOVED_FROM: u32 = 0x0000_0040;
pub const IN_MOVED_TO: u32 = 0x0000_0080;
pub const IN_CREATE: u32 = 0x0000_0100;
pub const IN_DELETE: u32 = 0x0000_0200;
pub const IN_DELETE_SELF: u32 = 0x0000_0400;
pub const IN_MOVE_SELF: u32 = 0x0000_0800;
pub const IN_UNMOUNT: u32 = 0x0000_2000;
pub const IN_Q_OVERFLOW: u32 = 0x0000_4000;
pub const IN_IGNORED: u32 = 0x0000_8000;
pub const IN_ONLYDIR: u32 = 0x0100_0000;
pub const IN_DONT_FOLLOW: u32 = 0x0200_0000;
pub const IN_EXCL_UNLINK: u32 = 0x0400_0000;
pub const IN_MASK_CREATE: u32 = 0x1000_0000;
pub const IN_MASK_ADD: u32 = 0x2000_0000;
pub const IN_ISDIR: u32 = 0x4000_0000;
pub const IN_ONESHOT: u32 = 0x8000_0000;

/// Mask Sourcerer subscribes with on every directory watch. Excludes
/// `IN_ACCESS`, `IN_OPEN`, `IN_CLOSE_NOWRITE` — read-only events flood
/// the queue with no value to the indexer (we care about content
/// changes, not query traffic). Includes `IN_EXCL_UNLINK` so a watch
/// on an already-unlinked-but-still-open file does not chatter.
pub const SOURCERER_INOTIFY_MASK: u32 = IN_MODIFY
    | IN_ATTRIB
    | IN_CLOSE_WRITE
    | IN_MOVED_FROM
    | IN_MOVED_TO
    | IN_CREATE
    | IN_DELETE
    | IN_DELETE_SELF
    | IN_MOVE_SELF
    | IN_EXCL_UNLINK;

// ---------------------------------------------------------------------
// Fanotify constants (mirror <sys/fanotify.h> / linux/fanotify.h).
// ---------------------------------------------------------------------
pub const FAN_ACCESS: u64 = 0x0000_0001;
pub const FAN_MODIFY: u64 = 0x0000_0002;
pub const FAN_ATTRIB: u64 = 0x0000_0004;
pub const FAN_CLOSE_WRITE: u64 = 0x0000_0008;
pub const FAN_CLOSE_NOWRITE: u64 = 0x0000_0010;
pub const FAN_OPEN: u64 = 0x0000_0020;
pub const FAN_MOVED_FROM: u64 = 0x0000_0040;
pub const FAN_MOVED_TO: u64 = 0x0000_0080;
pub const FAN_CREATE: u64 = 0x0000_0100;
pub const FAN_DELETE: u64 = 0x0000_0200;
pub const FAN_DELETE_SELF: u64 = 0x0000_0400;
pub const FAN_MOVE_SELF: u64 = 0x0000_0800;
pub const FAN_OPEN_EXEC: u64 = 0x0000_1000;
pub const FAN_Q_OVERFLOW: u64 = 0x0000_4000;
pub const FAN_FS_ERROR: u64 = 0x0000_8000;
pub const FAN_ONDIR: u64 = 0x4000_0000_0000_0000;

/// Mask Sourcerer subscribes with on the elevated fanotify path. Mirrors
/// the inotify mask above plus `FAN_FS_ERROR` (unique to fanotify; lets
/// the daemon log fs-internal errors that would otherwise be invisible).
pub const SOURCERER_FANOTIFY_MASK: u64 = FAN_MODIFY
    | FAN_ATTRIB
    | FAN_CLOSE_WRITE
    | FAN_MOVED_FROM
    | FAN_MOVED_TO
    | FAN_CREATE
    | FAN_DELETE
    | FAN_DELETE_SELF
    | FAN_MOVE_SELF
    | FAN_FS_ERROR;

/// Coarse classification used by the subscriber to route a single
/// inotify or fanotify record to one `JournalEvent` variant.
///
/// Precedence rationale (mirrors Phase 1's USN classifier):
/// - `QueueOverflow` short-circuits because the kernel signalled that
///   it dropped events — we cannot trust any other bit in the same
///   record. The subscriber rescans the affected subtree to recover.
/// - `Ignored` (inotify-only) tells us a watch went away (rm_watch
///   ack, or the watched dir was deleted). The subscriber prunes its
///   wd→path table; nothing user-facing is emitted.
/// - `Delete` outranks `Create`: a created-then-deleted-in-coalesce-
///   window record's net file state is "gone".
/// - `RenameOld` / `RenameNew` carry a non-zero cookie (inotify) or a
///   matching FID (fanotify) so the subscriber can pair the halves
///   into a `JournalEvent::Rename`. Unpaired halves degrade to
///   Delete (rename-out-of-watch) or Create (rename-in-from-outside).
/// - `Pending` is reserved for write events without `IN_CLOSE_WRITE`
///   — the subscriber waits for the close before emitting `Modify`,
///   so a write-write-write-close sequence emits exactly one event.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ReasonKind {
    /// Wait for more — record carries `IN_MODIFY` but no
    /// `IN_CLOSE_WRITE` yet. Drop on the floor.
    Pending,
    Create,
    Delete,
    /// Old-name half of a rename pair. Inotify carries a non-zero
    /// `cookie` field; fanotify carries a matching info-record FID.
    RenameOld,
    /// New-name half of a rename pair.
    RenameNew,
    Modify,
    AttrChange,
    /// Kernel queue overflow — events were dropped. Subscriber rescans.
    QueueOverflow,
    /// inotify-only: a watch descriptor went away. Subscriber prunes.
    Ignored,
    /// Bit pattern we choose not to forward (e.g. `IN_OPEN`,
    /// `IN_ACCESS` — read-only chatter we don't subscribe to but
    /// might see if a third party adds a watch). Subscriber drops.
    Other,
}

/// Classify a single inotify event mask. The subscriber must check
/// `cookie != 0` separately to disambiguate the `RenameOld` vs
/// `RenameNew` halves; this classifier only routes by mask bits.
pub fn classify_inotify(mask: u32) -> ReasonKind {
    if mask & IN_Q_OVERFLOW != 0 {
        return ReasonKind::QueueOverflow;
    }
    if mask & IN_IGNORED != 0 {
        return ReasonKind::Ignored;
    }
    if mask & IN_MOVED_FROM != 0 {
        return ReasonKind::RenameOld;
    }
    if mask & IN_MOVED_TO != 0 {
        return ReasonKind::RenameNew;
    }
    // Delete outranks Create: see ReasonKind doc.
    if mask & (IN_DELETE | IN_DELETE_SELF) != 0 {
        return ReasonKind::Delete;
    }
    if mask & IN_CREATE != 0 {
        return ReasonKind::Create;
    }
    if mask & IN_CLOSE_WRITE != 0 {
        return ReasonKind::Modify;
    }
    if mask & IN_ATTRIB != 0 {
        return ReasonKind::AttrChange;
    }
    if mask & IN_MODIFY != 0 {
        return ReasonKind::Pending;
    }
    ReasonKind::Other
}

/// Classify a single fanotify event mask. Like the inotify classifier,
/// rename pairing is resolved by the subscriber via the per-event FID;
/// this routine only routes by mask bits.
pub fn classify_fanotify(mask: u64) -> ReasonKind {
    if mask & FAN_Q_OVERFLOW != 0 {
        return ReasonKind::QueueOverflow;
    }
    if mask & FAN_MOVED_FROM != 0 {
        return ReasonKind::RenameOld;
    }
    if mask & FAN_MOVED_TO != 0 {
        return ReasonKind::RenameNew;
    }
    if mask & (FAN_DELETE | FAN_DELETE_SELF) != 0 {
        return ReasonKind::Delete;
    }
    if mask & FAN_CREATE != 0 {
        return ReasonKind::Create;
    }
    if mask & FAN_CLOSE_WRITE != 0 {
        return ReasonKind::Modify;
    }
    if mask & FAN_ATTRIB != 0 {
        return ReasonKind::AttrChange;
    }
    if mask & FAN_MODIFY != 0 {
        return ReasonKind::Pending;
    }
    ReasonKind::Other
}

pub fn is_dir_inotify(mask: u32) -> bool {
    mask & IN_ISDIR != 0
}

pub fn is_dir_fanotify(mask: u64) -> bool {
    mask & FAN_ONDIR != 0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn overflow_outranks_everything() {
        let m = IN_Q_OVERFLOW | IN_CREATE | IN_MOVED_FROM;
        assert_eq!(classify_inotify(m), ReasonKind::QueueOverflow);
        let m2 = FAN_Q_OVERFLOW | FAN_CREATE | FAN_MOVED_FROM;
        assert_eq!(classify_fanotify(m2), ReasonKind::QueueOverflow);
    }

    #[test]
    fn ignored_outranks_payload_inotify() {
        // A watch removal can carry `IN_IGNORED | IN_DELETE_SELF` — we
        // need to see this as Ignored so the subscriber prunes the wd
        // table without double-emitting a Delete (the parent watch
        // already emitted IN_DELETE for the same path).
        let m = IN_IGNORED | IN_DELETE_SELF;
        assert_eq!(classify_inotify(m), ReasonKind::Ignored);
    }

    #[test]
    fn rename_outranks_create_inotify() {
        // A file that's created then immediately renamed inside the
        // same coalesce window arrives with `IN_CREATE | IN_MOVED_FROM`
        // on the old half — the subscriber must see this as RenameOld
        // so the pairing logic with the matching IN_MOVED_TO closes
        // out a Rename event instead of Create + Delete.
        assert_eq!(
            classify_inotify(IN_CREATE | IN_MOVED_FROM),
            ReasonKind::RenameOld
        );
        assert_eq!(
            classify_inotify(IN_CREATE | IN_MOVED_TO),
            ReasonKind::RenameNew
        );
    }

    #[test]
    fn delete_outranks_create_inotify() {
        // Rare in practice but possible: a fast create-then-delete
        // produces a single coalesced record with both bits.
        assert_eq!(classify_inotify(IN_CREATE | IN_DELETE), ReasonKind::Delete);
    }

    #[test]
    fn close_write_settles_modify_inotify() {
        // A bare IN_MODIFY (no close yet) is Pending; once
        // IN_CLOSE_WRITE arrives the subscriber emits Modify exactly
        // once for the burst.
        assert_eq!(classify_inotify(IN_MODIFY), ReasonKind::Pending);
        assert_eq!(classify_inotify(IN_CLOSE_WRITE), ReasonKind::Modify);
        // Coalesced "wrote and closed in the same record" — the
        // CLOSE_WRITE bit takes precedence over the bare MODIFY bit
        // so the burst still emits exactly one Modify.
        assert_eq!(
            classify_inotify(IN_MODIFY | IN_CLOSE_WRITE),
            ReasonKind::Modify
        );
    }

    #[test]
    fn attrib_only_is_attr_change_inotify() {
        assert_eq!(classify_inotify(IN_ATTRIB), ReasonKind::AttrChange);
    }

    #[test]
    fn unknown_mask_is_other_inotify() {
        // IN_ACCESS / IN_OPEN are not in our subscribe mask but a third
        // party might add a watch for them on the same fd. Drop.
        assert_eq!(classify_inotify(IN_ACCESS), ReasonKind::Other);
        assert_eq!(classify_inotify(IN_OPEN), ReasonKind::Other);
        assert_eq!(classify_inotify(0), ReasonKind::Other);
    }

    #[test]
    fn rename_outranks_create_fanotify() {
        let m = FAN_CREATE | FAN_MOVED_FROM;
        assert_eq!(classify_fanotify(m), ReasonKind::RenameOld);
    }

    #[test]
    fn delete_outranks_create_fanotify() {
        let m = FAN_CREATE | FAN_DELETE;
        assert_eq!(classify_fanotify(m), ReasonKind::Delete);
    }

    #[test]
    fn close_write_settles_modify_fanotify() {
        assert_eq!(classify_fanotify(FAN_MODIFY), ReasonKind::Pending);
        assert_eq!(classify_fanotify(FAN_CLOSE_WRITE), ReasonKind::Modify);
    }

    #[test]
    fn dir_predicates() {
        assert!(is_dir_inotify(IN_ISDIR | IN_CREATE));
        assert!(!is_dir_inotify(IN_CREATE));
        assert!(is_dir_fanotify(FAN_ONDIR | FAN_CREATE));
        assert!(!is_dir_fanotify(FAN_CREATE));
    }

    #[test]
    fn sourcerer_mask_excludes_read_only_chatter() {
        // We must never subscribe to IN_ACCESS / IN_OPEN — those fire
        // on every read and would flood the queue. Locking in the
        // assertion here so a future "subscribe to everything" refactor
        // can't silently regress on it.
        assert_eq!(SOURCERER_INOTIFY_MASK & IN_ACCESS, 0);
        assert_eq!(SOURCERER_INOTIFY_MASK & IN_OPEN, 0);
        assert_eq!(SOURCERER_INOTIFY_MASK & IN_CLOSE_NOWRITE, 0);
        assert_eq!(SOURCERER_FANOTIFY_MASK & FAN_ACCESS, 0);
        assert_eq!(SOURCERER_FANOTIFY_MASK & FAN_OPEN, 0);
        assert_eq!(SOURCERER_FANOTIFY_MASK & FAN_CLOSE_NOWRITE, 0);
    }
}
