// Phase 12 (TASK-098) Fluent loader: all 18 locales are eagerly bundled
// at build time via `import.meta.glob`. The source-of-truth .ftl files
// live at the workspace root under `locales/<code>/freally.ftl` and
// stay in lockstep on key set (Standing Rule #4). Switching locale at
// runtime returns a different `FluentBundle` instance; combined with
// Svelte 5 reactive reads in `t.ts`, every `t("key")` call re-runs and
// the UI re-renders.

import { FluentBundle, FluentResource } from "@fluent/bundle";

// Vite resolves this glob at build time. The `?raw` query returns the
// file contents as a string. `eager: true` inlines all 18 modules into
// the main chunk (each is ~6–8 KB so the total is well under 200 KB
// even before gzip — small enough that lazy loading isn't worth the
// flash-of-untranslated-content risk on locale switch).
const LOCALE_FILES = import.meta.glob<string>(
  "../../../../../locales/*/freally.ftl",
  { query: "?raw", import: "default", eager: true }
);

const FTL_SOURCES = new Map<string, string>();
for (const [path, source] of Object.entries(LOCALE_FILES)) {
  const m = path.match(/locales\/([^/]+)\/freally\.ftl$/);
  if (m) FTL_SOURCES.set(m[1], source);
}

// The vendored "Central inside" panel (More Freally apps) ships its own 18
// fcp-*-prefixed catalogs — one .ftl per locale, same 18 codes as ours. They
// are layered into each bundle below so the panel localizes through THIS app's
// t(); the fcp- prefix means they can never collide with freally.ftl keys, and
// they live in separate files so they stay outside the freally.ftl lockstep.
const PANEL_LOCALE_FILES = import.meta.glob<string>(
  "../../../../../vendor/freally-central/ui/src/panel/locales/*.ftl",
  { query: "?raw", import: "default", eager: true }
);

const FCP_SOURCES = new Map<string, string>();
for (const [path, source] of Object.entries(PANEL_LOCALE_FILES)) {
  const m = path.match(/locales\/([^/]+)\.ftl$/);
  if (m) FCP_SOURCES.set(m[1], source);
}

const cache = new Map<string, FluentBundle>();

export function bundleFor(locale: string): FluentBundle {
  if (cache.has(locale)) return cache.get(locale)!;
  // FluentBundle's locale list drives plural / number formatting; we
  // pass the requested locale first and `en` as fallback for both
  // locale-aware functions and (via the resource loader below) any
  // key that's accidentally missing from the per-locale .ftl.
  const bundle = new FluentBundle([locale, "en"]);
  const primary = FTL_SOURCES.get(locale);
  if (primary) {
    bundle.addResource(new FluentResource(primary));
  }
  // Always layer the English resource underneath so any unexpected
  // missing key falls back to English rather than the raw key string.
  if (locale !== "en") {
    const en = FTL_SOURCES.get("en");
    if (en) bundle.addResource(new FluentResource(en), { allowOverrides: false });
  }
  // Same layering for the panel's fcp-* catalogs: this locale first, English
  // underneath. Disjoint keyspace from freally.ftl, so no overrides collide.
  const fcp = FCP_SOURCES.get(locale);
  if (fcp) bundle.addResource(new FluentResource(fcp));
  if (locale !== "en") {
    const fcpEn = FCP_SOURCES.get("en");
    if (fcpEn) bundle.addResource(new FluentResource(fcpEn), { allowOverrides: false });
  }
  cache.set(locale, bundle);
  return bundle;
}

// Test/debug helper: lets unit tests assert lockstep without touching
// the disk (the glob already proved file presence at build time).
export function loadedLocales(): readonly string[] {
  return Array.from(FTL_SOURCES.keys()).sort();
}

// Single source of truth for the 18 ship-locales (PRD §8.26). English is
// pinned first; the remaining 17 sit below in alphabetical order by their
// English language name — a fixed order that does not change with the active
// locale. Each label is the language's own native self-name so a user can pick
// their language even when the UI is in a script they cannot read — see
// LocalePanel and FirstRunWizard.
export const LOCALES = [
  { value: "en", label: "English" },
  { value: "ar", label: "العربية" },
  { value: "zh-CN", label: "简体中文" },
  { value: "nl", label: "Nederlands" },
  { value: "fr", label: "Français" },
  { value: "de", label: "Deutsch" },
  { value: "hi", label: "हिन्दी" },
  { value: "id", label: "Bahasa Indonesia" },
  { value: "it", label: "Italiano" },
  { value: "ja", label: "日本語" },
  { value: "ko", label: "한국어" },
  { value: "pl", label: "Polski" },
  { value: "pt-BR", label: "Português (Brasil)" },
  { value: "ru", label: "Русский" },
  { value: "es", label: "Español" },
  { value: "tr", label: "Türkçe" },
  { value: "uk", label: "Українська" },
  { value: "vi", label: "Tiếng Việt" }
] as const;

export type Locale = (typeof LOCALES)[number]["value"];

export const SUPPORTED_LOCALES: readonly Locale[] = LOCALES.map((l) => l.value);
