//! Builds the Tauri 2 native menu for macOS (global menu bar) from
//! `menu_spec.rs`. Win/Linux still render the in-window MenuBar.svelte;
//! native menu integration on those platforms is a Phase 12 nicety.
//!
//! Click events emit a `menu-command` Tauri event carrying the
//! `CommandId`; the UI's `bootstrap.ts` listens and dispatches through
//! the in-process command registry. Keyboard-shortcut routing also
//! flows through the registry on the UI side, so a keystroke that
//! matches an accelerator runs the same handler whether the user
//! clicked the native menu or pressed the key.

use tauri::menu::{
    IsMenuItem, Menu, MenuBuilder, MenuItem, MenuItemBuilder, PredefinedMenuItem, Submenu,
    SubmenuBuilder,
};
use tauri::{AppHandle, Emitter, Manager, Runtime};

use crate::menu_spec::{NodeSpec, RootSpec, menu_bar};

pub fn build_app_menu<R: Runtime>(app: &AppHandle<R>) -> tauri::Result<Menu<R>> {
    let mut builder = MenuBuilder::new(app);

    // macOS-required app menu (Freally → About / Preferences / Hide / Quit).
    if cfg!(target_os = "macos") {
        let about = MenuItemBuilder::with_id("help.about", "About Freally").build(app)?;
        let prefs = MenuItemBuilder::with_id("tools.options", "Preferences…")
            .accelerator("CmdOrCtrl+,")
            .build(app)?;
        let quit = MenuItemBuilder::with_id("file.exit", "Quit Freally")
            .accelerator("CmdOrCtrl+Q")
            .build(app)?;
        let sep1 = PredefinedMenuItem::separator(app)?;
        let sep2 = PredefinedMenuItem::separator(app)?;
        let app_menu = SubmenuBuilder::new(app, "Freally")
            .item(&about)
            .item(&sep1)
            .item(&prefs)
            .item(&sep2)
            .item(&quit)
            .build()?;
        builder = builder.item(&app_menu);
    }

    for root in menu_bar() {
        let sm = build_root::<R>(app, &root)?;
        builder = builder.item(&sm);
    }

    builder.build()
}

fn build_root<R: Runtime>(app: &AppHandle<R>, root: &RootSpec) -> tauri::Result<Submenu<R>> {
    let kids = build_children(app, &root.children)?;
    let mut sb = SubmenuBuilder::new(app, root.label);
    for k in &kids {
        sb = sb.item(k.as_ref());
    }
    sb.build()
}

fn build_children<R: Runtime>(
    app: &AppHandle<R>,
    nodes: &[NodeSpec],
) -> tauri::Result<Vec<Box<dyn IsMenuItem<R>>>> {
    let mut out: Vec<Box<dyn IsMenuItem<R>>> = Vec::with_capacity(nodes.len());
    for node in nodes {
        out.push(build_node(app, node)?);
    }
    Ok(out)
}

fn build_node<R: Runtime>(
    app: &AppHandle<R>,
    node: &NodeSpec,
) -> tauri::Result<Box<dyn IsMenuItem<R>>> {
    Ok(match node {
        NodeSpec::Separator => {
            let s = PredefinedMenuItem::separator(app)?;
            Box::new(s)
        }
        NodeSpec::Item(spec) => {
            let mut b = MenuItemBuilder::with_id(spec.id, spec.label);
            if let Some(a) = spec.accel {
                b = b.accelerator(a.replace("Ctrl", "CmdOrCtrl"));
            }
            let item: MenuItem<R> = b.build(app)?;
            Box::new(item)
        }
        NodeSpec::Submenu {
            label, children, ..
        } => {
            let kids = build_children(app, children)?;
            let mut sb = SubmenuBuilder::new(app, *label);
            for k in &kids {
                sb = sb.item(k.as_ref());
            }
            let sm: Submenu<R> = sb.build()?;
            Box::new(sm)
        }
    })
}

/// Wires the `on_menu_event` handler so the macOS native menu fires the same
/// command path the in-window menu uses. Win/Linux UIs render the in-window
/// menu directly and never see this event.
pub fn register_menu_event_relay<R: Runtime>(app: &AppHandle<R>) {
    let app_clone = app.clone();
    app.on_menu_event(move |_app, event| {
        let id = event.id().0.clone();
        if let Some(window) = app_clone.get_webview_window("main") {
            let _ = window.emit("menu-command", id);
        }
    });
}
