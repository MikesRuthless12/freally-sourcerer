import { describe, it, expect, beforeEach } from "vitest";
import { themeStore } from "../../src/lib/stores/theme.svelte";

describe("themeStore", () => {
  beforeEach(() => {
    themeStore.set("system");
  });

  it("starts in a valid state", () => {
    expect(["system", "light", "dark"]).toContain(themeStore.choice);
  });

  it("set('dark') applies data-theme attribute", () => {
    themeStore.set("dark");
    expect(themeStore.choice).toBe("dark");
    expect(document.documentElement.getAttribute("data-theme")).toBe("dark");
  });

  it("set('system') removes data-theme attribute", () => {
    themeStore.set("dark");
    themeStore.set("system");
    expect(themeStore.choice).toBe("system");
    expect(document.documentElement.getAttribute("data-theme")).toBeNull();
  });

  it("cycle() walks system → light → dark → system", () => {
    themeStore.set("system");
    themeStore.cycle();
    expect(themeStore.choice).toBe("light");
    themeStore.cycle();
    expect(themeStore.choice).toBe("dark");
    themeStore.cycle();
    expect(themeStore.choice).toBe("system");
  });
});
