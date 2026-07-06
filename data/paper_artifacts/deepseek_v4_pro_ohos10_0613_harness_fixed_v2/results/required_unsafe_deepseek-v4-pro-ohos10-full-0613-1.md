| 项目 | total lines | raw unsafe lines | raw unsafe rate | required unsafe lines | required unsafe rate | top reasons |
|---|---:|---:|---:|---:|---:|---|
| `host__25c1898e1626` | 7506 | 1630 | 21.72% | 847 | 11.28% | raw_pointer_deref:376, unsafe_extern_block_contract:185, unsafe_abi_contract:128 |
| `appverify_lite__e5ebe91a98b9` | 11223 | 1833 | 16.33% | 914 | 8.14% | raw_pointer_deref:301, ffi_call:257, unsafe_extern_block_contract:163 |
| `manager__c248934e0221` | 6604 | 1568 | 23.74% | 910 | 13.78% | raw_pointer_deref:459, ffi_call:165, unsafe_extern_block_contract:125 |
| `shared__541f4e547bdb` | 3018 | 382 | 12.66% | 217 | 7.19% | unsafe_abi_contract:75, unsafe_extern_block_contract:48, raw_pointer_deref:46 |
| `posix__1b7f59c68bbc` | 2067 | 543 | 26.27% | 194 | 9.39% | unsafe_extern_block_contract:79, ffi_call:63, raw_pointer_deref:53 |
| `common__89d5ecaafdff` | 11963 | 1686 | 14.09% | 805 | 6.73% | unsafe_abi_contract:291, raw_pointer_deref:249, ffi_call:130 |
| `core__ef5242b7ab08` | 9979 | 1837 | 18.41% | 742 | 7.44% | raw_pointer_deref:287, unsafe_abi_contract:215, ffi_call:140 |
| `shared__12e38ea922f7` | 1891 | 150 | 7.93% | 90 | 4.76% | unsafe_abi_contract:37, ffi_call:29, unsafe_extern_block_contract:20 |
| `osal__0bc4f21396ad` | 1074 | 138 | 12.85% | 98 | 9.12% | raw_pointer_deref:52, ffi_call:23, unsafe_extern_block_contract:17 |
| `sapm__193cdeb43a97` | 9352 | 892 | 9.54% | 650 | 6.95% | raw_pointer_deref:309, unsafe_abi_contract:223, ffi_call:45 |
| **Paper macro average** | 64677 | 10659 | 16.35% | 5467 | 8.48% | - |

The final row is the paper metric policy: macro average over project-level rates.
