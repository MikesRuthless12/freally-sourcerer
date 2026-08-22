#!/usr/bin/env node
// TASK-104 — generate the distribution-channel manifests from a published
// release.
//
// Five channels want the same handful of facts (version, asset URLs, their
// SHA256s, the product blurb) in five incompatible shapes. Writing them by
// hand means five places to forget on the next version bump — the same
// failure the docs site already had when it hard-coded download links, and
// a stale checksum here is worse than a stale link: the package manager
// refuses the install and blames the mirror.
//
// So they are generated. The digests come from the GitHub release API,
// which reports them per asset, so nothing has to be downloaded and the
// checksum is the one GitHub is actually serving.
//
// Usage:
//   node scripts/gen-packaging.mjs            # the latest release
//   node scripts/gen-packaging.mjs v0.23.2    # a specific tag
//
// Requires `gh` authenticated against the repo. Writes into packaging/.
import { execFileSync } from "node:child_process";
import { mkdirSync, writeFileSync } from "node:fs";
import { dirname, join } from "node:path";
import { fileURLToPath } from "node:url";

const repoRoot = join(dirname(fileURLToPath(import.meta.url)), "..");
const out = join(repoRoot, "packaging");

const REPO = "MikesRuthless12/freally-sourcerer";
const IDENT = "MikeWeaver.FreallySourcerer";
const APP_ID = "io.mikeweaver.freally";
const PRODUCT = "Freally Sourcerer";
const PUBLISHER = "Mike Weaver";
const HOMEPAGE = `https://github.com/${REPO}`;
const DOCS = "https://mikesruthless12.github.io/freally-sourcerer/";
const SHORT = "One search. Every source. Every OS.";
const LONG =
  "Freally is a cross-platform realtime filesystem search engine — filename, " +
  "content, audio, and similarity lenses on one journal-fed index.";

const tag = process.argv[2];
const api = tag ? `repos/${REPO}/releases/tags/${tag}` : `repos/${REPO}/releases/latest`;
const release = JSON.parse(execFileSync("gh", ["api", api], { cwd: repoRoot, encoding: "utf8" }));
const version = release.tag_name.replace(/^v/, "");
const releaseDate = release.published_at.slice(0, 10);

const assets = new Map(
  release.assets.map((a) => [
    a.name,
    {
      url: a.browser_download_url,
      size: a.size,
      sha256: (a.digest ?? "").replace(/^sha256:/, "").toUpperCase(),
    },
  ]),
);

/** A published asset by exact file name, or a loud failure.
 *  A missing asset means the release matrix dropped a leg — the shape the
 *  v0.23.0 release failed in — and generating a manifest that points at a
 *  URL nobody published would hide it until a user hit the 404. */
function asset(name) {
  const a = assets.get(name);
  if (!a) throw new Error(`release ${release.tag_name} has no asset ${name}`);
  if (!a.sha256) throw new Error(`release asset ${name} reports no digest`);
  return a;
}

const nsis = asset(`Freally.Sourcerer_${version}_x64-setup.exe`);
const dmgArm = asset(`Freally.Sourcerer_${version}_aarch64.dmg`);
const dmgIntel = asset(`Freally.Sourcerer_${version}_x64.dmg`);
const deb = asset(`Freally.Sourcerer_${version}_amd64.deb`);

function write(rel, body) {
  const path = join(out, rel);
  mkdirSync(dirname(path), { recursive: true });
  writeFileSync(path, body);
  console.log(`  wrote packaging/${rel}`);
}

console.log(`Generating packaging manifests for ${release.tag_name}\n`);

// ---------------------------------------------------------------- winget
// Three files, because that is the shape winget-pkgs wants: one for the
// version, one for the installers, one per locale. Only the NSIS installer
// is listed — the MSI installs the same program, and two installers of one
// architecture make winget choose for reasons the user cannot see.
write(
  `winget/${IDENT}.yaml`,
  `# yaml-language-server: $schema=https://aka.ms/winget-manifest.version.1.6.0.schema.json
PackageIdentifier: ${IDENT}
PackageVersion: ${version}
DefaultLocale: en-US
ManifestType: version
ManifestVersion: 1.6.0
`,
);

