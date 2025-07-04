#![expect(missing_docs, reason = "Tests")]

use frozen_collections_core::macros::{fz_ordered_map_macro, fz_ordered_set_macro};
use frozen_collections_macros::*;
use quote::quote;
use std::collections::BTreeMap as StdBTreeMap;
use std::collections::BTreeSet as StdBTreeSet;

#[derive(Hash, Clone, Copy, Debug, PartialEq, Eq, Ord, PartialOrd)]
struct Person {
    name: &'static str,
    age: i32,
}

macro_rules! test_ordered {
    ( $type:ty, $( $arg:expr ),* $(,)?) => {
        {
            _ = fz_ordered_set_macro(quote!({
                $(
                    $arg,
                )*
            })).unwrap();

            let s0 = fz_ordered_set!({
                $(
                    $arg,
                )*
            });

            let v = vec![
                $(
                    $arg,
                )*
            ];

            let mut s2 = StdBTreeSet::new();
            for x in v.into_iter() {
                _ = s2.insert(x);
            }

            _ = fz_ordered_set_macro(quote!(static _S3: Foo< $type >, {
                $(
                    $arg,
                )*
            })).unwrap();

            fz_ordered_set!(static _S3: Foo< $type >, {
                $(
                    $arg,
                )*
            });

            _ = fz_ordered_set_macro(quote!(let s4: Bar< $type >, {
                $(
                    $arg,
                )*
            })).unwrap();

            fz_ordered_set!(let s4: Bar< $type >, {
                $(
                    $arg,
                )*
            });

            assert_eq!(s0, s2);
            // assert_eq!(s0, S3);
            assert_eq!(s0, s4);
        }

        {
            _ = fz_ordered_map_macro(quote!({
                $(
                    $arg: 42,
                )*
            })).unwrap();

            let m0 = fz_ordered_map!({
                $(
                    $arg: 42,
                )*
            });

            let v = vec![
                $(
                    ($arg, 42),
                )*
            ];

            let mut m2 = StdBTreeMap::new();
            for x in v.into_iter() {
                _ = m2.insert(x.0, x.1);
            }

            _ = fz_ordered_map_macro(quote!(static _M3: Foo< $type, i32 >, {
                $(
                    $arg: 42,
                )*
            })).unwrap();

            fz_ordered_map!(static _M3: Foo< $type, i32 >, {
                $(
                    $arg: 42,
                )*
            });

            _ = fz_ordered_map_macro(quote!(let m4: Bar< $type, i32 >, {
                $(
                    $arg: 42,
                )*
            })).unwrap();

            fz_ordered_map!(let m4: Bar< $type, i32 >, {
                $(
                    $arg: 42,
                )*
            });

            assert_eq!(m0, m2);
            // assert_eq!(m0, M3);
            assert_eq!(m0, m4);
        }
    }
}

