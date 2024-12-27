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
| **`3`**    | `45.65 ns` (✅ **1.00x**)    | `12.94 ns` (🚀 **3.53x faster**) | `4.31 ns` (🚀 **10.60x faster**)   | `4.31 ns` (🚀 **10.60x faster**)   |
| **`16`**   | `239.86 ns` (✅ **1.00x**)   | `72.37 ns` (🚀 **3.31x faster**) | `23.58 ns` (🚀 **10.17x faster**)  | `24.37 ns` (🚀 **9.84x faster**)   |
| **`256`**  | `4.02 us` (✅ **1.00x**)     | `1.14 us` (🚀 **3.54x faster**)  | `376.78 ns` (🚀 **10.68x faster**) | `410.56 ns` (🚀 **9.80x faster**)  |
| **`1000`** | `15.84 us` (✅ **1.00x**)    | `4.59 us` (🚀 **3.45x faster**)  | `1.45 us` (🚀 **10.94x faster**)   | `1.60 us` (🚀 **9.93x faster**)    |

### sparse_scalar

Scalar sets where the values are in a non-contiguous range.

|            | `HashSet(classic)`          | `HashSet(foldhash)`             | `FzScalarSet`                     | `fz_scalar_set`                    |
|:-----------|:----------------------------|:--------------------------------|:----------------------------------|:---------------------------------- |
| **`3`**    | `45.63 ns` (✅ **1.00x**)    | `12.94 ns` (🚀 **3.53x faster**) | `4.46 ns` (🚀 **10.22x faster**)   | `6.06 ns` (🚀 **7.53x faster**)     |
| **`16`**   | `239.98 ns` (✅ **1.00x**)   | `68.95 ns` (🚀 **3.48x faster**) | `20.67 ns` (🚀 **11.61x faster**)  | `24.17 ns` (🚀 **9.93x faster**)    |
| **`256`**  | `4.08 us` (✅ **1.00x**)     | `1.13 us` (🚀 **3.61x faster**)  | `330.15 ns` (🚀 **12.36x faster**) | `379.51 ns` (🚀 **10.75x faster**)  |
| **`1000`** | `15.76 us` (✅ **1.00x**)    | `4.61 us` (🚀 **3.42x faster**)  | `1.27 us` (🚀 **12.43x faster**)   | `1.46 us` (🚀 **10.76x faster**)    |

### random_scalar

Scalar sets where the values are randomly distributed.

|            | `HashSet(classic)`          | `HashSet(foldhash)`             | `FzScalarSet`                    | `fz_scalar_set`                   |
|:-----------|:----------------------------|:--------------------------------|:---------------------------------|:--------------------------------- |
| **`3`**    | `49.38 ns` (✅ **1.00x**)    | `12.52 ns` (🚀 **3.94x faster**) | `9.29 ns` (🚀 **5.32x faster**)   | `11.27 ns` (🚀 **4.38x faster**)   |
| **`16`**   | `240.76 ns` (✅ **1.00x**)   | `67.29 ns` (🚀 **3.58x faster**) | `42.18 ns` (🚀 **5.71x faster**)  | `43.37 ns` (🚀 **5.55x faster**)   |
| **`256`**  | `4.02 us` (✅ **1.00x**)     | `1.11 us` (🚀 **3.61x faster**)  | `818.15 ns` (🚀 **4.92x faster**) | `838.08 ns` (🚀 **4.80x faster**)  |
| **`1000`** | `15.82 us` (✅ **1.00x**)    | `4.65 us` (🚀 **3.41x faster**)  | `3.25 us` (🚀 **4.87x faster**)   | `3.24 us` (🚀 **4.89x faster**)    |

### random_string

String sets where the values are random.

