# TODOs

## Engineering Excellence

- Test coverage

- Global doc review & expansion, more examples

## Performance

- Look at https://lib.rs/crates/small-map for SIMD accelerated small maps

- Create a benchmark suite to try and come up with better numbers for the various threshold and percentages
  used in the analysis code.

- with strings, use unique substrings as the hash code.

- The slice length analysis is too naive. It should not have a limit on the number of lengths considered,
  and it should limit collisions as a percentage instead of with a fixed number.

- Make sure there are no bounds checks in all cases

## Misc

- Would it be possible to remove the requirements in the collections that the data be held in an array of
  (K, V)? This is currently required primarily due to the common iterators for these types. Not having this requirement
  would allow, for example, the DenseSequenceLookupMap to be smaller since it wouldn't need to store keys, and
  it would enable a bit-vector-based integer set design.

- What about case-insensitivity?

- Add update_values to the maps

- Can we use LazyLock to enable using frozen_XXX macros in more static contexts?
