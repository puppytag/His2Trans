#!/bin/bash
# Run all RQ3 analysis (Ablation Study)

set -e

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
DATA_DIR="$SCRIPT_DIR/../data/rq3"

CONDITIONS=("c0:Base-1Shot" "c1:Base-Rep" "c2:Pred-1Shot" "c3:Pred-Rep" "c4:GT-API" "c5:GT-Frag" "c6:GT-Full")

for cond in "${CONDITIONS[@]}"; do
    IFS=':' read -r dir name <<< "$cond"
    python3 "$SCRIPT_DIR/analyze_c2r_compilation_rate_ohos_test5.py" \
        --run-dir "$DATA_DIR/$dir" \
        --run-ohos-tests
done
