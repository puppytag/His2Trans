#!/usr/bin/env bash
# 用归档的 OSS8 最小子集一键复现论文指标。

set -euo pipefail

ARCHIVE_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
REPO_ROOT="$(cd "${ARCHIVE_DIR}/../../../.." && pwd)"
OUTPUT_DIR="${1:-/tmp/oss8_archive_reproduce}"

cd "${REPO_ROOT}"
export PYTHONDONTWRITEBYTECODE=1
python3 "${ARCHIVE_DIR}/scripts/run_archived_oss8_metrics.py" \
  --output-dir "${OUTPUT_DIR}"
