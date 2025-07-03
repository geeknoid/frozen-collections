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
use alloc::vec::Vec;

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
/// # Type Parameters
///
/// - `K`: The key type.
/// - `V`: The value type.
/// - `SZ`: The number of entries in the map.
#[derive(Clone)]
pub struct InlineScanMap<K, V, const SZ: usize> {
    entries: [(K, V); SZ],
}

impl<K, V, const SZ: usize> InlineScanMap<K, V, SZ> {
    /// Creates a frozen map.
    ///
    /// # Panics
    ///
    /// Panics if the number of entries in the array differs from the size of the map as specified by the `SZ` generic argument.
    #[must_use]
    pub fn new(entries: Vec<(K, V)>) -> Self
    where
        K: Eq,
    {
        Self::new_raw(DeduppedVec::using_eq(entries, |x, y| x.0.eq(&y.0)).into_array())
    }

    /// Creates a frozen map.
    #[must_use]
    pub const fn new_raw(processed_entries: [(K, V); SZ]) -> Self {
        Self {
            entries: processed_entries,
        }
    }

    scan_primary_funcs!();
    common_primary_funcs!(const_len, entries);
}

impl<K, V, Q, const SZ: usize> Map<K, V, Q> for InlineScanMap<K, V, SZ> where Q: ?Sized + Equivalent<K> {}

impl<K, V, Q, const SZ: usize> MapExtras<K, V, Q> for InlineScanMap<K, V, SZ>
where
    Q: ?Sized + Equivalent<K>,
{
    map_extras_trait_funcs!();
}

impl<K, V, Q, const SZ: usize> MapQuery<Q, V> for InlineScanMap<K, V, SZ>
where
    Q: ?Sized + Equivalent<K>,
{
    map_query_trait_funcs!();
}

impl<K, V, const SZ: usize> MapIteration<K, V> for InlineScanMap<K, V, SZ> {
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

impl<K, V, const SZ: usize> Len for InlineScanMap<K, V, SZ> {
    len_trait_funcs!();
}

impl<Q, K, V, const SZ: usize> Index<&Q> for InlineScanMap<K, V, SZ>
where
    Q: ?Sized + Equivalent<K>,
{
    index_trait_funcs!();
}

impl<K, V, const SZ: usize> IntoIterator for InlineScanMap<K, V, SZ> {
    into_iterator_trait_funcs!();
}

impl<'a, K, V, const SZ: usize> IntoIterator for &'a InlineScanMap<K, V, SZ> {
    into_iterator_trait_ref_funcs!();
}

impl<'a, K, V, const SZ: usize> IntoIterator for &'a mut InlineScanMap<K, V, SZ> {
    into_iterator_trait_mut_ref_funcs!();
}

impl<K, V, MT, const N: usize> PartialEq<MT> for InlineScanMap<K, V, N>
where
    K: PartialEq,
    V: PartialEq,
    MT: MapQuery<K, V>,
{
    partial_eq_trait_funcs!();
}

impl<K, V, const N: usize> Eq for InlineScanMap<K, V, N>
where
    K: Eq,
    V: Eq,
{
}

impl<K, V, const SZ: usize> Debug for InlineScanMap<K, V, SZ>
where
    K: Debug,
    V: Debug,
{
    debug_trait_funcs!();
}

#[cfg(feature = "serde")]
impl<K, V, const SZ: usize> Serialize for InlineScanMap<K, V, SZ>
where
    K: Serialize,
    V: Serialize,
{
    serialize_trait_funcs!();
}
