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

test("Escape closes a dialog", async ({ page }) => {
  // Regression: through Build 3 it did not. Every dialog bound its own
  // Escape handler to the backdrop — a `role="presentation"` div with no
  // tabindex, which cannot receive a keydown — and the panel on top of it
  // stopped propagation anyway. Each spec above opens a dialog and
  // asserts it is visible, so all of them passed while the keyboard route
  // out was dead. The shared <Modal> binds Escape on `window`.
  await bootMain(page);
  await page.keyboard.press("Control+F1");
  await expect(page.getByRole("dialog")).toBeVisible();
  await page.keyboard.press("Escape");
  await expect(page.getByRole("dialog")).toBeHidden();
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

// ---- Build 2 (v0.22.0) ----

// Submenus are click-to-pin: a click holds the submenu open regardless of
// where the pointer then travels, so the child is reachable without racing
// a `mouseleave`. Before that, this spec was `test.fixme` — the submenu
// opened on `mouseenter` and closed on `mouseleave`, so every route toward
// the child detached it mid-click.
test("index health panel opens via Tools › Index maintenance (SRC-M13)", async ({ page }) => {
  await bootMain(page);
  await page.getByRole("button", { name: "Tools" }).click();
  await page.getByRole("menuitem", { name: /Index maintenance/ }).click();
  const child = page.getByRole("menuitem", { name: /Index Health/ });
  await expect(child).toBeVisible();
  await child.click();
  const dialog = page.getByRole("dialog");
  await expect(dialog).toBeVisible();
  // Both watcher states and the advisory that carries a one-click fix.
  await expect(dialog.getByText("System")).toBeVisible();
  await expect(dialog.getByText("Orange WD 4TB")).toBeVisible();
  await expect(dialog.getByRole("button", { name: /Rebuild index/ })).toBeVisible();
  await page.screenshot({ path: SHOT("08-index-health"), fullPage: true });
});

test("bulk rename dialog previews and blocks a collision (SRC-M15)", async ({ page }) => {
  await bootMain(page);
  // Rename acts on the selection, so select a result first.
  await page.getByText("quarterly-report.pdf").first().click();
  await page.keyboard.press("F2");
  const dialog = page.getByRole("dialog");
  await expect(dialog).toBeVisible();
  // The preview table renders all three statuses, and Apply stays
  // disabled while a row is blocking.
  await expect(dialog.getByText("photo-01.jpg").first()).toBeVisible();
  await expect(dialog.getByRole("button", { name: /^Rename$/ })).toBeDisabled();
  await page.screenshot({ path: SHOT("09-bulk-rename"), fullPage: true });
});

test("first-run wizard renders for a fresh install (?wizard=1)", async ({ page }) => {
  await page.goto("/?wizard=1");
  await expect(page.getByRole("dialog")).toBeVisible({ timeout: 15_000 });
  await page.screenshot({ path: SHOT("07-first-run-wizard"), fullPage: true });
});

// ---- Build 3 (v0.23.0) ----

test("regex builder popover previews matches (SRC-M20)", async ({ page }) => {
  await bootMain(page);
  // Reachable from a button in the search bar, so this needs none of the
  // menu-bar hover machinery the index-health spec is blocked on.
  await page.getByTestId("regex-builder-open").click();
  const popover = page.getByRole("dialog", { name: /Regular expression builder/i });
  await expect(popover).toBeVisible();
  await page.getByTestId("regex-pattern").fill("report");
  // The count only renders once the (mocked) engine has answered.
  await expect(popover.getByText(/of .* match/i)).toBeVisible();
  await page.screenshot({ path: SHOT("10-regex-builder"), fullPage: true });
});

test("regex builder surfaces the engine's own compile error (SRC-M20)", async ({ page }) => {
  await bootMain(page);
  await page.getByTestId("regex-builder-open").click();
  await page.getByTestId("regex-pattern").fill("(");
  await expect(page.getByText(/unclosed group/)).toBeVisible();
  // "Use this pattern" must stay disabled while the pattern is invalid.
  await expect(page.getByTestId("regex-use")).toBeDisabled();
});

test("permission health report drills down by volume (SRC-M21)", async ({ page }) => {
  await bootMain(page);
  // The status-bar badge is the discoverable entry point; clicking it is
  // also the assertion that it appears when there is something to report.
  await page.getByTestId("permissions-badge").click();
  const dialog = page.getByTestId("permission-health");
  await expect(dialog).toBeVisible();
  await dialog.getByRole("button", { name: /C:/ }).first().click();
  await expect(dialog.getByText("C:\\System Volume Information")).toBeVisible();
  await page.screenshot({ path: SHOT("11-permission-health"), fullPage: true });
});

test("sidebar lists bookmarks, filters, volumes and recent searches (SRC-M22)", async ({
  page,
}) => {
  await bootMain(page);
  await page.getByRole("button", { name: "View" }).click();
  // `menuitemcheckbox`, not `menuitem`: MenuBar gives checkable entries
  // the checkbox role, so a `menuitem` query silently never matches.
  await page.getByRole("menuitemcheckbox", { name: /Sidebar/ }).click();
  const sidebar = page.getByTestId("sidebar");
  await expect(sidebar).toBeVisible();
  await expect(sidebar.getByText("Bookmarks")).toBeVisible();
  await expect(sidebar.getByText("Big videos")).toBeVisible();
  await page.screenshot({ path: SHOT("12-sidebar"), fullPage: true });
});
