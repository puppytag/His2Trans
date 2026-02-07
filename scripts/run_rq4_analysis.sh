#!/bin/bash
# Run all RQ4 analysis (Knowledge Base Ablation)

set -e

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
DATA_DIR="$SCRIPT_DIR/../data/rq4"

python3 "$SCRIPT_DIR/analyze_c2r_compilation_rate_ohos_test5.py" \
    --run-dir "$DATA_DIR/base_kb_only" \
    --run-ohos-tests

python3 "$SCRIPT_DIR/analyze_c2r_compilation_rate_ohos_test5.py" \
    --run-dir "$DATA_DIR/base_kb_sedimented" \
    --run-ohos-tests
