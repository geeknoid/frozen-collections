use crate::maps::decl_macros::{
    common_primary_funcs, debug_trait_funcs, eytzinger_search_primary_funcs, get_disjoint_mut_funcs, index_trait_funcs,
    into_iterator_trait_funcs, into_iterator_trait_mut_ref_funcs, into_iterator_trait_ref_funcs, len_trait_funcs, map_extras_trait_funcs,
    map_iteration_trait_funcs, map_query_trait_funcs, partial_eq_trait_funcs,
};
use crate::maps::{IntoIter, IntoKeys, IntoValues, Iter, IterMut, Keys, Values, ValuesMut};
use crate::traits::{Len, Map, MapExtras, MapIteration, MapQuery};
use crate::utils::{SortedAndDeduppedVec, eytzinger_layout, eytzinger_search_by};
use core::fmt::{Debug, Formatter, Result};
use core::ops::Index;
use equivalent::Comparable;

#[cfg(not(feature = "std"))]
use alloc::vec::Vec;

#[cfg(feature = "serde")]
use {
    crate::maps::decl_macros::serialize_trait_funcs,
    serde::ser::SerializeMap,
    serde::{Serialize, Serializer},
};

/// A general-purpose map implemented using Eytzinger search.
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
pub struct InlineEytzingerSearchMap<K, V, const SZ: usize> {
    entries: [(K, V); SZ],
}

impl<K, V, const SZ: usize> InlineEytzingerSearchMap<K, V, SZ> {
    /// Creates a frozen map.
    ///
    /// # Panics
    ///
    /// Panics if the number of entries in the array differs from the size of the map as specified by the `SZ` generic argument.
    #[must_use]
    pub fn new(entries: Vec<(K, V)>) -> Self
    where
        K: Ord,
    {
        let mut entries = SortedAndDeduppedVec::new(entries, |x, y| x.0.cmp(&y.0)).into_vec();
        eytzinger_layout(&mut entries);
        Self::new_raw(
            entries
                .try_into()
                .unwrap_or_else(|_| panic!("Cannot convert to array of size {SZ}: length mismatch")),
        )
    }

    /// Creates a frozen map.
    ///
    /// This function assumes the vector is sorted according to the Eytzinger layout.
    #[must_use]
    pub const fn new_raw(eytzinger_layout_dedupped_entries: [(K, V); SZ]) -> Self {
        Self {
            entries: eytzinger_layout_dedupped_entries,
        }
    }

    eytzinger_search_primary_funcs!();
    common_primary_funcs!(const_len, entries);
}

impl<K, V, Q, const SZ: usize> Map<K, V, Q> for InlineEytzingerSearchMap<K, V, SZ> where Q: ?Sized + Comparable<K> {}

impl<K, V, Q, const SZ: usize> MapExtras<K, V, Q> for InlineEytzingerSearchMap<K, V, SZ>
where
    Q: ?Sized + Comparable<K>,
{
    map_extras_trait_funcs!();
}

impl<K, V, Q, const SZ: usize> MapQuery<Q, V> for InlineEytzingerSearchMap<K, V, SZ>
where
    Q: ?Sized + Comparable<K>,
{
    map_query_trait_funcs!();
}

impl<K, V, const SZ: usize> MapIteration<K, V> for InlineEytzingerSearchMap<K, V, SZ> {
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

impl<K, V, const SZ: usize> Len for InlineEytzingerSearchMap<K, V, SZ> {
    len_trait_funcs!();
}

impl<Q, K, V, const SZ: usize> Index<&Q> for InlineEytzingerSearchMap<K, V, SZ>
where
    Q: ?Sized + Comparable<K>,
{
    index_trait_funcs!();
}

impl<K, V, const SZ: usize> IntoIterator for InlineEytzingerSearchMap<K, V, SZ> {
    into_iterator_trait_funcs!();
}

impl<'a, K, V, const SZ: usize> IntoIterator for &'a InlineEytzingerSearchMap<K, V, SZ> {
    into_iterator_trait_ref_funcs!();
}

impl<'a, K, V, const SZ: usize> IntoIterator for &'a mut InlineEytzingerSearchMap<K, V, SZ> {
    into_iterator_trait_mut_ref_funcs!();
}

impl<K, V, MT, const N: usize> PartialEq<MT> for InlineEytzingerSearchMap<K, V, N>
where
    K: Ord,
    V: PartialEq,
    MT: MapQuery<K, V>,
{
    partial_eq_trait_funcs!();
}

impl<K, V, const N: usize> Eq for InlineEytzingerSearchMap<K, V, N>
where
    K: Ord,
    V: PartialEq,
{
}

impl<K, V, const SZ: usize> Debug for InlineEytzingerSearchMap<K, V, SZ>
where
    K: Debug,
    V: Debug,
{
    debug_trait_funcs!();
}

#[cfg(feature = "serde")]
impl<K, V, const SZ: usize> Serialize for InlineEytzingerSearchMap<K, V, SZ>
where
    K: Serialize,
    V: Serialize,
{
    serialize_trait_funcs!();
}
