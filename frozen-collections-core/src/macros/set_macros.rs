use crate::macros::generator;
use crate::macros::generator::MacroKind;
use crate::macros::parsing::long_form_set::LongFormSet;
use crate::macros::parsing::set::Set;
use crate::macros::parsing::short_form_set::ShortFormSet;
use crate::utils::pick_compile_time_random_seeds;
use proc_macro2::TokenStream;
use quote::quote;
use syn::parse2;

/// Implementation logic for the `fz_hash_set!` macro.
///
/// # Errors
///
/// Bad things happen to bad input
pub fn fz_hash_set_macro(args: TokenStream) -> syn::Result<TokenStream> {
    fz_set_macro(args, pick_compile_time_random_seeds(), MacroKind::Hashed)
}

/// Implementation logic for the `fz_ordered_set!` macro.
///
/// # Errors
///
/// Bad things happen to bad input
pub fn fz_ordered_set_macro(args: TokenStream) -> syn::Result<TokenStream> {
    fz_set_macro(args, pick_compile_time_random_seeds(), MacroKind::Ordered)
}

/// Implementation logic for the `fz_string_set!` macro.
///
/// # Errors
///
/// Bad things happen to bad input
pub fn fz_string_set_macro(args: TokenStream) -> syn::Result<TokenStream> {
    fz_set_macro(args, pick_compile_time_random_seeds(), MacroKind::String)
}

/// Implementation logic for the `fz_scalar_set!` macro.
///
/// # Errors
///
/// Bad things happen to bad input
pub fn fz_scalar_set_macro(args: TokenStream) -> syn::Result<TokenStream> {
    fz_set_macro(args, pick_compile_time_random_seeds(), MacroKind::Scalar)
}

fn fz_set_macro(
    args: TokenStream,
    seeds: (u64, u64, u64, u64),
    macro_kind: MacroKind,
) -> syn::Result<TokenStream> {
    let input = parse2::<Set>(args)?;

    match input {
        Set::Short(set) => short_form_fz_set_macro(set, seeds, macro_kind),
        Set::Long(set) => long_form_fz_set_macro(set, seeds, macro_kind),
    }
}

fn short_form_fz_set_macro(
    set: ShortFormSet,
    seeds: (u64, u64, u64, u64),
    macro_kind: MacroKind,
) -> syn::Result<TokenStream> {
    Ok(generator::generate(set.payload, seeds, true, quote!(_), quote!(_), macro_kind)?.ctor)
}

