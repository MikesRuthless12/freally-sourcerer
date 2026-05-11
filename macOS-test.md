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
| **B. Cloud Mac rental** (AWS EC2 `mac2.metal`, MacStadium, Scaleway Apple Silicon) | 30 min + ~$2-5/hr | Low | True production kernel; pay-per-use. Skip to §1B. |
| **C. QEMU on Windows** (OSX-KVM image) | 2-4 hr setup | Highest | Works but slow. Skip to §1C. |

If you're stuck on Windows with no other option, **C** is documented at
the bottom, but please try **A** or **B** first.

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

## 1C. QEMU on Windows (last resort)

> **Read this before starting:** Apple's macOS Software Licence
> Agreement permits installation only on Apple-branded hardware.
> Running macOS in QEMU on a Windows PC is a grey area in the EULA.
> I'm documenting the *technical* setup here because you asked; the
> *legal* decision is yours. If you're shipping a product, the cloud
> Mac path in §1B is the cleaner answer.

### Setup

You'll run QEMU inside WSL2 (it performs much better than QEMU
directly on Windows, and gives you KVM acceleration once KVM is
exposed to the guest VM).

#### 1. Install WSL2 if you haven't already

Follow §1-2 of [`LinuxOS-test.md`](LinuxOS-test.md) to get Ubuntu in
WSL2.

#### 2. Install QEMU and supporting tools

Inside Ubuntu/WSL:

```bash
sudo apt-get update
sudo apt-get install -y qemu-system-x86 qemu-utils python3 python3-pip \
    libguestfs-tools dmg2img
```

#### 3. Clone OSX-KVM (the community macOS-on-QEMU recipe)

```bash
cd ~
git clone https://github.com/kholia/OSX-KVM.git
cd OSX-KVM
```

OSX-KVM's `README.md` is the authoritative install guide; the short
version is:

```bash
# Fetch a macOS recovery image (~700 MB)
./fetch-macOS-v2.py
# Convert dmg → img
dmg2img -i BaseSystem.dmg BaseSystem.img
# Create a 200 GB qcow2 virtual disk for the install
qemu-img create -f qcow2 mac_hdd_ng.img 200G
# Boot the installer
./OpenCore-Boot.sh
```

A QEMU window pops up showing the macOS installer. Use it to:
1. Open Disk Utility → erase the 200 GB virtual disk as APFS.
2. Quit Disk Utility → "Reinstall macOS" → target the disk you just
   formatted.
3. Wait ~45-90 minutes for the install.
4. Reboot. Run through the welcome assistant.

This is the slow part. Expect 1-2 hours wall-clock the first time.

#### 4. Inside the macOS VM

Open Terminal.app and follow §1A from "Install Rust" onward, using
the QEMU guest's clipboard or a shared folder to get the
`phase-13-fast-bootstrap` branch into the VM.

**Sharing files** between WSL host and macOS guest: the simplest
option is to expose the host's `~/Sourcerer` over Samba, or `scp` from
the guest to a Linux host (the QEMU `user-mode` network has the host
at `10.0.2.2`).

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

If you used QEMU, you can shut down the macOS guest and leave the
qcow2 image on disk for next time.

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
| QEMU boots to a black screen | OpenCore config drift; see OSX-KVM's `README.md` and issue tracker | — |

The fast walker lives at
[`crates/sourcerer-fsbench/src/fast_macos.rs`](crates/sourcerer-fsbench/src/fast_macos.rs)
if you want to read the `getattrlistbulk` setup directly.