write(
  `winget/${IDENT}.installer.yaml`,
  `# yaml-language-server: $schema=https://aka.ms/winget-manifest.installer.1.6.0.schema.json
PackageIdentifier: ${IDENT}
PackageVersion: ${version}
InstallerLocale: en-US
MinimumOSVersion: 10.0.17763.0
InstallModes:
  - interactive
  - silent
UpgradeBehavior: install
ReleaseDate: ${releaseDate}
Installers:
  - Architecture: x64
    InstallerType: nullsoft
    InstallerUrl: ${nsis.url}
    InstallerSha256: ${nsis.sha256}
ManifestType: installer
ManifestVersion: 1.6.0
`,
);

write(
  `winget/${IDENT}.locale.en-US.yaml`,
  `# yaml-language-server: $schema=https://aka.ms/winget-manifest.defaultLocale.1.6.0.schema.json
PackageIdentifier: ${IDENT}
PackageVersion: ${version}
PackageLocale: en-US
Publisher: ${PUBLISHER}
PublisherUrl: ${HOMEPAGE}
PublisherSupportUrl: ${HOMEPAGE}/issues
PackageName: ${PRODUCT}
PackageUrl: ${HOMEPAGE}
License: Proprietary
LicenseUrl: ${HOMEPAGE}/blob/main/LICENSE.md
Copyright: Copyright (c) 2026 ${PUBLISHER}. All rights reserved.
ShortDescription: ${SHORT}
Description: ${LONG}
Moniker: freally
Tags:
  - search
  - file-search
  - everything
  - productivity
  - indexer
ReleaseNotesUrl: ${HOMEPAGE}/releases/tag/${release.tag_name}
Documentation:
  - DocumentLabel: User guide
    DocumentUrl: ${DOCS}documentation.html
ManifestType: defaultLocale
ManifestVersion: 1.6.0
`,
);

// ------------------------------------------------------------ chocolatey
// The installer is downloaded at install time rather than embedded in the
// .nupkg. That is not a size optimisation: the licence is all-rights-
// reserved, so embedding the binary would make the package itself a
// redistribution nobody has been granted the right to make. Chocolatey
// requires the download form for exactly this case.
write(
  "chocolatey/freally-sourcerer.nuspec",
  `<?xml version="1.0" encoding="utf-8"?>
<package xmlns="http://schemas.microsoft.com/packaging/2015/06/nuspec.xsd">
  <metadata>
    <id>freally-sourcerer</id>
    <version>${version}</version>
    <packageSourceUrl>${HOMEPAGE}/tree/main/packaging/chocolatey</packageSourceUrl>
    <owners>${PUBLISHER}</owners>
    <title>${PRODUCT}</title>
    <authors>${PUBLISHER}</authors>
    <projectUrl>${HOMEPAGE}</projectUrl>
    <iconUrl>https://raw.githubusercontent.com/${REPO}/main/assets/icons/freally.svg</iconUrl>
    <copyright>Copyright (c) 2026 ${PUBLISHER}. All rights reserved.</copyright>
    <licenseUrl>${HOMEPAGE}/blob/main/LICENSE.md</licenseUrl>
    <requireLicenseAcceptance>true</requireLicenseAcceptance>
    <projectSourceUrl>${HOMEPAGE}</projectSourceUrl>
    <docsUrl>${DOCS}documentation.html</docsUrl>
    <bugTrackerUrl>${HOMEPAGE}/issues</bugTrackerUrl>
    <tags>search file-search everything productivity indexer</tags>
    <summary>${SHORT}</summary>
    <description>${LONG}</description>
    <releaseNotes>${HOMEPAGE}/releases/tag/${release.tag_name}</releaseNotes>
  </metadata>
  <files>
    <file src="tools\\**" target="tools" />
  </files>
</package>
`,
);

