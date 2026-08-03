<script lang="ts">
  import Section from "../controls/Section.svelte";
  import { t } from "../../../lib/i18n/t";
  import { environment, type AppEnvironment } from "../../../lib/ipc/app_env";

  const VENDOR_URL = "https://github.com/MikesRuthless12/freally-sourcerer";
  const VOIDTOOLS_URL = "https://www.voidtools.com/";

  let osVer = $state<string | null>(null);
  // Read from the running process. This used to be a hardcoded constant
  // and had drifted three releases behind what shipped.
  let env = $state<AppEnvironment | null>(null);

  $effect(() => {
    void (async () => {
      try {
        const os = await import("@tauri-apps/plugin-os");
        const v = `${os.platform()} ${os.version()} ${os.arch()}`;
        osVer = v;
      } catch {
        osVer = null;
      }
    })();
  });

  $effect(() => {
    void (async () => {
      try {
        env = await environment();
      } catch {
        env = null;
      }
    })();
  });

  async function openUrl(u: string) {
    try {
      const opener = await import("@tauri-apps/plugin-opener");
      await opener.openUrl(u);
    } catch (e) {
      console.warn("openUrl failed", e);
    }
  }
  async function openNotices() {
    try {
      const opener = await import("@tauri-apps/plugin-opener");
      await opener.openPath("THIRD-PARTY-NOTICES.md");
    } catch (e) {
      console.warn("open notices failed", e);
    }
  }
  async function openVendor() { void openUrl(VENDOR_URL); }
  async function openVoidtools() { void openUrl(VOIDTOOLS_URL); }
</script>

<h1>{t("settings-node-about")}</h1>

<Section title={t("about-section-version")}>
  <p>{t("settings-about-version", { version: env ? `v${env.version}` : "…" })}</p>
  <p class="muted">{osVer ?? "Detecting OS…"}</p>
  {#if env?.portable}
    <p><strong>{t("about-portable-on")}</strong></p>
    <p class="muted">{t("about-portable-hint")}</p>
    <p class="path">{env.data_dir}</p>
  {/if}
</Section>

<Section title={t("about-section-license")}>
  <p><strong>{t("about-license-text")}</strong></p>
  <p class="muted">{t("about-license-spdx", { spdx: "LicenseRef-MikeWeaver-Proprietary-AllRightsReserved" })}</p>
</Section>

<Section title={t("about-section-credits")}>
  <p>
    <em>{t("about-credits-inspired")}</em>
    <button type="button" class="link" onclick={openVoidtools}>{t("about-credits-voidtools")}</button>
  </p>
  <button type="button" class="link" onclick={openVendor}>{t("about-credits-repo")}</button>
  ·
  <button type="button" class="link" onclick={openNotices}>{t("settings-about-notices")}</button>
  ·
  <button type="button" class="link" onclick={() => openUrl("https://www.voidtools.com/")}>{t("about-credits-voidtools")}</button>
</Section>

<style>
  h1 { margin: 0 0 4px; font-size: 18px; color: var(--text-primary); }
  p { color: var(--text-primary); font-size: 13px; line-height: 1.5; margin: 4px 0; }
  p.muted { color: var(--text-secondary); font-size: 12px; }
  p.path { font-family: var(--font-mono, monospace); font-size: 12px; word-break: break-all; }
  button.link { background: none; border: 0; color: var(--accent-cyan); cursor: pointer; padding: 0; font: inherit; text-decoration: underline; }
</style>
