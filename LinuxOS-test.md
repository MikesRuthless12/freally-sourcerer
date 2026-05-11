# Testing the Linux fast-bootstrap walker (`getdents64 + statx`)

This walks you, end-to-end, from a stock Windows 11 box to a real Ubuntu
environment that can execute `getdents64` and `statx` against a 1 M-file
synthetic tree. We're not after benchmark numbers — the goal is "the
code runs without crashing on the real kernel and counts the same files
the cross-platform baseline counts."

> **TL;DR:** install WSL2 → Ubuntu → Rust → clone the repo inside WSL →
> `cargo build -p sourcerer-fsbench --release` → `gen` a tree → `scan` it
> twice (once fast path, once `--baseline`) → confirm the two reports
> agree.

---

## 0. Why WSL2 is a real test target

WSL2 runs an actual Linux kernel (currently 5.15.x or 6.x) inside a
hyper-light VM, not a translation shim. `getdents64`, `statx`, `inotify`,
and `fanotify` are all native syscalls there — your binary executes the
same instructions it would on a bare-metal Ubuntu server. The only
caveat: I/O against Windows paths (`/mnt/c/...`) goes through 9P/virtiofs
and is much slower than the WSL2-native ext4 disk. **Generate the
synthetic tree inside the WSL2 filesystem**, not on `/mnt/c`.

---

## 1. Install WSL2 + Ubuntu (Windows 11, one-time)

Open **PowerShell as Administrator** and run:

```powershell
wsl --install -d Ubuntu
```

That command:
1. Enables the `Microsoft-Windows-Subsystem-Linux` and
   `VirtualMachinePlatform` Windows features (reboots may be required on
   older Win11 builds).
2. Downloads the latest WSL2 kernel.
3. Installs the Ubuntu distribution from the Microsoft Store.
4. Opens a terminal where Ubuntu finishes its first-boot setup — pick a
   UNIX username and password. These are *local to WSL*; they don't have
   to match your Windows account.

Reboot if prompted, then re-open Ubuntu (Start menu → "Ubuntu"). You
should land at a `$` prompt looking like `mike@LAPTOP:~$`.

Verify:

```bash
uname -a
# Linux LAPTOP 5.15.x.x-microsoft-standard-WSL2 ...

cat /etc/os-release | head -2
# PRETTY_NAME="Ubuntu 22.04.x LTS"
```

---

## 2. Install the Rust toolchain inside Ubuntu

```bash
sudo apt-get update
sudo apt-get install -y build-essential pkg-config libssl-dev
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y --default-toolchain stable
source $HOME/.cargo/env
rustc --version
# rustc 1.94.0 (or newer)
```

> Sourcerer pins `1.94.0` via `rust-toolchain.toml`. When you `cargo
> build` inside the repo, rustup will auto-install that exact toolchain
> if it's not already there — no extra steps needed.

---

## 3. Get the repo into WSL

You have two reasonable options. **Option A is faster and required for
the benchmark to mean anything.**

### Option A — clone inside WSL (recommended)

```bash
cd ~
git clone https://github.com/MikesRuthless12/Sourcerer.git
cd Sourcerer
git checkout phase-13-fast-bootstrap
```

This puts the source tree on the WSL2 ext4 filesystem (~50 MB/s+).

### Option B — share the Windows working tree (slow)

If you'd rather edit on Windows and run on Linux, the Windows path is
available as `/mnt/c/Users/<you>/Desktop/Havoc Software/Sourcerer`. You
*can* `cargo build` from there but compilation will be 5-10× slower
and the synthetic tree will inherit Windows I/O latency. Don't use this
for `gen` / `scan`; use Option A.

---

## 4. Build the fsbench binary

From the Sourcerer root inside WSL:

```bash
cargo build -p sourcerer-fsbench --release
```

The compiled binary lands at:

```
target/release/sourcerer-fsbench
```

Sanity-check:

```bash
./target/release/sourcerer-fsbench --help
```

---

## 5. Generate a synthetic tree

Pick a working directory **inside the WSL filesystem** — not on
`/mnt/c`. The Ubuntu home directory (`~`) is the right place.

```bash
# 100 000 empty files, balanced across a 4-deep × 16-wide fanout.
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

Each `gen` prints a line like:

```
gen: 100001 file(s) across 4369 dir(s) (0 bytes) in 8.41s (11891 files/s)
```

The `+1` on the file count is the `.fsbench-tree` marker file the
generator drops at the root.

The generator is **destructive but safe**: it refuses to run against
any directory that isn't either empty or already an fsbench tree
(checked via the marker file). You cannot accidentally point it at your
`~/Documents`.

---

## 6. Scan via the fast path and the baseline

Run the scan **twice** — once via the OS-specific fast path, once via
the cross-platform `walkdir` baseline. The two reports should agree on
file and directory counts. If they don't, the fast path has a bug.

```bash
# Fast path — getdents64 + statx
./target/release/sourcerer-fsbench scan --root ~/fsbench-1m

# Baseline — walkdir (uses readdir/getdents64 under the hood, but
# does one syscall per file for stat)
./target/release/sourcerer-fsbench scan --root ~/fsbench-1m --baseline
```

Expected output shape:

```
scan [linux/getdents64+statx]: 1000001 file(s), 4369 dir(s), 0 bytes, 7.2s (139000 files/s)
scan [baseline (walkdir)]:     1000001 file(s), 4369 dir(s), 0 bytes, 19.4s (51500 files/s)
```

**The "code works" pass criteria:**

1. Both scans complete without panicking.
2. `file(s)` counts match exactly.
3. `dir(s)` counts match exactly.
4. `bytes` totals match exactly.

Anything else (the specific elapsed times, the throughput numbers) is
benchmark territory — useful for the Phase-13 perf write-up but not
what we're verifying here.

---

## 7. Test with non-empty files

To exercise the size-field plumbing through `statx`:

```bash
./target/release/sourcerer-fsbench gen \
    --root ~/fsbench-100k-sized \
    --count 100000 \
    --size 256

./target/release/sourcerer-fsbench scan --root ~/fsbench-100k-sized
./target/release/sourcerer-fsbench scan --root ~/fsbench-100k-sized --baseline
```

The two scans should agree on `bytes` (≈ 25.6 MB plus the marker).

---

## 8. Cleanup

```bash
rm -rf ~/fsbench-100k ~/fsbench-500k ~/fsbench-1m ~/fsbench-100k-sized
```

If you want to remove WSL entirely (you probably don't):

```powershell
# From an Admin PowerShell on Windows
wsl --unregister Ubuntu
```

---

## Troubleshooting

| Symptom | Likely cause | Fix |
|---|---|---|
| `error: linker 'cc' not found` during build | Missing `build-essential` | `sudo apt-get install -y build-essential` |
| `error[E0463]: can't find crate for 'libc'` | Cross-target left over from your Windows host | `cargo build` from inside WSL, not from `/mnt/c` |
| `scan` counts don't match `--baseline` | Probable bug in the fast path; please file an issue with the tree size, `cargo --version`, `uname -a` | — |
| `gen` runs forever on `/mnt/c` | I/O across the WSL/Windows boundary is slow | Generate inside `~`, not `/mnt/c/...` |
| `EACCES` errors during scan | The fsbench walker is best-effort; it logs and skips unreadable subtrees, same as Sourcerer's real bootstrap walker. Not a bug. | — |

The fast walker lives at
[`crates/sourcerer-fsbench/src/fast_linux.rs`](crates/sourcerer-fsbench/src/fast_linux.rs)
if you want to read the syscall loop directly.
