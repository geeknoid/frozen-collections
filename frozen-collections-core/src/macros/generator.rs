use crate::analyzers::{
    analyze_scalar_keys, analyze_slice_keys, ScalarKeyAnalysisResult, SliceKeyAnalysisResult,
};
use crate::hash_tables::HashTable;
use crate::hashers::{LeftRangeHasher, PassthroughHasher, RightRangeHasher};
use crate::macros::parsing::entry::Entry;
use crate::macros::parsing::payload::Payload;
use crate::traits::{
    CollectionMagnitude, Hasher, LargeCollection, MediumCollection, Scalar, SmallCollection,
};
use crate::utils::{dedup_by_keep_last, slow_dedup_by_keep_last};
use ahash::RandomState;
use proc_macro2::{Span, TokenStream};
use quote::{format_ident, quote, ToTokens};
use std::fmt::Display;
use std::ops::Range;
use std::str::FromStr;
use syn::{parse2, Expr, Lit, LitInt, LitStr};

struct ProcessedEntry<K> {
    base: Entry,
    parsed_key: K,
    hash_code: u64,
}

impl<K> ToTokens for ProcessedEntry<K> {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        self.base.to_tokens(tokens);
    }
}

pub struct Output {
    pub ctor: TokenStream,
    pub type_sig: TokenStream,
    pub constant: bool,
}

impl Output {
    const fn new(ctor: TokenStream, type_sig: TokenStream, constant: bool) -> Self {
        Self {
            ctor,
            type_sig,
            constant,
        }
    }
}

#[derive(Clone, Copy)]
pub enum KeyKind {
    Scalar,
    String,
    Hashed,
    Ordered,
}

struct Generator {
    entries: Vec<Entry>,
    seeds: (u64, u64, u64, u64),
    as_set: bool,
    key_type: TokenStream,
    value_type: TokenStream,
}

pub fn generate(
    payload: Payload,
    seeds: (u64, u64, u64, u64),
    as_set: bool,
    key_type: TokenStream,
    value_type: TokenStream,
    key_kind: KeyKind,
) -> syn::Result<Output> {
    let mut gen = Generator {
        entries: vec![],
        seeds,
        as_set,
        key_type,
        value_type,
    };

    match payload {
        Payload::InlineEntries(entries) => {
            gen.entries = entries;

            if gen.entries.is_empty() {
                return Err(syn::Error::new(
                    Span::call_site(),
                    "no collection entries supplied",
                ));
            }

            match key_kind {
                KeyKind::Scalar => match &gen.entries[0].key {
                    Expr::Lit(expr) => match &expr.lit {
                        Lit::Int(expr) => match expr.suffix() {
                            "i8" => gen.handle_literal_scalar_keys::<i8>(),
                            "i16" => gen.handle_literal_scalar_keys::<i16>(),
                            "i32" | "" => gen.handle_literal_scalar_keys::<i32>(),
                            "i64" => gen.handle_literal_scalar_keys::<i64>(),
                            "isize" => gen.handle_literal_scalar_keys::<isize>(),
                            "u8" => gen.handle_literal_scalar_keys::<u8>(),
                            "u16" => gen.handle_literal_scalar_keys::<u16>(),
                            "u32" => gen.handle_literal_scalar_keys::<u32>(),
                            "u64" => gen.handle_literal_scalar_keys::<u64>(),
                            "usize" => gen.handle_literal_scalar_keys::<usize>(),
                            _ => Err(syn::Error::new_spanned(
                                expr,
                                format!("unknown suffix {} for scalar value", expr.suffix()),
                            )),
                        },
                        _ => Err(syn::Error::new_spanned(
                            expr,
                            "invalid literal, expecting an integer value",
                        )),
                    },
                    _ => gen.handle_non_literal_scalar_keys(),
                },

                KeyKind::String => match &gen.entries[0].key {
                    Expr::Lit(expr) => match &expr.lit {
                        Lit::Str(_) => gen.handle_literal_string_keys(),
                        _ => Err(syn::Error::new_spanned(
                            expr,
                            "invalid literal, expecting a string value",
                        )),
                    },
                    _ => gen.handle_non_literal_hashed_keys(),
                },
                KeyKind::Hashed => gen.handle_non_literal_hashed_keys(),
                KeyKind::Ordered => gen.handle_non_literal_ordered_keys(),
            }
        }

        Payload::Vector(expr) => match key_kind {
            KeyKind::Scalar => Ok(gen.emit_facade(&expr, "Scalar")),
            KeyKind::String => Ok(gen.emit_facade(&expr, "String")),
            KeyKind::Hashed => Ok(gen.emit_facade(&expr, "Hash")),
            KeyKind::Ordered => Ok(gen.emit_facade(&expr, "Ordered")),
        },
    }
}

