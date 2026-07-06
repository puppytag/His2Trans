# Current Paper Metrics Alignment

Macro project average for method and ablation tables unless a row explicitly reports micro evidence.

## Key Number Checks

- RQ1 Ours ICompRate: 100.00
- RQ1 Ours TestPass: 94.92
- RQ1 Ours Unsafe: 16.35
- RQ2 Ours ICompRate: 100.00
- RQ2 Ours TestPass: 100.00
- RQ2 Ours Unsafe: 8.59
- RQ3 A1 ICompRate: 95.82
- RQ3 A1 TestPass: 39.29
- RQ3 A1 Unsafe: 15.95
- RQ3 A2 ICompRate: 100.00
- RQ3 A2 TestPass: 94.92
- RQ3 A2 Unsafe: 22.52
- RQ3 A3 ICompRate: 100.00
- RQ3 A3 TestPass: 94.92
- RQ3 A3 Unsafe: 16.35

## RQ1: OpenHarmony Module Dataset

| Method | ICompRate | TestPass | Unsafe | RequiredUnsafe |
| --- | ---: | ---: | ---: | ---: |
| Ours | 100.00 | 94.92 | 16.35 | 8.48 |
| Claude Code | 100.00 | 80.08 | 33.67 | 14.00 |
| C2Rust | 99.58 | 67.50 | 58.11 | 13.82 |
| C2SaferRust | 79.99 | 40.92 | 27.63 | 8.54 |
| EvoC2Rust | 0.00 | 0.00 | 1.16 | 0.28 |
| Tymcrat | 9.09 | 0.00 | 38.03 | 17.73 |

## RQ2: Open-Source Project Dataset

| Method | ICompRate | TestPass | Unsafe | RequiredUnsafe |
| --- | ---: | ---: | ---: | ---: |
| Ours | 100.00 | 100.00 | 8.59 | 2.94 |
| Claude Code | 100.00 | 100.00 | 4.04 | 1.84 |
| C2Rust | 100.00 | 100.00 | 42.88 | 21.55 |
| C2SaferRust | 100.00 | 46.88 | 24.19 | 10.08 |
| EvoC2Rust | 22.30 | 12.50 | 1.33 | 0.31 |
| Tymcrat | 49.14 | 0.00 | 17.29 | 11.27 |

## RQ3: Ablation Study

| Group | Setting | ICompRate | TestPass | Unsafe | RequiredUnsafe |
| --- | ---: | ---: | ---: | ---: | ---: |
| A1 | initial function translation | 95.82 | 39.29 | 15.95 | 7.19 |
| A2 | A1 + compiler-driven per-function repair | 100.00 | 94.92 | 22.52 | 9.83 |
| A3 | A2 + post-repair agent | 100.00 | 94.92 | 16.35 | 8.48 |

## RQ4: Historical Knowledge Reuse Cases

| Case | Boundary | Evidence | PaperSection |
| --- | ---: | ---: | ---: |
| manager | HDF service event notification | His2Trans preserves the system-visible event path and cleanup logic. | RQ4 Historical Knowledge Reuse Analysis |
| shared_12 | HDF SBuf wire-format serialization | His2Trans preserves external HdfSbufRead*/Write* calls and passes 15/15 tests. | RQ4 Historical Knowledge Reuse Analysis |
