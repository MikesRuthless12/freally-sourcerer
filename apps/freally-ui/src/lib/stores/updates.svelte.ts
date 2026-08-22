// TASK-UP1 — the check-for-updates flow, shared by the launch check and
// the Updates panel's button so the two cannot answer differently.
//
// The throttle timestamp lives in `localStorage` rather than in the
// settings schema. It is per-machine bookkeeping, not a preference: it
// does not belong in an exported settings file, and putting it there
// would mean importing someone else's settings could suppress your update
// check. `localStorage` is also where the settings dialog already keeps
// its own per-machine state (selected panel, scroll positions).

import { t } from "../i18n/t";
import * as ipc from "../ipc/updates";
import type { UpdateCheck } from "../ipc/updates";
import { settingsStore } from "./settings.svelte";

export type { UpdateCheck };

const LAST_CHECK_KEY = "freally.updates.lastCheck";

export function lastCheckedUnixSecs(): number | null {
  try {
    const raw = window.localStorage.getItem(LAST_CHECK_KEY);
    const n = raw ? Number(raw) : NaN;
    return Number.isFinite(n) && n > 0 ? n : null;
  } catch {
    return null;
  }
}

function rememberCheck(secs: number) {
  try {
    window.localStorage.setItem(LAST_CHECK_KEY, String(secs));
  } catch {
    // Private mode or a cleared store. A throttle that forgets is a
    // check that runs more often than it needs to, which is harmless.
  }
}

/** Seconds between automatic checks for the configured cadence, or
 *  `null` when the user has turned automatic checking off. */
function cadenceSeconds(): number | null {
  switch (settingsStore.state.privacy_and_updates.auto_update) {
    case "off":
      return null;
    case "weekly":
      return 7 * 24 * 3600;
    case "monthly":
      return 30 * 24 * 3600;
    default:
      // "default" is once a day. Checking on every launch would hit
      // GitHub's unauthenticated rate limit — 60 requests/hour **per
      // IP**, shared behind corporate NAT, which is exactly the
      // population a silent failure hurts most.
      return 24 * 3600;
  }
}

/** Ask the updater. Throws on transport or signature failure — the
 *  caller decides whether that is worth showing. */
export async function check(): Promise<UpdateCheck> {
  const result = await ipc.check();
  rememberCheck(result.checkedAtUnixSecs);
  return result;
}

/** Ask Rust to show the Yes/No box — naming the version on offer and the
 *  one running — and install if the answer is yes.
 *
 *  Returns whether the install was authorised. Installing replaces the
 *  running application and closes it, so it is never implied by having
 *  asked whether an update exists. */
export async function confirmAndInstall(r: UpdateCheck): Promise<boolean> {
  // Fluent has no literal newline in a single-line value, so the message
  // is authored with `\n` and unescaped here.
  const message = t("updates-confirm-body", {
    version: r.availableVersion,
    current: r.currentVersion
  })
    .split("\\n")
    .join("\n");

  // The confirmation itself lives in Rust — see `updates_install`. The
  // webview must not be the thing that decides whether the running
  // application gets replaced, so the strings travel and the decision
  // does not.
  //
  // Release notes deliberately do **not** travel with them. They are
  // remote text from the update manifest, which the artifact signature
  // does not cover, and a native box captioned with the app's own
  // update prompt is a poor place to render something an outside party
  // wrote. The panel shows them in full, labelled, with working links.
  return await ipc.install({
    title: t("updates-confirm-title"),
    message,
    yesLabel: t("updates-confirm-yes"),
    noLabel: t("updates-confirm-no")
  });
}

/** The launch check.
 *
 *  Four rules, all from the Havoc standard and each one a way this goes
 *  wrong if skipped:
 *
 *  - **A pending crash report wins the dialog slot.** Being asked to
 *    update by the app that just crashed on you, before it acknowledges
 *    the crash, reads as the app ignoring what happened.
 *  - **Honour the cadence, including "off".** A setting that drives
 *    nothing is worse than no setting.
 *  - **Throttle.** GitHub's unauthenticated limit is per-IP.
 *  - **Stay silent on failure.** Offline is the normal state of a laptop
 *    at launch, and a startup error box about the network is noise. The
 *    manual button in Settings is where a failure is worth reporting,
 *    because there somebody asked.
 */
export async function checkOnLaunch(hasPendingCrash: boolean): Promise<void> {
  if (hasPendingCrash) return;
  // Settings first. This runs alongside `bootstrap()`, and until its
  // `hydrate()` lands `auto_update` reads as the fallback `"default"` —
  // so a user who turned updates **off** would still get an outbound
  // request to github.com on launch, which is precisely what the Privacy
  // panel promises does not happen.
  try {
    await settingsStore.hydrate();
  } catch (e) {
    // A settings read that fails leaves the cadence unknowable. Treat
    // that as "do not reach the network", which is the safe reading.
    console.warn("[updates] settings unavailable; skipping the launch check:", e);
    return;
  }
  const cadence = cadenceSeconds();
  if (cadence === null) return;

  const last = lastCheckedUnixSecs();
  const now = Math.floor(Date.now() / 1000);
  if (last !== null && now - last < cadence) return;

  try {
    const result = await check();
    if (result.isNewer) await confirmAndInstall(result);
  } catch (e) {
    console.warn("[updates] launch check failed:", e);
  }
}
