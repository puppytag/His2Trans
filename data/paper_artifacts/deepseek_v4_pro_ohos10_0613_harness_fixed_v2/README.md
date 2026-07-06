# OHOS10 DeepSeek v4 Pro Archive

This directory contains the minimal paper artifact for the ten OpenHarmony modules used in the current His2Trans paper results. It includes the final translated Rust crates, metric scripts, and verified metric evidence.

## Contents

- `projects/`: ten final Rust crates from `deepseek-v4-pro-ohos10-full-0613-1`, with `target/` directories removed.
- `scripts/`: metric replay scripts and script-level regression tests.
- `results/`: verified gtest evidence, incremental-compilation results, unsafe-code results, warning inputs, and the final paper-metric table.
- `SHA256SUMS` and `file_list.txt`: integrity and file inventory records.

The full OpenHarmony source tree and full self-contained C test tree are not included in this minimal open-source artifact.

## Fast Replay

From the repository root:

```bash
python3 data/paper_artifacts/deepseek_v4_pro_ohos10_0613_harness_fixed_v2/scripts/run_archived_ohos_metrics.py \
  --output-dir /tmp/ohos10_archive_metrics
```

The default replay recomputes incremental-compilation and unsafe-code metrics from the archived Rust crates, then reuses the verified archived gtest evidence. Re-running the full OHOS C gtest bridge requires the full external OpenHarmony/self-contained source tree; use `--run-gtest` only in that environment.

## Script Tests

```bash
PYTHONPATH=data/paper_artifacts/deepseek_v4_pro_ohos10_0613_harness_fixed_v2/scripts \
python3 -m pytest -q \
  data/paper_artifacts/deepseek_v4_pro_ohos10_0613_harness_fixed_v2/scripts/tests
```

## Verified Metrics

The final row follows the paper metric policy: macro average over project-level rates.

| Project | Test pass rate | Incremental compilation | Warnings | Raw unsafe | Required unsafe |
|---|---:|---:|---:|---:|---:|
| host | 8/8=100.00% | 127/127=100.00% | 257 | 1630/7506=21.72% | 847/7506=11.28% |
| appverify_lite | 5/8=62.50% | 173/173=100.00% | 424 | 1833/11223=16.33% | 914/11223=8.14% |
| manager | 4/4=100.00% | 123/123=100.00% | 200 | 1568/6604=23.74% | 910/6604=13.78% |
| shared_541 | 5/5=100.00% | 27/27=100.00% | 52 | 382/3018=12.66% | 217/3018=7.19% |
| posix | 135/135=100.00% | 31/31=100.00% | 104 | 543/2067=26.27% | 194/2067=9.39% |
| common | 42/42=100.00% | 88/88=100.00% | 523 | 1686/11963=14.09% | 805/11963=6.73% |
| core | 26/30=86.67% | 77/77=100.00% | 390 | 1837/9979=18.41% | 742/9979=7.44% |
| shared_12 | 15/15=100.00% | 8/8=100.00% | 29 | 150/1891=7.93% | 90/1891=4.76% |
| osal | 4/4=100.00% | 12/12=100.00% | 11 | 138/1074=12.85% | 98/1074=9.12% |
| sapm | 8/8=100.00% | 56/56=100.00% | 239 | 892/9352=9.54% | 650/9352=6.95% |
| **Paper macro average** | **94.92%** | **100.00%** | **2229** | **16.35%** | **8.48%** |
