$ErrorActionPreference = 'Stop'

# Ask the registry where the uninstaller is rather than guessing an install
# path the user may have changed at install time.
[array]$key = Get-UninstallRegistryKey -SoftwareName 'Freally Sourcerer*'

if ($key.Count -eq 1) {
  $key | ForEach-Object {
    # Not the automatic variable $args — Chocolatey's
    # package validation runs PSScriptAnalyzer, which fails the package
    # on PSAvoidAssignmentToAutomaticVariable.
    $uninstallArgs = @{
      packageName    = 'freally-sourcerer'
      fileType       = 'exe'
      silentArgs     = '/S'
      validExitCodes = @(0)
      file           = $_.UninstallString.Trim('"')
    }
    Uninstall-ChocolateyPackage @uninstallArgs
  }
} elseif ($key.Count -eq 0) {
  Write-Warning 'Freally Sourcerer is not installed; nothing to uninstall.'
} else {
  # Uninstalling an arbitrary one of several would remove the wrong copy.
  Write-Warning "Found $($key.Count) installs matching 'Freally Sourcerer'. Remove the intended one by hand."
}
