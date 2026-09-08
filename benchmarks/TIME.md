# Benchmarks (Wall-Clock Time)

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

These benchmarks compare the wall-clock performance of the frozen collections relative
to the classic Rust collections, measured using criterion.

The frozen collections have different optimizations depending on the type of data they
store and how it is declared. The benchmarks probe those different features to show
the effect of the different optimizations on effective performance.

When you see `HashSet(SipHasher)` vs. `HashSet(foldhash)` this reflects the performance difference between the
normal hasher used by the standard collections as opposed to the performance that the
`foldhash` hasher provides using those same collections.

The benchmarks assume a 50% hit rate when probing for lookup, meaning that
half the queries are for non-existing data. Some algorithms perform differently between
present vs. non-existing cases, so real world performance of these algorithms depends on the
real world hit rate you experience.

## Benchmark Results

### dense_scalar

Scalar sets where the values are in a contiguous range.

|            | `HashSet(foldhash)` | `HashSet(SipHasher)` | `FzScalarSet` | `fz_scalar_set` |
|:-----------|:----------------------------------|:----------------------------------|:----------------------------------|:----------------------------------|
| **`3`** | `18.28 ns` (baseline) | `53.58 ns` (❌ **193% slower**) | `8.34 ns` (🚀 **54% faster**) | `6.22 ns` (🚀 **66% faster**) |
| **`16`** | `88.16 ns` (baseline) | `286.75 ns` (❌ **225% slower**) | `35.32 ns` (🚀 **60% faster**) | `30.47 ns` (🚀 **65% faster**) |
| **`256`** | `1.52 us` (baseline) | `4.69 us` (❌ **208% slower**) | `566.13 ns` (🚀 **63% faster**) | `495.56 ns` (🚀 **67% faster**) |
| **`1000`** | `5.71 us` (baseline) | `18.07 us` (❌ **216% slower**) | `2.15 us` (🚀 **62% faster**) | `1.95 us` (🚀 **66% faster**) |

### sparse_scalar

Scalar sets where the values are in a non-contiguous range.

|            | `HashSet(foldhash)` | `HashSet(SipHasher)` | `FzScalarSet` | `fz_scalar_set` |
|:-----------|:----------------------------------|:----------------------------------|:----------------------------------|:----------------------------------|
| **`3`** | `16.31 ns` (baseline) | `52.26 ns` (❌ **220% slower**) | `7.74 ns` (🚀 **53% faster**) | `6.37 ns` (🚀 **61% faster**) |
| **`16`** | `108.32 ns` (baseline) | `278.44 ns` (❌ **157% slower**) | `37.10 ns` (🚀 **66% faster**) | `30.36 ns` (🚀 **72% faster**) |
| **`256`** | `1.57 us` (baseline) | `4.73 us` (❌ **202% slower**) | `587.53 ns` (🚀 **62% faster**) | `493.47 ns` (🚀 **68% faster**) |
| **`1000`** | `5.69 us` (baseline) | `17.99 us` (❌ **216% slower**) | `2.25 us` (🚀 **60% faster**) | `2.11 us` (🚀 **63% faster**) |

### random_scalar

Scalar sets where the values are randomly distributed.

|            | `HashSet(foldhash)` | `HashSet(SipHasher)` | `FzScalarSet` | `fz_scalar_set` |
|:-----------|:----------------------------------|:----------------------------------|:----------------------------------|:----------------------------------|
| **`3`** | `15.45 ns` (baseline) | `51.96 ns` (❌ **236% slower**) | `10.22 ns` (✅ **34% faster**) | `6.51 ns` (🚀 **58% faster**) |
| **`16`** | `82.67 ns` (baseline) | `275.57 ns` (❌ **233% slower**) | `57.96 ns` (✅ **30% faster**) | `57.09 ns` (✅ **31% faster**) |
| **`256`** | `1.53 us` (baseline) | `4.68 us` (❌ **206% slower**) | `998.48 ns` (✅ **35% faster**) | `990.24 ns` (✅ **35% faster**) |
| **`1000`** | `5.91 us` (baseline) | `18.08 us` (❌ **206% slower**) | `3.86 us` (✅ **35% faster**) | `3.87 us` (✅ **35% faster**) |

### random_string

String sets where the values are random.

