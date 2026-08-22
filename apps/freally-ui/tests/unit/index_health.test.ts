// The Index Health panel starts its poller from a Svelte `$effect`.
//
// `refresh()` used to derive `loading` from `health` — `this.loading =
// this.health === null` — which made it *read* reactive state. That single
// read enrolled the dialog's effect in this store's own state: every `health`
// write re-ran the effect, whose cleanup calls `stop()` and whose body calls
// `start()` again. The panel then polled as fast as the RPC answered rather
// than every POLL_MS, pegging the main thread. It is what kept the
// `08-index-health` e2e spec on `test.fixme` for two builds.
//
// The fix is that `refresh()` reads no reactive state at all (`#everLoaded` is
// a plain private field). That property is invisible at the line that would
// break it, so what is pinned here is the observable consequence: one
// `start()` produces exactly one in-flight refresh, and the next read is a
// full POLL_MS later rather than one microtask later.

import { describe, it, expect, beforeEach, afterEach, vi } from "vitest";
import { setIpcMock } from "../../src/lib/ipc/client";
import { indexHealthStore } from "../../src/lib/stores/index_health.svelte";

let calls: string[] = [];

beforeEach(() => {
  vi.useFakeTimers();
  calls = [];
  setIpcMock(<T,>(cmd: string) => {
    calls.push(cmd);
    return Promise.resolve({ volumes: [], advisories: [] } as T);
  });
});

afterEach(() => {
  indexHealthStore.stop();
  setIpcMock(null);
  vi.useRealTimers();
});

describe("indexHealthStore polling", () => {
  it("polls once per tick rather than once per answer", async () => {
    indexHealthStore.start();
    // Draining every pending microtask is what the feedback loop fed on: each
    // answer restarted the poller, so the count climbed without the clock
    // moving at all.
    await vi.advanceTimersByTimeAsync(0);
    expect(calls).toEqual(["index_health"]);
    await Promise.resolve();
    await Promise.resolve();
    expect(calls).toEqual(["index_health"]);

    await vi.advanceTimersByTimeAsync(2000);
    expect(calls).toEqual(["index_health", "index_health"]);
  });

  it("is idempotent, so a second start does not stack a second poller", async () => {
    indexHealthStore.start();
    indexHealthStore.start();
    await vi.advanceTimersByTimeAsync(0);
    expect(calls).toEqual(["index_health"]);
  });

  it("stops polling when the panel closes", async () => {
    indexHealthStore.start();
    await vi.advanceTimersByTimeAsync(0);
    indexHealthStore.stop();
    await vi.advanceTimersByTimeAsync(10_000);
    expect(calls).toEqual(["index_health"]);
  });

  it("reports loading only until the first answer lands", async () => {
    // `loading` is what used to be derived from `health`. It has to keep
    // behaving the same way now that it is not.
    indexHealthStore.start();
    await vi.advanceTimersByTimeAsync(0);
    expect(indexHealthStore.loading).toBe(false);
    await vi.advanceTimersByTimeAsync(2000);
    expect(indexHealthStore.loading).toBe(false);
  });
});
