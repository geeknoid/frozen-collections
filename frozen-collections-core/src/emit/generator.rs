#![allow(clippy::redundant_pub_crate)]
#![allow(clippy::needless_pass_by_value)]

use crate::emit::collection_entry::CollectionEntry;
use crate::hash_tables::HashTable;
use crate::hashers::BridgeHasher;
use crate::traits::{
    CollectionMagnitude, Hasher, LargeCollection, MediumCollection, Scalar, SmallCollection,
};
use crate::utils::pick_compile_time_random_seeds;
use alloc::vec;
use alloc::vec::Vec;
use core::hash::Hash;
use core::ops::Range;
use proc_macro2::{Literal, TokenStream};
use quote::quote;
use syn::{parse_quote, Type};

#[derive(Debug)]
pub(crate) struct Generator {
    key_type: Type,
    value_type: Type,
    pub seeds: (u64, u64, u64, u64),
    len: Literal,
    gen_set: bool,
}

pub(crate) struct Output {
    pub ctor: TokenStream,
    pub type_sig: TokenStream,
}

impl Generator {
    #[allow(clippy::option_if_let_else)]
    pub fn new(key_type: &Type, value_type: Option<&Type>, len: usize) -> Self {
        Self {
            key_type: (*key_type).clone(),
            value_type: if let Some(value_type) = value_type {
                value_type.clone()
            } else {
                parse_quote!(())
            },
            seeds: pick_compile_time_random_seeds(),
            len: Literal::usize_unsuffixed(len),
            gen_set: value_type.is_none(),
        }
    }

    /*
        pub fn gen_facade_hash<K>(self, entries: Vec<CollectionEntry<K>>) -> Output {
            let key_type = &self.key_type;
            let value_type = &self.value_type;

            let mut ty = quote!(::frozen_collections::FzHashMap);
            let mut type_sig = quote!(#ty::<#key_type, #value_type>);
            let mut ctor = quote!(#type_sig::new(vec![
                #(
                    #entries,
                )*
            ]));

            if self.gen_set {
                ty = quote!(::frozen_collections::FzHashSet);
                type_sig = quote!(#ty);
                ctor = quote!(#type_sig::from(#ctor));
            }

            Output { ctor, type_sig }
        }

        pub fn gen_facade_ordered<K>(self, entries: Vec<CollectionEntry<K>>) -> Output {
            let key_type = &self.key_type;
            let value_type = &self.value_type;

            let mut ty = quote!(::frozen_collections::FzOrderedMap);
            let mut type_sig = quote!(#ty::<#key_type, #value_type>);
            let mut ctor = quote!(#type_sig::new(vec![
                #(
                    #entries,
                )*
            ]));

            if self.gen_set {
                ty = quote!(::frozen_collections::FzOrderedSet);
                type_sig = quote!(#ty);
                ctor = quote!(#type_sig::from(#ctor));
            }

            Output { ctor, type_sig }
        }
    */

