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
                    $arg.to_string(),
                )*
            ];

            _ = fz_string_set_macro(quote!(v)).unwrap();

            let _s1 = fz_string_set!(v);

            let v = vec![
                $(
                    $arg,
                )*
            ];

            let mut s2 = StdBTreeSet::new();
            for x in v.into_iter() {
                s2.insert(x);
            }

            _ = fz_string_set_macro(quote!(static _S3: Foo< $type > = {
                $(
                    $arg,
                )*
            })).unwrap();

            fz_string_set!(static _S3: Foo< $type > = {
                $(
                    $arg,
                )*
            });

            _ = fz_string_set_macro(quote!(let s4: Bar< $type > = {
                $(
                    $arg,
                )*
            })).unwrap();

            fz_string_set!(let s4: Bar< $type > = {
                $(
                    $arg,
                )*
            });

            _ = fz_string_set_macro(quote!(let mut s5: Baz< $type > = {
                $(
                    $arg,
                )*
            })).unwrap();

            fz_string_set!(let mut s5: Baz< $type > = {
                $(
                    $arg,
                )*
            });

            // assert_eq!(s0, s1);
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
                    ($arg.to_string(), 42),
                )*
            ];

            _ = fz_string_map_macro(quote!(v)).unwrap();

            let _m1 = fz_string_map!(v);

            let v = vec![
                $(
                    ($arg, 42),
                )*
            ];

            let mut m2 = StdBTreeMap::new();
            for x in v.into_iter() {
                m2.insert(x.0, x.1);
            }

            _ = fz_string_map_macro(quote!(static _M3: Foo< $type, i32 > = {
                $(
                    $arg: 42,
                )*
            })).unwrap();

            fz_string_map!(static _M3: Foo< $type, i32 > = {
                $(
                    $arg: 42,
                )*
            });

            _ = fz_string_map_macro(quote!(let m4: Bar< $type, i32 > = {
                $(
                    $arg: 42,
                )*
            })).unwrap();

            fz_string_map!(let m4: Bar< $type, i32 > = {
                $(
                    $arg: 42,
                )*
            });

            _ = fz_string_map_macro(quote!(let mut m5: Baz< $type, i32 > = {
                $(
                    $arg: 42,
                )*
            })).unwrap();

            fz_string_map!(let mut m5: Baz< $type, i32 > = {
                $(
                    $arg: 42,
                )*
            });

            // assert_eq!(m0, m1);
            assert_eq!(m0, m2);
            // assert_eq!(m0, M3);
            assert_eq!(m0, m4);
            assert_eq!(m0, m5);
        }
    }
}

#[test]
fn string() {
    test_string!(&str, "0");
    test_string!(&str, "0", "1");
    test_string!(&str, "0", "1", "2");
    test_string!(&str, "0", "1", "2", "3");
    test_string!(&str, "0", "1", "2", "3", "4");
    test_string!(&str, "0", "1", "2", "3", "4", "5");
    test_string!(&str, "0", "1", "2", "3", "4", "5", "6");
    test_string!(&str, "0", "1", "2", "3", "4", "5", "6", "7");
    test_string!(&str, "0", "1", "2", "3", "4", "5", "6", "7", "8");
    test_string!(&str, "0", "1", "2", "3", "4", "5", "6", "7", "8", "9");
    test_string!(&str, "0", "1", "2", "3", "4", "5", "6", "7", "8", "9", "10");
    test_string!(&str, "0", "1", "2", "3", "4", "5", "6", "7", "8", "9", "10", "11");
    test_string!(&str, "0", "1", "2", "3", "4", "5", "6", "7", "8", "9", "10", "11", "12");
    test_string!(&str, "0", "1", "2", "3", "4", "5", "6", "7", "8", "9", "10", "11", "12", "13");

    // test duplicate logic
    test_string!(&str, "0", "1", "0", "0");

    test_string!(
        &str,
        "ColorRed",
        "ColorGreen",
        "ColorBlue",
        "ColorYellow",
        "ColorCyan",
        "ColorMagenta"
    );

    test_string!(
        &str,
        "RedColor",
        "GreenColor",
        "BlueColor",
        "YellowColor",
        "CyanColor",
        "MagentaColor"
    );

    test_string!(
        &str,
        "ColorRed1111",
        "ColorGreen22",
        "ColorBlue333",
        "ColorYellow4",
        "ColorCyan555",
        "ColorMagenta"
    );

    test_string!(&str, "XXA", "XXB", "XXC", "XXD", "XXE", "XXF", "XXG", "XXH", "XXHI");
}

/*
#[test]
fn inline_hash_table() {
    let s = fz_string_set!({"a", "b", "c", "d", "e", "f", "g", "h", "i", "j", "k"});

    assert!(s.contains("a"));
    assert!(!s.contains("z"));
    assert_eq!(11, s.len());
}
*/
