# TODO

## Performance

- Look at https://lib.rs/crates/small-map for SIMD accelerated small maps.

- with strings, use unique substrings as the actual hash code when possible.

- Fine-tune the thresholds when we switch between different implementation types in
  the facades and in the generator.

## Features

- Consider adding support for case-insensitive strings.

- Extend the Scalar derive macro to support more varieties of enum types.

- Simplify the sets to only have hash/ordered/scalar/string variants with the maps as a
  generic argument. This would require the Map and Set traits to implement Borrow<T>
  (see below)

- Support serde serialization/deserialization. All of the collections should be serializable. On the input side,
  we should be smart and do the runtime analysis needed to pick the right collection implementation type.

## Type System Nightmares

These are some things which I haven't done yet since I can't figure out how to express these things in the
Rust type system. If you read this, and you have some ideas, let me know :-)

- The Map and Set traits do not implement Equivalent<T> semantics on their APIs, unlike all the maps & set
  implementation types. It doesn't seem possible to implement Equivalent<T> on a trait unfortunately, given how
  it composes.

- Make the SetIterator and MapIterator traits have IntoIterator<Item = &T> as a super-trait. This would seem to require
  introducing lifetime annotations in these traits, which would really mess everything else us.

- Make the Set and Map traits implement Eq and PartialEq.

- Would be great if the Map and Set traits could be object safe.
