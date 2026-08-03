// SRC-M18 — inline media playback.

import { call } from "./client";

export interface Waveform {
  /** Peak amplitude per bucket, each 0..1. */
  peaks: number[];
  duration_ms: number;
  codec: string;
  sample_rate: number;
  channels: number;
  /** Null when the measurement was non-finite (a clip too short to
   *  have a meaningful integrated value). */
  lufs_integrated: number | null;
}

export function waveform(path: string): Promise<Waveform> {
  return call<Waveform>("media_waveform", { path });
}

/** The file's bytes. Rejects past the backend's inline size cap. */
export function bytes(path: string): Promise<ArrayBuffer> {
  return call<ArrayBuffer>("media_bytes", { path });
}

/** Extensions the preview pane will try to play inline. */
const AUDIO = new Set([
  "mp3",
  "flac",
  "wav",
  "ogg",
  "oga",
  "m4a",
  "aac",
  "opus",
  "aiff",
  "aif",
  "wma"
]);
const VIDEO = new Set(["mp4", "m4v", "webm", "mov", "mkv", "avi"]);

export function mediaKind(ext: string): "audio" | "video" | null {
  const e = ext.toLowerCase().replace(/^\./, "");
  if (AUDIO.has(e)) return "audio";
  if (VIDEO.has(e)) return "video";
  return null;
}
