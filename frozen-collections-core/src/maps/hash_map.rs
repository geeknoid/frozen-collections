use crate::DefaultHashBuilder;
use crate::hash_tables::HashTable;
use crate::hashers::BridgeHasher;
use crate::maps::decl_macros::{
    common_primary_funcs, debug_trait_funcs, get_disjoint_mut_funcs, hash_primary_funcs, index_trait_funcs, into_iterator_trait_funcs,
    into_iterator_trait_mut_ref_funcs, into_iterator_trait_ref_funcs, len_trait_funcs, map_extras_trait_funcs, map_iteration_trait_funcs,
    map_query_trait_funcs, partial_eq_trait_funcs,
};
use crate::maps::{IntoIter, IntoKeys, IntoValues, Iter, IterMut, Keys, Values, ValuesMut};
use crate::traits::{CollectionMagnitude, Hasher, Len, Map, MapExtras, MapIteration, MapQuery, SmallCollection};
use crate::utils::dedup_by_hash_keep_last;
use core::fmt::{Debug, Formatter, Result};
use core::hash::Hash;
use core::ops::Index;
use equivalent::Equivalent;

#[cfg(not(feature = "std"))]
use {alloc::string::String, alloc::vec::Vec};

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
#[derive(Clone)]
pub struct HashMap<K, V, CM = SmallCollection, H = BridgeHasher> {
    entries: HashTable<(K, V), CM>,
    hasher: H,
}

impl<K, V, CM> HashMap<K, V, CM, BridgeHasher<DefaultHashBuilder>>
where
    CM: CollectionMagnitude,
{
    /// Creates a frozen map.
    ///
    /// # Errors
    ///
    /// Fails if the number of entries in the vector, after deduplication, exceeds the
    /// magnitude of the collection as specified by the `CM` generic argument.
    pub fn new(entries: Vec<(K, V)>) -> core::result::Result<Self, String>
    where
        K: Hash + Eq,
    {
        Self::with_hasher(entries, BridgeHasher::default())
    }
}

impl<K, V, CM, H> HashMap<K, V, CM, H>
where
    CM: CollectionMagnitude,
{
    /// Creates a frozen map.
    ///
    /// # Errors
    ///
    /// Fails if the number of entries in the vector, after deduplication, exceeds the
    /// magnitude of the collection as specified by the `CM` generic argument.
    pub fn with_hasher(mut entries: Vec<(K, V)>, hasher: H) -> core::result::Result<Self, String>
    where
        K: Eq,
        H: Hasher<K>,
    {
        dedup_by_hash_keep_last(&mut entries, |x| hasher.hash_one(&x.0), |x, y| x.0 == y.0);

        Self::with_hasher_half_baked(entries, hasher)
    }

    /// Creates a frozen map.
    ///
    /// # Errors
    ///
    /// Fails if the number of entries in the vector, after deduplication, exceeds the
    /// magnitude of the collection as specified by the `CM` generic argument.
    pub(crate) fn with_hasher_half_baked(processed_entries: Vec<(K, V)>, hasher: H) -> core::result::Result<Self, String>
    where
        H: Hasher<K>,
    {
        let c = &hasher;
        let h = |entry: &(K, V)| c.hash_one(&entry.0);
        Ok(Self::new_raw(HashTable::<(K, V), CM>::new(processed_entries, h)?, hasher))
    }

    /// Creates a frozen map.
    pub(crate) const fn new_raw(table: HashTable<(K, V), CM>, hasher: H) -> Self {
        Self { entries: table, hasher }
    }

    hash_primary_funcs!();
    common_primary_funcs!(non_const_len, entries entries);
}

impl<K, V, CM, H> Default for HashMap<K, V, CM, H>
where
    CM: CollectionMagnitude,
    H: Default,
{
    fn default() -> Self {
        Self {
            entries: HashTable::default(),
            hasher: H::default(),
        }
    }
}

impl<K, V, Q, CM, H> Map<K, V, Q> for HashMap<K, V, CM, H>
where
    CM: CollectionMagnitude,
    Q: ?Sized + Equivalent<K>,
    H: Hasher<Q>,
{
}

impl<K, V, Q, CM, H> MapExtras<K, V, Q> for HashMap<K, V, CM, H>
where
    CM: CollectionMagnitude,
    Q: ?Sized + Equivalent<K>,
    H: Hasher<Q>,
{
    map_extras_trait_funcs!();
}

impl<K, V, Q, CM, H> MapQuery<Q, V> for HashMap<K, V, CM, H>
where
    CM: CollectionMagnitude,
    Q: ?Sized + Equivalent<K>,
    H: Hasher<Q>,
{
    map_query_trait_funcs!();
}

impl<K, V, CM, H> MapIteration<K, V> for HashMap<K, V, CM, H>
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

impl<K, V, CM, H> Len for HashMap<K, V, CM, H>
where
    CM: CollectionMagnitude,
{
    len_trait_funcs!();
}

impl<Q, K, V, CM, H> Index<&Q> for HashMap<K, V, CM, H>
where
    Q: ?Sized + Equivalent<K>,
    CM: CollectionMagnitude,
    H: Hasher<Q>,
{
    index_trait_funcs!();
}

impl<K, V, CM, H> IntoIterator for HashMap<K, V, CM, H>
where
    CM: CollectionMagnitude,
{
    into_iterator_trait_funcs!();
}

impl<'a, K, V, CM, H> IntoIterator for &'a HashMap<K, V, CM, H>
where
    CM: CollectionMagnitude,
{
    into_iterator_trait_ref_funcs!();
}

impl<'a, K, V, CM, H> IntoIterator for &'a mut HashMap<K, V, CM, H>
where
    CM: CollectionMagnitude,
{
    into_iterator_trait_mut_ref_funcs!();
}

impl<K, V, MT, CM, H> PartialEq<MT> for HashMap<K, V, CM, H>
where
    K: PartialEq,
    V: PartialEq,
    MT: MapQuery<K, V>,
    CM: CollectionMagnitude,
    H: Hasher<K>,
{
    partial_eq_trait_funcs!();
}

impl<K, V, CM, H> Eq for HashMap<K, V, CM, H>
where
    K: Eq,
    V: Eq,
    CM: CollectionMagnitude,
    H: Hasher<K>,
{
}

impl<K, V, CM, H> Debug for HashMap<K, V, CM, H>
where
    K: Debug,
    V: Debug,
    CM: CollectionMagnitude,
{
    debug_trait_funcs!();
}

#[cfg(feature = "serde")]
impl<K, V, CM, H> Serialize for HashMap<K, V, CM, H>
where
    K: Serialize,
    V: Serialize,
    CM: CollectionMagnitude,
{
    serialize_trait_funcs!();
}

#[cfg(test)]
mod test {
    use crate::hashers::BridgeHasher;
    use crate::maps::HashMap;
    use crate::traits::SmallCollection;

    #[test]
    fn fails_when_not_in_magnitude() {
        let mut input: Vec<(i32, i32)> = Vec::new();
        for i in 0..255 {
            input.push((i, i));
        }

        assert!(HashMap::<_, _, SmallCollection, BridgeHasher>::with_hasher(input, BridgeHasher::default()).is_ok());

        let mut input: Vec<(i32, i32)> = Vec::new();
        for i in 0..256 {
            input.push((i, i));
        }

        assert!(HashMap::<_, _, SmallCollection, BridgeHasher>::with_hasher(input, BridgeHasher::default()).is_err());
    }
}
