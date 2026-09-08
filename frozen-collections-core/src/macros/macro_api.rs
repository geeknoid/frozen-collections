use crate::emit::CollectionEmitter;
use crate::macros::parsing::map::Map;
use crate::macros::parsing::set::Set;
use crate::macros::processor::{MacroKind, process};
use proc_macro2::TokenStream;
use syn::parse2;

#[cfg(not(feature = "std"))]
use alloc::string::ToString;

/// Implementation logic for the `fz_hash_map!` macro.
///
/// # Errors
///
/// Bad things happen to bad input
pub fn fz_hash_map_macro(args: TokenStream) -> syn::Result<TokenStream> {
    fz_map_macro(args, MacroKind::Hashed)
}

/// Implementation logic for the `fz_ordered_map!` macro.
///
/// # Errors
///
/// Bad things happen to bad input
pub fn fz_ordered_map_macro(args: TokenStream) -> syn::Result<TokenStream> {
    fz_map_macro(args, MacroKind::Ordered)
}

/// Implementation logic for the `fz_string_map!` macro.
///
/// # Errors
///
/// Bad things happen to bad input
pub fn fz_string_map_macro(args: TokenStream) -> syn::Result<TokenStream> {
    fz_map_macro(args, MacroKind::String)
}

/// Implementation logic for the `fz_scalar_map!` macro.
///
/// # Errors
///
/// Bad things happen to bad input
pub fn fz_scalar_map_macro(args: TokenStream) -> syn::Result<TokenStream> {
    fz_map_macro(args, MacroKind::Scalar)
}

/// Implementation logic for the `fz_hash_set!` macro.
///
/// # Errors
///
/// Bad things happen to bad input
pub fn fz_hash_set_macro(args: TokenStream) -> syn::Result<TokenStream> {
    fz_set_macro(args, MacroKind::Hashed)
}

/// Implementation logic for the `fz_ordered_set!` macro.
///
/// # Errors
///
/// Bad things happen to bad input
pub fn fz_ordered_set_macro(args: TokenStream) -> syn::Result<TokenStream> {
    fz_set_macro(args, MacroKind::Ordered)
}

/// Implementation logic for the `fz_string_set!` macro.
///
/// # Errors
///
/// Bad things happen to bad input
pub fn fz_string_set_macro(args: TokenStream) -> syn::Result<TokenStream> {
    fz_set_macro(args, MacroKind::String)
}

/// Implementation logic for the `fz_scalar_set!` macro.
///
/// # Errors
///
/// Bad things happen to bad input
pub fn fz_scalar_set_macro(args: TokenStream) -> syn::Result<TokenStream> {
    fz_set_macro(args, MacroKind::Scalar)
}

fn fz_map_macro(args: TokenStream, macro_kind: MacroKind) -> syn::Result<TokenStream> {
    let input = parse2::<Map>(args)?;

    match input {
        Map::Short(map) => {
            let emitter = CollectionEmitter::new_with_inferred_types();
            process(map.payload, emitter, macro_kind)
        }
        Map::Long(map) => {
            let emitter = CollectionEmitter::new(&map.key_type)
                .value_type(&map.value_type)
                .alias_name(map.type_name.to_string().as_str())
                .symbol_name(map.var_name.to_string().as_str())
                .mutable(map.is_mutable)
                .static_instance(map.is_static)
                .visibility(map.visibility);

            process(map.payload, emitter, macro_kind)
        }
    }
}

