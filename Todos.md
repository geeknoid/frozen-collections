# TODOs

## Engineering Excellence

- Get code coverage to 100%

- Update benchmark results

## Performance

- *ScalarLookupSet should be implemented with bit vectors.

- Look at https://lib.rs/crates/small-map for SIMD accelerated small maps

- Create a benchmark suite to try and come up with better numbers for the various threshold and percentages
  used in the analysis code.

- with strings, use unique substrings as the actual hash code when possible.

- The slice length analysis is too naive. It should not have a limit on the number of lengths considered,
  and it should limit collisions as a percentage instead of with a fixed number.

- Make sure there are no bounds checks in all cases

## Misc

- Would it be possible to remove the requirements in the collections that the data be held in an array of
  (K, V)? This is currently required primarily due to the common iterators for these types. Not having this requirement
  would allow, for example, the DenseScalarLookupMap to be smaller since it wouldn't need to store keys, and
  it would enable a bit-vector-based integer set design.

- What about case-insensitivity?

- Add update_values to the maps

- Extend the Scalar derive macro to support more varieties of enum types

- Should the maps and sets implement From([])?
