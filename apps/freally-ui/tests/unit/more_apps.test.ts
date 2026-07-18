// Embed smoke for "More Freally apps" (Central's FC-50 DoD): the vendored,
// view-only CentralPanel renders through THIS app's Fluent runtime — i.e. the
// submodule's fcp-* catalogs really are layered into our bundles (bundle.ts),
// and the React island mounts real catalog cards via the @freally/central-panel
// alias. Uses createElement (not JSX) so the file stays a .test.ts.

import { describe, it, expect, beforeEach, afterEach, vi } from "vitest";
import { createElement } from "react";
import { render, screen, cleanup, waitFor } from "@testing-library/react";
import { CentralPanel } from "@freally/central-panel";
import { t } from "../../src/lib/i18n/t";
import { bundleFor } from "../../src/lib/i18n/bundle";
import { settingsStore } from "../../src/lib/stores/settings.svelte";

const HOST = { openExternal: () => {} };

describe("More Freally apps — panel i18n wiring", () => {
  beforeEach(() => {
    settingsStore.state.locale = "en";
  });

  it("resolves the panel's fcp-* keys through our own t()", () => {
    expect(t("fcp-coming-soon")).toBe("Coming soon");
    expect(t("fcp-available")).toBe("Available");
    expect(t("fcp-refresh")).not.toBe("fcp-refresh");
  });

  it("layers the fcp-* catalog into every locale bundle", () => {
    for (const code of ["en", "de", "ja", "ar", "pt-BR"]) {
      expect(bundleFor(code).hasMessage("fcp-coming-soon")).toBe(true);
    }
  });
});

describe("More Freally apps — panel renders (view-only)", () => {
  beforeEach(() => {
    settingsStore.state.locale = "en";
    // Offline: hosted catalog + GitHub fetches fail, so the panel falls back to
    // its bundled catalog and hides counts — its honest empty state.
    vi.stubGlobal(
      "fetch",
      vi.fn(() => Promise.reject(new Error("offline")))
    );
  });
  afterEach(() => {
    cleanup();
    vi.unstubAllGlobals();
  });

  it("mounts catalog cards localized via our t(), with no download control", async () => {
    render(createElement(CentralPanel, { t, locale: "en", host: HOST, allowDownloads: false }));

    // Real panel + bundled manifest: a catalog card renders.
    expect(await screen.findByText("Freally Capture")).toBeTruthy();
    // A vendored fcp-* string resolves through OUR t() (the "Coming soon" pill).
    await waitFor(() =>
      expect(screen.getAllByText(t("fcp-coming-soon")).length).toBeGreaterThan(0)
    );
    // View-only: the Download-All control is absent.
    expect(screen.queryByText(t("fcp-install-all"))).toBeNull();
  });
});
