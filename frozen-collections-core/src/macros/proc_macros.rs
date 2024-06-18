use crate::macros::generator;
use crate::macros::generator::KeyKind;
use crate::macros::parsing::map::Map;
use crate::macros::parsing::map_combo::MapCombo;
use crate::macros::parsing::set::Set;
use crate::macros::parsing::set_combo::SetCombo;
use crate::macros::parsing::static_map::StaticMap;
use crate::macros::parsing::static_set::StaticSet;
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

/// Implementation logic for the `fz_hash_set!` macro.
///
/// # Errors
///
/// Bad things happen to bad input
pub fn fz_hash_set_macro(args: TokenStream) -> syn::Result<TokenStream> {
    fz_set_macro(args, pick_compile_time_random_seeds(), KeyKind::Hashed)
}

/// Implementation logic for the `fz_ordered_map!` macro.
///
/// # Errors
///
/// Bad things happen to bad input
pub fn fz_ordered_map_macro(args: TokenStream) -> syn::Result<TokenStream> {
    fz_map_macro(args, pick_compile_time_random_seeds(), KeyKind::Ordered)
}

/// Implementation logic for the `fz_ordered_set!` macro.
///
/// # Errors
///
/// Bad things happen to bad input
pub fn fz_ordered_set_macro(args: TokenStream) -> syn::Result<TokenStream> {
    fz_set_macro(args, pick_compile_time_random_seeds(), KeyKind::Ordered)
}

/// Implementation logic for the `fz_string_map!` macro.
///
/// # Errors
///
/// Bad things happen to bad input
pub fn fz_string_map_macro(args: TokenStream) -> syn::Result<TokenStream> {
    fz_map_macro(args, pick_compile_time_random_seeds(), KeyKind::String)
}

/// Implementation logic for the `fz_string_set!` macro.
///
/// # Errors
///
/// Bad things happen to bad input
pub fn fz_string_set_macro(args: TokenStream) -> syn::Result<TokenStream> {
    fz_set_macro(args, pick_compile_time_random_seeds(), KeyKind::String)
}

/// Implementation logic for the `fz_scalar_map!` macro.
///
/// # Errors
///
/// Bad things happen to bad input
pub fn fz_scalar_map_macro(args: TokenStream) -> syn::Result<TokenStream> {
    fz_map_macro(args, pick_compile_time_random_seeds(), KeyKind::Scalar)
}

/// Implementation logic for the `fz_scalar_set!` macro.
///
/// # Errors
///
/// Bad things happen to bad input
pub fn fz_scalar_set_macro(args: TokenStream) -> syn::Result<TokenStream> {
    fz_set_macro(args, pick_compile_time_random_seeds(), KeyKind::Scalar)
}

fn fz_map_macro(
    args: TokenStream,
    seeds: (u64, u64, u64, u64),
    key_kind: KeyKind,
) -> syn::Result<TokenStream> {
    let input = parse2::<MapCombo>(args)?;

    match input {
        MapCombo::Map(map) => normal_fz_map_macro(map, seeds, key_kind),
        MapCombo::StaticMap(map) => static_fz_map_macro(map, seeds, key_kind),
    }
}

fn normal_fz_map_macro(
    map: Map,
    seeds: (u64, u64, u64, u64),
    key_kind: KeyKind,
) -> syn::Result<TokenStream> {
    let key_type = quote!(_);
    let value_type = quote!(_);
    let entries = map
        .entries
        .into_iter()
        .map(std::convert::Into::into)
        .collect();

    Ok(generator::generate(entries, seeds, false, key_type, value_type, key_kind)?.ctor)
}