    #[cfg(feature = "macros")]
    pub fn gen_facade_scalar<K>(self, entries: Vec<CollectionEntry<K>>) -> Output {
        let key_type = &self.key_type;
        let value_type = &self.value_type;

        let mut ty = quote!(::frozen_collections::FzScalarMap);
        let mut type_sig = quote!(#ty::<#key_type, #value_type>);
        let mut ctor = quote!(#type_sig::new(vec![
            #(
                #entries,
            )*
        ]));

        if self.gen_set {
            ty = quote!(::frozen_collections::FzScalarSet);
            type_sig = quote!(#ty);
            ctor = quote!(#type_sig::from(#ctor));
        }

        Output { ctor, type_sig }
    }

    #[cfg(feature = "macros")]
    pub fn gen_facade_string<K>(self, entries: Vec<CollectionEntry<K>>) -> Output {
        let key_type = &self.key_type;
        let value_type = &self.value_type;

        let mut ty = quote!(::frozen_collections::FzStringMap);
        let mut type_sig = quote!(#ty::<#key_type, #value_type>);
        let mut ctor = quote!(#type_sig::new(vec![
            #(
                #entries,
            )*
        ]));

        if self.gen_set {
            ty = quote!(::frozen_collections::FzStringSet);
            type_sig = quote!(#ty);
            ctor = quote!(#type_sig::from(#ctor));
        }

        Output { ctor, type_sig }
    }

    #[cfg(feature = "emit")]
    pub fn gen_inline_binary_search<K>(&self, sorted_entries: Vec<CollectionEntry<K>>) -> Output {
        let key_type = &self.key_type;
        let value_type = &self.value_type;
        let len = &self.len;

        let mut ty = quote!(::frozen_collections::inline_maps::InlineBinarySearchMap);
        let mut generics = quote!(<#key_type, #value_type, #len>);
        let mut type_sig = quote!(#ty::#generics);
        let mut ctor = quote!(#type_sig::new_raw([
            #(
                #sorted_entries,
            )*
        ]));

        if self.gen_set {
            ty = quote!(::frozen_collections::inline_sets::InlineBinarySearchSet);
            generics = quote!(<#key_type, #len>);
            type_sig = quote!(#ty::#generics);
            ctor = quote!(#type_sig::new(#ctor));
        }

        Output { ctor, type_sig }
    }

    pub fn gen_inline_dense_scalar_lookup<K>(
        &self,
        sorted_entries: Vec<CollectionEntry<K>>,
    ) -> Output
    where
        K: Scalar,
    {
        let key_type = &self.key_type;
        let value_type = &self.value_type;
        let len = &self.len;
        let min_key = &sorted_entries[0].key.index();
        let max_key = &sorted_entries[sorted_entries.len() - 1].key.index();

        let mut ty = quote!(::frozen_collections::inline_maps::InlineDenseScalarLookupMap);
        let mut generics = quote!(<#key_type, #value_type, #len>);
        let mut type_sig = quote!(#ty::#generics);
        let mut ctor = quote!(#type_sig::new_raw([
            #(
                #sorted_entries,
            )*
        ], #min_key, #max_key));

        if self.gen_set {
            ty = quote!(::frozen_collections::inline_sets::InlineDenseScalarLookupSet);
            generics = quote!(<#key_type, #len>);
            type_sig = quote!(#ty::#generics);
            ctor = quote!(#type_sig::new(#ctor));
        }

        Output { ctor, type_sig }
    }

    #[cfg(feature = "emit")]
    pub fn gen_inline_eytzinger_search<K>(
        &self,
        sorted_entries: Vec<CollectionEntry<K>>,
    ) -> Output {
        let key_type = &self.key_type;
        let value_type = &self.value_type;
        let len = &self.len;

        let mut ty = quote!(::frozen_collections::inline_maps::InlineEytzingerSearchMap);
        let mut generics = quote!(<#key_type, #value_type, #len>);
        let mut type_sig = quote!(#ty::#generics);
        let mut ctor = quote!(#type_sig::new_raw([
            #(
                #sorted_entries,
            )*
        ]));

        if self.gen_set {
            ty = quote!(::frozen_collections::inline_sets::InlineEytzingerSearchSet);
            generics = quote!(<#key_type, #len>);
            type_sig = quote!(#ty::#generics);
            ctor = quote!(#type_sig::new(#ctor));
        }

        Output { ctor, type_sig }
    }

    pub fn gen_inline_hash_with_bridge<K>(&self, entries: Vec<CollectionEntry<K>>) -> Output
    where
        K: Hash + Eq,
    {
        let key_type = &self.key_type;
        let value_type = &self.value_type;
        let len = &self.len;
        let (ht, magnitude, num_slots) = self.gen_inline_hash_table_components(
            entries,
            &BridgeHasher::new(ahash::RandomState::with_seeds(
                self.seeds.0,
                self.seeds.1,
                self.seeds.2,
                self.seeds.3,
            )),
        );
        let (s0, s1, s2, s3) = (self.seeds.0, self.seeds.1, self.seeds.2, self.seeds.3);

        let mut ty = quote!(::frozen_collections::inline_maps::InlineHashMap);
        let mut generics = quote!(<#key_type, #value_type, #len, #num_slots, #magnitude, ::frozen_collections::hashers::BridgeHasher<::frozen_collections::ahash::RandomState>>);
        let mut type_sig = quote!(#ty::#generics);
        let mut ctor = quote!(#type_sig::new_raw(#ht, ::frozen_collections::hashers::BridgeHasher::new(::frozen_collections::ahash::RandomState::with_seeds(#s0, #s1, #s2, #s3))));

        if self.gen_set {
            ty = quote!(::frozen_collections::inline_sets::InlineHashSet);
            generics = quote!(<#key_type, #len, #num_slots, #magnitude, ::frozen_collections::hashers::BridgeHasher<::frozen_collections::ahash::RandomState>>);
            type_sig = quote!(#ty::#generics);
            ctor = quote!(#type_sig::new(#ctor));
        }

        Output { ctor, type_sig }
    }

    pub fn gen_inline_hash_with_passthrough<K, H>(
        &self,
        entries: Vec<CollectionEntry<K>>,
        hasher: &H,
    ) -> Output
    where
        H: Hasher<K>,
    {
        let key_type = &self.key_type;
        let value_type = &self.value_type;
        let len = &self.len;
        let (ht, magnitude, num_slots) = self.gen_inline_hash_table_components(entries, hasher);

        let mut ty = quote!(::frozen_collections::inline_maps::InlineHashMap);
        let mut generics = quote!(<#key_type, #value_type, #len, #num_slots, #magnitude, ::frozen_collections::hashers::PassthroughHasher>);
        let mut type_sig = quote!(#ty::#generics);
        let mut ctor = quote!(#type_sig::new_raw(#ht, ::frozen_collections::hashers::PassthroughHasher::new()));

        if self.gen_set {
            ty = quote!(::frozen_collections::inline_sets::InlineHashSet);
            generics = quote!(<#key_type, #len, #num_slots, #magnitude, ::frozen_collections::hashers::PassthroughHasher>);
            type_sig = quote!(#ty::#generics);
            ctor = quote!(#type_sig::new(#ctor));
        }

        Output { ctor, type_sig }
    }

    pub fn gen_inline_hash_with_range<K, H>(
        &self,
        entries: Vec<CollectionEntry<K>>,
        hash_range: Range<usize>,
        hasher_type: &TokenStream,
        hasher: &H,
    ) -> Output
    where
        H: Hasher<K>,
    {
        let key_type = &self.key_type;
        let value_type = &self.value_type;
        let len = &self.len;
        let (ht, magnitude, num_slots) = self.gen_inline_hash_table_components(entries, hasher);
        let (s0, s1, s2, s3) = self.seeds;
        let range_start = Literal::usize_unsuffixed(hash_range.start);
        let range_end = Literal::usize_unsuffixed(hash_range.end);

        let mut ty = quote!(::frozen_collections::inline_maps::InlineHashMap);
        let mut generics = quote!(<#key_type, #value_type, #len, #num_slots, #magnitude, ::frozen_collections::hashers::#hasher_type<#range_start, #range_end, ::frozen_collections::ahash::RandomState>>);
        let mut type_sig = quote!(#ty::#generics);
        let mut ctor = quote!(#type_sig::new_raw(#ht, ::frozen_collections::hashers::#hasher_type::new(::frozen_collections::ahash::RandomState::with_seeds(#s0, #s1, #s2, #s3))));

        if self.gen_set {
            ty = quote!(::frozen_collections::inline_sets::InlineHashSet);
            generics = quote!(<#key_type, #len, #num_slots, #magnitude, ::frozen_collections::hashers::#hasher_type<#range_start, #range_end, ::frozen_collections::ahash::RandomState>>);
            type_sig = quote!(#ty::#generics);
            ctor = quote!(#type_sig::new(#ctor));
        }

        Output { ctor, type_sig }
    }

    pub fn gen_inline_ordered_scan<K>(&self, sorted_entries: Vec<CollectionEntry<K>>) -> Output {
        let key_type = &self.key_type;
        let value_type = &self.value_type;
        let len = &self.len;

        let mut ty = quote!(::frozen_collections::inline_maps::InlineOrderedScanMap);
        let mut generics = quote!(<#key_type, #value_type, #len>);
        let mut type_sig = quote!(#ty::#generics);
        let mut ctor = quote!(#type_sig::new_raw([
            #(
                #sorted_entries,
            )*
        ]));

        if self.gen_set {
            ty = quote!(::frozen_collections::inline_sets::InlineOrderedScanSet);
            generics = quote!(<#key_type, #len>);
            type_sig = quote!(#ty::#generics);
            ctor = quote!(#type_sig::new(#ctor));
        }

        Output { ctor, type_sig }
    }

    pub fn gen_inline_scan<K>(&self, entries: Vec<CollectionEntry<K>>) -> Output {
        let key_type = &self.key_type;
        let value_type = &self.value_type;
        let len = &self.len;

        let mut ty = quote!(::frozen_collections::inline_maps::InlineScanMap);
        let mut generics = quote!(<#key_type, #value_type, #len>);
        let mut type_sig = quote!(#ty::#generics);
        let mut ctor = quote!(#type_sig::new_raw([
            #(
                #entries,
            )*
        ]));

        if self.gen_set {
            ty = quote!(::frozen_collections::inline_sets::InlineScanSet);
            generics = quote!(<#key_type, #len>);
            type_sig = quote!(#ty::#generics);
            ctor = quote!(#type_sig::new(#ctor));
        }

        Output { ctor, type_sig }
    }

    pub fn gen_inline_sparse_scalar_lookup<K>(
        &self,
        sorted_entries: Vec<CollectionEntry<K>>,
    ) -> Output
    where
        K: Scalar,
    {
        let min_key = &sorted_entries[0].key.index();
        let max_key = &sorted_entries[sorted_entries.len() - 1].key.index();

        let count = max_key - min_key + 1;
        let mut lookup = vec![0; count];

        for (i, entry) in sorted_entries.iter().enumerate() {
            let index_in_lookup = entry.key.index() - min_key;
            let index_in_entries = i + 1;
            lookup[index_in_lookup] = index_in_entries;
        }

        let key_type = &self.key_type;
        let value_type = &self.value_type;
        let len = &self.len;
        let magnitude = Self::collection_magnitude(count);
        let lookup = lookup.iter().map(|x| Literal::usize_unsuffixed(*x));
        let num_slots = Literal::usize_unsuffixed(lookup.len());

        let mut ty = quote!(::frozen_collections::inline_maps::InlineSparseScalarLookupMap);
        let mut generics = quote!(<#key_type, #value_type, #len, #num_slots, #magnitude>);
        let mut type_sig = quote!(#ty::#generics);
        let mut ctor = quote!(#type_sig::new_raw([
            #(
                #sorted_entries,
            )*
        ],
        [
            #(
                #lookup,
            )*
        ], #min_key, #max_key));

        if self.gen_set {
            ty = quote!(::frozen_collections::inline_sets::InlineSparseScalarLookupSet);
            generics = quote!(<#key_type, #len, #num_slots, #magnitude>);
            type_sig = quote!(#ty::#generics);
            ctor = quote!(#type_sig::new(#ctor));
        }

        Output { ctor, type_sig }
    }

    #[cfg(feature = "macros")]
    pub fn gen_binary_search<K>(&self, entries: Vec<CollectionEntry<K>>) -> Output {
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

        if self.gen_set {
            ty = quote!(::frozen_collections::sets::BinarySearchSet);
            generics = quote!(<#key_type>);
            type_sig = quote!(#ty::#generics);
            ctor = quote!(#type_sig::new(#ctor));
        }

        Output { ctor, type_sig }
    }

    #[cfg(feature = "macros")]
    pub fn gen_eytzinger_search<K>(&self, entries: Vec<CollectionEntry<K>>) -> Output {
        let key_type = &self.key_type;
        let value_type = &self.value_type;

        let mut ty = quote!(::frozen_collections::maps::EytzingerSearchMap);
        let mut generics = quote!(<#key_type, #value_type>);
        let mut type_sig = quote!(#ty::#generics);
        let mut ctor = quote!(#type_sig::new(vec![
            #(
                #entries,
            )*
        ]));

        if self.gen_set {
            ty = quote!(::frozen_collections::sets::EytzingerSearchSet);
            generics = quote!(<#key_type>);
            type_sig = quote!(#ty::#generics);
            ctor = quote!(#type_sig::new(#ctor));
        }

        Output { ctor, type_sig }
    }

    #[cfg(feature = "macros")]
    pub fn gen_hash_with_bridge<K>(&self, entries: Vec<CollectionEntry<K>>) -> Output {
        let key_type = &self.key_type;
        let value_type = &self.value_type;
        let magnitude = Self::collection_magnitude(entries.len());

        let mut ty = quote!(::frozen_collections::maps::HashMap);
        let mut generics = quote!(<#key_type, #value_type, #magnitude>);
        let mut type_sig = quote!(#ty::#generics);
        let mut ctor = quote!(#type_sig::new(vec![
            #(
                #entries,
            )*
            ]).unwrap()
        );

        if self.gen_set {
            ty = quote!(::frozen_collections::sets::HashSet);
            generics = quote!(<#key_type, #magnitude>);
            type_sig = quote!(#ty::#generics);
            ctor = quote!(#type_sig::new(#ctor));
        }

        Output { ctor, type_sig }
    }

    #[cfg(feature = "macros")]
    pub fn gen_ordered_scan<K>(&self, entries: Vec<CollectionEntry<K>>) -> Output {
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

        if self.gen_set {
            ty = quote!(::frozen_collections::sets::OrderedScanSet);
            generics = quote!(<#key_type>);
            type_sig = quote!(#ty::#generics);
            ctor = quote!(#type_sig::new(#ctor));
        }

        Output { ctor, type_sig }
    }

    #[cfg(feature = "macros")]
    pub fn gen_scan<K>(&self, entries: Vec<CollectionEntry<K>>) -> Output {
        let key_type = &self.key_type;
        let value_type = &self.value_type;

        let mut ty = quote!(::frozen_collections::maps::ScanMap);
        let mut generics = quote!(<#key_type, #value_type>);
        let mut type_sig = quote!(#ty::#generics);
        let mut ctor = quote!(#type_sig::new(vec![
            #(
                #entries,
            )*
        ]));

        if self.gen_set {
            ty = quote!(::frozen_collections::sets::ScanSet);
            generics = quote!(<#key_type>);
            type_sig = quote!(#ty::#generics);
            ctor = quote!(#type_sig::new(#ctor));
        }

        Output { ctor, type_sig }
    }

    fn gen_inline_hash_table_components<K, H>(
        &self,
        entries: Vec<CollectionEntry<K>>,
        hasher: &H,
    ) -> (TokenStream, TokenStream, Literal)
    where
        H: Hasher<K>,
    {
        let key_type = &self.key_type;
        let value_type = &self.value_type;
        let len = &self.len;

        let ht = HashTable::<_, LargeCollection>::new(entries, |x| hasher.hash(&x.key)).unwrap();
        let slots = ht.slots;
        let num_slots = Literal::usize_unsuffixed(slots.len());
        let entries = ht.entries;
        let magnitude = Self::collection_magnitude(entries.len());

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
            parse_quote!(#num_slots),
        )
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
}