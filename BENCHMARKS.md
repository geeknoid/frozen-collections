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
| **`3`**    | `26.83 ns` (✅ **1.00x**)    | `7.75 ns` (🚀 **3.46x faster**)  | `2.69 ns` (🚀 **9.96x faster**)   | `3.98 ns` (🚀 **6.73x faster**)     |
| **`16`**   | `207.07 ns` (✅ **1.00x**)   | `65.75 ns` (🚀 **3.15x faster**) | `21.39 ns` (🚀 **9.68x faster**)  | `21.06 ns` (🚀 **9.83x faster**)    |
| **`256`**  | `3.27 us` (✅ **1.00x**)     | `1.05 us` (🚀 **3.12x faster**)  | `327.73 ns` (🚀 **9.96x faster**) | `310.68 ns` (🚀 **10.51x faster**)  |
| **`1000`** | `12.84 us` (✅ **1.00x**)    | `3.82 us` (🚀 **3.36x faster**)  | `1.17 us` (🚀 **11.01x faster**)  | `1.18 us` (🚀 **10.91x faster**)    |

### sparse_scalar

Scalar sets where the values are in a non-contiguous range.

|            | `HashSet(classic)`          | `HashSet(foldhash)`             | `FzScalarSet`                     | `fz_scalar_set`                   |
|:-----------|:----------------------------|:--------------------------------|:----------------------------------|:--------------------------------- |
| **`3`**    | `37.11 ns` (✅ **1.00x**)    | `10.79 ns` (🚀 **3.44x faster**) | `3.73 ns` (🚀 **9.95x faster**)    | `3.74 ns` (🚀 **9.92x faster**)    |
| **`16`**   | `177.22 ns` (✅ **1.00x**)   | `52.82 ns` (🚀 **3.36x faster**) | `18.95 ns` (🚀 **9.35x faster**)   | `18.29 ns` (🚀 **9.69x faster**)   |
| **`256`**  | `3.95 us` (✅ **1.00x**)     | `1.08 us` (🚀 **3.66x faster**)  | `379.45 ns` (🚀 **10.42x faster**) | `481.16 ns` (🚀 **8.22x faster**)  |
| **`1000`** | `16.41 us` (✅ **1.00x**)    | `5.02 us` (🚀 **3.27x faster**)  | `1.47 us` (🚀 **11.18x faster**)   | `1.47 us` (🚀 **11.15x faster**)   |

### random_scalar

Scalar sets where the values are randomly distributed.

|            | `HashSet(classic)`          | `HashSet(foldhash)`              | `FzScalarSet`                    | `fz_scalar_set`                   |
|:-----------|:----------------------------|:---------------------------------|:---------------------------------|:--------------------------------- |
| **`3`**    | `45.62 ns` (✅ **1.00x**)    | `13.70 ns` (🚀 **3.33x faster**)  | `8.20 ns` (🚀 **5.56x faster**)   | `9.82 ns` (🚀 **4.64x faster**)    |
| **`16`**   | `181.57 ns` (✅ **1.00x**)   | `55.33 ns` (🚀 **3.28x faster**)  | `38.66 ns` (🚀 **4.70x faster**)  | `39.36 ns` (🚀 **4.61x faster**)   |
| **`256`**  | `2.83 us` (✅ **1.00x**)     | `936.44 ns` (🚀 **3.02x faster**) | `688.31 ns` (🚀 **4.10x faster**) | `676.42 ns` (🚀 **4.18x faster**)  |
| **`1000`** | `11.42 us` (✅ **1.00x**)    | `3.52 us` (🚀 **3.25x faster**)   | `2.73 us` (🚀 **4.18x faster**)   | `2.78 us` (🚀 **4.10x faster**)    |

### random_string

String sets where the values are random.

