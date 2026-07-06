| 项目 | total lines | raw unsafe lines | raw unsafe rate | required unsafe lines | required unsafe rate | top reasons |
|---|---:|---:|---:|---:|---:|---|
| `urlparser` | 993 | 105 | 10.57% | 40 | 4.03% | unsafe_extern_block_contract:40 |
| `avl` | 1513 | 116 | 7.67% | 29 | 1.92% | unsafe_extern_block_contract:27, raw_pointer_deref:2 |
| `ht` | 174 | 2 | 1.15% | 1 | 0.57% | unsafe_extern_block_contract:1 |
| `qsort` | 185 | 13 | 7.03% | 3 | 1.62% | unsafe_extern_block_contract:3 |
| `buffer` | 815 | 129 | 15.83% | 42 | 5.15% | unsafe_extern_block_contract:37, raw_pointer_deref:3, raw_ownership_transfer:1 |
| `rgba` | 781 | 30 | 3.84% | 9 | 1.15% | unsafe_extern_block_contract:9 |
| `quadtree` | 974 | 104 | 10.68% | 48 | 4.93% | unsafe_extern_block_contract:30, unsafe_abi_contract:14, raw_ownership_transfer:4 |
| `genann` | 792 | 95 | 11.99% | 33 | 4.17% | unsafe_extern_block_contract:23, ffi_call:10, raw_pointer_deref:4 |
| **Paper macro average** | 6227 | 594 | 8.59% | 205 | 2.94% | - |

The final row is the paper metric policy: macro average over project-level rates.
