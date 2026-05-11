//! Real JSON-backed bookmarks store. The data is mock (only what the
//! user adds), but the persistence path is real — restart preserves
//! bookmarks per Rule #10.

use std::path::{Path, PathBuf};
use std::sync::Mutex;
use std::time::{SystemTime, UNIX_EPOCH};

use serde::{Deserialize, Serialize};
use tauri::{Manager, State};

const MAX_BOOKMARK_NAME_LEN: usize = 256;
const MAX_BOOKMARK_QUERY_LEN: usize = 64_000;
const MAX_BOOKMARKS: usize = 1024;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Bookmark {
    pub id: String,
    pub name: String,
    pub query: String,
    pub created_ms: u64,
    /// Type-filter chip set active when the bookmark was saved (e.g.
    /// `["audio","video"]`). On restore the UI re-applies it to
    /// `typeFilterStore`. Optional + defaulted so older bookmarks.json
    /// files keep deserializing.
    #[serde(default)]
    pub filters: Vec<String>,
}

pub struct BookmarksStore {
    pub path: PathBuf,
    pub items: Mutex<Vec<Bookmark>>,
}

impl BookmarksStore {
    pub fn new(app: &tauri::AppHandle) -> Self {
        let path = data_path(app, "bookmarks.json");
        let items = read_from_disk(&path).unwrap_or_default();
        Self {
            path,
            items: Mutex::new(items),
        }
    }
}

fn data_path(app: &tauri::AppHandle, file: &str) -> PathBuf {
    let dir = app
        .path()
        .app_data_dir()
        .unwrap_or_else(|_| std::env::temp_dir().join("sourcerer"));
    let _ = std::fs::create_dir_all(&dir);
    dir.join(file)
}

fn read_from_disk(path: &PathBuf) -> Option<Vec<Bookmark>> {
    let data = std::fs::read_to_string(path).ok()?;
    serde_json::from_str(&data).ok()
}

fn write_to_disk(path: &Path, items: &[Bookmark]) {
    // M2 fix: tmp+rename so a crash mid-write can't truncate the file
    // and silently reset bookmarks to defaults on next launch.
    let json = match serde_json::to_string_pretty(items) {
        Ok(j) => j,
        Err(_) => return,
    };
    let tmp = path.with_extension("json.tmp");
    if std::fs::write(&tmp, &json).is_err() {
        return;
    }
    let _ = std::fs::rename(&tmp, path);
}

fn fresh_id() -> String {
    // L5 fix: include a non-monotonic component so id enumeration can't
    // probe how many bookmarks the user has saved historically.
    use std::sync::atomic::{AtomicU64, Ordering};
    static N: AtomicU64 = AtomicU64::new(1);
    let counter = N.fetch_add(1, Ordering::Relaxed);
    let salt = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|d| d.subsec_nanos())
        .unwrap_or(0);
    format!("bm-{counter}-{salt:08x}")
}

fn now_ms() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|d| d.as_millis() as u64)
        .unwrap_or(0)
}

#[tauri::command]
pub fn bookmarks_list(store: State<'_, BookmarksStore>) -> Vec<Bookmark> {
    store.items.lock().unwrap().clone()
}

#[tauri::command]
pub fn bookmarks_save(
    name: String,
    query: String,
    filters: Option<Vec<String>>,
    store: State<'_, BookmarksStore>,
) -> Result<Bookmark, String> {
    if name.is_empty() {
        return Err("bookmark name cannot be empty".into());
    }
    if name.len() > MAX_BOOKMARK_NAME_LEN {
        return Err(format!(
            "bookmark name too long ({} > {MAX_BOOKMARK_NAME_LEN})",
            name.len()
        ));
    }
    if query.len() > MAX_BOOKMARK_QUERY_LEN {
        return Err(format!(
            "bookmark query too long ({} > {MAX_BOOKMARK_QUERY_LEN})",
            query.len()
        ));
    }
    let filters = filters.unwrap_or_default();
    let mut guard = store.items.lock().unwrap();
    // Dedupe on (name, query, filters) — repeated Ctrl+D on the same
    // state returns the existing bookmark instead of creating duplicates.
    if let Some(existing) = guard
        .iter()
        .find(|b| b.name == name && b.query == query && b.filters == filters)
    {
        return Ok(existing.clone());
    }
    if guard.len() >= MAX_BOOKMARKS {
        return Err(format!(
            "bookmark limit reached ({MAX_BOOKMARKS}); delete one to add another"
        ));
    }
    let bm = Bookmark {
        id: fresh_id(),
        name,
        query,
        created_ms: now_ms(),
        filters,
    };
    guard.push(bm.clone());
    write_to_disk(&store.path, &guard);
    Ok(bm)
}

#[tauri::command]
pub fn bookmarks_delete(id: String, store: State<'_, BookmarksStore>) {
    let mut guard = store.items.lock().unwrap();
    guard.retain(|b| b.id != id);
    write_to_disk(&store.path, &guard);
}

#[tauri::command]
pub fn bookmarks_rename(
    id: String,
    name: String,
    store: State<'_, BookmarksStore>,
) -> Result<(), String> {
    if name.is_empty() {
        return Err("bookmark name cannot be empty".into());
    }
    if name.len() > MAX_BOOKMARK_NAME_LEN {
        return Err(format!(
            "bookmark name too long ({} > {MAX_BOOKMARK_NAME_LEN})",
            name.len()
        ));
    }
    let mut guard = store.items.lock().unwrap();
    if let Some(b) = guard.iter_mut().find(|b| b.id == id) {
        b.name = name;
    }
    write_to_disk(&store.path, &guard);
    Ok(())
}
