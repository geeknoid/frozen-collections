use crate::maps::decl_macros::{
    binary_search_primary_funcs, common_primary_funcs, debug_trait_funcs, get_disjoint_mut_funcs, index_trait_funcs,
    into_iterator_trait_funcs, into_iterator_trait_mut_ref_funcs, into_iterator_trait_ref_funcs, len_trait_funcs, map_extras_trait_funcs,
    map_iteration_trait_funcs, map_query_trait_funcs, partial_eq_trait_funcs,
};
use crate::maps::{IntoIter, IntoKeys, IntoValues, Iter, IterMut, Keys, Values, ValuesMut};
use crate::traits::{Len, Map, MapExtras, MapIteration, MapQuery};
use core::fmt::{Debug, Formatter, Result};
use core::ops::Index;
use equivalent::Comparable;

#[cfg(feature = "serde")]
use {
    crate::maps::decl_macros::serialize_trait_funcs,
    serde::ser::SerializeMap,
    serde::{Serialize, Serializer},
};

/// A general-purpose map implemented using binary search.
///
#[doc = include_str!("../doc_snippets/private_api_warning.md")]
#[doc = include_str!("../doc_snippets/about.md")]
#[doc = include_str!("../doc_snippets/ord_warning.md")]
///
/// # Type Parameters
///
/// - `K`: The key type.
/// - `V`: The value type.
/// - `SZ`: The number of entries in the map.
#[derive(Clone)]
pub struct InlineBinarySearchMap<K, V, const SZ: usize> {
    entries: [(K, V); SZ],
}

impl<K, V, const SZ: usize> InlineBinarySearchMap<K, V, SZ> {
    /// Creates a frozen map.
    ///
    /// This function assumes the vector is sorted according to the ordering of the [`Ord`] trait.
    #[must_use]
    pub const fn new_raw(processed_entries: [(K, V); SZ]) -> Self {
        Self {
            entries: processed_entries,
        }
    }

    binary_search_primary_funcs!();
    common_primary_funcs!(const_len, entries);
}

impl<K, V, Q, const SZ: usize> Map<K, V, Q> for InlineBinarySearchMap<K, V, SZ> where Q: ?Sized + Comparable<K> {}

impl<K, V, Q, const SZ: usize> MapExtras<K, V, Q> for InlineBinarySearchMap<K, V, SZ>
where
    Q: ?Sized + Comparable<K>,
{
    map_extras_trait_funcs!();
}

impl<K, V, Q, const SZ: usize> MapQuery<Q, V> for InlineBinarySearchMap<K, V, SZ>
where
    Q: ?Sized + Comparable<K>,
{
    map_query_trait_funcs!();
}

impl<K, V, const SZ: usize> MapIteration<K, V> for InlineBinarySearchMap<K, V, SZ> {
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

impl<K, V, const SZ: usize> Len for InlineBinarySearchMap<K, V, SZ> {
    len_trait_funcs!();
}

impl<Q, K, V, const SZ: usize> Index<&Q> for InlineBinarySearchMap<K, V, SZ>
where
    Q: ?Sized + Comparable<K>,
{
    index_trait_funcs!();
}

impl<K, V, const SZ: usize> IntoIterator for InlineBinarySearchMap<K, V, SZ> {
    into_iterator_trait_funcs!();
}

impl<'a, K, V, const SZ: usize> IntoIterator for &'a InlineBinarySearchMap<K, V, SZ> {
    into_iterator_trait_ref_funcs!();
}

impl<'a, K, V, const SZ: usize> IntoIterator for &'a mut InlineBinarySearchMap<K, V, SZ> {
    into_iterator_trait_mut_ref_funcs!();
}

impl<K, V, MT, const N: usize> PartialEq<MT> for InlineBinarySearchMap<K, V, N>
where
    K: Ord,
    V: PartialEq,
    MT: MapQuery<K, V>,
{
    partial_eq_trait_funcs!();
}

impl<K, V, const N: usize> Eq for InlineBinarySearchMap<K, V, N>
where
    K: Ord,
    V: PartialEq,
{
}

impl<K, V, const SZ: usize> Debug for InlineBinarySearchMap<K, V, SZ>
where
    K: Debug,
    V: Debug,
{
    debug_trait_funcs!();
}

#[cfg(feature = "serde")]
impl<K, V, const SZ: usize> Serialize for InlineBinarySearchMap<K, V, SZ>
where
    K: Serialize,
    V: Serialize,
{
    serialize_trait_funcs!();
}
