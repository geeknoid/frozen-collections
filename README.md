# Frozen Collections - Fast _Partially_ Immutable Collections

[![crate.io](https://img.shields.io/crates/v/frozen-collections.svg)](https://crates.io/crates/frozen-collections)
[![docs.rs](https://docs.rs/frozen-collections/badge.svg)](https://docs.rs/frozen-collections)
[![CI](https://github.com/geeknoid/frozen-collections/workflows/main/badge.svg)](https://github.com/geeknoid/frozen-collections/actions)
[![Coverage](https://codecov.io/gh/geeknoid/frozen-collections/graph/badge.svg?token=FCUG0EL5TI)](https://codecov.io/gh/geeknoid/frozen-collections)
[![Minimum Supported Rust Version 1.83](https://img.shields.io/badge/MSRV-1.83-blue.svg)]()
[![License](https://img.shields.io/badge/license-MIT-blue.svg)](./LICENSE)

* [Summary](#summary)
* [Creation](#creation)
  * [Short Form](#short-form)
  * [Long Form](#long-form)
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
Depending on the situation, sometimes the analysis is done at compile-time whereas in
other cases it is done at runtime when the collection is initialized.
This analysis can take some time, but the value in spending this time up front
is that the collections provide faster read-time performance.

Frozen maps are only partially immutable. The keys associated with a frozen map are determined
at creation time and cannot change, but the values can be updated at will if you have a
mutable reference to the map. Frozen sets however are completely immutable and so never
change after creation.

See [BENCHMARKS.md](./BENCHMARKS.md) for current benchmark numbers.

## Creation

Frozen collections are created with one of eight macros:
[`fz_hash_map!`](https://docs.rs/frozen-collections/macro.fz_hash_map.html),
[`fz_ordered_map!`](https://docs.rs/frozen-collections/macro.fz_ordered_map.html),
[`fz_scalar_map!`](https://docs.rs/frozen-collections/macro.fz_scalar_map.html),
[`fz_string_map!`](https://docs.rs/frozen-collections/macro.fz_string_map.html),
[`fz_hash_set!`](https://docs.rs/frozen-collections/macro.fz_hash_set.html),
[`fz_ordered_set!`](https://docs.rs/frozen-collections/macro.fz_ordered_set.html),
[`fz_scalar_set!`](https://docs.rs/frozen-collections/macro.fz_scalar_set.html), or
[`fz_string_set!`](https://docs.rs/frozen-collections/macro.fz_string_set.html).
These macros analyze the data you provide
and return a custom implementation type that's optimized for the data. All the
possible implementations types implement the
[`Map`](https://docs.rs/frozen-collections/trait.Map.html) or
[`Set`](https://docs.rs/frozen-collections/trait.Set.html) traits.

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

At build time, the  macro analyzes the data supplied and determines the best map
implementation type to use. As such, the type of `m` is not known to this code. `m` will
always implement the [`Map`] trait however, so you can leverage type inference even though
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

Rather than specifying all the data inline, you can also create a frozen collection by passing
a vector as input:

```rust
use frozen_collections::*;

let v = vec![
    ("Alice", 1),
    ("Bob", 2),
    ("Sandy", 3),
    ("Tom", 4),
];

let m = fz_string_map!(v);
```

The inline form is preferred however since it results in faster code. However, whereas the inline form
requires all the data to be provided at compile time, the vector form enables the content of the
frozen collection to be determined at runtime.

### Long Form

The long form lets you provide a type alias name which will be created to
correspond to the collection implementation type chosen by the macro invocation.
Note that you must use the long form if you want to declare a static frozen collection.

```rust
use frozen_collections::*;

fz_string_map!(static MAP: MyMapType<&str, i32>, {
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

fz_string_map!(let m: MyMapType<&str, i32>, {
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

And like in the short form, you can also supply the collection's data via a vector:

```rust
use frozen_collections::*;

let v = vec![
    ("Alice", 1),
    ("Bob", 2),
    ("Sandy", 3),
    ("Tom", 4),
];

fz_string_map!(let m: MyMapType<&str, i32>, v);
```

## Traits

The maps created by the frozen collections macros implement the following traits:

- [`Map`](https://docs.rs/frozen-collections/trait.Map.html). The primary representation of a map. This trait has [`MapQuery`](https://docs.rs/frozen-collections/trait.MapQuery.html) and
  [`MapIteration`](https://docs.rs/frozen-collections/trait.MapIteration.html) as super-traits.
- [`MapQuery`](https://docs.rs/frozen-collections/trait.MapQuery.html). A trait for querying maps. This is an object-safe trait.
- [`MapIteration`](https://docs.rs/frozen-collections/trait.MapIteration.html). A trait for iterating over maps.

The sets created by the frozen collection macros implement the following traits:

- [`Set`](https://docs.rs/frozen-collections/trait.Set.html). The primary representation of a set. This trait has [`SetQuery`](https://docs.rs/frozen-collections/trait.Map.html),
  [`SetIteration`](https://docs.rs/frozen-collections/trait.Map.html) and [`SetOps`](https://docs.rs/frozen-collections/trait.Map.html) as super-traits.
- [`SetQuery`](https://docs.rs/frozen-collections/trait.Map.html). A trait for querying sets. This is an object-safe trait.
- [`SetIteration`](https://docs.rs/frozen-collections/trait.Map.html). A trait for iterating over sets.
- [`SetOps`](https://docs.rs/frozen-collections/trait.Map.html). A trait for set operations like union and intersections.

## Performance Considerations

The analysis performed when creating maps tries to find the best concrete implementation type
given the data at hand. If all the data is visible to the macro at compile time, then you get
the best possible performance. If you supply a vector instead, then the analysis can only be
done at runtime and the resulting collection types are slightly slower.

When creating static collections, the collections produced can often be embedded directly as constant data
into the binary of the application, thus requiring no initialization time and no heap space. at
This also happens to be the fastest form for these collections. If possible, this happens
automatically, you don't need to do anything special to enable this behavior.

## Analysis and Optimizations

Unlike normal collections, the frozen collections require you to provide all the data for
the collection when you create the collection. The data you supply is analyzed which determines
which specific underlying implementation strategy to use and how to lay out data internally.

The available implementation strategies are:

- **Scalar as Hash**. When the keys are of an integer or enum type, this uses the keys themselves
  as hash codes, avoiding the overhead of hashing.

- **Length as Hash**. When the keys are of a string type, the length of the keys
  are used as hash code, avoiding the overhead of hashing.

- **Dense Scalar Lookup**. When the keys represent a contiguous range of integer or enum values,
  lookups use a simple array access instead of hashing.

- **Sparse Scalar Lookup**. When the keys represent a sparse range of integer or enum values,
  lookups use a sparse array access instead of hashing.

- **Left Hand Substring Hashing**. When the keys are of a string type, this uses sub-slices of
  the keys for hashing, reducing the overhead of hashing.

- **Right Hand Substring Hashing**. Similar to the Left Hand Substring Hashing from above, but
  using right-aligned sub-slices instead.

- **Linear Scan**. For very small collections, this avoids hashing completely by scanning through
  the entries in linear order.

- **Ordered Scan**. For very small collections where the keys implement the [`Ord`] trait,
  this avoids hashing completely by scanning through the entries in linear order. Unlike the
  Linear Scan strategy, this one can early exit when keys are not found during the scan.

- **Classic Hashing**. This is the fallback when none of the previous strategies apply. The
  frozen implementations are generally faster than
  [`HashMap`](https://doc.rust-lang.org/std/collections/hash/map/struct.HashMap.html) and
  [`HashSet`](https://doc.rust-lang.org/std/collections/hash/set/struct.HashSet.html).

- **Binary Search**. For relatively small collections where the keys implement the [`Ord`] trait,
  classic binary searching is used.

- **Eytzinger Search**. For larger collections where the keys implement the [`Ord`] trait,
a cache-friendly Eytzinger search is used.

## Cargo Features

You can specify the following features when you include the `frozen_collections` crate in your
`Cargo.toml` file:

- **`std`**. Enables small features only available when building with the standard library.

The `std` feature is enabled by default.
