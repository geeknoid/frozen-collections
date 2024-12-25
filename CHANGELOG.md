# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

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
