#!/usr/bin/env node
// Local CI — run the SAME checks as .github/workflows/ci.yml before pushing.
//
// Mirrors the single matrix `ci` job (windows / macOS / ubuntu):
//   Rust: cargo fmt --check · clippy -D warnings · nextest · doctests ·
//         src-tauri fmt+clippy+test · xtask i18n-lint (+ cargo-deny if
//         installed — CI runs it on Linux only)
//   UI (apps/freally-ui, pnpm): svelte-check · vitest unit · tauri build
//         --debug --no-bundle
//
// Unlike CI (which stops a job at the first failing step), this runs EVERY
// check and prints one summary at the end, so a single pass surfaces all
// problems. It exits non-zero if anything failed, so it's safe to gate a
// push on it.
//
// The Rust and UI lanes run CONCURRENTLY, each sequential within itself.
// They touch different toolchains and different directories, so there is
// nothing to serialize them for — except the final `tauri build`, which
// compiles Rust and therefore waits for the Rust lane to release the cargo
// lock. Ordering it last in the UI lane is what makes that free.
//
// Not mirrored locally:
//   • fsbench correctness+benchmark (macOS/Linux only, 1.5M synthetic
//     files) — CI skips it on Windows anyway; too heavy for the inner loop.
//   • FSEvents smoke (macOS only).
//
// Usage:  node scripts/ci-local.mjs [--rust-only] [--ui-only] [--no-build]
//                                   [--install] [--serial]
//   --rust-only  run only the Rust checks
//   --ui-only    run only the UI checks
//   --no-build   skip the heavy `tauri build` step (fast inner-loop)
//   --install    (re)install UI deps first: pnpm install --frozen-lockfile
//   --serial     run the lanes one after another (readable interleaved
//                output is the tradeoff; use when a failure is confusing)
import { spawn, spawnSync } from "node:child_process";
import { existsSync } from "node:fs";
import { dirname, join } from "node:path";
import { fileURLToPath } from "node:url";

const repoRoot = join(dirname(fileURLToPath(import.meta.url)), "..");
const uiDir = join(repoRoot, "apps", "freally-ui");
const tauriDir = join(uiDir, "src-tauri");

const args = new Set(process.argv.slice(2));
const rustOnly = args.has("--rust-only");
const uiOnly = args.has("--ui-only");
const noBuild = args.has("--no-build");
const doInstall = args.has("--install");
const serial = args.has("--serial");

// Pass the whole probe as one shell string (not an args array) — with
// shell:true an args array triggers a Node deprecation warning and isn't
// escaped anyway.
function have(commandLine) {
  return spawnSync(commandLine, { stdio: "ignore", shell: true }).status === 0;
}

const hasRust = existsSync(join(repoRoot, "Cargo.toml"));
const hasUi = existsSync(join(uiDir, "package.json"));

// CI runs `cargo nextest`, which is markedly faster on this suite (most of
// the 760 tests build their own Tantivy index, and a process per test keeps
// them off each other's locks). Fall back to `cargo test` when it isn't
// installed rather than making it a hard prerequisite.
const hasNextest = hasRust && have("cargo nextest --version");
// A handful of tests assert a wall-clock budget, and the lanes below run
// concurrently — so those two facts cannot both hold at once. Measured
// while `vitest` and `svelte-check` are competing for cores, the parser
// budget test fails on a machine that is fine: it is measuring the load,
// not the parser. Its own comment says so, and best-of-5 rounds cannot
// escape *sustained* load the way it escapes a single stolen core.
//
// So they come out of the parallel phase and run on their own at the end,
// which is the only condition under which the number means anything.
// Kept identical to the filter in `.github/workflows/ci.yml`, which splits
// its nextest step the same way. Two copies of one expression that have to
// agree: if they drift, the local gate stops predicting CI for exactly the
// tests that are hardest to reproduce.
const TIMED = "test(magic_moment) or test(latency)";
const testCmd = hasNextest
  ? `cargo nextest run --workspace --locked -E "not (${TIMED})"`
  : "cargo test --workspace --locked";
// Probed once: the step and the note that explains its absence have to
// agree, and two separate `have()` calls is two chances for them not to.
const hasDeny = hasRust && have("cargo deny --version");

const rustLane = [];
const uiLane = [];

if (!uiOnly && hasRust) {
  rustLane.push(["rust: fmt", "cargo fmt --all -- --check", repoRoot]);
  rustLane.push([
    "rust: clippy",
    "cargo clippy --workspace --all-targets --locked -- -D warnings",
    repoRoot
  ]);
  // `apps/freally-ui/src-tauri` is excluded from the workspace, so neither
  // of the two above ever reaches it. CI grew its own step for this after
  // three clippy errors sat there unnoticed; local CI has to match or it
  // stops being a pre-push gate.
  rustLane.push(["rust: fmt (src-tauri)", "cargo fmt -- --check", tauriDir]);
  rustLane.push([
    "rust: clippy (src-tauri)",
    "cargo clippy --all-targets --locked -- -D warnings",
    tauriDir
  ]);
  // Its tests too. Same exclusion, same consequence one step over: the
  // menu-bar / status-bar parity suites and the Phase-13 packaging +
  // updater gates live here, and `--workspace` has never run them.
  rustLane.push(["rust: test (src-tauri)", "cargo test --locked", tauriDir]);
  rustLane.push([hasNextest ? "rust: nextest" : "rust: test", testCmd, repoRoot]);
  // nextest does not run doctests, so CI runs them separately (Linux
  // only, since they are not platform-specific). Mirrored here or a
  // doctest added tomorrow passes the pre-push gate and fails CI.
  if (hasNextest) {
    rustLane.push(["rust: doctests", "cargo test --workspace --doc --locked", repoRoot]);
  }
  rustLane.push(["rust: xtask i18n-lint", "cargo run -p xtask --locked -- i18n-lint", repoRoot]);
  // CI runs cargo-deny on Linux only (Docker action); run it locally when
  // installed.
  if (hasDeny) {
    rustLane.push(["rust: cargo-deny", "cargo deny check", repoRoot]);
  }
}

