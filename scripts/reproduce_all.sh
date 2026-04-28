#!/usr/bin/env bash
#
# scripts/reproduce_all.sh
#
# 1-command reproduction of every numeric claim in paper.md.
# Intended for `bash scripts/reproduce_all.sh` from a fresh checkout.
#
# Stages:
#   1. Build the workspace in release mode.
#   2. Run the experiments harness (scripts/run_experiments.sh).
#   3. Render the paper tables (scripts/render_paper.sh).
#
# Output goes to data/reproduce_output/.
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/.." && pwd)"
cd "$ROOT_DIR"

echo "[reproduce_all] === bitrag-theorems reproduction ==="
echo "[reproduce_all] Repo: $ROOT_DIR"
echo "[reproduce_all] Date: $(date -u +%Y-%m-%dT%H:%M:%SZ)"

OUT_DIR="$ROOT_DIR/data/reproduce_output"
mkdir -p "$OUT_DIR"

echo "[reproduce_all] Stage 1/3: cargo build --release"
cargo build --workspace --release

echo "[reproduce_all] Stage 2/3: scripts/run_experiments.sh"
bash "$ROOT_DIR/scripts/run_experiments.sh" | tee "$OUT_DIR/experiments.log"

echo "[reproduce_all] Stage 3/3: scripts/render_paper.sh"
bash "$ROOT_DIR/scripts/render_paper.sh" | tee "$OUT_DIR/render_paper.log"

echo "[reproduce_all] === done ==="
echo "[reproduce_all] Artifacts in: $OUT_DIR"
