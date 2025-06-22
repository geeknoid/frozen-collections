use crate::hash_tables::InlineHashTable;
use crate::hashers::BridgeHasher;
use crate::maps::decl_macros::{
    common_primary_funcs, debug_trait_funcs, get_disjoint_mut_funcs, hash_primary_funcs, index_trait_funcs, into_iterator_trait_funcs,
    into_iterator_trait_mut_ref_funcs, into_iterator_trait_ref_funcs, len_trait_funcs, map_extras_trait_funcs, map_iteration_trait_funcs,
    map_query_trait_funcs, partial_eq_trait_funcs,
};
use crate::maps::{IntoIter, IntoKeys, IntoValues, Iter, IterMut, Keys, Values, ValuesMut};
use crate::traits::{CollectionMagnitude, Hasher, Len, Map, MapExtras, MapIteration, MapQuery, SmallCollection};
use core::fmt::{Debug, Formatter, Result};
use core::ops::Index;
use equivalent::Equivalent;

#[cfg(feature = "serde")]
use {
    crate::maps::decl_macros::serialize_trait_funcs,
    serde::ser::SerializeMap,
    serde::{Serialize, Serializer},
};

/// A general-purpose map implemented using a hash table.
///
#[doc = include_str!("../doc_snippets/private_api_warning.md")]
#[doc = include_str!("../doc_snippets/about.md")]
#[doc = include_str!("../doc_snippets/hash_warning.md")]
///
/// # Type Parameters
///
/// - `K`: The key type.
/// - `V`: The value type.
/// - `CM`: The magnitude of the map, one of [`SmallCollection`](SmallCollection), [`MediumCollection`](crate::traits::MediumCollection), or [`LargeCollection`](crate::traits::LargeCollection).
/// - `SZ`: The number of entries in the map.
/// - `NHS`: The number of hash table slots.
/// - `H`: The hasher to generate hash codes.
#[derive(Clone)]
pub struct InlineHashMap<K, V, const SZ: usize, const NHS: usize, CM = SmallCollection, H = BridgeHasher> {
    entries: InlineHashTable<(K, V), SZ, NHS, CM>,
    hasher: H,
}

impl<K, V, const SZ: usize, const NHS: usize, CM, H> InlineHashMap<K, V, SZ, NHS, CM, H>
where
    CM: CollectionMagnitude,
{
    /// Creates a frozen map.
    #[must_use]
    pub const fn new_raw(table: InlineHashTable<(K, V), SZ, NHS, CM>, hasher: H) -> Self {
        Self { entries: table, hasher }
    }

    hash_primary_funcs!();
    common_primary_funcs!(const_len, entries entries);
}

impl<K, V, Q, const SZ: usize, const NHS: usize, CM, H> Map<K, V, Q> for InlineHashMap<K, V, SZ, NHS, CM, H>
where
    CM: CollectionMagnitude,
    Q: ?Sized + Equivalent<K>,
    H: Hasher<Q>,
{
}

impl<K, V, Q, const SZ: usize, const NHS: usize, CM, H> MapExtras<K, V, Q> for InlineHashMap<K, V, SZ, NHS, CM, H>
where
    CM: CollectionMagnitude,
    Q: ?Sized + Equivalent<K>,
    H: Hasher<Q>,
{
    map_extras_trait_funcs!();
}

impl<K, V, Q, const SZ: usize, const NHS: usize, CM, H> MapQuery<Q, V> for InlineHashMap<K, V, SZ, NHS, CM, H>
where
    CM: CollectionMagnitude,
    Q: ?Sized + Equivalent<K>,
    H: Hasher<Q>,
{
    map_query_trait_funcs!();
}

impl<K, V, const SZ: usize, const NHS: usize, CM, H> MapIteration<K, V> for InlineHashMap<K, V, SZ, NHS, CM, H>
where
    CM: CollectionMagnitude,
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

    map_iteration_trait_funcs!();
}

impl<K, V, const SZ: usize, const NHS: usize, CM, H> Len for InlineHashMap<K, V, SZ, NHS, CM, H>
where
    CM: CollectionMagnitude,
{
    len_trait_funcs!();
}

impl<Q, K, V, const SZ: usize, const NHS: usize, CM, H> Index<&Q> for InlineHashMap<K, V, SZ, NHS, CM, H>
where
    CM: CollectionMagnitude,
    Q: ?Sized + Equivalent<K>,
    H: Hasher<Q>,
{
    index_trait_funcs!();
}

impl<K, V, const SZ: usize, const NHS: usize, CM, H> IntoIterator for InlineHashMap<K, V, SZ, NHS, CM, H>
where
    CM: CollectionMagnitude,
{
    into_iterator_trait_funcs!();
}

impl<'a, K, V, const SZ: usize, const NHS: usize, CM, H> IntoIterator for &'a InlineHashMap<K, V, SZ, NHS, CM, H>
where
    CM: CollectionMagnitude,
{
    into_iterator_trait_ref_funcs!();
}

impl<'a, K, V, const SZ: usize, const NHS: usize, CM, H> IntoIterator for &'a mut InlineHashMap<K, V, SZ, NHS, CM, H>
where
    CM: CollectionMagnitude,
{
    into_iterator_trait_mut_ref_funcs!();
}

impl<K, V, MT, const SZ: usize, const NHS: usize, CM, H> PartialEq<MT> for InlineHashMap<K, V, SZ, NHS, CM, H>
where
    K: PartialEq,
    V: PartialEq,
    CM: CollectionMagnitude,
    MT: MapQuery<K, V>,
    H: Hasher<K>,
{
    partial_eq_trait_funcs!();
}

impl<K, V, const SZ: usize, const NHS: usize, CM, H> Eq for InlineHashMap<K, V, SZ, NHS, CM, H>
where
    K: Eq,
    CM: CollectionMagnitude,
    V: Eq,
    H: Hasher<K>,
{
}

impl<K, V, const SZ: usize, const NHS: usize, CM, H> Debug for InlineHashMap<K, V, SZ, NHS, CM, H>
where
    K: Debug,
    V: Debug,
    CM: CollectionMagnitude,
{
    debug_trait_funcs!();
}

#[cfg(feature = "serde")]
impl<K, V, const SZ: usize, const NHS: usize, CM, H> Serialize for InlineHashMap<K, V, SZ, NHS, CM, H>
where
    K: Serialize,
    V: Serialize,
    CM: CollectionMagnitude,
{
    serialize_trait_funcs!();
}
