# TODO

## Engineering Excellence

- Get code coverage to 100%

- Generate BENCHMARK.md

## Performance

- Look at https://lib.rs/crates/small-map for SIMD accelerated small maps.

- with strings, use unique substrings as the actual hash code when possible.

- The slice length analysis is too naive. It should not have a limit on the number of lengths considered,
  and it should limit collisions as a percentage instead of with a fixed number.

## Type System Nightmares

These are some things which I haven't done yet since I can't figure out how to express these things in the
Rust type system.

- The Map and Set traits do not implement Borrow<T> semantics on their APIs, unlike all the maps & set
implementation types. It doesn't seem possible to implement Borrow<T> on a trait unfortunately, given how
it composes.

- Update FacadeString* to have &str as key type instead of String in order to be compatible with all the other
  collections.

- Make SetIterator and MapIterator have IntoIterator<Item = &T> as a super-trait. This would seem to require introducing
lifetime annotations in these traits, which would really mess everything else us.

- Make the Set and Map implement Eq and PartialEq.

## Misc

- Add support for empty literal collections in the macros.

- Add support for non-literal values when using the macros. This will end up using a facade
  type as implementation.

- Consider adding support for case-insensitive strings.

- Extend the Scalar derive macro to support more varieties of enum types.
