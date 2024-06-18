use alloc::vec::Vec;
use core::borrow::Borrow;
use core::fmt::{Debug, Formatter, Result};
use core::ops::{Index, IndexMut};

use crate::maps::decl_macros::{
    contains_key_fn, debug_fn, get_many_mut_body, get_many_mut_fn, index_fn, index_mut_fn,
    into_iter_fn_for_slice, into_iter_mut_ref_fn, into_iter_ref_fn, map_boilerplate_for_slice,
    map_iterator_boilerplate_for_slice, partial_eq_fn, sparse_sequence_lookup_core,
};
use crate::maps::{IntoIter, IntoKeys, IntoValues, Iter, IterMut, Keys, Values, ValuesMut};
use crate::traits::{CollectionMagnitude, Len, Map, MapIterator, Sequence};

/// A map whose keys are a sparse range of integers.
///
/// # Type Parameters
///
/// - `K`: The key type.
/// - `V`: The value type.
/// - `CM`: The magnitude of the map, one of [`SmallCollection`](crate::traits::SmallCollection), [`MediumCollection`](crate::traits::MediumCollection), or [`LargeCollection`](crate::traits::LargeCollection).
/// - `SZ`: The number of entries in the map.
/// - `LTSZ`: The number of entries in the lookup table.
///
/// # Compatibility Note
///
/// This type is an implementation detail of the `frozen-collections` crate.
/// This API is therefore not stable and may change at any time.
#[derive(Clone)]
pub struct InlineSparseSequenceLookupMap<K, V, CM, const SZ: usize, const LTSZ: usize> {
    min: K,
    max: K,
    lookup: [CM; LTSZ],
    entries: [(K, V); SZ],
}

impl<K, V, CM, const SZ: usize, const LTSZ: usize>
    InlineSparseSequenceLookupMap<K, V, CM, SZ, LTSZ>
{
    pub const fn new(min: K, max: K, lookup: [CM; LTSZ], entries: [(K, V); SZ]) -> Self {
        Self {
            min,
            max,
            lookup,
            entries,
        }
    }
}

impl<K, V, CM, const SZ: usize, const LTSZ: usize> InlineSparseSequenceLookupMap<K, V, CM, SZ, LTSZ>
where
    CM: CollectionMagnitude,
{
    sparse_sequence_lookup_core!();
}

impl<K, V, CM, const SZ: usize, const LTSZ: usize> Len
    for InlineSparseSequenceLookupMap<K, V, CM, SZ, LTSZ>
{
    fn len(&self) -> usize {
        SZ
    }
}

impl<K, V, CM, const SZ: usize, const LTSZ: usize> Debug
    for InlineSparseSequenceLookupMap<K, V, CM, SZ, LTSZ>
where
    K: Debug,
    V: Debug,
{
    debug_fn!();
}

impl<Q, K, V, CM, const SZ: usize, const LTSZ: usize> Index<&Q>
    for InlineSparseSequenceLookupMap<K, V, CM, SZ, LTSZ>
where
    K: Borrow<Q>,
    Q: Sequence,
    CM: CollectionMagnitude,
{
    index_fn!();
}

impl<Q, K, V, CM, const SZ: usize, const LTSZ: usize> IndexMut<&Q>
    for InlineSparseSequenceLookupMap<K, V, CM, SZ, LTSZ>
where
    K: Borrow<Q>,
    Q: Sequence,
    CM: CollectionMagnitude,
{
    index_mut_fn!();
}

impl<K, V, CM, const SZ: usize, const LTSZ: usize> IntoIterator
    for InlineSparseSequenceLookupMap<K, V, CM, SZ, LTSZ>
{
    into_iter_fn_for_slice!(entries);
}

impl<'a, K, V, CM, const SZ: usize, const LTSZ: usize> IntoIterator
    for &'a InlineSparseSequenceLookupMap<K, V, CM, SZ, LTSZ>
where
    CM: CollectionMagnitude,
{
    into_iter_ref_fn!();
}

impl<'a, K, V, CM, const SZ: usize, const LTSZ: usize> IntoIterator
    for &'a mut InlineSparseSequenceLookupMap<K, V, CM, SZ, LTSZ>
where
    CM: CollectionMagnitude,
{
    into_iter_mut_ref_fn!();
}

impl<K, V, CM, MT, const SZ: usize, const LTSZ: usize> PartialEq<MT>
    for InlineSparseSequenceLookupMap<K, V, CM, SZ, LTSZ>
where
    K: Sequence,
    V: PartialEq,
    MT: Map<K, V>,
    CM: CollectionMagnitude,
{
    partial_eq_fn!();
}

impl<K, V, CM, const SZ: usize, const LTSZ: usize> Eq
    for InlineSparseSequenceLookupMap<K, V, CM, SZ, LTSZ>
where
    K: Sequence,
    V: Eq,
    CM: CollectionMagnitude,
{
}

impl<K, V, CM, const SZ: usize, const LTSZ: usize> MapIterator<K, V>
    for InlineSparseSequenceLookupMap<K, V, CM, SZ, LTSZ>
{
    type Iterator<'a> = Iter<'a, K, V>
    where
        K: 'a,
        V: 'a,
        CM: 'a;

    type KeyIterator<'a> = Keys<'a, K, V>
    where
        K: 'a,
        V: 'a,
        CM: 'a;

    type ValueIterator<'a> = Values<'a, K, V>
    where
        K: 'a,
        V: 'a,
        CM: 'a;

    type MutIterator<'a> = IterMut<'a, K, V>
    where
        K: 'a,
        V: 'a,
        CM: 'a;

    type ValueMutIterator<'a> = ValuesMut<'a, K, V>
    where
        K: 'a,
        V: 'a,
        CM: 'a;

    map_iterator_boilerplate_for_slice!(entries);
}

impl<K, V, CM, const SZ: usize, const LTSZ: usize> Map<K, V>
    for InlineSparseSequenceLookupMap<K, V, CM, SZ, LTSZ>
where
    K: Sequence,
    CM: CollectionMagnitude,
{
    map_boilerplate_for_slice!(entries);
}