#[test]
fn ordered_complex() {
    test_ordered!(Person, Person { name: "A", age: 1 },);
    test_ordered!(Person, Person { name: "A", age: 1 }, Person { name: "B", age: 2 },);
    test_ordered!(
        Person,
        Person { name: "A", age: 1 },
        Person { name: "B", age: 2 },
        Person { name: "C", age: 3 },
    );
    test_ordered!(
        Person,
        Person { name: "A", age: 1 },
        Person { name: "B", age: 2 },
        Person { name: "C", age: 3 },
        Person { name: "D", age: 4 },
    );
    test_ordered!(
        Person,
        Person { name: "A", age: 1 },
        Person { name: "B", age: 2 },
        Person { name: "C", age: 3 },
        Person { name: "D", age: 4 },
        Person { name: "E", age: 5 },
    );
    test_ordered!(
        Person,
        Person { name: "A", age: 1 },
        Person { name: "B", age: 2 },
        Person { name: "C", age: 3 },
        Person { name: "D", age: 4 },
        Person { name: "E", age: 5 },
        Person { name: "F", age: 6 },
    );
    test_ordered!(
        Person,
        Person { name: "A", age: 1 },
        Person { name: "B", age: 2 },
        Person { name: "C", age: 3 },
        Person { name: "D", age: 4 },
        Person { name: "E", age: 5 },
        Person { name: "F", age: 6 },
        Person { name: "G", age: 7 },
    );
    test_ordered!(
        Person,
        Person { name: "A", age: 1 },
        Person { name: "B", age: 2 },
        Person { name: "C", age: 3 },
        Person { name: "D", age: 4 },
        Person { name: "E", age: 5 },
        Person { name: "F", age: 6 },
        Person { name: "G", age: 7 },
        Person { name: "H", age: 8 },
    );
    test_ordered!(
        Person,
        Person { name: "A", age: 1 },
        Person { name: "B", age: 2 },
        Person { name: "C", age: 3 },
        Person { name: "D", age: 4 },
        Person { name: "E", age: 5 },
        Person { name: "F", age: 6 },
        Person { name: "G", age: 7 },
        Person { name: "H", age: 8 },
        Person { name: "I", age: 9 },
    );
    test_ordered!(
        Person,
        Person { name: "A", age: 1 },
        Person { name: "B", age: 2 },
        Person { name: "C", age: 3 },
        Person { name: "D", age: 4 },
        Person { name: "E", age: 5 },
        Person { name: "F", age: 6 },
        Person { name: "G", age: 7 },
        Person { name: "H", age: 8 },
        Person { name: "I", age: 9 },
        Person { name: "J", age: 10 },
    );
    test_ordered!(
        Person,
        Person { name: "A", age: 1 },
        Person { name: "B", age: 2 },
        Person { name: "C", age: 3 },
        Person { name: "D", age: 4 },
        Person { name: "E", age: 5 },
        Person { name: "F", age: 6 },
        Person { name: "G", age: 7 },
        Person { name: "H", age: 8 },
        Person { name: "I", age: 9 },
        Person { name: "J", age: 10 },
        Person { name: "K", age: 11 },
    );
    test_ordered!(
        Person,
        Person { name: "A", age: 1 },
        Person { name: "B", age: 2 },
        Person { name: "C", age: 3 },
        Person { name: "D", age: 4 },
        Person { name: "E", age: 5 },
        Person { name: "F", age: 6 },
        Person { name: "G", age: 7 },
        Person { name: "H", age: 8 },
        Person { name: "I", age: 9 },
        Person { name: "J", age: 10 },
        Person { name: "K", age: 11 },
        Person { name: "L", age: 12 },
    );
    test_ordered!(
        Person,
        Person { name: "A", age: 1 },
        Person { name: "B", age: 2 },
        Person { name: "C", age: 3 },
        Person { name: "D", age: 4 },
        Person { name: "E", age: 5 },
        Person { name: "F", age: 6 },
        Person { name: "G", age: 7 },
        Person { name: "H", age: 8 },
        Person { name: "I", age: 9 },
        Person { name: "J", age: 10 },
        Person { name: "K", age: 11 },
        Person { name: "L", age: 12 },
        Person { name: "M", age: 13 },
    );

    test_ordered!(
        Person,
        Person { name: "A", age: 1 },
        Person { name: "B", age: 2 },
        Person { name: "C", age: 3 },
        Person { name: "D", age: 4 },
        Person { name: "E", age: 5 },
        Person { name: "F", age: 6 },
        Person { name: "G", age: 7 },
        Person { name: "H", age: 8 },
        Person { name: "I", age: 9 },
        Person { name: "J", age: 10 },
        Person { name: "K", age: 11 },
        Person { name: "L", age: 12 },
        Person { name: "M", age: 13 },
        Person { name: "xA", age: 1 },
        Person { name: "xB", age: 2 },
        Person { name: "xC", age: 3 },
        Person { name: "xD", age: 4 },
        Person { name: "xE", age: 5 },
        Person { name: "xF", age: 6 },
        Person { name: "xG", age: 7 },
        Person { name: "xH", age: 8 },
        Person { name: "xI", age: 9 },
        Person { name: "xJ", age: 10 },
        Person { name: "xK", age: 11 },
        Person { name: "xL", age: 12 },
        Person { name: "xM", age: 13 },
        Person { name: "aA", age: 1 },
        Person { name: "aB", age: 2 },
        Person { name: "aC", age: 3 },
        Person { name: "aD", age: 4 },
        Person { name: "aE", age: 5 },
        Person { name: "aF", age: 6 },
        Person { name: "aG", age: 7 },
        Person { name: "aH", age: 8 },
        Person { name: "aI", age: 9 },
        Person { name: "aJ", age: 10 },
        Person { name: "aK", age: 11 },
        Person { name: "aL", age: 12 },
        Person { name: "aM", age: 13 },
        Person { name: "zA", age: 1 },
        Person { name: "zB", age: 2 },
        Person { name: "zC", age: 3 },
        Person { name: "zD", age: 4 },
        Person { name: "zE", age: 5 },
        Person { name: "zF", age: 6 },
        Person { name: "zG", age: 7 },
        Person { name: "zH", age: 8 },
        Person { name: "zI", age: 9 },
        Person { name: "zJ", age: 10 },
        Person { name: "zK", age: 11 },
        Person { name: "zL", age: 12 },
        Person { name: "zM", age: 13 },
        Person { name: "vA", age: 1 },
        Person { name: "vB", age: 2 },
        Person { name: "vC", age: 3 },
        Person { name: "vD", age: 4 },
        Person { name: "vE", age: 5 },
        Person { name: "vF", age: 6 },
        Person { name: "vG", age: 7 },
        Person { name: "vH", age: 8 },
        Person { name: "vI", age: 9 },
        Person { name: "vJ", age: 10 },
        Person { name: "vK", age: 11 },
        Person { name: "vL", age: 12 },
        Person { name: "vM", age: 13 },
    );

    // test duplicate logic
    test_ordered!(
        Person,
        Person { name: "A", age: 1 },
        Person { name: "B", age: 2 },
        Person { name: "A", age: 3 },
        Person { name: "A", age: 4 },
    );
}

