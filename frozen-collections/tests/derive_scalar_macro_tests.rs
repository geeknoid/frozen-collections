#![expect(missing_docs, reason = "Tests")]

use frozen_collections::*;
use frozen_collections_core::macros::derive_scalar_macro;
use quote::quote;

#[derive(Scalar, Copy, Ord, PartialOrd, Eq, PartialEq, Clone)]
enum Color {
    Red,
    Green,
    Blue,
}

#[test]
fn test_derive_scalar() {
    _ = derive_scalar_macro(quote!(
        enum Color {
            Red,
            Green,
            Blue,
        }
    ))
    .unwrap();

    assert_eq!(0, Color::index(&Color::Red));
    assert_eq!(1, Color::index(&Color::Green));
    assert_eq!(2, Color::index(&Color::Blue));
}
