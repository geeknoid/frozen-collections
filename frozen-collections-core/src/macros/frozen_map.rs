use std::cmp::PartialEq;

use bitvec::macros::internal::funty::Fundamental;
use proc_macro2::TokenStream;
use quote::{format_ident, quote, ToTokens};
use syn::parse::{Parse, ParseStream};
use syn::{parse2, Error, Expr, Token};

use crate::analyzers::IntKeyAnalysisResult;
use crate::analyzers::SliceKeyAnalysisResult;
use crate::macros::analysis::{process_int_keys, process_string_keys};
use crate::macros::key_type::{extract, KeyType};

pub struct Entry(pub Expr, pub Expr);

struct Map {
    key_type: KeyType,
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
        let mut key_type = KeyType::Complex;

        while !input.is_empty() {
            let key = input.parse::<Expr>()?;
            input.parse::<Token![:]>()?;
            let value = input.parse::<Expr>()?;

            if entries.is_empty() {
                key_type = extract(key.clone());
            }

            entries.push(Entry(key, value));

            if input.peek(Token![,]) {
                input.parse::<Token![,]>()?;
            }
        }

        Ok(Self { key_type, entries })
    }
}

#[derive(PartialEq)]
enum KeyVariety {
    Complex,
    Integer,
    String,
}

/// Implementation logic for the `frozen_map!` macro.
#[must_use]
#[allow(clippy::module_name_repetitions)]
pub fn frozen_map_macro(args: TokenStream) -> TokenStream {
    frozen_map_macro_internal(args).unwrap_or_else(|error| error.to_compile_error())
}