|            | `HashSet(classic)`          | `HashSet(foldhash)`              | `FzStringSet`                    | `fz_string_set`                   |
|:-----------|:----------------------------|:---------------------------------|:---------------------------------|:--------------------------------- |
| **`3`**    | `78.36 ns` (✅ **1.00x**)    | `29.21 ns` (🚀 **2.68x faster**)  | `30.31 ns` (🚀 **2.58x faster**)  | `26.16 ns` (🚀 **3.00x faster**)   |
| **`16`**   | `429.10 ns` (✅ **1.00x**)   | `164.50 ns` (🚀 **2.61x faster**) | `204.49 ns` (🚀 **2.10x faster**) | `145.09 ns` (🚀 **2.96x faster**)  |
| **`256`**  | `6.81 us` (✅ **1.00x**)     | `2.76 us` (🚀 **2.46x faster**)   | `3.57 us` (🚀 **1.91x faster**)   | `2.72 us` (🚀 **2.50x faster**)    |
| **`1000`** | `27.79 us` (✅ **1.00x**)    | `11.55 us` (🚀 **2.41x faster**)  | `16.61 us` (✅ **1.67x faster**)  | `11.14 us` (🚀 **2.49x faster**)   |

### prefixed_string

String sets where the values are random, but share a common prefix.

|            | `HashSet(classic)`          | `HashSet(foldhash)`              | `FzStringSet`                    | `fz_string_set`                   |
|:-----------|:----------------------------|:---------------------------------|:---------------------------------|:--------------------------------- |
| **`3`**    | `84.32 ns` (✅ **1.00x**)    | `36.40 ns` (🚀 **2.32x faster**)  | `36.94 ns` (🚀 **2.28x faster**)  | `26.60 ns` (🚀 **3.17x faster**)   |
| **`16`**   | `449.26 ns` (✅ **1.00x**)   | `193.76 ns` (🚀 **2.32x faster**) | `222.28 ns` (🚀 **2.02x faster**) | `168.35 ns` (🚀 **2.67x faster**)  |
| **`256`**  | `7.49 us` (✅ **1.00x**)     | `3.19 us` (🚀 **2.35x faster**)   | `3.73 us` (🚀 **2.01x faster**)   | `2.95 us` (🚀 **2.54x faster**)    |
| **`1000`** | `30.50 us` (✅ **1.00x**)    | `13.17 us` (🚀 **2.32x faster**)  | `26.53 us` (✅ **1.15x faster**)  | `11.83 us` (🚀 **2.58x faster**)   |

### hashed

Sets with a complex key type that is hashable.

|            | `HashSet(classic)`          | `HashSet(foldhash)`              | `FzHashSet`                      | `fz_hash_set`                     |
|:-----------|:----------------------------|:---------------------------------|:---------------------------------|:--------------------------------- |
| **`3`**    | `103.58 ns` (✅ **1.00x**)   | `33.09 ns` (🚀 **3.13x faster**)  | `30.29 ns` (🚀 **3.42x faster**)  | `52.54 ns` (🚀 **1.97x faster**)   |
| **`16`**   | `480.71 ns` (✅ **1.00x**)   | `170.43 ns` (🚀 **2.82x faster**) | `145.71 ns` (🚀 **3.30x faster**) | `183.41 ns` (🚀 **2.62x faster**)  |
| **`256`**  | `8.52 us` (✅ **1.00x**)     | `2.96 us` (🚀 **2.87x faster**)   | `2.95 us` (🚀 **2.89x faster**)   | `2.94 us` (🚀 **2.90x faster**)    |
| **`1000`** | `34.12 us` (✅ **1.00x**)    | `12.26 us` (🚀 **2.78x faster**)  | `12.24 us` (🚀 **2.79x faster**)  | `11.71 us` (🚀 **2.91x faster**)   |

### ordered

Sets with a complex key type that is ordered.

|            | `BTreeSet`                | `FzOrderedSet`                   | `fz_ordered_set`                  |
|:-----------|:--------------------------|:---------------------------------|:--------------------------------- |
| **`3`**    | `73.06 ns` (✅ **1.00x**)  | `70.27 ns` (✅ **1.04x faster**)  | `54.21 ns` (✅ **1.35x faster**)   |
| **`16`**   | `941.42 ns` (✅ **1.00x**) | `962.02 ns` (✅ **1.02x slower**) | `1.02 us` (✅ **1.08x slower**)    |
| **`256`**  | `32.30 us` (✅ **1.00x**)  | `20.75 us` (✅ **1.56x faster**)  | `20.87 us` (✅ **1.55x faster**)   |
| **`1000`** | `226.00 us` (✅ **1.00x**) | `188.74 us` (✅ **1.20x faster**) | `190.02 us` (✅ **1.19x faster**)  |

---
Made with [criterion-table](https://github.com/nu11ptr/criterion-table)

