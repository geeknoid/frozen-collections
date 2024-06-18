use std::fmt::Display;
use std::hash::Hash;
use std::str::FromStr;

use ahash::RandomState;
use num_traits::{AsPrimitive, PrimInt};
use proc_macro2::TokenStream;
use quote::ToTokens;
use syn::{parse2, LitInt, LitStr};

use crate::analyzers::{
    analyze_hash_codes, analyze_int_keys, check_duplicate_keys, HashCodeAnalysisResult,
    IntKeyAnalysisResult,
};
use crate::analyzers::{analyze_slice_keys, SliceKeyAnalysisResult};

pub fn process_int_keys<I, K>(
    keys: I,
) -> syn::Result<(IntKeyAnalysisResult, Option<HashCodeAnalysisResult>)>
where
    I: Iterator<Item = TokenStream>,
    K: Hash + PrimInt + FromStr + AsPrimitive<u64>,
    K::Err: Display,
{
    let keys = Vec::from_iter(keys);
    let mut parsed = Vec::new();
    for key in &keys {
        let li = parse2::<LitInt>(key.to_token_stream())?;
        let v = li.base10_parse::<K>()?;
        parsed.push(v);
    }

    if check_duplicate_keys(parsed.iter()).is_err() {
        return Err(syn::Error::new_spanned(
            keys[0].to_token_stream(),
            "Duplicate keys are not allowed in a frozen map.",
        ));
    }

    Ok((
        analyze_int_keys(parsed.iter().copied()),
        Some(analyze_hash_codes(parsed.iter().map(|entry| entry.as_()))),
    ))
}

pub fn process_string_keys<I>(
    keys: I,
) -> syn::Result<(SliceKeyAnalysisResult, Option<HashCodeAnalysisResult>)>
where
    I: Iterator<Item = TokenStream>,
{
    let keys = Vec::from_iter(keys);
    let mut parsed = Vec::new();
    for key in &keys {
        let ls = parse2::<LitStr>(key.to_token_stream())?;
        parsed.push(ls.value());
    }

    if check_duplicate_keys(parsed.iter()).is_err() {
        return Err(syn::Error::new_spanned(
            keys[0].to_token_stream(),
            "Duplicate keys are not allowed in a frozen map.",
        ));
    }

    let bh = RandomState::with_seeds(0, 0, 0, 0);
    let slice_analysis = analyze_slice_keys(parsed.iter().map(String::as_bytes), &bh);

    let code_analysis = match &slice_analysis {
        SliceKeyAnalysisResult::Normal => analyze_hash_codes(
            parsed
                .iter()
                .map(String::as_bytes)
                .map(|entry| bh.hash_one(entry)),
        ),

        SliceKeyAnalysisResult::Length => analyze_hash_codes(
            parsed
                .iter()
                .map(String::as_bytes)
                .map(|entry| entry.len() as u64),
        ),

        SliceKeyAnalysisResult::LeftHandSubslice(range) => analyze_hash_codes(
            parsed
                .iter()
                .map(String::as_bytes)
                .map(|entry| &entry[range.start..range.end])
                .map(|entry| bh.hash_one(entry)),
        ),

        SliceKeyAnalysisResult::RightHandSubslice(range) => analyze_hash_codes(
            parsed
                .iter()
                .map(String::as_bytes)
                .map(|entry| &entry[entry.len() - range.start..entry.len() - range.end])
                .map(|entry| bh.hash_one(entry)),
        ),
    };

    Ok((slice_analysis, Some(code_analysis)))
}
