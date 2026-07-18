// Declarative menu spec — single source of truth for the menu bar.
// Both the in-window MenuBar.svelte (Win/Linux) and the macOS native
// menu builder (src-tauri) consume this. PRD §8.28.
//
// Every label and submenu has an `l10n` key that lives in
// `locales/<code>/freally.ftl`. MenuBar.svelte calls
// `t(spec.l10n) || spec.label` so untranslated keys fall back to the
// English literal here.

import type { CommandId } from "./ids";

export interface MenuItemSpec {
  kind: "item";
  id: CommandId;
  label: string;
  /** Localization key (overrides label when present). */
  l10n?: string;
  /** Hover hint shown in the rightmost status-bar segment. */
  hint?: string;
  /** Localization key for `hint`. */
  hintL10n?: string;
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
  hintL10n?: string;
  children: MenuNode[];
}

export type MenuNode = MenuItemSpec | MenuSeparator | MenuSubmenu;

export interface MenuRoot {
  label: string;
  l10n?: string;
  hint: string;
  hintL10n?: string;
  children: MenuNode[];
}

export const MENU_BAR: MenuRoot[] = [
  {
    label: "File",
    l10n: "menu-file",
    hint: "Contains commands for working with Freally.",
    hintL10n: "menu-file-hint",
    children: [
      { kind: "item", id: "file.new_window", label: "New Search Window", l10n: "menu-file-new-window", accelerator: "Ctrl+N" },
      { kind: "item", id: "file.open_file_list", label: "Open File List…", l10n: "menu-file-open-list", accelerator: "Ctrl+O" },
      { kind: "item", id: "file.close_file_list", label: "Close File List", l10n: "menu-file-close-list" },
      { kind: "item", id: "file.close", label: "Close", l10n: "menu-file-close", accelerator: "Ctrl+W" },
      { kind: "item", id: "file.export_results", label: "Export Results…", l10n: "menu-file-export-results", accelerator: "Ctrl+S" },
      { kind: "item", id: "file.export_index_bundle", label: "Export Index Bundle…", l10n: "menu-file-export-bundle" },
      { kind: "separator" },
      { kind: "item", id: "file.exit", label: "Exit", l10n: "menu-file-exit", accelerator: "Ctrl+Q" }
    ]
  },
  {
    label: "Edit",
    l10n: "menu-edit",
    hint: "Contains commands for editing search results.",
    hintL10n: "menu-edit-hint",
    children: [
      { kind: "item", id: "edit.cut", label: "Cut", l10n: "menu-edit-cut", accelerator: "Ctrl+X" },
      { kind: "item", id: "edit.copy", label: "Copy", l10n: "menu-edit-copy", accelerator: "Ctrl+C" },
      { kind: "item", id: "edit.paste", label: "Paste", l10n: "menu-edit-paste", accelerator: "Ctrl+V" },
      { kind: "item", id: "edit.copy_to_folder", label: "Copy to Folder…", l10n: "menu-edit-copy-to-folder" },
      { kind: "item", id: "edit.move_to_folder", label: "Move to Folder…", l10n: "menu-edit-move-to-folder" },
      { kind: "item", id: "edit.select_all", label: "Select All", l10n: "menu-edit-select-all", accelerator: "Ctrl+A" },
      { kind: "item", id: "edit.invert_selection", label: "Invert Selection", l10n: "menu-edit-invert-selection" },
      {
        kind: "submenu",
        label: "Advanced",
        l10n: "menu-edit-advanced",
        children: [
          { kind: "item", id: "edit.advanced.copy_full_name", label: "Copy Full Name", l10n: "menu-edit-copy-full-name" },
          { kind: "item", id: "edit.advanced.copy_path", label: "Copy Path", l10n: "menu-edit-copy-path" },
          { kind: "item", id: "edit.advanced.copy_filename", label: "Copy Filename", l10n: "menu-edit-copy-filename" },
          { kind: "item", id: "edit.advanced.copy_as_json", label: "Copy as JSON", l10n: "menu-edit-copy-as-json" },
          { kind: "item", id: "edit.advanced.copy_with_metadata", label: "Copy with metadata", l10n: "menu-edit-copy-with-metadata" },
          { kind: "item", id: "edit.advanced.copy_as_bundle_ref", label: "Copy as Freally Bundle reference", l10n: "menu-edit-copy-as-bundle-ref" }
        ]
      }
    ]
  },
  {
    label: "View",
    l10n: "menu-view",
    hint: "Contains commands for manipulating the view.",
    hintL10n: "menu-view-hint",
    children: [
      { kind: "item", id: "view.filters", label: "Filters", l10n: "menu-view-filters" },
      { kind: "item", id: "view.preview", label: "Preview", l10n: "menu-view-preview", accelerator: "Alt+P", checkable: true },
      { kind: "item", id: "view.status_bar", label: "Status Bar", l10n: "menu-view-status-bar", checkable: true },
      { kind: "item", id: "view.thumbs.xl", label: "Extra Large Thumbnails", l10n: "menu-view-thumbs-xl", accelerator: "Ctrl+Shift+1" },
      { kind: "item", id: "view.thumbs.l", label: "Large Thumbnails", l10n: "menu-view-thumbs-l", accelerator: "Ctrl+Shift+2" },
      { kind: "item", id: "view.thumbs.m", label: "Medium Thumbnails", l10n: "menu-view-thumbs-m", accelerator: "Ctrl+Shift+3" },
      { kind: "item", id: "view.details", label: "Details", l10n: "menu-view-details", accelerator: "Ctrl+Shift+6", checkable: true },
      {
        kind: "submenu",
        label: "Window Size",
        l10n: "menu-view-window-size",
        hint: "Contains commands for adjusting the size of the window.",
        hintL10n: "menu-view-window-size-hint",
        children: [
          { kind: "item", id: "view.window_size.small", label: "Small", l10n: "menu-view-window-small", accelerator: "Alt+1" },
          { kind: "item", id: "view.window_size.medium", label: "Medium", l10n: "menu-view-window-medium", accelerator: "Alt+2" },
          { kind: "item", id: "view.window_size.large", label: "Large", l10n: "menu-view-window-large", accelerator: "Alt+3" },
          { kind: "item", id: "view.window_size.auto", label: "Auto Fit", l10n: "menu-view-window-auto", accelerator: "Alt+4" }
        ]
      },
      {
        kind: "submenu",
        label: "Zoom",
        l10n: "menu-view-zoom",
        hint: "Contains commands for adjusting the font and icon size.",
        hintL10n: "menu-view-zoom-hint",
        children: [
          { kind: "item", id: "view.zoom.in", label: "Zoom In", l10n: "menu-view-zoom-in", accelerator: "Ctrl+=" },
          { kind: "item", id: "view.zoom.out", label: "Zoom Out", l10n: "menu-view-zoom-out", accelerator: "Ctrl+-" },
          { kind: "item", id: "view.zoom.reset", label: "Reset", l10n: "menu-view-zoom-reset", accelerator: "Ctrl+0" }
        ]
      },
      {
        kind: "submenu",
        label: "Sort by",
        l10n: "menu-view-sort-by",
        hint: "Contains commands for sorting the result list.",
        hintL10n: "menu-view-sort-by-hint",
        children: [
          { kind: "item", id: "view.sort.name", label: "Name", l10n: "menu-view-sort-name", accelerator: "Ctrl+1", radio: true },
          { kind: "item", id: "view.sort.path", label: "Path", l10n: "menu-view-sort-path", accelerator: "Ctrl+2", radio: true },
          { kind: "item", id: "view.sort.size", label: "Size", l10n: "menu-view-sort-size", accelerator: "Ctrl+3", radio: true },
          { kind: "item", id: "view.sort.ext", label: "Extension", l10n: "menu-view-sort-ext", accelerator: "Ctrl+4", radio: true },
          { kind: "item", id: "view.sort.type", label: "Type", l10n: "menu-view-sort-type", accelerator: "Ctrl+5", radio: true },
          { kind: "item", id: "view.sort.modified", label: "Date Modified", l10n: "menu-view-sort-modified", accelerator: "Ctrl+6", radio: true },
          { kind: "item", id: "view.sort.created", label: "Date Created", l10n: "menu-view-sort-created", accelerator: "Ctrl+7", radio: true },
          { kind: "item", id: "view.sort.accessed", label: "Date Accessed", l10n: "menu-view-sort-accessed", radio: true },
          { kind: "item", id: "view.sort.attributes", label: "Attributes", l10n: "menu-view-sort-attributes", accelerator: "Ctrl+8", radio: true },
          { kind: "item", id: "view.sort.recently_changed", label: "Date Recently Changed", l10n: "menu-view-sort-recently-changed", accelerator: "Ctrl+9", radio: true },
          { kind: "item", id: "view.sort.run_count", label: "Run Count", l10n: "menu-view-sort-run-count", radio: true },
          { kind: "item", id: "view.sort.run_date", label: "Date Run", l10n: "menu-view-sort-run-date", radio: true },
          { kind: "item", id: "view.sort.file_list_filename", label: "File List Filename", l10n: "menu-view-sort-file-list-filename", radio: true },
          { kind: "item", id: "view.sort.lufs", label: "LUFS", l10n: "menu-view-sort-lufs", accelerator: "Ctrl+L", radio: true },
          { kind: "item", id: "view.sort.length", label: "Length", l10n: "menu-view-sort-length", accelerator: "Ctrl+Shift+L", radio: true },
          { kind: "item", id: "view.sort.similarity", label: "Similarity Score", l10n: "menu-view-sort-similarity", radio: true },
          { kind: "separator" },
          { kind: "item", id: "view.sort.ascending", label: "Ascending", l10n: "menu-view-sort-asc", radio: true },
          { kind: "item", id: "view.sort.descending", label: "Descending", l10n: "menu-view-sort-desc", radio: true }
        ]
      },
      { kind: "item", id: "view.go_to", label: "Go To", l10n: "menu-view-go-to" },
      { kind: "item", id: "view.refresh", label: "Refresh", l10n: "menu-view-refresh", accelerator: "F5" },
      {
        kind: "submenu",
        label: "Theme",
        l10n: "menu-view-theme",
        hint: "Switch between system, light, or dark themes.",
        hintL10n: "menu-view-theme-hint",
        children: [
          { kind: "item", id: "view.theme.system", label: "System", l10n: "theme-system", radio: true },
          { kind: "item", id: "view.theme.light", label: "Light", l10n: "theme-light", radio: true },
          { kind: "item", id: "view.theme.dark", label: "Dark", l10n: "theme-dark", radio: true }
        ]
      },
      {
        kind: "submenu",
        label: "Lenses",
        l10n: "menu-view-lenses",
        hint: "Toggle visibility of each lens in the result list.",
        hintL10n: "menu-view-lenses-hint",
        children: [
          { kind: "item", id: "view.lens.filename", label: "Filename", l10n: "lens-filename", checkable: true },
          { kind: "item", id: "view.lens.content", label: "Content", l10n: "lens-content", checkable: true },
          { kind: "item", id: "view.lens.audio", label: "Audio", l10n: "lens-audio", checkable: true },
          { kind: "item", id: "view.lens.similarity", label: "Similarity", l10n: "lens-similarity", checkable: true }
        ]
      },
      {
        kind: "submenu",
        label: "On Top",
        l10n: "menu-view-on-top",
        hint: "Contains commands for keeping this window on top of other windows.",
        hintL10n: "menu-view-on-top-hint",
        children: [
          { kind: "item", id: "view.on_top.never", label: "Never", l10n: "menu-view-on-top-never", radio: true },
          { kind: "item", id: "view.on_top.always", label: "Always", l10n: "menu-view-on-top-always", radio: true },
          { kind: "item", id: "view.on_top.while_searching", label: "While Searching", l10n: "menu-view-on-top-while-searching", radio: true }
        ]
      }
    ]
  },
  {
    label: "Search",
    l10n: "menu-search",
    hint: "Contains search toggles.",
    hintL10n: "menu-search-hint",
    children: [
      { kind: "item", id: "search.match_case", label: "Match Case", l10n: "menu-search-match-case", accelerator: "Ctrl+I", checkable: true },
      { kind: "item", id: "search.match_whole_word", label: "Match Whole Word", l10n: "menu-search-match-whole-word", accelerator: "Ctrl+B", checkable: true },
      { kind: "item", id: "search.match_path", label: "Match Path", l10n: "menu-search-match-path", accelerator: "Ctrl+U", checkable: true },
      { kind: "item", id: "search.match_diacritics", label: "Match Diacritics", l10n: "menu-search-match-diacritics", accelerator: "Ctrl+M", checkable: true },
      { kind: "item", id: "search.enable_regex", label: "Enable Regex", l10n: "menu-search-enable-regex", accelerator: "Ctrl+R", checkable: true },
      { kind: "item", id: "search.advanced", label: "Advanced Search…", l10n: "menu-search-advanced" },
      { kind: "item", id: "search.add_to_filters", label: "Add to Filters…", l10n: "menu-search-add-to-filters" },
      { kind: "item", id: "search.organize_filters", label: "Organize Filters…", l10n: "menu-search-organize-filters", accelerator: "Ctrl+Shift+F" },
      { kind: "separator" },
      { kind: "item", id: "search.filter.everything", label: "Everything", l10n: "menu-search-filter-everything", checkable: true },
      { kind: "item", id: "search.filter.audio", label: "Audio", l10n: "quick-filter-audio", checkable: true },
      { kind: "item", id: "search.filter.compressed", label: "Compressed (Archive)", l10n: "menu-search-filter-archive", checkable: true },
      { kind: "item", id: "search.filter.document", label: "Document", l10n: "quick-filter-document", checkable: true },
      { kind: "item", id: "search.filter.executable", label: "Executable", l10n: "quick-filter-executable", checkable: true },
      { kind: "item", id: "search.filter.folder", label: "Folder", l10n: "menu-search-filter-folder", checkable: true },
      { kind: "item", id: "search.filter.picture", label: "Picture", l10n: "quick-filter-image", checkable: true },
      { kind: "item", id: "search.filter.video", label: "Video", l10n: "quick-filter-video", checkable: true },
      { kind: "separator" },
      { kind: "item", id: "search.filter.custom", label: "Custom Filter…", l10n: "menu-search-filter-custom" }
    ]
  },
  {
    label: "Bookmarks",
    l10n: "menu-bookmarks",
    hint: "Contains commands for working with bookmarks.",
    hintL10n: "menu-bookmarks-hint",
    children: [
      { kind: "item", id: "bookmarks.add", label: "Add to Bookmarks", l10n: "menu-bookmarks-add", accelerator: "Ctrl+D" },
      { kind: "item", id: "bookmarks.organize", label: "Organize Bookmarks…", l10n: "menu-bookmarks-organize", accelerator: "Ctrl+Shift+B" },
      { kind: "separator" }
    ]
  },
  {
    label: "Tools",
    l10n: "menu-tools",
    hint: "Contains tools commands.",
    hintL10n: "menu-tools-hint",
    children: [
      { kind: "item", id: "tools.connect_endpoint", label: "Connect to FTP Server…", l10n: "menu-tools-connect" },
      { kind: "item", id: "tools.disconnect_endpoint", label: "Disconnect from FTP Server", l10n: "menu-tools-disconnect" },
      { kind: "item", id: "tools.file_list_editor", label: "File List Editor…", l10n: "menu-tools-file-list-editor" },
      {
        kind: "submenu",
        label: "Index maintenance",
        l10n: "menu-tools-index-maintenance",
        hint: "Index maintenance tools.",
        hintL10n: "menu-tools-index-maintenance-hint",
        children: [
          { kind: "item", id: "tools.verify_index", label: "Verify Index…", l10n: "menu-tools-verify-index" },
          { kind: "item", id: "tools.compact_index", label: "Compact Index…", l10n: "menu-tools-compact-index" },
          { kind: "item", id: "tools.rebuild_index", label: "Force Rebuild Index…", l10n: "menu-tools-rebuild-index" }
        ]
      },
      { kind: "item", id: "tools.custom_extractor_manager", label: "Custom Extractor Manager…", l10n: "menu-tools-custom-extractor", hint: "Manage Wasm-sandboxed custom extractors.", hintL10n: "menu-tools-custom-extractor-hint" },
      { kind: "item", id: "tools.options", label: "Options…", l10n: "menu-tools-options", accelerator: "Ctrl+," }
    ]
  },
  {
    label: "Help",
    l10n: "menu-help",
    hint: "Contains help commands.",
    hintL10n: "menu-help-hint",
    children: [
      { kind: "item", id: "help.help", label: "Freally Help", l10n: "menu-help-help", accelerator: "F1" },
      { kind: "item", id: "help.search_syntax", label: "Search Syntax", l10n: "menu-help-search-syntax" },
      { kind: "item", id: "help.regex_syntax", label: "Regex Syntax", l10n: "menu-help-regex-syntax" },
      { kind: "item", id: "help.audio_modifier_reference", label: "Audio Modifier Reference", l10n: "menu-help-audio-ref" },
      { kind: "item", id: "help.similarity_modifier_reference", label: "Similarity Modifier Reference", l10n: "menu-help-similarity-ref" },
      { kind: "item", id: "help.command_line_options", label: "Command Line Options", l10n: "menu-help-cli-options" },
      { kind: "item", id: "help.website", label: "Freally Website", l10n: "menu-help-website" },
      { kind: "item", id: "help.check_for_updates", label: "Check for Updates…", l10n: "menu-help-check-updates" },
      { kind: "item", id: "help.sponsor", label: "Sponsor / Donate", l10n: "menu-help-sponsor" },
      { kind: "separator" },
      { kind: "item", id: "help.about", label: "About Freally…", l10n: "menu-help-about", accelerator: "Ctrl+F1" },
      { kind: "separator" },
      { kind: "item", id: "help.more_freally_apps", label: "More Freally apps…", l10n: "menu-help-more-apps" }
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
