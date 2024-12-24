# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Added

- All frozen collection types can now be serialized using `serde`.

- The `fz_deserialize_*` family of functions can be used to deserialize using serde
into a frozen collection.

- The new `serde` cargo feature must be enabled to use the serialization/deserialization features.

## 0.1.0 - 2024-12-19

### Added

- Initial release
