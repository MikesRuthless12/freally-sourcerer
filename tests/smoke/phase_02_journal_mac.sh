#!/usr/bin/env bash
# Phase 2 smoke (macOS).
#
# Builds the Phase-2 smoke driver, runs the FSEvents subscriber against a
# scratch directory under $HOME, performs the spec workload, and asserts
# observed event counts stay within 1% of the expected set (the spec's
# "overlap >= 99%" gate). After the driver finishes, the harness also
# walks the post-workload tree with `find -newer` for a sanity-check
# ground-truth comparison.
#
# Skipped on non-macOS hosts so CI on Windows / Linux runs other smokes.

set -euo pipefail

if [[ "$(uname -s)" != "Darwin" ]]; then
    echo "phase_02_journal_mac.sh: skipping on non-macOS host ($(uname -s))"
    exit 0
fi

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
cd "$ROOT"

echo "==> cargo build --release --example phase02_smoke_driver -p sourcerer-journal-mac"
cargo build --release --example phase02_smoke_driver -p sourcerer-journal-mac

DRIVER="$ROOT/target/release/examples/phase02_smoke_driver"
if [[ ! -x "$DRIVER" ]]; then
    echo "smoke driver missing at $DRIVER" >&2
    exit 1
fi

stamp="$(date +%Y%m%d-%H%M%S)"
scratch="${TMPDIR:-/tmp}/sourcerer-phase02-$stamp"
mkdir -p "$scratch"
events_file="$scratch.events.jsonl"
# The smoke driver writes the cursor to a sibling dir
# (`<scratch>_cursors`) — outside the watched root — so cursor saves
# don't generate self-loop FSEvents events that would inflate counts.
cursor_dir="${scratch}_cursors"

# Baseline marker for the find-newer ground-truth comparison.
baseline="$scratch/.baseline"
touch "$baseline"
sleep 1   # ensure subsequent file mtimes are strictly newer

cleanup() {
    rm -rf "$scratch" "$events_file" "$cursor_dir"
}
trap cleanup EXIT

CREATES=1000
MODIFIES=200
RENAMES=100
DELETES=100

echo "==> running smoke against $scratch"
"$DRIVER" \
    --scratch "$scratch" \
    --creates "$CREATES" \
    --modifies "$MODIFIES" \
    --renames "$RENAMES" \
    --deletes "$DELETES" \
    --timeout-secs 30 \
    --out-events "$events_file"

# -- find-newer ground-truth comparison ---------------------------------
# After the workload: 1000 Creates, 100 Renames (move within tree),
# 100 Deletes. Net post-workload file count under $scratch should be:
#     CREATES - DELETES = 900
# We compare with `find -newer $baseline -type f` which lists files
# whose mtime is newer than the baseline — every workload-touched file
# qualifies (creates + modifies + the rename-targets). The exact count
# depends on whether the renamed-target's mtime survived the rename
# (it does on APFS), so we expect at least CREATES - DELETES.

ground_truth_count=$(find "$scratch" -newer "$baseline" -type f -not -name '.baseline' 2>/dev/null | wc -l | tr -d ' ')
expected_min=$((CREATES - DELETES))
echo "ground-truth file count via find-newer: $ground_truth_count (expected >= $expected_min)"
if (( ground_truth_count < expected_min )); then
    echo "FAIL: ground-truth file count $ground_truth_count is below expected minimum $expected_min" >&2
    exit 1
fi

# -- Sourcerer-event-set cross-check ------------------------------------
# Count distinct workload-touched paths from the Sourcerer event stream.
# The Phase-2 acceptance gate is 99%: at least 99% of (CREATES - DELETES)
# distinct paths should appear as a Sourcerer event.

if [[ -s "$events_file" ]]; then
    # Use python for line-oriented JSON parsing — preinstalled on macOS.
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

threshold=$(( expected_min * 99 / 100 ))
echo "Sourcerer-event distinct paths: $distinct_paths (>= $threshold required for 99% gate)"
if (( distinct_paths < threshold )); then
    echo "FAIL: Sourcerer event coverage $distinct_paths < 99% gate ($threshold)" >&2
    exit 1
fi

echo "Phase 2 smoke OK"
