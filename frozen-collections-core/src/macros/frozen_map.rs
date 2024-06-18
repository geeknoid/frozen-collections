use std::cmp::PartialEq;
use std::fmt::Display;
use std::hash::RandomState;
use std::str::FromStr;

use bitvec::macros::internal::funty::Fundamental;
use num_traits::PrimInt;
use proc_macro2::TokenStream;
use quote::{format_ident, quote, ToTokens};
use syn::parse::{Parse, ParseStream};
use syn::{parse2, Error, Expr, LitInt, LitStr, Token, Type};

use crate::analyzers::{analyze_int_keys, IntKeyAnalysisResult};
use crate::analyzers::{analyze_slice_keys, SliceKeyAnalysisResult};

struct Entry(Expr, Expr);

struct Map {
    ty: Type,
    entries: Vec<Entry>,
}

impl ToTokens for Entry {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let key = self.0.clone();
        let value = self.1.clone();

        tokens.extend(quote!(#key, #value));
    }
}

impl Parse for Map {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let mut entries = vec![];

        let ty = input.parse::<Type>()?;
        input.parse::<Token![,]>()?;

        while !input.is_empty() {
            let key = input.parse::<Expr>()?;
            input.parse::<Token![:]>()?;
            let value = input.parse::<Expr>()?;

            entries.push(Entry(key, value));

            if input.peek(Token![,]) {
                input.parse::<Token![,]>()?;
            }
        }

        Ok(Self { ty, entries })
    }
}

#[derive(PartialEq)]
enum KeyVariety {
    Common,
    Integer,
    String,
}

/// Implementation logic for the `frozen_map!` macro.
#[must_use]
#[allow(clippy::module_name_repetitions)]
pub fn frozen_map_macro(args: TokenStream) -> TokenStream {
    frozen_map_macro_internal(args).unwrap_or_else(|error| error.to_compile_error())
}

