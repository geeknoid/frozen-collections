# Frozen Collections

[![Crate](https://img.shields.io/crates/v/frozen-collections.svg)](https://crates.io/crates/frozen-collections)
[![Docs](https://docs.rs/frozen-collections/badge.svg)](https://docs.rs/frozen-collections)
[![Build](https://github.com/geeknoid/frozen-collections/workflows/main/badge.svg)](https://github.com/geeknoid/frozen-collections/actions)
[![Coverage](https://codecov.io/gh/geeknoid/frozen-collections/graph/badge.svg?token=FCUG0EL5TI)](https://codecov.io/gh/geeknoid/frozen-collections)
[![Minimum Supported Rust Version 1.82](https://img.shields.io/badge/MSRV-1.82-blue.svg)]()

Frozen collections: fast partially immutable collections

Frozen collections are designed to deliver improved
read performance relative to the standard [`HashMap`](https://doc.rust-lang.org/std/collections/struct.HashMap.html) and
[`HashSet`](https://doc.rust-lang.org/std/collections/struct.HashSet.html) types. They are ideal for use with
long-lasting collections
which get initialized when an application starts and remain unchanged
permanently, or at least extended periods of time. This is a common
pattern in service applications.

As part of initializing a frozen, an analysis is performed over the data that the collection
will hood, to determine the best layout and algorithm for the specific case.
This analysis can take some time. But the value in spending this time up front
is that the collections provide faster read-time performance.

# Macros and Structs

Frozen collections can be created in one of two ways:

- **via Macros**. When you know the data to load into the collection at build time, you can use the
  [`frozen_set!`](https://docs.rs/frozen-collections/macro.frozen_set.html),
  [`frozen_map!`](https://docs.rs/frozen-collections/macro.frozen_map.html), or
  [`static_frozen_map!`](https://docs.rs/frozen-collections/macro.static_frozen_map.html)
  macros. Analysis of the input
  data is done at build time, so there isn't any analysis cost spent when creating the
  collections. Additionally, using the macros results in faster runtime code.

* **via Facade Types**. When you don't know the data to load into the collection at
  build time, you use the
  [`FrozenSet`](https://docs.rs/frozen-collections/struct.FrozenSet.html),
  [`FrozenMap`](https://docs.rs/frozen-collections/struct.FrozenMap.html),
  [`FrozenIntSet`](https://docs.rs/frozen-collections/struct.FrozenIntSet.html),
  [`FrozenIntMap`](https://docs.rs/frozen-collections/struct.FrozenIntMap.html),
  [`FrozenStringSet`](https://docs.rs/frozen-collections/struct.FrozenStringSet.html) and
  [`FrozenStringMap`](https://docs.rs/frozen-collections/struct.FrozenStringMap.html)
  types. Analysis is performed at runtime when
  the collections are created, which can take a while if there are millions of
  entries.

Using the macros results in faster runtime performance, so they are the preferred choice if
possible. The facade type dispatch at runtime to
different specialized implementation types. When you use the macros instead, the selection
of implementation type is done at build time, and thus the cost of the runtime dispatch is
completely eliminated.

The [`static_frozen_map!`](https://docs.rs/frozen-collections/macro.static_frozen_map.html) macro lets you create static
maps that are stored directly in the
binary image, thus eliminating the need to replicate the collection in the heap on startup.
The lookup performance for these maps tends to be the fastest because all the data is inlined
and indirections have been eliminated.

# Analysis and Optimizations

Unlike normal collections, the frozen collections require you to provide all the data for
the collection when you create the collection. The data you supply is analyzed which determines
what specific underlying implementation strategy to use and how to lay out data in the hash tables
(assuming the implementation uses hash tables at all)

The available implementation strategies are:

- **Integer as Hash**. When the keys are of an integer type, this uses the keys themselves
  as hash codes, avoiding the overhead of hashing.

- **Dense Integer Lookup**. When the keys represent a contiguous range of integer values, this eliminates
  hashing completely and uses direct indexing instead.

- **Sparse Integer Lookup**. When the keys represent a sparse range of integer values, this eliminates
  hashing completely and uses a simple table lookup instead.

- **Length as Hash**. When the keys are of a slice type, this uses the length of the slices
  as hash codes, avoiding the overhead of hashing.

- **Left Hand Hashing**. When the keys are of a slice type, this uses sub-slices of the keys
  for hashing, reducing the overhead of hashing.

- **Right Hand Hashing**. Similar to the Right Hand Hashing from above, but using right-aligned
  sub-slices instead.

- **Linear Scan**. For very small maps, this avoids hashing completely by scanning through the
  keys in linear order.

- **Ordered Linear Scan**. For very small maps where the keys implement
  the [`Ord`](https://doc.rust-lang.org/std/cmp/trait.Ord.html) trait,
  this avoids hashing completely by scanning through the keys in linear order.

- **Classic Hashing**. This is the fallback when none of the previous strategies apply.

# Features

You can specify the following features when you include the `frozen-collections` crate in your
`Cargo.toml` file:

- **`std`**. Enables small features only available when building with the standard library.
- **`macros`**. This feature enables the use of the [`frozen_map!`] and [`frozen_set!`] macros.
- **`facades`**. This feature enables the facade types, such as [`FrozenMap`] and [`FrozenSet`].

All of the above features are enabled by default.

# Traits

The frozen collections define a few traits:

- [`Len`](https://docs.rs/frozen-collections/trait.Len.html). Used to represent keys that have lengths. This is used by
  the Length as Hash,
  Left Hand Hashing, and Right Hand Hashing strategies.

- [`RangeHash`](https://docs.rs/frozen-collections/trait.RangeHash.html). Used to enable hashing of a sub-slice of a
  value. This is used by the
  Left Hand Hashing and Right Hand Hashing strategies.

- [`Set`](https://docs.rs/frozen-collections/trait.Set.html). Used to represent common features of a set. This makes it
  possible for
  frozen collections to do logical operations, such as union or intersection, between various
  concrete set types.

- [`Map`](https://docs.rs/frozen-collections/trait.Map.html). Used to represent common features of a map.

- [`SelfHash`](https://docs.rs/frozen-collections/trait.SelfHash.html). Used to extract a hash code from a value.

- [`Int`](https://docs.rs/frozen-collections/trait.Int.html). Used to represent primitive integer types.

- [`CollectionScale`](https://docs.rs/frozen-collections/trait.CollectionScale.html). Used to represent primitive
  unsigned
  integer types.