fn fz_set_macro(args: TokenStream, macro_kind: MacroKind) -> syn::Result<TokenStream> {
    let input = parse2::<Set>(args)?;

    match input {
        Set::Short(set) => {
            let emitter = CollectionEmitter::new_with_inferred_key_type();
            process(set.payload, emitter, macro_kind)
        }
        Set::Long(set) => {
            let emitter = CollectionEmitter::new(&set.value_type)
                .alias_name(set.type_name.to_string().as_str())
                .symbol_name(set.var_name.to_string().as_str())
                .mutable(set.is_mutable)
                .static_instance(set.is_static)
                .visibility(set.visibility);

            process(set.payload, emitter, macro_kind)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use proc_macro2::Delimiter::Brace;
    use proc_macro2::{Group, TokenTree};
    use quote::{ToTokens, TokenStreamExt, quote};

    #[test]
    fn missing_static_in_map() {
        let r = fz_scalar_map_macro(quote!(
            pub Bar<i32, i43>, { 1 : 2, 2 : 3 }
        ));

        assert_eq!("expected `static`", r.unwrap_err().to_string());
    }

    #[test]
    fn no_entries() {
        let r = fz_scalar_set_macro(quote!({}));

        assert_eq!("no collection entries supplied", r.unwrap_err().to_string());
    }

    #[test]
    fn invalid_suffix() {
        let r = fz_scalar_set_macro(quote!(
            { 123iXXX, 234iXXX }
        ));

        assert_eq!("unknown suffix iXXX for scalar value", r.unwrap_err().to_string());
    }

    #[test]
    fn incompatible_literal_types() {
        let r = fz_scalar_set_macro(quote!(
            { 1i8, 2i16 }
        ));

        assert_eq!("incompatible scalar literal type", r.unwrap_err().to_string());
    }

    #[test]
    fn invalid_literal() {
        let r = fz_scalar_set_macro(quote!(
            { 123.0, 234.0 }
        ));

        assert_eq!("invalid literal, expecting a scalar or string value", r.unwrap_err().to_string());

        let r = fz_string_set_macro(quote!(
            { 1, 2 }
        ));

        assert_eq!("string macro cannot contain scalar keys", r.unwrap_err().to_string());

        let r = fz_scalar_set_macro(quote!(
            { "1", "2" }
        ));

        assert_eq!("scalar macro cannot contain string keys", r.unwrap_err().to_string());
    }

    #[test]
    fn missing_static_in_set() {
        let r = fz_scalar_set_macro(quote!(
            pub Bar<i32>, { 1, 2 }
        ));

        assert_eq!("expected `static`", r.unwrap_err().to_string());
    }

    #[test]
    fn magnitude() {
        test_magnitude(255, "SmallCollection");
        test_magnitude(256, "MediumCollection");
        test_magnitude(65535, "MediumCollection");
        test_magnitude(65536, "LargeCollection");
    }

    fn test_magnitude(count: i32, expected: &str) {
        let mut s = TokenStream::new();

        for i in 1..count {
            s.append_all(quote!(#i,));
        }

        s.append_all(quote!(12322225));
        let s = TokenTree::Group(Group::new(Brace, s)).to_token_stream();

        let r = fz_scalar_set_macro(s).unwrap();
        assert!(r.to_string().contains(expected), "{}", expected);
    }

    #[test]
    fn test_selected_string_set_implementation_types() {
        fn check_impl(expected: &str, ts: TokenStream) {
            let r = fz_string_set_macro(ts).unwrap().to_string();
            assert!(r.contains(expected), "{r} doesn't contain {expected}");
        }

        check_impl(":: InlineScanSet", quote!({ "1", "2", "3", }));
        check_impl(":: InlineHashSet", quote!({ "1", "2", "3", "4" }));

        check_impl(":: BridgeHasher", quote!({ "1", "2", "3", "4", "5", "6", "7" }));

        check_impl(
            ":: LengthHasher",
            quote!({ "1", "22", "333", "4444", "55555", "666666", "7777777" }),
        );

        check_impl(
            ":: InlineLeftRangeHasher",
            quote!({ "1111", "1112", "1113", "1114", "1115", "1116", "1117" }),
        );

        check_impl(":: InlineScanSet", quote!({ x, "2", "3", }));
        check_impl(":: FzStringSet", quote!({ x, "2", "3", "4" }));

        check_impl(":: FzStringSet", quote!({ x, y, z, a, b, c, d }));

        // When compact_str is enabled, expression-key string collections use CompactString
        #[cfg(feature = "compact_str")]
        {
            fn check_compact(ts: TokenStream) {
                let r = fz_string_set_macro(ts).unwrap().to_string();
                assert!(r.contains("CompactString"), "{r} doesn't contain CompactString");
            }

            // Small collection (< 4): InlineScanSet with CompactString key type + wrapped key_exprs
            check_compact(quote!({ x, "2", "3", }));
            // Large collection (>= 4): FzStringSet with CompactString, uses new_compact
            check_compact(quote!({ x, "2", "3", "4" }));
            check_compact(quote!({ x, y, z, a, b, c, d }));

            // Verify new_compact is used for large collections
            let r = fz_string_set_macro(quote!({ x, "2", "3", "4" })).unwrap().to_string();
            assert!(r.contains("new_compact"), "large expression-key set should use new_compact: {r}");

            // Verify CompactString::from wrapping for small collections
            let r = fz_string_set_macro(quote!({ x, "2", "3", })).unwrap().to_string();
            assert!(r.contains("CompactString :: from"), "small expression-key set should wrap key_exprs: {r}");
        }
    }

    #[cfg(feature = "compact_str")]
    #[test]
    fn test_selected_string_map_compact_str() {
        // Verify CompactString is used for expression-key string MAPS (not just sets)
        fn check_compact_map(ts: TokenStream) {
            let r = fz_string_map_macro(ts).unwrap().to_string();
            assert!(r.contains("CompactString"), "{r} doesn't contain CompactString");
        }

        // Small map (< 4): InlineScanMap with CompactString
        check_compact_map(quote!({ x: 1, "2": 2, "3": 3 }));
        // Large map (>= 4): FzStringMap with CompactString + new_compact
        check_compact_map(quote!({ x: 1, "2": 2, "3": 3, "4": 4 }));

        let r = fz_string_map_macro(quote!({ x: 1, "2": 2, "3": 3, "4": 4 })).unwrap().to_string();
        assert!(r.contains("new_compact"), "large expression-key map should use new_compact: {r}");

        let r = fz_string_map_macro(quote!({ x: 1, "2": 2, "3": 3 })).unwrap().to_string();
        assert!(r.contains("CompactString :: from"), "small expression-key map should wrap key_exprs: {r}");
    }

    #[cfg(feature = "compact_str")]
    #[test]
    fn test_literal_string_keys_no_compact_str() {
        // All-literal string keys use const_keys=true, so CompactString is NOT used
        let r = fz_string_set_macro(quote!({ "a", "b", "c", "d" })).unwrap().to_string();
        assert!(!r.contains("CompactString"), "literal string keys should not use CompactString: {r}");

        let r = fz_string_map_macro(quote!({ "a": 1, "b": 2, "c": 3, "d": 4 })).unwrap().to_string();
        assert!(!r.contains("CompactString"), "literal string map keys should not use CompactString: {r}");
    }

    #[test]
    fn test_selected_scalar_set_implementation_types() {
        fn check_impl(expected: &str, ts: TokenStream) {
            let r = fz_scalar_set_macro(ts).unwrap().to_string();
            assert!(r.contains(expected), "{r} doesn't contain {expected}");
        }

        check_impl(":: InlineDenseScalarLookupSet", quote!({ 1, 2, 3, }));
        check_impl(":: InlineSparseScalarLookupSet", quote!({ 1, 2, 3, 4, 6 }));
        check_impl(":: InlineScanSet", quote!({ 1, 2, 3, 4, 5, 60000, 70000 }));
        check_impl(":: InlineHashSet", quote!({ 1, 2, 3, 4, 5, 60000, 70000, 80000 }));

        check_impl(":: InlineScanSet", quote!({ x, 2, 3, 4, 5, 6, 7 }));
        check_impl(":: FzScalarSet", quote!({ x, 2, 3, 4, 5, 6, 7, 8 }));
    }

    #[test]
    fn test_selected_ordered_set_implementation_types() {
        fn check_impl(expected: &str, ts: TokenStream) {
            let r = fz_ordered_set_macro(ts).unwrap().to_string();
            assert!(r.contains(expected), "{r} doesn't contain {expected}");
        }

        check_impl(":: InlineDenseScalarLookupSet", quote!({ 1, 2, 3, }));
        check_impl(":: InlineSparseScalarLookupSet", quote!({ 1, 2, 3, 4, 6 }));
        check_impl(":: InlineScanSet", quote!({ 1, 2, 3, 4, 5, 60000, 70000 }));
        check_impl(":: InlineHashSet", quote!({ 1, 2, 3, 4, 5, 60000, 70000, 80000 }));

        check_impl(":: InlineScanSet", quote!({ "1", "2", "3", }));
        check_impl(":: InlineHashSet", quote!({ "1", "2", "3", "4" }));

        check_impl(":: InlineScanSet", quote!({ Foo(1), Foo(2), Foo(3),}));
        check_impl(":: InlineEytzingerSearchSet", quote!({ Foo(1), Foo(2), Foo(3), Foo(4) }));

        check_impl(":: InlineScanSet", quote!({ x, 2, 3, 4, 5, 6, 7 }));
        check_impl(":: FzScalarSet", quote!({ x, 2, 3, 4, 5, 6, 7, 8 }));

        check_impl(":: InlineScanSet", quote!({ x, "2", "3", }));
        check_impl(":: FzStringSet", quote!({ x, "2", "3", "4" }));
    }

    #[test]
    fn test_selected_hash_set_implementation_types() {
        fn check_impl(expected: &str, ts: TokenStream) {
            let r = fz_hash_set_macro(ts).unwrap().to_string();
            assert!(r.contains(expected), "{r} doesn't contain {expected}");
        }

        check_impl(":: InlineDenseScalarLookupSet", quote!({ 1, 2, 3, }));
        check_impl(":: InlineSparseScalarLookupSet", quote!({ 1, 2, 3, 4, 6 }));
        check_impl(":: InlineScanSet", quote!({ 1, 2, 3, 4, 5, 60000, 70000 }));
        check_impl(":: InlineHashSet", quote!({ 1, 2, 3, 4, 5, 60000, 70000, 80000 }));

        check_impl(":: InlineScanSet", quote!({ "1", "2", "3", }));
        check_impl(":: InlineHashSet", quote!({ "1", "2", "3", "4" }));

        check_impl(":: InlineScanSet", quote!({ Foo(1), Foo(2), Foo(3), }));
        check_impl(":: FzHashSet", quote!({ Foo(1), Foo(2), Foo(3), Foo(4) }));

        check_impl(":: InlineScanSet", quote!({ x, 2, 3, 4, 5, 6, 7 }));
        check_impl(":: FzScalarSet", quote!({ x, 2, 3, 4, 5, 6, 7, 8 }));

        check_impl(":: InlineScanSet", quote!({ x, "2", "3", }));
        check_impl(":: FzStringSet", quote!({ x, "2", "3", "4" }));
    }

    #[test]
    fn test_scalar_suffixes() {
        let r = fz_scalar_set_macro(quote!({ 1i8, 2, 3, 4, 5, 6 })).unwrap().to_string();
        assert!(r.contains("1i8"));

        let r = fz_scalar_set_macro(quote!({ 1, 2, 3, 4, 5, 6 })).unwrap().to_string();
        assert!(!r.contains("1i8"));
        assert!(r.contains("1i32"));
    }
}
