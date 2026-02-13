# his2trans

## Included

- `framework/`: end-to-end C-to-Rust translation pipeline (`batch_test_staged.sh`).
- `data/ohos/source_projects/`: 5 minimal OHOS projects for the `ohos` (test5) suite, each with a relocatable `compile_commands.json`.
- `data/ohos/ohos_root_min/`: a minimal OpenHarmony header tree used for include resolution / bindgen.
- `framework/workspace/rag/`: the base RAG knowledge base (`knowledge_base.json` + `bm25_index.pkl`) and precomputed reranked results.
- `scripts/` + `data/rq{1,2,3,4}/`: paper analysis scripts and minimal inputs (RQ1-RQ4).


## Tested environment (reference)

The commands below are tested on:

- OS: Ubuntu Linux
- Python (framework): 3.11.x (via conda env `c2r_frame`)
- Rust: **nightly** toolchain
- Clang + libclang: 14.x (`clang`, `libclang-dev`)

## Prerequisites

### What you need to download/install (external dependencies)

System tools (required):
- Python 3.8+ (paper analysis). Python 3.10+ recommended for the framework.
- Rust toolchain: `rustc`, `cargo`, `clippy` (recommend installing via `rustup`). **Rust nightly is recommended/required**.
- C/C++ toolchain: `clang` (or `gcc`) for preprocessing and bindgen-related steps.
- `libclang` runtime (required when using `--use-libclang`).
- Conda (`conda` command in PATH) to use the provided one-click environment setup.

Network downloads (only needed for some modes):
- Conda packages + pip wheels (when creating the framework env).
- NLTK corpora: `stopwords`, `wordnet`, `omw-1.4` (downloaded into `framework/data/nltk_data`).
- External LLM API access (when `USE_VLLM=false`).
- HuggingFace model weights (when `--run-rag true` and you enable the Jina reranker).

### Example installation commands (Ubuntu/Debian)

Install system packages:
```bash
sudo apt-get update
sudo apt-get install -y \
  build-essential \
  clang \
  libclang-dev \
  pkg-config \
  cmake \
  python3-venv
```

Install Rust via `rustup` (downloads the toolchain). Use **nightly**:
```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
source "$HOME/.cargo/env"
rustup toolchain install nightly
rustup default nightly
rustup component add clippy
```

Install Miniconda/Anaconda so `conda` is available, then follow the conda setup below.

### Python deps

- Paper analysis (RQ1–RQ4): standard library only.
- Framework: use the provided conda environment (recommended).

#### Option A (recommended): one-click conda env (full features)

```bash
# Create/update conda env from framework/environment.yml (downloads packages).
bash framework/setup_conda_env.sh

# Activate the env.
#
# If your conda crashes with PermissionError / CUDA probing issues, this workaround helps:
# (it disables conda's CUDA virtual-package probing which may spawn processes and fail in restricted envs)
export CONDA_OVERRIDE_CUDA=
export CONDA_NO_PLUGINS=true
conda activate c2r_frame

# Ensure NLTK corpora are found (the setup script downloads them into this folder).
export NLTK_DATA="$(pwd)/framework/data/nltk_data"
```

If you run with `--run-rag true`, the reranker step uses `torch` + `transformers` and may download model weights.

#### Option B: Python venv + pip (external API, no RAG)

If you only want to run the framework in external API mode (recommended smoke run uses `--run-rag false` and `--skip-learned-kb`),
you can avoid conda and install a minimal set of Python deps:

```bash
python3 -m venv .venv
source .venv/bin/activate
python -m pip install -U pip
python -m pip install -r requirements.txt

# Download NLTK corpora into the repo-local folder
python - <<'PY'
import nltk, os
dst = os.path.abspath("framework/data/nltk_data")
os.makedirs(dst, exist_ok=True)
for p in ["stopwords", "wordnet", "omw-1.4"]:
    nltk.download(p, download_dir=dst, quiet=True)
print("NLTK_DATA =", dst)
PY

export NLTK_DATA="$(pwd)/framework/data/nltk_data"
```

