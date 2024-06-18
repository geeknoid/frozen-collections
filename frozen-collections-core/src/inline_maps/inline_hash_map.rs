use alloc::vec::Vec;
use core::borrow::Borrow;
use core::fmt::{Debug, Formatter, Result};
use core::ops::Index;

use crate::maps::decl_macros::{
    hash_core, index_fn, into_iter_fn_for_slice, into_iter_mut_ref_fn, into_iter_ref_fn,
    map_boilerplate_for_slice, map_iterator_boilerplate_for_slice, partial_eq_fn,
};
use crate::maps::{IntoIter, IntoKeys, IntoValues, Iter, IterMut, Keys, Values, ValuesMut};
use crate::traits::{CollectionMagnitude, Hasher, Len, Map, MapIterator};
use crate::utils::inline_hash_table::InlineHashTable;

/// A general purpose map implemented using a hash table.
///
/// # Type Parameters
///
/// - `K`: The key type.
/// - `V`: The value type.
/// - `CM`: The magnitude of the map, one of [`SmallCollection`](crate::traits::SmallCollection), [`MediumCollection`](crate::traits::MediumCollection), or [`LargeCollection`](crate::traits::LargeCollection).
/// - `SZ`: The number of entries in the map.
/// - `NHS`: The number of hash table slots.
/// - `H`: The hasher to generate hash codes.
///
/// # Compatibility Note
///
/// This type is an implementation detail of the `frozen-collections` crate.
/// This API is therefore not stable and may change at any time.
#[derive(Clone)]
pub struct InlineHashMap<K, V, CM, const SZ: usize, const NHS: usize, H> {
    table: InlineHashTable<(K, V), CM, SZ, NHS>,
    hasher: H,
}

impl<K, V, CM, const SZ: usize, const NHS: usize, H> InlineHashMap<K, V, CM, SZ, NHS, H>
where
    CM: CollectionMagnitude,
    H: Hasher<K>,
{
    #[allow(clippy::missing_errors_doc)]
    pub fn new(entries: Vec<(K, V)>, hasher: H) -> core::result::Result<Self, String> {
        let c = &hasher;
        let h = |entry: &(K, V)| c.hash(&entry.0);
        let table = InlineHashTable::<(K, V), CM, SZ, NHS>::new(entries, h)?;

        Ok(Self { table, hasher })
    }

    pub const fn new_raw(table: InlineHashTable<(K, V), CM, SZ, NHS>, bh: H) -> Self {
        Self { table, hasher: bh }
    }
}

impl<K, V, CM, const SZ: usize, const NHS: usize, H> InlineHashMap<K, V, CM, SZ, NHS, H>
where
    CM: CollectionMagnitude,
{
    hash_core!();
}

impl<K, V, CM, const SZ: usize, const NHS: usize, H> Len for InlineHashMap<K, V, CM, SZ, NHS, H> {
    fn len(&self) -> usize {
        SZ
    }
}

impl<K, V, CM, const SZ: usize, const NHS: usize, H> Debug for InlineHashMap<K, V, CM, SZ, NHS, H>
where
    K: Debug,
    V: Debug,
{
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        self.table.fmt(f)
    }
}

impl<Q, K, V, CM, const SZ: usize, const NHS: usize, H> Index<&Q>
    for InlineHashMap<K, V, CM, SZ, NHS, H>
where
    K: Borrow<Q>,
    Q: ?Sized + Eq,
    CM: CollectionMagnitude,
    H: Hasher<Q>,
{
    index_fn!();
}

impl<K, V, CM, const SZ: usize, const NHS: usize, H> IntoIterator
    for InlineHashMap<K, V, CM, SZ, NHS, H>
{
    into_iter_fn_for_slice!(table entries);
}

impl<'a, K, V, CM, const SZ: usize, const NHS: usize, H> IntoIterator
    for &'a InlineHashMap<K, V, CM, SZ, NHS, H>
{
    into_iter_ref_fn!();
}

impl<'a, K, V, CM, const SZ: usize, const NHS: usize, H> IntoIterator
    for &'a mut InlineHashMap<K, V, CM, SZ, NHS, H>
{
    into_iter_mut_ref_fn!();
}

impl<K, V, MT, CM, const SZ: usize, const NHS: usize, H> PartialEq<MT>
    for InlineHashMap<K, V, CM, SZ, NHS, H>
where
    K: Eq,
    CM: CollectionMagnitude,
    V: PartialEq,
    MT: Map<K, V>,
    H: Hasher<K>,
{
    partial_eq_fn!();
}

impl<K, V, CM, const SZ: usize, const NHS: usize, H> Eq for InlineHashMap<K, V, CM, SZ, NHS, H>
where
    K: Eq,
    CM: CollectionMagnitude,
    V: Eq,
    H: Hasher<K>,
{
}

impl<K, V, CM, const SZ: usize, const NHS: usize, H> MapIterator<K, V>
    for InlineHashMap<K, V, CM, SZ, NHS, H>
{
    type Iterator<'a>
        = Iter<'a, K, V>
    where
        K: 'a,
        V: 'a,
        CM: 'a,
        H: 'a;

    type KeyIterator<'a>
        = Keys<'a, K, V>
    where
        K: 'a,
        V: 'a,
        CM: 'a,
        H: 'a;

    type ValueIterator<'a>
        = Values<'a, K, V>
    where
        K: 'a,
        V: 'a,
        CM: 'a,
        H: 'a;

    type MutIterator<'a>
        = IterMut<'a, K, V>
    where
        K: 'a,
        V: 'a,
        CM: 'a,
        H: 'a;

    type ValueMutIterator<'a>
        = ValuesMut<'a, K, V>
    where
        K: 'a,
        V: 'a,
        CM: 'a,
        H: 'a;

    map_iterator_boilerplate_for_slice!(table entries);
}

impl<K, V, CM, const SZ: usize, const NHS: usize, H> Map<K, V>
    for InlineHashMap<K, V, CM, SZ, NHS, H>
where
    K: Eq,
    CM: CollectionMagnitude,
    H: Hasher<K>,
{
    map_boilerplate_for_slice!(table entries);
}
