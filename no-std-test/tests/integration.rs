//! Integration tests that exercise the no_std-compiled library.
//!
//! The `no_std_test` crate's lib target is compiled as `#![no_std]`.
//! These tests verify that the collections created under that constraint
//! actually work correctly.

use no_std_test::*;

// === Runtime-constructed collections ===

#[test]
fn hash_map_lookup() {
    let map = make_hash_map();
    assert_eq!(map.get(&1), Some(&"one"));
    assert_eq!(map.get(&2), Some(&"two"));
    assert_eq!(map.get(&3), Some(&"three"));
    assert_eq!(map.get(&4), None);
}

#[test]
fn ordered_map_lookup() {
    let map = make_ordered_map();
    assert_eq!(map.get(&1), Some(&"one"));
    assert_eq!(map.get(&2), Some(&"two"));
    assert_eq!(map.get(&3), Some(&"three"));
    assert_eq!(map.get(&4), None);
}

#[test]
fn scalar_map_lookup() {
    let map = make_scalar_map();
    assert_eq!(map.get(&1), Some(&"one"));
    assert_eq!(map.get(&2), Some(&"two"));
    assert_eq!(map.get(&3), Some(&"three"));
    assert_eq!(map.get(&4), None);
}

#[test]
fn string_map_lookup() {
    let map = make_string_map();
    assert_eq!(map.get("one"), Some(&1));
    assert_eq!(map.get("two"), Some(&2));
    assert_eq!(map.get("three"), Some(&3));
    assert_eq!(map.get("four"), None);
}

#[test]
fn hash_set_contains() {
    let set = make_hash_set();
    assert!(set.contains(&1));
    assert!(set.contains(&2));
    assert!(set.contains(&3));
    assert!(!set.contains(&4));
}

#[test]
fn ordered_set_contains() {
    let set = make_ordered_set();
    assert!(set.contains(&1));
    assert!(set.contains(&2));
    assert!(set.contains(&3));
    assert!(!set.contains(&4));
}

#[test]
fn scalar_set_contains() {
    let set = make_scalar_set();
    assert!(set.contains(&1));
    assert!(set.contains(&2));
    assert!(set.contains(&3));
    assert!(!set.contains(&4));
}

#[test]
fn string_set_contains() {
    let set = make_string_set();
    assert!(set.contains("one"));
    assert!(set.contains("two"));
    assert!(set.contains("three"));
    assert!(!set.contains("four"));
}

// === Macro-constructed collections ===

#[test]
fn macro_hash_map_compiles() {
    assert_eq!(macro_hash_map(), 3);
}

#[test]
fn macro_ordered_map_compiles() {
    assert_eq!(macro_ordered_map(), 3);
}

#[test]
fn macro_scalar_map_compiles() {
    assert_eq!(macro_scalar_map(), 3);
}

#[test]
fn macro_string_map_compiles() {
    assert_eq!(macro_string_map(), 3);
}

#[test]
fn macro_hash_set_compiles() {
    assert_eq!(macro_hash_set(), 3);
}

#[test]
fn macro_ordered_set_compiles() {
    assert_eq!(macro_ordered_set(), 3);
}

#[test]
fn macro_scalar_set_compiles() {
    assert_eq!(macro_scalar_set(), 3);
}

#[test]
fn macro_string_set_compiles() {
    assert_eq!(macro_string_set(), 3);
}
