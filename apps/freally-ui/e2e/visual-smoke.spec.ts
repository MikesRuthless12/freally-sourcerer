// Visual smoke: boot the REAL built UI with the mocked IPC bridge
// (tauri-mock.js) and screenshot each feature panel. Assertions are
// deliberately shallow — "did it render?" — the point is the gallery in
// e2e/screenshots/. Backend behavior is covered by cargo test + tests/smoke.
//
// Run:  pnpm build && pnpm test:e2e
import { test, expect, type Page } from "@playwright/test";

// Paths are cwd-relative — run Playwright from apps/freally-ui (pnpm test:e2e).
const MOCK = "e2e/tauri-mock.js";
const SHOT = (name: string) => `e2e/screenshots/${name}.png`;

test.beforeEach(async ({ page }) => {
  await page.addInitScript({ path: MOCK });
});

async function bootMain(page: Page) {
  await page.goto("/");
  await expect(page.getByTestId("search-input")).toBeVisible();
  // The mock streams canned batches shortly after the initial "Everything"
  // query; wait for the first filename hit so shots aren't empty.
  await expect(page.getByText("quarterly-report.pdf").first()).toBeVisible({
    timeout: 15_000,
  });
}

test("main window: menu bar, search, filters, results, status bar", async ({ page }) => {
  await bootMain(page);
  await page.screenshot({ path: SHOT("01-main-results"), fullPage: true });
});

test("typed query re-runs the search", async ({ page }) => {
  await bootMain(page);
  await page.getByTestId("search-input").fill("report lufs:>-14");
  await expect(page.getByText("quarterly-report.pdf").first()).toBeVisible({
    timeout: 15_000,
  });
  await page.screenshot({ path: SHOT("02-search-query"), fullPage: true });
});

test("settings dialog opens (Ctrl+,)", async ({ page }) => {
  await bootMain(page);
  await page.keyboard.press("Control+,");
  await expect(page.getByRole("dialog")).toBeVisible();
  await page.screenshot({ path: SHOT("03-settings-dialog"), fullPage: true });
});

test("about dialog opens (Ctrl+F1)", async ({ page }) => {
  await bootMain(page);
  await page.keyboard.press("Control+F1");
  await expect(page.getByRole("dialog")).toBeVisible();
  await page.screenshot({ path: SHOT("04-about-dialog"), fullPage: true });
});

test("organize bookmarks dialog opens (Ctrl+Shift+B)", async ({ page }) => {
  await bootMain(page);
  await page.keyboard.press("Control+Shift+B");
  await expect(page.getByRole("dialog")).toBeVisible();
  // Canned bookmarks from the mock should be listed.
  await expect(page.getByText("Big videos")).toBeVisible();
  await page.screenshot({ path: SHOT("05-organize-bookmarks"), fullPage: true });
});

test("connect-endpoint dialog opens via Tools menu", async ({ page }) => {
  await bootMain(page);
  await page.getByRole("button", { name: "Tools" }).click();
  await page.getByRole("menuitem", { name: /Connect to FTP Server/ }).click();
  await expect(page.getByRole("dialog")).toBeVisible();
  await page.screenshot({ path: SHOT("06-connect-endpoint"), fullPage: true });
});

test("first-run wizard renders for a fresh install (?wizard=1)", async ({ page }) => {
  await page.goto("/?wizard=1");
  await expect(page.getByRole("dialog")).toBeVisible({ timeout: 15_000 });
  await page.screenshot({ path: SHOT("07-first-run-wizard"), fullPage: true });
});