#[allow(clippy::cognitive_complexity)]
fn frozen_map_macro_internal(args: TokenStream) -> Result<TokenStream, Error> {
    // proc_marco2 version of "parse_macro_input!(input as ParsedMap)"
    let input = parse2::<Map>(args)?;
    let mut entries = input.entries;

    if entries.len() < 3 {
        return Ok(quote!(
            ::frozen_collections::specialized_maps::ScanningMap::new(vec![
            #(
                (#entries),
            )*
            ])
            .unwrap()
        ));
    }

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

    let mut variety = KeyVariety::Integer;
    let mut int_analysis = IntKeyAnalysisResult::Normal;
    let mut slice_analysis = SliceKeyAnalysisResult::Normal;
    let mut code_analysis = None;

    match input.key_type {
        KeyType::U8 => {
            (int_analysis, code_analysis) =
                process_int_keys::<_, u8>(entries.iter().map(|x| x.0.to_token_stream()))?;
        }
        KeyType::I8 => {
            (int_analysis, code_analysis) =
                process_int_keys::<_, i8>(entries.iter().map(|x| x.0.to_token_stream()))?;
        }
        KeyType::U16 => {
            (int_analysis, code_analysis) =
                process_int_keys::<_, u16>(entries.iter().map(|x| x.0.to_token_stream()))?;
        }
        KeyType::I16 => {
            (int_analysis, code_analysis) =
                process_int_keys::<_, i16>(entries.iter().map(|x| x.0.to_token_stream()))?;
        }
        KeyType::U32 => {
            (int_analysis, code_analysis) =
                process_int_keys::<_, u32>(entries.iter().map(|x| x.0.to_token_stream()))?;
        }
        KeyType::I32 => {
            (int_analysis, code_analysis) =
                process_int_keys::<_, i32>(entries.iter().map(|x| x.0.to_token_stream()))?;
        }
        KeyType::U64 => {
            (int_analysis, code_analysis) =
                process_int_keys::<_, u64>(entries.iter().map(|x| x.0.to_token_stream()))?;
        }
        KeyType::I64 => {
            (int_analysis, code_analysis) =
                process_int_keys::<_, i64>(entries.iter().map(|x| x.0.to_token_stream()))?;
        }
        KeyType::U128 => {
            (int_analysis, code_analysis) =
                process_int_keys::<_, u128>(entries.iter().map(|x| x.0.to_token_stream()))?;
        }
        KeyType::I128 => {
            (int_analysis, code_analysis) =
                process_int_keys::<_, i128>(entries.iter().map(|x| x.0.to_token_stream()))?;
        }

        KeyType::String => {
            variety = KeyVariety::String;
            (slice_analysis, code_analysis) =
                process_string_keys(entries.iter().map(|x| x.0.to_token_stream()))?;

            let mut copy = Vec::with_capacity(entries.len());
            for kv in entries {
                let original = kv.0.to_token_stream();
                let modified = quote!(String::from(#original));
                copy.push(Entry(parse2::<Expr>(modified)?, kv.1));
            }

            entries = copy;
        }

        KeyType::Complex => variety = KeyVariety::Complex,
    }

    Ok(match variety {
        KeyVariety::Integer => {
            let ca = code_analysis.clone().unwrap();
            let num_hash_slots = ca.num_hash_slots;
            let num_hash_collisions = ca.num_hash_collisions;

            if int_analysis == IntKeyAnalysisResult::Range {
                quote!(
                    ::frozen_collections::specialized_maps::IntegerRangeMap::new(vec![
                    #(
                        (#entries),
                    )*
                    ])
                    .unwrap()
                )
            } else if code_analysis.is_some() && code_analysis.unwrap().num_hash_collisions > 0 {
                quote!(::frozen_collections::specialized_maps::IntegerMap::<_, _, #payload_size>::with_analysis(vec![
                    #(
                        (#entries),
                    )*
                    ], ::frozen_collections::analyzers::HashCodeAnalysisResult { num_hash_slots: #num_hash_slots, num_hash_collisions: #num_hash_collisions}).unwrap())
            } else {
                quote!(::frozen_collections::specialized_maps::IntegerMapNoCollisions::<_, _, #payload_size>::with_analysis(vec![
                    #(
                        (#entries),
                    )*
                    ], ::frozen_collections::analyzers::HashCodeAnalysisResult { num_hash_slots: #num_hash_slots, num_hash_collisions: #num_hash_collisions}).unwrap())
            }
        }

        KeyVariety::String => {
            let ca = code_analysis.clone().unwrap();
            let num_hash_slots = ca.num_hash_slots;
            let num_hash_collisions = ca.num_hash_collisions;

            match slice_analysis {
                SliceKeyAnalysisResult::Normal => {
                    if code_analysis.is_some() && code_analysis.unwrap().num_hash_collisions > 0 {
                        quote!(::frozen_collections::specialized_maps::CommonMap::with_hasher_and_analysis(vec![
                        #(
                            (#entries),
                        )*
                        ], ::ahash::RandomState::with_seeds(0, 0, 0, 0), ::frozen_collections::analyzers::HashCodeAnalysisResult { num_hash_slots: #num_hash_slots, num_hash_collisions: #num_hash_collisions}).unwrap())
                    } else {
                        quote!(::frozen_collections::specialized_maps::CommonMapNoCollisions::with_hasher_and_analysis(vec![
                        #(
                            (#entries),
                        )*
                        ], ::ahash::RandomState::with_seeds(0, 0, 0, 0), ::frozen_collections::analyzers::HashCodeAnalysisResult { num_hash_slots: #num_hash_slots, num_hash_collisions: #num_hash_collisions}).unwrap())
                    }
                }

                SliceKeyAnalysisResult::Length => {
                    if code_analysis.is_some() && code_analysis.unwrap().num_hash_collisions > 0 {
                        quote!(::frozen_collections::specialized_maps::LengthMap::<_, _, #payload_size>::with_analysis(vec![
                        #(
                            (#entries),
                        )*
                        ], ::frozen_collections::analyzers::HashCodeAnalysisResult { num_hash_slots: #num_hash_slots, num_hash_collisions: #num_hash_collisions}).unwrap())
                    } else {
                        quote!(::frozen_collections::specialized_maps::LengthMapNoCollisions::<_, _, #payload_size>::with_analysis(vec![
                        #(
                            (#entries),
                        )*
                        ], ::frozen_collections::analyzers::HashCodeAnalysisResult { num_hash_slots: #num_hash_slots, num_hash_collisions: #num_hash_collisions}).unwrap())
                    }
                }

                SliceKeyAnalysisResult::LeftHandSubslice(range) => {
                    let start = range.start;
                    let end = range.end;

                    if code_analysis.is_some() && code_analysis.unwrap().num_hash_collisions > 0 {
                        quote!(::frozen_collections::specialized_maps::LeftSliceMap::<_, _, #payload_size>::with_hasher_and_analysis(vec![
                        #(
                            (#entries),
                        )*
                        ], #start..#end, ::ahash::RandomState::with_seeds(0, 0, 0, 0), ::frozen_collections::analyzers::HashCodeAnalysisResult { num_hash_slots: #num_hash_slots, num_hash_collisions: #num_hash_collisions}).unwrap())
                    } else {
                        quote!(::frozen_collections::specialized_maps::LeftSliceMapNoCollisions::<_, _, #payload_size>::with_hasher_and_analysis(vec![
                        #(
                            (#entries),
                        )*
                        ], #start..#end, ::ahash::RandomState::with_seeds(0, 0, 0, 0), ::frozen_collections::analyzers::HashCodeAnalysisResult { num_hash_slots: #num_hash_slots, num_hash_collisions: #num_hash_collisions}).unwrap())
                    }
                }

                SliceKeyAnalysisResult::RightHandSubslice(range) => {
                    let start = range.start;
                    let end = range.end;

                    if code_analysis.is_some() && code_analysis.unwrap().num_hash_collisions > 0 {
                        quote!(::frozen_collections::specialized_maps::RightSliceMap::<_, _, #payload_size>::with_hasher_and_analysis(vec![
                        #(
                            (#entries),
                        )*
                        ], #start..#end, ::ahash::RandomState::with_seeds(0, 0, 0, 0), ::frozen_collections::analyzers::HashCodeAnalysisResult { num_hash_slots: #num_hash_slots, num_hash_collisions: #num_hash_collisions}).unwrap())
                    } else {
                        quote!(::frozen_collections::specialized_maps::RightSliceMapNoCollisions::<_, _, #payload_size>::with_hasher_and_analysis(vec![
                        #(
                            (#entries),
                        )*
                        ], #start..#end, ::ahash::RandomState::with_seeds(0, 0, 0, 0), ::frozen_collections::analyzers::HashCodeAnalysisResult { num_hash_slots: #num_hash_slots, num_hash_collisions: #num_hash_collisions}).unwrap())
                    }
                }
            }
        }

        KeyVariety::Complex => {
            quote!(
                ::frozen_collections::specialized_maps::CommonMap::<_, _, #payload_size, _>::with_hasher(
                    vec![
                    #(
                        (#entries),
                    )*
                    ],
                    ::ahash::RandomState::with_seeds(0, 0, 0, 0)).unwrap())
        }
    })
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

    #[test]
    fn integer_range() {
        let ts = TokenStream::from_str(
            "
            1u8: (1, \"first_value\"),
            2: (2, \"second_value\"),
            3: (3, \"third_value\"),
            4: (4, \"fourth_value\"),
            5: (5, \"fifth_value\"),
        ",
        )
        .unwrap();

        let ts2 = frozen_map_macro(ts);

        println!("{ts2}");
    }
}
