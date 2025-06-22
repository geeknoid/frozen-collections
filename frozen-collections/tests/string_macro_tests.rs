#![expect(missing_docs, reason = "Tests")]

use frozen_collections::*;
use frozen_collections_core::macros::{fz_string_map_macro, fz_string_set_macro};
use quote::quote;
use std::collections::BTreeMap as StdBTreeMap;
use std::collections::BTreeSet as StdBTreeSet;

macro_rules! test_string {
    ( $type:ty, $( $arg:expr ),* $(,)?) => {
        {
            _ = fz_string_set_macro(quote!({
                $(
                    $arg,
                )*
            })).unwrap();

            let s0 = fz_string_set!({
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

            _ = fz_string_set_macro(quote!(static _S3: Foo< $type >, {
                $(
                    $arg,
                )*
            })).unwrap();

            fz_string_set!(static _S3: Foo< $type >, {
                $(
                    $arg,
                )*
            });

            _ = fz_string_set_macro(quote!(let s4: Bar< $type >, {
                $(
                    $arg,
                )*
            })).unwrap();

            fz_string_set!(let s4: Bar< $type >, {
                $(
                    $arg,
                )*
            });

            _ = fz_string_set_macro(quote!(let mut s5: Baz< $type >, {
                $(
                    $arg,
                )*
            })).unwrap();

            fz_string_set!(let mut s5: Baz< $type >, {
                $(
                    $arg,
                )*
            });

            assert_eq!(s0, s2);
            // assert_eq!(s0, S3);
            assert_eq!(s0, s4);
            assert_eq!(s0, s5);
        }

        {
            _ = fz_string_map_macro(quote!({
                $(
                    $arg: 42,
                )*
            })).unwrap();

            let m0 = fz_string_map!({
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

            _ = fz_string_map_macro(quote!(static _M3: Foo< $type, i32 >, {
                $(
                    $arg: 42,
                )*
            })).unwrap();

            fz_string_map!(static _M3: Foo< $type, i32 >, {
                $(
                    $arg: 42,
                )*
            });

            _ = fz_string_map_macro(quote!(let m4: Bar< $type, i32 >, {
                $(
                    $arg: 42,
                )*
            })).unwrap();

            fz_string_map!(let m4: Bar< $type, i32 >, {
                $(
                    $arg: 42,
                )*
            });

            _ = fz_string_map_macro(quote!(let mut m5: Baz< $type, i32 >, {
                $(
                    $arg: 42,
                )*
            })).unwrap();

            fz_string_map!(let mut m5: Baz< $type, i32 >, {
                $(
                    $arg: 42,
                )*
            });

            assert_eq!(m0, m2);
            // assert_eq!(m0, M3);
            assert_eq!(m0, m4);
            assert_eq!(m0, m5);
        }
    }
}

#[test]
fn string() {
    test_string!(&'static str, "0");
    test_string!(&'static str, "0", "1");
    test_string!(&'static str, "0", "1", "2");
    test_string!(&'static str, "0", "1", "2", "3");
    test_string!(&'static str, "0", "1", "2", "3", "4");
    test_string!(&'static str, "0", "1", "2", "3", "4", "5");
    test_string!(&'static str, "0", "1", "2", "3", "4", "5", "6");
    test_string!(&'static str, "0", "1", "2", "3", "4", "5", "6", "7");
    test_string!(&'static str, "0", "1", "2", "3", "4", "5", "6", "7", "8");
    test_string!(&'static str, "0", "1", "2", "3", "4", "5", "6", "7", "8", "9");
    test_string!(&'static str, "0", "1", "2", "3", "4", "5", "6", "7", "8", "9", "10");
    test_string!(&'static str, "0", "1", "2", "3", "4", "5", "6", "7", "8", "9", "10", "11");
    test_string!(&'static str, "0", "1", "2", "3", "4", "5", "6", "7", "8", "9", "10", "11", "12");
    test_string!(
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

    // test duplicate logic
    test_string!(&'static str, "0", "1", "0", "0");

    test_string!(
        &'static str,
        "ColorRed",
        "ColorGreen",
        "ColorBlue",
        "ColorYellow",
        "ColorCyan",
        "ColorMagenta"
    );

    test_string!(
        &'static str,
        "RedColor",
        "GreenColor",
        "BlueColor",
        "YellowColor",
        "CyanColor",
        "MagentaColor"
    );

    test_string!(
        &'static str,
        "ColorRed1111",
        "ColorGreen22",
        "ColorBlue333",
        "ColorYellow4",
        "ColorCyan555",
        "ColorMagenta"
    );

    test_string!(&'static str, "XXA", "XXB", "XXC", "XXD", "XXE", "XXF", "XXG", "XXH", "XXHI");
}

#[test]
fn non_literal_key_string_map() {
    let s0 = "Zero";
    let s1 = "One";
    let s2 = "Two";
    let s3 = "Three";
    let m = fz_string_map!({s0: 2, s1: 3, s2: 4, s3: 3});
    assert_eq!(4, m.len());
    assert!(m.contains_key(s0));
    assert!(m.contains_key(s1));
    assert!(!m.contains_key("Foo"));

    assert!(fz_string_map_macro(quote!({s0: 2, s1: 3, s2: 4, s3: 3})).is_ok());
}
