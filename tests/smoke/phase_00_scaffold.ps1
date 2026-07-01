# Phase 0 smoke (Windows).
# Asserts: cargo build --all = 0; pnpm tauri build --debug = 0.

$ErrorActionPreference = "Stop"

$root = Resolve-Path (Join-Path $PSScriptRoot "..\..")
Set-Location $root

Write-Host "==> cargo build --all"
cargo build --all
if ($LASTEXITCODE -ne 0) { throw "cargo build --all failed (exit $LASTEXITCODE)" }

Write-Host "==> pnpm install (apps/freally-ui)"
Set-Location (Join-Path $root "apps\freally-ui")
pnpm install --frozen-lockfile=false
if ($LASTEXITCODE -ne 0) { throw "pnpm install failed (exit $LASTEXITCODE)" }

Write-Host "==> pnpm tauri build --debug"
pnpm tauri build --debug
if ($LASTEXITCODE -ne 0) { throw "pnpm tauri build --debug failed (exit $LASTEXITCODE)" }

Write-Host "Phase 0 smoke OK"