write(
  "chocolatey/tools/chocolateyinstall.ps1",
  `$ErrorActionPreference = 'Stop'

# Generated by scripts/gen-packaging.mjs — do not hand-edit the version or
# the checksum. Regenerate against the published release instead; a stale
# checksum here fails the install and reads like a corrupted download.
$packageArgs = @{
  packageName    = 'freally-sourcerer'
  fileType       = 'exe'
  url64bit       = '${nsis.url}'
  checksum64     = '${nsis.sha256}'
  checksumType64 = 'sha256'
  # Tauri's NSIS installer takes NSIS's own silent switch.
  silentArgs     = '/S'
  validExitCodes = @(0)
}

Install-ChocolateyPackage @packageArgs
`,
);

write(
  "chocolatey/tools/chocolateyuninstall.ps1",
  `$ErrorActionPreference = 'Stop'

# Ask the registry where the uninstaller is rather than guessing an install
# path the user may have changed at install time.
[array]$key = Get-UninstallRegistryKey -SoftwareName '${PRODUCT}*'

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
  Write-Warning '${PRODUCT} is not installed; nothing to uninstall.'
} else {
  # Uninstalling an arbitrary one of several would remove the wrong copy.
  Write-Warning "Found $($key.Count) installs matching '${PRODUCT}'. Remove the intended one by hand."
}
`,
);

// -------------------------------------------------------- homebrew cask
write(
  "homebrew/freally-sourcerer.rb",
  `cask "freally-sourcerer" do
  arch arm: "aarch64", intel: "x64"

  version "${version}"
  sha256 arm:   "${dmgArm.sha256.toLowerCase()}",
         intel: "${dmgIntel.sha256.toLowerCase()}"

  url "${HOMEPAGE}/releases/download/v#{version}/Freally.Sourcerer_#{version}_#{arch}.dmg",
      verified: "github.com/${REPO}/"
  name "${PRODUCT}"
  desc "${SHORT}"
  homepage "${HOMEPAGE}"

  livecheck do
    url :url
    strategy :github_latest
  end

  depends_on macos: ">= :monterey"

  app "${PRODUCT}.app"

  # The indexer is a separate long-lived process with its own per-user
  # state. Leaving it running after an uninstall keeps a stale index alive
  # and a LaunchAgent pointing at a binary that is gone.
  uninstall quit:      "${APP_ID}",
            launchctl: "${APP_ID}.indexd"

  zap trash: [
    "~/Library/Application Support/freally",
    "~/Library/Caches/${APP_ID}",
    "~/Library/LaunchAgents/${APP_ID}.indexd.plist",
    "~/Library/Preferences/${APP_ID}.plist",
    "~/Library/Saved Application State/${APP_ID}.savedState",
  ]
end
`,
);

