## RQ1: OHOS test5

| Method | ICompRate | FC | Unsafe | Warnings |
| --- | --- | --- | --- | --- |
| C2Rust | 10.28 | 22.50 | 47.75 | 45.00 |
| C2SaferRust | 53.62 | 10.00 | 36.33 | 27.50 |
| EvoC2Rust | 0.00 | -- | -- | -- |
| Tymcrat | 51.53 | -- | -- | -- |
| Ours(DS-V3.2 K = 1) | 87.85 | 75.00 | 32.15 | 464.00 |
| Ours(DS-V3.2 K = 3) | 88.47 | 75.00 | 33.02 | 448.80 |
| Ours(DS-V3.2 K = 5) | 90.34 | 75.00 | 35.43 | 463.00 |
| Ours(DS-V3.2 K = 10) | 87.23 | 75.00 | 34.06 | 446.40 |
| Ours(Claude-4.5 K = 5) | 97.51 | 75.00 | 37.09 | 315.40 |

## RQ2: test_module

| Method | ICompRate | FC | Unsafe | Warnings |
| --- | --- | --- | --- | --- |
| C2Rust | 100.00 | 94.55 | 67.93 | 124.11 |
| SmartC2Rust | 99.38 | 100.00 | -- | -- |
| RUSTINE | 100.00 | 100.00 | -- | -- |
| PTRMAPPER | 100.00 | 82.44 | -- | -- |
| C2SaferRust | 94.89 | 80.00 | 45.20 | 117.78 |
| EvoC2Rust | 91.06 | 89.09 | 6.50 | 431.00 |
| Tymcrat | 1.28 | 10.91 | 0.00 | 2.50 |
| Ours(DS-V3.2) | 93.19 | 47.27 | 37.01 | 136.44 |
| Ours(Claude-4.5) | 95.74 | 58.18 | 43.91 | 101.00 |

## RQ3: Ablation Study

| Config ID | ICompRate | FC | AvgRepair |
| --- | --- | --- | --- |
| Base-1Shot | 42.50 | 100.00 | 0.00 |
| Base-Rep | 90.00 | 100.00 | 1.52 |
| Pred-1Shot | 47.50 | 100.00 | 0.00 |
| Pred-Rep | 82.50 | 100.00 | 1.55 |
| GT-API | 80.00 | 100.00 | 1.35 |
| GT-Frag | 87.50 | 100.00 | 1.45 |
| GT-Full | 87.50 | 100.00 | 1.40 |

## RQ4: Knowledge Base Ablation

| Setting | ICompRate | FC | AvgRepair |
| --- | --- | --- | --- |
| Base KB only | 41.28 | 37.50 | 1.55 |
| Base KB + Accumulated KB | 41.64 | 37.50 | 0.61 |
| Relative Improvement | 0.87 | -- | -60.69 |
