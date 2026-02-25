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

|            | `HashSet(classic)`          | `HashSet(foldhash)`             | `FzScalarSet`                    | `fz_scalar_set`                    |
|:-----------|:----------------------------|:--------------------------------|:---------------------------------|:---------------------------------- |
| **`3`**    | `54.20 ns` (✅ **1.00x**)    | `16.84 ns` (🚀 **3.22x faster**) | `6.51 ns` (🚀 **8.32x faster**)   | `5.36 ns` (🚀 **10.11x faster**)    |
| **`16`**   | `294.10 ns` (✅ **1.00x**)   | `79.52 ns` (🚀 **3.70x faster**) | `31.51 ns` (🚀 **9.33x faster**)  | `28.23 ns` (🚀 **10.42x faster**)   |
| **`256`**  | `4.69 us` (✅ **1.00x**)     | `1.44 us` (🚀 **3.25x faster**)  | `493.76 ns` (🚀 **9.49x faster**) | `455.94 ns` (🚀 **10.28x faster**)  |
| **`1000`** | `18.63 us` (✅ **1.00x**)    | `5.79 us` (🚀 **3.22x faster**)  | `1.92 us` (🚀 **9.71x faster**)   | `1.78 us` (🚀 **10.45x faster**)    |

### sparse_scalar

Scalar sets where the values are in a non-contiguous range.

|            | `HashSet(classic)`          | `HashSet(foldhash)`             | `FzScalarSet`                    | `fz_scalar_set`                   |
|:-----------|:----------------------------|:--------------------------------|:---------------------------------|:--------------------------------- |
| **`3`**    | `53.94 ns` (✅ **1.00x**)    | `15.83 ns` (🚀 **3.41x faster**) | `9.48 ns` (🚀 **5.69x faster**)   | `6.68 ns` (🚀 **8.07x faster**)    |
| **`16`**   | `284.27 ns` (✅ **1.00x**)   | `84.07 ns` (🚀 **3.38x faster**) | `43.25 ns` (🚀 **6.57x faster**)  | `29.66 ns` (🚀 **9.59x faster**)   |
| **`256`**  | `4.75 us` (✅ **1.00x**)     | `1.32 us` (🚀 **3.60x faster**)  | `677.30 ns` (🚀 **7.01x faster**) | `629.10 ns` (🚀 **7.55x faster**)  |
| **`1000`** | `18.55 us` (✅ **1.00x**)    | `5.69 us` (🚀 **3.26x faster**)  | `2.62 us` (🚀 **7.07x faster**)   | `2.14 us` (🚀 **8.65x faster**)    |

### random_scalar

Scalar sets where the values are randomly distributed.

|            | `HashSet(classic)`          | `HashSet(foldhash)`             | `FzScalarSet`                   | `fz_scalar_set`                  |
|:-----------|:----------------------------|:--------------------------------|:--------------------------------|:-------------------------------- |
| **`3`**    | `55.56 ns` (✅ **1.00x**)    | `14.90 ns` (🚀 **3.73x faster**) | `11.11 ns` (🚀 **5.00x faster**) | `6.21 ns` (🚀 **8.94x faster**)   |
| **`16`**   | `288.60 ns` (✅ **1.00x**)   | `86.91 ns` (🚀 **3.32x faster**) | `55.74 ns` (🚀 **5.18x faster**) | `56.35 ns` (🚀 **5.12x faster**)  |
| **`256`**  | `4.76 us` (✅ **1.00x**)     | `1.46 us` (🚀 **3.26x faster**)  | `1.20 us` (🚀 **3.97x faster**)  | `1.23 us` (🚀 **3.86x faster**)   |
| **`1000`** | `18.73 us` (✅ **1.00x**)    | `5.72 us` (🚀 **3.28x faster**)  | `4.54 us` (🚀 **4.13x faster**)  | `4.61 us` (🚀 **4.06x faster**)   |

### random_string

String sets where the values are random.

