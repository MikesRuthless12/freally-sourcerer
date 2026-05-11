# Testing the macOS fast-bootstrap walker (`getattrlistbulk`)

Same goal as the Linux doc: verify the OS-specific fast walker
(`getattrlistbulk` here) **runs correctly** against a synthetic tree.
We're not chasing benchmark numbers — the pass criterion is "fast path
and `--baseline` agree on file count, dir count, and byte total."

> **The honest recommendation:** if you have any access at all to a
> real Mac (your own, a friend's, a cloud-rented mac1/mac2 instance),
> use it. macOS testing on Windows hardware via QEMU is technically
> doable but slow, finicky, and legally awkward (Apple's macOS EULA
> restricts the OS to Apple hardware). The cloud-Mac path is below.

---

## 0. Pick your test host

| Path | Time-to-test | Friction | Notes |
|---|---|---|---|
| **A. Real Mac you own / borrow** | 15 min | Lowest | Best signal. Skip to §1A. |
| **B. Cloud Mac rental** (AWS EC2 `mac2.metal`, MacStadium, Scaleway Apple Silicon) | 30 min + ~$2-5/hr | Low, legally clean | True production kernel; pay-per-use. Skip to §1B. |
| **C. VMware Workstation on Windows** (with the macOS Unlocker patch) | 1-2 hr setup | Medium, EULA grey area | Local VM you can iterate against. Skip to §1C. |

**A** is the cleanest. **B** is the cleanest *legally* if you're going
to ship a product. **C** is what you'll likely reach for during day-to-day
dev iteration; see the EULA note in §1C before committing to that path.

---

## 1A. Real Mac

Open Terminal.app.

### Install Rust (one-time)

```bash
xcode-select --install   # if you don't already have it
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y --default-toolchain stable
source $HOME/.cargo/env
rustc --version
```

### Clone + build

```bash
cd ~/Developer   # or anywhere on your $HOME volume
git clone https://github.com/MikesRuthless12/Sourcerer.git
cd Sourcerer
git checkout phase-13-fast-bootstrap
cargo build -p sourcerer-fsbench --release
```

Now jump to §2 — the rest of the doc is OS-agnostic.

---

## 1B. Cloud Mac (AWS EC2 mac2.metal example)

1. Open the AWS console → **EC2** → **Dedicated Hosts** → request a
   `mac2.metal` allocation. AWS requires a 24-hour minimum dedication.
2. Launch an EC2 instance of type `mac2.metal` onto that dedicated
   host. Pick the official **macOS 14 (Sonoma)** AMI.
3. Wait ~5 minutes for the instance to come up.
4. SSH in: `ssh -i your-key.pem ec2-user@<instance-public-ip>`. The
   first connection runs `automated-setup` and reboots once.
5. Follow §1A from "Install Rust" onward.

When you're done: stop the instance, then release the dedicated host
(otherwise you keep paying).

Other providers (MacStadium, Scaleway, MacInCloud) work the same way —
SSH in, install Rust, clone, build.

---

## 1C. VMware Workstation on Windows

> **Read this before starting:** Apple's macOS Software Licence
> Agreement permits installation only on Apple-branded hardware.
> Running macOS in any hypervisor on a Windows PC — VMware, QEMU,
> VirtualBox — is a grey area under that EULA. VMware Inc. doesn't
> officially support macOS as a guest on non-Apple hosts; the macOS
> option only appears in the guest-OS dropdown after applying a
> community patch ("Unlocker"). I'm documenting the *technical* setup
> because you asked; the *legal* decision is yours. **If you're
> shipping a product publicly, cloud Mac (§1B) is the clean answer.**

### What you'll need

| Item | Notes |
|---|---|
| **VMware Workstation Pro** | Free for personal use as of May 2024 (Broadcom changed licensing). Download from the official Broadcom support portal — you need a free Broadcom account. |
| **macOS Unlocker** | Community patch that enables the macOS guest-OS family in VMware. The most-maintained fork right now is `paolo-projects/auto-unlocker` on GitHub (handles Workstation 16 / 17). Other forks exist; pick the one whose README explicitly names your Workstation version. |
| **macOS installer .iso** | You need a fresh `Install macOS <Sonoma|Sequoia>.iso`. The clean way: build it on a real Mac (`createinstallmedia` + `hdiutil convert`). If you don't have one handy, community-built ISOs exist on the same auto-unlocker / OSX-KVM ecosystem repos — same legal grey area as the Unlocker itself. |
| **Disk space** | ~80 GB free for the macOS VM image (40 GB minimum at install; macOS grows with use). |
| **CPU** | An Intel or AMD x86-64 CPU with virtualization enabled in BIOS (VT-x / AMD-V). Hyper-V should be **disabled** on Windows or VMware can't use VT-x — check `Get-WindowsOptionalFeature -Online -FeatureName Microsoft-Hyper-V` in admin PowerShell and remove it if present. |

### Setup

#### 1. Install VMware Workstation Pro

1. Sign up for a free Broadcom account at <https://support.broadcom.com/>.
2. Navigate to **VMware Workstation Pro 17 (Personal Use)** → download the Windows installer.
3. Run the installer (default options are fine). Reboot when prompted.
4. Launch VMware Workstation. **Don't** create a VM yet — Unlocker has to be applied first.

#### 2. Apply the macOS Unlocker

> **VMware must be fully closed.** Quit all `vmware*.exe` processes in Task Manager before patching, or the Unlocker can't replace its DLLs.

1. Grab the latest release of an Unlocker compatible with your VMware version. As of May 2026 the most commonly-used is `paolo-projects/auto-unlocker` ([github.com/paolo-projects/auto-unlocker/releases](https://github.com/paolo-projects/auto-unlocker)). Read the README first.
2. Right-click the `auto-unlocker.exe` (or the equivalent for whichever fork you picked) → **Run as administrator**.
3. Click **Patch**. It edits `vmware-vmx.exe`, `vmwarebase.dll`, and friends in `C:\Program Files (x86)\VMware\VMware Workstation\` and downloads the macOS guest tools (`darwin.iso`).
4. Close the patcher. Re-launch VMware Workstation — "Apple Mac OS X" should now appear in the **Create New Virtual Machine** wizard's guest-OS dropdown.

If Unlocker fails: the most common cause is a leftover `vmware-vmx.exe` process, antivirus interference (whitelist the patcher directory), or a Workstation version the patcher fork doesn't support yet.

#### 3. Create the macOS VM

1. **File → New Virtual Machine** → **Custom** → defaults until "Guest Operating System".
2. **Guest OS family:** Apple Mac OS X. **Version:** macOS 14 (or whichever your installer is). If "Apple Mac OS X" isn't there, Unlocker didn't take — repeat §2.
3. **Disk size:** 80 GB, single file.
4. Before finishing, click **Customize Hardware** and:
    - **Memory:** 8 GB minimum (16 GB if you have it).
    - **Processors:** 4 cores minimum.
    - **CD/DVD:** point at your macOS installer `.iso`.
    - **USB Controller:** USB 3.1.
5. Finish. Don't power on yet.

#### 4. Edit the .vmx file (one critical line)

1. Locate the VM's directory (usually `C:\Users\<you>\Documents\Virtual Machines\<vm-name>\`).
2. Open the `.vmx` file in Notepad.
3. Add this line at the bottom (replace `Pro` if you're on Player):

    ```text
    smc.version = "0"
    ```

    This stops the EFI firmware from refusing to boot when it can't find a genuine SMC chip. Without it, macOS panics during early boot.

4. Save and close.

#### 5. Install macOS

1. Power on the VM. EFI should boot to the macOS installer.
2. **Disk Utility** → erase the virtual disk as **APFS**, name it whatever (e.g. `Macintosh HD`).
3. Quit Disk Utility → **Install macOS** → target the disk you just formatted.
4. Wait 30-60 minutes for the install. The VM reboots itself several times.
5. Run through the welcome assistant — skip Apple ID if you don't want to sign in.

#### 6. Install VMware Tools for macOS

1. From the macOS guest's menu bar: **VM → Install VMware Tools** (the menu item exists in the host Workstation app).
2. A `VMware Tools` installer mounts on the desktop. Run it. Reboot.
3. After reboot, copy/paste between host and guest works, screen resolution matches the window size, and shared folders are available (**VM → Settings → Options → Shared Folders**).

#### 7. Move the repo in

The cleanest path is a shared folder:

1. **VM → Settings → Options → Shared Folders → Always enabled**, then **Add** → point at `C:\Users\<you>\Desktop\Havoc Software\Sourcerer`.
2. Inside the macOS guest, that folder lives at `/Volumes/VMware Shared Folders/Sourcerer/`.
3. Symlink it into your `$HOME` so `cargo` builds against a saner path:

    ```bash
    ln -s "/Volumes/VMware Shared Folders/Sourcerer" ~/Sourcerer
    cd ~/Sourcerer
    ```

If shared folders give you grief (sometimes flaky on the Unlocker-patched
combo), use SCP instead: enable SSH on the macOS guest (**System
Settings → General → Sharing → Remote Login**), then from Windows
PowerShell:

```powershell
scp -r "C:\Users\<you>\Desktop\Havoc Software\Sourcerer" macuser@<vm-ip>:~/
```

#### 8. Install Rust + build

Inside Terminal.app on the macOS guest, follow §1A from "Install Rust"
onward. The build should take 10-15 minutes on a 4-core / 8 GB VM.

---

## 2. Generate a synthetic tree

Pick a working directory on the macOS volume (your $HOME is fine).

```bash
# 100 000 files
./target/release/sourcerer-fsbench gen \
    --root ~/fsbench-100k \
    --count 100000

# 500 000 files
./target/release/sourcerer-fsbench gen \
    --root ~/fsbench-500k \
    --count 500000

# 1 000 000 files
./target/release/sourcerer-fsbench gen \
    --root ~/fsbench-1m \
    --count 1000000
```

Each `gen` reports something like:

```
gen: 100001 file(s) across 4369 dir(s) (0 bytes) in 11.2s (8920 files/s)
```

> APFS create-file speed is slower than ext4. Generating 1 M files will
> take 5-15 minutes; the bottleneck is `creat()`, not the walker.

The `+1` on the count is the `.fsbench-tree` marker the generator drops
at the root so it can refuse to clobber unrelated directories on a
re-run.

---

## 3. Scan with the fast path and the baseline

Run the scan twice. The two reports must agree on file/dir counts and
byte totals.

```bash
# Fast path — getattrlistbulk (one syscall returns a batch of entries
# plus their attributes; Spotlight uses this)
./target/release/sourcerer-fsbench scan --root ~/fsbench-1m

# Baseline — walkdir (one syscall per entry for `lstat`)
./target/release/sourcerer-fsbench scan --root ~/fsbench-1m --baseline
```

Expected output:

```
scan [macos/getattrlistbulk]: 1000001 file(s), 4369 dir(s), 0 bytes, 14.8s (67500 files/s)
scan [baseline (walkdir)]:    1000001 file(s), 4369 dir(s), 0 bytes, 38.1s (26200 files/s)
```

**Pass criteria — "the code works":**

1. Both scans complete without panicking or crashing.
2. `file(s)` counts match.
3. `dir(s)` counts match.
4. `bytes` totals match.

Speed numbers are secondary. The fast path will likely be 2-3× the
baseline on APFS, but the goal here is correctness.

---

## 4. Test with non-empty files

To exercise the `ATTR_FILE_TOTALSIZE` size-field plumbing:

```bash
./target/release/sourcerer-fsbench gen \
    --root ~/fsbench-100k-sized \
    --count 100000 \
    --size 256

./target/release/sourcerer-fsbench scan --root ~/fsbench-100k-sized
./target/release/sourcerer-fsbench scan --root ~/fsbench-100k-sized --baseline
```

Both should report a `bytes` total ≈ 25.6 MB plus the marker.

---

## 5. Cleanup

```bash
rm -rf ~/fsbench-100k ~/fsbench-500k ~/fsbench-1m ~/fsbench-100k-sized
```

If you used VMware, shut down the macOS guest (**Apple menu → Shut
Down**) and leave the VM directory on disk for next time. To reclaim
the space, delete the `.vmwarevm` / VM directory wholesale; nothing
on the Windows host depends on it.

If you used a cloud Mac, **don't forget to stop the instance and
release the dedicated host** — both bill independently.

---

## Troubleshooting

| Symptom | Likely cause | Fix |
|---|---|---|
| `xcrun: error: invalid active developer path` during `cargo build` | Xcode CLI tools missing | `xcode-select --install` |
| `error: linker 'cc' not found` | Same as above | `xcode-select --install` |
| `scan` counts don't match `--baseline` | Probable bug in the fast walker; please file an issue with `sw_vers` output, tree size, and the `gen` invocation that produced the tree | — |
| `gen` is very slow on APFS | Expected — APFS create-file throughput is the bottleneck, not the walker | Use smaller `--count` values for iterative testing |
| `EACCES` from `~/Library/...` during scan | macOS sandboxes some user dirs; the fsbench walker logs and skips them, same as Sourcerer's real walker. Not a bug. | — |
| VMware installer panics on boot with "EFI VMware Hard Drive (0.0)" | Missing `smc.version = "0"` in the `.vmx` | §1C step 4 — add the line, save, retry |
| VMware "Apple Mac OS X" not in the guest-OS dropdown | Unlocker didn't apply (still-running VMware process, AV, wrong patcher version) | Close VMware completely, re-run the Unlocker as Administrator |
| `cargo build` extremely slow on a shared folder | Shared-folder I/O is hot-loop-heavy; `cargo` does many small writes | Copy the repo into the guest's `~` with `scp` and build there |

The fast walker lives at
[`crates/sourcerer-fsbench/src/fast_macos.rs`](crates/sourcerer-fsbench/src/fast_macos.rs)
if you want to read the `getattrlistbulk` setup directly.
