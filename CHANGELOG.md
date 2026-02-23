# Changelog

## 0.9.0 - 2026-02-22

### Changed

- Changed the layout in the core hash table types to improve cache
  locality.

- Implement contains_key as a first-class method on the low-level map types
  to help out the compiler by avoiding format a reference to the value when
  we don't care about it.

- FzStringMap/Set now go into turbo mode if all the keys are of
  different lengths by using their lengths as hash code, eliminating
  the need to actually hash anything.

- Removed a needless temp allocation when building FzStringMap and FzStringSet.

### Fixed

- Fix a bug in the hash code analysis which lead to some maps not applying the right
  optimization strategy.

- Added a missing compiler hint in one of the hashing paths.

- Added the missing ability to deserialize a FzStringMap using a custom hash builder.

- Added missing `FusedIterator` implementation for `maps::Iter`. All other map
  iterators (`IterMut`, `Keys`, `Values`, `ValuesMut`, `IntoIter`, `IntoKeys`,
  `IntoValues`) already implemented it.

- Fixed incorrect `Eq` bound on `InlineEytzingerSearchMap`: changed `V: PartialEq`
  to `V: Eq`, matching every other map type in the codebase.

## 0.8.0 - 2025-07-04

### Changed

- Save a few cycles in the hashing collections.

- Renamed the DefaultHashBuilder type to DefaultBuildHasher which makes more sense.

- Fixed a couple bugs where undedupped vectors were being used when they should have been dedupped.

- Fixed a bug where fz_ordered_map/set would sometimes produce non-working maps due to the
  data vector not being sorted correctly.

## 0.7.0 - 2025-06-22

### Added

- Introduce hash collections optimized for the common no-collision case.

## 0.6.0 - 2025-06-21

### Added

- Added the get_disjoint_unchecked_mut function to the Map trait to match what HashMap has.

### Changed

- The get_many_mut function on the Map trait has been renamed to get_disjoint_mut to match the stable name
used in HashMap.

- Improved usability by implementing all the methods that
were previously just on the traits as normal methods on the
collections themselves. This avoids the need to import
the traits to use the collections, making them more user-friendly.

- Revamped the FzStringMap/Set types. Their implementation
is now simpler, yet the API is more flexible.

- Tidied up a lot of generic bounds. Many of the
bounds are removed, making the types easier to use

- SetQuery and MapQuery now have one fewer generics,
making them considerably easier and natural to use.

- Enabled more lints and fixed the resulting warnings.

## 0.5.0 - 2025-04-17

### Added

- Added support for keys of types String to the FzStringMap and FzStringSet types.

### Changed

- Update to latest Rust version and dependencies.

## 0.4.0 - 2025-03-15

### Changed

- Various small perf improvements.

- Completed conversion from ahash to foldhash which gave some good performance gains in specific scenarios.

- Added missing ?Sized to the definition of the Q generic in a few
collection types. This missing annotation would lead to compilation
errors depending on the collection used and the type of the key

- Update to latest Rust version and dependencies.

## 0.3.0 - 2024-12-29

### Added

- The `emit` cargo feature controls the availability of the `emit` module, which provides
a way to emit frozen collections from a cargo build script.

- Added the `DefaultHashBuilder` alias for the hash builder used by default in the 
crate.

### Removed

- You can no longer use the frozen collection macros with a vector of input values, now
you can only use inline literal values. To use a vector of values, you are expected to
use the concrete FzXXX types instead.

- A few of the FxXXX::new functions that used to take a BuildHasher instance no longer do,
and instead with_hasher functions were added. This was done to better align with the
way the standard HashMap/HashSet APIs work.

### Changed

- Changed the default hash builder from `ahash` to `foldhash` which is generally
faster. Unfortunately, since `foldhash` doesn't currently provide a mechanism to
use explicitly initialized seeds, we need to use `ahash` in a few
situations.

## 0.2.0 - 2024-12-25

### Added

- All frozen collection types can now be serialized using `serde`.

- The `serde` cargo feature controls the availability of serialization/deserialization support.

- The `macros` cargo feature controls the availability of the frozen collection macros.

- The frozen collection facade types are now exposed as first-class types and are the 
recommended way to use frozen collections with data discovered at runtime, whereas the
macros are for data known at compile time.

## 0.1.0 - 2024-12-19

### Added

- Initial release
