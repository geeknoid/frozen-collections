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
| **`3`**    | `45.63 ns` (✅ **1.00x**)    | `17.59 ns` (🚀 **2.59x faster**) | `4.74 ns` (🚀 **9.63x faster**)   | `4.31 ns` (🚀 **10.59x faster**)     |
| **`16`**   | `242.30 ns` (✅ **1.00x**)   | `95.88 ns` (🚀 **2.53x faster**) | `28.13 ns` (🚀 **8.61x faster**)  | `24.36 ns` (🚀 **9.95x faster**)     |
| **`256`**  | `4.05 us` (✅ **1.00x**)     | `1.51 us` (🚀 **2.69x faster**)  | `470.94 ns` (🚀 **8.61x faster**) | `412.29 ns` (🚀 **9.83x faster**)    |
| **`1000`** | `15.72 us` (✅ **1.00x**)    | `6.12 us` (🚀 **2.57x faster**)  | `1.82 us` (🚀 **8.65x faster**)   | `1.59 us` (🚀 **9.86x faster**)      |

### sparse_scalar

Scalar sets where the values are in a non-contiguous range.

|            | `HashSet(classic)`          | `HashSet(ahash)`                | `fz_scalar_set(vector)`           | `fz_scalar_set(literals)`           |
|:-----------|:----------------------------|:--------------------------------|:----------------------------------|:----------------------------------- |
| **`3`**    | `49.63 ns` (✅ **1.00x**)    | `17.57 ns` (🚀 **2.82x faster**) | `4.60 ns` (🚀 **10.80x faster**)   | `5.89 ns` (🚀 **8.42x faster**)      |
| **`16`**   | `251.71 ns` (✅ **1.00x**)   | `91.15 ns` (🚀 **2.76x faster**) | `20.71 ns` (🚀 **12.15x faster**)  | `23.82 ns` (🚀 **10.57x faster**)    |
| **`256`**  | `4.03 us` (✅ **1.00x**)     | `1.57 us` (🚀 **2.57x faster**)  | `330.83 ns` (🚀 **12.18x faster**) | `379.66 ns` (🚀 **10.61x faster**)   |
| **`1000`** | `15.72 us` (✅ **1.00x**)    | `6.09 us` (🚀 **2.58x faster**)  | `1.27 us` (🚀 **12.39x faster**)   | `1.46 us` (🚀 **10.76x faster**)     |

### random_scalar

Scalar sets where the values are randomly distributed.

|            | `HashSet(classic)`          | `HashSet(ahash)`                | `fz_scalar_set(vector)`          | `fz_scalar_set(literals)`           |
|:-----------|:----------------------------|:--------------------------------|:---------------------------------|:----------------------------------- |
| **`3`**    | `48.23 ns` (✅ **1.00x**)    | `17.15 ns` (🚀 **2.81x faster**) | `9.19 ns` (🚀 **5.25x faster**)   | `11.41 ns` (🚀 **4.23x faster**)     |
| **`16`**   | `251.72 ns` (✅ **1.00x**)   | `96.97 ns` (🚀 **2.60x faster**) | `53.23 ns` (🚀 **4.73x faster**)  | `53.32 ns` (🚀 **4.72x faster**)     |
| **`256`**  | `4.03 us` (✅ **1.00x**)     | `1.55 us` (🚀 **2.60x faster**)  | `814.80 ns` (🚀 **4.94x faster**) | `814.97 ns` (🚀 **4.94x faster**)    |
| **`1000`** | `15.68 us` (✅ **1.00x**)    | `6.15 us` (🚀 **2.55x faster**)  | `3.27 us` (🚀 **4.79x faster**)   | `3.22 us` (🚀 **4.86x faster**)      |

### random_string

String sets where the values are random.

