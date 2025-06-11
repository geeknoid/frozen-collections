use crate::maps::decl_macros::{
    common_primary_funcs, debug_trait_funcs, get_disjoint_mut_funcs, index_trait_funcs, into_iterator_trait_funcs,
    into_iterator_trait_mut_ref_funcs, into_iterator_trait_ref_funcs, len_trait_funcs, map_extras_trait_funcs, map_iteration_trait_funcs,
    map_query_trait_funcs, partial_eq_trait_funcs, sparse_scalar_lookup_primary_funcs,
};
use crate::maps::{IntoIter, IntoKeys, IntoValues, Iter, IterMut, Keys, Values, ValuesMut};
use crate::traits::{Len, Map, MapExtras, MapIteration, MapQuery, Scalar};
use crate::utils::dedup_by_keep_last;
use alloc::vec;
use core::fmt::{Debug, Formatter, Result};
use core::ops::Index;
use equivalent::Comparable;

#[cfg(not(feature = "std"))]
use {alloc::boxed::Box, alloc::vec::Vec};

#[cfg(feature = "serde")]
use {
    crate::maps::decl_macros::serialize_trait_funcs,
    serde::ser::SerializeMap,
    serde::{Serialize, Serializer},
};

/// A map whose keys are a sparse range of values from a scalar.
///
#[doc = include_str!("../doc_snippets/private_api_warning.md")]
#[doc = include_str!("../doc_snippets/about.md")]
///
#[derive(Clone)]
pub struct SparseScalarLookupMap<K, V> {
    min: usize,
    max: usize,
    lookup: Box<[usize]>,
    entries: Box<[(K, V)]>,
}

impl<K, V> SparseScalarLookupMap<K, V> {
    /// Creates a new `SparseScalarLookupMap` from a list of entries.
    #[must_use]
    pub fn new(mut entries: Vec<(K, V)>) -> Self
    where
        K: Scalar,
    {
        entries.sort_by_key(|x| x.0);
        dedup_by_keep_last(&mut entries, |x, y| x.0.eq(&y.0));

        if entries.is_empty() {
            return Self::default();
        }

        Self::new_raw(entries)
    }

    /// Creates a new frozen map.
    #[must_use]
    pub(crate) fn new_raw(processed_entries: Vec<(K, V)>) -> Self
    where
        K: Scalar,
    {
        let min = processed_entries[0].0.index();
        let max = processed_entries[processed_entries.len() - 1].0.index();
        let count = max - min + 1;

        let mut lookup = vec![0; count];

        for (i, entry) in processed_entries.iter().enumerate() {
            let index_in_lookup = entry.0.index() - min;
            let index_in_entries = i + 1;
            lookup[index_in_lookup] = index_in_entries;
        }

        Self {
            min,
            max,
            lookup: lookup.into_boxed_slice(),
            entries: processed_entries.into_boxed_slice(),
        }
    }

    sparse_scalar_lookup_primary_funcs!();
    common_primary_funcs!(non_const_len, entries);
}

impl<K, V> Default for SparseScalarLookupMap<K, V> {
    fn default() -> Self {
        Self {
            min: 1,
            max: 0,
            lookup: Box::new([]),
            entries: Box::new([]),
        }
    }
}

impl<K, V, Q> Map<K, V, Q> for SparseScalarLookupMap<K, V> where Q: Scalar + Comparable<K> {}

impl<K, V, Q> MapExtras<K, V, Q> for SparseScalarLookupMap<K, V>
where
    Q: Scalar + Comparable<K>,
{
    map_extras_trait_funcs!();
}

impl<K, V, Q> MapQuery<Q, V> for SparseScalarLookupMap<K, V>
where
    Q: Scalar + Comparable<K>,
{
    map_query_trait_funcs!();
}

impl<K, V> MapIteration<K, V> for SparseScalarLookupMap<K, V> {
    type Iterator<'a>
        = Iter<'a, K, V>
    where
        K: 'a,
        V: 'a;

    type KeyIterator<'a>
        = Keys<'a, K, V>
    where
        K: 'a,
        V: 'a;

    type ValueIterator<'a>
        = Values<'a, K, V>
    where
        K: 'a,
        V: 'a;

    type MutIterator<'a>
        = IterMut<'a, K, V>
    where
        K: 'a,
        V: 'a;

    type ValueMutIterator<'a>
        = ValuesMut<'a, K, V>
    where
        K: 'a,
        V: 'a;

    map_iteration_trait_funcs!();
}

impl<K, V> Len for SparseScalarLookupMap<K, V> {
    len_trait_funcs!();
}

impl<K, V, Q> Index<&Q> for SparseScalarLookupMap<K, V>
where
    Q: Comparable<K> + Scalar,
{
    index_trait_funcs!();
}

impl<K, V> IntoIterator for SparseScalarLookupMap<K, V> {
    into_iterator_trait_funcs!();
}

impl<'a, K, V> IntoIterator for &'a SparseScalarLookupMap<K, V> {
    into_iterator_trait_ref_funcs!();
}

impl<'a, K, V> IntoIterator for &'a mut SparseScalarLookupMap<K, V> {
    into_iterator_trait_mut_ref_funcs!();
}

impl<K, V, MT> PartialEq<MT> for SparseScalarLookupMap<K, V>
where
    K: Scalar,
    V: PartialEq,
    MT: MapQuery<K, V>,
{
    partial_eq_trait_funcs!();
}

impl<K, V> Eq for SparseScalarLookupMap<K, V>
where
    K: Scalar,
    V: Eq,
{
}

impl<K, V> Debug for SparseScalarLookupMap<K, V>
where
    K: Debug,
    V: Debug,
{
    debug_trait_funcs!();
}

#[cfg(feature = "serde")]
impl<K, V> Serialize for SparseScalarLookupMap<K, V>
where
    K: Serialize + Scalar,
    V: Serialize,
{
    serialize_trait_funcs!();
}