impl Generator {
    fn handle_literal_scalar_keys<K>(self) -> syn::Result<Output>
    where
        K: Scalar + Ord + FromStr,
        K::Err: Display,
    {
        let mut processed_entries = Vec::with_capacity(self.entries.len());

        let hasher = PassthroughHasher::new();
        for entry in &self.entries {
            let k = parse2::<LitInt>(entry.key.to_token_stream())?.base10_parse::<K>()?;
            processed_entries.push(ProcessedEntry {
                base: entry.clone(),
                parsed_key: k,
                hash_code: hasher.hash(&k),
            });
        }

        processed_entries.sort_by_key(|x| x.parsed_key);
        dedup_by_keep_last(&mut processed_entries, |x, y| x.parsed_key == y.parsed_key);

        let analysis = analyze_scalar_keys(processed_entries.iter().map(|x| x.parsed_key));
        Ok(match analysis {
            ScalarKeyAnalysisResult::DenseRange => {
                self.emit_inline_dense_scalar_lookup(&processed_entries)
            }
            ScalarKeyAnalysisResult::SparseRange => {
                self.emit_inline_sparse_scalar_lookup(&processed_entries)
            }
            ScalarKeyAnalysisResult::General => {
                if processed_entries.len() < 3 {
                    self.emit_inline_scan(&processed_entries)
                } else {
                    self.emit_inline_hash_with_passthrough(processed_entries)
                }
            }
        })
    }

    fn handle_literal_string_keys(self) -> syn::Result<Output> {
        let bh = RandomState::with_seeds(self.seeds.0, self.seeds.1, self.seeds.2, self.seeds.3);
        let mut processed_entries = Vec::with_capacity(self.entries.len());

        for entry in &self.entries {
            let ls = parse2::<LitStr>(entry.key.to_token_stream())?;
            processed_entries.push(ProcessedEntry {
                base: entry.clone(),
                parsed_key: ls.value(),
                hash_code: bh.hash_one(ls.value()),
            });
        }

        processed_entries.sort_by(|x, y| x.parsed_key.cmp(&y.parsed_key));
        dedup_by_keep_last(&mut processed_entries, |x, y| x.parsed_key == y.parsed_key);

        if processed_entries.len() < 3 {
            return Ok(self.emit_inline_scan(&processed_entries));
        } else if processed_entries.len() < 5 {
            return Ok(self.emit_inline_ordered_scan(&processed_entries));
        }

        let analysis = analyze_slice_keys(
            processed_entries.iter().map(|x| x.parsed_key.as_bytes()),
            &bh,
        );

        Ok(match analysis {
            SliceKeyAnalysisResult::LeftHandSubslice(range) => {
                let hasher = LeftRangeHasher::new(bh, range.clone());
                for entry in &mut processed_entries {
                    entry.hash_code = hasher.hash(&entry.parsed_key);
                }

                self.emit_inline_hash_with_range(
                    processed_entries,
                    range,
                    &quote!(InlineLeftRangeHasher),
                )
            }

            SliceKeyAnalysisResult::RightHandSubslice(range) => {
                let hasher = RightRangeHasher::new(bh, range.clone());
                for entry in &mut processed_entries {
                    entry.hash_code = hasher.hash(&entry.parsed_key);
                }

                self.emit_inline_hash_with_range(
                    processed_entries,
                    range,
                    &quote!(InlineRightRangeHasher),
                )
            }

            SliceKeyAnalysisResult::Length => {
                let hasher = PassthroughHasher::new();
                for entry in &mut processed_entries {
                    entry.hash_code = hasher.hash(&entry.parsed_key);
                }

                self.emit_inline_hash_with_passthrough(processed_entries)
            }

            SliceKeyAnalysisResult::General => self.emit_inline_hash_with_bridge(processed_entries),
        })
    }