|            | `HashSet(classic)`          | `HashSet(foldhash)`              | `FzStringSet`                    | `fz_string_set`                   |
|:-----------|:----------------------------|:---------------------------------|:---------------------------------|:--------------------------------- |
| **`3`**    | `59.10 ns` (✅ **1.00x**)    | `26.54 ns` (🚀 **2.23x faster**)  | `25.13 ns` (🚀 **2.35x faster**)  | `20.80 ns` (🚀 **2.84x faster**)   |
| **`16`**   | `331.63 ns` (✅ **1.00x**)   | `124.91 ns` (🚀 **2.65x faster**) | `165.83 ns` (🚀 **2.00x faster**) | `116.13 ns` (🚀 **2.86x faster**)  |
| **`256`**  | `5.45 us` (✅ **1.00x**)     | `2.19 us` (🚀 **2.48x faster**)   | `3.08 us` (✅ **1.77x faster**)   | `2.38 us` (🚀 **2.29x faster**)    |
| **`1000`** | `23.47 us` (✅ **1.00x**)    | `9.63 us` (🚀 **2.44x faster**)   | `13.07 us` (✅ **1.80x faster**)  | `9.15 us` (🚀 **2.57x faster**)    |

### prefixed_string

String sets where the values are random, but share a common prefix.

|            | `HashSet(classic)`          | `HashSet(foldhash)`              | `FzStringSet`                    | `fz_string_set`                   |
|:-----------|:----------------------------|:---------------------------------|:---------------------------------|:--------------------------------- |
| **`3`**    | `68.35 ns` (✅ **1.00x**)    | `30.15 ns` (🚀 **2.27x faster**)  | `31.39 ns` (🚀 **2.18x faster**)  | `25.52 ns` (🚀 **2.68x faster**)   |
| **`16`**   | `355.08 ns` (✅ **1.00x**)   | `155.86 ns` (🚀 **2.28x faster**) | `171.22 ns` (🚀 **2.07x faster**) | `131.68 ns` (🚀 **2.70x faster**)  |
| **`256`**  | `5.80 us` (✅ **1.00x**)     | `2.50 us` (🚀 **2.32x faster**)   | `3.24 us` (✅ **1.79x faster**)   | `2.23 us` (🚀 **2.61x faster**)    |
| **`1000`** | `25.68 us` (✅ **1.00x**)    | `11.92 us` (🚀 **2.15x faster**)  | `16.69 us` (✅ **1.54x faster**)  | `10.13 us` (🚀 **2.53x faster**)   |

### hashed

Sets with a complex key type that is hashable.

|            | `HashSet(classic)`          | `HashSet(foldhash)`              | `FzHashSet`                      | `fz_hash_set`                     |
|:-----------|:----------------------------|:---------------------------------|:---------------------------------|:--------------------------------- |
| **`3`**    | `78.97 ns` (✅ **1.00x**)    | `26.22 ns` (🚀 **3.01x faster**)  | `43.42 ns` (🚀 **1.82x faster**)  | `45.17 ns` (✅ **1.75x faster**)   |
| **`16`**   | `424.21 ns` (✅ **1.00x**)   | `153.06 ns` (🚀 **2.77x faster**) | `137.41 ns` (🚀 **3.09x faster**) | `148.56 ns` (🚀 **2.86x faster**)  |
| **`256`**  | `6.65 us` (✅ **1.00x**)     | `2.60 us` (🚀 **2.56x faster**)   | `2.84 us` (🚀 **2.34x faster**)   | `2.58 us` (🚀 **2.57x faster**)    |
| **`1000`** | `27.76 us` (✅ **1.00x**)    | `10.24 us` (🚀 **2.71x faster**)  | `11.03 us` (🚀 **2.52x faster**)  | `10.11 us` (🚀 **2.75x faster**)   |

### ordered

Sets with a complex key type that is ordered.

|            | `BTreeSet`                | `FzOrderedSet`                   | `fz_ordered_set`                  |
|:-----------|:--------------------------|:---------------------------------|:--------------------------------- |
| **`3`**    | `68.85 ns` (✅ **1.00x**)  | `62.66 ns` (✅ **1.10x faster**)  | `45.39 ns` (✅ **1.52x faster**)   |
| **`16`**   | `859.33 ns` (✅ **1.00x**) | `896.02 ns` (✅ **1.04x slower**) | `834.60 ns` (✅ **1.03x faster**)  |
| **`256`**  | `29.44 us` (✅ **1.00x**)  | `19.59 us` (✅ **1.50x faster**)  | `18.74 us` (✅ **1.57x faster**)   |
| **`1000`** | `213.63 us` (✅ **1.00x**) | `191.99 us` (✅ **1.11x faster**) | `184.03 us` (✅ **1.16x faster**)  |

---
Made with [criterion-table](https://github.com/nu11ptr/criterion-table)

