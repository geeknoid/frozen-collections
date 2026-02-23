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

|            | `HashSet(classic)`          | `HashSet(foldhash)`             | `FzScalarSet`                     | `fz_scalar_set`                   |
|:-----------|:----------------------------|:--------------------------------|:----------------------------------|:--------------------------------- |
| **`3`**    | `45.58 ns` (✅ **1.00x**)    | `12.09 ns` (🚀 **3.77x faster**) | `4.33 ns` (🚀 **10.54x faster**)   | `4.31 ns` (🚀 **10.58x faster**)   |
| **`16`**   | `232.33 ns` (✅ **1.00x**)   | `62.04 ns` (🚀 **3.75x faster**) | `25.20 ns` (🚀 **9.22x faster**)   | `26.23 ns` (🚀 **8.86x faster**)   |
| **`256`**  | `3.80 us` (✅ **1.00x**)     | `1.05 us` (🚀 **3.61x faster**)  | `379.34 ns` (🚀 **10.02x faster**) | `410.70 ns` (🚀 **9.26x faster**)  |
| **`1000`** | `15.02 us` (✅ **1.00x**)    | `4.26 us` (🚀 **3.52x faster**)  | `1.46 us` (🚀 **10.32x faster**)   | `1.60 us` (🚀 **9.41x faster**)    |

### sparse_scalar

Scalar sets where the values are in a non-contiguous range.

|            | `HashSet(classic)`          | `HashSet(foldhash)`             | `FzScalarSet`                    | `fz_scalar_set`                  |
|:-----------|:----------------------------|:--------------------------------|:---------------------------------|:-------------------------------- |
| **`3`**    | `43.61 ns` (✅ **1.00x**)    | `12.14 ns` (🚀 **3.59x faster**) | `6.22 ns` (🚀 **7.01x faster**)   | `5.52 ns` (🚀 **7.89x faster**)   |
| **`16`**   | `229.09 ns` (✅ **1.00x**)   | `60.80 ns` (🚀 **3.77x faster**) | `36.53 ns` (🚀 **6.27x faster**)  | `36.65 ns` (🚀 **6.25x faster**)  |
| **`256`**  | `3.82 us` (✅ **1.00x**)     | `1.02 us` (🚀 **3.76x faster**)  | `596.40 ns` (🚀 **6.40x faster**) | `1.97 us` (🚀 **1.94x faster**)   |
| **`1000`** | `14.88 us` (✅ **1.00x**)    | `4.13 us` (🚀 **3.60x faster**)  | `2.32 us` (🚀 **6.42x faster**)   | `8.34 us` (✅ **1.78x faster**)   |

### random_scalar

Scalar sets where the values are randomly distributed.

|            | `HashSet(classic)`          | `HashSet(foldhash)`             | `FzScalarSet`                    | `fz_scalar_set`                   |
|:-----------|:----------------------------|:--------------------------------|:---------------------------------|:--------------------------------- |
| **`3`**    | `44.12 ns` (✅ **1.00x**)    | `12.34 ns` (🚀 **3.57x faster**) | `6.51 ns` (🚀 **6.78x faster**)   | `5.39 ns` (🚀 **8.19x faster**)    |
| **`16`**   | `228.76 ns` (✅ **1.00x**)   | `61.55 ns` (🚀 **3.72x faster**) | `31.78 ns` (🚀 **7.20x faster**)  | `33.11 ns` (🚀 **6.91x faster**)   |
| **`256`**  | `3.79 us` (✅ **1.00x**)     | `1.04 us` (🚀 **3.64x faster**)  | `742.78 ns` (🚀 **5.11x faster**) | `737.87 ns` (🚀 **5.14x faster**)  |
| **`1000`** | `14.77 us` (✅ **1.00x**)    | `4.26 us` (🚀 **3.46x faster**)  | `2.89 us` (🚀 **5.12x faster**)   | `2.77 us` (🚀 **5.33x faster**)    |

### random_string

String sets where the values are random.

