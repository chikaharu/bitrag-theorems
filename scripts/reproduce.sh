#!/usr/bin/env bash
# Reproduce all four supplementary experiments of bitrag-theorems §4.
# Output goes to stdout in markdown. Deterministic for fixed PRNG seeds.

set -euo pipefail

cd "$(dirname "$0")/../experiments"

cargo build --release --bins --quiet

echo "<!-- Reproduced by scripts/reproduce.sh on $(date -u +%Y-%m-%dT%H:%M:%SZ) -->"
echo
./target/release/exp_corpus_4_5
echo
./target/release/exp_xi_asymptotic
echo
./target/release/exp_jl_bound
echo
./target/release/exp_failure_cases