|            | `HashSet(classic)`          | `HashSet(foldhash)`              | `FzStringSet`                    | `fz_string_set`                   |
|:-----------|:----------------------------|:---------------------------------|:---------------------------------|:--------------------------------- |
| **`3`**    | `95.37 ns` (✅ **1.00x**)    | `36.70 ns` (🚀 **2.60x faster**)  | `42.89 ns` (🚀 **2.22x faster**)  | `30.66 ns` (🚀 **3.11x faster**)   |
| **`16`**   | `463.78 ns` (✅ **1.00x**)   | `196.70 ns` (🚀 **2.36x faster**) | `248.13 ns` (🚀 **1.87x faster**) | `165.23 ns` (🚀 **2.81x faster**)  |
| **`256`**  | `7.40 us` (✅ **1.00x**)     | `3.19 us` (🚀 **2.32x faster**)   | `3.91 us` (🚀 **1.89x faster**)   | `2.60 us` (🚀 **2.84x faster**)    |
| **`1000`** | `30.06 us` (✅ **1.00x**)    | `13.35 us` (🚀 **2.25x faster**)  | `16.09 us` (🚀 **1.87x faster**)  | `10.89 us` (🚀 **2.76x faster**)   |

### prefixed_string

String sets where the values are random but share a common prefix.

|            | `HashSet(classic)`          | `HashSet(foldhash)`              | `FzStringSet`                    | `fz_string_set`                   |
|:-----------|:----------------------------|:---------------------------------|:---------------------------------|:--------------------------------- |
| **`3`**    | `94.89 ns` (✅ **1.00x**)    | `42.38 ns` (🚀 **2.24x faster**)  | `46.34 ns` (🚀 **2.05x faster**)  | `32.12 ns` (🚀 **2.95x faster**)   |
| **`16`**   | `506.01 ns` (✅ **1.00x**)   | `217.12 ns` (🚀 **2.33x faster**) | `222.23 ns` (🚀 **2.28x faster**) | `163.41 ns` (🚀 **3.10x faster**)  |
| **`256`**  | `8.24 us` (✅ **1.00x**)     | `3.69 us` (🚀 **2.23x faster**)   | `3.98 us` (🚀 **2.07x faster**)   | `2.76 us` (🚀 **2.98x faster**)    |
| **`1000`** | `33.01 us` (✅ **1.00x**)    | `15.52 us` (🚀 **2.13x faster**)  | `16.72 us` (🚀 **1.97x faster**)  | `10.88 us` (🚀 **3.03x faster**)   |

### hashed

Sets with a complex value type that is hashable.

|            | `HashSet(classic)`          | `HashSet(foldhash)`              | `FzHashSet`                      | `fz_hash_set`                     |
|:-----------|:----------------------------|:---------------------------------|:---------------------------------|:--------------------------------- |
| **`3`**    | `112.27 ns` (✅ **1.00x**)   | `39.03 ns` (🚀 **2.88x faster**)  | `33.61 ns` (🚀 **3.34x faster**)  | `31.02 ns` (🚀 **3.62x faster**)   |
| **`16`**   | `539.45 ns` (✅ **1.00x**)   | `202.56 ns` (🚀 **2.66x faster**) | `188.83 ns` (🚀 **2.86x faster**) | `187.25 ns` (🚀 **2.88x faster**)  |
| **`256`**  | `8.69 us` (✅ **1.00x**)     | `3.27 us` (🚀 **2.66x faster**)   | `2.87 us` (🚀 **3.03x faster**)   | `2.91 us` (🚀 **2.99x faster**)    |
| **`1000`** | `35.50 us` (✅ **1.00x**)    | `13.41 us` (🚀 **2.65x faster**)  | `12.02 us` (🚀 **2.95x faster**)  | `11.71 us` (🚀 **3.03x faster**)   |

### ordered

Sets with a complex value type that is ordered.

|            | `BTreeSet`                | `FzOrderedSet`                   | `fz_ordered_set`                  |
|:-----------|:--------------------------|:---------------------------------|:--------------------------------- |
| **`3`**    | `77.64 ns` (✅ **1.00x**)  | `71.60 ns` (✅ **1.08x faster**)  | `31.97 ns` (🚀 **2.43x faster**)   |
| **`16`**   | `881.34 ns` (✅ **1.00x**) | `650.29 ns` (✅ **1.36x faster**) | `636.88 ns` (✅ **1.38x faster**)  |
| **`256`**  | `30.91 us` (✅ **1.00x**)  | `19.03 us` (✅ **1.62x faster**)  | `18.77 us` (✅ **1.65x faster**)   |
| **`1000`** | `220.06 us` (✅ **1.00x**) | `181.13 us` (✅ **1.21x faster**) | `184.28 us` (✅ **1.19x faster**)  |

---
Made with [criterion-table](https://github.com/nu11ptr/criterion-table)

