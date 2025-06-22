use crate::maps::decl_macros::{
    common_primary_funcs, debug_trait_funcs, dense_scalar_lookup_primary_funcs, get_disjoint_mut_funcs, index_trait_funcs,
    into_iterator_trait_funcs, into_iterator_trait_mut_ref_funcs, into_iterator_trait_ref_funcs, len_trait_funcs, map_extras_trait_funcs,
    map_iteration_trait_funcs, map_query_trait_funcs, partial_eq_trait_funcs,
};
use crate::maps::{IntoIter, IntoKeys, IntoValues, Iter, IterMut, Keys, Values, ValuesMut};
use crate::traits::{Len, Map, MapExtras, MapIteration, MapQuery, Scalar};
use core::fmt::{Debug, Formatter, Result};
use core::ops::Index;
use equivalent::Comparable;

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
/// # Type Parameters
///
/// - `K`: The key type.
/// - `V`: The value type.
/// - `SZ`: The number of entries in the map.
#[derive(Clone)]
pub struct InlineDenseScalarLookupMap<K, V, const SZ: usize> {
    min: usize,
    max: usize,
    entries: [(K, V); SZ],
}

impl<K, V, const SZ: usize> InlineDenseScalarLookupMap<K, V, SZ> {
    /// Creates a frozen map.
    ///
    /// This function assumes that `min` <= `max` and that the vector is sorted according to the
    /// order of the [`Ord`] trait.
    #[must_use]
    pub const fn new_raw(processed_entries: [(K, V); SZ], min: usize, max: usize) -> Self {
        Self {
            min,
            max,
            entries: processed_entries,
        }
    }

    dense_scalar_lookup_primary_funcs!();
    common_primary_funcs!(const_len, entries);
}

impl<K, V, Q, const SZ: usize> Map<K, V, Q> for InlineDenseScalarLookupMap<K, V, SZ> where Q: Scalar + Comparable<K> {}

impl<K, V, Q, const SZ: usize> MapExtras<K, V, Q> for InlineDenseScalarLookupMap<K, V, SZ>
where
    Q: Scalar + Comparable<K>,
{
    map_extras_trait_funcs!();
}

impl<K, V, Q, const SZ: usize> MapQuery<Q, V> for InlineDenseScalarLookupMap<K, V, SZ>
where
    Q: Scalar + Comparable<K>,
{
    map_query_trait_funcs!();
}

impl<K, V, const SZ: usize> MapIteration<K, V> for InlineDenseScalarLookupMap<K, V, SZ> {
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

impl<K, V, const SZ: usize> Len for InlineDenseScalarLookupMap<K, V, SZ> {
    len_trait_funcs!();
}

impl<K, V, Q, const SZ: usize> Index<&Q> for InlineDenseScalarLookupMap<K, V, SZ>
where
    Q: Comparable<K> + Scalar,
{
    index_trait_funcs!();
}

impl<K, V, const SZ: usize> IntoIterator for InlineDenseScalarLookupMap<K, V, SZ> {
    into_iterator_trait_funcs!();
}

impl<'a, K, V, const SZ: usize> IntoIterator for &'a InlineDenseScalarLookupMap<K, V, SZ> {
    into_iterator_trait_ref_funcs!();
}

impl<'a, K, V, const SZ: usize> IntoIterator for &'a mut InlineDenseScalarLookupMap<K, V, SZ> {
    into_iterator_trait_mut_ref_funcs!();
}

impl<K, V, MT, const SZ: usize> PartialEq<MT> for InlineDenseScalarLookupMap<K, V, SZ>
where
    K: Scalar,
    V: PartialEq,
    MT: MapQuery<K, V>,
{
    partial_eq_trait_funcs!();
}

impl<K, V, const SZ: usize> Eq for InlineDenseScalarLookupMap<K, V, SZ>
where
    K: Scalar,
    V: Eq,
{
}

impl<K, V, const SZ: usize> Debug for InlineDenseScalarLookupMap<K, V, SZ>
where
    K: Debug,
    V: Debug,
{
    debug_trait_funcs!();
}

#[cfg(feature = "serde")]
impl<K, V, const SZ: usize> Serialize for InlineDenseScalarLookupMap<K, V, SZ>
where
    K: Serialize,
    V: Serialize,
{
    serialize_trait_funcs!();
}
