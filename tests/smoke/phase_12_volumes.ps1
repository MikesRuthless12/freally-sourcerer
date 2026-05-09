# Phase 12 smoke — Windows-specific volume detection.
#
# Verifies that the Rust-side `volumes::detect()` returns at least one
# fixed-disk volume by walking GetLogicalDrives + GetVolumeInformationW.
# The actual integration test runs as part of phase_12_indexd_client.

$ErrorActionPreference = "Stop"

$root = Resolve-Path "$PSScriptRoot/../.."
Push-Location $root
try {
    cargo test -p sourcerer-indexd --test phase_12_indexd_client --quiet | Out-Host
} finally {
    Pop-Location
}
Write-Host "phase_12_volumes.ps1: ok"
