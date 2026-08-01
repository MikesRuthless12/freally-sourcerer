// Path splitting for the UI.
//
// Paths reach the frontend as opaque strings that may use either
// separator — a `.efu` written on Windows loads on Linux and vice
// versa — so every split here handles both.

/** File name component of a path, either separator. */
export function baseName(path: string): string {
  const i = Math.max(path.lastIndexOf("/"), path.lastIndexOf("\\"));
  return i >= 0 ? path.slice(i + 1) : path;
}

/** Extension without the dot, in its original case, or "" when there is
 *  none. A leading dot is not an extension (`.bashrc` has none), and a
 *  dot in a *folder* name doesn't become the file's extension.
 *
 *  Case is preserved so each caller folds the way its Rust counterpart
 *  does — `CustomCommand::applies_to` compares ASCII-insensitively, and
 *  a full `toLowerCase()` here would disagree with it on non-ASCII. */
export function extensionOf(path: string): string {
  const name = baseName(path);
  const dot = name.lastIndexOf(".");
  return dot > 0 ? name.slice(dot + 1) : "";
}

/** Does `needle` match this row?
 *
 *  The one place the app decides what "narrow by this text" means: a
 *  needle containing a separator matches the full path, otherwise the
 *  file name. Both the refinement chips (SRC-M02) and the file-list
 *  search (SRC-M03) go through this so they can't drift apart.
 *
 *  **All three arguments must already be lowercased.** Both callers run
 *  this over every row of a result set, so lowercasing belongs at their
 *  loop boundaries — once per chip and once per row — not in here,
 *  where it would happen once per chip *per* row.
 */
export function matchesLoweredTerm(
  needle: string,
  nameLower: string,
  pathLower: string
): boolean {
  const haystack = needle.includes("/") || needle.includes("\\") ? pathLower : nameLower;
  return haystack.includes(needle);
}
