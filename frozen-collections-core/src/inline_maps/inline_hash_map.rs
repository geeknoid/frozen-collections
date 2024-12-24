use crate::hash_tables::InlineHashTable;
use crate::hashers::BridgeHasher;
use crate::maps::decl_macros::{
    get_many_mut_body, get_many_mut_fn, hash_query_funcs, index_fn, into_iter_fn,
    into_iter_mut_ref_fn, into_iter_ref_fn, map_iteration_funcs, partial_eq_fn,
};
use crate::maps::{IntoIter, IntoKeys, IntoValues, Iter, IterMut, Keys, Values, ValuesMut};
use crate::traits::{
    CollectionMagnitude, Hasher, Len, Map, MapIteration, MapQuery, SmallCollection,
};
use alloc::vec::Vec;
use core::fmt::{Debug, Formatter, Result};
use core::ops::Index;
use equivalent::Equivalent;

#[cfg(feature = "serde")]
use {
    crate::maps::decl_macros::serialize_fn,
    serde::ser::SerializeMap,
    serde::{Serialize, Serializer},
};

/// A general purpose map implemented using a hash table.
///
#[doc = include_str!("../doc_snippets/type_compat_warning.md")]
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
pub struct InlineHashMap<
    K,
    V,
    const SZ: usize,
    const NHS: usize,
    CM = SmallCollection,
    H = BridgeHasher,
> {
    table: InlineHashTable<(K, V), SZ, NHS, CM>,
    hasher: H,
}

impl<K, V, const SZ: usize, const NHS: usize, CM, H> InlineHashMap<K, V, SZ, NHS, CM, H>
where
    K: Eq,
    CM: CollectionMagnitude,
    H: Hasher<K>,
{
    /// Creates a frozen map.
    #[must_use]
    pub const fn new_raw(table: InlineHashTable<(K, V), SZ, NHS, CM>, hasher: H) -> Self {
        Self { table, hasher }
    }
}

impl<K, V, Q, const SZ: usize, const NHS: usize, CM, H> Map<K, V, Q>
    for InlineHashMap<K, V, SZ, NHS, CM, H>
where
    CM: CollectionMagnitude,
    Q: ?Sized + Eq + Equivalent<K>,
    H: Hasher<Q>,
{
    get_many_mut_fn!("Hash");
}

impl<K, V, Q, const SZ: usize, const NHS: usize, CM, H> MapQuery<K, V, Q>
    for InlineHashMap<K, V, SZ, NHS, CM, H>
where
    CM: CollectionMagnitude,
    Q: ?Sized + Eq + Equivalent<K>,
    H: Hasher<Q>,
{
    hash_query_funcs!();
}

impl<K, V, const SZ: usize, const NHS: usize, CM, H> MapIteration<K, V>
    for InlineHashMap<K, V, SZ, NHS, CM, H>
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

    map_iteration_funcs!(table entries);
}

impl<K, V, const SZ: usize, const NHS: usize, CM, H> Len for InlineHashMap<K, V, SZ, NHS, CM, H> {
    fn len(&self) -> usize {
        SZ
    }
}

impl<Q, K, V, const SZ: usize, const NHS: usize, CM, H> Index<&Q>
    for InlineHashMap<K, V, SZ, NHS, CM, H>
where
    CM: CollectionMagnitude,
    Q: ?Sized + Eq + Equivalent<K>,
    H: Hasher<Q>,
{
    index_fn!();
}

impl<K, V, const SZ: usize, const NHS: usize, CM, H> IntoIterator
    for InlineHashMap<K, V, SZ, NHS, CM, H>
{
    into_iter_fn!(table entries);
}

impl<'a, K, V, const SZ: usize, const NHS: usize, CM, H> IntoIterator
    for &'a InlineHashMap<K, V, SZ, NHS, CM, H>
{
    into_iter_ref_fn!();
}

impl<'a, K, V, const SZ: usize, const NHS: usize, CM, H> IntoIterator
    for &'a mut InlineHashMap<K, V, SZ, NHS, CM, H>
{
    into_iter_mut_ref_fn!();
}

impl<K, V, MT, const SZ: usize, const NHS: usize, CM, H> PartialEq<MT>
    for InlineHashMap<K, V, SZ, NHS, CM, H>
where
    K: Eq,
    CM: CollectionMagnitude,
    V: PartialEq,
    MT: Map<K, V>,
    H: Hasher<K>,
{
    partial_eq_fn!();
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
{
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        let pairs = self.table.entries.iter().map(|x| (&x.0, &x.1));
        f.debug_map().entries(pairs).finish()
    }
}

#[cfg(feature = "serde")]
impl<K, V, const SZ: usize, const NHS: usize, CM, H> Serialize
    for InlineHashMap<K, V, SZ, NHS, CM, H>
where
    K: Serialize,
    V: Serialize,
{
    serialize_fn!();
}