#[test]
fn ordered_i8() {
    test_ordered!(i8, 0_i8);
    test_ordered!(i8, 0_i8, 1,);
    test_ordered!(i8, 0_i8, 1, 2,);
    test_ordered!(i8, 0_i8, 1, 2, 3,);
    test_ordered!(i8, 0_i8, 1, 2, 3, 4,);
    test_ordered!(i8, 0_i8, 1, 2, 3, 4, 5,);
    test_ordered!(i8, 0_i8, 1, 2, 3, 4, 5, 6,);
    test_ordered!(i8, 0_i8, 1, 2, 3, 4, 5, 6, 7,);
    test_ordered!(i8, 0_i8, 1, 2, 3, 4, 5, 6, 7, 8,);
    test_ordered!(i8, 0_i8, 1, 2, 3, 4, 5, 6, 7, 8, 9,);
    test_ordered!(i8, 0_i8, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10,);
    test_ordered!(i8, 0_i8, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11);
    test_ordered!(i8, 0_i8, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12);
    test_ordered!(i8, 0_i8, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13);

    test_ordered!(
        i8, 0_i8, 0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24, 25, 26, 27, 28, 29,
    );

    test_ordered!(
        i8, 0_i8, 0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24, 25, 26, 27, 28, 29, 30, 31, 32,
        33, 34, 35, 36, 37, 38, 39, 40, 41, 42, 43, 44, 45, 46, 47, 48, 49, 50, 51, 52, 53, 54, 55, 56, 57, 58, 59,
    );

    // test duplicate logic
    test_ordered!(i8, 0_i8, 1, 2, 1, 1);
}

#[test]
fn ordered_u8() {
    test_ordered!(u8, 0_u8);
    test_ordered!(u8, 0_u8, 1,);
    test_ordered!(u8, 0_u8, 1, 2,);
    test_ordered!(u8, 0_u8, 1, 2, 3,);
    test_ordered!(u8, 0_u8, 1, 2, 3, 4,);
    test_ordered!(u8, 0_u8, 1, 2, 3, 4, 5,);
    test_ordered!(u8, 0_u8, 1, 2, 3, 4, 5, 6,);
    test_ordered!(u8, 0_u8, 1, 2, 3, 4, 5, 6, 7,);
    test_ordered!(u8, 0_u8, 1, 2, 3, 4, 5, 6, 7, 8,);
    test_ordered!(u8, 0_u8, 1, 2, 3, 4, 5, 6, 7, 8, 9,);
    test_ordered!(u8, 0_u8, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10,);
    test_ordered!(u8, 0_u8, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11);
    test_ordered!(u8, 0_u8, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12);
    test_ordered!(u8, 0_u8, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13);
}

