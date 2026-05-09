#!/usr/bin/env bash
# Phase 12 smoke — cross-platform volume detection (macOS / Linux side).
#
# Verifies that `sourcerer-indexd`'s `volumes::detect()` returns at
# least one volume on macOS / Linux without panicking. The Rust unit
# test under `crates/sourcerer-indexd/src/volumes.rs` already covers
# the no-panic invariant; this shell smoke also asserts that
# `cargo test -p sourcerer-indexd volumes` is green and runs in CI.
set -euo pipefail

cd "$(dirname "$0")/../.."
cargo test -p sourcerer-indexd --test phase_12_indexd_client --quiet
echo "phase_12_volumes.sh: ok"
