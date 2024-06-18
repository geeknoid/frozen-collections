use alloc::vec::Vec;
use core::borrow::Borrow;
use core::fmt::{Debug, Formatter, Result};
use core::ops::{Index, IndexMut};

use crate::maps::decl_macros::{
    contains_key_fn, debug_fn, get_many_mut_body, get_many_mut_fn, index_fn, index_mut_fn,
    into_iter_fn_for_slice, into_iter_mut_ref_fn, into_iter_ref_fn, map_boilerplate_for_slice,
    map_iterator_boilerplate_for_slice, partial_eq_fn, scan_core,
};
use crate::maps::{IntoIter, IntoKeys, IntoValues, Iter, IterMut, Keys, Values, ValuesMut};
use crate::traits::{Len, Map, MapIterator};

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
pub struct InlineScanMap<K, V, const SZ: usize> {
    entries: [(K, V); SZ],
}

impl<K, V, const SZ: usize> InlineScanMap<K, V, SZ> {
    pub const fn new(entries: [(K, V); SZ]) -> Self {
        Self { entries }
    }
}

impl<K, V, const SZ: usize> InlineScanMap<K, V, SZ> {
    scan_core!();
}

impl<K, V, const SZ: usize> Len for InlineScanMap<K, V, SZ> {
    fn len(&self) -> usize {
        SZ
    }
}

impl<K, V, const SZ: usize> Debug for InlineScanMap<K, V, SZ>
where
    K: Debug,
    V: Debug,
{
    debug_fn!();
}

impl<Q, K, V, const SZ: usize> Index<&Q> for InlineScanMap<K, V, SZ>
where
    K: Borrow<Q>,
    Q: ?Sized + Eq,
{
    index_fn!();
}

impl<Q, K, V, const SZ: usize> IndexMut<&Q> for InlineScanMap<K, V, SZ>
where
    K: Borrow<Q>,
    Q: ?Sized + Eq,
{
    index_mut_fn!();
}

impl<K, V, const SZ: usize> IntoIterator for InlineScanMap<K, V, SZ> {
    into_iter_fn_for_slice!(entries);
}

impl<'a, K, V, const SZ: usize> IntoIterator for &'a InlineScanMap<K, V, SZ> {
    into_iter_ref_fn!();
}

impl<'a, K, V, const SZ: usize> IntoIterator for &'a mut InlineScanMap<K, V, SZ> {
    into_iter_mut_ref_fn!();
}

impl<K, V, MT, const N: usize> PartialEq<MT> for InlineScanMap<K, V, N>
where
    K: Eq,
    V: PartialEq,
    MT: Map<K, V>,
{
    partial_eq_fn!();
}

impl<K, V, const SZ: usize> MapIterator<K, V> for InlineScanMap<K, V, SZ> {
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

impl<K, V, const SZ: usize> Map<K, V> for InlineScanMap<K, V, SZ>
where
    K: Eq,
{
    map_boilerplate_for_slice!(entries);
}
