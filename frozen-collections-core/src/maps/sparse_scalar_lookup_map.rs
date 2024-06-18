use crate::maps::decl_macros::{
    contains_key_fn, debug_fn, get_many_mut_body, get_many_mut_fn, index_fn,
    into_iter_fn_for_slice, into_iter_mut_ref_fn, into_iter_ref_fn, map_boilerplate_for_slice,
    map_iterator_boilerplate_for_slice, partial_eq_fn, sparse_scalar_lookup_core,
};
use crate::maps::{IntoIter, IntoKeys, IntoValues, Iter, IterMut, Keys, Values, ValuesMut};
use crate::traits::{CollectionMagnitude, Len, Map, MapIterator, Scalar, SmallCollection};
use crate::utils::dedup_by_keep_last;
use alloc::boxed::Box;
use alloc::string::String;
use alloc::vec::Vec;
use core::borrow::Borrow;
use core::fmt::{Debug, Formatter, Result};
use core::ops::Index;

/// A map whose keys are a sparse range of values from a scalar.
///
#[doc = include_str!("../doc_snippets/type_compat_warning.md")]
#[doc = include_str!("../doc_snippets/about.md")]
///
#[derive(Clone)]
pub struct SparseScalarLookupMap<K, V, CM = SmallCollection> {
    min: usize,
    max: usize,
    lookup: Box<[CM]>,
    entries: Box<[(K, V)]>,
}

impl<K, V, CM> SparseScalarLookupMap<K, V, CM>
where
    K: Scalar,
    CM: CollectionMagnitude,
    <CM as TryFrom<usize>>::Error: Debug,
{
    /// Creates a new `IntegerSparseLookupMap` from a list of entries.
    ///
    /// Note that this supports 1 less entry relative to the maximum capacity of the collection scale
    /// since 0 is used as a sentinel value within the lookup table.
    ///
    /// # Errors
    ///
    /// Fails if the number of entries in the vector, after deduplication, exceeds the
    /// magnitude of the collection as specified by the `CM` generic argument.
    pub fn new(mut entries: Vec<(K, V)>) -> core::result::Result<Self, String> {
        entries.sort_by_key(|x| x.0);
        dedup_by_keep_last(&mut entries, |x, y| x.0.eq(&y.0));

        if entries.is_empty() {
            return Ok(Self::default());
        }

        let min = entries[0].0.index();
        let max = entries[entries.len() - 1].0.index();

        if max - min + 1 >= CM::MAX_CAPACITY {
            Err("the range of keys is too large for the selected collection magnitude".to_string())
        } else {
            Ok(Self::new_raw(entries))
        }
    }

    /// Creates a new frozen map.
    ///
    /// # Panics
    ///
    /// Panics if the number of entries in the vector exceeds the
    /// magnitude of the collection as specified by the `CM` generic argument.
    #[must_use]
    pub(crate) fn new_raw(processed_entries: Vec<(K, V)>) -> Self {
        let min = processed_entries[0].0.index();
        let max = processed_entries[processed_entries.len() - 1].0.index();
        let count = max - min + 1;

        let mut lookup = Vec::<CM>::with_capacity(count);
        lookup.resize(lookup.capacity(), CM::ZERO);

        for (i, entry) in processed_entries.iter().enumerate() {
            let index_in_lookup = entry.0.index() - min;
            let index_in_entries = CM::try_from(i + 1).expect("less than CM::MAX");
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

impl<K, V, CM> SparseScalarLookupMap<K, V, CM>
where
    CM: CollectionMagnitude,
{
    sparse_scalar_lookup_core!();
}

impl<K, V, CM> Len for SparseScalarLookupMap<K, V, CM> {
    fn len(&self) -> usize {
        self.entries.len()
    }
}

impl<K, V, CM> Debug for SparseScalarLookupMap<K, V, CM>
where
    K: Debug,
    V: Debug,
{
    debug_fn!();
}

impl<K, V, CM> Default for SparseScalarLookupMap<K, V, CM> {
    fn default() -> Self {
        Self {
            min: 1,
            max: 0,
            lookup: Box::new([]),
            entries: Box::new([]),
        }
    }
}

impl<Q, K, V, CM> Index<&Q> for SparseScalarLookupMap<K, V, CM>
where
    K: Borrow<Q>,
    Q: Scalar,
    CM: CollectionMagnitude,
{
    index_fn!();
}

impl<K, V, CM> IntoIterator for SparseScalarLookupMap<K, V, CM> {
    into_iter_fn_for_slice!(entries);
}

impl<'a, K, V, CM> IntoIterator for &'a SparseScalarLookupMap<K, V, CM> {
    into_iter_ref_fn!();
}

impl<'a, K, V, CM> IntoIterator for &'a mut SparseScalarLookupMap<K, V, CM> {
    into_iter_mut_ref_fn!();
}

impl<K, V, MT, CM> PartialEq<MT> for SparseScalarLookupMap<K, V, CM>
where
    K: Scalar,
    V: PartialEq,
    MT: Map<K, V>,
    CM: CollectionMagnitude,
{
    partial_eq_fn!();
}

impl<K, V, CM> Eq for SparseScalarLookupMap<K, V, CM>
where
    K: Scalar,
    V: Eq,
    CM: CollectionMagnitude,
{
}

impl<K, V, CM> MapIterator<K, V> for SparseScalarLookupMap<K, V, CM> {
    type Iterator<'a>
        = Iter<'a, K, V>
    where
        K: 'a,
        V: 'a,
        CM: 'a;

    type KeyIterator<'a>
        = Keys<'a, K, V>
    where
        K: 'a,
        V: 'a,
        CM: 'a;

    type ValueIterator<'a>
        = Values<'a, K, V>
    where
        K: 'a,
        V: 'a,
        CM: 'a;

    type MutIterator<'a>
        = IterMut<'a, K, V>
    where
        K: 'a,
        V: 'a,
        CM: 'a;

    type ValueMutIterator<'a>
        = ValuesMut<'a, K, V>
    where
        K: 'a,
        V: 'a,
        CM: 'a;

    map_iterator_boilerplate_for_slice!(entries);
}

impl<K, V, CM> Map<K, V> for SparseScalarLookupMap<K, V, CM>
where
    K: Scalar,
    CM: CollectionMagnitude,
{
    map_boilerplate_for_slice!(entries);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_fails_when_entries_exceed_collection_magnitude() {
        let mut entries = Vec::new();
        for i in 0..SmallCollection::MAX_CAPACITY {
            entries.push((i, 42));
        }

        let map = SparseScalarLookupMap::<_, _, SmallCollection>::new(entries);
        assert_eq!(
            map,
            Err("the range of keys is too large for the selected collection magnitude".to_string())
        );
    }
}
