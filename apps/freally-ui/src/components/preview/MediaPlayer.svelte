<script lang="ts">
  // SRC-M18 — inline audio/video playback in the preview pane.
  //
  // The preview pane renders static previews; it never played anything.
  // Auditioning a `lufs:<-14` result meant leaving Freally, which is
  // exactly the workflow the audio lens exists to keep you inside.

  import * as files from "../../lib/ipc/files";
  import * as media from "../../lib/ipc/media";
  import type { Waveform } from "../../lib/ipc/media";
  import { formatBytes } from "../../lib/util/format";
  import { t } from "../../lib/i18n/t";

  interface Props {
    path: string;
    name: string;
    kind: "audio" | "video";
  }
  let { path, name, kind }: Props = $props();

  let src = $state<string | null>(null);
  let waveform = $state<Waveform | null>(null);
  let error = $state<string | null>(null);
  // `bind:paused` / `bind:currentTime` / `bind:duration` / `bind:volume`
  // keep these in step with the element in both directions, so the
  // transport reads and writes plain variables and there are no
  // state-syncing event handlers to keep matched between the <audio>
  // and <video> branches.
  let paused = $state(true);
  let loop = $state(false);
  let volume = $state(1);
  let position = $state(0);
  let duration = $state(0);

  $effect(() => {
    const target = path;
    let cancelled = false;
    let objectUrl: string | null = null;

    error = null;
    src = null;
    waveform = null;
    paused = true;
    position = 0;
    duration = 0;

    void (async () => {
      // Started together, not in sequence: the byte read and the
      // waveform decode are independent full passes over the file, and
      // serializing them doubles time-to-first-frame for audio.
      const bytesPromise = media.bytes(target);
      // A waveform is a nicety — failing to draw one must not stop the
      // file playing, so its rejection is swallowed here rather than
      // taken as a load failure below.
      const wavePromise =
        kind === "audio" ? media.waveform(target).catch(() => null) : Promise.resolve(null);

      try {
        const bytes = await bytesPromise;
        if (cancelled) return;
        objectUrl = URL.createObjectURL(new Blob([bytes]));
        src = objectUrl;
      } catch (e) {
        if (!cancelled) error = String(e);
        // Still await the sibling so a rejection cannot surface as an
        // unhandled rejection after this function returns.
        await wavePromise;
        return;
      }
      const w = await wavePromise;
      if (!cancelled && w) waveform = w;
    })();

    return () => {
      cancelled = true;
      // Revoking is not optional bookkeeping: without it every
      // selection change leaks the whole decoded file, and this pane
      // changes selection on every arrow key.
      if (objectUrl) URL.revokeObjectURL(objectUrl);
    };
  });

  function seekFromWaveform(ev: MouseEvent) {
    if (!duration) return;
    const box = (ev.currentTarget as HTMLElement).getBoundingClientRect();
    const ratio = Math.min(1, Math.max(0, (ev.clientX - box.left) / box.width));
    position = ratio * duration;
  }

  function clock(seconds: number): string {
    if (!Number.isFinite(seconds) || seconds < 0) return "0:00";
    const m = Math.floor(seconds / 60);
    const s = Math.floor(seconds % 60);
    return `${m}:${String(s).padStart(2, "0")}`;
  }

  const progress = $derived(duration > 0 ? position / duration : 0);
</script>