|            | `HashSet(foldhash)` | `HashSet(SipHasher)` | `FzStringSet(box)` | `FzStringSet(str)` | `FzStringSet(compact)` | `fz_string_set` |
|:-----------|:----------------------------------|:----------------------------------|:----------------------------------|:----------------------------------|:----------------------------------|:----------------------------------|
| **`3`** | `31.96 ns` (baseline) | `81.36 ns` (❌ **155% slower**) | `32.46 ns` (❌ **2% slower**) | `33.84 ns` (❌ **6% slower**) | `43.74 ns` (❌ **37% slower**) | `20.96 ns` (✅ **34% faster**) |
| **`16`** | `182.17 ns` (baseline) | `447.92 ns` (❌ **146% slower**) | `187.40 ns` (❌ **3% slower**) | `210.62 ns` (❌ **16% slower**) | `235.08 ns` (❌ **29% slower**) | `113.00 ns` (✅ **38% faster**) |
| **`256`** | `2.96 us` (baseline) | `7.01 us` (❌ **137% slower**) | `3.08 us` (❌ **4% slower**) | `2.99 us` (❌ **1% slower**) | `4.26 us` (❌ **44% slower**) | `2.12 us` (✅ **28% faster**) |
| **`1000`** | `11.93 us` (baseline) | `27.83 us` (❌ **133% slower**) | `12.69 us` (❌ **6% slower**) | `12.69 us` (❌ **6% slower**) | `18.04 us` (❌ **51% slower**) | `8.61 us` (✅ **28% faster**) |

### prefixed_string

String sets where the values are random but share a common prefix.

|            | `HashSet(foldhash)` | `HashSet(SipHasher)` | `FzStringSet(box)` | `FzStringSet(str)` | `FzStringSet(compact)` | `fz_string_set` |
|:-----------|:----------------------------------|:----------------------------------|:----------------------------------|:----------------------------------|:----------------------------------|:----------------------------------|
| **`3`** | `37.14 ns` (baseline) | `85.79 ns` (❌ **131% slower**) | `32.17 ns` (✅ **13% faster**) | `33.95 ns` (✅ **9% faster**) | `43.17 ns` (❌ **16% slower**) | `20.47 ns` (✅ **45% faster**) |
| **`16`** | `199.67 ns` (baseline) | `480.16 ns` (❌ **140% slower**) | `175.44 ns` (✅ **12% faster**) | `179.80 ns` (✅ **10% faster**) | `235.17 ns` (❌ **18% slower**) | `112.14 ns` (✅ **44% faster**) |
| **`256`** | `3.25 us` (baseline) | `7.69 us` (❌ **137% slower**) | `2.95 us` (✅ **9% faster**) | `3.06 us` (✅ **6% faster**) | `4.63 us` (❌ **43% slower**) | `2.16 us` (✅ **33% faster**) |
| **`1000`** | `13.82 us` (baseline) | `31.42 us` (❌ **127% slower**) | `12.30 us` (✅ **11% faster**) | `12.82 us` (✅ **7% faster**) | `18.44 us` (❌ **33% slower**) | `9.02 us` (✅ **35% faster**) |

### hashed

Sets with a complex value type that is hashable.

|            | `HashSet(foldhash)` | `HashSet(SipHasher)` | `FzHashSet` | `fz_hash_set` |
|:-----------|:----------------------------------|:----------------------------------|:----------------------------------|:----------------------------------|
| **`3`** | `35.02 ns` (baseline) | `102.65 ns` (❌ **193% slower**) | `27.64 ns` (✅ **21% faster**) | `19.56 ns` (✅ **44% faster**) |
| **`16`** | `183.15 ns` (baseline) | `539.41 ns` (❌ **195% slower**) | `140.93 ns` (✅ **23% faster**) | `134.65 ns` (✅ **26% faster**) |
| **`256`** | `3.06 us` (baseline) | `8.43 us` (❌ **176% slower**) | `2.49 us` (✅ **19% faster**) | `2.47 us` (✅ **19% faster**) |
| **`1000`** | `12.10 us` (baseline) | `33.45 us` (❌ **176% slower**) | `10.61 us` (✅ **12% faster**) | `10.66 us` (✅ **12% faster**) |

### ordered

Sets with a complex value type that is ordered.

|            | `BTreeSet` | `FzOrderedSet` | `fz_ordered_set` |
|:-----------|:----------------------------------|:----------------------------------|:----------------------------------|
| **`3`** | `48.90 ns` (baseline) | `46.37 ns` (✅ **5% faster**) | `17.51 ns` (🚀 **64% faster**) |
| **`16`** | `627.12 ns` (baseline) | `431.71 ns` (✅ **31% faster**) | `449.30 ns` (✅ **28% faster**) |
| **`256`** | `22.49 us` (baseline) | `15.20 us` (✅ **32% faster**) | `13.57 us` (✅ **40% faster**) |
| **`1000`** | `166.04 us` (baseline) | `182.12 us` (❌ **10% slower**) | `176.12 us` (❌ **6% slower**) |

---
Generated using criterion benchmarks

