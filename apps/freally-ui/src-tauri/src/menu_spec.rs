//! Rust mirror of `apps/freally-ui/src/lib/commands/menu_spec.ts`.
//!
//! Two sources of truth feels wrong, but the alternative — generating one
//! from the other at build time — adds a codegen step that does not pay
//! off for Phase 11. The parity test in
//! `tests/ui/menubar/parity.rs` asserts both specs cover the same set of
//! `CommandId`s by hardcoding the list against PRD §8.28.

#[derive(Debug, Clone)]
pub struct MenuItemSpec {
    pub id: &'static str,
    pub label: &'static str,
    pub accel: Option<&'static str>,
    pub hint: Option<&'static str>,
    pub checkable: bool,
}

#[derive(Debug, Clone)]
pub enum NodeSpec {
    Item(MenuItemSpec),
    Separator,
    Submenu {
        label: &'static str,
        hint: Option<&'static str>,
        children: Vec<NodeSpec>,
    },
}

#[derive(Debug, Clone)]
pub struct RootSpec {
    pub label: &'static str,
    pub hint: &'static str,
    pub children: Vec<NodeSpec>,
}

fn item(id: &'static str, label: &'static str) -> NodeSpec {
    NodeSpec::Item(MenuItemSpec {
        id,
        label,
        accel: None,
        hint: None,
        checkable: false,
    })
}

fn item_a(id: &'static str, label: &'static str, accel: &'static str) -> NodeSpec {
    NodeSpec::Item(MenuItemSpec {
        id,
        label,
        accel: Some(accel),
        hint: None,
        checkable: false,
    })
}

fn item_check(id: &'static str, label: &'static str) -> NodeSpec {
    NodeSpec::Item(MenuItemSpec {
        id,
        label,
        accel: None,
        hint: None,
        checkable: true,
    })
}

fn item_check_a(id: &'static str, label: &'static str, accel: &'static str) -> NodeSpec {
    NodeSpec::Item(MenuItemSpec {
        id,
        label,
        accel: Some(accel),
        hint: None,
        checkable: true,
    })
}

fn sep() -> NodeSpec {
    NodeSpec::Separator
}

