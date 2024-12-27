# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

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

- The frozen collection facade types are now exposed as first class types and are the 
recommended way to use frozen collections with data discovered at runtime, whereas the
macros are for data known at compile time.

## 0.1.0 - 2024-12-19

### Added

- Initial release