|            | `HashSet(classic)`          | `HashSet(ahash)`                 | `fz_string_set(vector)`          | `fz_string_set(literals)`           |
|:-----------|:----------------------------|:---------------------------------|:---------------------------------|:----------------------------------- |
| **`3`**    | `82.65 ns` (✅ **1.00x**)    | `42.94 ns` (🚀 **1.92x faster**)  | `39.36 ns` (🚀 **2.10x faster**)  | `26.20 ns` (🚀 **3.15x faster**)     |
| **`16`**   | `434.62 ns` (✅ **1.00x**)   | `225.46 ns` (🚀 **1.93x faster**) | `221.74 ns` (🚀 **1.96x faster**) | `141.19 ns` (🚀 **3.08x faster**)    |
| **`256`**  | `6.79 us` (✅ **1.00x**)     | `3.58 us` (🚀 **1.90x faster**)   | `3.49 us` (🚀 **1.95x faster**)   | `2.56 us` (🚀 **2.66x faster**)      |
| **`1000`** | `27.72 us` (✅ **1.00x**)    | `14.98 us` (🚀 **1.85x faster**)  | `13.96 us` (🚀 **1.99x faster**)  | `10.23 us` (🚀 **2.71x faster**)     |

### prefixed_string

String sets where the values are random, but share a common prefix.

|            | `HashSet(classic)`          | `HashSet(ahash)`                 | `fz_string_set(vector)`          | `fz_string_set(literals)`           |
|:-----------|:----------------------------|:---------------------------------|:---------------------------------|:----------------------------------- |
| **`3`**    | `81.22 ns` (✅ **1.00x**)    | `43.76 ns` (🚀 **1.86x faster**)  | `35.47 ns` (🚀 **2.29x faster**)  | `24.98 ns` (🚀 **3.25x faster**)     |
| **`16`**   | `459.80 ns` (✅ **1.00x**)   | `243.87 ns` (🚀 **1.89x faster**) | `214.12 ns` (🚀 **2.15x faster**) | `135.28 ns` (🚀 **3.40x faster**)    |
| **`256`**  | `7.55 us` (✅ **1.00x**)     | `4.00 us` (🚀 **1.89x faster**)   | `3.68 us` (🚀 **2.05x faster**)   | `2.84 us` (🚀 **2.66x faster**)      |
| **`1000`** | `30.38 us` (✅ **1.00x**)    | `16.47 us` (🚀 **1.84x faster**)  | `14.06 us` (🚀 **2.16x faster**)  | `10.64 us` (🚀 **2.85x faster**)     |

### hashed

Sets with a complex key type that is hashable.

|            | `HashSet(classic)`          | `HashSet(ahash)`                 | `fz_hash_set(vector)`            | `fz_hash_set(literals)`           |
|:-----------|:----------------------------|:---------------------------------|:---------------------------------|:--------------------------------- |
| **`3`**    | `92.20 ns` (✅ **1.00x**)    | `50.63 ns` (🚀 **1.82x faster**)  | `40.94 ns` (🚀 **2.25x faster**)  | `57.09 ns` (✅ **1.61x faster**)   |
| **`16`**   | `515.48 ns` (✅ **1.00x**)   | `265.23 ns` (🚀 **1.94x faster**) | `207.83 ns` (🚀 **2.48x faster**) | `231.67 ns` (🚀 **2.23x faster**)  |
| **`256`**  | `8.34 us` (✅ **1.00x**)     | `4.33 us` (🚀 **1.93x faster**)   | `3.80 us` (🚀 **2.20x faster**)   | `3.75 us` (🚀 **2.22x faster**)    |
| **`1000`** | `33.77 us` (✅ **1.00x**)    | `17.59 us` (🚀 **1.92x faster**)  | `16.08 us` (🚀 **2.10x faster**)  | `15.52 us` (🚀 **2.18x faster**)   |

### ordered

Sets with a complex key type that is ordered.

|            | `BTreeSet`                | `fz_hash_set(vector)`            | `fz_ordered_set(literals)`           |
|:-----------|:--------------------------|:---------------------------------|:------------------------------------ |
| **`3`**    | `70.62 ns` (✅ **1.00x**)  | `62.25 ns` (✅ **1.13x faster**)  | `59.52 ns` (✅ **1.19x faster**)      |
| **`16`**   | `954.52 ns` (✅ **1.00x**) | `984.68 ns` (✅ **1.03x slower**) | `990.07 ns` (✅ **1.04x slower**)     |
| **`256`**  | `30.65 us` (✅ **1.00x**)  | `27.37 us` (✅ **1.12x faster**)  | `27.01 us` (✅ **1.13x faster**)      |
| **`1000`** | `219.26 us` (✅ **1.00x**) | `199.92 us` (✅ **1.10x faster**) | `199.09 us` (✅ **1.10x faster**)     |

---
Made with [criterion-table](https://github.com/nu11ptr/criterion-table)