    fn handle_non_literal_scalar_keys(mut self) -> syn::Result<Output> {
        slow_dedup_by_keep_last(&mut self.entries, |x, y| x.key == y.key);

        let mut processed_entries = Vec::with_capacity(self.entries.len());

        for entry in &self.entries {
            processed_entries.push(ProcessedEntry {
                base: entry.clone(),
                parsed_key: (),
                hash_code: 0,
            });
        }

        if processed_entries.len() < 3 {
            Ok(self.emit_inline_scan(&processed_entries))
        } else {
            Ok(self.emit_hash_with_bridge(&processed_entries))
        }
    }

    fn handle_non_literal_hashed_keys(mut self) -> syn::Result<Output> {
        slow_dedup_by_keep_last(&mut self.entries, |x, y| x.key == y.key);

        let mut processed_entries = Vec::with_capacity(self.entries.len());

        for entry in &self.entries {
            processed_entries.push(ProcessedEntry {
                base: entry.clone(),
                parsed_key: (),
                hash_code: 0,
            });
        }

        if processed_entries.len() < 3 {
            Ok(self.emit_inline_scan(&processed_entries))
        } else {
            Ok(self.emit_hash_with_bridge(&processed_entries))
        }
    }

    fn handle_non_literal_ordered_keys(mut self) -> syn::Result<Output> {
        slow_dedup_by_keep_last(&mut self.entries, |x, y| x.key == y.key);

        let mut processed_entries = Vec::with_capacity(self.entries.len());

        for entry in &self.entries {
            processed_entries.push(ProcessedEntry {
                base: entry.clone(),
                parsed_key: (),
                hash_code: 0,
            });
        }

        if processed_entries.len() < 3 {
            Ok(self.emit_inline_scan(&processed_entries))
        } else if processed_entries.len() < 6 {
            Ok(self.emit_ordered_scan(&processed_entries))
        } else {
            Ok(self.emit_binary_search(&processed_entries))
        }
    }

    fn emit_inline_scan<K>(self, entries: &[ProcessedEntry<K>]) -> Output {
        let key_type = &self.key_type;
        let value_type = &self.value_type;
        let len = proc_macro2::Literal::usize_unsuffixed(entries.len());

        let mut ty = quote!(::frozen_collections::inline_maps::InlineScanMap);
        let mut generics = quote!(<#key_type, #value_type, #len>);
        let mut type_sig = quote!(#ty::#generics);
        let mut ctor = quote!(#type_sig::new_raw([
            #(
                #entries,
            )*
        ]));

