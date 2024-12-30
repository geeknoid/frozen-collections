use frozen_collections::*;
use frozen_collections_core::macros::{fz_hash_map_macro, fz_hash_set_macro};
use hashbrown::HashMap as HashbrownMap;
use hashbrown::HashSet as HashbrownSet;
use quote::quote;

#[derive(Hash, Clone, Copy, Debug, PartialEq, Eq, Ord, PartialOrd)]
struct Person {
    name: &'static str,
    age: i32,
}

macro_rules! test_hash {
    ( $type:ty, $( $arg:expr ),* $(,)?) => {
        {
            _ = fz_hash_set_macro(quote!({
                $(
                    $arg,
                )*
                })).unwrap();

            let s0 = fz_hash_set!({
                $(
                    $arg,
                )*
            });

            let v = vec![
                $(
                    $arg,
                )*
            ];

            let mut s2 = HashbrownSet::new();
            for x in v.into_iter() {
                s2.insert(x);
            }

            _ = fz_hash_set_macro(quote!(static _S3: Foo< $type >, {
                $(
                    $arg,
                )*
            })).unwrap();

            fz_hash_set!(static _S3: Foo< $type >, {
                $(
                    $arg,
                )*
            });

            _ = fz_hash_set_macro(quote!(let s4: Bar< $type >, {
                $(
                    $arg,
                )*
            })).unwrap();

            fz_hash_set!(let s4: Bar< $type >, {
                $(
                    $arg,
                )*
            });

            assert_eq!(s0, s2);
            // assert_eq!(s0, S3);
            assert_eq!(s0, s4);
        }

        {
            _ = fz_hash_map_macro(quote!({
                $(
                    $arg:42,
                )*
            })).unwrap();

            let m0 = fz_hash_map!({
                $(
                    $arg: 42,
                )*
            });

            let v = vec![
                $(
                    ($arg, 42),
                )*
            ];

            let mut m2 = HashbrownMap::new();
            for x in v.into_iter() {
                m2.insert(x.0, x.1);
            }

            _ = fz_hash_map_macro(quote!(static _M3: Foo< $type, i32 >, {
                $(
                    $arg: 42,
                )*
            })).unwrap();

            fz_hash_map!(static _M3: Foo< $type, i32 >, {
                $(
                    $arg: 42,
                )*
            });

            _ = fz_hash_map_macro(quote!(let m4: Bar< $type, i32 >, {
                $(
                    $arg: 42,
                )*
            })).unwrap();

            fz_hash_map!(let m4: Bar< $type, i32 >, {
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
fn hash_complex() {
    test_hash!(Person, Person { name: "A", age: 1 },);
    test_hash!(
        Person,
        Person { name: "A", age: 1 },
        Person { name: "B", age: 2 },
    );
    test_hash!(
        Person,
        Person { name: "A", age: 1 },
        Person { name: "B", age: 2 },
        Person { name: "C", age: 3 },
    );
    test_hash!(
        Person,
        Person { name: "A", age: 1 },
        Person { name: "B", age: 2 },
        Person { name: "C", age: 3 },
        Person { name: "D", age: 4 },
    );
    test_hash!(
        Person,
        Person { name: "A", age: 1 },
        Person { name: "B", age: 2 },
        Person { name: "C", age: 3 },
        Person { name: "D", age: 4 },
        Person { name: "E", age: 5 },
    );
    test_hash!(
        Person,
        Person { name: "A", age: 1 },
        Person { name: "B", age: 2 },
        Person { name: "C", age: 3 },
        Person { name: "D", age: 4 },
        Person { name: "E", age: 5 },
        Person { name: "F", age: 6 },
    );
    test_hash!(
        Person,
        Person { name: "A", age: 1 },
        Person { name: "B", age: 2 },
        Person { name: "C", age: 3 },
        Person { name: "D", age: 4 },
        Person { name: "E", age: 5 },
        Person { name: "F", age: 6 },
        Person { name: "G", age: 7 },
    );
    test_hash!(
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
    test_hash!(
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
    test_hash!(
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
    test_hash!(
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
    test_hash!(
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
    test_hash!(
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

    // test duplicate logic
    test_hash!(
        Person,
        Person { name: "A", age: 1 },
        Person { name: "B", age: 2 },
        Person { name: "A", age: 3 },
        Person { name: "A", age: 4 },
    );
}

#[test]
fn hash_i8() {
    test_hash!(i8, 0i8);
    test_hash!(i8, 0i8, 1,);
    test_hash!(i8, 0i8, 1, 2,);
    test_hash!(i8, 0i8, 1, 2, 3,);
    test_hash!(i8, 0i8, 1, 2, 3, 4,);
    test_hash!(i8, 0i8, 1, 2, 3, 4, 5,);
    test_hash!(i8, 0i8, 1, 2, 3, 4, 5, 6,);
    test_hash!(i8, 0i8, 1, 2, 3, 4, 5, 6, 7,);
    test_hash!(i8, 0i8, 1, 2, 3, 4, 5, 6, 7, 8,);
    test_hash!(i8, 0i8, 1, 2, 3, 4, 5, 6, 7, 8, 9,);
    test_hash!(i8, 0i8, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10,);
    test_hash!(i8, 0i8, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11);
    test_hash!(i8, 0i8, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12);
    test_hash!(i8, 0i8, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13);

    // test duplicate logic
    test_hash!(i8, 0i8, 1, 2, 1, 1);
}

#[test]
fn hash_u8() {
    test_hash!(u8, 0u8);
    test_hash!(u8, 0u8, 1,);
    test_hash!(u8, 0u8, 1, 2,);
    test_hash!(u8, 0u8, 1, 2, 3,);
    test_hash!(u8, 0u8, 1, 2, 3, 4,);
    test_hash!(u8, 0u8, 1, 2, 3, 4, 5,);
    test_hash!(u8, 0u8, 1, 2, 3, 4, 5, 6,);
    test_hash!(u8, 0u8, 1, 2, 3, 4, 5, 6, 7,);
    test_hash!(u8, 0u8, 1, 2, 3, 4, 5, 6, 7, 8,);
    test_hash!(u8, 0u8, 1, 2, 3, 4, 5, 6, 7, 8, 9,);
    test_hash!(u8, 0u8, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10,);
    test_hash!(u8, 0u8, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11);
    test_hash!(u8, 0u8, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12);
    test_hash!(u8, 0u8, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13);
}

#[test]
fn hash_i16() {
    test_hash!(i16, 0i16);
    test_hash!(i16, 0i16, 1,);
    test_hash!(i16, 0i16, 1, 2,);
    test_hash!(i16, 0i16, 1, 2, 3,);
    test_hash!(i16, 0i16, 1, 2, 3, 4,);
    test_hash!(i16, 0i16, 1, 2, 3, 4, 5,);
    test_hash!(i16, 0i16, 1, 2, 3, 4, 5, 6,);
    test_hash!(i16, 0i16, 1, 2, 3, 4, 5, 6, 7,);
    test_hash!(i16, 0i16, 1, 2, 3, 4, 5, 6, 7, 8,);
    test_hash!(i16, 0i16, 1, 2, 3, 4, 5, 6, 7, 8, 9,);
    test_hash!(i16, 0i16, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10,);
    test_hash!(i16, 0i16, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11);
    test_hash!(i16, 0i16, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12);
    test_hash!(i16, 0i16, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13);
}

#[test]
fn hash_u16() {
    test_hash!(u16, 0u16);
    test_hash!(u16, 0u16, 1,);
    test_hash!(u16, 0u16, 1, 2,);
    test_hash!(u16, 0u16, 1, 2, 3,);
    test_hash!(u16, 0u16, 1, 2, 3, 4,);
    test_hash!(u16, 0u16, 1, 2, 3, 4, 5,);
    test_hash!(u16, 0u16, 1, 2, 3, 4, 5, 6,);
    test_hash!(u16, 0u16, 1, 2, 3, 4, 5, 6, 7,);
    test_hash!(u16, 0u16, 1, 2, 3, 4, 5, 6, 7, 8,);
    test_hash!(u16, 0u16, 1, 2, 3, 4, 5, 6, 7, 8, 9,);
    test_hash!(u16, 0u16, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10,);
    test_hash!(u16, 0u16, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11);
    test_hash!(u16, 0u16, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12);
    test_hash!(u16, 0u16, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13);
}

#[test]
fn hash_i32() {
    test_hash!(i32, 0i32);
    test_hash!(i32, 0i32, 1,);
    test_hash!(i32, 0i32, 1, 2,);
    test_hash!(i32, 0i32, 1, 2, 3,);
    test_hash!(i32, 0i32, 1, 2, 3, 4,);
    test_hash!(i32, 0i32, 1, 2, 3, 4, 5,);
    test_hash!(i32, 0i32, 1, 2, 3, 4, 5, 6,);
    test_hash!(i32, 0i32, 1, 2, 3, 4, 5, 6, 7,);
    test_hash!(i32, 0i32, 1, 2, 3, 4, 5, 6, 7, 8,);
    test_hash!(i32, 0i32, 1, 2, 3, 4, 5, 6, 7, 8, 9,);
    test_hash!(i32, 0i32, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10,);
    test_hash!(i32, 0i32, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11);
    test_hash!(i32, 0i32, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12);
    test_hash!(i32, 0i32, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13);
}

#[test]
fn hash_u32() {
    test_hash!(u32, 0u32);
    test_hash!(u32, 0u32, 1,);
    test_hash!(u32, 0u32, 1, 2,);
    test_hash!(u32, 0u32, 1, 2, 3,);
    test_hash!(u32, 0u32, 1, 2, 3, 4,);
    test_hash!(u32, 0u32, 1, 2, 3, 4, 5,);
    test_hash!(u32, 0u32, 1, 2, 3, 4, 5, 6,);
    test_hash!(u32, 0u32, 1, 2, 3, 4, 5, 6, 7,);
    test_hash!(u32, 0u32, 1, 2, 3, 4, 5, 6, 7, 8,);
    test_hash!(u32, 0u32, 1, 2, 3, 4, 5, 6, 7, 8, 9,);
    test_hash!(u32, 0u32, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10,);
    test_hash!(u32, 0u32, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11);
    test_hash!(u32, 0u32, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12);
    test_hash!(u32, 0u32, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13);
}

#[test]
fn hash_i64() {
    test_hash!(i64, 0i64);
    test_hash!(i64, 0i64, 1,);
    test_hash!(i64, 0i64, 1, 2,);
    test_hash!(i64, 0i64, 1, 2, 3,);
    test_hash!(i64, 0i64, 1, 2, 3, 4,);
    test_hash!(i64, 0i64, 1, 2, 3, 4, 5,);
    test_hash!(i64, 0i64, 1, 2, 3, 4, 5, 6,);
    test_hash!(i64, 0i64, 1, 2, 3, 4, 5, 6, 7,);
    test_hash!(i64, 0i64, 1, 2, 3, 4, 5, 6, 7, 8,);
    test_hash!(i64, 0i64, 1, 2, 3, 4, 5, 6, 7, 8, 9,);
    test_hash!(i64, 0i64, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10,);
    test_hash!(i64, 0i64, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11);
    test_hash!(i64, 0i64, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12);
    test_hash!(i64, 0i64, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13);
}

#[test]
fn hash_u64() {
    test_hash!(u64, 0u64);
    test_hash!(u64, 0u64, 1,);
    test_hash!(u64, 0u64, 1, 2,);
    test_hash!(u64, 0u64, 1, 2, 3,);
    test_hash!(u64, 0u64, 1, 2, 3, 4,);
    test_hash!(u64, 0u64, 1, 2, 3, 4, 5,);
    test_hash!(u64, 0u64, 1, 2, 3, 4, 5, 6,);
    test_hash!(u64, 0u64, 1, 2, 3, 4, 5, 6, 7,);
    test_hash!(u64, 0u64, 1, 2, 3, 4, 5, 6, 7, 8,);
    test_hash!(u64, 0u64, 1, 2, 3, 4, 5, 6, 7, 8, 9,);
    test_hash!(u64, 0u64, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10,);
    test_hash!(u64, 0u64, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11);
    test_hash!(u64, 0u64, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12);
    test_hash!(u64, 0u64, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13);
}

#[test]
fn hash_isize() {
    test_hash!(isize, 0isize);
    test_hash!(isize, 0isize, 1,);
    test_hash!(isize, 0isize, 1, 2,);
    test_hash!(isize, 0isize, 1, 2, 3,);
    test_hash!(isize, 0isize, 1, 2, 3, 4,);
    test_hash!(isize, 0isize, 1, 2, 3, 4, 5,);
    test_hash!(isize, 0isize, 1, 2, 3, 4, 5, 6,);
    test_hash!(isize, 0isize, 1, 2, 3, 4, 5, 6, 7,);
    test_hash!(isize, 0isize, 1, 2, 3, 4, 5, 6, 7, 8,);
    test_hash!(isize, 0isize, 1, 2, 3, 4, 5, 6, 7, 8, 9,);
    test_hash!(isize, 0isize, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10,);
    test_hash!(isize, 0isize, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11);
    test_hash!(isize, 0isize, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12);
    test_hash!(isize, 0isize, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13);
}

#[test]
fn hash_usize() {
    test_hash!(usize, 0usize);
    test_hash!(usize, 0usize, 1,);
    test_hash!(usize, 0usize, 1, 2,);
    test_hash!(usize, 0usize, 1, 2, 3,);
    test_hash!(usize, 0usize, 1, 2, 3, 4,);
    test_hash!(usize, 0usize, 1, 2, 3, 4, 5,);
    test_hash!(usize, 0usize, 1, 2, 3, 4, 5, 6,);
    test_hash!(usize, 0usize, 1, 2, 3, 4, 5, 6, 7,);
    test_hash!(usize, 0usize, 1, 2, 3, 4, 5, 6, 7, 8,);
    test_hash!(usize, 0usize, 1, 2, 3, 4, 5, 6, 7, 8, 9,);
    test_hash!(usize, 0usize, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10,);
    test_hash!(usize, 0usize, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11);
    test_hash!(usize, 0usize, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12);
    test_hash!(usize, 0usize, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13);
}

#[test]
fn hash_string() {
    test_hash!(&'static str, "0");
    test_hash!(&'static str, "0", "1");
    test_hash!(&'static str, "0", "1", "2");
    test_hash!(&'static str, "0", "1", "2", "3");
    test_hash!(&'static str, "0", "1", "2", "3", "4");
    test_hash!(&'static str, "0", "1", "2", "3", "4", "5");
    test_hash!(&'static str, "0", "1", "2", "3", "4", "5", "6");
    test_hash!(&'static str, "0", "1", "2", "3", "4", "5", "6", "7");
    test_hash!(&'static str, "0", "1", "2", "3", "4", "5", "6", "7", "8");
    test_hash!(
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
        "9"
    );
    test_hash!(
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
        "10"
    );
    test_hash!(
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
        "11"
    );
    test_hash!(
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
        "12"
    );
    test_hash!(
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
