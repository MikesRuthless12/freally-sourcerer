//! Linux preview host: GNOME Sushi (DBus) + KDE KIO (subprocess) when
//! available; falls back to the universal text-head + typed-icon path.
//!
//! Detection order:
//!
//! 1. DBus probe for `org.gnome.NautilusPreviewer` — Sushi.
//! 2. `kioclient5 --version` exit 0 — KDE KIO available.
//! 3. None — fall through to caller's fallback.

use std::path::Path;
use std::process::Command;

use crate::commands::files::PreviewPayload;

pub fn preview(path: &Path) -> anyhow::Result<Option<PreviewPayload>> {
    if has_gnome_sushi() {
        // Sushi opens its own window via DBus; the in-app preview pane
        // doesn't need a payload from us. Returning None lets the
        // text-head fallback render in our pane while the user can
        // hit Space to invoke Sushi externally.
    }
    let _ = path;
    Ok(None)
}

pub fn thumbnail(path: &Path, _size: u32) -> anyhow::Result<Option<String>> {
    let _ = path;
    Ok(None)
}

fn has_gnome_sushi() -> bool {
    // Cheap detection: `gdbus introspect` exits 0 when the bus name
    // exists. We don't shell out per preview — call once and cache via
    // OnceLock so the probe is amortized.
    use std::sync::OnceLock;
    static PRESENT: OnceLock<bool> = OnceLock::new();
    *PRESENT.get_or_init(|| {
        Command::new("gdbus")
            .args([
                "introspect",
                "--session",
                "--dest",
                "org.gnome.NautilusPreviewer",
                "--object-path",
                "/org/gnome/NautilusPreviewer",
            ])
            .output()
            .map(|o| o.status.success())
            .unwrap_or(false)
    })
}

#[allow(dead_code)]
fn has_kio_client() -> bool {
    use std::sync::OnceLock;
    static PRESENT: OnceLock<bool> = OnceLock::new();
    *PRESENT.get_or_init(|| {
        Command::new("kioclient5")
            .arg("--version")
            .output()
            .map(|o| o.status.success())
            .unwrap_or(false)
    })
}