fn static_fz_map_macro(
    map: StaticMap,
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

    let entries = map
        .entries
        .into_iter()
        .map(std::convert::Into::into)
        .collect();

    let output = generator::generate(entries, seeds, false, key_type, value_type, key_kind)?;

    let type_sig = output.type_sig;
    let ctor = output.ctor;
    let var_name = &map.var_name;
    let type_name = &map.type_name;
    let visibility = &map.visibility;

    if output.constant {
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

fn fz_set_macro(
    args: TokenStream,
    seeds: (u64, u64, u64, u64),
    key_kind: KeyKind,
) -> syn::Result<TokenStream> {
    let input = parse2::<SetCombo>(args)?;

    match input {
        SetCombo::Set(set) => normal_fz_set_macro(set, seeds, key_kind),
        SetCombo::StaticSet(set) => static_fz_set_macro(set, seeds, key_kind),
    }
}

fn normal_fz_set_macro(
    set: Set,
    seeds: (u64, u64, u64, u64),
    key_kind: KeyKind,
) -> syn::Result<TokenStream> {
    let key_type = quote!(_);
    let value_type = quote!(_);
    let entries = set
        .values
        .into_iter()
        .map(std::convert::Into::into)
        .collect();

    Ok(generator::generate(entries, seeds, true, key_type, value_type, key_kind)?.ctor)
}

fn static_fz_set_macro(
    set: StaticSet,
    seeds: (u64, u64, u64, u64),
    key_kind: KeyKind,
) -> syn::Result<TokenStream> {
    let value_type = set.value_type;

    let value_type = if set.value_type_amp {
        quote!(&'static #value_type)
    } else {
        quote!(#value_type)
    };

    let entries = set
        .entries
        .into_iter()
        .map(std::convert::Into::into)
        .collect();

    let output = generator::generate(entries, seeds, true, value_type, quote!(_), key_kind)?;

    let type_sig = output.type_sig;
    let ctor = output.ctor;
    let var_name = &set.var_name;
    let type_name = &set.type_name;
    let visibility = &set.visibility;

    if output.constant {
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
    fn dense_lookup() {
        let input = quote!(
            1 => 2, 2: 3, 3: 4, 4: 5, 5: 6,
        );

        let output = fz_scalar_map_macro(input).unwrap();
        let expected = quote!(
            ::frozen_collections::inline_maps::InlineDenseScalarLookupMap::<_, _, 5>::new_raw(
                [(1, 2), (2, 3), (3, 4), (4, 5), (5, 6),],
                1,
                5
            )
        );

        assert_eq!(expected.to_string(), output.to_string());
    }

    #[test]
    fn self_hash() {
        let input = quote!(1u64: 0,
            10: 1,
            11: 1
            100: 2,
            101: 2,
            1_000: 3,
            1_001: 3,
            10_000: 4,
            10_001: 4,
            100_000: 5,
            100_001: 5,
            1_000_000: 6,
            1_000_001: 6,
            10_000_000: 7,
            10_000_001: 7,
            100_000_000: 8,
            100_000_001: 8,
            1_000_000_000: 9
            1_000_000_001: 9);
        let seeds = (1, 2, 3, 4);
        let output = fz_map_macro(input, seeds, KeyKind::Scalar).unwrap();

        let generics = quote!(< _ , _ , :: frozen_collections :: SmallCollection , 19 , 32, :: frozen_collections :: hashers :: PassthroughHasher >);
        let expected = quote!(:: frozen_collections :: inline_maps :: InlineHashMap :: #generics :: new_raw (:: frozen_collections :: inline_maps :: InlineHashTable :: < (_ , _) , :: frozen_collections :: SmallCollection , 19 , 32 > :: new_raw ([:: frozen_collections :: inline_maps :: HashTableSlot :: new (0 , 5) , :: frozen_collections :: inline_maps :: HashTableSlot :: new (5 , 11) , :: frozen_collections :: inline_maps :: HashTableSlot :: new (0 , 0) , :: frozen_collections :: inline_maps :: HashTableSlot :: new (0 , 0) , :: frozen_collections :: inline_maps :: HashTableSlot :: new (11 , 12) , :: frozen_collections :: inline_maps :: HashTableSlot :: new (12 , 13) , :: frozen_collections :: inline_maps :: HashTableSlot :: new (0 , 0) , :: frozen_collections :: inline_maps :: HashTableSlot :: new (0 , 0) , :: frozen_collections :: inline_maps :: HashTableSlot :: new (13 , 14) , :: frozen_collections :: inline_maps :: HashTableSlot :: new (14 , 15) , :: frozen_collections :: inline_maps :: HashTableSlot :: new (15 , 16) , :: frozen_collections :: inline_maps :: HashTableSlot :: new (16 , 17) , :: frozen_collections :: inline_maps :: HashTableSlot :: new (0 , 0) , :: frozen_collections :: inline_maps :: HashTableSlot :: new (0 , 0) , :: frozen_collections :: inline_maps :: HashTableSlot :: new (0 , 0) , :: frozen_collections :: inline_maps :: HashTableSlot :: new (0 , 0) , :: frozen_collections :: inline_maps :: HashTableSlot :: new (17 , 18) , :: frozen_collections :: inline_maps :: HashTableSlot :: new (18 , 19) , :: frozen_collections :: inline_maps :: HashTableSlot :: new (0 , 0) , :: frozen_collections :: inline_maps :: HashTableSlot :: new (0 , 0) , :: frozen_collections :: inline_maps :: HashTableSlot :: new (0 , 0) , :: frozen_collections :: inline_maps :: HashTableSlot :: new (0 , 0) , :: frozen_collections :: inline_maps :: HashTableSlot :: new (0 , 0) , :: frozen_collections :: inline_maps :: HashTableSlot :: new (0 , 0) , :: frozen_collections :: inline_maps :: HashTableSlot :: new (0 , 0) , :: frozen_collections :: inline_maps :: HashTableSlot :: new (0 , 0) , :: frozen_collections :: inline_maps :: HashTableSlot :: new (0 , 0) , :: frozen_collections :: inline_maps :: HashTableSlot :: new (0 , 0) , :: frozen_collections :: inline_maps :: HashTableSlot :: new (0 , 0) , :: frozen_collections :: inline_maps :: HashTableSlot :: new (0 , 0) , :: frozen_collections :: inline_maps :: HashTableSlot :: new (0 , 0) , :: frozen_collections :: inline_maps :: HashTableSlot :: new (0 , 0) ,] , [(1_000_000_000 , 9) , (100_000_000 , 8) , (10_000_000 , 7) , (1_000_000 , 6) , (100_000 , 5) , (1_000_000_001 , 9) , (100_000_001 , 8) , (10_000_001 , 7) , (1_000_001 , 6) , (100_001 , 5) , (1u64 , 0) , (100 , 2) , (101 , 2) , (1_000 , 3) , (1_001 , 3) , (10 , 1) , (11 , 1) , (10_000 , 4) , (10_001 , 4) ,] ,) , :: frozen_collections :: hashers :: PassthroughHasher :: new ()));

        assert_eq!(expected.to_string(), output.to_string());
    }

    #[test]
    fn self_hash_again() {
        let input = quote!("1": 1, "22":2, "333":3, "4444":4, "55555":5, "666666":6, "7777777":7, "88888888":8, "999999999":9);
        let seeds = (1, 2, 3, 4);
        let output = fz_map_macro(input, seeds, KeyKind::String).unwrap();

        let generics = quote!(< _ , _ , :: frozen_collections :: SmallCollection , 9 , 16 , :: frozen_collections :: hashers :: PassthroughHasher>);
        let expected = quote!(:: frozen_collections :: inline_maps :: InlineHashMap :: #generics :: new_raw (:: frozen_collections :: inline_maps :: InlineHashTable :: < (_ , _) , :: frozen_collections :: SmallCollection , 9 , 16 > :: new_raw ([:: frozen_collections :: inline_maps :: HashTableSlot :: new (0 , 0) , :: frozen_collections :: inline_maps :: HashTableSlot :: new (0 , 1) , :: frozen_collections :: inline_maps :: HashTableSlot :: new (1 , 2) , :: frozen_collections :: inline_maps :: HashTableSlot :: new (2 , 3) , :: frozen_collections :: inline_maps :: HashTableSlot :: new (3 , 4) , :: frozen_collections :: inline_maps :: HashTableSlot :: new (4 , 5) , :: frozen_collections :: inline_maps :: HashTableSlot :: new (5 , 6) , :: frozen_collections :: inline_maps :: HashTableSlot :: new (6 , 7) , :: frozen_collections :: inline_maps :: HashTableSlot :: new (7 , 8) , :: frozen_collections :: inline_maps :: HashTableSlot :: new (8 , 9) , :: frozen_collections :: inline_maps :: HashTableSlot :: new (0 , 0) , :: frozen_collections :: inline_maps :: HashTableSlot :: new (0 , 0) , :: frozen_collections :: inline_maps :: HashTableSlot :: new (0 , 0) , :: frozen_collections :: inline_maps :: HashTableSlot :: new (0 , 0) , :: frozen_collections :: inline_maps :: HashTableSlot :: new (0 , 0) , :: frozen_collections :: inline_maps :: HashTableSlot :: new (0 , 0) ,] , [("1" , 1) , ("22" , 2) , ("333" , 3) , ("4444" , 4) , ("55555" , 5) , ("666666" , 6) , ("7777777" , 7) , ("88888888" , 8) , ("999999999" , 9) ,] ,) , :: frozen_collections :: hashers :: PassthroughHasher :: new ()));

        assert_eq!(expected.to_string(), output.to_string());
    }
}
