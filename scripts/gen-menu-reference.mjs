#!/usr/bin/env node
// TASK-107 — generate MENU_REFERENCE.md from the menu spec itself.
//
// A hand-written menu reference is stale the first time somebody adds an
// item, and nothing tells you: the doc and the menu are two descriptions
// of the same thing with no link between them. `menu_spec.ts` is already
// the single source of truth that both the in-window menu bar and the
// macOS native menu read, and the accelerators and hover hints live there
// too. So the reference is derived from it, and `docs/MENU_REFERENCE.md`
// is a build artifact rather than a document anyone edits.
//
// Labels come from `locales/en/freally.ftl` where an `l10n` key exists —
// the same lookup `MenuBar.svelte` does at render time — so the reference
// says what the user actually sees, not the English fallback literal.
//
// Usage:  node scripts/gen-menu-reference.mjs
import { readFileSync, writeFileSync } from "node:fs";
import { dirname, join } from "node:path";
import { fileURLToPath } from "node:url";

const repoRoot = join(dirname(fileURLToPath(import.meta.url)), "..");
const specPath = join(repoRoot, "apps/freally-ui/src/lib/commands/menu_spec.ts");
const ftlPath = join(repoRoot, "locales/en/freally.ftl");
const outPath = join(repoRoot, "docs/MENU_REFERENCE.md");

/** The en catalogue as a flat key → value map. Fluent multi-line values are
 *  not used by any menu key, so a line-wise parse is enough and avoids
 *  pulling a Fluent dependency into a docs script. */
const strings = new Map();
for (const line of readFileSync(ftlPath, "utf8").split("\n")) {
  const m = /^([a-zA-Z][\w-]*)\s*=\s*(.*)$/.exec(line);
  if (m) strings.set(m[1], m[2].trim());
}

/** Evaluate the spec's exported tree.
 *
 *  The file is TypeScript with one `import type` (erased at runtime) and a
 *  single exported const. Stripping the types and evaluating it is a good
 *  deal more honest than regexing the source: if the spec stops being a
 *  plain literal this throws, rather than silently emitting a reference to
 *  a menu that no longer exists. */
function loadSpec() {
  const src = readFileSync(specPath, "utf8");
  const marker = "export const MENU_BAR";
  const start = src.indexOf(marker);
  if (start < 0) throw new Error("menu_spec.ts no longer exports MENU_BAR");
  // Cut at the array literal’s own terminator, not at end of file: the
  // exports after it are functions, and dragging them in makes this a
  // syntax error rather than a wrong answer — which is the failure mode
  // to prefer, but only once.
  const rest = src.slice(start);
  const end = rest.indexOf("\n];");
  if (end < 0) throw new Error('MENU_BAR is no longer a top-level array literal');
  let body = rest.slice(0, end + 3).replace(marker, 'const MENU_BAR');
  // Drop the type annotation between the name and the `=`.
  body = body.replace(/^const MENU_BAR\s*:[^=]+=/, "const MENU_BAR =");
  // `satisfies X` / `as const` tails, if they ever appear.
  body = body.replace(/\bsatisfies\s+[A-Za-z_$][\w$.<>[\]]*/g, "");
  return new Function(`${body}\nreturn MENU_BAR;`)();
}

const label = (node) => (node.l10n && strings.get(node.l10n)) || node.label;
const hint = (node) => (node.hintL10n && strings.get(node.hintL10n)) || node.hint || "";

/** `|` inside a table cell ends the cell. Accelerators do not contain one
 *  today; a future "Ctrl+|" binding would silently break the table. */
const cell = (s) => String(s ?? "").replace(/\|/g, "\\|");

const lines = [];
let items = 0;

function renderMenu(menu) {
  lines.push(`## ${label(menu)}`, "");
  const blurb = hint(menu);
  if (blurb) lines.push(blurb, "");
  // No "what it does" column. Only the seven top-level menus carry a
  // `hint`; not one of the 111 items does, so the column would be a
  // hundred em-dashes advertising information this file does not have.
  // That gap is also why the status bar's per-item description (TASK-084)
  // has nothing to show when you hover an item rather than a menu.
  lines.push("| Item | Shortcut |", "| --- | --- |");
  walk(menu.children, "");
  lines.push("");
}

function walk(nodes, prefix) {
  for (const node of nodes) {
    if (node.kind === "separator") continue;
    if (node.kind === "submenu") {
      // Submenu children are flattened with a `Parent → Child` name rather
      // than nested tables. The menus are two deep at most, and a table
      // per submenu buries three items under a heading each.
      walk(node.children, `${prefix}${label(node)} → `);
      continue;
    }
    items++;
    const name = `${prefix}${label(node)}`;
    const kind = node.checkable ? " *(toggle)*" : node.radio ? " *(choice)*" : "";
    lines.push(`| ${cell(name)}${kind} | ${cell(node.accelerator) || "—"} |`);
  }
}

const spec = loadSpec();
for (const menu of spec) renderMenu(menu);

const header = `# Menu reference

Every item in Freally Sourcerer’s menu bar and its keyboard shortcut,
grouped by menu.

> **Generated file — do not edit.** Produced by
> \`scripts/gen-menu-reference.mjs\` from
> \`apps/freally-ui/src/lib/commands/menu_spec.ts\`, which is the single
> source of truth both the in-window menu bar (Windows, Linux) and the
> native macOS menu read. Labels are resolved through
> \`locales/en/freally.ftl\`, so this says what the menu actually shows.
> Regenerate after changing the spec.

Shortcuts are shown as they appear in the menu. On macOS, \`Ctrl\` is
\`Cmd\` — the menu renders the platform's own modifier symbols.

A **(toggle)** item is a checkbox that remembers its state; a **(choice)**
item is one option in a group where picking one clears the others.

`;

writeFileSync(outPath, header + lines.join("\n").trimEnd() + "\n");
console.log(`wrote docs/MENU_REFERENCE.md — ${spec.length} menus, ${items} items`);
