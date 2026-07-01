//! FSEvents flag bitmask constants + classification.
//!
//! Apple's `<CoreServices/FSEvents.h>` defines the bit values; we mirror
//! them here as `u32` constants so the classifier compiles + runs on every
//! dev OS, not just macOS. The values are stable Apple ABI dating back to
//! 10.7 (`ItemCreated` / `ItemRemoved` / `ItemRenamed` / `ItemModified` etc.
//! are documented in `kFSEventStreamEventFlag*`).

#![allow(non_upper_case_globals)]

pub const kFSEventStreamEventFlagNone: u32 = 0x0000_0000;
pub const kFSEventStreamEventFlagMustScanSubDirs: u32 = 0x0000_0001;
pub const kFSEventStreamEventFlagUserDropped: u32 = 0x0000_0002;
pub const kFSEventStreamEventFlagKernelDropped: u32 = 0x0000_0004;
pub const kFSEventStreamEventFlagEventIdsWrapped: u32 = 0x0000_0008;
pub const kFSEventStreamEventFlagHistoryDone: u32 = 0x0000_0010;
pub const kFSEventStreamEventFlagRootChanged: u32 = 0x0000_0020;
pub const kFSEventStreamEventFlagMount: u32 = 0x0000_0040;
pub const kFSEventStreamEventFlagUnmount: u32 = 0x0000_0080;
pub const kFSEventStreamEventFlagItemCreated: u32 = 0x0000_0100;
pub const kFSEventStreamEventFlagItemRemoved: u32 = 0x0000_0200;
pub const kFSEventStreamEventFlagItemInodeMetaMod: u32 = 0x0000_0400;
pub const kFSEventStreamEventFlagItemRenamed: u32 = 0x0000_0800;
pub const kFSEventStreamEventFlagItemModified: u32 = 0x0000_1000;
pub const kFSEventStreamEventFlagItemFinderInfoMod: u32 = 0x0000_2000;
pub const kFSEventStreamEventFlagItemChangeOwner: u32 = 0x0000_4000;
pub const kFSEventStreamEventFlagItemXattrMod: u32 = 0x0000_8000;
pub const kFSEventStreamEventFlagItemIsFile: u32 = 0x0001_0000;
pub const kFSEventStreamEventFlagItemIsDir: u32 = 0x0002_0000;
pub const kFSEventStreamEventFlagItemIsSymlink: u32 = 0x0004_0000;
pub const kFSEventStreamEventFlagItemIsHardlink: u32 = 0x0010_0000;
pub const kFSEventStreamEventFlagItemIsLastHardlink: u32 = 0x0020_0000;
pub const kFSEventStreamEventFlagItemCloned: u32 = 0x0040_0000;

/// Coarse classification used by the subscriber to route a single FSEvents
/// flag bitmask to one `JournalEvent` variant. `RenameMaybe` indicates an
/// `ItemRenamed` whose pair-half hasn't shown up yet — the subscriber
/// buffers it and pairs within a 100 ms window before deciding `Rename`
/// vs `Create` (rename-from-outside) vs `Delete` (rename-to-outside).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FlagKind {
    Create,
    Delete,
    Modify,
    AttrChange,
    /// One half of a rename pair — held pending until the matching half
    /// arrives or a timeout fires. The subscriber decides on the final
    /// `JournalEvent` variant after pairing.
    RenameMaybe,
    /// Kernel-queue overflow on a subtree. The subscriber rescans the
    /// affected directory and emits diff-events to recover.
    MustScanSubDirs,
    /// Root path itself moved/disappeared. The subscriber re-resolves the
    /// root and re-arms the stream; nothing user-facing is emitted.
    RootChanged,
    /// Symlink, dir, or other event we choose not to forward as a file
    /// event. The subscriber drops these.
    Ignore,
}

const FLAG_RENAME: u32 = kFSEventStreamEventFlagItemRenamed;
const FLAG_DELETE: u32 = kFSEventStreamEventFlagItemRemoved;
const FLAG_CREATE: u32 = kFSEventStreamEventFlagItemCreated;
const FLAG_MODIFY: u32 =
    kFSEventStreamEventFlagItemModified | kFSEventStreamEventFlagItemInodeMetaMod;
const FLAG_ATTR: u32 = kFSEventStreamEventFlagItemFinderInfoMod
    | kFSEventStreamEventFlagItemChangeOwner
    | kFSEventStreamEventFlagItemXattrMod;

