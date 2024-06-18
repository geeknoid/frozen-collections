# TODO

## Docs

- Update docs to discuss traits and object safety.

- Update docs to mention no_std support.

## Performance

- Look at https://lib.rs/crates/small-map for SIMD accelerated small maps.

- with strings, use unique substrings as the actual hash code when possible.

- Fine-tune the thresholds when we switch between different implementation types in
  the facades and in the generator.

## Features

- Consider adding support for case-insensitive strings.

- Extend the Scalar derive macro to support more varieties of enum types.

- Simplify the sets/inline_sets to only have hash/ordered/scalar/string variants with the maps as a
  generic argument.

- Support serde serialization/deserialization. All  the collections should be serializable. On the input side,
  we should be smart and do the runtime analysis needed to pick the right collection implementation type.

- Provide a mechanism that can be used in a build.rs script to generate static frozen collections from
  arbitrary input. Read from file into data structure, give the data structure to the API, and you get
  back a token stream representing the initialization of a frozen collection optimized for the data.

## Trait Nightmares

I'd like to add some super-traits to some existing traits, but unfortunately doing
so seemingly requires adding lifetime annotations to the existing traits, which I think
would be pretty painful. Any better idea?

```
SetIterator : IntoIterator<Item = &T>
MatIterator: IntroIterator<Item = &(K, V)>
Map: Eq + PartialEq
Set: Eq + PartialEq + BitOr + BitAnd + BitXor + Sub + SetOps
MapQuery: Index
```