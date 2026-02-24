//! Compile-time verification that frozen-collections-core works without std.
//!
//! This crate's lib target is compiled as `#![no_std]`. If any `std`-only
//! code leaks into the non-std build, this crate will fail to compile.
//! The integration tests exercise the API from a std test harness.

#![no_std]

extern crate alloc;

use alloc::string::String;
use alloc::vec;
use frozen_collections::{
    FzHashMap, FzHashSet, FzOrderedMap, FzOrderedSet, FzScalarMap, FzScalarSet, FzStringMap, FzStringSet, fz_hash_map, fz_hash_set,
    fz_ordered_map, fz_ordered_set, fz_scalar_map, fz_scalar_set, fz_string_map, fz_string_set,
};

// === Runtime-constructed collections ===

/// Creates a [`FzHashMap`] with sample data.
#[must_use]
pub fn make_hash_map() -> FzHashMap<i64, &'static str> {
    FzHashMap::new(vec![(1, "one"), (2, "two"), (3, "three")])
}

/// Creates a [`FzOrderedMap`] with sample data.
#[must_use]
pub fn make_ordered_map() -> FzOrderedMap<i64, &'static str> {
    FzOrderedMap::new(vec![(1, "one"), (2, "two"), (3, "three")])
}

/// Creates a [`FzScalarMap`] with sample data.
#[must_use]
pub fn make_scalar_map() -> FzScalarMap<i32, &'static str> {
    FzScalarMap::new(vec![(1, "one"), (2, "two"), (3, "three")])
}

/// Creates a [`FzStringMap`] with sample data.
#[must_use]
pub fn make_string_map() -> FzStringMap<alloc::boxed::Box<str>, i32> {
    FzStringMap::new(vec![(String::from("one"), 1), (String::from("two"), 2), (String::from("three"), 3)])
}

/// Creates a [`FzHashSet`] with sample data.
#[must_use]
pub fn make_hash_set() -> FzHashSet<i64> {
    FzHashSet::new(vec![1, 2, 3])
}

/// Creates a [`FzOrderedSet`] with sample data.
#[must_use]
pub fn make_ordered_set() -> FzOrderedSet<i64> {
    FzOrderedSet::new(vec![1, 2, 3])
}

/// Creates a [`FzScalarSet`] with sample data.
#[must_use]
pub fn make_scalar_set() -> FzScalarSet<i32> {
    FzScalarSet::new(vec![1, 2, 3])
}

/// Creates a [`FzStringSet`] with sample data.
#[must_use]
pub fn make_string_set() -> FzStringSet<alloc::boxed::Box<str>> {
    FzStringSet::new(vec![String::from("one"), String::from("two"), String::from("three")])
}

// === Macro-constructed collections ===

/// Exercises [`fz_hash_map!`] under `no_std`. Returns the number of entries.
///
/// # Panics
///
/// Panics if the macro-constructed map doesn't contain the expected entry.
#[must_use]
pub fn macro_hash_map() -> usize {
    let m = fz_hash_map!({
        1_i64: "one",
        2_i64: "two",
        3_i64: "three",
    });

    assert!(m.get(&1) == Some(&"one"));
    m.len()
}

/// Exercises [`fz_ordered_map!`] under `no_std`. Returns the number of entries.
///
/// # Panics
///
/// Panics if the macro-constructed map doesn't contain the expected entry.
#[must_use]
pub fn macro_ordered_map() -> usize {
    let m = fz_ordered_map!({
        1_i64: "one",
        2_i64: "two",
        3_i64: "three",
    });

    assert!(m.get(&1) == Some(&"one"));
    m.len()
}

/// Exercises [`fz_scalar_map!`] under `no_std`. Returns the number of entries.
///
/// # Panics
///
/// Panics if the macro-constructed map doesn't contain the expected entry.
#[must_use]
pub fn macro_scalar_map() -> usize {
    let m = fz_scalar_map!({
        1_i32: "one",
        2_i32: "two",
        3_i32: "three",
    });

    assert!(m.get(&1) == Some(&"one"));
    m.len()
}

/// Exercises [`fz_string_map!`] under `no_std`. Returns the number of entries.
///
/// # Panics
///
/// Panics if the macro-constructed map doesn't contain the expected entry.
#[must_use]
pub fn macro_string_map() -> usize {
    let m = fz_string_map!({
        "one": 1,
        "two": 2,
        "three": 3,
    });

    assert!(m.get("one") == Some(&1));
    m.len()
}

/// Exercises [`fz_hash_set!`] under `no_std`. Returns the number of entries.
///
/// # Panics
///
/// Panics if the macro-constructed set doesn't contain the expected entry.
#[must_use]
pub fn macro_hash_set() -> usize {
    let s = fz_hash_set!({1_i64, 2_i64, 3_i64});
    assert!(s.contains(&1));
    s.len()
}

/// Exercises [`fz_ordered_set!`] under `no_std`. Returns the number of entries.
///
/// # Panics
///
/// Panics if the macro-constructed set doesn't contain the expected entry.
#[must_use]
pub fn macro_ordered_set() -> usize {
    let s = fz_ordered_set!({1_i64, 2_i64, 3_i64});
    assert!(s.contains(&1));
    s.len()
}

/// Exercises [`fz_scalar_set!`] under `no_std`. Returns the number of entries.
///
/// # Panics
///
/// Panics if the macro-constructed set doesn't contain the expected entry.
#[must_use]
pub fn macro_scalar_set() -> usize {
    let s = fz_scalar_set!({1_i32, 2_i32, 3_i32});
    assert!(s.contains(&1));
    s.len()
}

/// Exercises [`fz_string_set!`] under `no_std`. Returns the number of entries.
///
/// # Panics
///
/// Panics if the macro-constructed set doesn't contain the expected entry.
#[must_use]
pub fn macro_string_set() -> usize {
    let s = fz_string_set!({"one", "two", "three"});
    assert!(s.contains("one"));
    s.len()
}
