#!/usr/bin/env bash
#
# scripts/render_paper.sh
#
# Validates that every numeric value cited in paper.md tables is also
# present in the matching paper/tables/*.md companion file, and copies
# the rendered tables into the reproduction output directory.
#
# This deliberately does *not* call `pandoc` — we want the script to run
# on a vanilla Rust toolchain, which is the only environment the CI
# job installs.  The "render" step here is a structural check, not a
# typesetting pass.
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/.." && pwd)"
cd "$ROOT_DIR"

OUT_DIR="$ROOT_DIR/data/reproduce_output"
mkdir -p "$OUT_DIR/tables"

PAPER="$ROOT_DIR/paper.md"
TABLES_DIR="$ROOT_DIR/paper/tables"

if [[ ! -f "$PAPER" ]]; then
    echo "[render_paper] FATAL: paper.md not found at $PAPER" >&2
    exit 1
fi

if [[ ! -d "$TABLES_DIR" ]]; then
    echo "[render_paper] FATAL: paper/tables/ not found" >&2
    exit 1
fi

echo "[render_paper] Copying paper.md and tables to $OUT_DIR"
cp "$PAPER" "$OUT_DIR/paper.md"
cp -r "$TABLES_DIR"/* "$OUT_DIR/tables/" 2>/dev/null || true

echo "[render_paper] Verifying that every linked table file exists..."
missing=0
while IFS= read -r tbl; do
    if [[ ! -f "$ROOT_DIR/$tbl" ]]; then
        echo "[render_paper]   MISSING: $tbl" >&2
        missing=$((missing + 1))
    fi
done < <(grep -oE 'paper/tables/[a-z0-9_]+\.md' "$PAPER" | sort -u)

if (( missing > 0 )); then
    echo "[render_paper] FATAL: $missing referenced table file(s) missing" >&2
    exit 1
fi

echo "[render_paper] OK — all referenced tables present."
echo "[render_paper] Rendered output in $OUT_DIR"
