<script lang="ts">
  import Modal from "../common/Modal.svelte";
  import { t } from "../../lib/i18n/t";

  interface Props {
    open: boolean;
    onClose: () => void;
  }
  let { open, onClose }: Props = $props();
</script>

<Modal
  {open}
  {onClose}
  label={t("menu-help-about")}
  style="width: 360px; padding: 24px; border-radius: 10px; text-align: center;"
>
  <h1>{t("app-name")}</h1>
  <p class="version">{t("about-version", { version: "0.19.84" })}</p>
  <p>{t("tagline")}</p>
  <p class="meta">{t("about-copyright")}</p>
  <footer>
    <button type="button" class="primary" onclick={onClose}>{t("about-close")}</button>
  </footer>
</Modal>

<style>
  h1 {
    margin: 0;
    color: var(--accent-cyan);
    font-size: 28px;
    letter-spacing: -0.02em;
  }
  .version {
    color: var(--text-secondary);
    font-size: 12px;
    margin: 4px 0 16px;
    letter-spacing: 0.06em;
    text-transform: uppercase;
  }
  .meta {
    color: var(--text-secondary);
    font-size: 11px;
    margin-top: 16px;
  }
  footer {
    margin-top: 16px;
    display: flex;
    justify-content: center;
  }
  footer button {
    padding: 6px 18px;
    background: var(--accent-cyan);
    border: 0;
    border-radius: 4px;
    color: var(--bg-canvas);
    cursor: pointer;
    font-size: 13px;
  }
</style>
