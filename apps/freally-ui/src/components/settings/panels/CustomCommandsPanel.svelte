<script lang="ts">
  // Settings → General → Custom Commands (SRC-M06).
  //
  // Each row edits one command in place; every change writes straight
  // through to the settings store, so there is no separate Save.
  import Section from "../controls/Section.svelte";
  import TextInput from "../controls/TextInput.svelte";
  import {
    customCommandsStore,
    MAX_CUSTOM_COMMANDS
  } from "../../../lib/stores/custom_commands.svelte";
  import { settingsDialog } from "../../../lib/stores/settings_dialog.svelte";
  import { t } from "../../../lib/i18n/t";

  const commands = $derived(customCommandsStore.all);

  /** Every edit funnels through the store and then marks this panel
   *  dirty, so the Apply/Revert bar tracks it like every other panel. */
  async function edit(change: Promise<unknown>) {
    await change;
    settingsDialog.markDirty("general.custom_commands");
  }

  const add = () => edit(customCommandsStore.add());
  const update = (id: string, patch: Parameters<typeof customCommandsStore.update>[1]) =>
    edit(customCommandsStore.update(id, patch));
  const remove = (id: string) => edit(customCommandsStore.remove(id));

  /** Arguments are stored as a list but edited as one line — splitting
   *  on whitespace outside quotes keeps `--flag "two words"` intact. */
  function parseArgs(line: string): string[] {
    const out: string[] = [];
    let current = "";
    let quote: string | null = null;
    for (const c of line) {
      if (quote) {
        if (c === quote) quote = null;
        else current += c;
      } else if (c === '"' || c === "'") {
        quote = c;
      } else if (/\s/.test(c)) {
        if (current) out.push(current);
        current = "";
      } else {
        current += c;
      }
    }
    if (current) out.push(current);
    return out;
  }

  function joinArgs(args: string[]): string {
    return args.map((a) => (/\s/.test(a) ? `"${a}"` : a)).join(" ");
  }

  function parseExtensions(line: string): string[] {
    return line
      .split(/[,;\s]+/)
      .map((s) => s.replace(/^\./, "").toLowerCase())
      .filter((s) => s.length > 0);
  }
</script>

<h1>{t("settings-node-custom-commands")}</h1>

<Section title={t("settings-node-custom-commands")}>
  <p class="hint">{t("settings-custom-commands-hint")}</p>

  {#if commands.length === 0}
    <p class="empty">{t("settings-custom-commands-empty")}</p>
  {/if}

  {#each commands as c (c.id)}
    <fieldset class="cmd">
      <legend>{c.name || t("settings-custom-commands-unnamed")}</legend>
      <TextInput
        label={t("settings-custom-commands-name")}
        value={c.name}
        onChange={(v: string) => update(c.id, { name: v })}
      />
      <TextInput
        label={t("settings-custom-commands-program")}
        value={c.program}
        onChange={(v: string) => update(c.id, { program: v })}
      />
      <TextInput
        label={t("settings-custom-commands-args")}
        value={joinArgs(c.args)}
        onChange={(v: string) => update(c.id, { args: parseArgs(v) })}
      />
      <TextInput
        label={t("settings-custom-commands-extensions")}
        value={c.extensions.join(", ")}
        onChange={(v: string) => update(c.id, { extensions: parseExtensions(v) })}
      />
      <button type="button" class="remove" onclick={() => remove(c.id)}>
        {t("settings-vol-remove")}
      </button>
    </fieldset>
  {/each}

  <div class="actions">
    <button type="button" onclick={add} disabled={commands.length >= MAX_CUSTOM_COMMANDS}>
      {t("settings-custom-commands-add")}
    </button>
  </div>
</Section>

<style>
  .hint,
  .empty {
    color: var(--text-secondary);
    font-size: 12px;
    margin: 0 0 12px;
  }
  .cmd {
    border: 1px solid var(--border);
    border-radius: 6px;
    padding: 10px 12px;
    margin-bottom: 12px;
  }
  legend {
    font-size: 12px;
    color: var(--text-secondary);
    padding: 0 4px;
  }
  .remove {
    margin-top: 8px;
  }
  .actions {
    margin-top: 8px;
  }
</style>