pub fn menu_bar() -> Vec<RootSpec> {
    vec![
        RootSpec {
            label: "File",
            hint: "Contains commands for working with Freally.",
            children: vec![
                item_a("file.new_window", "New Search Window", "Ctrl+N"),
                item_a("file.open_file_list", "Open File List…", "Ctrl+O"),
                item("file.close_file_list", "Close File List"),
                item_a("file.close", "Close", "Ctrl+W"),
                item_a("file.export_results", "Export Results…", "Ctrl+S"),
                item("file.export_index_bundle", "Export Index Bundle…"),
                sep(),
                item_a("file.exit", "Exit", "Ctrl+Q"),
            ],
        },
        RootSpec {
            label: "Edit",
            hint: "Contains commands for editing search results.",
            children: vec![
                item_a("edit.cut", "Cut", "Ctrl+X"),
                item_a("edit.copy", "Copy", "Ctrl+C"),
                item_a("edit.paste", "Paste", "Ctrl+V"),
                item("edit.copy_to_folder", "Copy to Folder…"),
                item("edit.move_to_folder", "Move to Folder…"),
                item_a("edit.select_all", "Select All", "Ctrl+A"),
                item("edit.invert_selection", "Invert Selection"),
                NodeSpec::Submenu {
                    label: "Advanced",
                    hint: None,
                    children: vec![
                        item("edit.advanced.copy_full_name", "Copy Full Name"),
                        item("edit.advanced.copy_path", "Copy Path"),
                        item("edit.advanced.copy_filename", "Copy Filename"),
                        item("edit.advanced.copy_as_json", "Copy as JSON"),
                        item("edit.advanced.copy_with_metadata", "Copy with metadata"),
                        item(
                            "edit.advanced.copy_as_bundle_ref",
                            "Copy as Freally Bundle reference",
                        ),
                    ],
                },
            ],
        },
        RootSpec {
            label: "View",
            hint: "Contains commands for manipulating the view.",
            children: vec![
                item("view.filters", "Filters"),
                item_check_a("view.preview", "Preview", "Alt+P"),
                item_check("view.status_bar", "Status Bar"),
                item_a("view.thumbs.xl", "Extra Large Thumbnails", "Ctrl+Shift+1"),
                item_a("view.thumbs.l", "Large Thumbnails", "Ctrl+Shift+2"),
                item_a("view.thumbs.m", "Medium Thumbnails", "Ctrl+Shift+3"),
                item_check_a("view.details", "Details", "Ctrl+Shift+6"),
                NodeSpec::Submenu {
                    label: "Window Size",
                    hint: Some("Contains commands for adjusting the size of the window."),
                    children: vec![
                        item_a("view.window_size.small", "Small", "Alt+1"),
                        item_a("view.window_size.medium", "Medium", "Alt+2"),
                        item_a("view.window_size.large", "Large", "Alt+3"),
                        item_a("view.window_size.auto", "Auto Fit", "Alt+4"),
                    ],
                },
                NodeSpec::Submenu {
                    label: "Zoom",
                    hint: Some("Contains commands for adjusting the font and icon size."),
                    children: vec![
                        item_a("view.zoom.in", "Zoom In", "Ctrl+="),
                        item_a("view.zoom.out", "Zoom Out", "Ctrl+-"),
                        item_a("view.zoom.reset", "Reset", "Ctrl+0"),
                    ],
                },
                NodeSpec::Submenu {
                    label: "Sort by",
                    hint: Some("Contains commands for sorting the result list."),
                    children: vec![
                        item_a("view.sort.name", "Name", "Ctrl+1"),
                        item_a("view.sort.path", "Path", "Ctrl+2"),
                        item_a("view.sort.size", "Size", "Ctrl+3"),
                        item_a("view.sort.ext", "Extension", "Ctrl+4"),
                        item_a("view.sort.type", "Type", "Ctrl+5"),
                        item_a("view.sort.modified", "Date Modified", "Ctrl+6"),
                        item_a("view.sort.created", "Date Created", "Ctrl+7"),
                        item("view.sort.accessed", "Date Accessed"),
                        item_a("view.sort.attributes", "Attributes", "Ctrl+8"),
                        item_a(
                            "view.sort.recently_changed",
                            "Date Recently Changed",
                            "Ctrl+9",
                        ),
                        item("view.sort.run_count", "Run Count"),
                        item("view.sort.run_date", "Date Run"),
                        item("view.sort.file_list_filename", "File List Filename"),
                        item_a("view.sort.lufs", "LUFS", "Ctrl+L"),
                        item_a("view.sort.length", "Length", "Ctrl+Shift+L"),
                        item("view.sort.similarity", "Similarity Score"),
                        sep(),
                        item("view.sort.ascending", "Ascending"),
                        item("view.sort.descending", "Descending"),
                    ],
                },
                item("view.go_to", "Go To"),
                item_a("view.refresh", "Refresh", "F5"),
                NodeSpec::Submenu {
                    label: "Theme",
                    hint: Some("Switch between system, light, or dark themes."),
                    children: vec![
                        item("view.theme.system", "System"),
                        item("view.theme.light", "Light"),
                        item("view.theme.dark", "Dark"),
                    ],
                },
                NodeSpec::Submenu {
                    label: "Lenses",
                    hint: Some("Toggle visibility of each lens in the result list."),
                    children: vec![
                        item_check("view.lens.filename", "Filename"),
                        item_check("view.lens.content", "Content"),
                        item_check("view.lens.audio", "Audio"),
                        item_check("view.lens.similarity", "Similarity"),
                    ],
                },
                NodeSpec::Submenu {
                    label: "On Top",
                    hint: Some(
                        "Contains commands for keeping this window on top of other windows.",
                    ),
                    children: vec![
                        item("view.on_top.never", "Never"),
                        item("view.on_top.always", "Always"),
                        item("view.on_top.while_searching", "While Searching"),
                    ],
                },
            ],
        },
        RootSpec {
            label: "Search",
            hint: "Contains search toggles.",
            children: vec![
                item_check_a("search.match_case", "Match Case", "Ctrl+I"),
                item_check_a("search.match_whole_word", "Match Whole Word", "Ctrl+B"),
                item_check_a("search.match_path", "Match Path", "Ctrl+U"),
                item_check_a("search.match_diacritics", "Match Diacritics", "Ctrl+M"),
                item_check_a("search.enable_regex", "Enable Regex", "Ctrl+R"),
                item("search.advanced", "Advanced Search…"),
                item("search.add_to_filters", "Add to Filters…"),
                item_a(
                    "search.organize_filters",
                    "Organize Filters…",
                    "Ctrl+Shift+F",
                ),
                sep(),
                item("search.filter.everything", "Everything"),
                item("search.filter.audio", "Audio"),
                item("search.filter.compressed", "Compressed (Archive)"),
                item("search.filter.document", "Document"),
                item("search.filter.executable", "Executable"),
                item("search.filter.folder", "Folder"),
                item("search.filter.picture", "Picture"),
                item("search.filter.video", "Video"),
                sep(),
                item("search.filter.custom", "Custom Filter…"),
            ],
        },
        RootSpec {
            label: "Bookmarks",
            hint: "Contains commands for working with bookmarks.",
            children: vec![
                item_a("bookmarks.add", "Add to Bookmarks", "Ctrl+D"),
                item_a("bookmarks.organize", "Organize Bookmarks…", "Ctrl+Shift+B"),
            ],
        },
        RootSpec {
            label: "Tools",
            hint: "Contains tools commands.",
            children: vec![
                item("tools.connect_endpoint", "Connect to HTTPS API Endpoint…"),
                item(
                    "tools.disconnect_endpoint",
                    "Disconnect from HTTPS API Endpoint",
                ),
                item("tools.file_list_editor", "File List Editor…"),
                NodeSpec::Submenu {
                    label: "Index maintenance",
                    hint: Some("Index maintenance tools."),
                    children: vec![
                        item("tools.verify_index", "Verify Index…"),
                        item("tools.compact_index", "Compact Index…"),
                        item("tools.rebuild_index", "Force Rebuild Index…"),
                    ],
                },
                item(
                    "tools.custom_extractor_manager",
                    "Custom Extractor Manager…",
                ),
                item_a("tools.options", "Options…", "Ctrl+,"),
            ],
        },
        RootSpec {
            label: "Help",
            hint: "Contains help commands.",
            children: vec![
                item_a("help.help", "Freally Help", "F1"),
                item("help.search_syntax", "Search Syntax"),
                item("help.regex_syntax", "Regex Syntax"),
                item("help.audio_modifier_reference", "Audio Modifier Reference"),
                item(
                    "help.similarity_modifier_reference",
                    "Similarity Modifier Reference",
                ),
                item("help.command_line_options", "Command Line Options"),
                item("help.website", "Freally Website"),
                item("help.check_for_updates", "Check for Updates…"),
                item("help.sponsor", "Sponsor / Donate"),
                sep(),
                item_a("help.about", "About Freally…", "Ctrl+F1"),
            ],
        },
    ]
}

/// Flat list of every command id this spec covers — used by the parity test
/// to assert lockstep with PRD §8.28.
pub fn all_command_ids() -> Vec<&'static str> {
    let mut out = Vec::new();
    for root in menu_bar() {
        collect(&root.children, &mut out);
    }
    out
}

fn collect(nodes: &[NodeSpec], out: &mut Vec<&'static str>) {
    for n in nodes {
        match n {
            NodeSpec::Item(it) => out.push(it.id),
            NodeSpec::Submenu { children, .. } => collect(children, out),
            NodeSpec::Separator => {}
        }
    }
}
