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
| **`3`**    | `45.67 ns` (✅ **1.00x**)    | `13.82 ns` (🚀 **3.31x faster**) | `4.31 ns` (🚀 **10.60x faster**)   | `4.30 ns` (🚀 **10.63x faster**)   |
| **`16`**   | `241.20 ns` (✅ **1.00x**)   | `72.30 ns` (🚀 **3.34x faster**) | `23.56 ns` (🚀 **10.24x faster**)  | `24.39 ns` (🚀 **9.89x faster**)   |
| **`256`**  | `4.02 us` (✅ **1.00x**)     | `1.14 us` (🚀 **3.54x faster**)  | `378.70 ns` (🚀 **10.62x faster**) | `414.30 ns` (🚀 **9.70x faster**)  |
| **`1000`** | `15.93 us` (✅ **1.00x**)    | `4.59 us` (🚀 **3.47x faster**)  | `1.45 us` (🚀 **11.00x faster**)   | `1.60 us` (🚀 **9.95x faster**)    |

### sparse_scalar

Scalar sets where the values are in a non-contiguous range.

|            | `HashSet(classic)`          | `HashSet(foldhash)`              | `FzScalarSet`                     | `fz_scalar_set`                    |
|:-----------|:----------------------------|:---------------------------------|:----------------------------------|:---------------------------------- |
| **`3`**    | `45.56 ns` (✅ **1.00x**)    | `20.50 ns` (🚀 **2.22x faster**)  | `4.45 ns` (🚀 **10.23x faster**)   | `4.95 ns` (🚀 **9.21x faster**)     |
| **`16`**   | `239.85 ns` (✅ **1.00x**)   | `112.58 ns` (🚀 **2.13x faster**) | `20.71 ns` (🚀 **11.58x faster**)  | `23.85 ns` (🚀 **10.06x faster**)   |
| **`256`**  | `4.03 us` (✅ **1.00x**)     | `1.83 us` (🚀 **2.21x faster**)   | `330.31 ns` (🚀 **12.21x faster**) | `379.29 ns` (🚀 **10.64x faster**)  |
| **`1000`** | `15.85 us` (✅ **1.00x**)    | `7.10 us` (🚀 **2.23x faster**)   | `1.27 us` (🚀 **12.50x faster**)   | `1.46 us` (🚀 **10.84x faster**)    |

### random_scalar

Scalar sets where the values are randomly distributed.

|            | `HashSet(classic)`          | `HashSet(foldhash)`             | `FzScalarSet`                    | `fz_scalar_set`                   |
|:-----------|:----------------------------|:--------------------------------|:---------------------------------|:--------------------------------- |
| **`3`**    | `45.12 ns` (✅ **1.00x**)    | `13.74 ns` (🚀 **3.28x faster**) | `9.39 ns` (🚀 **4.81x faster**)   | `11.14 ns` (🚀 **4.05x faster**)   |
| **`16`**   | `239.51 ns` (✅ **1.00x**)   | `71.28 ns` (🚀 **3.36x faster**) | `42.80 ns` (🚀 **5.60x faster**)  | `42.35 ns` (🚀 **5.65x faster**)   |
| **`256`**  | `4.01 us` (✅ **1.00x**)     | `1.21 us` (🚀 **3.31x faster**)  | `838.75 ns` (🚀 **4.78x faster**) | `845.33 ns` (🚀 **4.74x faster**)  |
| **`1000`** | `15.77 us` (✅ **1.00x**)    | `4.61 us` (🚀 **3.42x faster**)  | `3.29 us` (🚀 **4.80x faster**)   | `3.26 us` (🚀 **4.84x faster**)    |

### random_string

String sets where the values are random.

