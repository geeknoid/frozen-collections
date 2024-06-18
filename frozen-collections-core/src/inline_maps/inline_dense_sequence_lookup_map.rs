use alloc::vec::Vec;
use core::borrow::Borrow;
use core::fmt::{Debug, Formatter, Result};
use core::ops::{Index, IndexMut};

use crate::maps::decl_macros::{
    contains_key_fn, debug_fn, dense_sequence_lookup_core, get_many_mut_body, get_many_mut_fn,
    index_fn, index_mut_fn, into_iter_fn_for_slice, into_iter_mut_ref_fn, into_iter_ref_fn,
    map_boilerplate_for_slice, map_iterator_boilerplate_for_slice, partial_eq_fn,
};
use crate::maps::{IntoIter, IntoKeys, IntoValues, Iter, IterMut, Keys, Values, ValuesMut};
use crate::traits::{Len, Map, MapIterator, Sequence};

/// A map whose keys are a continuous range in a sequence.
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
pub struct InlineDenseSequenceLookupMap<K, V, const SZ: usize> {
    min: K,
    max: K,
    entries: [(K, V); SZ],
}

impl<K, V, const SZ: usize> InlineDenseSequenceLookupMap<K, V, SZ> {
    pub const fn new(min: K, max: K, entries: [(K, V); SZ]) -> Self {
        Self { min, max, entries }
    }
}

impl<K, V, const SZ: usize> InlineDenseSequenceLookupMap<K, V, SZ> {
    dense_sequence_lookup_core!();
}

impl<K, V, const SZ: usize> Len for InlineDenseSequenceLookupMap<K, V, SZ> {
    fn len(&self) -> usize {
        SZ
    }
}

impl<K, V, const SZ: usize> Debug for InlineDenseSequenceLookupMap<K, V, SZ>
where
    K: Debug,
    V: Debug,
{
    debug_fn!();
}

impl<Q, K, V, const SZ: usize> Index<&Q> for InlineDenseSequenceLookupMap<K, V, SZ>
where
    K: Borrow<Q>,
    Q: Sequence,
{
    index_fn!();
}

impl<Q, K, V, const SZ: usize> IndexMut<&Q> for InlineDenseSequenceLookupMap<K, V, SZ>
where
    K: Borrow<Q>,
    Q: Sequence,
{
    index_mut_fn!();
}

impl<K, V, const SZ: usize> IntoIterator for InlineDenseSequenceLookupMap<K, V, SZ> {
    into_iter_fn_for_slice!(entries);
}

impl<'a, K, V, const SZ: usize> IntoIterator for &'a InlineDenseSequenceLookupMap<K, V, SZ> {
    into_iter_ref_fn!();
}

impl<'a, K, V, const SZ: usize> IntoIterator for &'a mut InlineDenseSequenceLookupMap<K, V, SZ> {
    into_iter_mut_ref_fn!();
}

impl<K, V, MT, const SZ: usize> PartialEq<MT> for InlineDenseSequenceLookupMap<K, V, SZ>
where
    K: Sequence,
    V: PartialEq,
    MT: Map<K, V>,
{
    partial_eq_fn!();
}

impl<K, V, const SZ: usize> Eq for InlineDenseSequenceLookupMap<K, V, SZ>
where
    K: Sequence,
    V: Eq,
{
}

impl<K, V, const SZ: usize> MapIterator<K, V> for InlineDenseSequenceLookupMap<K, V, SZ> {
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

impl<K, V, const SZ: usize> Map<K, V> for InlineDenseSequenceLookupMap<K, V, SZ>
where
    K: Sequence,
{
    map_boilerplate_for_slice!(entries);
}
