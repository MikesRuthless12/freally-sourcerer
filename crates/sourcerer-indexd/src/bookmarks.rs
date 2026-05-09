//! Bookmark store — kept on the UI side in Phase 12 because the
//! bookmarks list is a piece of per-user UI state, not an index-affecting
//! setting. The daemon doesn't need to know which queries the user has
//! starred. This module exists to anchor a future migration if bookmarks
//! ever need to follow the user across machines.

#[derive(Debug, Clone)]
pub struct BookmarksOnDaemon;
