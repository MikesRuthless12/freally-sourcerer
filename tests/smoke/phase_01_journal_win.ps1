# Phase 1 smoke (Windows).
#
# Builds the journal-win Phase 1 smoke driver, then drives the subscriber
# against a temp directory on the host's NTFS volume. Asserts: 1000 Create,
# 200 Modify, 100 Rename, 100 Delete events observed within 2 s of workload
# completion. Requires admin so DeviceIoControl on `\\.\<drive>:` is allowed.

$ErrorActionPreference = "Stop"

$root = Resolve-Path (Join-Path $PSScriptRoot "..\..")
Set-Location $root

Write-Host "==> cargo build --release --example phase01_smoke_driver -p sourcerer-journal-win"
cargo build --release --example phase01_smoke_driver -p sourcerer-journal-win 2>&1 | Out-Host
if ($LASTEXITCODE -ne 0) { throw "build failed (exit $LASTEXITCODE)" }

$driver = Join-Path $root "target\release\examples\phase01_smoke_driver.exe"
if (-not (Test-Path $driver)) { throw "smoke driver missing at $driver" }

# Pick a scratch directory on the same NTFS volume as the user profile.
$stamp = (Get-Date).ToString("yyyyMMdd-HHmmss")
$scratch = Join-Path $env:TEMP "sourcerer-phase01-$stamp"
New-Item -ItemType Directory -Path $scratch -Force | Out-Null

try {
    $drive = (Get-Item $scratch).PSDrive
    $fs = (Get-Volume -DriveLetter $drive.Name -ErrorAction SilentlyContinue).FileSystemType
    if ($fs -ne "NTFS") {
        Write-Warning "Scratch volume $($drive.Name): is $fs, not NTFS — skipping (Phase 1 requires NTFS)."
        return
    }

    Write-Host "==> running smoke against $scratch (volume $($drive.Name): / $fs)"
    & $driver --scratch $scratch --creates 1000 --modifies 200 --renames 100 --deletes 100 --timeout-secs 10
    if ($LASTEXITCODE -ne 0) {
        throw "phase01_smoke_driver failed (exit $LASTEXITCODE)"
    }

    Write-Host "Phase 1 smoke OK"
}
finally {
    Remove-Item -Recurse -Force -ErrorAction SilentlyContinue $scratch
}
