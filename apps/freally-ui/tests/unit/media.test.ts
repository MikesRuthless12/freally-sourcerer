// SRC-M18 — which files the preview pane offers to play.

import { describe, it, expect } from "vitest";
import { mediaKind } from "../../src/lib/ipc/media";

describe("mediaKind", () => {
  it("recognises audio and video extensions", () => {
    expect(mediaKind("flac")).toBe("audio");
    expect(mediaKind("mp3")).toBe("audio");
    expect(mediaKind("mp4")).toBe("video");
    expect(mediaKind("mkv")).toBe("video");
  });

  it("ignores case and a leading dot", () => {
    // Hits carry `ext` without a dot, but a caller passing `.MP3` must
    // not silently fall through to "not playable".
    expect(mediaKind("FLAC")).toBe("audio");
    expect(mediaKind(".Mp4")).toBe("video");
  });

  it("returns null for everything else", () => {
    expect(mediaKind("txt")).toBeNull();
    expect(mediaKind("")).toBeNull();
    // Not a media file despite the substring.
    expect(mediaKind("mp3x")).toBeNull();
  });
});
