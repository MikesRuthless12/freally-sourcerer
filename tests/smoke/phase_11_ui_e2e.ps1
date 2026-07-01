# Phase 11 smoke - Windows. Validates the Tauri 2 + Svelte 5 UI builds
# cleanly and the mock IPC backend compiles.

$ErrorActionPreference = "Stop"

$Root = Resolve-Path "$PSScriptRoot/../.."
Set-Location $Root

Write-Output "[phase 11 smoke] freally-query routing test"
cargo test -p freally-query --test phase_10_query

Write-Output "[phase 11 smoke] freally-ui src-tauri compiles"
Set-Location "$Root/apps/freally-ui/src-tauri"
cargo check --quiet
if ($LASTEXITCODE -ne 0) { throw "src-tauri cargo check failed" }

Write-Output "[phase 11 smoke] freally-ui pnpm install + check"
Set-Location "$Root/apps/freally-ui"
if (-not (Get-Command pnpm -ErrorAction SilentlyContinue)) {
  Write-Output "[phase 11 smoke] pnpm not available - skipping JS portion (CI-only)"
  exit 0
}
pnpm install
if ($LASTEXITCODE -ne 0) { throw "pnpm install failed" }
pnpm run check
if ($LASTEXITCODE -ne 0) { throw "pnpm run check failed" }
pnpm run build
if ($LASTEXITCODE -ne 0) { throw "pnpm run build failed" }

Write-Output "[phase 11 smoke] OK"