|            | `HashSet(classic)`          | `HashSet(foldhash)`              | `FzStringSet`                    | `fz_string_set`                   |
|:-----------|:----------------------------|:---------------------------------|:---------------------------------|:--------------------------------- |
| **`3`**    | `81.83 ns` (✅ **1.00x**)    | `33.91 ns` (🚀 **2.41x faster**)  | `33.21 ns` (🚀 **2.46x faster**)  | `27.27 ns` (🚀 **3.00x faster**)   |
| **`16`**   | `428.93 ns` (✅ **1.00x**)   | `165.96 ns` (🚀 **2.58x faster**) | `199.44 ns` (🚀 **2.15x faster**) | `142.23 ns` (🚀 **3.02x faster**)  |
| **`256`**  | `6.84 us` (✅ **1.00x**)     | `2.81 us` (🚀 **2.44x faster**)   | `3.60 us` (🚀 **1.90x faster**)   | `2.63 us` (🚀 **2.60x faster**)    |
| **`1000`** | `27.88 us` (✅ **1.00x**)    | `11.75 us` (🚀 **2.37x faster**)  | `15.37 us` (🚀 **1.81x faster**)  | `10.43 us` (🚀 **2.67x faster**)   |

### prefixed_string

String sets where the values are random, but share a common prefix.

|            | `HashSet(classic)`          | `HashSet(foldhash)`              | `FzStringSet`                    | `fz_string_set`                   |
|:-----------|:----------------------------|:---------------------------------|:---------------------------------|:--------------------------------- |
| **`3`**    | `87.64 ns` (✅ **1.00x**)    | `35.40 ns` (🚀 **2.48x faster**)  | `39.90 ns` (🚀 **2.20x faster**)  | `30.18 ns` (🚀 **2.90x faster**)   |
| **`16`**   | `467.63 ns` (✅ **1.00x**)   | `190.26 ns` (🚀 **2.46x faster**) | `208.62 ns` (🚀 **2.24x faster**) | `136.42 ns` (🚀 **3.43x faster**)  |
| **`256`**  | `7.50 us` (✅ **1.00x**)     | `3.23 us` (🚀 **2.32x faster**)   | `3.78 us` (🚀 **1.98x faster**)   | `2.93 us` (🚀 **2.56x faster**)    |
| **`1000`** | `30.36 us` (✅ **1.00x**)    | `13.16 us` (🚀 **2.31x faster**)  | `24.79 us` (✅ **1.22x faster**)  | `11.52 us` (🚀 **2.64x faster**)   |

### hashed

Sets with a complex key type that is hashable.

|            | `HashSet(classic)`          | `HashSet(foldhash)`              | `FzHashSet`                      | `fz_hash_set`                     |
|:-----------|:----------------------------|:---------------------------------|:---------------------------------|:--------------------------------- |
| **`3`**    | `103.13 ns` (✅ **1.00x**)   | `35.75 ns` (🚀 **2.88x faster**)  | `58.77 ns` (✅ **1.75x faster**)  | `59.64 ns` (✅ **1.73x faster**)   |
| **`16`**   | `525.55 ns` (✅ **1.00x**)   | `181.13 ns` (🚀 **2.90x faster**) | `194.15 ns` (🚀 **2.71x faster**) | `182.93 ns` (🚀 **2.87x faster**)  |
| **`256`**  | `8.41 us` (✅ **1.00x**)     | `2.98 us` (🚀 **2.82x faster**)   | `2.96 us` (🚀 **2.84x faster**)   | `3.00 us` (🚀 **2.80x faster**)    |
| **`1000`** | `33.60 us` (✅ **1.00x**)    | `12.05 us` (🚀 **2.79x faster**)  | `12.26 us` (🚀 **2.74x faster**)  | `11.81 us` (🚀 **2.84x faster**)   |

### ordered

Sets with a complex key type that is ordered.

|            | `BTreeSet`                | `FzOrderedSet`                   | `fz_ordered_set`                  |
|:-----------|:--------------------------|:---------------------------------|:--------------------------------- |
| **`3`**    | `89.67 ns` (✅ **1.00x**)  | `83.63 ns` (✅ **1.07x faster**)  | `51.12 ns` (✅ **1.75x faster**)   |
| **`16`**   | `987.67 ns` (✅ **1.00x**) | `978.57 ns` (✅ **1.01x faster**) | `1.02 us` (✅ **1.03x slower**)    |
| **`256`**  | `32.46 us` (✅ **1.00x**)  | `21.45 us` (✅ **1.51x faster**)  | `20.84 us` (✅ **1.56x faster**)   |
| **`1000`** | `230.54 us` (✅ **1.00x**) | `192.71 us` (✅ **1.20x faster**) | `192.36 us` (✅ **1.20x faster**)  |

---
Made with [criterion-table](https://github.com/nu11ptr/criterion-table)

