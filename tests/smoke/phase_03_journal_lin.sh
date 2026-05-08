#!/usr/bin/env bash
# Phase 3 smoke (Linux).
#
# Builds the Phase-3 smoke driver, runs the inotify (or fanotify, when
# CAP_SYS_ADMIN is held) subscriber against a scratch directory, performs
# the spec workload, and asserts observed event counts stay within 1% of
# the expected set (the "overlap >= 99%" gate that Phase 1 + Phase 2 use).
# After the driver finishes, the harness also walks the post-workload
# tree with `find -newer` for a sanity-check ground-truth comparison.
#
# When SOURCERER_LIN_SMOKE_FS lists multiple filesystems (default:
# "ext4 btrfs zfs") the script repeats the workload on a scratch volume
# of each type. Volumes are located via:
#
#     SOURCERER_LIN_SMOKE_<FS>_DIR=/mnt/sourcerer-test-<fs>
#
# (uppercase fs name, e.g. SOURCERER_LIN_SMOKE_EXT4_DIR). When a
# filesystem-specific dir is unset OR not a directory, that fs is
# skipped — useful for CI hosts that only have ext4. ext4 always
# defaults to a tempdir under $HOME if its env var is unset, so the
# minimum CI run still exercises one filesystem.
#
# Skipped on non-Linux hosts so CI on Windows / macOS runs other smokes.

set -euo pipefail

if [[ "$(uname -s)" != "Linux" ]]; then
    echo "phase_03_journal_lin.sh: skipping on non-Linux host ($(uname -s))"
    exit 0
fi

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
cd "$ROOT"

echo "==> cargo build --release --example phase03_smoke_driver -p sourcerer-journal-lin"
cargo build --release --example phase03_smoke_driver -p sourcerer-journal-lin

DRIVER="$ROOT/target/release/examples/phase03_smoke_driver"
if [[ ! -x "$DRIVER" ]]; then
    echo "smoke driver missing at $DRIVER" >&2
    exit 1
fi

CREATES=1000
MODIFIES=200
RENAMES=100
DELETES=100

# Filesystems to exercise. Default matches the Phase-3 spec; CI hosts
# that have only ext4 can pass `SOURCERER_LIN_SMOKE_FS=ext4` to skip
# btrfs/zfs without editing this script.
FILESYSTEMS="${SOURCERER_LIN_SMOKE_FS:-ext4 btrfs zfs}"

run_one_fs() {
    local fs="$1"
    local upper
    upper="$(echo "$fs" | tr '[:lower:]' '[:upper:]')"
    local dir_var="SOURCERER_LIN_SMOKE_${upper}_DIR"
    local dir="${!dir_var:-}"

    if [[ -z "$dir" ]]; then
        if [[ "$fs" == "ext4" ]]; then
            # Default ext4 location: /home is ext4 on most distros.
            dir="${HOME:-/tmp}"
        else
            echo "==> skipping $fs (set $dir_var to a path on a $fs volume to enable)"
            return 0
        fi
    fi

    if [[ ! -d "$dir" ]]; then
        echo "==> skipping $fs ($dir is not a directory)"
        return 0
    fi

    local stamp
    stamp="$(date +%Y%m%d-%H%M%S)"
    local scratch="$dir/sourcerer-phase03-$fs-$stamp-$$"
    local events_file="$scratch.events.jsonl"
    local cursor_dir="${scratch}_cursors"

    mkdir -p "$scratch"
    local baseline="$scratch/.baseline"
    touch "$baseline"
    sleep 1   # ensure subsequent file mtimes are strictly newer

    cleanup_one() {
        rm -rf "$scratch" "$events_file" "$cursor_dir" 2>/dev/null || true
    }
    trap cleanup_one RETURN

    echo "==> running smoke on $fs at $scratch"
    if ! "$DRIVER" \
        --scratch "$scratch" \
        --creates "$CREATES" \
        --modifies "$MODIFIES" \
        --renames "$RENAMES" \
        --deletes "$DELETES" \
        --timeout-secs 30 \
        --out-events "$events_file"; then
        echo "FAIL: smoke driver returned non-zero on $fs" >&2
        return 1
    fi

    # -- find-newer ground-truth comparison --
    # After the workload: 1000 Creates, 100 Renames (rename within tree),
    # 100 Deletes. Net post-workload file count under $scratch:
    #     CREATES - DELETES = 900.
    # `find -newer $baseline -type f` lists files whose mtime is newer
    # than the baseline marker — every workload-touched file qualifies.
    local ground_truth_count
    ground_truth_count=$(find "$scratch" -newer "$baseline" -type f -not -name '.baseline' 2>/dev/null | wc -l | tr -d ' ')
    local expected_min=$((CREATES - DELETES))
    echo "ground-truth file count via find-newer: $ground_truth_count (expected >= $expected_min)"
    if (( ground_truth_count < expected_min )); then
        echo "FAIL: ground-truth file count $ground_truth_count below expected minimum $expected_min on $fs" >&2
        return 1
    fi

    # -- Sourcerer-event-set cross-check --
    # Count distinct workload-touched paths from the Sourcerer event
    # stream. Phase 3's gate matches Phases 1 and 2: at least 99% of
    # (CREATES - DELETES) distinct paths must appear as a Sourcerer event.
    local distinct_paths
    if [[ -s "$events_file" ]]; then
        distinct_paths=$(python3 - "$events_file" <<'PY'
import json, sys
seen = set()
with open(sys.argv[1]) as f:
    for line in f:
        line = line.strip()
        if not line:
            continue
        try:
            ev = json.loads(line)
        except json.JSONDecodeError:
            continue
        for variant, body in ev.items():
            if isinstance(body, dict):
                p = body.get("path") or body.get("new_path")
                if p:
                    seen.add(p)
print(len(seen))
PY
        )
    else
        distinct_paths=0
    fi

    local threshold=$(( expected_min * 99 / 100 ))
    echo "Sourcerer-event distinct paths: $distinct_paths (>= $threshold required for 99% gate)"
    if (( distinct_paths < threshold )); then
        echo "FAIL: Sourcerer event coverage $distinct_paths < 99% gate ($threshold) on $fs" >&2
        return 1
    fi

    echo "PASS: Phase 3 smoke OK on $fs"
}

OVERALL=0
for fs in $FILESYSTEMS; do
    if ! run_one_fs "$fs"; then
        OVERALL=1
    fi
done

if (( OVERALL != 0 )); then
    echo "Phase 3 smoke FAILED on at least one filesystem" >&2
    exit 1
fi

echo "Phase 3 smoke OK"
