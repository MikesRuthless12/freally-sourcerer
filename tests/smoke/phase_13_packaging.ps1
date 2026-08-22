# Phase 13 smoke — packaging (Windows side).
#
# Two halves, because they fail differently:
#
#   1. The OS-agnostic preconditions (`phase_13_packaging.rs`) — changelog
#      notes for the shipping version, the four release legs, `bundle.targets`,
#      version agreement. Cheap, always run.
#
#   2. The bundle shape itself, which needs a real `tauri build` and so is
#      opt-in. Set FREALLY_SMOKE_BUNDLE=1 to run it. Without a build there is
#      nothing to inspect, and pretending otherwise is how a release ships
#      missing a format nobody noticed was gone.
#
# Usage:
#   pwsh tests/smoke/phase_13_packaging.ps1
#   $env:FREALLY_SMOKE_BUNDLE = '1'; pwsh tests/smoke/phase_13_packaging.ps1
$ErrorActionPreference = 'Stop'

$root = Resolve-Path (Join-Path $PSScriptRoot '..\..')
Set-Location $root

Write-Output 'phase_13_packaging: preconditions'
Push-Location (Join-Path $root 'apps\freally-ui\src-tauri')
try {
  cargo test --test phase_13_packaging --locked --quiet
  if ($LASTEXITCODE -ne 0) { throw 'phase_13_packaging preconditions failed' }
} finally {
  Pop-Location
}

if ($env:FREALLY_SMOKE_BUNDLE -ne '1') {
  Write-Output 'phase_13_packaging.ps1: ok (bundle check skipped; set FREALLY_SMOKE_BUNDLE=1)'
  exit 0
}

Write-Output 'phase_13_packaging: building bundles (this takes several minutes)'
Push-Location (Join-Path $root 'apps\freally-ui')
try {
  pnpm tauri build
  if ($LASTEXITCODE -ne 0) { throw 'tauri build failed' }
} finally {
  Pop-Location
}

$bundle = Join-Path $root 'apps\freally-ui\src-tauri\target\release\bundle'
if (-not (Test-Path $bundle)) { throw "no bundle directory at $bundle" }

$failed = $false
# `msi` and `nsis` are the two Windows formats. The site hands people the
# NSIS setup.exe, and the updater manifest is built to prefer it — see
# `updaterJsonPreferNsis` in release.yml — so a missing NSIS bundle breaks
# both the download page and every in-app update.
foreach ($fmt in @('msi', 'nsis')) {
  $dir = Join-Path $bundle $fmt
  # Count the files rather than testing the directory: a failed leg leaves
  # the directory behind, empty, and `Test-Path` would call that a pass.
  $found = @(Get-ChildItem -Path $dir -File -ErrorAction SilentlyContinue)
  if ($found.Count -gt 0) {
    Write-Output "  ok       $fmt"
  } else {
    Write-Output "  MISSING  $fmt"
    $failed = $true
  }
}

# The updater refuses an unsigned artifact, and it does so silently — an
# install that never updates again looks exactly like one that is current.
$sigs = @(Get-ChildItem -Path $bundle -Filter '*.sig' -Recurse -File -ErrorAction SilentlyContinue)
if ($sigs.Count -gt 0) {
  Write-Output '  ok       updater signatures'
} else {
  Write-Output '  MISSING  updater signatures (.sig) — is TAURI_SIGNING_PRIVATE_KEY set?'
  $failed = $true
}

if ($failed) { throw 'phase_13_packaging.ps1: FAILED' }
Write-Output 'phase_13_packaging.ps1: ok'
