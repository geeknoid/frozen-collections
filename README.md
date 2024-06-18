# Frozen Collections

[![Test Status](https://github.com/geeknoid/frozen-collections/workflows/Tests/badge.svg?event=push)](https://github.com/geeknoid/frozen-collections/actions)
[![Crate](https://img.shields.io/crates/v/frozen-collections.svg)](https://crates.io/crates/frozen-collections)
[![API](https://docs.rs/frozen-collections/badge.svg)](https://docs.rs/frozen-collections)

Frozen collections are designed to trade creation time for improved
read performance. They are ideal for use with long-lasting collections
which get initialized when an application starts and remain unchanged
permanently, or at least extended periods of time. This is a common
pattern in service applications.

During creation, the input data is analyzed to determine the best layout and algorithm for the specific case.
This analysis can take some time, but the value in spending this time up front
is that the collections provide blazingly fast read-time performance.
