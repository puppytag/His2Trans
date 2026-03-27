#!/usr/bin/env bash
set -euo pipefail

# =============================================================================
# C2Rust baseline (OHOS test5): run c2rust transpile for the 5 "testable" modules
#
# Output is a directory of c2rust-converted crates (one crate per project).
# We intentionally write to a NEW directory so it won't conflict with older OHOS(5) outputs.
#
# Projects:
#   - appverify_lite__e5ebe91a98b9
#   - host__25c1898e1626
#   - osal__0bc4f21396ad
#   - shared__12e38ea922f7
#   - shared__541f4e547bdb
# =============================================================================

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
REPO_ROOT="$(cd "$SCRIPT_DIR/../../.." && pwd)" # .../c2-rust_framework

# Reuse the existing conversion driver (wraps `c2rust transpile` + Cargo.toml generation).
CONVERTER="$REPO_ROOT/ComparisonMethod/c2saferrust/convert_test_module.sh"

# Default: align with C2SaferRust OHOS(test5) conversion cache dir (safe to reuse).
OUT_DIR_DEFAULT="$REPO_ROOT/ComparisonMethod/c2saferrust/ohos_converted_test5"
OUT_DIR="${OUT_DIR:-$OUT_DIR_DEFAULT}"

OHOS_BASE_WITH_TEST="${OHOS_BASE_WITH_TEST:-$REPO_ROOT/SelfContained/self_contained_modules_v2/with_third_party/others/with_test}"
OHOS_BASE_SRC_TEST_NO_INCLUDE="${OHOS_BASE_SRC_TEST_NO_INCLUDE:-$REPO_ROOT/SelfContained/self_contained_modules_v2/src_test_no_include/others/with_test}"

PROJECTS_DEFAULT="appverify_lite__e5ebe91a98b9,host__25c1898e1626,osal__0bc4f21396ad,shared__12e38ea922f7,shared__541f4e547bdb"
PROJECTS="$PROJECTS_DEFAULT"

usage() {
  cat <<EOF
Run c2rust transpile for OHOS(test5) projects (baseline).

Usage:
  bash $0 [--projects a,b,c] [--out-dir DIR]

Options:
  --projects LIST   Comma-separated project list (default: $PROJECTS_DEFAULT)
  --out-dir DIR     Output directory (default: $OUT_DIR_DEFAULT)
  --help, -h        Show this help

Environment:
  OHOS_BASE_WITH_TEST=...            (default: $OHOS_BASE_WITH_TEST)
  OHOS_BASE_SRC_TEST_NO_INCLUDE=...  (default: $OHOS_BASE_SRC_TEST_NO_INCLUDE)
  OUT_DIR=...                        (same as --out-dir)
EOF
}

while [[ $# -gt 0 ]]; do
  case "$1" in
    --projects) PROJECTS="$2"; shift 2 ;;
    --out-dir) OUT_DIR="$2"; shift 2 ;;
    --help|-h) usage; exit 0 ;;
    *) echo "Unknown option: $1" >&2; usage >&2; exit 2 ;;
  esac
done

if [[ ! -x "$CONVERTER" ]]; then
  echo "ERROR: converter not found or not executable: $CONVERTER" >&2
  exit 2
fi

declare -A SRC_DIR_MAP=(
  ["appverify_lite__e5ebe91a98b9"]="$OHOS_BASE_WITH_TEST/appverify_lite__e5ebe91a98b9"
  ["host__25c1898e1626"]="$OHOS_BASE_WITH_TEST/host__25c1898e1626"
  ["osal__0bc4f21396ad"]="$OHOS_BASE_SRC_TEST_NO_INCLUDE/osal__0bc4f21396ad"
  ["shared__12e38ea922f7"]="$OHOS_BASE_WITH_TEST/shared__12e38ea922f7"
  ["shared__541f4e547bdb"]="$OHOS_BASE_WITH_TEST/shared__541f4e547bdb"
)

echo "=============================================="
echo "C2Rust baseline: OHOS(test5) c2rust transpile"
echo "=============================================="
echo "[config] Output:    $OUT_DIR"
echo "[config] Projects:  $PROJECTS"
echo "=============================================="

mkdir -p "$OUT_DIR"

IFS=',' read -ra PROJECT_LIST <<< "$PROJECTS"
for PROJECT in "${PROJECT_LIST[@]}"; do
  PROJECT="$(echo "$PROJECT" | xargs)"
  SRC_DIR="${SRC_DIR_MAP[$PROJECT]:-}"
  if [[ -z "$SRC_DIR" ]]; then
    echo "[SKIP] Unknown project: $PROJECT"
    continue
  fi
  if [[ ! -d "$SRC_DIR" ]]; then
    echo "[FAIL] Missing source dir: $SRC_DIR"
    continue
  fi

  echo ""
  echo "----------------------------------------------"
  echo "[RUN] $PROJECT"
  echo "  src: $SRC_DIR"
  echo "  out: $OUT_DIR/$PROJECT"
  echo "----------------------------------------------"

  # Convert each project independently because the dataset lives in mixed base dirs.
  bash "$CONVERTER" \
    --base-dir "$(dirname "$SRC_DIR")" \
    --output-dir "$OUT_DIR" \
    --ohos-compile-commands \
    --projects "$PROJECT"
done

echo ""
echo "=============================================="
echo "Done. Converted crates: $OUT_DIR"
echo "Run analysis with:"
echo "  python3 $REPO_ROOT/ComparisonMethod/test_module/rust_tests/analyze_c2rust_compilation_ohos_test5.py --base-dir \"$OUT_DIR\" --all"
echo "=============================================="