## Paper Analysis (RQ1–RQ4)

All scripts only print the paper metrics to the terminal.

```bash
cd scripts

# Run everything (RQ1–RQ4)
bash run_all_analysis.sh

# Or run per RQ
bash run_rq1_analysis.sh
bash run_rq2_analysis.sh
bash run_rq3_analysis.sh
bash run_rq4_analysis.sh
```

## Framework Run (External API)

This is the end-to-end pipeline. It will generate output under `framework/translation_outputs/<run-dir>/`.

### 1) Set environment variables

```bash
# Recommended: force Rust nightly without changing global toolchain.
export RUSTUP_TOOLCHAIN=nightly

# Use external LLM API instead of local vLLM.
export USE_VLLM=false
export EXTERNAL_API_BASE_URL="https://api.deepseek.com/beta"
export EXTERNAL_API_MODEL="deepseek-coder"
export EXTERNAL_API_KEY="YOUR_KEY"

# Optional: avoid accidentally using any host-local OpenHarmony compile DB.
export USE_PREPROCESSING=false

# Optional (recommended): keep HuggingFace cache inside this folder (used by reranker).
export HF_HOME="$(pwd)/framework/data/my-huggingface"
export TRANSFORMERS_CACHE="$HF_HOME"
export HF_HUB_CACHE="$HF_HOME/hub"
```

### 2) Run (full command)

```bash
cd framework
bash batch_test_staged.sh \
  --layered --incremental --max-repair 5 \
  --max-parallel 20 \
  --run-rag true --jina-parallel --use-libclang \
  --bindgen-debug-keep-files \
  --vllm-global-limit 120 \
  --suite ohos \
  --run-dir deepseek-coder-ohos10
```

Outputs are written under: `framework/translation_outputs/<run-dir>/`.

### 3) Quick smoke run (recommended first)

If you just want to validate the pipeline end-to-end on a single shipped project:

```bash
cd framework
bash batch_test_staged.sh \
  --layered --incremental --max-repair 1 \
  --max-parallel 1 --max-parallel-workers 1 \
  --run-rag false --skip-learned-kb --use-libclang \
  --suite ohos \
  --only osal__0bc4f21396ad \
  --run-dir smoke_api
```

## Optional Large External Resources (Not Shipped)

- **Full OpenHarmony source tree + full `compile_commands.json`** (only needed if you want preprocessing/type recovery to use the full build context):
  - Set `USE_PREPROCESSING=true`
  - Provide `OHOS_ROOT=/path/to/OpenHarmony` and `OHOS_COMPILE_COMMANDS=/path/to/compile_commands.json`
- **Jina reranker weights** (only needed when `--run-rag true`):
  - Model id: `jinaai/jina-reranker-v3`
  - The framework caches downloads under `framework/data/my-huggingface/` (via `HF_HOME`).

## What to run (cheat sheet)

Paper tables (RQ1-RQ4):
```bash
cd scripts
bash run_all_analysis.sh
```

Framework (single project smoke run, external API):
```bash
# 1) setup env (one-time)
bash framework/setup_conda_env.sh
export CONDA_OVERRIDE_CUDA=
export CONDA_NO_PLUGINS=true
conda activate c2r_frame
export NLTK_DATA="$(pwd)/framework/data/nltk_data"

# 2) set API env vars
export RUSTUP_TOOLCHAIN=nightly
export USE_VLLM=false
export EXTERNAL_API_BASE_URL="https://api.deepseek.com/beta"
export EXTERNAL_API_MODEL="deepseek-coder"
export EXTERNAL_API_KEY="YOUR_KEY"

# 3) run
cd framework
bash batch_test_staged.sh \
  --layered --incremental --max-repair 1 \
  --max-parallel 1 --max-parallel-workers 1 \
  --run-rag false --skip-learned-kb --use-libclang \
  --suite ohos \
  --only osal__0bc4f21396ad \
  --run-dir smoke_api
```
