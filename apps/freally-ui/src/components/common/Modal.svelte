<script lang="ts">
  // The backdrop-and-panel shell the app's dialogs were re-rolling.
  //
  // Eight of them had hand-written copies of the same four things, and
  // the copies had all inherited the same two faults:
  //
  //   * **Escape did not work.** The handler sat on the backdrop, which
  //     is a `role="presentation"` div with no tabindex, so it never
  //     received a keydown — and the panel stopped propagation anyway.
  //     It lives on `window` here, which is where it actually fires.
  //   * **The panel was not focusable.** `role="dialog"` without a
  //     tabindex is what raised `a11y_interactive_supports_focus` in
  //     every one of them, and it meant opening a dialog left focus
  //     behind it, on whatever was underneath.
  //
  // Metrics stay with the caller: dialogs range from a 360px About box
  // to a 960×720 settings window, and a `size` prop covering that would
  // be a worse abstraction than a `style` string. What is shared here is
  // the behaviour and the surface chrome, not the box.
  //
  // `HitViewer`, `QuickLook` and `RegexBuilder` keep their own shells on
  // purpose — a backdrop that is a sibling rather than a parent,
  // arrow-key navigation routed through `bootstrap.ts`, and a popover
  // with no backdrop at all are not this component wearing a hat.

  import type { Snippet } from "svelte";
  import { chromeOverrides } from "../../lib/util/modal_style";

  interface Props {
    open: boolean;
    onClose: () => void;
    /** `aria-label` for the panel. Use this or `labelledBy`, not both. */
    label?: string;
    /** Id of the element titling the panel, for `aria-labelledby`. */
    labelledBy?: string;
    /**
     * Whether clicking the backdrop closes. On by default; off for
     * dialogs where a stray click would discard typed input.
     */
    dismissOnBackdrop?: boolean;
    /**
     * Whether Escape closes. On by default. The first-run wizard turns
     * it off: it is a gate, not a dialog, and dismissing it would leave
     * the app unconfigured with no way back to it.
     */
    dismissOnEscape?: boolean;
    /**
     * Per-dialog box **metrics** — width, height, padding, overflow, layout.
     *
     * Not chrome: background, border, border-radius, box-shadow and color
     * belong to the shell, and are what make every dialog look like the same
     * app. Setting one here beats the scoped class silently, so it is caught
     * in dev rather than discovered later as a second corner radius.
     */
    style?: string;
    /** `data-testid` on the panel, where an e2e spec targets one. */
    testId?: string;
    children: Snippet;
  }
  let {
    open,
    onClose,
    label,
    labelledBy,
    dismissOnBackdrop = true,
    dismissOnEscape = true,
    style = "",
    testId,
    children
  }: Props = $props();

  let panel = $state<HTMLElement | undefined>();

  if (import.meta.env.DEV) {
    // In an effect, not read once at init: a dialog that recomputes its
    // metrics can reach into chrome on a later render than the first.
    $effect(() => {
      const stolen = chromeOverrides(style);
      if (stolen.length === 0) return;
      console.error(
        "Modal: style sets shell chrome (" +
          stolen.join(", ") +
          "). The `style` prop is for box metrics; chrome belongs to `.panel` " +
          "so every dialog matches. Change it in Modal's own stylesheet if " +
          "the whole app should follow."
      );
    });
  }

  $effect(() => {
    if (!open) return;
    // Hand focus back to whatever had it when this closes; without that
    // it falls to <body> and a keyboard user restarts from the top of
    // the app every time they dismiss a dialog.
    const restoreTo = document.activeElement as HTMLElement | null;
    panel?.focus();
    return () => restoreTo?.focus?.();
  });

  /**
   * Keys pressed inside the panel stop here.
   *
   * `bootstrap.ts` binds the app's shortcuts on `window`, so without
   * this every keystroke typed into a dialog also runs a global command:
   * Ctrl+Z in a text field reverts the last file operation **on disk**
   * instead of undoing the typo, F2 opens Bulk Rename over the open
   * dialog, Ctrl+, opens Settings on top of itself. Each dialog used to
   * carry its own `onkeydown={(e) => e.stopPropagation()}` for exactly
   * this; the shell has to keep doing it.
   *
   * Escape is handled here rather than being allowed through, because
   * the shield would otherwise swallow it — which is the trap the old
   * per-dialog handlers fell into and why Escape never closed anything.
   */
  function onPanelKeydown(e: KeyboardEvent) {
    e.stopPropagation();
    if (dismissOnEscape && e.key === "Escape") onClose();
  }

  /**
   * The same Escape, for when focus is not inside the panel — the panel
   * is focused on open, but nothing traps focus there.
   */
  function onWindowKeydown(e: KeyboardEvent) {
    if (!open || !dismissOnEscape || e.key !== "Escape") return;
    // Stop here so Escape closes the dialog rather than also reaching
    // the search bar's clear-query handler underneath it.
    e.stopPropagation();
    onClose();
  }
</script>

<svelte:window onkeydown={onWindowKeydown} />

{#if open}
  <!-- The keyboard route out of this dialog is Escape, bound on `window`
       above — the backdrop click is the pointer convenience beside it, not
       the only way out, so it needs no key handler of its own. -->
  <!-- svelte-ignore a11y_click_events_have_key_events -->
  <div class="backdrop" role="presentation" onclick={() => dismissOnBackdrop && onClose()}>
    <!-- svelte-ignore a11y_no_noninteractive_element_interactions -->
    <div
      bind:this={panel}
      class="panel"
      role="dialog"
      aria-modal="true"
      aria-label={label}
      aria-labelledby={labelledBy}
      tabindex="-1"
      {style}
      data-testid={testId}
      onclick={(e) => e.stopPropagation()}
      onkeydown={onPanelKeydown}
    >
      {@render children()}
    </div>
  </div>
{/if}

<style>
  .backdrop {
    position: fixed;
    inset: 0;
    background: rgba(0, 0, 0, 0.45);
    display: grid;
    place-items: center;
    z-index: 100;
  }
  .panel {
    background: var(--bg-surface);
    border: 1px solid var(--border);
    border-radius: 10px;
    box-shadow: 0 12px 48px rgba(0, 0, 0, 0.4);
    color: var(--text-primary);
  }
  /* The panel is focused on open so Escape and screen readers land in
     the right place — but it is a container, not a control, so it must
     not draw a focus ring the way a button would. */
  .panel:focus {
    outline: none;
  }
</style>
