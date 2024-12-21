use crate::maps::decl_macros::{
    debug_fn, get_many_mut_body, get_many_mut_fn, index_fn, into_iter_fn, into_iter_mut_ref_fn,
    into_iter_ref_fn, map_iteration_funcs, ordered_scan_query_funcs, partial_eq_fn,
};
use crate::maps::{IntoIter, IntoKeys, IntoValues, Iter, IterMut, Keys, Values, ValuesMut};
use crate::traits::{Len, Map, MapIteration, MapQuery};
use alloc::vec::Vec;
use core::cmp::Ordering;
use core::fmt::{Debug, Formatter, Result};
use core::ops::Index;
use equivalent::Comparable;

/// A general purpose map implemented using linear scanning.
///
#[doc = include_str!("../doc_snippets/type_compat_warning.md")]
#[doc = include_str!("../doc_snippets/about.md")]
#[doc = include_str!("../doc_snippets/order_warning.md")]
///
/// # Type Parameters
///
/// - `K`: The key type.
/// - `V`: The value type.
/// - `SZ`: The number of entries in the map.
#[derive(Clone)]
pub struct InlineOrderedScanMap<K, V, const SZ: usize> {
    entries: [(K, V); SZ],
}

impl<K, V, const SZ: usize> InlineOrderedScanMap<K, V, SZ>
where
    K: Ord,
{
    /// Creates a frozen map.
    ///
    /// This function assumes the vector is sorted according to the ordering of the [`Ord`] trait.
    #[must_use]
    pub const fn new_raw(processed_entries: [(K, V); SZ]) -> Self {
        Self {
            entries: processed_entries,
        }
    }
}

impl<K, V, Q, const SZ: usize> Map<K, V, Q> for InlineOrderedScanMap<K, V, SZ>
where
    Q: ?Sized + Eq + Comparable<K>,
{
    get_many_mut_fn!();
}

impl<K, V, Q, const SZ: usize> MapQuery<K, V, Q> for InlineOrderedScanMap<K, V, SZ>
where
    Q: ?Sized + Eq + Comparable<K>,
{
    ordered_scan_query_funcs!();
}

impl<K, V, const SZ: usize> MapIteration<K, V> for InlineOrderedScanMap<K, V, SZ> {
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

    map_iteration_funcs!(entries);
}

impl<K, V, const SZ: usize> Len for InlineOrderedScanMap<K, V, SZ> {
    fn len(&self) -> usize {
        SZ
    }
}

impl<Q, K, V, const SZ: usize> Index<&Q> for InlineOrderedScanMap<K, V, SZ>
where
    Q: ?Sized + Eq + Comparable<K>,
{
    index_fn!();
}

impl<K, V, const SZ: usize> IntoIterator for InlineOrderedScanMap<K, V, SZ> {
    into_iter_fn!(entries);
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

impl<K, V, const N: usize> Eq for InlineOrderedScanMap<K, V, N>
where
    K: Ord,
    V: PartialEq,
{
}

impl<K, V, const SZ: usize> Debug for InlineOrderedScanMap<K, V, SZ>
where
    K: Debug,
    V: Debug,
{
    debug_fn!();
}