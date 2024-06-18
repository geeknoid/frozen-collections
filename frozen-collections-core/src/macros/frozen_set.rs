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

struct Value(Expr);

struct Set {
    key_type: KeyType,
    values: Vec<Value>,
}

impl ToTokens for Value {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let value = self.0.clone();

        tokens.extend(quote!(#value));
    }
}

impl Parse for Set {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let mut values = vec![];
        let mut key_type = KeyType::Complex;

        while !input.is_empty() {
            let value = input.parse::<Expr>()?;

            if values.is_empty() {
                key_type = extract(value.clone());
            }

            values.push(Value(value));

            if input.peek(Token![,]) {
                input.parse::<Token![,]>()?;
            }
        }

        Ok(Self { key_type, values })
    }
}

#[derive(PartialEq)]
enum ValueVariety {
    Complex,
    Integer,
    String,
}

/// Implementation logic for the `frozen_set!` macro.
#[must_use]
#[allow(clippy::module_name_repetitions)]
pub fn frozen_set_macro(args: TokenStream) -> TokenStream {
    frozen_set_macro_internal(args).unwrap_or_else(|error| error.to_compile_error())
}

#[allow(clippy::cognitive_complexity)]
fn frozen_set_macro_internal(args: TokenStream) -> Result<TokenStream, Error> {
    // proc_marco2 version of "parse_macro_input!(input as ParsedSet)"
    let input = parse2::<Set>(args)?;
    let mut values = input.values;

    if values.len() < 3 {
        return Ok(quote!(
            ::frozen_collections::specialized_sets::ScanningSet::new(vec![
            #(
                (#values),
            )*
            ])
            .unwrap()
        ));
    }

    let payload_size = format_ident!(
        "{}",
        if values.len() <= u8::MAX.as_usize() {
            "u8"
        } else if values.len() <= u16::MAX.as_usize() {
            "u16"
        } else {
            "usize"
        }
    );

    let mut variety = crate::macros::frozen_set::ValueVariety::Integer;
    let mut int_analysis = IntKeyAnalysisResult::Normal;
    let mut slice_analysis = SliceKeyAnalysisResult::Normal;
    let mut code_analysis = None;

    match input.key_type {
        KeyType::U8 => {
            (int_analysis, code_analysis) =
                process_int_keys::<_, u8>(values.iter().map(ToTokens::to_token_stream))?;
        }
        KeyType::I8 => {
            (int_analysis, code_analysis) =
                process_int_keys::<_, i8>(values.iter().map(ToTokens::to_token_stream))?;
        }
        KeyType::U16 => {
            (int_analysis, code_analysis) =
                process_int_keys::<_, u16>(values.iter().map(ToTokens::to_token_stream))?;
        }
        KeyType::I16 => {
            (int_analysis, code_analysis) =
                process_int_keys::<_, i16>(values.iter().map(ToTokens::to_token_stream))?;
        }
        KeyType::U32 => {
            (int_analysis, code_analysis) =
                process_int_keys::<_, u32>(values.iter().map(ToTokens::to_token_stream))?;
        }
        KeyType::I32 => {
            (int_analysis, code_analysis) =
                process_int_keys::<_, i32>(values.iter().map(ToTokens::to_token_stream))?;
        }
        KeyType::U64 => {
            (int_analysis, code_analysis) =
                process_int_keys::<_, u64>(values.iter().map(ToTokens::to_token_stream))?;
        }
        KeyType::I64 => {
            (int_analysis, code_analysis) =
                process_int_keys::<_, i64>(values.iter().map(ToTokens::to_token_stream))?;
        }
        KeyType::U128 => {
            (int_analysis, code_analysis) =
                process_int_keys::<_, u128>(values.iter().map(ToTokens::to_token_stream))?;
        }
        KeyType::I128 => {
            (int_analysis, code_analysis) =
                process_int_keys::<_, i128>(values.iter().map(ToTokens::to_token_stream))?;
        }

        KeyType::String => {
            variety = crate::macros::frozen_set::ValueVariety::String;
            (slice_analysis, code_analysis) =
                process_string_keys(values.iter().map(ToTokens::to_token_stream))?;

            let mut copy = Vec::with_capacity(values.len());
            for v in values {
                let original = v.to_token_stream();
                let modified = quote!(String::from(#original));
                copy.push(Value(parse2::<Expr>(modified)?));
            }

            values = copy;
        }

        KeyType::Complex => variety = crate::macros::frozen_set::ValueVariety::Complex,
    }

    Ok(match variety {
        ValueVariety::Integer => {
            let ca = code_analysis.clone().unwrap();
            let num_hash_slots = ca.num_hash_slots;
            let num_hash_collisions = ca.num_hash_collisions;

            if int_analysis == IntKeyAnalysisResult::Range {
                quote!(
                    ::frozen_collections::specialized_sets::IntegerRangeSet::new(vec![
                    #(
                        (#values),
                    )*
                    ])
                    .unwrap()
                )
            } else if code_analysis.is_some() && code_analysis.unwrap().num_hash_collisions > 0 {
                quote!(::frozen_collections::specialized_sets::IntegerSet::<_, #payload_size>::with_analysis(vec![
                    #(
                        (#values),
                    )*
                    ], ::frozen_collections::analyzers::HashCodeAnalysisResult { num_hash_slots: #num_hash_slots, num_hash_collisions: #num_hash_collisions}).unwrap())
            } else {
                quote!(::frozen_collections::specialized_sets::IntegerSetNoCollisions::<_, #payload_size>::with_analysis(vec![
                    #(
                        (#values),
                    )*
                    ], ::frozen_collections::analyzers::HashCodeAnalysisResult { num_hash_slots: #num_hash_slots, num_hash_collisions: #num_hash_collisions}).unwrap())
            }
        }

        ValueVariety::String => {
            let ca = code_analysis.clone().unwrap();
            let num_hash_slots = ca.num_hash_slots;
            let num_hash_collisions = ca.num_hash_collisions;

            match slice_analysis {
                SliceKeyAnalysisResult::Normal => {
                    if code_analysis.is_some() && code_analysis.unwrap().num_hash_collisions > 0 {
                        quote!(::frozen_collections::specialized_sets::CommonSet::with_hasher_and_analysis(vec![
                        #(
                            (#values),
                        )*
                        ], ::ahash::RandomState::with_seeds(0, 0, 0, 0), ::frozen_collections::analyzers::HashCodeAnalysisResult { num_hash_slots: #num_hash_slots, num_hash_collisions: #num_hash_collisions}).unwrap())
                    } else {
                        quote!(::frozen_collections::specialized_sets::CommonSetNoCollisions::with_hasher_and_analysis(vec![
                        #(
                            (#values),
                        )*
                        ], ::ahash::RandomState::with_seeds(0, 0, 0, 0), ::frozen_collections::analyzers::HashCodeAnalysisResult { num_hash_slots: #num_hash_slots, num_hash_collisions: #num_hash_collisions}).unwrap())
                    }
                }

                SliceKeyAnalysisResult::Length => {
                    if code_analysis.is_some() && code_analysis.unwrap().num_hash_collisions > 0 {
                        quote!(::frozen_collections::specialized_sets::LengthSet::<_, #payload_size>::with_analysis(vec![
                        #(
                            (#values),
                        )*
                        ], ::frozen_collections::analyzers::HashCodeAnalysisResult { num_hash_slots: #num_hash_slots, num_hash_collisions: #num_hash_collisions}).unwrap())
                    } else {
                        quote!(::frozen_collections::specialized_sets::LengthSetNoCollisions::<_, #payload_size>::with_analysis(vec![
                        #(
                            (#values),
                        )*
                        ], ::frozen_collections::analyzers::HashCodeAnalysisResult { num_hash_slots: #num_hash_slots, num_hash_collisions: #num_hash_collisions}).unwrap())
                    }
                }

                SliceKeyAnalysisResult::LeftHandSubslice(range) => {
                    let start = range.start;
                    let end = range.end;

                    if code_analysis.is_some() && code_analysis.unwrap().num_hash_collisions > 0 {
                        quote!(::frozen_collections::specialized_sets::LeftSliceSet::<_, #payload_size>::with_hasher_and_analysis(vec![
                        #(
                            (#values),
                        )*
                        ], #start..#end, ::ahash::RandomState::with_seeds(0, 0, 0, 0), ::frozen_collections::analyzers::HashCodeAnalysisResult { num_hash_slots: #num_hash_slots, num_hash_collisions: #num_hash_collisions}).unwrap())
                    } else {
                        quote!(::frozen_collections::specialized_sets::LeftSliceSetNoCollisions::<_, #payload_size>::with_hasher_and_analysis(vec![
                        #(
                            (#values),
                        )*
                        ], #start..#end, ::ahash::RandomState::with_seeds(0, 0, 0, 0), ::frozen_collections::analyzers::HashCodeAnalysisResult { num_hash_slots: #num_hash_slots, num_hash_collisions: #num_hash_collisions}).unwrap())
                    }
                }

                SliceKeyAnalysisResult::RightHandSubslice(range) => {
                    let start = range.start;
                    let end = range.end;

                    if code_analysis.is_some() && code_analysis.unwrap().num_hash_collisions > 0 {
                        quote!(::frozen_collections::specialized_sets::RightSliceSet::<_, #payload_size>::with_hasher_and_analysis(vec![
                        #(
                            (#values),
                        )*
                        ], #start..#end, ::ahash::RandomState::with_seeds(0, 0, 0, 0), ::frozen_collections::analyzers::HashCodeAnalysisResult { num_hash_slots: #num_hash_slots, num_hash_collisions: #num_hash_collisions}).unwrap())
                    } else {
                        quote!(::frozen_collections::specialized_sets::RightSliceSetNoCollisions::<_, #payload_size>::with_hasher_and_analysis(vec![
                        #(
                            (#values),
                        )*
                        ], #start..#end, ::ahash::RandomState::with_seeds(0, 0, 0, 0), ::frozen_collections::analyzers::HashCodeAnalysisResult { num_hash_slots: #num_hash_slots, num_hash_collisions: #num_hash_collisions}).unwrap())
                    }
                }
            }
        }

        ValueVariety::Complex => {
            quote!(
                ::frozen_collections::specialized_sets::CommonSet::<_, #payload_size, _>::with_hasher(
                    vec![
                    #(
                        (#values),
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

    use crate::macros::frozen_set::frozen_set_macro;

    #[test]
    fn basic() {
        let ts = TokenStream::from_str(
            "
            \"first_value\",
            \"second_value\",
            \"third_value\",
            \"fourth_value\",
            \"fifth_value\",
        ",
        )
        .unwrap();

        let ts2 = frozen_set_macro(ts);

        println!("{ts2}");
    }
}
