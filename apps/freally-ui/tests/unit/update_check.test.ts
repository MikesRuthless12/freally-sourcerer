// TASK-UP1 — the launch check's four rules.
//
// Each one is a way this goes wrong that nobody would notice: an update
// prompt that talks over a crash report, a cadence setting that drives
// nothing, a check on every launch that burns GitHub's per-IP rate limit,
// and a startup error box every time a laptop wakes up somewhere without
// wifi. None of them fail loudly, so they are pinned here.
import { beforeEach, describe, expect, it } from "vitest";
import { setIpcMock } from "../../src/lib/ipc/client";
import { checkOnLaunch, lastCheckedUnixSecs } from "../../src/lib/stores/updates.svelte";

const NOW = () => Math.floor(Date.now() / 1000);

interface Call {
  cmd: string;
  args?: Record<string, unknown>;
}

type Cadence = "default" | "weekly" | "monthly" | "off";
let cadence: Cadence = "default";

function setCadence(v: Cadence) {
  cadence = v;
}

function mockIpc(overrides: Record<string, unknown> = {}) {
  const calls: Call[] = [];
  setIpcMock(async <T,>(cmd: string, args?: Record<string, unknown>): Promise<T> => {
    calls.push({ cmd, args });
    if (cmd in overrides) {
      const v = overrides[cmd];
      if (v instanceof Error) throw v;
      return v as T;
    }
    // The cadence travels through the real path: `checkOnLaunch` hydrates
    // settings before reading it, precisely so a user who turned updates
    // off is not overruled by the fallback while hydration is in flight.
    if (cmd === "settings_get") {
      return { privacy_and_updates: { auto_update: cadence, pre_release_channel: false } } as T;
    }
    if (cmd === "updates_install") return false as T;
    if (cmd === "updates_check") {
      return {
        currentVersion: "0.23.2",
        availableVersion: "",
        notes: "",
        isNewer: false,
        checkedAtUnixSecs: NOW()
      } as T;
    }
    return undefined as T;
  });
  return calls;
}

/** Just the command names, and without the settings read every launch
 *  check makes — it is machinery, not the thing under test. */
const cmds = (calls: Call[]) =>
  calls.map((c) => c.cmd).filter((c) => c !== "settings_get");

/** An in-memory `localStorage`.
 *
 *  jsdom does not supply one under this vitest config — `tests/setup.ts`
 *  guards for exactly that. The store treats a missing store as "never
 *  checked" and carries on, which is right in production and useless
 *  here: without somewhere to record a timestamp the throttle can never
 *  be observed to fire. */
function installStorage() {
  const map = new Map<string, string>();
  Object.defineProperty(window, "localStorage", {
    configurable: true,
    value: {
      getItem: (k: string) => map.get(k) ?? null,
      setItem: (k: string, v: string) => void map.set(k, v),
      removeItem: (k: string) => void map.delete(k),
      clear: () => map.clear()
    }
  });
}

beforeEach(() => {
  installStorage();
  setCadence("default");
});

describe("checkOnLaunch", () => {
  it("stands down when a crash report is claiming the dialog slot", async () => {
    const calls = mockIpc();
    await checkOnLaunch(true);
    expect(cmds(calls)).toEqual([]);
  });

  it("does not check at all when the cadence is off", async () => {
    setCadence("off");
    const calls = mockIpc();
    await checkOnLaunch(false);
    expect(cmds(calls)).toEqual([]);
  });

  it("checks on a first launch and records when it did", async () => {
    const calls = mockIpc();
    await checkOnLaunch(false);
    expect(cmds(calls)).toEqual(["updates_check"]);
    expect(lastCheckedUnixSecs()).toBeGreaterThan(0);
  });

  it("throttles a second launch inside the cadence window", async () => {
    const calls = mockIpc();
    await checkOnLaunch(false); // records a timestamp
    expect(cmds(calls)).toEqual(["updates_check"]);
    calls.length = 0;
    await checkOnLaunch(false);
    expect(cmds(calls)).toEqual([]);
  });

  it("checks again once the cadence window has passed", async () => {
    // Two days ago, against the default cadence of one day.
    window.localStorage.setItem("freally.updates.lastCheck", String(NOW() - 2 * 24 * 3600));
    const calls = mockIpc();
    await checkOnLaunch(false);
    expect(cmds(calls)).toEqual(["updates_check"]);
  });

  it("respects a weekly cadence that a daily one would have re-checked", async () => {
    setCadence("weekly");
    window.localStorage.setItem("freally.updates.lastCheck", String(NOW() - 2 * 24 * 3600));
    const calls = mockIpc();
    await checkOnLaunch(false);
    expect(cmds(calls)).toEqual([]);
  });

  it("stays silent when the check fails", async () => {
    // Offline is the normal state of a laptop at launch. A startup error
    // box about the network is noise; the manual button in Settings is
    // where a failure is worth reporting, because there somebody asked.
    const calls = mockIpc({ updates_check: new Error("network unreachable") });
    await expect(checkOnLaunch(false)).resolves.toBeUndefined();
    expect(cmds(calls)).toEqual(["updates_check"]);
  });

  it("does not reach the installer when there is nothing newer", async () => {
    const calls = mockIpc();
    await checkOnLaunch(false);
    expect(cmds(calls)).toEqual(["updates_check"]);
  });

  it("hands the confirm strings to Rust, naming both versions", async () => {
    // The box is shown by `updates_install` itself, so that a compromised
    // webview cannot skip it. What the frontend still owns is the wording,
    // because that is the part that has to be localised — and it has to
    // name both versions, since "an update is available" without saying
    // which is not something anyone can act on.
    const calls = mockIpc({
      updates_check: {
        currentVersion: "0.23.2",
        availableVersion: "0.24.0",
        notes: "Full Changelog: https://example.com/c",
        isNewer: true,
        checkedAtUnixSecs: NOW()
      }
    });
    await checkOnLaunch(false);
    const install = calls.find((c) => c.cmd === "updates_install");
    expect(install, "the installer command was never reached").toBeTruthy();
    const message = String(install!.args?.message ?? "");
    expect(message).toContain("0.24.0");
    expect(message).toContain("0.23.2");
    expect(install!.args?.yesLabel).toBeTruthy();
    expect(install!.args?.noLabel).toBeTruthy();
  });

  it("keeps the remote release notes out of the native box", async () => {
    // The notes come off the network and the artifact signature does not
    // cover them. A native dialog captioned with the app's own update
    // prompt is a poor place to render text an outside party wrote; the
    // panel shows them instead.
    const calls = mockIpc({
      updates_check: {
        currentVersion: "0.23.2",
        availableVersion: "0.24.0",
        notes: "CLICK HERE TO CLAIM YOUR PRIZE https://evil.example",
        isNewer: true,
        checkedAtUnixSecs: NOW()
      }
    });
    await checkOnLaunch(false);
    const install = calls.find((c) => c.cmd === "updates_install");
    const message = String(install!.args?.message ?? "");
    expect(message).not.toContain("CLAIM YOUR PRIZE");
    expect(message).not.toContain("evil.example");
  });
});
