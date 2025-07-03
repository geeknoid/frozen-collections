#![expect(clippy::needless_pass_by_value, reason = "Expected in syn-related code")]

use crate::emit::collection_entry::CollectionEntry;
use crate::hash_tables::HashTable;
use crate::traits::{CollectionMagnitude, Hasher, LargeCollection, MediumCollection, Scalar, SmallCollection};
use crate::utils::{DeduppedVec, SortedAndDeduppedVec};
use alloc::vec;
use proc_macro2::{Literal, TokenStream};
use quote::{ToTokens, quote};
use syn::{Type, parse_quote, parse_str};

#[cfg(not(feature = "std"))]
use {alloc::string::String, alloc::string::ToString, alloc::vec::Vec};

#[derive(Debug)]
pub(super) struct Generator {
    key_type: Type,
    value_type: Type,
    len: TokenStream,
    gen_set: bool,
}

pub(super) struct Output {
    pub ctor: TokenStream,
    pub type_sig: TokenStream,
}

impl Generator {
    pub(super) fn new(key_type: &Type, value_type: Option<&Type>, len: usize) -> Self {
        Self {
            key_type: (*key_type).clone(),
            value_type: value_type.map_or_else(|| parse_quote!(()), Clone::clone),
            len: Self::inject_underscores(Literal::usize_unsuffixed(len).to_token_stream()),
            gen_set: value_type.is_none(),
        }
    }

