# Frozen Collections - Fast _Partially_ Immutable Collections

[![crate.io](https://img.shields.io/crates/v/frozen-collections.svg)](https://crates.io/crates/frozen-collections)
[![docs.rs](https://docs.rs/frozen-collections/badge.svg)](https://docs.rs/frozen-collections)
[![CI](https://github.com/geeknoid/frozen-collections/workflows/main/badge.svg)](https://github.com/geeknoid/frozen-collections/actions)
[![Coverage](https://codecov.io/gh/geeknoid/frozen-collections/graph/badge.svg?token=FCUG0EL5TI)](https://codecov.io/gh/geeknoid/frozen-collections)
[![Minimum Supported Rust Version 1.85](https://img.shields.io/badge/MSRV-1.85-blue.svg)]()
[![License](https://img.shields.io/badge/license-MIT-blue.svg)](./LICENSE)

* [Summary](#summary)
* [Handling Compile-Time Data](#handling-compile-time-data)
  * [Short Form](#short-form)
  * [Long Form](#long-form)
* [Using in a Build Script](#using-in-a-build-script)
* [Handling Runtime Data](#handling-runtime-data)
* [Traits](#traits)
* [Performance Considerations](#performance-considerations)
* [Analysis and Optimizations](#analysis-and-optimizations)
* [Cargo Features](#cargo-features)

## Summary

Frozen collections are designed to deliver improved
read performance relative to the standard [`HashMap`](https://doc.rust-lang.org/std/collections/hash/map/struct.HashMap.html) and
[`HashSet`](https://doc.rust-lang.org/std/collections/hash/set/struct.HashSet.html) types. They are ideal for use with long-lasting collections
which get initialized when an application starts and remain unchanged
permanently, or at least extended periods of time. This is a common
pattern in service applications.

As part of creating a frozen collection, analysis is performed over the data that the collection
will hold to determine the best layout and algorithm to use to deliver optimal performance.
Depending on the situation, sometimes the analysis is done at compile-time, whereas in
other cases it is done at runtime when the collection is initialized.
This analysis can take some time, but the value in spending this time up front
is that the collections provide faster read-time performance.

Frozen maps are only partially immutable. The keys associated with a frozen map are determined
at creation time and cannot change, but the values can be updated at will if you have a
mutable reference to the map. Frozen sets, however, are completely immutable and so never
change after creation.

See [BENCHMARKS.md](./BENCHMARKS.md) for current benchmark numbers.

## Handling Compile-Time Data

If you know the keys and values that will be in your collection at compile time, you can use
one of eight macros to create frozen collections:
[`fz_hash_map!`](https://docs.rs/frozen-collections/latest/frozen_collections/macro.fz_hash_map.html),
[`fz_ordered_map!`](https://docs.rs/frozen-collections/latest/frozen_collections/macro.fz_ordered_map.html),
[`fz_scalar_map!`](https://docs.rs/frozen-collections/latest/frozen_collections/macro.fz_scalar_map.html),
[`fz_string_map!`](https://docs.rs/frozen-collections/latest/frozen_collections/macro.fz_string_map.html),
[`fz_hash_set!`](https://docs.rs/frozen-collections/latest/frozen_collections/macro.fz_hash_set.html),
[`fz_ordered_set!`](https://docs.rs/frozen-collections/latest/frozen_collections/macro.fz_ordered_set.html),
[`fz_scalar_set!`](https://docs.rs/frozen-collections/latest/frozen_collections/macro.fz_scalar_set.html), or
[`fz_string_set!`](https://docs.rs/frozen-collections/latest/frozen_collections/macro.fz_string_set.html).
These macros analyze the data you provide
and return a custom implementation type optimized for the data. All the
possible types implement the
[`Map`](https://docs.rs/frozen-collections/latest/frozen_collections/trait.Map.html) or
[`Set`](https://docs.rs/frozen-collections/latest/frozen_collections/trait.Set.html) traits.

The macros exist in a short form and a long form, described below.

### Short Form

With the short form, you supply the data that
goes into the collection and get in return an initialized collection of an unnamed
type. For example:

```rust
use frozen_collections::*;

let m = fz_string_map!({
    "Alice": 1,
    "Bob": 2,
    "Sandy": 3,
    "Tom": 4,
});
```

At build time, the macro analyzes the data supplied and determines the best map
implementation type to use. As such, the type of `m` is not known to this code. `m` will
always implement the [`Map`](https://docs.rs/frozen-collections/latest/frozen_collections/trait.Map.html) trait however, so you can leverage type inference even though
you don't know the actual type of `m`:

```rust
use frozen_collections::*;

fn main() {
    let m = fz_string_map!({
        "Alice": 1,
        "Bob": 2,
        "Sandy": 3,
        "Tom": 4,
    });

    more(m);
}

fn more<M>(m: M)
where
    M: Map<&'static str, i32>
{
    assert!(m.contains_key(&"Alice"));
}
```

### Long Form

The long form lets you provide a type alias name which will be created to
correspond to the collection implementation type chosen by the macro invocation.
Note that you must use the long form if you want to declare a static frozen collection.

```rust
use frozen_collections::*;

fz_string_map!(static MAP: MyMapType<&'static str, i32>, {
    "Alice": 1,
    "Bob": 2,
    "Sandy": 3,
    "Tom": 4,
});
```

The above creates a static variable called `MAP` with keys that are strings and values which are
integers. As before, you don't know the specific implementation type selected by the macro, but
this time you have a type alias (i.e. `MyMapType`) representing that type. You can then use this alias
anywhere you'd like to in your code where you'd like to mention the type explicitly.

To use the long form for non-static uses, replace `static` with `let`:

```rust
use frozen_collections::*;

fz_string_map!(let m: MyMapType<&'static str, i32>, {
    "Alice": 1,
    "Bob": 2,
    "Sandy": 3,
    "Tom": 4,
});

more(m);

struct S {
    m: MyMapType,
}
 
fn more(m: MyMapType) {
    assert!(m.contains_key("Alice"));
}
```

## Using in a Build Script

You can use the
[`CollectionEmitter`](https://docs.rs/frozen-collections/latest/frozen_collections/emit/struct.CollectionEmitter.html),
struct to initialize a frozen collection from a build
script and output the results in a file that then gets compiled into your application. Due
to the fact build scripts run in a richer environment than procedural macros, the resulting
efficiency of collections generated from build scripts can be slightly faster than the ones
generated with the macros.

## Handling Runtime Data

If you don't know the exact keys and values that will be in your collection at compile time,
you use the dedicated map and collection types to hold your data:
[`FzHashMap`](https://docs.rs/frozen-collections/latest/frozen_collections/struct.FzHashMap.html),
[`FzOrderedMap`](https://docs.rs/frozen-collections/latest/frozen_collections/struct.FzOrderedMap.html),
[`FzScalarMap`](https://docs.rs/frozen-collections/latest/frozen_collections/struct.FzScalarMap.html),
[`FzStringMap`](https://docs.rs/frozen-collections/latest/frozen_collections/struct.FzStringMap.html),
[`FzHashSet`](https://docs.rs/frozen-collections/latest/frozen_collections/struct.FzHashSet.html),
[`FzOrderedSet`](https://docs.rs/frozen-collections/latest/frozen_collections/struct.FzOrderedSet.html),
[`FzScalarSet`](https://docs.rs/frozen-collections/latest/frozen_collections/struct.FzScalarSet.html), or
[`FzStringSet`](https://docs.rs/frozen-collections/latest/frozen_collections/struct.FzStringSet.html).
These
types analyze the data you provide at runtime and determine the best strategy to handle your
data dynamically.

```rust
use frozen_collections::*;

let v = vec![
    ("Alice", 1),
    ("Bob", 2),
    ("Sandy", 3),
    ("Tom", 4),
];

let m = FzStringMap::new(v);
```

Note that in general, if possible, it's more efficient to use the macros to create your frozen
collection instances.

## Traits

The maps produced by this crate implement the following traits:

- [`Map`](https://docs.rs/frozen-collections/latest/frozen_collections/trait.Map.html). The primary representation of a map. This trait has [`MapQuery`](https://docs.rs/frozen-collections/latest/frozen_collections/trait.MapQuery.html) and
  [`MapIteration`](https://docs.rs/frozen-collections/latest/frozen_collections/trait.MapIteration.html) as super-traits.
- [`MapQuery`](https://docs.rs/frozen-collections/latest/frozen_collections/trait.MapQuery.html). A trait for querying maps. This is an object-safe trait.
- [`MapIteration`](https://docs.rs/frozen-collections/latest/frozen_collections/trait.MapIteration.html). A trait for iterating over maps.

The sets produced by this crate implement the following traits:

- [`Set`](https://docs.rs/frozen-collections/latest/frozen_collections/trait.Set.html). The primary representation of a set. This trait has [`SetQuery`](https://docs.rs/frozen-collections/latest/frozen_collections/trait.Map.html),
  [`SetIteration`](https://docs.rs/frozen-collections/latest/frozen_collections/trait.Map.html) and [`SetOps`](https://docs.rs/frozen-collections/latest/frozen_collections/trait.Map.html) as super-traits.
- [`SetQuery`](https://docs.rs/frozen-collections/latest/frozen_collections/trait.Map.html). A trait for querying sets. This is an object-safe trait.
- [`SetIteration`](https://docs.rs/frozen-collections/latest/frozen_collections/trait.Map.html). A trait for iterating over sets.
- [`SetOps`](https://docs.rs/frozen-collections/latest/frozen_collections/trait.Map.html). A trait for set operations like union and intersections.

## Performance Considerations

The analysis performed when creating maps tries to find the best concrete implementation type
given the data at hand. The macros perform analysis at build time and generally produce slightly
faster results. The collection types meanwhile perform analysis at runtime, and the resulting
collections are slightly slower.

When creating static collections using the macros, the collections produced can often be embedded directly as constant data
into the binary of the application, thus requiring no initialization time and no heap space.
This also happens to be the fastest form for these collections. When possible, this happens
automatically. You don't need to do anything special to enable this behavior.

## Analysis and Optimizations

Unlike normal collections, the frozen collections require you to provide all the data for
the collection when you create the collection. The data you supply is analyzed which determines
which specific underlying implementation strategy to use and how to lay out data internally.

The available implementation strategies are:

- **Scalar as Hash**. When the keys are of an integer or enum type, this uses the keys themselves
  as hash codes, avoiding the overhead of hashing.

- **Length as Hash**. When the keys are of a string type, the lengths of the keys
  are used as hash code, avoiding the overhead of hashing.

- **Dense Scalar Lookup**. When the keys represent a contiguous range of integer or enum values,
  lookups use a simple array instead of hashing.

- **Sparse Scalar Lookup**. When the keys represent a sparse range of integer or enum values,
  lookups use a sparse array instead of hashing.

- **Left-Hand Substring Hashing**. When the keys are of a string type, this uses sub-slices of
  the keys for hashing, reducing the overhead of hashing.

- **Right-Hand Substring Hashing**. Similar to the Left-Hand Substring Hashing from above, but
  using right-aligned sub-slices instead.

- **Linear Scan**. For very small collections, this avoids hashing completely by scanning through
  the entries in linear order.

- **Ordered Scan**. For very small collections where the keys implement the [`Ord`](https://doc.rust-lang.org/std/cmp/trait.Ord.html) trait,
  this avoids hashing completely by scanning through the entries in linear order. Unlike the
  Linear Scan strategy, this one can early exit when keys are not found during the scan.

- **Classic Hashing**. This is the fallback when none of the previous strategies apply. The
  frozen implementations are generally faster than
  [`HashMap`](https://doc.rust-lang.org/std/collections/hash/map/struct.HashMap.html) and
  [`HashSet`](https://doc.rust-lang.org/std/collections/hash/set/struct.HashSet.html).

- **Binary Search**. For relatively small collections where the keys implement the [`Ord`](https://doc.rust-lang.org/std/cmp/trait.Ord.html) trait,
  classic binary searching is used.

- **Eytzinger Search**. For larger collections where the keys implement the [`Ord`](https://doc.rust-lang.org/std/cmp/trait.Ord.html) trait,
a cache-friendly Eytzinger search is used.

## Cargo Features

You can specify the following features when you include the `frozen_collections` crate in your
`Cargo.toml` file:

- **`macros`**. Enables the macros for creating frozen collections.
- **`emit`**. Enables the [`CollectionEmitter`](https://docs.rs/frozen-collections/latest/frozen_collections/emit/struct.CollectionEmitter.html) struct that lets you create frozen collections from a build script.
- **`serde`**. Enables serialization and deserialization support for the frozen collections.
- **`std`**. Enables small features only available when building with the standard library.

All features are enabled by default.
