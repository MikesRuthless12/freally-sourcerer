<script lang="ts">
  import Section from "../controls/Section.svelte";

  const VERSION = "0.19.84";
  const VENDOR_URL = "https://github.com/MikesRuthless12/Sourcerer";
  const VOIDTOOLS_URL = "https://www.voidtools.com/";

  let osVer = $state<string | null>(null);
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

<h1>About</h1>

<Section title="Version (+)">
  <p>Sourcerer <strong>v{VERSION}</strong></p>
  <p class="muted">{osVer ?? "Detecting OS…"}</p>
</Section>

<Section title="License (+)">
  <p><strong>Mike Weaver — All Rights Reserved.</strong> This is proprietary software.</p>
  <p class="muted">SPDX: <code>LicenseRef-MikeWeaver-Proprietary-AllRightsReserved</code></p>
</Section>

<Section title="Credits (+)">
  <p>
    <em>Inspired by Everything by voidtools.</em>
    <button type="button" class="link" onclick={openVoidtools}>voidtools.com</button>
  </p>
  <button type="button" class="link" onclick={openVendor}>Project repository</button>
  ·
  <button type="button" class="link" onclick={openNotices}>Open-source notices</button>
  ·
  <button type="button" class="link" onclick={() => openUrl("https://www.voidtools.com/")}>voidtools.com</button>
</Section>

<style>
  h1 { margin: 0 0 4px; font-size: 18px; color: var(--text-primary); }
  p { color: var(--text-primary); font-size: 13px; line-height: 1.5; margin: 4px 0; }
  p.muted { color: var(--text-secondary); font-size: 12px; }
  code { font-family: var(--font-mono); }
  button.link { background: none; border: 0; color: var(--accent-cyan); cursor: pointer; padding: 0; font: inherit; text-decoration: underline; }
</style>
