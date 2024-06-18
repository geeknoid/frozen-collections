use crate::macros::generator;
use crate::macros::generator::KeyKind;
use crate::macros::parsing::long_form_map::LongFormMap;
use crate::macros::parsing::map::Map;
use crate::macros::parsing::short_form_map::ShortFormMap;
use crate::utils::pick_compile_time_random_seeds;
use proc_macro2::TokenStream;
use quote::quote;
use syn::parse2;

/// Implementation logic for the `fz_hash_map!` macro.
///
/// # Errors
///
/// Bad things happen to bad input
pub fn fz_hash_map_macro(args: TokenStream) -> syn::Result<TokenStream> {
    fz_map_macro(args, pick_compile_time_random_seeds(), KeyKind::Hashed)
}

/// Implementation logic for the `fz_ordered_map!` macro.
///
/// # Errors
///
/// Bad things happen to bad input
pub fn fz_ordered_map_macro(args: TokenStream) -> syn::Result<TokenStream> {
    fz_map_macro(args, pick_compile_time_random_seeds(), KeyKind::Ordered)
}

/// Implementation logic for the `fz_string_map!` macro.
///
/// # Errors
///
/// Bad things happen to bad input
pub fn fz_string_map_macro(args: TokenStream) -> syn::Result<TokenStream> {
    fz_map_macro(args, pick_compile_time_random_seeds(), KeyKind::String)
}

/// Implementation logic for the `fz_scalar_map!` macro.
///
/// # Errors
///
/// Bad things happen to bad input
pub fn fz_scalar_map_macro(args: TokenStream) -> syn::Result<TokenStream> {
    fz_map_macro(args, pick_compile_time_random_seeds(), KeyKind::Scalar)
}

fn fz_map_macro(
    args: TokenStream,
    seeds: (u64, u64, u64, u64),
    key_kind: KeyKind,
) -> syn::Result<TokenStream> {
    let input = parse2::<Map>(args)?;

    match input {
        Map::Short(map) => short_form_fz_map_macro(map, seeds, key_kind),
        Map::Long(map) => long_form_fz_map_macro(map, seeds, key_kind),
    }
}

fn short_form_fz_map_macro(
    map: ShortFormMap,
    seeds: (u64, u64, u64, u64),
    key_kind: KeyKind,
) -> syn::Result<TokenStream> {
    Ok(generator::generate(map.payload, seeds, false, quote!(_), quote!(_), key_kind)?.ctor)
}

fn long_form_fz_map_macro(
    map: LongFormMap,
    seeds: (u64, u64, u64, u64),
    key_kind: KeyKind,
) -> syn::Result<TokenStream> {
    let key_type = map.key_type;
    let value_type = map.value_type;

    let key_type = if map.key_type_amp {
        quote!(&'static #key_type)
    } else {
        quote!(#key_type)
    };

    let value_type = quote!(#value_type);

    let output = generator::generate(map.payload, seeds, false, key_type, value_type, key_kind)?;

    let type_sig = output.type_sig;
    let ctor = output.ctor;
    let var_name = &map.var_name;
    let type_name = &map.type_name;
    let visibility = &map.visibility;

    if !map.is_static {
        let mutable = if map.is_mutable {
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
