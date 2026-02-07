#!/bin/bash
# Run all RQ1 analysis

set -e

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
DATA_DIR="$SCRIPT_DIR/../data/rq1"

python3 "$SCRIPT_DIR/analyze_c2r_compilation_rate_ohos_test5.py" \
    --run-dir "$DATA_DIR/k1" \
    --all

python3 "$SCRIPT_DIR/analyze_c2r_compilation_rate_ohos_test5.py" \
    --run-dir "$DATA_DIR/k3" \
    --all

python3 "$SCRIPT_DIR/analyze_c2r_compilation_rate_ohos_test5.py" \
    --run-dir "$DATA_DIR/k5" \
    --all

python3 "$SCRIPT_DIR/analyze_c2r_compilation_rate_ohos_test5.py" \
    --run-dir "$DATA_DIR/k10" \
    --all

python3 "$SCRIPT_DIR/analyze_c2r_compilation_rate_ohos_test5.py" \
    --run-dir "$DATA_DIR/claude" \
    --all --verify-incremental
