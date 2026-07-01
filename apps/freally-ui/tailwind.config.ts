import type { Config } from "tailwindcss";

// PRD §9: tokens drive every color; Tailwind utilities map to CSS custom properties.
// Phase 11 keeps darkMode tied to <html data-theme="dark">; absence of the attribute
// (system mode) defers to prefers-color-scheme via the tokens.dark.css media query.
export default {
  content: ["./index.html", "./src/**/*.{ts,svelte,html}"],
  darkMode: ["class", '[data-theme="dark"]'],
  theme: {
    extend: {
      colors: {
        canvas: "var(--bg-canvas)",
        surface: "var(--bg-surface)",
        "surface-2": "var(--bg-surface-2)",
        primary: "var(--text-primary)",
        secondary: "var(--text-secondary)",
        border: "var(--border)",
        "accent-orange": "var(--accent-orange)",
        "accent-cyan": "var(--accent-cyan)",
        "accent-violet": "var(--accent-violet)",
        success: "var(--success)",
        warning: "var(--warning)",
        danger: "var(--danger)",
        "lens-filename": "var(--lens-filename)",
        "lens-content": "var(--lens-content)",
        "lens-audio": "var(--lens-audio)",
        "lens-similarity": "var(--lens-similarity)"
      },
      spacing: {
        "row-compact": "var(--row-height-compact)",
        "row-comfortable": "var(--row-height-comfortable)",
        "lens-gap": "var(--lens-section-gap)"
      },
      borderRadius: {
        row: "var(--result-row-radius)"
      }
    }
  },
  plugins: []
} satisfies Config;
