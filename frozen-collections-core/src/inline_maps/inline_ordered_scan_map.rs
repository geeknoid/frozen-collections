use crate::maps::decl_macros::{
    contains_key_fn, debug_fn, get_many_mut_body, get_many_mut_fn, index_fn, index_mut_fn,
    into_iter_fn_for_slice, into_iter_mut_ref_fn, into_iter_ref_fn, map_boilerplate_for_slice,
    map_iterator_boilerplate_for_slice, ordered_scan_core, partial_eq_fn,
};
use crate::maps::{IntoIter, IntoKeys, IntoValues, Iter, IterMut, Keys, Values, ValuesMut};
use crate::traits::{Len, Map, MapIterator};
use alloc::vec::Vec;
use core::borrow::Borrow;
use core::fmt::{Debug, Formatter, Result};
use core::ops::{Index, IndexMut};
use std::cmp::Ordering;

/// A general purpose map implemented using linear scanning.
///
/// # Type Parameters
///
/// - `K`: The key type.
/// - `V`: The value type.
/// - `SZ`: The number of entries in the map.
///
/// # Compatibility Note
///
/// This type is an implementation detail of the `frozen-collections` crate.
/// This API is therefore not stable and may change at any time.
#[derive(Clone)]
pub struct InlineOrderedScanMap<K, V, const SZ: usize> {
    entries: [(K, V); SZ],
}

impl<K, V, const SZ: usize> InlineOrderedScanMap<K, V, SZ> {
    pub const fn new(entries: [(K, V); SZ]) -> Self {
        Self { entries }
    }
}

impl<K, V, const SZ: usize> InlineOrderedScanMap<K, V, SZ> {
    ordered_scan_core!();
}

impl<K, V, const SZ: usize> Len for InlineOrderedScanMap<K, V, SZ> {
    fn len(&self) -> usize {
        SZ
    }
}

impl<K, V, const SZ: usize> Debug for InlineOrderedScanMap<K, V, SZ>
where
    K: Debug,
    V: Debug,
{
    debug_fn!();
}

impl<Q, K, V, const SZ: usize> Index<&Q> for InlineOrderedScanMap<K, V, SZ>
where
    K: Borrow<Q>,
    Q: ?Sized + Ord,
{
    index_fn!();
}

impl<Q, K, V, const SZ: usize> IndexMut<&Q> for InlineOrderedScanMap<K, V, SZ>
where
    K: Borrow<Q>,
    Q: ?Sized + Ord,
{
    index_mut_fn!();
}

impl<K, V, const SZ: usize> IntoIterator for InlineOrderedScanMap<K, V, SZ> {
    into_iter_fn_for_slice!(entries);
}

impl<'a, K, V, const SZ: usize> IntoIterator for &'a InlineOrderedScanMap<K, V, SZ> {
    into_iter_ref_fn!();
}

impl<'a, K, V, const SZ: usize> IntoIterator for &'a mut InlineOrderedScanMap<K, V, SZ> {
    into_iter_mut_ref_fn!();
}

impl<K, V, MT, const N: usize> PartialEq<MT> for InlineOrderedScanMap<K, V, N>
where
    K: Ord,
    V: PartialEq,
    MT: Map<K, V>,
{
    partial_eq_fn!();
}

impl<K, V, const SZ: usize> MapIterator<K, V> for InlineOrderedScanMap<K, V, SZ> {
    type Iterator<'a> = Iter<'a, K, V>
    where
        K: 'a,
        V: 'a;

    type KeyIterator<'a> = Keys<'a, K, V>
    where
        K: 'a,
        V: 'a;

    type ValueIterator<'a> = Values<'a, K, V>
    where
        K: 'a,
        V: 'a;

    type MutIterator<'a> = IterMut<'a, K, V>
    where
        K: 'a,
        V: 'a;

    type ValueMutIterator<'a> = ValuesMut<'a, K, V>
    where
        K: 'a,
        V: 'a;

    map_iterator_boilerplate_for_slice!(entries);
}

impl<K, V, const SZ: usize> Map<K, V> for InlineOrderedScanMap<K, V, SZ>
where
    K: Ord,
{
    map_boilerplate_for_slice!(entries);
}
