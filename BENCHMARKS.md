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
storeta and how it is declared. The benchmarks probe those different features to show
the effect of the different optimizations on effective performance.

When you see `HashSet(classic)` vs. `HashSet(foldhash)` this reflects the performance difference between the
normal hasher used by the standard collections as opposed to the performnace that the
`foldhash` hasher provides.

The benchmarks assume a 50% hit rate when probing for lookup, meaning that
half the queries are for non-existing data. Some algorithms perform differently between
present vs. non-existing cases, so real world performance of these algorithms depends on the
real world hit rate you experience.

## Benchmark Results

### dense_scalar

Scalar sets where the values are in a contiguous range.

|            | `HashSet(classic)`          | `HashSet(foldhash)`             | `FzScalarSet`                     | `fz_scalar_set`                   |
|:-----------|:----------------------------|:--------------------------------|:----------------------------------|:--------------------------------- |
| **`3`**    | `45.56 ns` (✅ **1.00x**)    | `12.96 ns` (🚀 **3.52x faster**) | `4.30 ns` (🚀 **10.59x faster**)   | `4.31 ns` (🚀 **10.56x faster**)   |
| **`16`**   | `239.96 ns` (✅ **1.00x**)   | `70.93 ns` (🚀 **3.38x faster**) | `23.56 ns` (🚀 **10.19x faster**)  | `24.37 ns` (🚀 **9.85x faster**)   |
| **`256`**  | `4.04 us` (✅ **1.00x**)     | `1.10 us` (🚀 **3.67x faster**)  | `378.20 ns` (🚀 **10.69x faster**) | `415.59 ns` (🚀 **9.72x faster**)  |
| **`1000`** | `15.88 us` (✅ **1.00x**)    | `4.53 us` (🚀 **3.50x faster**)  | `1.45 us` (🚀 **10.96x faster**)   | `1.60 us` (🚀 **9.93x faster**)    |

### sparse_scalar

Scalar sets where the values are in a non-contiguous range.

|            | `HashSet(classic)`          | `HashSet(foldhash)`             | `FzScalarSet`                     | `fz_scalar_set`                    |
|:-----------|:----------------------------|:--------------------------------|:----------------------------------|:---------------------------------- |
| **`3`**    | `45.59 ns` (✅ **1.00x**)    | `12.94 ns` (🚀 **3.52x faster**) | `4.43 ns` (🚀 **10.30x faster**)   | `4.94 ns` (🚀 **9.23x faster**)     |
| **`16`**   | `253.92 ns` (✅ **1.00x**)   | `65.54 ns` (🚀 **3.87x faster**) | `20.67 ns` (🚀 **12.29x faster**)  | `23.83 ns` (🚀 **10.66x faster**)   |
| **`256`**  | `4.01 us` (✅ **1.00x**)     | `1.11 us` (🚀 **3.62x faster**)  | `330.33 ns` (🚀 **12.13x faster**) | `380.23 ns` (🚀 **10.54x faster**)  |
| **`1000`** | `15.70 us` (✅ **1.00x**)    | `4.67 us` (🚀 **3.36x faster**)  | `1.27 us` (🚀 **12.38x faster**)   | `1.46 us` (🚀 **10.75x faster**)    |

### random_scalar

Scalar sets where the values are randomly distributed.

|            | `HashSet(classic)`          | `HashSet(foldhash)`             | `FzScalarSet`                    | `fz_scalar_set`                   |
|:-----------|:----------------------------|:--------------------------------|:---------------------------------|:--------------------------------- |
| **`3`**    | `47.85 ns` (✅ **1.00x**)    | `14.34 ns` (🚀 **3.34x faster**) | `9.59 ns` (🚀 **4.99x faster**)   | `11.20 ns` (🚀 **4.27x faster**)   |
| **`16`**   | `250.65 ns` (✅ **1.00x**)   | `66.59 ns` (🚀 **3.76x faster**) | `50.36 ns` (🚀 **4.98x faster**)  | `52.01 ns` (🚀 **4.82x faster**)   |
| **`256`**  | `4.00 us` (✅ **1.00x**)     | `1.15 us` (🚀 **3.49x faster**)  | `877.96 ns` (🚀 **4.56x faster**) | `877.15 ns` (🚀 **4.57x faster**)  |
| **`1000`** | `15.85 us` (✅ **1.00x**)    | `4.58 us` (🚀 **3.46x faster**)  | `3.27 us` (🚀 **4.85x faster**)   | `3.24 us` (🚀 **4.89x faster**)    |

### random_string

String sets where the values are random.

