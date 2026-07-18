// Compile-time exhaustive list of every menu command Freally defines.
// PRD §8.28 enumerates them all. Extending the menu = extending this union
// = a TS error in any registry/handler that hasn't caught up.

export const COMMAND_IDS = [
  // File
  "file.new_window",
  "file.open_file_list",
  "file.close_file_list",
  "file.close",
  "file.export_results",
  "file.export_index_bundle",
  "file.exit",

  // Edit
  "edit.cut",
  "edit.copy",
  "edit.paste",
  "edit.copy_to_folder",
  "edit.move_to_folder",
  "edit.select_all",
  "edit.invert_selection",
  "edit.advanced.copy_full_name",
  "edit.advanced.copy_path",
  "edit.advanced.copy_filename",
  "edit.advanced.copy_as_json",
  "edit.advanced.copy_with_metadata",
  "edit.advanced.copy_as_bundle_ref",

  // View
  "view.filters",
  "view.preview",
  "view.status_bar",
  "view.thumbs.xl",
  "view.thumbs.l",
  "view.thumbs.m",
  "view.details",
  "view.window_size.small",
  "view.window_size.medium",
  "view.window_size.large",
  "view.window_size.auto",
  "view.zoom.in",
  "view.zoom.out",
  "view.zoom.reset",
  "view.sort.name",
  "view.sort.path",
  "view.sort.size",
  "view.sort.ext",
  "view.sort.type",
  "view.sort.modified",
  "view.sort.created",
  "view.sort.accessed",
  "view.sort.attributes",
  "view.sort.recently_changed",
  "view.sort.run_count",
  "view.sort.run_date",
  "view.sort.file_list_filename",
  "view.sort.lufs",
  "view.sort.length",
  "view.sort.similarity",
  "view.sort.ascending",
  "view.sort.descending",
  "view.go_to",
  "view.refresh",
  "view.theme.system",
  "view.theme.light",
  "view.theme.dark",
  "view.lens.filename",
  "view.lens.content",
  "view.lens.audio",
  "view.lens.similarity",
  "view.on_top.never",
  "view.on_top.always",
  "view.on_top.while_searching",

  // Search
  "search.match_case",
  "search.match_whole_word",
  "search.match_path",
  "search.match_diacritics",
  "search.enable_regex",
  "search.advanced",
  "search.add_to_filters",
  "search.organize_filters",
  "search.filter.everything",
  "search.filter.audio",
  "search.filter.compressed",
  "search.filter.document",
  "search.filter.executable",
  "search.filter.folder",
  "search.filter.picture",
  "search.filter.video",
  "search.filter.custom",

  // Bookmarks
  "bookmarks.add",
  "bookmarks.organize",

  // Tools
  "tools.connect_endpoint",
  "tools.disconnect_endpoint",
  "tools.file_list_editor",
  "tools.custom_extractor_manager",
  "tools.verify_index",
  "tools.compact_index",
  "tools.rebuild_index",
  "tools.options",

  // Help
  "help.help",
  "help.search_syntax",
  "help.regex_syntax",
  "help.audio_modifier_reference",
  "help.similarity_modifier_reference",
  "help.command_line_options",
  "help.website",
  "help.check_for_updates",
  "help.sponsor",
  "help.about",
  "help.more_freally_apps"
] as const;

export type CommandId = (typeof COMMAND_IDS)[number];

export function isCommandId(s: string): s is CommandId {
  return (COMMAND_IDS as readonly string[]).includes(s);
}
