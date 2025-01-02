# Benchmarks

## Table of Contents

- [Overview](#overview)
- [Benchmark Results](#benchmark-results)
    - [dense_scalar](#dense_scalar)
    - [sparse_scalar](#sparse_scalar)
    - [random_scalar](#random_scalar)
    - [random_string](#random_string)
    - [prefixed_string](#prefixed_string)
    - [hashed](#hashed)
    - [ordered](#ordered)

## Overview

These benchmarks compare the performance of the frozen collecitons relative
to the classic Rust collections.

The frozen collections have different optimizations depending on the type of data they
store and how it is declared. The benchmarks probe those different features to show
the effect of the different optimizations on effective performance.

When you see `HashSet(classic)` vs. `HashSet(foldhash)` this reflects the performance difference between the
normal hasher used by the standard collections as opposed to the performance that the
`foldhash` hasher provides.

The benchmarks assume a 50% hit rate when probing for lookup, meaning that
half the queries are for non-existing data. Some algorithms perform differently between
present vs. non-existing cases, so real world performance of these algorithms depends on the
real world hit rate you experience.

## Benchmark Results

### dense_scalar

Scalar sets where the values are in a contiguous range.

|            | `HashSet(classic)`          | `HashSet(foldhash)`              | `FzScalarSet`                     | `fz_scalar_set`                    |
|:-----------|:----------------------------|:---------------------------------|:----------------------------------|:---------------------------------- |
| **`3`**    | `27.14 ns` (✅ **1.00x**)    | `7.51 ns` (🚀 **3.62x faster**)   | `2.57 ns` (🚀 **10.57x faster**)   | `2.62 ns` (🚀 **10.35x faster**)    |
| **`16`**   | `136.92 ns` (✅ **1.00x**)   | `41.63 ns` (🚀 **3.29x faster**)  | `14.06 ns` (🚀 **9.73x faster**)   | `13.43 ns` (🚀 **10.19x faster**)   |
| **`256`**  | `2.32 us` (✅ **1.00x**)     | `651.33 ns` (🚀 **3.57x faster**) | `227.27 ns` (🚀 **10.23x faster**) | `226.92 ns` (🚀 **10.24x faster**)  |
| **`1000`** | `9.20 us` (✅ **1.00x**)     | `2.63 us` (🚀 **3.50x faster**)   | `832.45 ns` (🚀 **11.06x faster**) | `844.27 ns` (🚀 **10.90x faster**)  |

### sparse_scalar

Scalar sets where the values are in a non-contiguous range.

|            | `HashSet(classic)`          | `HashSet(foldhash)`              | `FzScalarSet`                     | `fz_scalar_set`                    |
|:-----------|:----------------------------|:---------------------------------|:----------------------------------|:---------------------------------- |
| **`3`**    | `26.11 ns` (✅ **1.00x**)    | `7.06 ns` (🚀 **3.70x faster**)   | `2.65 ns` (🚀 **9.84x faster**)    | `3.02 ns` (🚀 **8.65x faster**)     |
| **`16`**   | `140.36 ns` (✅ **1.00x**)   | `40.63 ns` (🚀 **3.45x faster**)  | `14.40 ns` (🚀 **9.74x faster**)   | `20.29 ns` (🚀 **6.92x faster**)    |
| **`256`**  | `2.28 us` (✅ **1.00x**)     | `629.81 ns` (🚀 **3.62x faster**) | `224.71 ns` (🚀 **10.15x faster**) | `222.06 ns` (🚀 **10.27x faster**)  |
| **`1000`** | `9.29 us` (✅ **1.00x**)     | `2.55 us` (🚀 **3.64x faster**)   | `831.15 ns` (🚀 **11.18x faster**) | `837.56 ns` (🚀 **11.10x faster**)  |

### random_scalar

Scalar sets where the values are randomly distributed.

|            | `HashSet(classic)`          | `HashSet(foldhash)`              | `FzScalarSet`                    | `fz_scalar_set`                   |
|:-----------|:----------------------------|:---------------------------------|:---------------------------------|:--------------------------------- |
| **`3`**    | `26.42 ns` (✅ **1.00x**)    | `7.00 ns` (🚀 **3.77x faster**)   | `4.61 ns` (🚀 **5.73x faster**)   | `4.60 ns` (🚀 **5.75x faster**)    |
| **`16`**   | `145.67 ns` (✅ **1.00x**)   | `40.70 ns` (🚀 **3.58x faster**)  | `25.75 ns` (🚀 **5.66x faster**)  | `24.40 ns` (🚀 **5.97x faster**)   |
| **`256`**  | `2.27 us` (✅ **1.00x**)     | `687.58 ns` (🚀 **3.31x faster**) | `561.43 ns` (🚀 **4.05x faster**) | `730.54 ns` (🚀 **3.11x faster**)  |
| **`1000`** | `11.14 us` (✅ **1.00x**)    | `3.51 us` (🚀 **3.17x faster**)   | `2.73 us` (🚀 **4.08x faster**)   | `2.68 us` (🚀 **4.15x faster**)    |

### random_string

String sets where the values are random.

|            | `HashSet(classic)`          | `HashSet(foldhash)`              | `FzStringSet`                    | `fz_string_set`                   |
|:-----------|:----------------------------|:---------------------------------|:---------------------------------|:--------------------------------- |
| **`3`**    | `61.04 ns` (✅ **1.00x**)    | `25.87 ns` (🚀 **2.36x faster**)  | `36.21 ns` (✅ **1.69x faster**)  | `22.89 ns` (🚀 **2.67x faster**)   |
| **`16`**   | `312.41 ns` (✅ **1.00x**)   | `120.63 ns` (🚀 **2.59x faster**) | `181.25 ns` (✅ **1.72x faster**) | `118.82 ns` (🚀 **2.63x faster**)  |
| **`256`**  | `5.04 us` (✅ **1.00x**)     | `2.02 us` (🚀 **2.50x faster**)   | `3.17 us` (✅ **1.59x faster**)   | `2.05 us` (🚀 **2.45x faster**)    |
| **`1000`** | `23.55 us` (✅ **1.00x**)    | `8.83 us` (🚀 **2.67x faster**)   | `14.15 us` (✅ **1.66x faster**)  | `9.51 us` (🚀 **2.48x faster**)    |

### prefixed_string

String sets where the values are random, but share a common prefix.

|            | `HashSet(classic)`          | `HashSet(foldhash)`              | `FzStringSet`                    | `fz_string_set`                   |
|:-----------|:----------------------------|:---------------------------------|:---------------------------------|:--------------------------------- |
| **`3`**    | `64.95 ns` (✅ **1.00x**)    | `28.36 ns` (🚀 **2.29x faster**)  | `35.17 ns` (🚀 **1.85x faster**)  | `23.74 ns` (🚀 **2.74x faster**)   |
| **`16`**   | `343.47 ns` (✅ **1.00x**)   | `140.68 ns` (🚀 **2.44x faster**) | `181.78 ns` (🚀 **1.89x faster**) | `141.47 ns` (🚀 **2.43x faster**)  |
| **`256`**  | `5.65 us` (✅ **1.00x**)     | `2.28 us` (🚀 **2.47x faster**)   | `3.40 us` (✅ **1.66x faster**)   | `2.42 us` (🚀 **2.34x faster**)    |
| **`1000`** | `24.01 us` (✅ **1.00x**)    | `10.93 us` (🚀 **2.20x faster**)  | `16.88 us` (✅ **1.42x faster**)  | `9.03 us` (🚀 **2.66x faster**)    |

### hashed

Sets with a complex key type that is hashable.

|            | `HashSet(classic)`          | `HashSet(foldhash)`              | `FzHashSet`                      | `fz_hash_set`                     |
|:-----------|:----------------------------|:---------------------------------|:---------------------------------|:--------------------------------- |
| **`3`**    | `75.34 ns` (✅ **1.00x**)    | `28.20 ns` (🚀 **2.67x faster**)  | `29.94 ns` (🚀 **2.52x faster**)  | `28.91 ns` (🚀 **2.61x faster**)   |
| **`16`**   | `395.30 ns` (✅ **1.00x**)   | `138.94 ns` (🚀 **2.85x faster**) | `147.65 ns` (🚀 **2.68x faster**) | `123.05 ns` (🚀 **3.21x faster**)  |
| **`256`**  | `6.51 us` (✅ **1.00x**)     | `2.43 us` (🚀 **2.68x faster**)   | `2.45 us` (🚀 **2.65x faster**)   | `2.37 us` (🚀 **2.74x faster**)    |
| **`1000`** | `26.28 us` (✅ **1.00x**)    | `9.55 us` (🚀 **2.75x faster**)   | `10.20 us` (🚀 **2.58x faster**)  | `10.05 us` (🚀 **2.61x faster**)   |

### ordered

Sets with a complex key type that is ordered.

|            | `BTreeSet`                | `FzOrderedSet`                   | `fz_ordered_set`                  |
|:-----------|:--------------------------|:---------------------------------|:--------------------------------- |
| **`3`**    | `68.52 ns` (✅ **1.00x**)  | `58.24 ns` (✅ **1.18x faster**)  | `57.56 ns` (✅ **1.19x faster**)   |
| **`16`**   | `934.93 ns` (✅ **1.00x**) | `593.69 ns` (✅ **1.57x faster**) | `588.56 ns` (✅ **1.59x faster**)  |
| **`256`**  | `33.38 us` (✅ **1.00x**)  | `24.05 us` (✅ **1.39x faster**)  | `23.75 us` (✅ **1.41x faster**)   |
| **`1000`** | `204.16 us` (✅ **1.00x**) | `181.22 us` (✅ **1.13x faster**) | `176.04 us` (✅ **1.16x faster**)  |

---
Made with [criterion-table](https://github.com/nu11ptr/criterion-table)