/// Classify a single FSEvents flag bitmask.
///
/// Precedence (descending): `MustScanSubDirs` > `RootChanged` > `Rename`
/// > `Delete` > `Create` > `Modify` > `AttrChange` > `Ignore`.
///
/// Rationale:
/// - `MustScanSubDirs` short-circuits because the kernel is signalling
///   that other bits in the same event may be missing — we can't trust
///   them. The subscriber rescans the subtree to recover.
/// - `RootChanged` similarly invalidates the stream's frame of reference;
///   the subscriber re-resolves the root before consuming further events.
/// - `Rename` outranks Create/Delete because FSEvents emits `ItemRenamed`
///   on both halves of a rename pair, often coalesced with `ItemCreated`
///   on the new-path side and `ItemRemoved` on the old-path side. The
///   pairing logic in the subscriber needs to see them as renames.
/// - `Delete` outranks `Create` because if the same path was created and
///   then deleted within the 0.5 s coalesce window, the net filesystem
///   state is "gone."
pub fn classify(flags: u32) -> FlagKind {
    if flags & kFSEventStreamEventFlagMustScanSubDirs != 0 {
        return FlagKind::MustScanSubDirs;
    }
    if flags & kFSEventStreamEventFlagRootChanged != 0 {
        return FlagKind::RootChanged;
    }
    if flags & FLAG_RENAME != 0 {
        return FlagKind::RenameMaybe;
    }
    if flags & FLAG_DELETE != 0 {
        return FlagKind::Delete;
    }
    if flags & FLAG_CREATE != 0 {
        return FlagKind::Create;
    }
    if flags & FLAG_MODIFY != 0 {
        return FlagKind::Modify;
    }
    if flags & FLAG_ATTR != 0 {
        return FlagKind::AttrChange;
    }
    FlagKind::Ignore
}

pub fn is_file(flags: u32) -> bool {
    flags & kFSEventStreamEventFlagItemIsFile != 0
}

pub fn is_dir(flags: u32) -> bool {
    flags & kFSEventStreamEventFlagItemIsDir != 0
}

pub fn is_symlink(flags: u32) -> bool {
    flags & kFSEventStreamEventFlagItemIsSymlink != 0
}

pub fn history_done(flags: u32) -> bool {
    flags & kFSEventStreamEventFlagHistoryDone != 0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn rescan_outranks_everything() {
        let r = kFSEventStreamEventFlagMustScanSubDirs | FLAG_CREATE | FLAG_RENAME;
        assert_eq!(classify(r), FlagKind::MustScanSubDirs);
    }

    #[test]
    fn root_changed_outranks_payload_flags() {
        let r = kFSEventStreamEventFlagRootChanged | FLAG_MODIFY;
        assert_eq!(classify(r), FlagKind::RootChanged);
    }

    #[test]
    fn rename_outranks_create_and_delete() {
        assert_eq!(classify(FLAG_RENAME | FLAG_CREATE), FlagKind::RenameMaybe);
        assert_eq!(classify(FLAG_RENAME | FLAG_DELETE), FlagKind::RenameMaybe);
    }

    #[test]
    fn delete_outranks_create() {
        assert_eq!(classify(FLAG_CREATE | FLAG_DELETE), FlagKind::Delete);
    }

    #[test]
    fn modify_only_paths() {
        assert_eq!(classify(FLAG_MODIFY), FlagKind::Modify);
        assert_eq!(
            classify(kFSEventStreamEventFlagItemInodeMetaMod),
            FlagKind::Modify
        );
        assert_eq!(
            classify(kFSEventStreamEventFlagItemModified),
            FlagKind::Modify
        );
    }

    #[test]
    fn attr_only_paths() {
        assert_eq!(
            classify(kFSEventStreamEventFlagItemFinderInfoMod),
            FlagKind::AttrChange
        );
        assert_eq!(
            classify(kFSEventStreamEventFlagItemChangeOwner),
            FlagKind::AttrChange
        );
        assert_eq!(
            classify(kFSEventStreamEventFlagItemXattrMod),
            FlagKind::AttrChange
        );
    }

    #[test]
    fn no_actionable_bits_is_ignored() {
        assert_eq!(classify(0), FlagKind::Ignore);
        // Type-only flags don't constitute an event by themselves.
        assert_eq!(
            classify(kFSEventStreamEventFlagItemIsFile),
            FlagKind::Ignore
        );
        assert_eq!(classify(kFSEventStreamEventFlagItemIsDir), FlagKind::Ignore);
    }

    #[test]
    fn type_predicates() {
        assert!(is_file(kFSEventStreamEventFlagItemIsFile));
        assert!(!is_file(kFSEventStreamEventFlagItemIsDir));
        assert!(is_dir(kFSEventStreamEventFlagItemIsDir));
        assert!(is_symlink(kFSEventStreamEventFlagItemIsSymlink));
        assert!(history_done(kFSEventStreamEventFlagHistoryDone));
    }
}
