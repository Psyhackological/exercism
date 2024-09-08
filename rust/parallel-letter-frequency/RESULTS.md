# Results

## Table

| Test                       | No Threads (ns/iter) | Yes Threads (ns/iter) | Rayon (ns/iter) |
| -------------------------- | -------------------- | --------------------- | --------------- |
| **bench_large_parallel**   | 470,717.85           | 342,444.23            | 275,566.25      |
| **bench_large_sequential** | 711,999.20           | 713,032.70            | 713,687.70      |
| **bench_small_parallel**   | 16,310.11            | 9,914.62              | 9,858.82        |
| **bench_small_sequential** | 24,398.17            | 24,527.93             | 24,423.64       |
| **bench_tiny_parallel**    | 68.90                | 59.79                 | 58.05           |
| **bench_tiny_sequential**  | 78.77                | 80.35                 | 79.13           |
