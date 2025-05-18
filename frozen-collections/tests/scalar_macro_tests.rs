use frozen_collections::*;
use frozen_collections_core::macros::{fz_scalar_map_macro, fz_scalar_set_macro};
use quote::quote;
use std::collections::BTreeMap as StdBTreeMap;
use std::collections::BTreeSet as StdBTreeSet;

macro_rules! test_scalar {
    ( $type:ty, $( $arg:expr ),* $(,)?) => {
        {
            _ = fz_scalar_set_macro(quote!({
                $(
                    $arg,
                )*
            })).unwrap();

            let s0 = fz_scalar_set!({
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

            _ = fz_scalar_set_macro(quote!(static _S3: Foo< $type >, {
                $(
                    $arg,
                )*
            })).unwrap();

            fz_scalar_set!(static _S3: Foo< $type >, {
                $(
                    $arg,
                )*
            });

            _ = fz_scalar_set_macro(quote!(let S4: Bar< $type >, {
                $(
                    $arg,
                )*
            })).unwrap();

            fz_scalar_set!(let s4: Bar< $type >, {
                $(
                    $arg,
                )*
            });

            assert_eq!(s0, s2);
            // assert_eq!(s0, S3);
            assert_eq!(s0, s4);
        }

        {
            _ = fz_scalar_map_macro(quote!({
                $(
                    $arg: 42,
                )*
            })).unwrap();

            let m0 = fz_scalar_map!({
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

            _ = fz_scalar_map_macro(quote!(static _M3: Foo< $type, i32 >, {
                $(
                    $arg: 42,
                )*
            })).unwrap();

            fz_scalar_map!(static _M3: Foo< $type, i32 >, {
                $(
                    $arg: 42,
                )*
            });

            _ = fz_scalar_map_macro(quote!(let m4: Bar< $type, i32 >, {
                $(
                    $arg: 42,
                )*
            })).unwrap();

            fz_scalar_map!(let m4: Bar< $type, i32 >, {
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
fn scalar_i8() {
    test_scalar!(i8, 0i8);
    test_scalar!(i8, 0i8, 1,);
    test_scalar!(i8, 0i8, 1, 2,);
    test_scalar!(i8, 0i8, 1, 2, 3,);
    test_scalar!(i8, 0i8, 1, 2, 3, 4,);
    test_scalar!(i8, 0i8, 1, 2, 3, 4, 5,);
    test_scalar!(i8, 0i8, 1, 2, 3, 4, 5, 6,);
    test_scalar!(i8, 0i8, 1, 2, 3, 4, 5, 6, 7,);
    test_scalar!(i8, 0i8, 1, 2, 3, 4, 5, 6, 7, 8,);
    test_scalar!(i8, 0i8, 1, 2, 3, 4, 5, 6, 7, 8, 9,);
    test_scalar!(i8, 0i8, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10,);
    test_scalar!(i8, 0i8, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11);
    test_scalar!(i8, 0i8, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12);
    test_scalar!(i8, 0i8, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13);

    // test duplicate logic
    test_scalar!(i8, 0i8, 1, 2, 1, 1);
}

#[test]
fn scalar_u8() {
    test_scalar!(u8, 0u8);
    test_scalar!(u8, 0u8, 1,);
    test_scalar!(u8, 0u8, 1, 2,);
    test_scalar!(u8, 0u8, 1, 2, 3,);
    test_scalar!(u8, 0u8, 1, 2, 3, 4,);
    test_scalar!(u8, 0u8, 1, 2, 3, 4, 5,);
    test_scalar!(u8, 0u8, 1, 2, 3, 4, 5, 6,);
    test_scalar!(u8, 0u8, 1, 2, 3, 4, 5, 6, 7,);
    test_scalar!(u8, 0u8, 1, 2, 3, 4, 5, 6, 7, 8,);
    test_scalar!(u8, 0u8, 1, 2, 3, 4, 5, 6, 7, 8, 9,);
    test_scalar!(u8, 0u8, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10,);
    test_scalar!(u8, 0u8, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11);
    test_scalar!(u8, 0u8, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12);
    test_scalar!(u8, 0u8, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13);
}

#[test]
fn scalar_i16() {
    test_scalar!(i16, 0i16);
    test_scalar!(i16, 0i16, 1,);
    test_scalar!(i16, 0i16, 1, 2,);
    test_scalar!(i16, 0i16, 1, 2, 3,);
    test_scalar!(i16, 0i16, 1, 2, 3, 4,);
    test_scalar!(i16, 0i16, 1, 2, 3, 4, 5,);
    test_scalar!(i16, 0i16, 1, 2, 3, 4, 5, 6,);
    test_scalar!(i16, 0i16, 1, 2, 3, 4, 5, 6, 7,);
    test_scalar!(i16, 0i16, 1, 2, 3, 4, 5, 6, 7, 8,);
    test_scalar!(i16, 0i16, 1, 2, 3, 4, 5, 6, 7, 8, 9,);
    test_scalar!(i16, 0i16, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10,);
    test_scalar!(i16, 0i16, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11);
    test_scalar!(i16, 0i16, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12);
    test_scalar!(i16, 0i16, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13);
}

#[test]
fn scalar_u16() {
    test_scalar!(u16, 0u16);
    test_scalar!(u16, 0u16, 1,);
    test_scalar!(u16, 0u16, 1, 2,);
    test_scalar!(u16, 0u16, 1, 2, 3,);
    test_scalar!(u16, 0u16, 1, 2, 3, 4,);
    test_scalar!(u16, 0u16, 1, 2, 3, 4, 5,);
    test_scalar!(u16, 0u16, 1, 2, 3, 4, 5, 6,);
    test_scalar!(u16, 0u16, 1, 2, 3, 4, 5, 6, 7,);
    test_scalar!(u16, 0u16, 1, 2, 3, 4, 5, 6, 7, 8,);
    test_scalar!(u16, 0u16, 1, 2, 3, 4, 5, 6, 7, 8, 9,);
    test_scalar!(u16, 0u16, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10,);
    test_scalar!(u16, 0u16, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11);
    test_scalar!(u16, 0u16, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12);
    test_scalar!(u16, 0u16, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13);
}

#[test]
fn scalar_i32() {
    test_scalar!(i32, 0i32);
    test_scalar!(i32, 0i32, 1,);
    test_scalar!(i32, 0i32, 1, 2,);
    test_scalar!(i32, 0i32, 1, 2, 3,);
    test_scalar!(i32, 0i32, 1, 2, 3, 4,);
    test_scalar!(i32, 0i32, 1, 2, 3, 4, 5,);
    test_scalar!(i32, 0i32, 1, 2, 3, 4, 5, 6,);
    test_scalar!(i32, 0i32, 1, 2, 3, 4, 5, 6, 7,);
    test_scalar!(i32, 0i32, 1, 2, 3, 4, 5, 6, 7, 8,);
    test_scalar!(i32, 0i32, 1, 2, 3, 4, 5, 6, 7, 8, 9,);
    test_scalar!(i32, 0i32, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10,);
    test_scalar!(i32, 0i32, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11);
    test_scalar!(i32, 0i32, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12);
    test_scalar!(i32, 0i32, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13);
}

#[test]
fn scalar_u32() {
    test_scalar!(u32, 0u32);
    test_scalar!(u32, 0u32, 1,);
    test_scalar!(u32, 0u32, 1, 2,);
    test_scalar!(u32, 0u32, 1, 2, 3,);
    test_scalar!(u32, 0u32, 1, 2, 3, 4,);
    test_scalar!(u32, 0u32, 1, 2, 3, 4, 5,);
    test_scalar!(u32, 0u32, 1, 2, 3, 4, 5, 6,);
    test_scalar!(u32, 0u32, 1, 2, 3, 4, 5, 6, 7,);
    test_scalar!(u32, 0u32, 1, 2, 3, 4, 5, 6, 7, 8,);
    test_scalar!(u32, 0u32, 1, 2, 3, 4, 5, 6, 7, 8, 9,);
    test_scalar!(u32, 0u32, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10,);
    test_scalar!(u32, 0u32, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11);
    test_scalar!(u32, 0u32, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12);
    test_scalar!(u32, 0u32, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13);
}

#[test]
fn scalar_i64() {
    test_scalar!(i64, 0i64);
    test_scalar!(i64, 0i64, 1,);
    test_scalar!(i64, 0i64, 1, 2,);
    test_scalar!(i64, 0i64, 1, 2, 3,);
    test_scalar!(i64, 0i64, 1, 2, 3, 4,);
    test_scalar!(i64, 0i64, 1, 2, 3, 4, 5,);
    test_scalar!(i64, 0i64, 1, 2, 3, 4, 5, 6,);
    test_scalar!(i64, 0i64, 1, 2, 3, 4, 5, 6, 7,);
    test_scalar!(i64, 0i64, 1, 2, 3, 4, 5, 6, 7, 8,);
    test_scalar!(i64, 0i64, 1, 2, 3, 4, 5, 6, 7, 8, 9,);
    test_scalar!(i64, 0i64, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10,);
    test_scalar!(i64, 0i64, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11);
    test_scalar!(i64, 0i64, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12);
    test_scalar!(i64, 0i64, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13);
}

#[test]
fn scalar_u64() {
    test_scalar!(u64, 0u64);
    test_scalar!(u64, 0u64, 1,);
    test_scalar!(u64, 0u64, 1, 2,);
    test_scalar!(u64, 0u64, 1, 2, 3,);
    test_scalar!(u64, 0u64, 1, 2, 3, 4,);
    test_scalar!(u64, 0u64, 1, 2, 3, 4, 5,);
    test_scalar!(u64, 0u64, 1, 2, 3, 4, 5, 6,);
    test_scalar!(u64, 0u64, 1, 2, 3, 4, 5, 6, 7,);
    test_scalar!(u64, 0u64, 1, 2, 3, 4, 5, 6, 7, 8,);
    test_scalar!(u64, 0u64, 1, 2, 3, 4, 5, 6, 7, 8, 9,);
    test_scalar!(u64, 0u64, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10,);
    test_scalar!(u64, 0u64, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11);
    test_scalar!(u64, 0u64, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12);
    test_scalar!(u64, 0u64, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13);
}

#[test]
fn scalar_isize() {
    test_scalar!(isize, 0isize);
    test_scalar!(isize, 0isize, 1,);
    test_scalar!(isize, 0isize, 1, 2,);
    test_scalar!(isize, 0isize, 1, 2, 3,);
    test_scalar!(isize, 0isize, 1, 2, 3, 4,);
    test_scalar!(isize, 0isize, 1, 2, 3, 4, 5,);
    test_scalar!(isize, 0isize, 1, 2, 3, 4, 5, 6,);
    test_scalar!(isize, 0isize, 1, 2, 3, 4, 5, 6, 7,);
    test_scalar!(isize, 0isize, 1, 2, 3, 4, 5, 6, 7, 8,);
    test_scalar!(isize, 0isize, 1, 2, 3, 4, 5, 6, 7, 8, 9,);
    test_scalar!(isize, 0isize, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10,);
    test_scalar!(isize, 0isize, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11);
    test_scalar!(isize, 0isize, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12);
    test_scalar!(isize, 0isize, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13);
}

#[test]
fn scalar_usize() {
    test_scalar!(usize, 0usize);
    test_scalar!(usize, 0usize, 1,);
    test_scalar!(usize, 0usize, 1, 2,);
    test_scalar!(usize, 0usize, 1, 2, 3,);
    test_scalar!(usize, 0usize, 1, 2, 3, 4,);
    test_scalar!(usize, 0usize, 1, 2, 3, 4, 5,);
    test_scalar!(usize, 0usize, 1, 2, 3, 4, 5, 6,);
    test_scalar!(usize, 0usize, 1, 2, 3, 4, 5, 6, 7,);
    test_scalar!(usize, 0usize, 1, 2, 3, 4, 5, 6, 7, 8,);
    test_scalar!(usize, 0usize, 1, 2, 3, 4, 5, 6, 7, 8, 9,);
    test_scalar!(usize, 0usize, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10,);
    test_scalar!(usize, 0usize, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11);
    test_scalar!(usize, 0usize, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12);
    test_scalar!(usize, 0usize, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13);
}

#[test]
fn scalar_extra() {
    // test sparse case
    test_scalar!(u64, 0u64);
    test_scalar!(u64, 0u64, 1,);
    test_scalar!(u64, 0u64, 2,);
    test_scalar!(u64, 0u64, 2, 3,);
    test_scalar!(u64, 0u64, 2, 3, 4,);
    test_scalar!(u64, 0u64, 2, 3, 4, 5,);
    test_scalar!(u64, 0u64, 2, 3, 4, 5, 6,);
    test_scalar!(u64, 0u64, 2, 3, 4, 5, 6, 7,);
    test_scalar!(u64, 0u64, 2, 3, 4, 5, 6, 7, 8,);
    test_scalar!(u64, 0u64, 2, 3, 4, 5, 6, 7, 8, 9,);
    test_scalar!(u64, 0u64, 2, 3, 4, 5, 6, 7, 8, 9, 10,);
    test_scalar!(u64, 0u64, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11);
    test_scalar!(u64, 0u64, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12);
    test_scalar!(u64, 0u64, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13);

    // test defaulting to hash table
    test_scalar!(u64, 0u64, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 1500);

    // test default to scan
    test_scalar!(u64, 0u64, 1500);
}

#[test]
fn duplicates() {
    let map = fz_scalar_map!({0: 1, 0: 2});
    assert_eq!(&2, map.get(&0).unwrap());

    let map = fz_scalar_map!({0: 1, 1: 2, 0: 2});
    assert_eq!(&2, map.get(&0).unwrap());

    let map = fz_scalar_map!({0: 1, 1: 2, 2: 3, 0: 2});
    assert_eq!(&2, map.get(&0).unwrap());

    let map = fz_scalar_map!({0: 1, 1: 2, 2: 3, 3: 4, 0: 2});
    assert_eq!(&2, map.get(&0).unwrap());

    let map = fz_scalar_map!({0: 1, 1: 2, 2: 3, 3: 4, 4: 5, 0: 2});
    assert_eq!(&2, map.get(&0).unwrap());
}
