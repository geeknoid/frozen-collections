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

When you see `HashSet(classic)` vs. `HashSet(ahash)` this reflects the performance difference between the
normal hasher used by the standard collections as opposed to the performnace that the
`ahash` hasher provides.

The benchmarks assume a 50% hit rate when probing for lookup, meaning that
half the queries are for non-existing data. Some algorithms perform differently between
present vs. non-existing cases, so real world performance of these algorithms depends on the
real world hit rate you experience.

## Benchmark Results

### dense_scalar

Scalar sets where the values are in a contiguous range.

|            | `HashSet(classic)`          | `HashSet(ahash)`                | `fz_scalar_set(vector)`          | `fz_scalar_set(literals)`           |
|:-----------|:----------------------------|:--------------------------------|:---------------------------------|:----------------------------------- |
| **`3`**    | `45.62 ns` (✅ **1.00x**)    | `17.55 ns` (🚀 **2.60x faster**) | `4.31 ns` (🚀 **10.59x faster**)  | `4.31 ns` (🚀 **10.59x faster**)     |
| **`10`**   | `163.84 ns` (✅ **1.00x**)   | `56.52 ns` (🚀 **2.90x faster**) | `14.94 ns` (🚀 **10.97x faster**) | `14.80 ns` (🚀 **11.07x faster**)    |
| **`256`**  | `3.98 us` (✅ **1.00x**)     | `1.55 us` (🚀 **2.56x faster**)  | `412.36 ns` (🚀 **9.65x faster**) | `378.67 ns` (🚀 **10.50x faster**)   |
| **`1000`** | `15.91 us` (✅ **1.00x**)    | `6.22 us` (🚀 **2.56x faster**)  | `1.59 us` (🚀 **9.98x faster**)   | `1.45 us` (🚀 **10.98x faster**)     |

### sparse_scalar

Scalar sets where the values are in a non-contiguous range.

|            | `HashSet(classic)`          | `HashSet(ahash)`                | `fz_scalar_set(vector)`          | `fz_scalar_set(literals)`           |
|:-----------|:----------------------------|:--------------------------------|:---------------------------------|:----------------------------------- |
| **`3`**    | `45.58 ns` (✅ **1.00x**)    | `19.07 ns` (🚀 **2.39x faster**) | `6.20 ns` (🚀 **7.36x faster**)   | `4.94 ns` (🚀 **9.22x faster**)      |
| **`10`**   | `151.38 ns` (✅ **1.00x**)   | `60.38 ns` (🚀 **2.51x faster**) | `22.55 ns` (🚀 **6.71x faster**)  | `15.84 ns` (🚀 **9.55x faster**)     |
| **`256`**  | `4.04 us` (✅ **1.00x**)     | `1.54 us` (🚀 **2.63x faster**)  | `596.88 ns` (🚀 **6.76x faster**) | `378.58 ns` (🚀 **10.66x faster**)   |
| **`1000`** | `15.81 us` (✅ **1.00x**)    | `6.13 us` (🚀 **2.58x faster**)  | `2.32 us` (🚀 **6.83x faster**)   | `1.46 us` (🚀 **10.81x faster**)     |

### random_scalar

Scalar sets where the values are randomly distributed.

|            | `HashSet(classic)`          | `HashSet(ahash)`                | `fz_scalar_set(vector)`          | `fz_scalar_set(literals)`           |
|:-----------|:----------------------------|:--------------------------------|:---------------------------------|:----------------------------------- |
| **`3`**    | `45.10 ns` (✅ **1.00x**)    | `17.09 ns` (🚀 **2.64x faster**) | `10.56 ns` (🚀 **4.27x faster**)  | `18.23 ns` (🚀 **2.47x faster**)     |
| **`10`**   | `151.84 ns` (✅ **1.00x**)   | `63.18 ns` (🚀 **2.40x faster**) | `24.61 ns` (🚀 **6.17x faster**)  | `24.24 ns` (🚀 **6.26x faster**)     |
| **`256`**  | `4.03 us` (✅ **1.00x**)     | `1.51 us` (🚀 **2.66x faster**)  | `982.10 ns` (🚀 **4.10x faster**) | `979.49 ns` (🚀 **4.11x faster**)    |
| **`1000`** | `15.71 us` (✅ **1.00x**)    | `6.13 us` (🚀 **2.56x faster**)  | `3.91 us` (🚀 **4.02x faster**)   | `3.93 us` (🚀 **3.99x faster**)      |

### random_string

String sets where the values are random.