fn frozen_map_macro_internal(args: TokenStream) -> Result<TokenStream, Error> {
    // proc_marco2 version of "parse_macro_input!(input as ParsedMap)"
    let input = parse2::<Map>(args)?;
    let mut entries = input.entries;

    if entries.len() < 3 {
        return Ok(quote!(
            ::frozen_collections::specialized_maps::ScanningMap::try_from(vec![
            #(
                (#entries),
            )*
            ])
            .unwrap()
        ));
    }

    let mut ty = input.ty;
    let type_name = format!("{}", ty.to_token_stream());

    let mut variety = KeyVariety::Integer;
    let mut int_analysis = IntKeyAnalysisResult::Normal;
    let mut slice_analysis = SliceKeyAnalysisResult::Normal;
    let mut include_payload_size = true;

    match type_name.as_str() {
        "u8" => int_analysis = process_int_keys::<u8>(&entries)?,
        "i8" => int_analysis = process_int_keys::<i8>(&entries)?,
        "u16" => int_analysis = process_int_keys::<u16>(&entries)?,
        "i16" => int_analysis = process_int_keys::<i16>(&entries)?,
        "u32" => int_analysis = process_int_keys::<u32>(&entries)?,
        "i32" => int_analysis = process_int_keys::<i32>(&entries)?,
        "u64" => int_analysis = process_int_keys::<u64>(&entries)?,
        "i64" => int_analysis = process_int_keys::<i64>(&entries)?,
        "u128" => int_analysis = process_int_keys::<u128>(&entries)?,
        "i128" => int_analysis = process_int_keys::<i128>(&entries)?,

        "& str" => {
            variety = KeyVariety::String;
            slice_analysis =
                process_string_keys(entries.iter().map(|x| x.0.to_token_stream())).unwrap();

            let mut copy = Vec::with_capacity(entries.len());
            for kv in entries {
                let original = kv.0.to_token_stream();
                let modified = quote!(String::from(#original));
                copy.push(Entry(parse2::<Expr>(modified)?, kv.1));
            }

            entries = copy;
            ty = parse2::<Type>(quote!(String))?;
        }

        _ => variety = KeyVariety::Common,
    }

    let map_type = match variety {
        KeyVariety::Integer => {
            if int_analysis == IntKeyAnalysisResult::Range {
                include_payload_size = false;
                format_ident!("{}", "IntegerRangeMap")
            } else {
                format_ident!("{}", "IntegerMap")
            }
        }

        KeyVariety::String => match slice_analysis {
            SliceKeyAnalysisResult::Normal => format_ident!("{}", "CommonMap"),
            SliceKeyAnalysisResult::Length => format_ident!("{}", "LengthMap"),

            SliceKeyAnalysisResult::LeftHandSubslice {
                subslice_index: _,
                subslice_len: _,
            } => format_ident!("{}", "LeftSliceMap"),

            SliceKeyAnalysisResult::RightHandSubslice {
                subslice_index: _,
                subslice_len: _,
            } => format_ident!("{}", "RightSliceMap"),
        },

        KeyVariety::Common => format_ident!("{}", "CommonMap"),
    };

    let payload_size = format_ident!(
        "{}",
        if entries.len() <= u8::MAX.as_usize() {
            "u8"
        } else if entries.len() <= u16::MAX.as_usize() {
            "u16"
        } else {
            "usize"
        }
    );

    Ok(match slice_analysis {
        SliceKeyAnalysisResult::LeftHandSubslice {
            subslice_index,
            subslice_len,
        } => {
            quote!(::frozen_collections::specialized_maps::#map_type::<#ty, _, #payload_size, ::std::hash::RandomState>::try_from(vec![
            #(
                (#entries),
            )*
            ], #subslice_index..#subslice_index + #subslice_len).unwrap())
        }

        SliceKeyAnalysisResult::RightHandSubslice {
            subslice_index,
            subslice_len,
        } => {
            quote!(::frozen_collections::specialized_maps::#map_type::<#ty, _, #payload_size, ::std::hash::RandomState>::try_from(vec![
            #(
                (#entries),
            )*
            ], #subslice_index..#subslice_index + #subslice_len).unwrap())
        }

        _ => {
            if include_payload_size {
                quote!(::frozen_collections::specialized_maps::#map_type::<#ty, _, #payload_size>::try_from(vec![
                    #(
                        (#entries),
                    )*
                    ]).unwrap())
            } else {
                quote!(::frozen_collections::specialized_maps::#map_type::<#ty, _>::try_from(vec![
                    #(
                        (#entries),
                    )*
                    ]).unwrap())
            }
        }
    })
}

fn process_int_keys<K>(entries: &[Entry]) -> syn::Result<IntKeyAnalysisResult>
where
    K: PrimInt + FromStr,
    K::Err: Display,
{
    let keys = entries.iter().map(|x| x.0.to_token_stream());
    let mut parsed = Vec::new();
    for key in keys {
        let li = parse2::<LitInt>(key)?;
        let v = li.base10_parse::<K>()?;
        parsed.push(v);
    }

    Ok(analyze_int_keys(parsed.into_iter()))
}

fn process_string_keys<I>(keys: I) -> syn::Result<SliceKeyAnalysisResult>
where
    I: Iterator<Item = TokenStream>,
{
    let mut parsed = Vec::new();
    for key in keys {
        let ls = parse2::<LitStr>(key)?;
        parsed.push(ls.value());
    }

    let bh = RandomState::new();
    Ok(analyze_slice_keys(parsed.iter().map(String::as_bytes), &bh))
}

#[cfg(test)]
mod tests {
    use std::str::FromStr;

    use proc_macro2::TokenStream;

    use crate::macros::frozen_map::frozen_map_macro;

    #[test]
    fn basic() {
        let ts = TokenStream::from_str(
            "
            &str,
            \"first_key\": (1, \"first_value\"),
            \"second_key\": (2, \"second_value\"),
            \"third_key\": (3, \"third_value\"),
            \"fourth_key\": (4, \"fourth_value\"),
            \"fifth_key\": (5, \"fifth_value\"),
        ",
        )
        .unwrap();

        let ts2 = frozen_map_macro(ts);

        println!("{ts2}");
    }
}
