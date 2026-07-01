#!/usr/bin/env bash
# Phase 11 smoke — Linux + macOS. Validates the Tauri 2 + Svelte 5 UI
# builds cleanly and the mock IPC backend compiles. tauri-driver
# magic-moment perf gate runs on top of this; the truthful 5M-file
# gate moves to Phase 13 (TASK-100).

set -euo pipefail

cd "$(dirname "$0")/../.."
ROOT="$(pwd)"

echo "[phase 11 smoke] freally-query routing test"
cargo test -p freally-query --test phase_10_query 2>&1 | tail -n 20

echo "[phase 11 smoke] freally-ui src-tauri compiles"
cd "$ROOT/apps/freally-ui/src-tauri"
cargo check --quiet

echo "[phase 11 smoke] freally-ui pnpm install + check"
cd "$ROOT/apps/freally-ui"
if ! command -v pnpm >/dev/null 2>&1; then
  echo "[phase 11 smoke] pnpm not available — skipping JS portion (CI-only)"
  exit 0
fi
pnpm install --frozen-lockfile=false
pnpm run check
pnpm run build

echo "[phase 11 smoke] OK"
