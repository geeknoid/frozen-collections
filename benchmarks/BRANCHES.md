# Benchmarks (Branch Mispredictions)

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

These benchmarks compare the branch misprediction counts of the frozen collections relative
to the classic Rust collections, measured using callgrind's branch simulation via gungraun.

Branch mispredictions are deterministic under callgrind's simulation and independent of
system load, making them reproducible across runs.

The frozen collections have different optimizations depending on the type of data they
store and how it is declared. The benchmarks probe those different features to show
the effect of the different optimizations on branch prediction behavior.

When you see `HashSet(SipHasher)` vs. `HashSet(foldhash)` this reflects the difference between the
normal hasher used by the standard collections as opposed to the
`foldhash` hasher used with those same collections. 

The benchmarks assume a 50% hit rate when probing for lookup, meaning that
half the queries are for non-existing data. Some algorithms perform differently between
present vs. non-existing cases, so real world performance of these algorithms depends on the
real world hit rate you experience.

## Benchmark Results

### dense_scalar

Scalar sets where the values are in a contiguous range.

|            | `HashSet(foldhash)` | `HashSet(siphasher)` | `FzScalarSet` | `fz_scalar_set` |
|:-----------|:----------------------------------|:----------------------------------|:----------------------------------|:----------------------------------|
| **`3`** | `14` (baseline) | `15` (❌ **7% more**) | `7` (🚀 **50% fewer**) | `6` (🚀 **57% fewer**) |
| **`16`** | `22` (baseline) | `19` (✅ **14% fewer**) | `7` (🚀 **68% fewer**) | `11` (🚀 **50% fewer**) |
| **`256`** | `73` (baseline) | `69` (✅ **5% fewer**) | `16` (🚀 **78% fewer**) | `20` (🚀 **73% fewer**) |
| **`1000`** | `75` (baseline) | `146` (❌ **95% more**) | `26` (🚀 **65% fewer**) | `22` (🚀 **71% fewer**) |

### sparse_scalar

Scalar sets where the values are in a non-contiguous range.

|            | `HashSet(foldhash)` | `HashSet(siphasher)` | `FzScalarSet` | `fz_scalar_set` |
|:-----------|:----------------------------------|:----------------------------------|:----------------------------------|:----------------------------------|
| **`3`** | `14` (baseline) | `15` (❌ **7% more**) | `9` (✅ **36% fewer**) | `7` (🚀 **50% fewer**) |
| **`16`** | `29` (baseline) | `23` (✅ **21% fewer**) | `12` (🚀 **59% fewer**) | `8` (🚀 **72% fewer**) |
| **`256`** | `58` (baseline) | `73` (❌ **26% more**) | `33` (✅ **43% fewer**) | `16` (🚀 **72% fewer**) |
| **`1000`** | `79` (baseline) | `139` (❌ **76% more**) | `38` (🚀 **52% fewer**) | `22` (🚀 **72% fewer**) |

### random_scalar

Scalar sets where the values are randomly distributed.

|            | `HashSet(foldhash)` | `HashSet(siphasher)` | `FzScalarSet` | `fz_scalar_set` |
|:-----------|:----------------------------------|:----------------------------------|:----------------------------------|:----------------------------------|
| **`3`** | `16` (baseline) | `17` (❌ **6% more**) | `16` (baseline) | `7` (🚀 **56% fewer**) |
| **`16`** | `17` (baseline) | `21` (❌ **24% more**) | `20` (❌ **18% more**) | `15` (✅ **12% fewer**) |
| **`256`** | `54` (baseline) | `58` (❌ **7% more**) | `464` (❌ **759% more**) | `470` (❌ **770% more**) |
| **`1000`** | `116` (baseline) | `122` (❌ **5% more**) | `1548` (❌ **1234% more**) | `1562` (❌ **1247% more**) |

### random_string

String sets where the values are random.

