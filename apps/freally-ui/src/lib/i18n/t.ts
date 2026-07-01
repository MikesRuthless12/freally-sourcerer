// `t(key, vars?)` — reactive on locale store. Falls back to `en` then to
// the key itself when missing.

import { bundleFor } from "./bundle";
import { settingsStore } from "../stores/settings.svelte";

export function t(key: string, vars?: Record<string, string | number>): string {
  const locale = settingsStore.state.locale || "en";
  const bundle = bundleFor(locale);
  const msg = bundle.getMessage(key);
  if (msg && msg.value) {
    const errs: Error[] = [];
    const out = bundle.formatPattern(msg.value, vars, errs);
    if (errs.length === 0) return out;
  }
  // Fallback to en.
  if (locale !== "en") {
    const enBundle = bundleFor("en");
    const enMsg = enBundle.getMessage(key);
    if (enMsg && enMsg.value) {
      const errs: Error[] = [];
      const out = enBundle.formatPattern(enMsg.value, vars, errs);
      if (errs.length === 0) return out;
    }
  }
  return key;
}
