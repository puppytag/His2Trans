#!/bin/bash
# Run all analysis scripts (RQ1-RQ4)

set -e

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"

# Run RQ1
bash "$SCRIPT_DIR/run_rq1_analysis.sh"

# Run RQ2
bash "$SCRIPT_DIR/run_rq2_analysis.sh"

# Run RQ3
bash "$SCRIPT_DIR/run_rq3_analysis.sh"

# Run RQ4
bash "$SCRIPT_DIR/run_rq4_analysis.sh"
