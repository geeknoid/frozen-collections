# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## Unreleased

### Added

### Changed

## 0.8.0 - 2025-07-04

### Changed

- Save a few cycles in the hashing collections.

- Renamed the DefaultHashBuilder type to DefaultBuildHasher which makes more sense.

- Fixed a couple bugs where undedupped vectors where being used when they should have been dedupped.

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