|            | `HashSet(foldhash)` | `HashSet(siphasher)` | `FzStringSet(box)` | `FzStringSet(str)` | `FzStringSet(compact)` | `fz_string_set` |
|:-----------|:----------------------------------|:----------------------------------|:----------------------------------|:----------------------------------|:----------------------------------|:----------------------------------|
| **`3`** | `20` (baseline) | `58` (❌ **190% more**) | `25` (❌ **25% more**) | `20` (baseline) | `25` (❌ **25% more**) | `30` (❌ **50% more**) |
| **`16`** | `38` (baseline) | `130` (❌ **242% more**) | `58` (❌ **53% more**) | `32` (✅ **16% fewer**) | `45` (❌ **18% more**) | `9` (🚀 **76% fewer**) |
| **`256`** | `309` (baseline) | `1196` (❌ **287% more**) | `693` (❌ **124% more**) | `641` (❌ **107% more**) | `673` (❌ **118% more**) | `499` (❌ **61% more**) |
| **`1000`** | `1095` (baseline) | `4740` (❌ **333% more**) | `2510` (❌ **129% more**) | `2490` (❌ **127% more**) | `2445` (❌ **123% more**) | `1685` (❌ **54% more**) |

### prefixed_string

String sets where the values are random but share a common prefix.

|            | `HashSet(foldhash)` | `HashSet(siphasher)` | `FzStringSet(box)` | `FzStringSet(str)` | `FzStringSet(compact)` | `fz_string_set` |
|:-----------|:----------------------------------|:----------------------------------|:----------------------------------|:----------------------------------|:----------------------------------|:----------------------------------|
| **`3`** | `26` (baseline) | `46` (❌ **77% more**) | `23` (✅ **12% fewer**) | `19` (✅ **27% fewer**) | `23` (✅ **12% fewer**) | `31` (❌ **19% more**) |
| **`16`** | `58` (baseline) | `144` (❌ **148% more**) | `49` (✅ **16% fewer**) | `38` (✅ **34% fewer**) | `46` (✅ **21% fewer**) | `13` (🚀 **78% fewer**) |
| **`256`** | `321` (baseline) | `1246` (❌ **288% more**) | `712` (❌ **122% more**) | `663` (❌ **107% more**) | `672` (❌ **109% more**) | `509` (❌ **59% more**) |
| **`1000`** | `953` (baseline) | `4611` (❌ **384% more**) | `2487` (❌ **161% more**) | `2533` (❌ **166% more**) | `2448` (❌ **157% more**) | `1744` (❌ **83% more**) |

### hashed

Sets with a complex value type that is hashable.

|            | `HashSet(foldhash)` | `HashSet(siphasher)` | `FzHashSet` | `fz_hash_set` |
|:-----------|:----------------------------------|:----------------------------------|:----------------------------------|:----------------------------------|
| **`3`** | `37` (baseline) | `67` (❌ **81% more**) | `23` (✅ **38% fewer**) | `31` (✅ **16% fewer**) |
| **`16`** | `119` (baseline) | `260` (❌ **118% more**) | `66` (✅ **45% fewer**) | `34` (🚀 **71% fewer**) |
| **`256`** | `612` (baseline) | `2055` (❌ **236% more**) | `662` (❌ **8% more**) | `813` (❌ **33% more**) |
| **`1000`** | `1965` (baseline) | `7314` (❌ **272% more**) | `3161` (❌ **61% more**) | `2788` (❌ **42% more**) |

### ordered

Sets with a complex value type that is ordered.

|            | `BTreeSet` | `FzOrderedSet` | `fz_ordered_set` |
|:-----------|:----------------------------------|:----------------------------------|:----------------------------------|
| **`3`** | `48` (baseline) | `48` (baseline) | `31` (✅ **35% fewer**) |
| **`16`** | `218` (baseline) | `204` (✅ **6% fewer**) | `165` (✅ **24% fewer**) |
| **`256`** | `3475` (baseline) | `3369` (✅ **3% fewer**) | `2635` (✅ **24% fewer**) |
| **`1000`** | `14489` (baseline) | `14958` (❌ **3% more**) | `12991` (✅ **10% fewer**) |

---
Generated using gungraun (iai-callgrind) benchmarks with branch simulation

