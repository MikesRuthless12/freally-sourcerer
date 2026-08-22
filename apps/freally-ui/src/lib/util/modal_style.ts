// `Modal`'s `style` prop is applied inline, so it beats the shell's scoped
// `.panel` class on every property it names. Metrics genuinely belong to the
// caller — dialogs run from a 360px About box to a 960×720 settings window —
// but surface chrome does not, and nothing stopped a caller from reaching
// past the metrics into it.
//
// Four had: two set `border-radius: 8px` against the shell's 10px and two
// set their own `box-shadow`, so the app had two dialog corner radii and
// three shadows with nothing to notice it. The point of the shell is that
// this is uniform, so the prop is constrained to metrics rather than growing
// named props for the chrome — a named `radius` prop would just make the
// divergence official.

/** The properties `.panel` owns. Set by the shell, not by a caller. */
const CHROME = ["background", "border", "border-radius", "box-shadow", "color"];

/**
 * Chrome properties present in an inline style string, in source order.
 * Empty when the string is metrics-only.
 */
export function chromeOverrides(style: string): string[] {
  const found: string[] = [];
  for (const decl of style.split(";")) {
    const prop = decl.split(":")[0]?.trim().toLowerCase();
    if (!prop) continue;
    // Longhands count: `border-color` reaches the same surface as `border`.
    const isChrome = CHROME.some((c) => prop === c || prop.startsWith(`${c}-`));
    if (isChrome && !found.includes(prop)) found.push(prop);
  }
  return found;
}
