use crate::macros::generator;
use crate::macros::generator::KeyKind;
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
    fz_set_macro(args, pick_compile_time_random_seeds(), KeyKind::Hashed)
}

/// Implementation logic for the `fz_ordered_set!` macro.
///
/// # Errors
///
/// Bad things happen to bad input
pub fn fz_ordered_set_macro(args: TokenStream) -> syn::Result<TokenStream> {
    fz_set_macro(args, pick_compile_time_random_seeds(), KeyKind::Ordered)
}

/// Implementation logic for the `fz_string_set!` macro.
///
/// # Errors
///
/// Bad things happen to bad input
pub fn fz_string_set_macro(args: TokenStream) -> syn::Result<TokenStream> {
    fz_set_macro(args, pick_compile_time_random_seeds(), KeyKind::String)
}

/// Implementation logic for the `fz_scalar_set!` macro.
///
/// # Errors
///
/// Bad things happen to bad input
pub fn fz_scalar_set_macro(args: TokenStream) -> syn::Result<TokenStream> {
    fz_set_macro(args, pick_compile_time_random_seeds(), KeyKind::Scalar)
}

fn fz_set_macro(
    args: TokenStream,
    seeds: (u64, u64, u64, u64),
    key_kind: KeyKind,
) -> syn::Result<TokenStream> {
    let input = parse2::<Set>(args)?;

    match input {
        Set::Short(set) => short_form_fz_set_macro(set, seeds, key_kind),
        Set::Long(set) => long_form_fz_set_macro(set, seeds, key_kind),
    }
}

fn short_form_fz_set_macro(
    set: ShortFormSet,
    seeds: (u64, u64, u64, u64),
    key_kind: KeyKind,
) -> syn::Result<TokenStream> {
    Ok(generator::generate(set.payload, seeds, true, quote!(_), quote!(_), key_kind)?.ctor)
}

fn long_form_fz_set_macro(
    set: LongFormSet,
    seeds: (u64, u64, u64, u64),
    key_kind: KeyKind,
) -> syn::Result<TokenStream> {
    let value_type = set.value_type;

    let value_type = if set.value_type_amp {
        quote!(&'static #value_type)
    } else {
        quote!(#value_type)
    };

    let output = generator::generate(set.payload, seeds, true, value_type, quote!(_), key_kind)?;

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
    use quote::quote;

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
    fn invalid_literal() {
        let r = fz_scalar_set_macro(quote!(
            { "123iXXX", "234iXXX" }
        ));

        assert_eq!(
            "invalid literal, expecting an integer value",
            r.unwrap_err().to_string()
        );

        let r = fz_string_set_macro(quote!(
            { 123, 456 }
        ));

        assert_eq!(
            "invalid literal, expecting a string value",
            r.unwrap_err().to_string()
        );
    }
}