|            | `HashSet(classic)`          | `HashSet(ahash)`                 | `fz_string_set(vector)`          | `fz_string_set(literals)`           |
|:-----------|:----------------------------|:---------------------------------|:---------------------------------|:----------------------------------- |
| **`3`**    | `80.20 ns` (✅ **1.00x**)    | `41.57 ns` (🚀 **1.93x faster**)  | `38.73 ns` (🚀 **2.07x faster**)  | `24.27 ns` (🚀 **3.30x faster**)     |
| **`10`**   | `259.64 ns` (✅ **1.00x**)   | `138.89 ns` (🚀 **1.87x faster**) | `148.60 ns` (✅ **1.75x faster**) | `97.32 ns` (🚀 **2.67x faster**)     |
| **`256`**  | `6.81 us` (✅ **1.00x**)     | `3.61 us` (🚀 **1.88x faster**)   | `3.92 us` (✅ **1.74x faster**)   | `2.81 us` (🚀 **2.42x faster**)      |
| **`1000`** | `27.89 us` (✅ **1.00x**)    | `15.07 us` (🚀 **1.85x faster**)  | `15.45 us` (🚀 **1.81x faster**)  | `11.18 us` (🚀 **2.50x faster**)     |

### prefixed_string

String sets where the values are random, but share a common prefix.

|            | `HashSet(classic)`          | `HashSet(ahash)`                 | `fz_string_set(vector)`          | `fz_string_set(literals)`           |
|:-----------|:----------------------------|:---------------------------------|:---------------------------------|:----------------------------------- |
| **`3`**    | `86.24 ns` (✅ **1.00x**)    | `46.96 ns` (🚀 **1.84x faster**)  | `39.61 ns` (🚀 **2.18x faster**)  | `30.80 ns` (🚀 **2.80x faster**)     |
| **`10`**   | `284.81 ns` (✅ **1.00x**)   | `149.95 ns` (🚀 **1.90x faster**) | `140.28 ns` (🚀 **2.03x faster**) | `109.55 ns` (🚀 **2.60x faster**)    |
| **`256`**  | `7.45 us` (✅ **1.00x**)     | `3.96 us` (🚀 **1.88x faster**)   | `3.97 us` (🚀 **1.88x faster**)   | `2.76 us` (🚀 **2.70x faster**)      |
| **`1000`** | `30.57 us` (✅ **1.00x**)    | `16.69 us` (🚀 **1.83x faster**)  | `15.97 us` (🚀 **1.91x faster**)  | `10.73 us` (🚀 **2.85x faster**)     |

### hashed

Sets with a complex key type that is hashable.

|            | `HashSet(classic)`          | `HashSet(ahash)`                 | `fz_hash_set(vector)`            | `fz_hash_set(literals)`           |
|:-----------|:----------------------------|:---------------------------------|:---------------------------------|:--------------------------------- |
| **`3`**    | `106.69 ns` (✅ **1.00x**)   | `48.25 ns` (🚀 **2.21x faster**)  | `42.40 ns` (🚀 **2.52x faster**)  | `62.77 ns` (✅ **1.70x faster**)   |
| **`10`**   | `343.05 ns` (✅ **1.00x**)   | `171.61 ns` (🚀 **2.00x faster**) | `134.06 ns` (🚀 **2.56x faster**) | `133.49 ns` (🚀 **2.57x faster**)  |
| **`256`**  | `8.44 us` (✅ **1.00x**)     | `4.36 us` (🚀 **1.93x faster**)   | `3.78 us` (🚀 **2.23x faster**)   | `3.92 us` (🚀 **2.15x faster**)    |
| **`1000`** | `33.75 us` (✅ **1.00x**)    | `17.52 us` (🚀 **1.93x faster**)  | `16.33 us` (🚀 **2.07x faster**)  | `16.03 us` (🚀 **2.11x faster**)   |

### ordered

Sets with a complex key type that is ordered.

|            | `BTreeSet`                | `fz_hash_set(vector)`            | `fz_ordered_set(literals)`           |
|:-----------|:--------------------------|:---------------------------------|:------------------------------------ |
| **`3`**    | `75.79 ns` (✅ **1.00x**)  | `72.35 ns` (✅ **1.05x faster**)  | `49.78 ns` (✅ **1.52x faster**)      |
| **`10`**   | `579.71 ns` (✅ **1.00x**) | `617.58 ns` (✅ **1.07x slower**) | `611.26 ns` (✅ **1.05x slower**)     |
| **`256`**  | `32.13 us` (✅ **1.00x**)  | `27.21 us` (✅ **1.18x faster**)  | `26.68 us` (✅ **1.20x faster**)      |
| **`1000`** | `223.10 us` (✅ **1.00x**) | `198.15 us` (✅ **1.13x faster**) | `195.40 us` (✅ **1.14x faster**)     |

---
Made with [criterion-table](https://github.com/nu11ptr/criterion-table)

