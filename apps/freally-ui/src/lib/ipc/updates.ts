// TASK-UP1 — the updater's IPC surface.
//
// Through `call` rather than raw `invoke`, like every other command
// family here, so `setIpcMock` can stub it in a unit test — which is what
// makes `checkOnLaunch`'s four rules (crash defers, cadence, throttle,
// silent on failure) testable at all. The DTO mirrors
// `apps/freally-ui/src-tauri/src/updates.rs`.

import { call } from "./client";

export interface UpdateCheck {
  /** The version running right now. */
  currentVersion: string;
  /** The advertised version when it is newer; empty when current. */
  availableVersion: string;
  /** The release body. Markdown — rendered as text plus links, never as
   *  HTML. See `lib/util/release_notes.ts`. */
  notes: string;
  isNewer: boolean;
  /** Unix seconds at which the check completed. */
  checkedAtUnixSecs: number;
}

/** Ask the endpoint what it has. Never downloads. */
export function check(): Promise<UpdateCheck> {
  return call<UpdateCheck>("updates_check");
}

/** The strings the native confirm box shows.
 *
 *  They travel because the box is shown by **Rust**, not here — the
 *  webview must not be the layer that decides whether the running
 *  application gets replaced. It can reword the prompt; it cannot skip
 *  it. Localisation is why the text comes from this side at all. */
export interface InstallPrompt {
  title: string;
  message: string;
  yesLabel: string;
  noLabel: string;
}

/** Show the confirm box and, if the user agrees, download, install and
 *  close the app.
 *
 *  Resolves `false` when the user declined. It never resolves `true`:
 *  the app is gone by then. */
export function install(prompt: InstallPrompt): Promise<boolean> {
  return call<boolean>("updates_install", { ...prompt });
}
