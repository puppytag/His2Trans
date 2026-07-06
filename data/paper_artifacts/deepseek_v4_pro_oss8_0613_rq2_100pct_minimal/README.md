# DeepSeek v4 Pro OSS8 Minimal Archive

This directory contains the minimal paper artifact for the eight open-source C projects used in the current His2Trans RQ2 results:

`urlparser,avl,ht,qsort,buffer,rgba,quadtree,genann`

The archive keeps only the files needed to reproduce the reported metrics: final Rust crates, RQ2 test templates, replay scripts, and verified result files. It excludes translation logs, agent traces, pipeline intermediate logs, `target/` directories, and temporary test copies.

## Replay

From the repository root:

```bash
bash data/paper_artifacts/deepseek_v4_pro_oss8_0613_rq2_100pct_minimal/reproduce.sh \
  /tmp/oss8_archive_reproduce
```

You can also call the Python entry directly:

```bash
python3 data/paper_artifacts/deepseek_v4_pro_oss8_0613_rq2_100pct_minimal/scripts/run_archived_oss8_metrics.py \
  --output-dir /tmp/oss8_archive_reproduce
```

The final summary is written to:

```text
/tmp/oss8_archive_reproduce/results/summary.md
```

## Verified Metrics

The final row follows the paper metric policy: macro average over project-level rates.

| Project | Incremental compilation | RQ2 test pass rate | Warnings | Raw unsafe | Required unsafe |
|---|---:|---:|---:|---:|---:|
| urlparser | 23/23 (100.00%) | 3/3 (100.00%) | 0 | 105/993 (10.57%) | 40/993 (4.03%) |
| avl | 75/75 (100.00%) | 2/2 (100.00%) | 9 | 116/1513 (7.67%) | 29/1513 (1.92%) |
| ht | 2/2 (100.00%) | 1/1 (100.00%) | 0 | 2/174 (1.15%) | 1/174 (0.57%) |
| qsort | 2/2 (100.00%) | 6/6 (100.00%) | 0 | 13/185 (7.03%) | 3/185 (1.62%) |
| buffer | 30/30 (100.00%) | 14/14 (100.00%) | 0 | 129/815 (15.83%) | 42/815 (5.15%) |
| rgba | 17/17 (100.00%) | 10/10 (100.00%) | 0 | 30/781 (3.84%) | 9/781 (1.15%) |
| quadtree | 27/27 (100.00%) | 4/4 (100.00%) | 0 | 104/974 (10.68%) | 48/974 (4.93%) |
| genann | 12/12 (100.00%) | 12/12 (100.00%) | 1 | 95/792 (11.99%) | 33/792 (4.17%) |
| **Paper macro average** | **100.00%** | **100.00%** | **10** | **8.59%** | **2.94%** |

The warnings column uses `rustc_warning_count`; `results/warnings.json` also preserves clippy warning counts and combined warning counts.

## Directory Layout

- `projects/<project>/`: eight final Rust crates.
- `tests/source_rq2_tests/<project>/c2r/test.rs`: RQ2 test templates.
- `scripts/run_archived_oss8_metrics.py`: archive replay entry.
- `scripts/run_oss_rq2_rust_tests.py`: RQ2 harness adaptation and real Rust tests.
- `scripts/verify_incremental_compilation.py` and `scripts/ohos_incremental_core.py`: per-function incremental compilation replay.
- `scripts/analyze_required_unsafe.py`: raw unsafe and required unsafe replay.
- `results/`: verified replay results shipped with the archive.

## Integrity Check

```bash
cd data/paper_artifacts/deepseek_v4_pro_oss8_0613_rq2_100pct_minimal
sha256sum -c SHA256SUMS --quiet
```
