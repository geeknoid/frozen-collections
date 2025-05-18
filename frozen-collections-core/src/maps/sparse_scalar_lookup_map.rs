use crate::maps::decl_macros::{
    debug_fn, get_disjoint_mut_fn, get_disjoint_unchecked_mut_body, get_disjoint_unchecked_mut_fn,
    index_fn, into_iter_fn, into_iter_mut_ref_fn, into_iter_ref_fn, map_iteration_funcs,
    partial_eq_fn, sparse_scalar_lookup_query_funcs,
};
use crate::maps::{IntoIter, IntoKeys, IntoValues, Iter, IterMut, Keys, Values, ValuesMut};
use crate::traits::{Len, Map, MapIteration, MapQuery, Scalar};
use crate::utils::dedup_by_keep_last;
use alloc::boxed::Box;
use alloc::vec;
use alloc::vec::Vec;
use core::fmt::{Debug, Formatter, Result};
use core::ops::Index;

#[cfg(feature = "serde")]
use {
    crate::maps::decl_macros::serialize_fn,
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

impl<K, V> SparseScalarLookupMap<K, V>
where
    K: Scalar,
{
    /// Creates a new `IntegerSparseLookupMap` from a list of entries.
    #[must_use]
    pub fn new(mut entries: Vec<(K, V)>) -> Self {
        entries.sort_by_key(|x| x.0);
        dedup_by_keep_last(&mut entries, |x, y| x.0.eq(&y.0));

        if entries.is_empty() {
            return Self::default();
        }

        Self::new_raw(entries)
    }

    /// Creates a new frozen map.
    #[must_use]
    pub(crate) fn new_raw(processed_entries: Vec<(K, V)>) -> Self {
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

impl<K, V> Map<K, V, K> for SparseScalarLookupMap<K, V>
where
    K: Scalar,
{
    get_disjoint_mut_fn!("Scalar");
    get_disjoint_unchecked_mut_fn!("Scalar");
}

impl<K, V> MapQuery<K, V, K> for SparseScalarLookupMap<K, V>
where
    K: Scalar,
{
    sparse_scalar_lookup_query_funcs!();
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

    map_iteration_funcs!(entries);
}

impl<K, V> Len for SparseScalarLookupMap<K, V> {
    fn len(&self) -> usize {
        self.entries.len()
    }
}

impl<Q, V> Index<&Q> for SparseScalarLookupMap<Q, V>
where
    Q: Scalar,
{
    index_fn!();
}

impl<K, V> IntoIterator for SparseScalarLookupMap<K, V> {
    into_iter_fn!(entries);
}

impl<'a, K, V> IntoIterator for &'a SparseScalarLookupMap<K, V> {
    into_iter_ref_fn!();
}

impl<'a, K, V> IntoIterator for &'a mut SparseScalarLookupMap<K, V> {
    into_iter_mut_ref_fn!();
}

impl<K, V, MT> PartialEq<MT> for SparseScalarLookupMap<K, V>
where
    K: Scalar,
    V: PartialEq,
    MT: Map<K, V>,
{
    partial_eq_fn!();
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
    debug_fn!();
}

#[cfg(feature = "serde")]
impl<K, V> Serialize for SparseScalarLookupMap<K, V>
where
    K: Serialize,
    V: Serialize,
{
    serialize_fn!();
}
