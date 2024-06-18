use crate::maps::decl_macros::get_many_mut_body;
use crate::maps::decl_macros::{
    debug_fn, get_many_mut_fn, index_fn, into_iter_fn, into_iter_mut_ref_fn, into_iter_ref_fn,
    map_iteration_funcs, partial_eq_fn, scan_query_funcs,
};
use crate::maps::{IntoIter, IntoKeys, IntoValues, Iter, IterMut, Keys, Values, ValuesMut};
use crate::traits::{Len, Map, MapIteration, MapQuery};
use alloc::vec::Vec;
use core::fmt::{Debug, Formatter, Result};
use core::ops::Index;
use equivalent::Equivalent;

/// A general purpose map implemented using linear scanning.
///
#[doc = include_str!("../doc_snippets/type_compat_warning.md")]
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

impl<K, V, const SZ: usize> InlineScanMap<K, V, SZ>
where
    K: Eq,
{
    /// Creates a frozen map.
    #[must_use]
    pub const fn new_raw(processed_entries: [(K, V); SZ]) -> Self {
        Self {
            entries: processed_entries,
        }
    }
}

impl<K, V, Q, const SZ: usize> Map<K, V, Q> for InlineScanMap<K, V, SZ>
where
    Q: ?Sized + Eq + Equivalent<K>,
{
    get_many_mut_fn!();
}

impl<K, V, Q, const SZ: usize> MapQuery<K, V, Q> for InlineScanMap<K, V, SZ>
where
    Q: ?Sized + Eq + Equivalent<K>,
{
    scan_query_funcs!();
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

    map_iteration_funcs!(entries);
}

impl<K, V, const SZ: usize> Len for InlineScanMap<K, V, SZ> {
    fn len(&self) -> usize {
        SZ
    }
}

impl<Q, K, V, const SZ: usize> Index<&Q> for InlineScanMap<K, V, SZ>
where
    Q: ?Sized + Eq + Equivalent<K>,
{
    index_fn!();
}

impl<K, V, const SZ: usize> IntoIterator for InlineScanMap<K, V, SZ> {
    into_iter_fn!(entries);
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

impl<K, V, const N: usize> Eq for InlineScanMap<K, V, N>
where
    K: Eq,
    V: PartialEq,
{
}

impl<K, V, const SZ: usize> Debug for InlineScanMap<K, V, SZ>
where
    K: Debug,
    V: Debug,
{
    debug_fn!();
}
