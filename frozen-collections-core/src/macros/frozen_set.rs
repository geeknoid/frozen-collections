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

struct Value(Expr);

struct Set {
    ty: Type,
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

        let ty = input.parse::<Type>()?;
        input.parse::<Token![,]>()?;

        while !input.is_empty() {
            let value = input.parse::<Expr>()?;

            values.push(Value(value));

            if input.peek(Token![,]) {
                input.parse::<Token![,]>()?;
            }
        }

        Ok(Self { ty, values })
    }
}

#[derive(PartialEq)]
enum ValueVariety {
    Common,
    Integer,
    String,
}

/// Implementation logic for the `frozen_set!` macro.
#[must_use]
#[allow(clippy::module_name_repetitions)]
pub fn frozen_set_macro(args: TokenStream) -> TokenStream {
    frozen_set_macro_internal(args).unwrap_or_else(|error| error.to_compile_error())
}

fn frozen_set_macro_internal(args: TokenStream) -> Result<TokenStream, Error> {
    // proc_marco2 version of "parse_macro_input!(input as ParsedSet)"
    let input = parse2::<Set>(args)?;
    let mut values = input.values;

    if values.len() < 3 {
        return Ok(quote!(
            ::frozen_collections::specialized_sets::ScanningSet::try_from(vec![
            #(
                (#values),
            )*
            ])
        ));
    }

    let mut ty = input.ty;
    let type_name = format!("{}", ty.to_token_stream());

    let mut variety = ValueVariety::Integer;
    let mut int_analysis = IntKeyAnalysisResult::Normal;
    let mut slice_analysis = SliceKeyAnalysisResult::Normal;
    let mut include_payload_size = true;

    match type_name.as_str() {
        "u8" => int_analysis = process_int_values::<u8>(&values)?,
        "i8" => int_analysis = process_int_values::<i8>(&values)?,
        "u16" => int_analysis = process_int_values::<u16>(&values)?,
        "i16" => int_analysis = process_int_values::<i16>(&values)?,
        "u32" => int_analysis = process_int_values::<u32>(&values)?,
        "i32" => int_analysis = process_int_values::<i32>(&values)?,
        "u64" => int_analysis = process_int_values::<u64>(&values)?,
        "i64" => int_analysis = process_int_values::<i64>(&values)?,
        "u128" => int_analysis = process_int_values::<u128>(&values)?,
        "i128" => int_analysis = process_int_values::<i128>(&values)?,

        "& str" => {
            variety = ValueVariety::String;
            slice_analysis =
                process_string_values(values.iter().map(|x| x.0.to_token_stream())).unwrap();

            let mut copy = Vec::with_capacity(values.len());
            for value in values {
                let original = value.0.to_token_stream();
                let modified = quote!(String::from(#original));
                copy.push(Value(parse2::<Expr>(modified)?));
            }

            values = copy;
            ty = parse2::<Type>(quote!(String))?;
        }

        _ => variety = ValueVariety::Common,
    }

    let set_type = match variety {
        ValueVariety::Integer => {
            if int_analysis == IntKeyAnalysisResult::Range {
                include_payload_size = false;
                format_ident!("{}", "IntegerRangeSet")
            } else {
                format_ident!("{}", "IntegerSet")
            }
        }

        ValueVariety::String => match slice_analysis {
            SliceKeyAnalysisResult::Normal => format_ident!("{}", "CommonSet"),
            SliceKeyAnalysisResult::Length => format_ident!("{}", "LengthSet"),

            SliceKeyAnalysisResult::LeftHandSubslice {
                subslice_index: _,
                subslice_len: _,
            } => format_ident!("{}", "LeftSliceSet"),

            SliceKeyAnalysisResult::RightHandSubslice {
                subslice_index: _,
                subslice_len: _,
            } => format_ident!("{}", "RightSliceSet"),
        },

        ValueVariety::Common => format_ident!("{}", "CommonSet"),
    };

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

    Ok(match slice_analysis {
        SliceKeyAnalysisResult::LeftHandSubslice {
            subslice_index,
            subslice_len,
        } => {
            quote!(::frozen_collections::specialized_sets::#set_type::<#ty, #payload_size, ::std::hash::RandomState>::try_from(vec![
            #(
                (#values),
            )*
            ], #subslice_index..#subslice_index + #subslice_len).unwrap())
        }

        SliceKeyAnalysisResult::RightHandSubslice {
            subslice_index,
            subslice_len,
        } => {
            quote!(::frozen_collections::specialized_sets::#set_type::<#ty, #payload_size, ::std::hash::RandomState>::try_from(vec![
            #(
                (#values),
            )*
            ], #subslice_index..#subslice_index + #subslice_len).unwrap())
        }

        _ => {
            if include_payload_size {
                quote!(::frozen_collections::specialized_sets::#set_type::<#ty, #payload_size>::try_from(vec![
                #(
                    (#values),
                )*
                ]).unwrap())
            } else {
                quote!(::frozen_collections::specialized_sets::#set_type::<#ty>::try_from(vec![
                #(
                    (#values),
                )*
                ]).unwrap())
            }
        }
    })
}

fn process_int_values<T>(values: &[Value]) -> syn::Result<IntKeyAnalysisResult>
where
    T: PrimInt + FromStr,
    T::Err: Display,
{
    let mut parsed = Vec::new();
    for v in values.iter().map(|x| x.0.to_token_stream()) {
        let li = parse2::<LitInt>(v)?;
        let v = li.base10_parse::<T>()?;
        parsed.push(v);
    }

    Ok(analyze_int_keys(parsed.into_iter()))
}

fn process_string_values<I>(values: I) -> syn::Result<SliceKeyAnalysisResult>
where
    I: Iterator<Item = TokenStream>,
{
    let mut parsed = Vec::new();
    for v in values {
        let ls = parse2::<LitStr>(v)?;
        parsed.push(ls.value());
    }

    let bh = RandomState::new();
    Ok(analyze_slice_keys(parsed.iter().map(String::as_bytes), &bh))
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
            &str,
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
