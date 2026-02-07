#!/bin/bash
# Run all RQ2 analysis

set -e

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
DATA_DIR="$SCRIPT_DIR/../data/rq2"

python3 "$SCRIPT_DIR/analyze_c2r_compilation_rate.py" \
    --run-dir "$DATA_DIR/deepseek" \
    --all

python3 "$SCRIPT_DIR/analyze_c2r_compilation_rate.py" \
    --run-dir "$DATA_DIR/claude" \
    --verify-incremental \
    --all
