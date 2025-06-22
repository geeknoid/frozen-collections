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
| **`3`**    | `41.35 ns` (✅ **1.00x**)    | `12.34 ns` (🚀 **3.35x faster**)  | `4.06 ns` (🚀 **10.19x faster**)   | `4.41 ns` (🚀 **9.37x faster**)     |
| **`16`**   | `230.49 ns` (✅ **1.00x**)   | `58.57 ns` (🚀 **3.94x faster**)  | `20.37 ns` (🚀 **11.31x faster**)  | `22.34 ns` (🚀 **10.32x faster**)   |
| **`256`**  | `3.69 us` (✅ **1.00x**)     | `984.78 ns` (🚀 **3.74x faster**) | `345.51 ns` (🚀 **10.67x faster**) | `343.21 ns` (🚀 **10.74x faster**)  |
| **`1000`** | `13.99 us` (✅ **1.00x**)    | `4.04 us` (🚀 **3.46x faster**)   | `1.27 us` (🚀 **10.98x faster**)   | `1.28 us` (🚀 **10.91x faster**)    |

### sparse_scalar

Scalar sets where the values are in a non-contiguous range.

|            | `HashSet(classic)`          | `HashSet(foldhash)`             | `FzScalarSet`                     | `fz_scalar_set`                    |
|:-----------|:----------------------------|:--------------------------------|:----------------------------------|:---------------------------------- |
| **`3`**    | `43.18 ns` (✅ **1.00x**)    | `11.56 ns` (🚀 **3.74x faster**) | `4.14 ns` (🚀 **10.42x faster**)   | `4.58 ns` (🚀 **9.42x faster**)     |
| **`16`**   | `214.82 ns` (✅ **1.00x**)   | `58.69 ns` (🚀 **3.66x faster**) | `22.01 ns` (🚀 **9.76x faster**)   | `21.24 ns` (🚀 **10.11x faster**)   |
| **`256`**  | `3.51 us` (✅ **1.00x**)     | `1.05 us` (🚀 **3.33x faster**)  | `347.87 ns` (🚀 **10.09x faster**) | `342.61 ns` (🚀 **10.25x faster**)  |
| **`1000`** | `13.65 us` (✅ **1.00x**)    | `4.46 us` (🚀 **3.06x faster**)  | `1.30 us` (🚀 **10.51x faster**)   | `1.29 us` (🚀 **10.58x faster**)    |

### random_scalar

Scalar sets where the values are randomly distributed.

|            | `HashSet(classic)`          | `HashSet(foldhash)`              | `FzScalarSet`                    | `fz_scalar_set`                   |
|:-----------|:----------------------------|:---------------------------------|:---------------------------------|:--------------------------------- |
| **`3`**    | `39.65 ns` (✅ **1.00x**)    | `11.86 ns` (🚀 **3.34x faster**)  | `7.35 ns` (🚀 **5.40x faster**)   | `5.75 ns` (🚀 **6.90x faster**)    |
| **`16`**   | `215.56 ns` (✅ **1.00x**)   | `57.22 ns` (🚀 **3.77x faster**)  | `41.44 ns` (🚀 **5.20x faster**)  | `39.73 ns` (🚀 **5.42x faster**)   |
| **`256`**  | `3.49 us` (✅ **1.00x**)     | `952.74 ns` (🚀 **3.67x faster**) | `727.06 ns` (🚀 **4.81x faster**) | `734.62 ns` (🚀 **4.76x faster**)  |
| **`1000`** | `13.86 us` (✅ **1.00x**)    | `4.19 us` (🚀 **3.31x faster**)   | `2.85 us` (🚀 **4.86x faster**)   | `2.88 us` (🚀 **4.81x faster**)    |

### random_string

String sets where the values are random.

