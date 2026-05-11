// Zoom store — scales the `#app` container via a CSS transform. The
// app uses absolute px everywhere, so adjusting :root font-size has no
// visible effect; a transform scales every descendant in lockstep.

const STORAGE_KEY = "sourcerer.zoom";
const MIN = 0.7;
const MAX = 1.6;
const STEP = 0.1;

function readInitial(): number {
  if (typeof localStorage === "undefined") return 1;
  const v = parseFloat(localStorage.getItem(STORAGE_KEY) ?? "1");
  if (!isFinite(v) || v < MIN || v > MAX) return 1;
  return v;
}

function apply(scale: number) {
  if (typeof document === "undefined") return;
  // Use WebView2's setting; falls through to the CSS transform for
  // anything outside `#app` (currently nothing of consequence). The
  // `zoom` property on the root element is the cross-engine knob the
  // Tauri webview honors and gives a crisp, non-blurry result that
  // `transform: scale` can't (transform produces aliased text below 1).
  const el = document.documentElement;
  el.style.setProperty("zoom", String(scale));
}

class ZoomStore {
  scale = $state(readInitial());

  constructor() {
    apply(this.scale);
  }

  in() {
    this.set(Math.min(MAX, Math.round((this.scale + STEP) * 10) / 10));
  }
  out() {
    this.set(Math.max(MIN, Math.round((this.scale - STEP) * 10) / 10));
  }
  reset() {
    this.set(1);
  }
  set(scale: number) {
    this.scale = scale;
    if (typeof localStorage !== "undefined") {
      localStorage.setItem(STORAGE_KEY, String(scale));
    }
    apply(scale);
  }
}

export const zoomStore = new ZoomStore();
