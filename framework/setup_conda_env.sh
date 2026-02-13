#!/usr/bin/env bash
#
# One-click environment bootstrap (matches the original conda-based workflow).
#
# Usage:
#   bash framework/setup_conda_env.sh
#
# Notes:
# - We disable conda plugins by default because some plugins (e.g. CUDA probing)
#   may crash on certain machines / restricted environments.
# - NLTK corpora are downloaded into framework/data/nltk_data so the repo can run
#   without writing to ~/.nltk_data. (You still need network for the first download.)

set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
ENV_YML="$SCRIPT_DIR/environment.yml"

if [[ ! -f "$ENV_YML" ]]; then
  echo "ERROR: environment.yml not found: $ENV_YML" >&2
  exit 2
fi

if ! command -v conda >/dev/null 2>&1; then
  echo "ERROR: conda not found in PATH. Please install Miniconda/Anaconda first." >&2
  exit 2
fi

# Make conda more robust in restricted environments.
export CONDA_NO_PLUGINS="${CONDA_NO_PLUGINS:-true}"
export CONDA_NO_NOTICES="${CONDA_NO_NOTICES:-true}"

# shellcheck disable=SC1091
eval "$(conda shell.bash hook 2>/dev/null)" || {
  # Fallback for older conda setups.
  conda_base="$(conda --no-plugins info --base 2>/dev/null || true)"
  if [[ -n "$conda_base" ]] && [[ -f "$conda_base/etc/profile.d/conda.sh" ]]; then
    # shellcheck disable=SC1091
    source "$conda_base/etc/profile.d/conda.sh"
  fi
}

ENV_NAME="$(sed -n 's/^name:[[:space:]]*//p' "$ENV_YML" | head -n 1 | tr -d '\r')"
ENV_NAME="${ENV_NAME:-c2r_frame}"

echo "==> CONDA_NO_PLUGINS=$CONDA_NO_PLUGINS"
echo "==> CONDA_NO_NOTICES=$CONDA_NO_NOTICES"
echo "==> Env file: $ENV_YML"
echo "==> Env name: $ENV_NAME"

echo "==> Checking if env exists..."
if conda --no-plugins run -n "$ENV_NAME" python -c "import sys; print(sys.executable)" >/dev/null 2>&1; then
  echo "==> Env exists: $ENV_NAME"
  echo "==> Updating env (best-effort)..."
  conda --no-plugins env update --solver classic -n "$ENV_NAME" -f "$ENV_YML" --prune || {
    echo "WARNING: conda env update failed (network/offline?). Continuing with existing env." >&2
  }
else
  echo "==> Env not found; creating..."
  conda --no-plugins env create --solver classic -f "$ENV_YML"
fi

echo "==> Activating env..."
conda activate "$ENV_NAME"

# Keep NLTK data inside the repo.
NLTK_DATA_DIR="$SCRIPT_DIR/data/nltk_data"
mkdir -p "$NLTK_DATA_DIR"

echo "==> Downloading required NLTK corpora into: $NLTK_DATA_DIR"
python3 - <<PY
import nltk
pkgs = ["stopwords", "wordnet", "omw-1.4"]
ok = []
failed = []
for p in pkgs:
    try:
        success = nltk.download(p, download_dir=r"$NLTK_DATA_DIR", quiet=True)
        if success:
            ok.append(p)
        else:
            failed.append(p)
    except Exception:
        failed.append(p)
print("NLTK OK:", ", ".join(ok))
if failed:
    print("NLTK WARNING: failed to download:", ", ".join(failed))
PY

cat <<EOF

Done.

Next:
  export CONDA_NO_PLUGINS=true
  conda activate $ENV_NAME

Recommended (so NLTK corpora are found):
  export NLTK_DATA="$NLTK_DATA_DIR"

EOF