|            | `HashSet(classic)`          | `HashSet(foldhash)`              | `FzStringSet`                    | `fz_string_set`                   |
|:-----------|:----------------------------|:---------------------------------|:---------------------------------|:--------------------------------- |
| **`3`**    | `71.86 ns` (✅ **1.00x**)    | `31.86 ns` (🚀 **2.26x faster**)  | `45.20 ns` (✅ **1.59x faster**)  | `40.09 ns` (✅ **1.79x faster**)   |
| **`16`**   | `391.98 ns` (✅ **1.00x**)   | `167.19 ns` (🚀 **2.34x faster**) | `217.38 ns` (🚀 **1.80x faster**) | `206.26 ns` (🚀 **1.90x faster**)  |
| **`256`**  | `6.24 us` (✅ **1.00x**)     | `2.32 us` (🚀 **2.69x faster**)   | `4.11 us` (✅ **1.52x faster**)   | `2.72 us` (🚀 **2.29x faster**)    |
| **`1000`** | `26.13 us` (✅ **1.00x**)    | `9.79 us` (🚀 **2.67x faster**)   | `16.33 us` (✅ **1.60x faster**)  | `14.26 us` (🚀 **1.83x faster**)   |

### prefixed_string

String sets where the values are random but share a common prefix.

|            | `HashSet(classic)`          | `HashSet(foldhash)`              | `FzStringSet`                    | `fz_string_set`                   |
|:-----------|:----------------------------|:---------------------------------|:---------------------------------|:--------------------------------- |
| **`3`**    | `74.62 ns` (✅ **1.00x**)    | `34.13 ns` (🚀 **2.19x faster**)  | `43.18 ns` (✅ **1.73x faster**)  | `40.19 ns` (🚀 **1.86x faster**)   |
| **`16`**   | `413.84 ns` (✅ **1.00x**)   | `175.48 ns` (🚀 **2.36x faster**) | `228.78 ns` (🚀 **1.81x faster**) | `187.36 ns` (🚀 **2.21x faster**)  |
| **`256`**  | `7.06 us` (✅ **1.00x**)     | `2.97 us` (🚀 **2.38x faster**)   | `4.66 us` (✅ **1.51x faster**)   | `3.05 us` (🚀 **2.31x faster**)    |
| **`1000`** | `32.70 us` (✅ **1.00x**)    | `13.46 us` (🚀 **2.43x faster**)  | `19.43 us` (✅ **1.68x faster**)  | `13.15 us` (🚀 **2.49x faster**)   |

### hashed

Sets with a complex key type that is hashable.

|            | `HashSet(classic)`          | `HashSet(foldhash)`              | `FzHashSet`                      | `fz_hash_set`                     |
|:-----------|:----------------------------|:---------------------------------|:---------------------------------|:--------------------------------- |
| **`3`**    | `102.59 ns` (✅ **1.00x**)   | `35.78 ns` (🚀 **2.87x faster**)  | `37.24 ns` (🚀 **2.75x faster**)  | `47.79 ns` (🚀 **2.15x faster**)   |
| **`16`**   | `455.30 ns` (✅ **1.00x**)   | `184.32 ns` (🚀 **2.47x faster**) | `253.99 ns` (✅ **1.79x faster**) | `185.86 ns` (🚀 **2.45x faster**)  |
| **`256`**  | `8.17 us` (✅ **1.00x**)     | `2.58 us` (🚀 **3.17x faster**)   | `3.22 us` (🚀 **2.53x faster**)   | `3.43 us` (🚀 **2.38x faster**)    |
| **`1000`** | `33.17 us` (✅ **1.00x**)    | `10.62 us` (🚀 **3.12x faster**)  | `13.52 us` (🚀 **2.45x faster**)  | `12.79 us` (🚀 **2.59x faster**)   |

### ordered

Sets with a complex key type that is ordered.

|            | `BTreeSet`                | `FzOrderedSet`                   | `fz_ordered_set`                  |
|:-----------|:--------------------------|:---------------------------------|:--------------------------------- |
| **`3`**    | `87.16 ns` (✅ **1.00x**)  | `78.47 ns` (✅ **1.11x faster**)  | `50.21 ns` (✅ **1.74x faster**)   |
| **`16`**   | `948.58 ns` (✅ **1.00x**) | `700.63 ns` (✅ **1.35x faster**) | `709.01 ns` (✅ **1.34x faster**)  |
| **`256`**  | `37.64 us` (✅ **1.00x**)  | `21.35 us` (✅ **1.76x faster**)  | `22.87 us` (✅ **1.65x faster**)   |
| **`1000`** | `260.69 us` (✅ **1.00x**) | `209.64 us` (✅ **1.24x faster**) | `213.07 us` (✅ **1.22x faster**)  |

---
Made with [criterion-table](https://github.com/nu11ptr/criterion-table)