if (!rustOnly && hasUi) {
  if (doInstall) {
    uiLane.push(["ui: pnpm install", "pnpm install --frozen-lockfile", uiDir]);
  }
  uiLane.push(["ui: svelte-check", "pnpm run check", uiDir]);
  uiLane.push(["ui: vitest unit", "pnpm run test:unit", uiDir]);
  if (!noBuild) {
    // Last in the lane on purpose: it compiles Rust, so it would otherwise
    // sit blocked on the cargo lock the Rust lane is holding.
    uiLane.push(["ui: tauri build", "pnpm tauri build --debug --no-bundle", uiDir]);
  }
}

if (rustLane.length === 0 && uiLane.length === 0) {
  console.error("ci-local: nothing to run (no Rust/UI detected, or filtered out).");
  process.exit(1);
}

if (!uiOnly && hasRust) {
  if (!hasNextest) {
    console.log("• note: cargo-nextest not installed — using `cargo test` (CI uses nextest).");
  }
  if (!hasDeny) {
    console.log("• note: cargo-deny not installed — skipping (CI runs it on Linux).");
  }
  console.log("• note: fsbench + FSEvents smoke are macOS/Linux-only CI steps — not run locally.");
}

// Concurrent lanes buffer their output and print it as each step ends, so
// two lanes cannot interleave mid-line into unreadable soup.
const buffered = !serial && rustLane.length > 0 && uiLane.length > 0;
if (buffered) {
  console.log("• lanes: rust ∥ ui — output is buffered per step (use --serial to stream).");
}

function run(cmd, cwd) {
  return new Promise((resolve) => {
    if (!buffered) {
      const r = spawnSync(cmd, { cwd, stdio: "inherit", shell: true });
      resolve({ status: r.status, out: "" });
      return;
    }
    const child = spawn(cmd, { cwd, shell: true });
    let out = "";
    child.stdout.on("data", (d) => (out += d));
    child.stderr.on("data", (d) => (out += d));
    child.on("close", (status) => resolve({ status, out }));
  });
}

async function runLane(steps, results) {
  for (const [name, cmd, cwd] of steps) {
    const label = cwd === repoRoot ? "." : cwd === uiDir ? "apps/freally-ui" : "src-tauri";
    const bar = "─".repeat(Math.max(0, 56 - name.length));
    // Streaming prints the header first; buffered prints it with the
    // output, so two lanes cannot interleave a header away from its log.
    const header = `\n▶ ${name} ${bar}\n  $ ${cmd}  (in ${label})`;
    if (!buffered) console.log(header);
    const started = process.hrtime.bigint();
    const { status, out } = await run(cmd, cwd);
    const secs = Number((process.hrtime.bigint() - started) / 1_000_000n) / 1000;
    if (buffered) {
      console.log(header);
      if (out) process.stdout.write(out.endsWith("\n") ? out : out + "\n");
    }
    results.push({ name, ok: status === 0, secs });
  }
}

const results = [];
const wall = process.hrtime.bigint();
if (buffered) {
  await Promise.all([runLane(rustLane, results), runLane(uiLane, results)]);
} else {
  await runLane(rustLane, results);
  await runLane(uiLane, results);
}

// After both lanes are done, with nothing else running.
if (!uiOnly && hasRust && hasNextest) {
  await runLane(
    [["rust: timed budgets", `cargo nextest run --workspace --locked -E "${TIMED}"`, repoRoot]],
    results
  );
}
const wallSecs = Number((process.hrtime.bigint() - wall) / 1_000_000n) / 1000;

console.log("\n" + "═".repeat(64));
console.log("  Local CI summary");
console.log("═".repeat(64));
let failed = 0;
for (const r of results) {
  const mark = r.ok ? "✓ pass" : "✗ FAIL";
  console.log(`  ${mark}  ${r.name.padEnd(24)} ${r.secs.toFixed(1)}s`);
  if (!r.ok) failed++;
}
console.log("─".repeat(64));
console.log(`  wall clock${" ".repeat(21)}${wallSecs.toFixed(1)}s`);
console.log("═".repeat(64));

if (failed > 0) {
  console.error(`\n✗ ${failed} check(s) failed — fix before pushing.`);
  process.exit(1);
}
console.log("\n✓ All checks passed — matches CI. Safe to push.");