|            | `HashSet(classic)`          | `HashSet(foldhash)`              | `FzStringSet`                    | `fz_string_set`                   |
|:-----------|:----------------------------|:---------------------------------|:---------------------------------|:--------------------------------- |
| **`3`**    | `84.61 ns` (✅ **1.00x**)    | `35.35 ns` (🚀 **2.39x faster**)  | `33.42 ns` (🚀 **2.53x faster**)  | `30.64 ns` (🚀 **2.76x faster**)   |
| **`16`**   | `422.60 ns` (✅ **1.00x**)   | `172.87 ns` (🚀 **2.44x faster**) | `204.94 ns` (🚀 **2.06x faster**) | `148.68 ns` (🚀 **2.84x faster**)  |
| **`256`**  | `6.84 us` (✅ **1.00x**)     | `2.84 us` (🚀 **2.41x faster**)   | `3.67 us` (🚀 **1.86x faster**)   | `2.67 us` (🚀 **2.56x faster**)    |
| **`1000`** | `27.90 us` (✅ **1.00x**)    | `11.75 us` (🚀 **2.38x faster**)  | `15.93 us` (✅ **1.75x faster**)  | `10.36 us` (🚀 **2.69x faster**)   |

### prefixed_string

String sets where the values are random, but share a common prefix.

|            | `HashSet(classic)`          | `HashSet(foldhash)`              | `FzStringSet`                    | `fz_string_set`                   |
|:-----------|:----------------------------|:---------------------------------|:---------------------------------|:--------------------------------- |
| **`3`**    | `93.07 ns` (✅ **1.00x**)    | `42.17 ns` (🚀 **2.21x faster**)  | `34.68 ns` (🚀 **2.68x faster**)  | `26.55 ns` (🚀 **3.51x faster**)   |
| **`16`**   | `456.13 ns` (✅ **1.00x**)   | `192.12 ns` (🚀 **2.37x faster**) | `211.13 ns` (🚀 **2.16x faster**) | `139.46 ns` (🚀 **3.27x faster**)  |
| **`256`**  | `7.53 us` (✅ **1.00x**)     | `3.23 us` (🚀 **2.33x faster**)   | `3.72 us` (🚀 **2.02x faster**)   | `2.93 us` (🚀 **2.57x faster**)    |
| **`1000`** | `30.59 us` (✅ **1.00x**)    | `13.40 us` (🚀 **2.28x faster**)  | `25.02 us` (✅ **1.22x faster**)  | `11.67 us` (🚀 **2.62x faster**)   |

### hashed

Sets with a complex key type that is hashable.

|            | `HashSet(classic)`          | `HashSet(foldhash)`              | `FzHashSet`                      | `fz_hash_set`                     |
|:-----------|:----------------------------|:---------------------------------|:---------------------------------|:--------------------------------- |
| **`3`**    | `109.24 ns` (✅ **1.00x**)   | `34.17 ns` (🚀 **3.20x faster**)  | `31.72 ns` (🚀 **3.44x faster**)  | `57.83 ns` (🚀 **1.89x faster**)   |
| **`16`**   | `533.52 ns` (✅ **1.00x**)   | `172.98 ns` (🚀 **3.08x faster**) | `147.13 ns` (🚀 **3.63x faster**) | `147.14 ns` (🚀 **3.63x faster**)  |
| **`256`**  | `8.40 us` (✅ **1.00x**)     | `2.97 us` (🚀 **2.82x faster**)   | `3.00 us` (🚀 **2.80x faster**)   | `3.15 us` (🚀 **2.67x faster**)    |
| **`1000`** | `33.59 us` (✅ **1.00x**)    | `11.98 us` (🚀 **2.80x faster**)  | `12.33 us` (🚀 **2.72x faster**)  | `11.99 us` (🚀 **2.80x faster**)   |

### ordered

Sets with a complex key type that is ordered.

|            | `BTreeSet`                | `FzOrderedSet`                   | `fz_ordered_set`                  |
|:-----------|:--------------------------|:---------------------------------|:--------------------------------- |
| **`3`**    | `80.70 ns` (✅ **1.00x**)  | `73.88 ns` (✅ **1.09x faster**)  | `47.64 ns` (✅ **1.69x faster**)   |
| **`16`**   | `940.53 ns` (✅ **1.00x**) | `925.77 ns` (✅ **1.02x faster**) | `962.00 ns` (✅ **1.02x slower**)  |
| **`256`**  | `30.47 us` (✅ **1.00x**)  | `20.00 us` (✅ **1.52x faster**)  | `19.68 us` (✅ **1.55x faster**)   |
| **`1000`** | `226.52 us` (✅ **1.00x**) | `187.35 us` (✅ **1.21x faster**) | `189.47 us` (✅ **1.20x faster**)  |

---
Made with [criterion-table](https://github.com/nu11ptr/criterion-table)

