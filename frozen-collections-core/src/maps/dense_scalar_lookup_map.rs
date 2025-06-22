use crate::maps::decl_macros::{
    common_primary_funcs, debug_trait_funcs, dense_scalar_lookup_primary_funcs, get_disjoint_mut_funcs, index_trait_funcs,
    into_iterator_trait_funcs, into_iterator_trait_mut_ref_funcs, into_iterator_trait_ref_funcs, len_trait_funcs, map_extras_trait_funcs,
    map_iteration_trait_funcs, map_query_trait_funcs, partial_eq_trait_funcs,
};
use crate::maps::{IntoIter, IntoKeys, IntoValues, Iter, IterMut, Keys, Values, ValuesMut};
use crate::traits::{Len, Map, MapExtras, MapIteration, MapQuery, Scalar};
use crate::utils::dedup_by_keep_last;
use core::fmt::{Debug, Formatter, Result};
use core::ops::Index;
use equivalent::Comparable;

#[cfg(not(feature = "std"))]
use {alloc::boxed::Box, alloc::string::String, alloc::string::ToString, alloc::vec::Vec};

#[cfg(feature = "serde")]
use {
    crate::maps::decl_macros::serialize_trait_funcs,
    serde::ser::SerializeMap,
    serde::{Serialize, Serializer},
};

/// A map whose keys are a continuous range in a sequence of scalar values.
///
#[doc = include_str!("../doc_snippets/private_api_warning.md")]
#[doc = include_str!("../doc_snippets/about.md")]
///
#[derive(Clone)]
pub struct DenseScalarLookupMap<K, V> {
    min: usize,
    max: usize,
    entries: Box<[(K, V)]>,
}

impl<K, V> DenseScalarLookupMap<K, V> {
    /// Creates a frozen map.
    ///
    /// # Errors
    ///
    /// Fails if all the keys in the input vector, after sorting and dedupping,
    /// don't represent a continuous range of values.
    pub fn new(mut entries: Vec<(K, V)>) -> core::result::Result<Self, String>
    where
        K: Scalar,
    {
        entries.sort_by_key(|x| x.0);
        dedup_by_keep_last(&mut entries, |x, y| x.0.eq(&y.0));

        if entries.is_empty() {
            return Ok(Self::default());
        }

        let min = entries[0].0.index();
        let max = entries[entries.len() - 1].0.index();

        if entries.len() == max - min + 1 {
            Ok(Self::new_raw(entries))
        } else {
            Err("keys must be in a contiguous range <= usize::MAX in size".to_string())
        }
    }

    /// Creates a new frozen map.
    ///
    /// This function assumes that `min` <= `max` and that the vector is sorted according to the
    /// order of the [`Ord`] trait.
    #[must_use]
    pub(crate) fn new_raw(processed_entries: Vec<(K, V)>) -> Self
    where
        K: Scalar,
    {
        Self {
            min: processed_entries[0].0.index(),
            max: processed_entries[processed_entries.len() - 1].0.index(),
            entries: processed_entries.into_boxed_slice(),
        }
    }

    dense_scalar_lookup_primary_funcs!();
    common_primary_funcs!(non_const_len, entries);
}

impl<K, V> Default for DenseScalarLookupMap<K, V> {
    fn default() -> Self {
        Self {
            min: 1,
            max: 0,
            entries: Box::new([]),
        }
    }
}

impl<K, V, Q> Map<K, V, Q> for DenseScalarLookupMap<K, V> where Q: Scalar + Comparable<K> {}

impl<K, V, Q> MapExtras<K, V, Q> for DenseScalarLookupMap<K, V>
where
    Q: Scalar + Comparable<K>,
{
    map_extras_trait_funcs!();
}

impl<K, V, Q> MapQuery<Q, V> for DenseScalarLookupMap<K, V>
where
    Q: Scalar + Comparable<K>,
{
    map_query_trait_funcs!();
}

impl<K, V> MapIteration<K, V> for DenseScalarLookupMap<K, V> {
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

impl<K, V> Len for DenseScalarLookupMap<K, V> {
    len_trait_funcs!();
}

impl<Q, V> Index<&Q> for DenseScalarLookupMap<Q, V>
where
    Q: Scalar,
{
    index_trait_funcs!();
}

impl<K, V> IntoIterator for DenseScalarLookupMap<K, V> {
    into_iterator_trait_funcs!();
}

impl<'a, K, V> IntoIterator for &'a DenseScalarLookupMap<K, V> {
    into_iterator_trait_ref_funcs!();
}

impl<'a, K, V> IntoIterator for &'a mut DenseScalarLookupMap<K, V> {
    into_iterator_trait_mut_ref_funcs!();
}

impl<K, V, MT> PartialEq<MT> for DenseScalarLookupMap<K, V>
where
    K: Scalar,
    V: PartialEq,
    MT: MapQuery<K, V>,
{
    partial_eq_trait_funcs!();
}

impl<K, V> Eq for DenseScalarLookupMap<K, V>
where
    K: Scalar,
    V: Eq,
{
}

impl<K, V> Debug for DenseScalarLookupMap<K, V>
where
    K: Scalar + Debug,
    V: Debug,
{
    debug_trait_funcs!();
}

#[cfg(feature = "serde")]
impl<K, V> Serialize for DenseScalarLookupMap<K, V>
where
    K: Serialize + Scalar,
    V: Serialize,
{
    serialize_trait_funcs!();
}

#[cfg(test)]
mod tests {
    use super::*;
    use alloc::vec;

    #[test]
    fn error_in_new() {
        let map = DenseScalarLookupMap::<u8, u8>::new(vec![(1, 1), (2, 2), (4, 3)]);
        assert_eq!(map, Err("keys must be in a contiguous range <= usize::MAX in size".to_string()));
    }
}
