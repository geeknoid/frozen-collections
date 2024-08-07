# Frozen Collections

[![Crate](https://img.shields.io/crates/v/frozen-collections.svg)](https://crates.io/crates/frozen-collections)
[![Docs](https://docs.rs/frozen-collections/badge.svg)](https://docs.rs/frozen-collections)
[![Build](https://github.com/geeknoid/frozen-collections/workflows/main/badge.svg)](https://github.com/geeknoid/frozen-collections/actions)
[![Coverage](https://codecov.io/gh/geeknoid/frozen-collections/graph/badge.svg?token=FCUG0EL5TI)](https://codecov.io/gh/geeknoid/frozen-collections)
[![Minimum Supported Rust Version 1.79](https://img.shields.io/badge/MSRV-1.79-blue.svg)]()

Frozen collections are designed to trade creation time for improved
read performance. They are ideal for use with long-lasting collections
which get initialized when an application starts and remain unchanged
permanently, or at least extended periods of time. This is a common
pattern in service applications.

During creation, the input data is analyzed to determine the best layout and algorithm for the specific case.
This analysis can take some time, but the value in spending this time up front
is that the collections provide blazingly fast read-time performance.
