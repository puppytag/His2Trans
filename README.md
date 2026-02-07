# His2Trans

This repository contains only:
- `scripts/`: paper metric analysis scripts + one-click run scripts
- `data/`: the corresponding minimal inputs (translation outputs + test inputs)

## Run

```bash
cd /data/home/wangshb/His2Trans/scripts

# Run everything (RQ1–RQ4)
bash run_all_analysis.sh

# Or run per RQ
bash run_rq1_analysis.sh
bash run_rq2_analysis.sh
bash run_rq3_analysis.sh
bash run_rq4_analysis.sh
```

You can also run a single analysis script:

```bash
cd /data/home/wangshb/His2Trans/scripts

# RQ1/RQ3/RQ4 (OHOS test5)
python3 analyze_c2r_compilation_rate_ohos_test5.py --run-dir ../data/rq1/k1 --all

# RQ2 (test_module)
python3 analyze_c2r_compilation_rate.py --run-dir ../data/rq2/deepseek --all
```
