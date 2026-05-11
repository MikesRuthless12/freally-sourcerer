// Declarative menu spec — single source of truth for the menu bar.
// Both the in-window MenuBar.svelte (Win/Linux) and the macOS native
// menu builder (src-tauri) consume this. PRD §8.28.

import type { CommandId } from "./ids";

export interface MenuItemSpec {
  kind: "item";
  id: CommandId;
  label: string;
  /** Localization key (overrides label when present). */
  l10n?: string;
  /** Hover hint shown in the rightmost status-bar segment. */
  hint?: string;
  /** Cosmetic shortcut label (real binding in shortcuts.ts). */
  accelerator?: string;
  /** Item is a checkbox; checked-state read from the store at render time. */
  checkable?: boolean;
  /** True when this item is part of a radio group. */
  radio?: boolean;
}

export interface MenuSeparator {
  kind: "separator";
}

export interface MenuSubmenu {
  kind: "submenu";
  label: string;
  l10n?: string;
  hint?: string;
  children: MenuNode[];
}

export type MenuNode = MenuItemSpec | MenuSeparator | MenuSubmenu;

export interface MenuRoot {
  label: string;
  l10n?: string;
  hint: string;
  children: MenuNode[];
}

export const MENU_BAR: MenuRoot[] = [
  {
    label: "File",
    l10n: "menu.file",
    hint: "Contains commands for working with Sourcerer.",
    children: [
      { kind: "item", id: "file.new_window", label: "New Search Window", accelerator: "Ctrl+N" },
      { kind: "item", id: "file.open_file_list", label: "Open File List…", accelerator: "Ctrl+O" },
      { kind: "item", id: "file.close_file_list", label: "Close File List" },
      { kind: "item", id: "file.close", label: "Close", accelerator: "Ctrl+W" },
      { kind: "item", id: "file.export_results", label: "Export Results…", accelerator: "Ctrl+S" },
      { kind: "item", id: "file.export_index_bundle", label: "Export Index Bundle…" },
      { kind: "separator" },
      { kind: "item", id: "file.exit", label: "Exit", accelerator: "Ctrl+Q" }
    ]
  },
  {
    label: "Edit",
    l10n: "menu.edit",
    hint: "Contains commands for editing search results.",
    children: [
      { kind: "item", id: "edit.cut", label: "Cut", accelerator: "Ctrl+X" },
      { kind: "item", id: "edit.copy", label: "Copy", accelerator: "Ctrl+C" },
      { kind: "item", id: "edit.paste", label: "Paste", accelerator: "Ctrl+V" },
      { kind: "item", id: "edit.copy_to_folder", label: "Copy to Folder…" },
      { kind: "item", id: "edit.move_to_folder", label: "Move to Folder…" },
      { kind: "item", id: "edit.select_all", label: "Select All", accelerator: "Ctrl+A" },
      { kind: "item", id: "edit.invert_selection", label: "Invert Selection" },
      {
        kind: "submenu",
        label: "Advanced",
        children: [
          { kind: "item", id: "edit.advanced.copy_full_name", label: "Copy Full Name" },
          { kind: "item", id: "edit.advanced.copy_path", label: "Copy Path" },
          { kind: "item", id: "edit.advanced.copy_filename", label: "Copy Filename" },
          { kind: "item", id: "edit.advanced.copy_as_json", label: "Copy as JSON" },
          { kind: "item", id: "edit.advanced.copy_with_metadata", label: "Copy with metadata" },
          { kind: "item", id: "edit.advanced.copy_as_bundle_ref", label: "Copy as Sourcerer Bundle reference" }
        ]
      }
    ]
  },
  {
    label: "View",
    l10n: "menu.view",
    hint: "Contains commands for manipulating the view.",
    children: [
      { kind: "item", id: "view.filters", label: "Filters" },
      { kind: "item", id: "view.preview", label: "Preview", accelerator: "Alt+P", checkable: true },
      { kind: "item", id: "view.status_bar", label: "Status Bar", checkable: true },
      { kind: "item", id: "view.thumbs.xl", label: "Extra Large Thumbnails", accelerator: "Ctrl+Shift+1" },
      { kind: "item", id: "view.thumbs.l", label: "Large Thumbnails", accelerator: "Ctrl+Shift+2" },
      { kind: "item", id: "view.thumbs.m", label: "Medium Thumbnails", accelerator: "Ctrl+Shift+3" },
      { kind: "item", id: "view.details", label: "Details", accelerator: "Ctrl+Shift+6", checkable: true },
      {
        kind: "submenu",
        label: "Window Size",
        hint: "Contains commands for adjusting the size of the window.",
        children: [
          { kind: "item", id: "view.window_size.small", label: "Small", accelerator: "Alt+1" },
          { kind: "item", id: "view.window_size.medium", label: "Medium", accelerator: "Alt+2" },
          { kind: "item", id: "view.window_size.large", label: "Large", accelerator: "Alt+3" },
          { kind: "item", id: "view.window_size.auto", label: "Auto Fit", accelerator: "Alt+4" }
        ]
      },
      {
        kind: "submenu",
        label: "Zoom",
        hint: "Contains commands for adjusting the font and icon size.",
        children: [
          { kind: "item", id: "view.zoom.in", label: "Zoom In", accelerator: "Ctrl+=" },
          { kind: "item", id: "view.zoom.out", label: "Zoom Out", accelerator: "Ctrl+-" },
          { kind: "item", id: "view.zoom.reset", label: "Reset", accelerator: "Ctrl+0" }
        ]
      },
      {
        kind: "submenu",
        label: "Sort by",
        hint: "Contains commands for sorting the result list.",
        children: [
          { kind: "item", id: "view.sort.name", label: "Name", accelerator: "Ctrl+1", radio: true },
          { kind: "item", id: "view.sort.path", label: "Path", accelerator: "Ctrl+2", radio: true },
          { kind: "item", id: "view.sort.size", label: "Size", accelerator: "Ctrl+3", radio: true },
          { kind: "item", id: "view.sort.ext", label: "Extension", accelerator: "Ctrl+4", radio: true },
          { kind: "item", id: "view.sort.type", label: "Type", accelerator: "Ctrl+5", radio: true },
          { kind: "item", id: "view.sort.modified", label: "Date Modified", accelerator: "Ctrl+6", radio: true },
          { kind: "item", id: "view.sort.created", label: "Date Created", accelerator: "Ctrl+7", radio: true },
          { kind: "item", id: "view.sort.accessed", label: "Date Accessed", radio: true },
          { kind: "item", id: "view.sort.attributes", label: "Attributes", accelerator: "Ctrl+8", radio: true },
          { kind: "item", id: "view.sort.recently_changed", label: "Date Recently Changed", accelerator: "Ctrl+9", radio: true },
          { kind: "item", id: "view.sort.run_count", label: "Run Count", radio: true },
          { kind: "item", id: "view.sort.run_date", label: "Date Run", radio: true },
          { kind: "item", id: "view.sort.file_list_filename", label: "File List Filename", radio: true },
          { kind: "item", id: "view.sort.lufs", label: "LUFS", accelerator: "Ctrl+L", radio: true },
          { kind: "item", id: "view.sort.length", label: "Length", accelerator: "Ctrl+Shift+L", radio: true },
          { kind: "item", id: "view.sort.similarity", label: "Similarity Score", radio: true },
          { kind: "separator" },
          { kind: "item", id: "view.sort.ascending", label: "Ascending", radio: true },
          { kind: "item", id: "view.sort.descending", label: "Descending", radio: true }
        ]
      },
      { kind: "item", id: "view.go_to", label: "Go To" },
      { kind: "item", id: "view.refresh", label: "Refresh", accelerator: "F5" },
      {
        kind: "submenu",
        label: "Theme",
        hint: "Switch between system, light, or dark themes.",
        children: [
          { kind: "item", id: "view.theme.system", label: "System", radio: true },
          { kind: "item", id: "view.theme.light", label: "Light", radio: true },
          { kind: "item", id: "view.theme.dark", label: "Dark", radio: true }
        ]
      },
      {
        kind: "submenu",
        label: "Lenses",
        hint: "Toggle visibility of each lens in the result list.",
        children: [
          { kind: "item", id: "view.lens.filename", label: "Filename", checkable: true },
          { kind: "item", id: "view.lens.content", label: "Content", checkable: true },
          { kind: "item", id: "view.lens.audio", label: "Audio", checkable: true },
          { kind: "item", id: "view.lens.similarity", label: "Similarity", checkable: true }
        ]
      },
      {
        kind: "submenu",
        label: "On Top",
        hint: "Contains commands for keeping this window on top of other windows.",
        children: [
          { kind: "item", id: "view.on_top.never", label: "Never", radio: true },
          { kind: "item", id: "view.on_top.always", label: "Always", radio: true },
          { kind: "item", id: "view.on_top.while_searching", label: "While Searching", radio: true }
        ]
      }
    ]
  },
  {
    label: "Search",
    l10n: "menu.search",
    hint: "Contains search toggles.",
    children: [
      { kind: "item", id: "search.match_case", label: "Match Case", accelerator: "Ctrl+I", checkable: true },
      { kind: "item", id: "search.match_whole_word", label: "Match Whole Word", accelerator: "Ctrl+B", checkable: true },
      { kind: "item", id: "search.match_path", label: "Match Path", accelerator: "Ctrl+U", checkable: true },
      { kind: "item", id: "search.match_diacritics", label: "Match Diacritics", accelerator: "Ctrl+M", checkable: true },
      { kind: "item", id: "search.enable_regex", label: "Enable Regex", accelerator: "Ctrl+R", checkable: true },
      { kind: "item", id: "search.advanced", label: "Advanced Search…" },
      { kind: "item", id: "search.add_to_filters", label: "Add to Filters…" },
      { kind: "item", id: "search.organize_filters", label: "Organize Filters…", accelerator: "Ctrl+Shift+F" },
      { kind: "separator" },
      { kind: "item", id: "search.filter.everything", label: "Everything", checkable: true },
      { kind: "item", id: "search.filter.audio", label: "Audio", checkable: true },
      { kind: "item", id: "search.filter.compressed", label: "Compressed (Archive)", checkable: true },
      { kind: "item", id: "search.filter.document", label: "Document", checkable: true },
      { kind: "item", id: "search.filter.executable", label: "Executable", checkable: true },
      { kind: "item", id: "search.filter.folder", label: "Folder", checkable: true },
      { kind: "item", id: "search.filter.picture", label: "Picture", checkable: true },
      { kind: "item", id: "search.filter.video", label: "Video", checkable: true },
      { kind: "separator" },
      { kind: "item", id: "search.filter.custom", label: "Custom Filter…" }
    ]
  },
  {
    label: "Bookmarks",
    l10n: "menu.bookmarks",
    hint: "Contains commands for working with bookmarks.",
    children: [
      { kind: "item", id: "bookmarks.add", label: "Add to Bookmarks", accelerator: "Ctrl+D" },
      { kind: "item", id: "bookmarks.organize", label: "Organize Bookmarks…", accelerator: "Ctrl+Shift+B" },
      { kind: "separator" }
    ]
  },
  {
    label: "Tools",
    l10n: "menu.tools",
    hint: "Contains tools commands.",
    children: [
      { kind: "item", id: "tools.connect_endpoint", label: "Connect to FTP Server…" },
      { kind: "item", id: "tools.disconnect_endpoint", label: "Disconnect from FTP Server" },
      { kind: "item", id: "tools.file_list_editor", label: "File List Editor…" },
      {
        kind: "submenu",
        label: "Index maintenance",
        hint: "Index maintenance tools.",
        children: [
          { kind: "item", id: "tools.verify_index", label: "Verify Index…" },
          { kind: "item", id: "tools.compact_index", label: "Compact Index…" },
          { kind: "item", id: "tools.rebuild_index", label: "Force Rebuild Index…" }
        ]
      },
      { kind: "item", id: "tools.custom_extractor_manager", label: "Custom Extractor Manager…", hint: "Manage Wasm-sandboxed custom extractors." },
      { kind: "item", id: "tools.options", label: "Options…", accelerator: "Ctrl+," }
    ]
  },
  {
    label: "Help",
    l10n: "menu.help",
    hint: "Contains help commands.",
    children: [
      { kind: "item", id: "help.help", label: "Sourcerer Help", accelerator: "F1" },
      { kind: "item", id: "help.search_syntax", label: "Search Syntax" },
      { kind: "item", id: "help.regex_syntax", label: "Regex Syntax" },
      { kind: "item", id: "help.audio_modifier_reference", label: "Audio Modifier Reference" },
      { kind: "item", id: "help.similarity_modifier_reference", label: "Similarity Modifier Reference" },
      { kind: "item", id: "help.command_line_options", label: "Command Line Options" },
      { kind: "item", id: "help.website", label: "Sourcerer Website" },
      { kind: "item", id: "help.check_for_updates", label: "Check for Updates…" },
      { kind: "item", id: "help.sponsor", label: "Sponsor / Donate" },
      { kind: "separator" },
      { kind: "item", id: "help.about", label: "About Sourcerer…", accelerator: "Ctrl+F1" }
    ]
  }
];

export function* iterItems(roots: MenuRoot[] = MENU_BAR): Generator<MenuItemSpec> {
  function* walk(node: MenuNode): Generator<MenuItemSpec> {
    if (node.kind === "item") yield node;
    else if (node.kind === "submenu") {
      for (const c of node.children) yield* walk(c);
    }
  }
  for (const r of roots) for (const c of r.children) yield* walk(c);
}