fn long_form_fz_set_macro(
    set: LongFormSet,
    seeds: (u64, u64, u64, u64),
    macro_kind: MacroKind,
) -> syn::Result<TokenStream> {
    let value_type = set.value_type;

    let value_type = if set.value_type_amp {
        quote!(&'static #value_type)
    } else {
        quote!(#value_type)
    };

    let output = generator::generate(set.payload, seeds, true, value_type, quote!(_), macro_kind)?;

    let type_sig = output.type_sig;
    let ctor = output.ctor;
    let var_name = &set.var_name;
    let type_name = &set.type_name;
    let visibility = &set.visibility;

    if !set.is_static {
        let mutable = if set.is_mutable {
            quote!(mut)
        } else {
            quote!()
        };

        Ok(quote!(
            type #type_name = #type_sig;
            let #mutable #var_name: #type_name = #ctor;
        ))
    } else if output.constant {
        Ok(quote!(
            #visibility type #type_name = #type_sig;
            #visibility static #var_name: #type_name = #ctor;
        ))
    } else {
        Ok(quote!(
            #visibility type #type_name = #type_sig;
            #visibility static #var_name: std::sync::LazyLock<#type_name> = std::sync::LazyLock::new(|| { #ctor });
        ))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use alloc::string::ToString;
    use proc_macro2::Delimiter::Brace;
    use proc_macro2::{Group, TokenTree};
    use quote::{quote, ToTokens, TokenStreamExt};

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

        assert_eq!(
            "unknown suffix iXXX for scalar value",
            r.unwrap_err().to_string()
        );
    }

    #[test]
    fn incompatible_literal_types() {
        let r = fz_scalar_set_macro(quote!(
            { 1i8, 2i16 }
        ));

        assert_eq!(
            "incompatible scalar literal type",
            r.unwrap_err().to_string()
        );
    }

    #[test]
    fn invalid_literal() {
        let r = fz_scalar_set_macro(quote!(
            { 123.0, 234.0 }
        ));

        assert_eq!(
            "invalid literal, expecting a scalar or string value",
            r.unwrap_err().to_string()
        );

        let r = fz_string_set_macro(quote!(
            { 1, 2 }
        ));

        assert_eq!(
            "string macro cannot contain scalar keys",
            r.unwrap_err().to_string()
        );

        let r = fz_scalar_set_macro(quote!(
            { "1", "2" }
        ));

        assert_eq!(
            "scalar macro cannot contain string keys",
            r.unwrap_err().to_string()
        );
    }

    #[test]
    fn missing_static() {
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
        check_impl(":: InlineOrderedScanSet", quote!({ "1", "2", "3", "4" }));
        check_impl(
            ":: InlineOrderedScanSet",
            quote!({ "1", "2", "3", "4", "5", "6"}),
        );

        check_impl(
            ":: InlineHashSet",
            quote!({ "1", "2", "3", "4", "5", "6", "7" }),
        );

        check_impl(
            ":: BridgeHasher",
            quote!({ "1", "2", "3", "4", "5", "6", "7" }),
        );

        check_impl(
            ":: PassthroughHasher",
            quote!({ "1", "22", "333", "4444", "55555", "666666", "7777777" }),
        );

        check_impl(
            ":: InlineLeftRangeHasher",
            quote!({ "1111", "1112", "1113", "1114", "1115", "1116", "1117" }),
        );

        check_impl(":: ScanSet", quote!({ x, "2", "3", }));
        check_impl(":: FacadeStringSet", quote!({ x, "2", "3", "4" }));
        check_impl(":: FacadeStringSet", quote!({ x, "2", "3", "4", "5", "6"}));
        check_impl(
            ":: FacadeStringSet",
            quote!({ x, "2", "3", "4", "5", "6", "7" }),
        );

        check_impl(":: FacadeStringSet", quote!({ x, y, z, a, b, c, d }));
    }

    #[test]
    fn test_selected_scalar_set_implementation_types() {
        fn check_impl(expected: &str, ts: TokenStream) {
            let r = fz_scalar_set_macro(ts).unwrap().to_string();
            assert!(r.contains(expected), "{r} doesn't contain {expected}");
        }

        check_impl(":: InlineDenseScalarLookupSet", quote!({ 1, 2, 3, }));
        check_impl(":: InlineSparseScalarLookupSet", quote!({ 1, 2, 3, 4, 6 }));
        check_impl(":: InlineScanSet", quote!({ 1, 2, 10000 }));
        check_impl(":: InlineOrderedScanSet", quote!({ 1, 2, 3, 4, 5, 10000 }));
        check_impl(":: InlineHashSet", quote!({ 1, 2, 3, 4, 5, 6, 10000 }));

        check_impl(":: ScanSet", quote!({ x, 2, 3, }));
        check_impl(":: FacadeScalarSet", quote!({ x, 2, 3, 4 }));
        check_impl(":: FacadeScalarSet", quote!({ x, 2, 3, 4, 5, 6}));
        check_impl(":: FacadeScalarSet", quote!({ x, 2, 3, 4, 5, 6, 7 }));
    }

    #[test]
    fn test_selected_ordered_set_implementation_types() {
        fn check_impl(expected: &str, ts: TokenStream) {
            let r = fz_ordered_set_macro(ts).unwrap().to_string();
            assert!(r.contains(expected), "{r} doesn't contain {expected}");
        }

        check_impl(":: InlineDenseScalarLookupSet", quote!({ 1, 2, 3, }));
        check_impl(":: InlineSparseScalarLookupSet", quote!({ 1, 2, 3, 4, 6 }));
        check_impl(":: InlineScanSet", quote!({ 1, 2, 10000 }));
        check_impl(":: InlineOrderedScanSet", quote!({ 1, 2, 3, 4, 5, 10000 }));
        check_impl(":: InlineHashSet", quote!({ 1, 2, 3, 4, 5, 6, 10000 }));

        check_impl(":: InlineScanSet", quote!({ "1", "2", "3", }));
        check_impl(":: InlineOrderedScanSet", quote!({ "1", "2", "3", "4" }));
        check_impl(
            ":: InlineOrderedScanSet",
            quote!({ "1", "2", "3", "4", "5", "6"}),
        );
        check_impl(
            ":: InlineHashSet",
            quote!({ "1", "2", "3", "4", "5", "6", "7" }),
        );

        check_impl(":: ScanSet", quote!({ Foo(1), Foo(2), Foo(3), }));
        check_impl(
            ":: OrderedScanSet",
            quote!({ Foo(1), Foo(2), Foo(3), Foo(4) }),
        );
        check_impl(
            ":: OrderedScanSet",
            quote!({ Foo(1), Foo(2), Foo(3), Foo(4), Foo(5), Foo(6)}),
        );
        check_impl(
            ":: BinarySearchSet",
            quote!({ Foo(1), Foo(2), Foo(3), Foo(4), Foo(5), Foo(6), Foo(7) }),
        );

        check_impl(
            ":: EytzingerSearchSet",
            quote!({ Foo(0), Foo(1), Foo(2), Foo(3), Foo(4), Foo(5), Foo(6), Foo(7), Foo(8), Foo(9),
                 Foo(10), Foo(11), Foo(12), Foo(13), Foo(14), Foo(15), Foo(16), Foo(17), Foo(18), Foo(19),
                 Foo(20), Foo(21), Foo(22), Foo(23), Foo(24), Foo(25), Foo(26), Foo(27), Foo(28), Foo(29),
                 Foo(30), Foo(31), Foo(32), Foo(33), Foo(34), Foo(35), Foo(36), Foo(37), Foo(38), Foo(39),
                 Foo(40), Foo(41), Foo(42), Foo(43), Foo(44), Foo(45), Foo(46), Foo(47), Foo(48), Foo(49),
                 Foo(50), Foo(51), Foo(52), Foo(53), Foo(54), Foo(55), Foo(56), Foo(57), Foo(58), Foo(59),
                 Foo(60), Foo(61), Foo(62), Foo(63), Foo(64), Foo(65), Foo(66), Foo(67), Foo(68), Foo(69),
            }),
        );

        check_impl(":: ScanSet", quote!({ x, 2, 3, }));
        check_impl(":: FacadeScalarSet", quote!({ x, 2, 3, 4 }));
        check_impl(":: FacadeScalarSet", quote!({ x, 2, 3, 4, 5, 6}));
        check_impl(":: FacadeScalarSet", quote!({ x, 2, 3, 4, 5, 6, 7 }));

        check_impl(":: ScanSet", quote!({ x, "2", "3", }));
        check_impl(":: FacadeStringSet", quote!({ x, "2", "3", "4" }));
        check_impl(":: FacadeStringSet", quote!({ x, "2", "3", "4", "5", "6"}));
        check_impl(
            ":: FacadeStringSet",
            quote!({ x, "2", "3", "4", "5", "6", "7" }),
        );
    }

    #[test]
    fn test_selected_hash_set_implementation_types() {
        fn check_impl(expected: &str, ts: TokenStream) {
            let r = fz_hash_set_macro(ts).unwrap().to_string();
            assert!(r.contains(expected), "{r} doesn't contain {expected}");
        }

        check_impl(":: InlineDenseScalarLookupSet", quote!({ 1, 2, 3, }));
        check_impl(":: InlineSparseScalarLookupSet", quote!({ 1, 2, 3, 4, 6 }));
        check_impl(":: InlineScanSet", quote!({ 1, 2, 10000 }));
        check_impl(":: InlineOrderedScanSet", quote!({ 1, 2, 3, 4, 5, 10000 }));
        check_impl(":: InlineHashSet", quote!({ 1, 2, 3, 4, 5, 6, 10000 }));

        check_impl(":: InlineScanSet", quote!({ "1", "2", "3", }));
        check_impl(":: InlineOrderedScanSet", quote!({ "1", "2", "3", "4" }));
        check_impl(
            ":: InlineOrderedScanSet",
            quote!({ "1", "2", "3", "4", "5", "6"}),
        );
        check_impl(
            ":: InlineHashSet",
            quote!({ "1", "2", "3", "4", "5", "6", "7" }),
        );

        check_impl(":: ScanSet", quote!({ Foo(1), Foo(2), Foo(3), }));
        check_impl(
            ":: HashSet",
            quote!({ Foo(1), Foo(2), Foo(3), Foo(4), Foo(5), Foo(6), Foo(7) }),
        );

        check_impl(":: ScanSet", quote!({ x, 2, 3, }));
        check_impl(":: FacadeScalarSet", quote!({ x, 2, 3, 4 }));
        check_impl(":: FacadeScalarSet", quote!({ x, 2, 3, 4, 5, 6}));
        check_impl(":: FacadeScalarSet", quote!({ x, 2, 3, 4, 5, 6, 7 }));

        check_impl(":: ScanSet", quote!({ x, "2", "3", }));
        check_impl(":: FacadeStringSet", quote!({ x, "2", "3", "4" }));
        check_impl(":: FacadeStringSet", quote!({ x, "2", "3", "4", "5", "6"}));
        check_impl(
            ":: FacadeStringSet",
            quote!({ x, "2", "3", "4", "5", "6", "7" }),
        );
    }

    #[test]
    fn test_scalar_suffixes() {
        let r = fz_scalar_set_macro(quote!({ 1i8, 2, 3, 4, 5, 6 }))
            .unwrap()
            .to_string();
        assert!(r.contains("1i8"));

        let r = fz_scalar_set_macro(quote!({ 1, 2, 3, 4, 5, 6 }))
            .unwrap()
            .to_string();
        assert!(!r.contains("1i8"));
        assert!(r.contains("1i32"));
    }
}
