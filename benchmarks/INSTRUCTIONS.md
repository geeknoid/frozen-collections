# Benchmarks (Instruction Counts)

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

These benchmarks compare the instruction counts of the frozen collections relative
to the classic Rust collections, measured using callgrind via gungraun.

Instruction counts are deterministic and independent of system load, making them
more reproducible than wall-clock time measurements.

The frozen collections have different optimizations depending on the type of data they
store and how it is declared. The benchmarks probe those different features to show
the effect of the different optimizations on instruction counts.

When you see `HashSet(SipHasher)` vs. `HashSet(foldhash)` this reflects the instruction count difference between the
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
| **`3`** | `432` (baseline) | `778` (❌ **80% more**) | `315` (✅ **27% fewer**) | `194` (🚀 **55% fewer**) |
| **`16`** | `1396` (baseline) | `3209` (❌ **130% more**) | `809` (✅ **42% fewer**) | `584` (🚀 **58% fewer**) |
| **`256`** | `19449` (baseline) | `48546` (❌ **150% more**) | `10039` (✅ **48% fewer**) | `7894` (🚀 **59% fewer**) |
| **`1000`** | `73915` (baseline) | `188564` (❌ **155% more**) | `38427` (✅ **48% fewer**) | `30247` (🚀 **59% fewer**) |

### sparse_scalar

Scalar sets where the values are in a non-contiguous range.

|            | `HashSet(foldhash)` | `HashSet(siphasher)` | `FzScalarSet` | `fz_scalar_set` |
|:-----------|:----------------------------------|:----------------------------------|:----------------------------------|:----------------------------------|
| **`3`** | `432` (baseline) | `778` (❌ **80% more**) | `426` (✅ **1% fewer**) | `222` (✅ **49% fewer**) |
| **`16`** | `1441` (baseline) | `3224` (❌ **124% more**) | `998` (✅ **31% fewer**) | `690` (🚀 **52% fewer**) |
| **`256`** | `19300` (baseline) | `48530` (❌ **151% more**) | `11747` (✅ **39% fewer**) | `9440` (🚀 **51% fewer**) |
| **`1000`** | `73884` (baseline) | `188461` (❌ **155% more**) | `44599` (✅ **40% fewer**) | `36257` (🚀 **51% fewer**) |

### random_scalar

Scalar sets where the values are randomly distributed.

|            | `HashSet(foldhash)` | `HashSet(siphasher)` | `FzScalarSet` | `fz_scalar_set` |
|:-----------|:----------------------------------|:----------------------------------|:----------------------------------|:----------------------------------|
| **`3`** | `427` (baseline) | `773` (❌ **81% more**) | `458` (❌ **7% more**) | `243` (✅ **43% fewer**) |
| **`16`** | `1391` (baseline) | `3219` (❌ **131% more**) | `1215` (✅ **13% fewer**) | `1141` (✅ **18% fewer**) |
| **`256`** | `19333` (baseline) | `48536` (❌ **151% more**) | `16305` (✅ **16% fewer**) | `17586` (✅ **9% fewer**) |
| **`1000`** | `74344` (baseline) | `188395` (❌ **153% more**) | `61865` (✅ **17% fewer**) | `67491` (✅ **9% fewer**) |

### random_string

String sets where the values are random.

|            | `HashSet(foldhash)` | `HashSet(siphasher)` | `FzStringSet(box)` | `FzStringSet(str)` | `FzStringSet(compact)` | `fz_string_set` |
|:-----------|:----------------------------------|:----------------------------------|:----------------------------------|:----------------------------------|:----------------------------------|:----------------------------------|
| **`3`** | `645` (baseline) | `1676` (❌ **160% more**) | `1076` (❌ **67% more**) | `793` (❌ **23% more**) | `885` (❌ **37% more**) | `396` (✅ **39% fewer**) |
| **`16`** | `2524` (baseline) | `7888` (❌ **213% more**) | `4546` (❌ **80% more**) | `2839` (❌ **12% more**) | `3325` (❌ **32% more**) | `1578` (✅ **37% fewer**) |
| **`256`** | `38387` (baseline) | `122812` (❌ **220% more**) | `73391` (❌ **91% more**) | `44037` (❌ **15% more**) | `55329` (❌ **44% more**) | `29332` (✅ **24% fewer**) |
| **`1000`** | `149107` (baseline) | `478185` (❌ **221% more**) | `285166` (❌ **91% more**) | `171132` (❌ **15% more**) | `212502` (❌ **43% more**) | `114086` (✅ **23% fewer**) |

### prefixed_string

String sets where the values are random but share a common prefix.

|            | `HashSet(foldhash)` | `HashSet(siphasher)` | `FzStringSet(box)` | `FzStringSet(str)` | `FzStringSet(compact)` | `fz_string_set` |
|:-----------|:----------------------------------|:----------------------------------|:----------------------------------|:----------------------------------|:----------------------------------|:----------------------------------|
| **`3`** | `746` (baseline) | `1724` (❌ **131% more**) | `1072` (❌ **44% more**) | `787` (❌ **5% more**) | `877` (❌ **18% more**) | `392` (✅ **47% fewer**) |
| **`16`** | `2983` (baseline) | `8296` (❌ **178% more**) | `4526` (❌ **52% more**) | `2851` (✅ **4% fewer**) | `3331` (❌ **12% more**) | `1590` (✅ **47% fewer**) |
| **`256`** | `43983` (baseline) | `131557` (❌ **199% more**) | `73404` (❌ **67% more**) | `44076` (❌ **0% more**) | `55317` (❌ **26% more**) | `29362` (✅ **33% fewer**) |
| **`1000`** | `169907` (baseline) | `511073` (❌ **201% more**) | `285052` (❌ **68% more**) | `171194` (❌ **1% more**) | `212445` (❌ **25% more**) | `114183` (✅ **33% fewer**) |

### hashed

Sets with a complex value type that is hashable.

|            | `HashSet(foldhash)` | `HashSet(siphasher)` | `FzHashSet` | `fz_hash_set` |
|:-----------|:----------------------------------|:----------------------------------|:----------------------------------|:----------------------------------|
| **`3`** | `1588` (baseline) | `3022` (❌ **90% more**) | `1632` (❌ **3% more**) | `970` (✅ **39% fewer**) |
| **`16`** | `8822` (baseline) | `16147` (❌ **83% more**) | `7702` (✅ **13% fewer**) | `5925` (✅ **33% fewer**) |
| **`256`** | `180770` (baseline) | `293867` (❌ **63% more**) | `121901` (✅ **33% fewer**) | `146114` (✅ **19% fewer**) |
| **`1000`** | `708779` (baseline) | `1150379` (❌ **62% more**) | `683632` (✅ **4% fewer**) | `574833` (✅ **19% fewer**) |

### ordered

Sets with a complex value type that is ordered.

|            | `BTreeSet` | `FzOrderedSet` | `fz_ordered_set` |
|:-----------|:----------------------------------|:----------------------------------|:----------------------------------|
| **`3`** | `1946` (baseline) | `1816` (✅ **7% fewer**) | `970` (🚀 **50% fewer**) |
| **`16`** | `15840` (baseline) | `12133` (✅ **23% fewer**) | `10352` (✅ **35% fewer**) |
| **`256`** | `443287` (baseline) | `338751` (✅ **24% fewer**) | `308151` (✅ **30% fewer**) |
| **`1000`** | `2191237` (baseline) | `1513651` (✅ **31% fewer**) | `1396833` (✅ **36% fewer**) |

---
Generated using gungraun (iai-callgrind) benchmarks

