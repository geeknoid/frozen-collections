# Ideas

Stuff for the future:

- Create a benchmark suite to try and come up with better numbers for the various threshold and percentages
  used in the analysis code.

- Consider some hint supplied by the caller for how much time/effort to put into analysis.

- Consider the use of perfect hashing or minimal perfect hashing.

- Consider introducing dynamic benchmarking as part of the analysis phase. We could build
  several prototype collections, measure effective perf, and then use the benchmark results to
  decide on the optimal collection configuration.

- Add a specialized set implementation for integer types which uses a bit vector for storage.

- For integer keys, consider expanding the model for ranges to include ranges with holes.
  Basically, the array would hold Option<V> instead of just V.

- Would it be possible to remove the requirements in the specialized maps/sets that the data be held in an array of
  (K, V)? This is currently required primarily due to the common iterators for these types. Not having this requirement
  would allow, for example, the IntegerRangeMap to be smaller since it wouldn't need to store keys. It would also
  provide more flexibility for things like a bit-vector-based set.

- Would it be possible to optimize for the case where the keys are enums?

- For string and complex keys, we should consider storing the hash code alongside the entries. This would
  eliminate most full key comparisons when looking up keys not in the collection.

- Support constant expressions for keys in the macros.