#[test]
fn ordered_i16() {
    test_ordered!(i16, 0_i16);
    test_ordered!(i16, 0_i16, 1,);
    test_ordered!(i16, 0_i16, 1, 2,);
    test_ordered!(i16, 0_i16, 1, 2, 3,);
    test_ordered!(i16, 0_i16, 1, 2, 3, 4,);
    test_ordered!(i16, 0_i16, 1, 2, 3, 4, 5,);
    test_ordered!(i16, 0_i16, 1, 2, 3, 4, 5, 6,);
    test_ordered!(i16, 0_i16, 1, 2, 3, 4, 5, 6, 7,);
    test_ordered!(i16, 0_i16, 1, 2, 3, 4, 5, 6, 7, 8,);
    test_ordered!(i16, 0_i16, 1, 2, 3, 4, 5, 6, 7, 8, 9,);
    test_ordered!(i16, 0_i16, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10,);
    test_ordered!(i16, 0_i16, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11);
    test_ordered!(i16, 0_i16, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12);
    test_ordered!(i16, 0_i16, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13);
}

#[test]
fn ordered_u16() {
    test_ordered!(u16, 0_u16);
    test_ordered!(u16, 0_u16, 1,);
    test_ordered!(u16, 0_u16, 1, 2,);
    test_ordered!(u16, 0_u16, 1, 2, 3,);
    test_ordered!(u16, 0_u16, 1, 2, 3, 4,);
    test_ordered!(u16, 0_u16, 1, 2, 3, 4, 5,);
    test_ordered!(u16, 0_u16, 1, 2, 3, 4, 5, 6,);
    test_ordered!(u16, 0_u16, 1, 2, 3, 4, 5, 6, 7,);
    test_ordered!(u16, 0_u16, 1, 2, 3, 4, 5, 6, 7, 8,);
    test_ordered!(u16, 0_u16, 1, 2, 3, 4, 5, 6, 7, 8, 9,);
    test_ordered!(u16, 0_u16, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10,);
    test_ordered!(u16, 0_u16, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11);
    test_ordered!(u16, 0_u16, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12);
    test_ordered!(u16, 0_u16, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13);
}

#[test]
fn ordered_i32() {
    test_ordered!(i32, 0_i32);
    test_ordered!(i32, 0_i32, 1,);
    test_ordered!(i32, 0_i32, 1, 2,);
    test_ordered!(i32, 0_i32, 1, 2, 3,);
    test_ordered!(i32, 0_i32, 1, 2, 3, 4,);
    test_ordered!(i32, 0_i32, 1, 2, 3, 4, 5,);
    test_ordered!(i32, 0_i32, 1, 2, 3, 4, 5, 6,);
    test_ordered!(i32, 0_i32, 1, 2, 3, 4, 5, 6, 7,);
    test_ordered!(i32, 0_i32, 1, 2, 3, 4, 5, 6, 7, 8,);
    test_ordered!(i32, 0_i32, 1, 2, 3, 4, 5, 6, 7, 8, 9,);
    test_ordered!(i32, 0_i32, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10,);
    test_ordered!(i32, 0_i32, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11);
    test_ordered!(i32, 0_i32, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12);
    test_ordered!(i32, 0_i32, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13);
}

