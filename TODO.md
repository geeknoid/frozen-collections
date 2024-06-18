# TODO

## Engineering Excellence

- Generate BENCHMARK.md

## Performance

- Look at https://lib.rs/crates/small-map for SIMD accelerated small maps.

- with strings, use unique substrings as the actual hash code when possible.

- Expand benchmarks to test the performance of the library with larger data sets.

- Introduce a partially inlined hashmap for use by the macros, when we know the number
  of entries but can't hash them at build time.

- Fine-tune the thresholds when we switch between different implementation types in
  the facades and in the generator.

## Features

- Consider adding support for case-insensitive strings.

- Extend the Scalar derive macro to support more varieties of enum types.

- Reintroduce the facades as a first class part of the library. This is to support
  truly dynamic data where you don't know the keys up front. Could be just aliases, depending
  on how this looks in the docs. If we do this, we should support initializing the collections
  from an iterator.

- Consider removing the magnitude value from the HashMap & SparseScalarHashMap, and
  only maintain this concept for the fixed-sized maps.

- Simplify the set types to hash/ordered/scalar/string sets with the maps as a
  generic argument.

## Type System Nightmares

These are some things which I haven't done yet since I can't figure out how to express these things in the
Rust type system. If you read this and you have some ideas, let me know :-)

- The Map and Set traits do not implement Borrow<T> semantics on their APIs, unlike all the maps & set
  implementation types. It doesn't seem possible to implement Borrow<T> on a trait unfortunately, given how
  it composes.

- FacadeStringSet/Map should have &str as key type instead of String in order to be compatible with all the other
  collections. Unfortunately, switching this over is proving difficult.

- Make the SetIterator and MapIterator traits have IntoIterator<Item = &T> as a super-trait. This would seem to require
  introducing lifetime annotations in these traits, which would really mess everything else us.

- Make the Set and Map traits implement Eq and PartialEq.
