use crate::maps::decl_macros::{
    common_primary_funcs, debug_trait_funcs, eytzinger_search_primary_funcs, get_disjoint_mut_funcs, index_trait_funcs,
    into_iterator_trait_funcs, into_iterator_trait_mut_ref_funcs, into_iterator_trait_ref_funcs, len_trait_funcs, map_extras_trait_funcs,
    map_iteration_trait_funcs, map_query_trait_funcs, partial_eq_trait_funcs,
};
use crate::maps::{IntoIter, IntoKeys, IntoValues, Iter, IterMut, Keys, Values, ValuesMut};
use crate::traits::{Len, Map, MapExtras, MapIteration, MapQuery};
use crate::utils::{dedup_by_keep_last, eytzinger_search_by, eytzinger_sort};
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

/// A general-purpose map implemented using Eytzinger search.
///
#[doc = include_str!("../doc_snippets/private_api_warning.md")]
#[doc = include_str!("../doc_snippets/about.md")]
#[doc = include_str!("../doc_snippets/ord_warning.md")]
///
#[derive(Clone)]
pub struct EytzingerSearchMap<K, V> {
    entries: Box<[(K, V)]>,
}

impl<K, V> EytzingerSearchMap<K, V> {
    /// Creates a frozen map.
    #[must_use]
    pub fn new(mut entries: Vec<(K, V)>) -> Self
    where
        K: Ord,
    {
        entries.sort_by(|x, y| x.0.cmp(&y.0));
        dedup_by_keep_last(&mut entries, |x, y| x.0.eq(&y.0));
        eytzinger_sort(&mut entries);
        Self::new_raw(entries)
    }

    /// Creates a frozen map.
    #[must_use]
    pub(crate) fn new_raw(processed_entries: Vec<(K, V)>) -> Self {
        Self {
            entries: processed_entries.into_boxed_slice(),
        }
    }

    eytzinger_search_primary_funcs!();
    common_primary_funcs!(non_const_len, entries);
}

impl<K, V> Default for EytzingerSearchMap<K, V> {
    fn default() -> Self {
        Self { entries: Box::default() }
    }
}

impl<K, V, Q> Map<K, V, Q> for EytzingerSearchMap<K, V> where Q: ?Sized + Comparable<K> {}

impl<K, V, Q> MapExtras<K, V, Q> for EytzingerSearchMap<K, V>
where
    Q: ?Sized + Comparable<K>,
{
    map_extras_trait_funcs!();
}

impl<K, V, Q> MapQuery<Q, V> for EytzingerSearchMap<K, V>
where
    Q: ?Sized + Comparable<K>,
{
    map_query_trait_funcs!();
}

impl<K, V> MapIteration<K, V> for EytzingerSearchMap<K, V> {
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

impl<K, V> Len for EytzingerSearchMap<K, V> {
    len_trait_funcs!();
}

impl<Q, K, V> Index<&Q> for EytzingerSearchMap<K, V>
where
    Q: ?Sized + Comparable<K>,
{
    index_trait_funcs!();
}

impl<K, V> IntoIterator for EytzingerSearchMap<K, V> {
    into_iterator_trait_funcs!();
}

impl<'a, K, V> IntoIterator for &'a EytzingerSearchMap<K, V> {
    into_iterator_trait_ref_funcs!();
}

impl<'a, K, V> IntoIterator for &'a mut EytzingerSearchMap<K, V> {
    into_iterator_trait_mut_ref_funcs!();
}

impl<K, V, MT> PartialEq<MT> for EytzingerSearchMap<K, V>
where
    K: Ord,
    V: PartialEq,
    MT: MapQuery<K, V>,
{
    partial_eq_trait_funcs!();
}

impl<K, V> Eq for EytzingerSearchMap<K, V>
where
    K: Ord,
    V: Eq,
{
}

impl<K, V> Debug for EytzingerSearchMap<K, V>
where
    K: Debug,
    V: Debug,
{
    debug_trait_funcs!();
}

#[cfg(feature = "serde")]
impl<K, V> Serialize for EytzingerSearchMap<K, V>
where
    K: Serialize,
    V: Serialize,
{
    serialize_trait_funcs!();
}
