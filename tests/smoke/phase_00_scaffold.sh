#!/usr/bin/env bash
# Phase 0 smoke (Linux + macOS).
# Asserts: cargo build --all = 0; pnpm tauri build --debug = 0.

set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
cd "$ROOT"

echo "==> cargo build --all"
cargo build --all

echo "==> pnpm install (apps/freally-ui)"
cd apps/freally-ui
pnpm install --frozen-lockfile=false

echo "==> pnpm tauri build --debug"
pnpm tauri build --debug

echo "Phase 0 smoke OK"
