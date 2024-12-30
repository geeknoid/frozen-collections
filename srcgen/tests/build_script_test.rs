use frozen_collections::emit::*;
use frozen_collections::Len;
use proc_macro2::TokenStream;
use syn::parse_quote;

include!(concat!(env!("OUT_DIR"), "/data.rs"));
include!("../includes/make_collections.rs");

// this test validates that the build script has generated the collections
#[test]
fn check_collections_exist() {
    assert!(!MY_SET_1.is_empty());
    assert!(!MY_SET_2.is_empty());
}

// this test runs the same code as the build script in order to register code coverage
#[test]
fn invoke_emitter() {
    let (set1, set2) = make_sets();

    assert!(!set1.is_empty());
    assert!(!set2.is_empty());
}
