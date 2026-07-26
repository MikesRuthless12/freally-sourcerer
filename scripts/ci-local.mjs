#!/usr/bin/env node
// Local CI — run the SAME checks as .github/workflows/ci.yml before pushing.
//
// Mirrors the single matrix `ci` job (windows / macOS / ubuntu):
//   Rust: cargo fmt --check · clippy -D warnings · test · xtask i18n-lint
//         (+ cargo-deny if installed — CI runs it on Linux only)
//   UI (apps/freally-ui, pnpm): svelte-check · vitest unit · tauri build --debug --no-bundle
//
// Unlike CI (which stops a job at the first failing step), this runs EVERY check
// and prints one summary at the end, so a single pass surfaces all problems. It
// exits non-zero if anything failed, so it's safe to gate a push on it.
//
// Not mirrored locally:
//   • fsbench correctness+benchmark (macOS/Linux only, 1.5M synthetic files) — CI
//     skips it on Windows anyway; too heavy for the inner loop. See ci.yml.
//   • FSEvents smoke (macOS only).
//
// Usage:  node scripts/ci-local.mjs [--rust-only] [--ui-only] [--no-build] [--install]
//   --rust-only  run only the Rust checks
//   --ui-only    run only the UI checks
//   --no-build   skip the heavy `tauri build` step (fast inner-loop)
//   --install    (re)install UI deps first: pnpm install --frozen-lockfile=false
import { spawnSync } from "node:child_process";
import { existsSync } from "node:fs";
import { dirname, join } from "node:path";
import { fileURLToPath } from "node:url";

const repoRoot = join(dirname(fileURLToPath(import.meta.url)), "..");
const uiDir = join(repoRoot, "apps", "freally-ui");

const args = new Set(process.argv.slice(2));
const rustOnly = args.has("--rust-only");
const uiOnly = args.has("--ui-only");
const noBuild = args.has("--no-build");
const doInstall = args.has("--install");

// Pass the whole probe as one shell string (not an args array) — with shell:true
// an args array triggers a Node deprecation warning and isn't escaped anyway.
function have(commandLine) {
  return spawnSync(commandLine, { stdio: "ignore", shell: true }).status === 0;
}

const steps = [];
function step(name, cmd, cwd) {
  steps.push({ name, cmd, cwd });
}

const hasRust = existsSync(join(repoRoot, "Cargo.toml"));
const hasUi = existsSync(join(uiDir, "package.json"));

if (doInstall && hasUi) {
  step("ui: pnpm install", "pnpm install --frozen-lockfile=false", uiDir);
}

if (!uiOnly && hasRust) {
  step("rust: fmt", "cargo fmt --all -- --check", repoRoot);
  step("rust: clippy", "cargo clippy --workspace --all-targets -- -D warnings", repoRoot);
  step("rust: test", "cargo test --workspace", repoRoot);
  step("rust: xtask i18n-lint", "cargo run -p xtask -- i18n-lint", repoRoot);
  // CI runs cargo-deny on Linux only (Docker action); run it locally when installed.
  if (have("cargo deny --version")) {
    step("rust: cargo-deny", "cargo deny check", repoRoot);
  } else {
    console.log("• note: cargo-deny not installed — skipping (CI runs it on Linux).");
  }
  console.log("• note: fsbench + FSEvents smoke are macOS/Linux-only CI steps — not run locally.");
}

if (!rustOnly && hasUi) {
  step("ui: svelte-check", "pnpm run check", uiDir);
  step("ui: vitest unit", "pnpm run test:unit", uiDir);
  if (!noBuild) {
    step("ui: tauri build", "pnpm tauri build --debug --no-bundle", uiDir);
  } else {
    console.log("• note: --no-build — skipping `tauri build` (CI runs it).");
  }
}

if (steps.length === 0) {
  console.error("ci-local: nothing to run (no Rust/UI detected, or filtered out).");
  process.exit(1);
}

const results = [];
for (const s of steps) {
  const label = s.cwd === repoRoot ? "." : "apps/freally-ui";
  const bar = "─".repeat(Math.max(0, 56 - s.name.length));
  console.log(`\n▶ ${s.name} ${bar}`);
  console.log(`  $ ${s.cmd}  (in ${label})`);
  const started = process.hrtime.bigint();
  const r = spawnSync(s.cmd, { cwd: s.cwd, stdio: "inherit", shell: true });
  const secs = Number((process.hrtime.bigint() - started) / 1_000_000n) / 1000;
  const ok = r.status === 0;
  results.push({ name: s.name, ok, secs });
}

console.log("\n" + "═".repeat(64));
console.log("  Local CI summary");
console.log("═".repeat(64));
let failed = 0;
for (const r of results) {
  const mark = r.ok ? "✓ pass" : "✗ FAIL";
  console.log(`  ${mark}  ${r.name.padEnd(24)} ${r.secs.toFixed(1)}s`);
  if (!r.ok) failed++;
}
console.log("═".repeat(64));

if (failed > 0) {
  console.error(`\n✗ ${failed} check(s) failed — fix before pushing.`);
  process.exit(1);
}
console.log("\n✓ All checks passed — matches CI. Safe to push.");
