# TODO

## Performance

- Look at https://lib.rs/crates/small-map for SIMD accelerated small maps.

- with strings, use unique substrings as the actual hash code when possible.

- Expand benchmarks to test the performance of the library with larger data sets.

- Fine-tune the thresholds when we switch between different implementation types in
  the facades and in the generator.

- Determine the fastest Eytzinger search implementation. Could add prefetching on x86.

## Features

- Consider adding support for case-insensitive strings.

- Extend the Scalar derive macro to support more varieties of enum types.

- Simplify the set types to only have hash/ordered/scalar/string sets with the maps as a
  generic argument. This would require Map and Set to implement Borrow<T>
  (see below)

- Add support for fz_string_set/map to support keys which are of type String.

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