|            | `HashSet(classic)`          | `HashSet(foldhash)`              | `FzStringSet`                    | `fz_string_set`                   |
|:-----------|:----------------------------|:---------------------------------|:---------------------------------|:--------------------------------- |
| **`3`**    | `79.51 ns` (✅ **1.00x**)    | `30.21 ns` (🚀 **2.63x faster**)  | `37.84 ns` (🚀 **2.10x faster**)  | `35.75 ns` (🚀 **2.22x faster**)   |
| **`16`**   | `423.75 ns` (✅ **1.00x**)   | `159.12 ns` (🚀 **2.66x faster**) | `193.69 ns` (🚀 **2.19x faster**) | `175.28 ns` (🚀 **2.42x faster**)  |
| **`256`**  | `6.78 us` (✅ **1.00x**)     | `2.64 us` (🚀 **2.57x faster**)   | `3.53 us` (🚀 **1.92x faster**)   | `2.68 us` (🚀 **2.53x faster**)    |
| **`1000`** | `27.36 us` (✅ **1.00x**)    | `10.81 us` (🚀 **2.53x faster**)  | `14.03 us` (🚀 **1.95x faster**)  | `10.66 us` (🚀 **2.57x faster**)   |

### prefixed_string

String sets where the values are random but share a common prefix.

|            | `HashSet(classic)`          | `HashSet(foldhash)`              | `FzStringSet`                    | `fz_string_set`                   |
|:-----------|:----------------------------|:---------------------------------|:---------------------------------|:--------------------------------- |
| **`3`**    | `83.80 ns` (✅ **1.00x**)    | `39.06 ns` (🚀 **2.15x faster**)  | `37.79 ns` (🚀 **2.22x faster**)  | `35.34 ns` (🚀 **2.37x faster**)   |
| **`16`**   | `454.14 ns` (✅ **1.00x**)   | `202.89 ns` (🚀 **2.24x faster**) | `198.44 ns` (🚀 **2.29x faster**) | `166.67 ns` (🚀 **2.72x faster**)  |
| **`256`**  | `7.52 us` (✅ **1.00x**)     | `3.22 us` (🚀 **2.34x faster**)   | `3.63 us` (🚀 **2.07x faster**)   | `2.95 us` (🚀 **2.55x faster**)    |
| **`1000`** | `30.58 us` (✅ **1.00x**)    | `13.10 us` (🚀 **2.33x faster**)  | `14.63 us` (🚀 **2.09x faster**)  | `12.52 us` (🚀 **2.44x faster**)   |

### hashed

Sets with a complex value type that is hashable.

|            | `HashSet(classic)`          | `HashSet(foldhash)`              | `FzHashSet`                      | `fz_hash_set`                     |
|:-----------|:----------------------------|:---------------------------------|:---------------------------------|:--------------------------------- |
| **`3`**    | `101.27 ns` (✅ **1.00x**)   | `31.93 ns` (🚀 **3.17x faster**)  | `32.94 ns` (🚀 **3.07x faster**)  | `31.57 ns` (🚀 **3.21x faster**)   |
| **`16`**   | `515.23 ns` (✅ **1.00x**)   | `165.96 ns` (🚀 **3.10x faster**) | `154.86 ns` (🚀 **3.33x faster**) | `170.49 ns` (🚀 **3.02x faster**)  |
| **`256`**  | `8.11 us` (✅ **1.00x**)     | `2.63 us` (🚀 **3.08x faster**)   | `2.69 us` (🚀 **3.01x faster**)   | `2.71 us` (🚀 **2.99x faster**)    |
| **`1000`** | `33.23 us` (✅ **1.00x**)    | `10.79 us` (🚀 **3.08x faster**)  | `10.66 us` (🚀 **3.12x faster**)  | `10.49 us` (🚀 **3.17x faster**)   |

### ordered

Sets with a complex value type that is ordered.

|            | `BTreeSet`                | `FzOrderedSet`                   | `fz_ordered_set`                  |
|:-----------|:--------------------------|:---------------------------------|:--------------------------------- |
| **`3`**    | `79.53 ns` (✅ **1.00x**)  | `67.84 ns` (✅ **1.17x faster**)  | `32.22 ns` (🚀 **2.47x faster**)   |
| **`16`**   | `910.40 ns` (✅ **1.00x**) | `652.11 ns` (✅ **1.40x faster**) | `643.08 ns` (✅ **1.42x faster**)  |
| **`256`**  | `31.17 us` (✅ **1.00x**)  | `20.96 us` (✅ **1.49x faster**)  | `18.85 us` (✅ **1.65x faster**)   |
| **`1000`** | `213.19 us` (✅ **1.00x**) | `181.35 us` (✅ **1.18x faster**) | `178.67 us` (✅ **1.19x faster**)  |

---
Made with [criterion-table](https://github.com/nu11ptr/criterion-table)

