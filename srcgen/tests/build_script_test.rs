use frozen_collections::{Len, SetQuery};

include!(concat!(env!("OUT_DIR"), "/data.rs"));
include!("../includes/make_collections.rs");

// this test validates that the build script has generated the collections
#[test]
fn check_collections_exist() {
    assert_eq!(2, SMALL_STATIC_ORDERED_SET.len());
    let _: &A1 = &SMALL_STATIC_ORDERED_SET;
    assert!(SMALL_STATIC_ORDERED_SET.contains(&"Red"));

    assert_eq!(9, MEDIUM_STATIC_ORDERED_SET.len());
    let _: &A2 = &MEDIUM_STATIC_ORDERED_SET;
    assert!(MEDIUM_STATIC_ORDERED_SET.contains(&"Red"));

    assert_eq!(72, LARGE_STATIC_ORDERED_SET.len());
    let _: &A3 = &LARGE_STATIC_ORDERED_SET;
    assert!(LARGE_STATIC_ORDERED_SET.contains(&"1Red"));

    assert_eq!(2, SMALL_STATIC_HASH_SET.len());
    assert!(SMALL_STATIC_HASH_SET.contains(&"Red"));

    assert_eq!(9, MEDIUM_STATIC_HASH_SET.len());
    assert!(MEDIUM_STATIC_HASH_SET.contains(&"Red"));
}

// this test runs the same code as the build script to register code coverage
#[test]
fn invoke_emitter() {
    for coll in &make_static_collections() {
        assert!(!coll.is_empty());
    }
}
