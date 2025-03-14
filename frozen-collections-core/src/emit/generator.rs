#![allow(clippy::needless_pass_by_value)]
#![allow(unexpected_cfgs)]

use crate::emit::collection_entry::CollectionEntry;
use crate::hash_tables::HashTable;
use crate::hashers::BridgeHasher;
use crate::traits::{
    CollectionMagnitude, Hasher, LargeCollection, MediumCollection, Scalar, SmallCollection,
};
use alloc::vec;
use alloc::vec::Vec;
use const_random::const_random;
use core::hash::Hash;
use core::ops::Range;
use foldhash::fast::FixedState;
use proc_macro2::{Literal, TokenStream};
use quote::quote;
use syn::{Type, parse_quote};

#[derive(Debug)]
pub struct Generator {
    key_type: Type,
    value_type: Type,
    pub seed: u64,
    len: Literal,
    gen_set: bool,
}

pub struct Output {
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
            seed: const_random!(u64),
            len: Literal::usize_unsuffixed(len),
            gen_set: value_type.is_none(),
        }
    }

    #[cfg(feature = "disabled")]
    pub fn gen_fz_hash<K>(self, entries: Vec<CollectionEntry<K>>) -> Output {
        let key_type = &self.key_type;
        let value_type = &self.value_type;

        let mut ty = quote!(::frozen_collections::FzHashMap);
        let mut generics = quote!(<#key_type, #len>);
        let mut type_sig = quote!(#ty::#generics);
        let mut ctor = quote!(#type_sig::new(vec![
            #(
                #entries,
            )*
        ]));

        if self.gen_set {
            ty = quote!(::frozen_collections::FzHashSet);
            generics = quote!(<#key_type>);
            type_sig = quote!(#ty::#generics);
            ctor = quote!(#type_sig::from(#ctor));
        }

        Output { ctor, type_sig }
    }

    #[cfg(feature = "disabled")]
    pub fn gen_fz_ordered<K>(self, entries: Vec<CollectionEntry<K>>) -> Output {
        let key_type = &self.key_type;
        let value_type = &self.value_type;

        let mut ty = quote!(::frozen_collections::FzOrderedMap);
        let mut generics = quote!(<#key_type, #value_type>);
        let mut type_sig = quote!(#ty::generics);
        let mut ctor = quote!(#type_sig::new(vec![
            #(
                #entries,
            )*
        ]));

        if self.gen_set {
            ty = quote!(::frozen_collections::FzOrderedSet);
            generics = quote!(<#key_type>);
            type_sig = quote!(#ty::#generics);
            ctor = quote!(#type_sig::from(#ctor));
        }

        Output { ctor, type_sig }
    }

    #[cfg(feature = "macros")]
    pub fn gen_fz_scalar<K>(self, entries: Vec<CollectionEntry<K>>) -> Output {
        let key_type = &self.key_type;
        let value_type = &self.value_type;

        let mut ty = quote!(::frozen_collections::FzScalarMap);
        let mut generics = quote!(<#key_type, #value_type>);
        let mut type_sig = quote!(#ty::#generics);
        let mut ctor = quote!(#type_sig::new(vec![
            #(
                #entries,
            )*
        ]));

        if self.gen_set {
            ty = quote!(::frozen_collections::FzScalarSet);
            generics = quote!(<#key_type>);
            type_sig = quote!(#ty::#generics);
            ctor = quote!(#type_sig::from(#ctor));
        }

        Output { ctor, type_sig }
    }

    #[cfg(feature = "macros")]
    pub fn gen_fz_string<K>(self, entries: Vec<CollectionEntry<K>>) -> Output {
        let key_type = &self.key_type;
        let value_type = &self.value_type;

        let mut ty = quote!(::frozen_collections::FzStringMap);
        let mut generics = quote!(<#key_type, #value_type>);
        let mut type_sig = quote!(#ty::#generics);
        let mut ctor = quote!(#type_sig::new(vec![
            #(
                #entries,
            )*
        ]));

        if self.gen_set {
            ty = quote!(::frozen_collections::FzStringSet);
            generics = quote!(<#key_type>);
            type_sig = quote!(#ty::#generics);
            ctor = quote!(#type_sig::from(#ctor));
        }

        Output { ctor, type_sig }
    }

    #[cfg(feature = "disabled")]
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
            &BridgeHasher::new(FixedState::with_seed(self.seed)),
        );
        let seed = self.seed;

        let mut ty = quote!(::frozen_collections::inline_maps::InlineHashMap);
        let mut generics = quote!(<#key_type, #value_type, #len, #num_slots, #magnitude, ::frozen_collections::hashers::BridgeHasher<::frozen_collections::foldhash::FixedState>>);
        let mut type_sig = quote!(#ty::#generics);
        let mut ctor = quote!(#type_sig::new_raw(#ht, ::frozen_collections::hashers::BridgeHasher::new(::frozen_collections::foldhash::FixedState::with_seed(#seed))));

        if self.gen_set {
            ty = quote!(::frozen_collections::inline_sets::InlineHashSet);
            generics = quote!(<#key_type, #len, #num_slots, #magnitude, ::frozen_collections::hashers::BridgeHasher<::frozen_collections::foldhash::FixedState>>);
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
        let seed = self.seed;
        let range_start = Literal::usize_unsuffixed(hash_range.start);
        let range_end = Literal::usize_unsuffixed(hash_range.end);

        let mut ty = quote!(::frozen_collections::inline_maps::InlineHashMap);
        let mut generics = quote!(<#key_type, #value_type, #len, #num_slots, #magnitude, ::frozen_collections::hashers::#hasher_type<#range_start, #range_end, ::frozen_collections::foldhash::FixedState>>);
        let mut type_sig = quote!(#ty::#generics);
        let mut ctor = quote!(#type_sig::new_raw(#ht, ::frozen_collections::hashers::#hasher_type::new(::frozen_collections::foldhash::FixedState::with_seed(#seed))));

        if self.gen_set {
            ty = quote!(::frozen_collections::inline_sets::InlineHashSet);
            generics = quote!(<#key_type, #len, #num_slots, #magnitude, ::frozen_collections::hashers::#hasher_type<#range_start, #range_end, ::frozen_collections::foldhash::FixedState>>);
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

    #[cfg(feature = "disabled")]
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

    #[cfg(feature = "disabled")]
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