        if self.as_set {
            ty = quote!(::frozen_collections::inline_sets::InlineScanSet);
            generics = quote!(<#key_type, #len>);
            type_sig = quote!(#ty::#generics);
            ctor = quote!(#type_sig::new(#ctor));
        }

        Output::new(ctor, type_sig, true)
    }

    fn emit_inline_ordered_scan<K>(self, entries: &[ProcessedEntry<K>]) -> Output {
        let key_type = &self.key_type;
        let value_type = &self.value_type;
        let len = proc_macro2::Literal::usize_unsuffixed(entries.len());

        let mut ty = quote!(::frozen_collections::inline_maps::InlineOrderedScanMap);
        let mut generics = quote!(<#key_type, #value_type, #len>);
        let mut type_sig = quote!(#ty::#generics);
        let mut ctor = quote!(#type_sig::new_raw([
            #(
                #entries,
            )*
        ]));

        if self.as_set {
            ty = quote!(::frozen_collections::inline_sets::InlineOrderedScanSet);
            generics = quote!(<#key_type, #len>);
            type_sig = quote!(#ty::#generics);
            ctor = quote!(#type_sig::new(#ctor));
        }

        Output::new(ctor, type_sig, true)
    }

    fn emit_inline_dense_scalar_lookup<K>(self, entries: &[ProcessedEntry<K>]) -> Output {
        let key_type = &self.key_type;
        let value_type = &self.value_type;
        let len = proc_macro2::Literal::usize_unsuffixed(entries.len());
        let min_key = &entries[0].base.key;
        let max_key = &entries[entries.len() - 1].base.key;

        let mut ty = quote!(::frozen_collections::inline_maps::InlineDenseScalarLookupMap);
        let mut generics = quote!(<#key_type, #value_type, #len>);
        let mut type_sig = quote!(#ty::#generics);
        let mut ctor = quote!(#type_sig::new_raw([
            #(
                #entries,
            )*
        ], #min_key, #max_key));

        if self.as_set {
            ty = quote!(::frozen_collections::inline_sets::InlineDenseScalarLookupSet);
            generics = quote!(<#key_type, #len>);
            type_sig = quote!(#ty::#generics);
            ctor = quote!(#type_sig::new(#ctor));
        }

        Output::new(ctor, type_sig, true)
    }

    fn emit_inline_sparse_scalar_lookup<K>(self, entries: &[ProcessedEntry<K>]) -> Output
    where
        K: Scalar + Ord + FromStr,
    {
        let min_key = &entries[0].parsed_key.index();
        let max_key = &entries[entries.len() - 1].parsed_key.index();

        let count = max_key - min_key + 1;
        let mut lookup = vec![0; count];

        for (i, entry) in entries.iter().enumerate() {
            let index_in_lookup = entry.parsed_key.index() - min_key;
            let index_in_entries = i + 1;
            lookup[index_in_lookup] = index_in_entries;
        }

        let key_type = &self.key_type;
        let value_type = &self.value_type;
        let len = proc_macro2::Literal::usize_unsuffixed(entries.len());
        let magnitude = collection_magnitude(count);
        let lookup = lookup
            .iter()
            .map(|x| proc_macro2::Literal::usize_unsuffixed(*x));
        let lookup_len = proc_macro2::Literal::usize_unsuffixed(lookup.len());

        let mut ty = quote!(::frozen_collections::inline_maps::InlineSparseScalarLookupMap);
        let mut generics = quote!(<#key_type, #value_type, #len, #lookup_len, #magnitude>);
        let mut type_sig = quote!(#ty::#generics);
        let mut ctor = quote!(#type_sig::new_raw([
            #(
                #entries,
            )*
        ],
        [
            #(
                #lookup,
            )*
        ], #min_key, #max_key));

        if self.as_set {
            ty = quote!(::frozen_collections::inline_sets::InlineSparseScalarLookupSet);
            generics = quote!(<#key_type, #len, #lookup_len, #magnitude>);
            type_sig = quote!(#ty::#generics);
            ctor = quote!(#type_sig::new(#ctor));
        }

        Output::new(ctor, type_sig, true)
    }

    fn emit_inline_hash_with_bridge<K>(self, entries: Vec<ProcessedEntry<K>>) -> Output {
        let key_type = &self.key_type;
        let value_type = &self.value_type;
        let len = proc_macro2::Literal::usize_unsuffixed(entries.len());
        let (ht, magnitude, num_slots) = self.hash_table(entries);
        let (s0, s1, s2, s3) = self.seeds;

        let mut ty = quote!(::frozen_collections::inline_maps::InlineHashMap);
        let mut generics = quote!(<#key_type, #value_type, #len, #num_slots, #magnitude, ::frozen_collections::hashers::BridgeHasher<::frozen_collections::ahash::RandomState>>);
        let mut type_sig = quote!(#ty::#generics);
        let mut ctor = quote!(#type_sig::new_raw(#ht, ::frozen_collections::hashers::BridgeHasher::new(::frozen_collections::ahash::RandomState::with_seeds(#s0, #s1, #s2, #s3))));

        if self.as_set {
            ty = quote!(::frozen_collections::inline_sets::InlineHashSet);
            generics = quote!(<#key_type, #len, #num_slots, #magnitude, ::frozen_collections::hashers::BridgeHasher<::frozen_collections::ahash::RandomState>>);
            type_sig = quote!(#ty::#generics);
            ctor = quote!(#type_sig::new(#ctor));
        }

        Output::new(ctor, type_sig, true)
    }

    fn emit_inline_hash_with_range<K>(
        self,
        entries: Vec<ProcessedEntry<K>>,
        hash_range: Range<usize>,
        hasher_type: &TokenStream,
    ) -> Output {
        let key_type = &self.key_type;
        let value_type = &self.value_type;
        let len = proc_macro2::Literal::usize_unsuffixed(entries.len());
        let (ht, magnitude, num_slots) = self.hash_table(entries);
        let (s0, s1, s2, s3) = self.seeds;
        let range_start = proc_macro2::Literal::usize_unsuffixed(hash_range.start);
        let range_end = proc_macro2::Literal::usize_unsuffixed(hash_range.end);

        let mut ty = quote!(::frozen_collections::inline_maps::InlineHashMap);
        let mut generics = quote!(<#key_type, #value_type, #len, #num_slots, #magnitude, ::frozen_collections::hashers::#hasher_type<#range_start, #range_end, ::frozen_collections::ahash::RandomState>>);
        let mut type_sig = quote!(#ty::#generics);
        let mut ctor = quote!(#type_sig::new_raw(#ht, ::frozen_collections::hashers::#hasher_type::new(::frozen_collections::ahash::RandomState::with_seeds(#s0, #s1, #s2, #s3))));

        if self.as_set {
            ty = quote!(::frozen_collections::inline_sets::InlineHashSet);
            generics = quote!(<#key_type, #len, #num_slots, #magnitude, ::frozen_collections::hashers::#hasher_type<#range_start, #range_end, ::frozen_collections::ahash::RandomState>>);
            type_sig = quote!(#ty::#generics);
            ctor = quote!(#type_sig::new(#ctor));
        }

        Output::new(ctor, type_sig, true)
    }

    fn emit_inline_hash_with_passthrough<K>(self, entries: Vec<ProcessedEntry<K>>) -> Output {
        let key_type = &self.key_type;
        let value_type = &self.value_type;
        let len = proc_macro2::Literal::usize_unsuffixed(entries.len());
        let (ht, magnitude, num_slots) = self.hash_table(entries);

        let mut ty = quote!(::frozen_collections::inline_maps::InlineHashMap);
        let mut generics = quote!(<#key_type, #value_type, #len, #num_slots, #magnitude, ::frozen_collections::hashers::PassthroughHasher>);
        let mut type_sig = quote!(#ty::#generics);
        let mut ctor = quote!(#type_sig::new_raw(#ht, ::frozen_collections::hashers::PassthroughHasher::new()));

        if self.as_set {
            ty = quote!(::frozen_collections::inline_sets::InlineHashSet);
            generics = quote!(<#key_type, #len, #num_slots, #magnitude, ::frozen_collections::hashers::PassthroughHasher>);
            type_sig = quote!(#ty::#generics);
            ctor = quote!(#type_sig::new(#ctor));
        }

        Output::new(ctor, type_sig, true)
    }

    fn emit_hash_with_bridge<K>(self, entries: &[ProcessedEntry<K>]) -> Output {
        let key_type = &self.key_type;
        let value_type = &self.value_type;

        let mut ty = quote!(::frozen_collections::maps::HashMap);
        let magnitude = collection_magnitude(entries.len());
        let mut generics = quote!(<#key_type, #value_type, #magnitude>);
        let mut type_sig = quote!(#ty::#generics);
        let mut ctor = quote!(#type_sig::new(vec![
        #(
            #entries,
        )*
        ], ::frozen_collections::hashers::BridgeHasher::new(::frozen_collections::ahash::RandomState::new())).unwrap());

        if self.as_set {
            ty = quote!(::frozen_collections::sets::HashSet);
            generics = quote!(<#key_type, #magnitude>);
            type_sig = quote!(#ty::#generics);
            ctor = quote!(#type_sig::new(#ctor));
        }

        Output::new(ctor, type_sig, false)
    }

    fn emit_ordered_scan<K>(self, entries: &[ProcessedEntry<K>]) -> Output {
        let key_type = &self.key_type;
        let value_type = &self.value_type;

        let mut ty = quote!(::frozen_collections::maps::OrderedScanMap);
        let mut generics = quote!(<#key_type, #value_type>);
        let mut type_sig = quote!(#ty::#generics);
        let mut ctor = quote!(#type_sig::new(vec![
        #(
            #entries,
        )*
        ]));

        if self.as_set {
            ty = quote!(::frozen_collections::sets::OrderedScanSet);
            generics = quote!(<#key_type>);
            type_sig = quote!(#ty::#generics);
            ctor = quote!(#type_sig::new(#ctor));
        }

        Output::new(ctor, type_sig, false)
    }

    fn emit_binary_search<K>(self, entries: &[ProcessedEntry<K>]) -> Output {
        let key_type = &self.key_type;
        let value_type = &self.value_type;

        let mut ty = quote!(::frozen_collections::maps::BinarySearchMap);
        let mut generics = quote!(<#key_type, #value_type>);
        let mut type_sig = quote!(#ty::#generics);
        let mut ctor = quote!(#type_sig::new(vec![
        #(
            #entries,
        )*
        ]));

        if self.as_set {
            ty = quote!(::frozen_collections::sets::BinarySearchSet);
            generics = quote!(<#key_type>);
            type_sig = quote!(#ty::#generics);
            ctor = quote!(#type_sig::new(#ctor));
        }

        Output::new(ctor, type_sig, false)
    }

    fn emit_facade(self, expr: &Expr, variety: &str) -> Output {
        let key_type = &self.key_type;
        let value_type = &self.value_type;

        let mut type_name = format_ident!("Facade{}Map", variety);
        let mut ty = quote!(::frozen_collections::facade_maps::#type_name);

        let mut type_sig = if variety == "String" {
            quote!(#ty::<#value_type>)
        } else {
            quote!(#ty::<#key_type, #value_type>)
        };

        let converted_expr = if self.as_set {
            quote!(#expr.into_iter().map(|x| (x, ())).collect())
        } else {
            quote!(#expr)
        };

        let mut ctor = if variety == "Hash" {
            quote!(#type_sig::new(#converted_expr, ::frozen_collections::hashers::BridgeHasher::new(::frozen_collections::ahash::RandomState::new())))
        } else if variety == "String" {
            quote!(#type_sig::new(#converted_expr, ::frozen_collections::ahash::RandomState::new()))
        } else {
            quote!(#type_sig::new(#converted_expr))
        };

        if self.as_set {
            type_name = format_ident!("Facade{}Set", variety);
            ty = quote!(::frozen_collections::facade_sets::#type_name);

            type_sig = if variety == "String" {
                quote!(#ty)
            } else {
                quote!(#ty::<#key_type>)
            };

            ctor = quote!(#type_sig::new(#ctor));
        }

        Output::new(ctor, type_sig, false)
    }

    fn hash_table<K>(
        &self,
        entries: Vec<ProcessedEntry<K>>,
    ) -> (TokenStream, TokenStream, TokenStream) {
        let key_type = &self.key_type;
        let value_type = &self.value_type;
        let len = proc_macro2::Literal::usize_unsuffixed(entries.len());

        let ht = HashTable::<_, LargeCollection>::new(entries, |x| x.hash_code).unwrap();
        let slots = ht.slots;
        let num_slots = proc_macro2::Literal::usize_unsuffixed(slots.len());
        let entries = ht.entries;
        let magnitude = collection_magnitude(slots.len());

        (
            quote!(::frozen_collections::hash_tables::InlineHashTable::<(#key_type, #value_type), #len, #num_slots, #magnitude>::new_raw(
                [
                #(
                    #slots,
                )*
                ],
                [
                #(
                    #entries,
                )*
                ],
            )),
            magnitude,
            quote!(#num_slots),
        )
    }
}

fn collection_magnitude(len: usize) -> TokenStream {
    if len <= SmallCollection::MAX_CAPACITY {
        quote!(::frozen_collections::SmallCollection)
    } else if len <= MediumCollection::MAX_CAPACITY {
        quote!(::frozen_collections::MediumCollection)
    } else {
        quote!(::frozen_collections::LargeCollection)
    }
}