    pub fn gen_fz_hash<K>(self, entries: Vec<CollectionEntry<K>>) -> Output {
        let key_type = &self.key_type;
        let value_type = &self.value_type;

        let mut ty = quote!(::frozen_collections::FzHashMap);
        let mut generics = quote!(<#key_type, #value_type>);
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

    pub fn gen_fz_ordered<K>(self, entries: Vec<CollectionEntry<K>>) -> Output {
        let key_type = &self.key_type;
        let value_type = &self.value_type;

        let mut ty = quote!(::frozen_collections::FzOrderedMap);
        let mut generics = quote!(<#key_type, #value_type>);
        let mut type_sig = quote!(#ty::#generics);
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
    pub(super) fn gen_fz_scalar<K>(self, entries: Vec<CollectionEntry<K>>) -> Output {
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
    pub(super) fn gen_fz_string<K>(self, entries: Vec<CollectionEntry<K>>) -> Output {
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

    pub(super) fn gen_inline_dense_scalar_lookup<K>(&self, entries: SortedAndDeduppedVec<CollectionEntry<K>>) -> Output
    where
        K: Scalar,
    {
        let entries = entries.into_vec();

        let key_type = &self.key_type;
        let value_type = &self.value_type;
        let len = &self.len;
        let min_key = Self::inject_underscores(entries[0].key.index().to_token_stream());
        let max_key = Self::inject_underscores(entries[entries.len() - 1].key.index().to_token_stream());

        let mut ty = quote!(::frozen_collections::inline_maps::InlineDenseScalarLookupMap);
        let mut generics = quote!(<#key_type, #value_type, #len>);
        let mut type_sig = quote!(#ty::#generics);
        let mut ctor = quote!(#type_sig::new_raw([
            #(
                #entries,
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
    pub(super) fn gen_inline_eytzinger_search<K>(&self, entries: SortedAndDeduppedVec<CollectionEntry<K>>) -> Output {
        let mut entries = entries.into_vec();
        crate::utils::eytzinger_sort(&mut entries);

        let key_type = &self.key_type;
        let value_type = &self.value_type;
        let len = &self.len;

        let mut ty = quote!(::frozen_collections::inline_maps::InlineEytzingerSearchMap);
        let mut generics = quote!(<#key_type, #value_type, #len>);
        let mut type_sig = quote!(#ty::#generics);
        let mut ctor = quote!(#type_sig::new_raw([
            #(
                #entries,
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

    pub(super) fn gen_inline_hash<K, H>(
        &self,
        entries: DeduppedVec<CollectionEntry<K>>,
        hasher: &H,
        hasher_type: &TokenStream,
        hasher_ctor: &TokenStream,
    ) -> Output
    where
        H: Hasher<K>,
    {
        let key_type = &self.key_type;
        let value_type = &self.value_type;
        let len = &self.len;
        let (ht, magnitude, num_slots, collisions) = self.gen_inline_hash_table_components(entries, hasher);

        let mut ty = if collisions {
            quote!(::frozen_collections::inline_maps::InlineHashMap)
        } else {
            quote!(::frozen_collections::inline_maps::InlineHashMapNoCollisions)
        };

        let mut generics = quote!(<#key_type, #value_type, #len, #num_slots, #magnitude, #hasher_type>);
        let mut type_sig = quote!(#ty::#generics);
        let mut ctor = quote!(#type_sig::new_raw(#ht, #hasher_ctor));

        if self.gen_set {
            ty = if collisions {
                quote!(::frozen_collections::inline_sets::InlineHashSet)
            } else {
                quote!(::frozen_collections::inline_sets::InlineHashSetNoCollisions)
            };

            generics = quote!(<#key_type, #len, #num_slots, #magnitude, #hasher_type>);
            type_sig = quote!(#ty::#generics);
            ctor = quote!(#type_sig::new(#ctor));
        }

        Output { ctor, type_sig }
    }

    pub(super) fn gen_inline_scan<K>(&self, entries: DeduppedVec<CollectionEntry<K>>) -> Output {
        let entries = entries.into_vec();

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

    pub fn gen_inline_scan_vec<K>(&self, entries: Vec<CollectionEntry<K>>) -> Output {
        let key_type = &self.key_type;
        let value_type = &self.value_type;
        let len = &self.len;

        let mut ty = quote!(::frozen_collections::inline_maps::InlineScanMap);
        let mut generics = quote!(<#key_type, #value_type, #len>);
        let mut type_sig = quote!(#ty::#generics);
        let mut ctor = quote!(#type_sig::new(vec![
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

    pub(super) fn gen_inline_sparse_scalar_lookup<K>(&self, entries: SortedAndDeduppedVec<CollectionEntry<K>>) -> Output
    where
        K: Scalar,
    {
        let min_key = &entries[0].key.index();
        let max_key = &entries[entries.len() - 1].key.index();

        let count = max_key - min_key + 1;
        let mut lookup = vec![0; count];

        for (i, entry) in entries.iter().enumerate() {
            let index_in_lookup = entry.key.index() - min_key;
            let index_in_entries = i + 1;
            lookup[index_in_lookup] = index_in_entries;
        }

        let entries = entries.into_vec();

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
                #entries,
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

    fn gen_inline_hash_table_components<K, H>(
        &self,
        entries: DeduppedVec<CollectionEntry<K>>,
        hasher: &H,
    ) -> (TokenStream, TokenStream, Literal, bool)
    where
        H: Hasher<K>,
    {
        let key_type = &self.key_type;
        let value_type = &self.value_type;
        let len = &self.len;

        let ht = HashTable::<_, LargeCollection>::new(entries, |x| hasher.hash_one(&x.key)).unwrap();
        let collisions = ht.has_collisions();
        let slots = ht.slots;
        let num_slots = Literal::usize_unsuffixed(slots.len());
        let entries = ht.entries;
        let magnitude = Self::collection_magnitude(entries.len());

        if collisions {
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
                true,
            )
        } else {
            let slots = slots.iter().map(|s| {
                if s.is_empty() {
                    Literal::usize_unsuffixed(0)
                } else {
                    Literal::usize_unsuffixed(s.min_index + 1)
                }
            });

            (
                quote!(::frozen_collections::hash_tables::InlineHashTableNoCollisions::<(#key_type, #value_type), #len, #num_slots, #magnitude>::new_raw(
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
                false,
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

    pub fn inject_underscores(v: TokenStream) -> TokenStream {
        let mut full: Vec<char> = v.to_string().chars().collect();
        let mut suffix_index = None;
        for (index, c) in full.iter().enumerate() {
            if !c.is_ascii_digit() {
                suffix_index = Some(index);
                break;
            }
        }

        let number = if let Some(index) = suffix_index {
            &mut full[0..index]
        } else {
            full.as_mut_slice()
        };

        number.reverse();
        let mut output = Vec::new();
        for (index, c) in number.iter().enumerate() {
            if index % 3 == 0 && index != 0 {
                output.push('_');
            }
            output.push(*c);
        }
        output.reverse();

        if let Some(index) = suffix_index {
            output.extend_from_slice(&full[index..]);
        }

        parse_str(&output.into_iter().collect::<String>()).unwrap()
    }
}