// --------------------------------------------------------------- flatpak
// `extra-data` rather than a source build: the source is not public, and
// this form has the user's own machine fetch the vendor .deb at install
// time. Flathub never hosts the binary, so the all-rights-reserved licence
// needs no redistribution grant for this channel.
write(
  `flatpak/${APP_ID}.yml`,
  `# Generated by scripts/gen-packaging.mjs for ${release.tag_name}.
app-id: ${APP_ID}
runtime: org.gnome.Platform
runtime-version: '46'
sdk: org.gnome.Sdk
command: freally-ui
separate-locales: false

finish-args:
  # A filesystem search engine that cannot read the filesystem is not one.
  # This is the permission that decides whether the app works at all, and
  # the one Flathub reviewers will ask about: the index is built by walking
  # the user's own files.
  - --filesystem=home
  - --filesystem=/media
  - --filesystem=/mnt
  - --share=ipc
  - --socket=fallback-x11
  - --socket=wayland
  - --device=dri
  # The updater and the docs site; nothing else.
  - --share=network
  - --talk-name=org.freedesktop.Notifications
  # Trash integration for delete-undo (SRC-M16) goes through the portal.
  - --talk-name=org.freedesktop.FileManager1

modules:
  - name: freally-sourcerer
    buildsystem: simple
    build-commands:
      - install -Dm755 apply_extra /app/bin/apply_extra
      - install -Dm755 freally-ui.sh /app/bin/freally-ui
      - install -Dm644 ${APP_ID}.metainfo.xml /app/share/metainfo/${APP_ID}.metainfo.xml
      - install -Dm644 ${APP_ID}.desktop /app/share/applications/${APP_ID}.desktop
    sources:
      - type: extra-data
        filename: freally.deb
        url: ${deb.url}
        sha256: ${deb.sha256.toLowerCase()}
        size: ${deb.size}
      - type: script
        dest-filename: apply_extra
        commands:
          - ar x freally.deb
          - tar xf data.tar.gz
          - rm -f freally.deb debian-binary control.tar.* data.tar.*
      - type: script
        dest-filename: freally-ui.sh
        commands:
          - exec /app/extra/usr/bin/freally-ui "$@"
      - type: file
        path: ${APP_ID}.metainfo.xml
      - type: file
        path: ${APP_ID}.desktop
`,
);

write(
  `flatpak/${APP_ID}.metainfo.xml`,
  `<?xml version="1.0" encoding="UTF-8"?>
<component type="desktop-application">
  <id>${APP_ID}</id>
  <name>${PRODUCT}</name>
  <summary>${SHORT}</summary>
  <developer id="io.mikeweaver">
    <name>${PUBLISHER}</name>
  </developer>
  <metadata_license>CC0-1.0</metadata_license>
  <project_license>LicenseRef-proprietary</project_license>
  <description>
    <p>${LONG}</p>
  </description>
  <launchable type="desktop-id">${APP_ID}.desktop</launchable>
  <url type="homepage">${HOMEPAGE}</url>
  <url type="bugtracker">${HOMEPAGE}/issues</url>
  <url type="help">${DOCS}documentation.html</url>
  <content_rating type="oars-1.1" />
  <releases>
    <release version="${version}" date="${releaseDate}">
      <url type="details">${HOMEPAGE}/releases/tag/${release.tag_name}</url>
    </release>
  </releases>
</component>
`,
);

write(
  `flatpak/${APP_ID}.desktop`,
  `[Desktop Entry]
Type=Application
Name=${PRODUCT}
Comment=${SHORT}
Exec=freally-ui %U
Icon=${APP_ID}
Terminal=false
Categories=Utility;FileTools;
Keywords=search;find;file;index;everything;
MimeType=x-scheme-handler/freally;
`,
);

// ------------------------------------------------------------------ snap
write(
  "snap/snapcraft.yaml",
  `# Generated by scripts/gen-packaging.mjs for ${release.tag_name}.
name: freally-sourcerer
version: '${version}'
summary: ${SHORT}
description: |
  ${LONG}
license: LicenseRef-proprietary
website: ${HOMEPAGE}
contact: ${HOMEPAGE}/issues
grade: stable
base: core22

# NOT strict. A filesystem search engine indexes the user's whole disk, and
# strict confinement's home interface cannot see /mnt, /media, or another
# user's mount points — which is a large part of what people search for.
# Classic confinement needs a human review and a written justification on
# the Snapcraft forum before the store will accept an upload. That request
# is the real blocker for this channel, not this file.
confinement: classic

parts:
  freally-sourcerer:
    plugin: dump
    source: ${deb.url}
    source-type: deb
    source-checksum: sha256/${deb.sha256.toLowerCase()}
    stage-packages:
      - libwebkit2gtk-4.1-0
      - libgtk-3-0
      - libayatana-appindicator3-1
      - librsvg2-2

apps:
  freally-sourcerer:
    command: usr/bin/freally-ui
    desktop: usr/share/applications/${APP_ID}.desktop
`,
);

console.log("\nDone. packaging/README.md records what each channel still needs from a human.");
