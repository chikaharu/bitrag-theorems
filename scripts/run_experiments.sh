#!/usr/bin/env bash
#
# scripts/run_experiments.sh
#
# Runs the four added paper experiments (Exp 4 — Exp 7) plus the failure-case
# Exp 8.  All experiments are deterministic: every seed flows through
# `crate::prng::XorShift64`, so reproductions are byte-identical across hosts.
#
# This script just wraps `cargo test --release` for the relevant test
# binaries and captures their output.
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/.." && pwd)"
cd "$ROOT_DIR"

OUT_DIR="$ROOT_DIR/data/reproduce_output"
mkdir -p "$OUT_DIR"

run_test() {
    local name="$1"
    echo "[run_experiments] cargo test --release --test $name"
    cargo test --workspace --release --test "$name" -- --nocapture \
        > "$OUT_DIR/${name}.log" 2>&1
    echo "[run_experiments]   -> wrote $OUT_DIR/${name}.log"
}

echo "[run_experiments] === Experiments 4–8 ==="
run_test exp4_corpus4
run_test exp5_corpus5
run_test exp_xi_asymptotic
run_test exp_jl_bound
run_test exp_failure_cases

echo "[run_experiments] === Lib unit tests ==="
cargo test --workspace --release --lib -- --nocapture \
    > "$OUT_DIR/lib_tests.log" 2>&1

echo "[run_experiments] === Bench (single-run) ==="
cargo run --release --bench scaling_bench --quiet \
    > "$OUT_DIR/scaling_bench.csv" 2>&1 || \
    echo "[run_experiments] WARNING: bench did not run (this is non-fatal in CI)"

echo "[run_experiments] === Done ==="
ls -1 "$OUT_DIR"
