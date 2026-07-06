# OHOS10 ablation full metrics summary

Primary rate policy: macro project average, i.e. sum the 10 per-project rates and divide by 10. Micro counts are retained only as evidence.

## Main metrics (macro project average)

| Group | Setting | Incremental compile | Test pass | Raw unsafe | Required unsafe |
|---|---:|---:|---:|---:|---:|
| A1 | initial function translation | 95.82% | 39.29% | 15.95% | 7.19% |
| A2 | A1 + compiler-driven per-function repair | 100.00% | 94.92% | 22.52% | 9.83% |
| A3 | A2 + post-repair agent | 100.00% | 94.92% | 16.35% | 8.48% |

## Micro count evidence

| Group | Incremental compile | Test pass | Raw unsafe | Required unsafe | Note |
|---|---:|---:|---:|---:|---|
| A1 | 483/495 (97.58%) | 63/265 (23.77%) | 11496/68580 (16.76%) | 5120/68580 (7.47%) | no compiler-driven incremental repair; no post-repair agent |
| A2 | 631/631 (100.00%) | 252/259 (97.30%) | 15001/64830 (23.14%) | 6667/64830 (10.28%) | no post-repair agent |
| A3 | 722/722 (100.00%) | 252/259 (97.30%) | 10659/64677 (16.48%) | 5467/64677 (8.45%) | uses RQ2 replacement output deepseek-v4-pro-ohos10-full-0613-1_harness_fixed_v2 as requested |

## Per-project detail

| Group | Project | Incremental compile | Test pass | Raw unsafe | Required unsafe | GTest stage |
|---|---|---:|---:|---:|---:|---|
| A1 | `host__25c1898e1626` | 84/85 (98.82%) | 0/8 (0.00%) | 1417/7665 (18.49%) | 655/7665 (8.55%) | rust_staticlib |
| A1 | `appverify_lite__e5ebe91a98b9` | 144/144 (100.00%) | 0/8 (0.00%) | 2308/11992 (19.25%) | 897/11992 (7.48%) | gtest_run |
| A1 | `manager__c248934e0221` | 63/63 (100.00%) | 0/4 (0.00%) | 1154/6469 (17.84%) | 615/6469 (9.51%) | gtest_run |
| A1 | `shared__541f4e547bdb` | 21/21 (100.00%) | 5/5 (100.00%) | 371/3070 (12.08%) | 190/3070 (6.19%) | done |
| A1 | `posix__1b7f59c68bbc` | 9/11 (81.82%) | 0/141 (0.00%) | 415/2001 (20.74%) | 118/2001 (5.90%) | rust_staticlib |
| A1 | `common__89d5ecaafdff` | 79/79 (100.00%) | 39/42 (92.86%) | 2195/12025 (18.25%) | 948/12025 (7.88%) | gtest_run |
| A1 | `core__ef5242b7ab08` | 33/37 (89.19%) | 0/30 (0.00%) | 1576/11602 (13.58%) | 667/11602 (5.75%) | rust_staticlib |
| A1 | `shared__12e38ea922f7` | 7/7 (100.00%) | 15/15 (100.00%) | 218/1905 (11.44%) | 114/1905 (5.98%) | done |
| A1 | `osal__0bc4f21396ad` | 5/5 (100.00%) | 4/4 (100.00%) | 125/1043 (11.98%) | 72/1043 (6.90%) | done |
| A1 | `sapm__193cdeb43a97` | 38/43 (88.37%) | 0/8 (0.00%) | 1717/10808 (15.89%) | 844/10808 (7.81%) | rust_staticlib |
| A2 | `host__25c1898e1626` | 118/118 (100.00%) | 8/8 (100.00%) | 1928/7296 (26.43%) | 900/7296 (12.34%) | done |
| A2 | `appverify_lite__e5ebe91a98b9` | 162/162 (100.00%) | 5/8 (62.50%) | 3690/12353 (29.87%) | 1426/12353 (11.54%) | gtest_run |
| A2 | `manager__c248934e0221` | 82/82 (100.00%) | 4/4 (100.00%) | 1590/6469 (24.58%) | 832/6469 (12.86%) | done |
| A2 | `shared__541f4e547bdb` | 26/26 (100.00%) | 5/5 (100.00%) | 440/3026 (14.54%) | 212/3026 (7.01%) | done |
| A2 | `posix__1b7f59c68bbc` | 31/31 (100.00%) | 135/135 (100.00%) | 607/1845 (32.90%) | 184/1845 (9.97%) | done |
| A2 | `common__89d5ecaafdff` | 84/84 (100.00%) | 42/42 (100.00%) | 2237/11706 (19.11%) | 1014/11706 (8.66%) | done |
| A2 | `core__ef5242b7ab08` | 67/67 (100.00%) | 26/30 (86.67%) | 2117/9702 (21.82%) | 920/9702 (9.48%) | gtest_run |
| A2 | `shared__12e38ea922f7` | 7/7 (100.00%) | 15/15 (100.00%) | 310/1975 (15.70%) | 130/1975 (6.58%) | done |
| A2 | `osal__0bc4f21396ad` | 7/7 (100.00%) | 4/4 (100.00%) | 216/1057 (20.44%) | 103/1057 (9.74%) | done |
| A2 | `sapm__193cdeb43a97` | 47/47 (100.00%) | 8/8 (100.00%) | 1866/9401 (19.85%) | 946/9401 (10.06%) | done |
| A3 | `host__25c1898e1626` | 127/127 (100.00%) | 8/8 (100.00%) | 1630/7506 (21.72%) | 847/7506 (11.28%) | done |
| A3 | `appverify_lite__e5ebe91a98b9` | 173/173 (100.00%) | 5/8 (62.50%) | 1833/11223 (16.33%) | 914/11223 (8.14%) | gtest_run |
| A3 | `manager__c248934e0221` | 123/123 (100.00%) | 4/4 (100.00%) | 1568/6604 (23.74%) | 910/6604 (13.78%) | done |
| A3 | `shared__541f4e547bdb` | 27/27 (100.00%) | 5/5 (100.00%) | 382/3018 (12.66%) | 217/3018 (7.19%) | done |
| A3 | `posix__1b7f59c68bbc` | 31/31 (100.00%) | 135/135 (100.00%) | 543/2067 (26.27%) | 194/2067 (9.39%) | done |
| A3 | `common__89d5ecaafdff` | 88/88 (100.00%) | 42/42 (100.00%) | 1686/11963 (14.09%) | 805/11963 (6.73%) | done |
| A3 | `core__ef5242b7ab08` | 77/77 (100.00%) | 26/30 (86.67%) | 1837/9979 (18.41%) | 742/9979 (7.44%) | gtest_run |
| A3 | `shared__12e38ea922f7` | 8/8 (100.00%) | 15/15 (100.00%) | 150/1891 (7.93%) | 90/1891 (4.76%) | done |
| A3 | `osal__0bc4f21396ad` | 12/12 (100.00%) | 4/4 (100.00%) | 138/1074 (12.85%) | 98/1074 (9.12%) | done |
| A3 | `sapm__193cdeb43a97` | 56/56 (100.00%) | 8/8 (100.00%) | 892/9352 (9.54%) | 650/9352 (6.95%) | done |