#[test]
fn ordered_u32() {
    test_ordered!(u32, 0_u32);
    test_ordered!(u32, 0_u32, 1,);
    test_ordered!(u32, 0_u32, 1, 2,);
    test_ordered!(u32, 0_u32, 1, 2, 3,);
    test_ordered!(u32, 0_u32, 1, 2, 3, 4,);
    test_ordered!(u32, 0_u32, 1, 2, 3, 4, 5,);
    test_ordered!(u32, 0_u32, 1, 2, 3, 4, 5, 6,);
    test_ordered!(u32, 0_u32, 1, 2, 3, 4, 5, 6, 7,);
    test_ordered!(u32, 0_u32, 1, 2, 3, 4, 5, 6, 7, 8,);
    test_ordered!(u32, 0_u32, 1, 2, 3, 4, 5, 6, 7, 8, 9,);
    test_ordered!(u32, 0_u32, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10,);
    test_ordered!(u32, 0_u32, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11);
    test_ordered!(u32, 0_u32, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12);
    test_ordered!(u32, 0_u32, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13);
}

#[test]
fn ordered_i64() {
    test_ordered!(i64, 0_i64);
    test_ordered!(i64, 0_i64, 1,);
    test_ordered!(i64, 0_i64, 1, 2,);
    test_ordered!(i64, 0_i64, 1, 2, 3,);
    test_ordered!(i64, 0_i64, 1, 2, 3, 4,);
    test_ordered!(i64, 0_i64, 1, 2, 3, 4, 5,);
    test_ordered!(i64, 0_i64, 1, 2, 3, 4, 5, 6,);
    test_ordered!(i64, 0_i64, 1, 2, 3, 4, 5, 6, 7,);
    test_ordered!(i64, 0_i64, 1, 2, 3, 4, 5, 6, 7, 8,);
    test_ordered!(i64, 0_i64, 1, 2, 3, 4, 5, 6, 7, 8, 9,);
    test_ordered!(i64, 0_i64, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10,);
    test_ordered!(i64, 0_i64, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11);
    test_ordered!(i64, 0_i64, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12);
    test_ordered!(i64, 0_i64, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13);
}

#[test]
fn ordered_u64() {
    test_ordered!(u64, 0_u64);
    test_ordered!(u64, 0_u64, 1,);
    test_ordered!(u64, 0_u64, 1, 2,);
    test_ordered!(u64, 0_u64, 1, 2, 3,);
    test_ordered!(u64, 0_u64, 1, 2, 3, 4,);
    test_ordered!(u64, 0_u64, 1, 2, 3, 4, 5,);
    test_ordered!(u64, 0_u64, 1, 2, 3, 4, 5, 6,);
    test_ordered!(u64, 0_u64, 1, 2, 3, 4, 5, 6, 7,);
    test_ordered!(u64, 0_u64, 1, 2, 3, 4, 5, 6, 7, 8,);
    test_ordered!(u64, 0_u64, 1, 2, 3, 4, 5, 6, 7, 8, 9,);
    test_ordered!(u64, 0_u64, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10,);
    test_ordered!(u64, 0_u64, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11);
    test_ordered!(u64, 0_u64, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12);
    test_ordered!(u64, 0_u64, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13);
}

#[test]
fn ordered_string() {
    test_ordered!(&'static str, "0");
    test_ordered!(&'static str, "0", "1");
    test_ordered!(&'static str, "0", "1", "2");
    test_ordered!(&'static str, "0", "1", "2", "3");
    test_ordered!(&'static str, "0", "1", "2", "3", "4");
    test_ordered!(&'static str, "0", "1", "2", "3", "4", "5");
    test_ordered!(&'static str, "0", "1", "2", "3", "4", "5", "6");
    test_ordered!(&'static str, "0", "1", "2", "3", "4", "5", "6", "7");
    test_ordered!(&'static str, "0", "1", "2", "3", "4", "5", "6", "7", "8");
    test_ordered!(&'static str, "0", "1", "2", "3", "4", "5", "6", "7", "8", "9");
    test_ordered!(&'static str, "0", "1", "2", "3", "4", "5", "6", "7", "8", "9", "10");
    test_ordered!(&'static str, "0", "1", "2", "3", "4", "5", "6", "7", "8", "9", "10", "11");
    test_ordered!(&'static str, "0", "1", "2", "3", "4", "5", "6", "7", "8", "9", "10", "11", "12");
    test_ordered!(
        &'static str,
        "0",
        "1",
        "2",
        "3",
        "4",
        "5",
        "6",
        "7",
        "8",
        "9",
        "10",
        "11",
        "12",
        "13"
    );
}
