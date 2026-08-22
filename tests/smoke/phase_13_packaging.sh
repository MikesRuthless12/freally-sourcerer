#!/usr/bin/env bash
# Phase 13 smoke — packaging (macOS / Linux side).
#
# Two halves, because they fail differently:
#
#   1. The OS-agnostic preconditions (`phase_13_packaging.rs`) — changelog
#      notes for the shipping version, the four release legs, `bundle.targets`,
#      version agreement. These are cheap and always run.
#
#   2. The bundle shape itself, which needs a real `tauri build` and so is
#      opt-in. Set FREALLY_SMOKE_BUNDLE=1 to run it. Without a build there is
#      nothing to inspect, and pretending otherwise is how a release ships
#      missing a format nobody noticed was gone.
#
# Usage:
#   tests/smoke/phase_13_packaging.sh                    # preconditions only
#   FREALLY_SMOKE_BUNDLE=1 tests/smoke/phase_13_packaging.sh   # + real bundle
set -euo pipefail

cd "$(dirname "$0")/../.."
root="$PWD"

echo "phase_13_packaging: preconditions"
(cd apps/freally-ui/src-tauri && cargo test --test phase_13_packaging --locked --quiet)

if [ "${FREALLY_SMOKE_BUNDLE:-0}" != "1" ]; then
  echo "phase_13_packaging.sh: ok (bundle check skipped; set FREALLY_SMOKE_BUNDLE=1)"
  exit 0
fi

echo "phase_13_packaging: building bundles (this takes several minutes)"
(cd apps/freally-ui && pnpm tauri build)

bundle="$root/apps/freally-ui/src-tauri/target/release/bundle"
[ -d "$bundle" ] || { echo "no bundle directory at $bundle" >&2; exit 1; }

case "$(uname -s)" in
  Darwin) want=(dmg macos) ;;   # macos/ holds the .app the dmg wraps
  Linux)  want=(deb rpm appimage) ;;
  *)      echo "phase_13_packaging.sh runs on macOS and Linux; use the .ps1 on Windows" >&2; exit 1 ;;
esac

status=0
for fmt in "${want[@]}"; do
  # `find`, not a glob: the bundler names files with the version in them and
  # a bare `-d` check would pass on an empty directory left by a failed leg.
  if [ -n "$(find "$bundle/$fmt" -mindepth 1 -maxdepth 1 -print -quit 2>/dev/null)" ]; then
    echo "  ok       $fmt"
  else
    echo "  MISSING  $fmt" >&2
    status=1
  fi
done

# The updater refuses an unsigned artifact, and it does so silently — an
# install that never updates again looks exactly like one that is current.
if [ -z "$(find "$bundle" -name '*.sig' -print -quit)" ]; then
  echo "  MISSING  updater signatures (.sig) — is TAURI_SIGNING_PRIVATE_KEY set?" >&2
  status=1
else
  echo "  ok       updater signatures"
fi

[ "$status" -eq 0 ] || { echo "phase_13_packaging.sh: FAILED" >&2; exit 1; }
echo "phase_13_packaging.sh: ok"
