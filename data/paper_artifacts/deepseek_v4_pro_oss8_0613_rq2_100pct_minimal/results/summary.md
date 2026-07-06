| project | incremental compile rate | RQ2 test pass rate | warnings | raw unsafe | required unsafe |
|---|---:|---:|---:|---:|---:|
| urlparser | 23/23 (100.00%) | 3/3 (100.00%) | 0 | 105/993 (10.57%) | 40/993 (4.03%) |
| avl | 75/75 (100.00%) | 2/2 (100.00%) | 9 | 116/1513 (7.67%) | 29/1513 (1.92%) |
| ht | 2/2 (100.00%) | 1/1 (100.00%) | 0 | 2/174 (1.15%) | 1/174 (0.57%) |
| qsort | 2/2 (100.00%) | 6/6 (100.00%) | 0 | 13/185 (7.03%) | 3/185 (1.62%) |
| buffer | 30/30 (100.00%) | 14/14 (100.00%) | 0 | 129/815 (15.83%) | 42/815 (5.15%) |
| rgba | 17/17 (100.00%) | 10/10 (100.00%) | 0 | 30/781 (3.84%) | 9/781 (1.15%) |
| quadtree | 27/27 (100.00%) | 4/4 (100.00%) | 0 | 104/974 (10.68%) | 48/974 (4.93%) |
| genann | 12/12 (100.00%) | 12/12 (100.00%) | 1 | 95/792 (11.99%) | 33/792 (4.17%) |
| **Paper macro average** | 100.00% | 100.00% | 10 | 8.59% | 2.94% |

The final row is the paper metric policy: macro average over project-level rates.
warnings 列为 `rustc_warning_count`；`results/warnings.json` 同时保留 clippy warnings 和总 warnings。