## Evidence files

| Group | Incremental compile JSON | GTest summary JSON | Unsafe JSON |
|---|---|---|---|
| A1 | `/data/home/wangshb/c2-rust_framework/paper_experiments/results/incremental_compile_ablation-ohos10-a1-initial-translation.json` | `/data/home/wangshb/c2-rust_framework/paper_experiments/results/ablation_ohos10_a1_initial_translation_gtests/summary.json` | `/data/home/wangshb/c2-rust_framework/paper_experiments/results/required_unsafe_ablation-ohos10-a1-initial-translation.json` |
| A2 | `/data/home/wangshb/c2-rust_framework/paper_experiments/results/incremental_compile_ablation-ohos10-a2-incremental-repair.json` | `/data/home/wangshb/c2-rust_framework/paper_experiments/results/ablation_ohos10_a2_incremental_repair_gtests/summary.json` | `/data/home/wangshb/c2-rust_framework/paper_experiments/results/required_unsafe_ablation-ohos10-a2-incremental-repair.json` |
| A3 | `/data/home/wangshb/c2-rust_framework/paper_experiments/results/incremental_compile_deepseek-v4-pro-ohos10-full-0613-1.json` | `/data/home/wangshb/c2-rust_framework/paper_experiments/results/ablation_ohos10_a3_post_agent_gtests/summary.json` | `/data/home/wangshb/c2-rust_framework/paper_experiments/results/required_unsafe_deepseek-v4-pro-ohos10-full-0613-1.json` |
