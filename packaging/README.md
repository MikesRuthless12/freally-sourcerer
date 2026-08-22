# Distribution channel manifests — TASK-104

Everything in this directory is **generated**. Do not hand-edit it:

```
node scripts/gen-packaging.mjs            # against the latest release
node scripts/gen-packaging.mjs v0.23.2    # against a specific tag
```

The generator reads the published release from the GitHub API, including
each asset's SHA256, so the checksums are the ones GitHub is actually
serving rather than ones somebody computed locally against a build that may
not be the one that shipped. It fails loudly if an expected asset is missing
— that is the shape the v0.23.0 release failed in, and a manifest pointing
at a URL nobody published would hide it until a user hit the 404.

Regenerate on **every** version bump. A stale checksum is worse than a stale
link: the package manager refuses the install and blames the download.

Current contents were generated for **v0.23.2**.

---

## What still needs a human, per channel

None of these can be submitted from here. Every one needs an account and a
pull request or upload under the owner's identity.

### winget → `microsoft/winget-pkgs`

Ready to submit. Fork `microsoft/winget-pkgs`, copy the three files to
`manifests/m/MikeWeaver/FreallySourcerer/<version>/`, and open a PR.
Validate first with `winget validate --manifest packaging/winget` and
install-test with `winget install --manifest packaging/winget`.

**Known friction:** the installer is not Authenticode-signed (there is no
code-signing certificate — see below), so SmartScreen will warn on first
run. winget accepts unsigned packages; users still see the warning.

### Chocolatey → `chocolatey.org`

Ready to pack. `choco pack packaging/chocolatey/freally-sourcerer.nuspec`
then `choco push`.

The installer is **downloaded at install time, not embedded** in the
`.nupkg`. That is required here, not a size choice: the licence is
all-rights-reserved, so embedding the binary would make the package itself
a redistribution nobody has been granted the right to make.

Expect the moderation queue to flag the unsigned installer and to ask about
the licence. Both answers are on the project page.

### Homebrew Cask → `Homebrew/homebrew-cask`

**Blocked on notarization, not on this file.** The cask itself is complete
and correct. But the `.app` inside the DMG is neither signed with a
Developer ID nor notarized, so on any current macOS a downloaded copy is
quarantined and Gatekeeper refuses it outright — "damaged and can't be
opened", which is a lie the user has no way to see through. A cask that
installs an app the OS then blocks is worse than no cask.

Unblocking it needs an Apple Developer ID **Application** certificate plus
notarization in the release workflow. That is the same missing-identity
problem as TASK-101's dropped `.pkg` target, one certificate class over.

### Flathub → `flathub/flathub`

Uses `extra-data`, so the user's own machine fetches the vendor `.deb` at
install time and **Flathub never hosts the binary**. That is deliberate: it
means this channel needs no redistribution grant for a proprietary licence.

Two things a reviewer will raise:

- **`--filesystem=home`, `/media` and `/mnt`.** A filesystem search engine
  that cannot read the filesystem is not one. Say so plainly in the PR; it
  is the single permission the whole app depends on.
- **`LicenseRef-proprietary`.** Legal for AppStream, and Flathub does host
  proprietary apps, but the submission needs the owner to confirm they are
  the copyright holder.

The manifest has not been built. Before submitting, run
`flatpak-builder --force-clean build packaging/flatpak/io.mikeweaver.freally.yml`
on a Linux machine and confirm the app launches.

### Snap Store → `snapcraft.io`

**Blocked on a classic-confinement grant.** `confinement: classic` is not a
shortcut taken to avoid work: strict confinement's `home` interface cannot
see `/mnt`, `/media`, or other users' mount points, which is a large part
of what people search for. A strictly-confined build would install, launch,
and then fail to find most of the user's files — the worst failure mode
available.

Classic confinement requires a written justification on the Snapcraft forum
and a human review before the store will accept an upload. Open that thread
*before* spending time on the build; if it is refused, this channel is not
available at all and the file above is moot.

The snap name also has to be registered (`snapcraft register
freally-sourcerer`) before the first push.

---

## The one blocker under all of this: no code-signing identity

`gh secret list` shows exactly two secrets, both minisign updater keys.
There is no Authenticode certificate and no Apple Developer ID. That is
survivable on winget and Chocolatey (a SmartScreen warning), fatal on
Homebrew Cask (Gatekeeper refuses the app), and irrelevant on Flathub and
Snap.

Decide that before submitting anywhere, because the answer changes which
three of the five are worth doing first.