<div class="player">
  {#if error}
    <p class="hint">{error}</p>
    <button type="button" class="link" onclick={() => void files.open(path)}>
      {t("media-open-externally")}
    </button>
  {:else if !src}
    <p class="hint">{t("preview-loading")}</p>
  {:else}
    {#if kind === "video"}
      <!-- svelte-ignore a11y_media_has_caption -->
      <video
        {src}
        class="video"
        bind:paused
        bind:currentTime={position}
        bind:duration
        bind:volume
        {loop}
        aria-label={name}
      ></video>
    {:else}
      <audio
        {src}
        bind:paused
        bind:currentTime={position}
        bind:duration
        bind:volume
        {loop}
        aria-label={name}
      ></audio>

      {#if waveform}
        <!-- Peaks are drawn as flex-sized bars rather than on a canvas:
             the shape is the same, it scales with the pane for free, and
             it stays legible when the OS is in a high-contrast mode. -->
        <div
          class="wave"
          role="presentation"
          onclick={seekFromWaveform}
          style="--progress: {progress}"
        >
          {#each waveform.peaks as p}
            <span class="bar" style="height: {Math.max(2, p * 100)}%"></span>
          {/each}
          <!-- Played portion as one overlay driven by `--progress`.
               A `class:played` on each bar would make all 800 of them
               depend on `progress`, so every tick would re-evaluate 800
               expressions and diff 800 class attributes; this is a
               single style write per tick.

               Ticks are ~60/s, not the ~4/s `timeupdate` fires at:
               Svelte drives `bind:currentTime` from a rAF loop while
               the element is playing, because `timeupdate` is too
               coarse to animate against. That is the trade this
               binding makes — a smooth playhead for a 15x busier
               tick — and it is only paid while something is playing. -->
          <span class="played" aria-hidden="true"></span>
        </div>
      {/if}
    {/if}

    <div class="transport">
      <button
        type="button"
        class="play"
        onclick={() => (paused = !paused)}
        aria-label={t("media-play-pause")}
      >
        {paused ? "▶" : "⏸"}
      </button>
      <span class="time">{clock(position)}</span>
      <input
        type="range"
        class="seek"
        min="0"
        max={duration || 0}
        step="0.01"
        bind:value={position}
        aria-label={t("media-seek")}
      />
      <span class="time">{clock(duration)}</span>
      <button
        type="button"
        class="toggle"
        class:on={loop}
        aria-pressed={loop}
        onclick={() => (loop = !loop)}
        title={t("media-loop")}
        aria-label={t("media-loop")}>↺</button
      >
      <input
        type="range"
        class="volume"
        min="0"
        max="1"
        step="0.01"
        bind:value={volume}
        aria-label={t("media-volume")}
      />
    </div>

    {#if waveform}
      <!-- The existing badges, overlaid on the player rather than made
           into a separate panel: they answer "is this the take I want?",
           which is the same question the transport is there for. -->
      <div class="badges">
        <span class="badge">{waveform.codec}</span>
        <span class="badge">{waveform.sample_rate} Hz</span>
        <span class="badge">{waveform.channels === 1 ? "mono" : `${waveform.channels}ch`}</span>
        {#if waveform.lufs_integrated !== null}
          <span class="badge lufs">{waveform.lufs_integrated.toFixed(1)} LUFS</span>
        {/if}
      </div>
    {/if}
  {/if}
</div>

<style>
  .player {
    display: flex;
    flex-direction: column;
    gap: 8px;
  }
  .hint {
    margin: 0;
    color: var(--text-secondary);
    font-size: 12px;
  }
  .link {
    background: none;
    border: 0;
    padding: 0;
    color: var(--accent-cyan);
    cursor: pointer;
    font: inherit;
    text-decoration: underline;
    text-align: left;
  }
  .video {
    width: 100%;
    max-height: 240px;
    background: #000;
    border-radius: 4px;
  }
  .wave {
    position: relative;
    display: flex;
    align-items: flex-end;
    gap: 1px;
    height: 64px;
    padding: 2px;
    background: var(--bg-canvas);
    border: 1px solid var(--border);
    border-radius: 4px;
    cursor: pointer;
    overflow: hidden;
  }
  .bar {
    flex: 1 1 0;
    min-width: 1px;
    background: var(--text-secondary);
    opacity: 0.5;
  }
  .wave .played {
    position: absolute;
    inset: 0 auto 0 0;
    width: calc(var(--progress, 0) * 100%);
    background: color-mix(in srgb, var(--accent-cyan) 30%, transparent);
    pointer-events: none;
  }
  .transport {
    display: flex;
    align-items: center;
    gap: 6px;
  }
  .play,
  .toggle {
    min-width: 26px;
    height: 24px;
    border: 1px solid var(--border);
    border-radius: 4px;
    background: none;
    color: var(--text-primary);
    cursor: pointer;
    font-size: 12px;
  }
  .toggle.on {
    border-color: var(--accent-cyan);
    color: var(--accent-cyan);
  }
  .time {
    color: var(--text-secondary);
    font: 11px/1 var(--font-mono);
    min-width: 34px;
  }
  .seek {
    flex: 1;
    min-width: 40px;
  }
  .volume {
    width: 64px;
  }
  .badges {
    display: flex;
    flex-wrap: wrap;
    gap: 4px;
  }
  .badge {
    padding: 1px 6px;
    border: 1px solid var(--border);
    border-radius: 10px;
    color: var(--text-secondary);
    font-size: 11px;
  }
  .badge.lufs {
    color: var(--lens-audio);
    border-color: var(--lens-audio);
  }
</style>
