# His2Trans

His2Trans is a knowledge-guided agentic framework for project-level C-to-Rust migration. It builds a Rust scaffold for the target C project, translates function bodies with retrieved historical migration knowledge, and then refines the assembled crate with compilation, semantic, and unsafe-code feedback.

This repository contains the framework implementation and the minimal paper-aligned artifacts needed to reproduce the current reported metrics.

## Repository Layout

- `framework/`: the C-to-Rust migration pipeline. The main entry is `framework/batch_test_staged.sh`.
- `data/ohos/`: compact OpenHarmony headers and five small source-project bundles used by the lightweight framework examples.
- `data/test_module_rust_tests/`: Rust test harnesses for the open-source RQ2 project set.
- `data/paper_artifacts/`: minimal final translated Rust crates, metric scripts, and verified result files for the current paper.
- `data/paper_metric_exports/`: paper-aligned reference tables and generated metric summaries.
- `scripts/export_current_plot_metrics.py`: regenerates the paper metric CSV/JSON/Markdown summaries from the shipped artifacts.
- `tests/`: lightweight tests for the paper metric helper code.

## Paper-Aligned Results

The current paper uses ten OpenHarmony modules and eight open-source C projects.

| Dataset | Incremental compilation | Test pass rate | Unsafe ratio | Required unsafe ratio |
|---|---:|---:|---:|---:|
| OpenHarmony 10 modules | 100.00% | 94.92% | 16.35% | 8.48% |
| OSS8 projects | 100.00% | 100.00% | 8.59% | 2.94% |

The OpenHarmony ablation table is:

| Setting | Incremental compilation | Test pass rate | Unsafe ratio |
|---|---:|---:|---:|
| Knowledge-guided function translation | 95.82% | 39.29% | 15.95% |
| With compiler-driven per-function repair | 100.00% | 94.92% | 22.52% |
| With project-level agentic refinement | 100.00% | 94.92% | 16.35% |

Regenerate the repository-local metric exports with:

```bash
python3 scripts/export_current_plot_metrics.py
```

This rewrites:

- `data/paper_metric_exports/current_plot_metrics_alignment.json`
- `data/paper_metric_exports/current_plot_metrics_alignment.md`
- `data/paper_metric_exports/reference_tables/*.csv`
- `data/paper_metric_exports/generated_structured_json/rq1_ohos10_method_comparison.json`
- `data/paper_metric_exports/generated_structured_json/rq2_oss8_method_comparison.json`
- `data/paper_metric_exports/generated_structured_json/rq3_ohos10_ablation.json`
- `data/paper_metric_exports/generated_structured_json/rq4_case_evidence.json`

The export script checks the key numbers above and fails if they drift.

## Reproducing Archived Metrics

### OpenHarmony 10-module archive

The OHOS10 archive is:

```text
data/paper_artifacts/deepseek_v4_pro_ohos10_0613_harness_fixed_v2/
```

Fast metric replay:

```bash
python3 data/paper_artifacts/deepseek_v4_pro_ohos10_0613_harness_fixed_v2/scripts/run_archived_ohos_metrics.py \
  --output-dir /tmp/his2trans_ohos10_metrics
```

This recomputes incremental-compilation and unsafe metrics from the archived Rust crates, and reuses the verified gtest result files shipped in the archive. Re-running the full OHOS C gtest bridge requires the original full OpenHarmony/self-contained source tree, which is too large for this minimal open-source artifact.

### OSS8 archive

The OSS8 archive is:

```text
data/paper_artifacts/deepseek_v4_pro_oss8_0613_rq2_100pct_minimal/
```

Full metric replay:

```bash
bash data/paper_artifacts/deepseek_v4_pro_oss8_0613_rq2_100pct_minimal/reproduce.sh \
  /tmp/his2trans_oss8_metrics
```

This reruns the archived OSS8 Rust tests, incremental compilation, unsafe analysis, and warning counting from the shipped final crates.

## Environment

The paper experiments were run on Ubuntu Linux with Clang 14, Rust nightly, and Python 3.13. The framework environment used Python 3.11 through the `c2r_frame` conda environment.

Required system tools:

- Python 3.10+ for framework scripts; Python 3.13 was used for the final paper metric scripts.
- Rust nightly with `cargo` and `clippy`.
- Clang and libclang.
- Conda, if you want to use the provided framework environment setup.

Set up the framework conda environment:

```bash
bash framework/setup_conda_env.sh
export CONDA_OVERRIDE_CUDA=
export CONDA_NO_PLUGINS=true
conda activate c2r_frame
export NLTK_DATA="$(pwd)/framework/data/nltk_data"
```

For external-API translation runs:

```bash
export RUSTUP_TOOLCHAIN=nightly
export USE_VLLM=false
export EXTERNAL_API_BASE_URL="https://api.deepseek.com/beta"
export EXTERNAL_API_MODEL="deepseek-coder"
export EXTERNAL_API_KEY="YOUR_KEY"
export USE_PREPROCESSING=false
export HF_HOME="$(pwd)/framework/data/my-huggingface"
export TRANSFORMERS_CACHE="$HF_HOME"
export HF_HUB_CACHE="$HF_HOME/hub"
```

## Framework Smoke Run

Run a single shipped OpenHarmony example without RAG or learned-KB retrieval:

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

Outputs are written to `framework/translation_outputs/<run-dir>/`.

## Development Checks

Run the lightweight metric tests:

```bash
python3 -m pytest -q tests
```

Compile-check the Python sources:

```bash
python3 -m py_compile \
  scripts/export_current_plot_metrics.py \
  framework/*.py \
  framework/generate/*.py \
  framework/scripts/*.py \
  framework/scripts/agentic_repair/*.py
```

## Notes on Large External Data

The repository intentionally does not ship the full OpenHarmony source tree, full `compile_commands.json`, model weights, or generated `target/` directories. The final translated crates and verified paper metric evidence needed for the current tables are included under `data/paper_artifacts/`.
