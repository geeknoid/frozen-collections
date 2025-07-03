use crate::maps::decl_macros::{
    common_primary_funcs, debug_trait_funcs, get_disjoint_mut_funcs, index_trait_funcs, into_iterator_trait_funcs,
    into_iterator_trait_mut_ref_funcs, into_iterator_trait_ref_funcs, len_trait_funcs, map_extras_trait_funcs, map_iteration_trait_funcs,
    map_query_trait_funcs, partial_eq_trait_funcs, scan_primary_funcs,
};
use crate::maps::{IntoIter, IntoKeys, IntoValues, Iter, IterMut, Keys, Values, ValuesMut};
use crate::traits::{Len, Map, MapExtras, MapIteration, MapQuery};
use crate::utils::DeduppedVec;
use core::fmt::{Debug, Formatter, Result};
use core::ops::Index;
use equivalent::Equivalent;

#[cfg(not(feature = "std"))]
use {alloc::boxed::Box, alloc::vec::Vec};

#[cfg(feature = "serde")]
use {
    crate::maps::decl_macros::serialize_trait_funcs,
    serde::ser::SerializeMap,
    serde::{Serialize, Serializer},
};

/// A general-purpose map implemented using linear scanning.
///
#[doc = include_str!("../doc_snippets/private_api_warning.md")]
#[doc = include_str!("../doc_snippets/about.md")]
///
#[derive(Clone, Eq)]
pub struct ScanMap<K, V> {
    entries: Box<[(K, V)]>,
}

impl<K, V> ScanMap<K, V> {
    /// Creates a frozen map.
    #[must_use]
    pub fn new(entries: Vec<(K, V)>) -> Self
    where
        K: Eq,
    {
        Self::from_dedupped(DeduppedVec::using_eq(entries, |x, y| x.0.eq(&y.0)))
    }

    /// Creates a frozen map.
    #[must_use]
    pub(crate) fn from_dedupped(entries: DeduppedVec<(K, V)>) -> Self {
        Self {
            entries: entries.into_boxed_slice(),
        }
    }

    scan_primary_funcs!();
    common_primary_funcs!(non_const_len, entries);
}

impl<K, V> Default for ScanMap<K, V> {
    fn default() -> Self {
        Self { entries: Box::default() }
    }
}

impl<K, V, Q> Map<K, V, Q> for ScanMap<K, V> where Q: ?Sized + Equivalent<K> {}

impl<K, V, Q> MapExtras<K, V, Q> for ScanMap<K, V>
where
    Q: ?Sized + Equivalent<K>,
{
    map_extras_trait_funcs!();
}

impl<K, V, Q> MapQuery<Q, V> for ScanMap<K, V>
where
    Q: ?Sized + Equivalent<K>,
{
    map_query_trait_funcs!();
}

impl<K, V> MapIteration<K, V> for ScanMap<K, V> {
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

impl<K, V> Len for ScanMap<K, V> {
    len_trait_funcs!();
}

impl<Q, K, V> Index<&Q> for ScanMap<K, V>
where
    Q: ?Sized + Equivalent<K>,
{
    index_trait_funcs!();
}

impl<K, V> IntoIterator for ScanMap<K, V> {
    into_iterator_trait_funcs!();
}

impl<'a, K, V> IntoIterator for &'a ScanMap<K, V> {
    into_iterator_trait_ref_funcs!();
}

impl<'a, K, V> IntoIterator for &'a mut ScanMap<K, V> {
    into_iterator_trait_mut_ref_funcs!();
}

impl<K, V, MT> PartialEq<MT> for ScanMap<K, V>
where
    K: PartialEq,
    V: PartialEq,
    MT: MapQuery<K, V>,
{
    partial_eq_trait_funcs!();
}

impl<K, V> Debug for ScanMap<K, V>
where
    K: Debug,
    V: Debug,
{
    debug_trait_funcs!();
}

#[cfg(feature = "serde")]
impl<K, V> Serialize for ScanMap<K, V>
where
    K: Serialize,
    V: Serialize,
{
    serialize_trait_funcs!();
}
