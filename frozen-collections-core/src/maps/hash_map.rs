use crate::hash_tables::HashTable;
use crate::hashers::BridgeHasher;
use crate::maps::decl_macros::{
    get_many_mut_body, get_many_mut_fn, hash_query_funcs, index_fn, into_iter_fn,
    into_iter_mut_ref_fn, into_iter_ref_fn, map_iteration_funcs, partial_eq_fn,
};
use crate::maps::{IntoIter, IntoKeys, IntoValues, Iter, IterMut, Keys, Values, ValuesMut};
use crate::traits::{
    CollectionMagnitude, Hasher, Len, Map, MapIteration, MapQuery, SmallCollection,
};
use crate::utils::dedup_by_hash_keep_last;
use alloc::string::String;
use alloc::vec::Vec;
use core::fmt::{Debug, Formatter, Result};
use core::hash::Hash;
use core::ops::Index;
use equivalent::Equivalent;

use crate::DefaultHashBuilder;
#[cfg(feature = "serde")]
use {
    crate::maps::decl_macros::serialize_fn,
    serde::ser::SerializeMap,
    serde::{Serialize, Serializer},
};

/// A general purpose map implemented using a hash table.
///
#[doc = include_str!("../doc_snippets/private_api_warning.md")]
#[doc = include_str!("../doc_snippets/about.md")]
#[doc = include_str!("../doc_snippets/hash_warning.md")]
///
#[derive(Clone)]
pub struct HashMap<K, V, CM = SmallCollection, H = BridgeHasher> {
    table: HashTable<(K, V), CM>,
    hasher: H,
}

impl<K, V, CM> HashMap<K, V, CM, BridgeHasher<DefaultHashBuilder>>
where
    K: Hash + Eq,
    CM: CollectionMagnitude,
{
    /// Creates a frozen map.
    ///
    /// # Errors
    ///
    /// Fails if the number of entries in the vector, after deduplication, exceeds the
    /// magnitude of the collection as specified by the `CM` generic argument.
    pub fn new(entries: Vec<(K, V)>) -> core::result::Result<Self, String> {
        Self::with_hasher(entries, BridgeHasher::default())
    }
}

impl<K, V, CM, H> HashMap<K, V, CM, H>
where
    K: Eq,
    CM: CollectionMagnitude,
    H: Hasher<K>,
{
    /// Creates a frozen map.
    ///
    /// # Errors
    ///
    /// Fails if the number of entries in the vector, after deduplication, exceeds the
    /// magnitude of the collection as specified by the `CM` generic argument.
    pub fn with_hasher(mut entries: Vec<(K, V)>, hasher: H) -> core::result::Result<Self, String> {
        dedup_by_hash_keep_last(&mut entries, |x| hasher.hash(&x.0), |x, y| x.0 == y.0);

        Self::with_hasher_half_baked(entries, hasher)
    }

    /// Creates a frozen map.
    ///
    /// # Errors
    ///
    /// Fails if the number of entries in the vector, after deduplication, exceeds the
    /// magnitude of the collection as specified by the `CM` generic argument.
    pub(crate) fn with_hasher_half_baked(
        processed_entries: Vec<(K, V)>,
        hasher: H,
    ) -> core::result::Result<Self, String> {
        let c = &hasher;
        let h = |entry: &(K, V)| c.hash(&entry.0);
        Ok(Self::new_raw(
            HashTable::<(K, V), CM>::new(processed_entries, h)?,
            hasher,
        ))
    }

    /// Creates a frozen map.
    pub(crate) const fn new_raw(table: HashTable<(K, V), CM>, hasher: H) -> Self {
        Self { table, hasher }
    }
}

impl<K, V, CM, H> Default for HashMap<K, V, CM, H>
where
    CM: CollectionMagnitude,
    H: Default,
{
    fn default() -> Self {
        Self {
            table: HashTable::default(),
            hasher: H::default(),
        }
    }
}

impl<K, V, Q, CM, H> Map<K, V, Q> for HashMap<K, V, CM, H>
where
    CM: CollectionMagnitude,
    Q: ?Sized + Eq + Equivalent<K>,
    H: Hasher<Q>,
{
    get_many_mut_fn!("Hash");
}

impl<K, V, Q, CM, H> MapQuery<K, V, Q> for HashMap<K, V, CM, H>
where
    CM: CollectionMagnitude,
    Q: ?Sized + Eq + Equivalent<K>,
    H: Hasher<Q>,
{
    hash_query_funcs!();
}

impl<K, V, CM, H> MapIteration<K, V> for HashMap<K, V, CM, H> {
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

impl<K, V, CM, H> Len for HashMap<K, V, CM, H> {
    fn len(&self) -> usize {
        self.table.len()
    }
}

impl<Q, K, V, CM, H> Index<&Q> for HashMap<K, V, CM, H>
where
    Q: ?Sized + Eq + Equivalent<K>,
    CM: CollectionMagnitude,
    H: Hasher<Q>,
{
    index_fn!();
}

impl<K, V, CM, H> IntoIterator for HashMap<K, V, CM, H> {
    into_iter_fn!(table entries);
}

impl<'a, K, V, CM, H> IntoIterator for &'a HashMap<K, V, CM, H> {
    into_iter_ref_fn!();
}

impl<'a, K, V, CM, H> IntoIterator for &'a mut HashMap<K, V, CM, H> {
    into_iter_mut_ref_fn!();
}

impl<K, V, MT, CM, H> PartialEq<MT> for HashMap<K, V, CM, H>
where
    K: Eq,
    V: PartialEq,
    MT: Map<K, V>,
    CM: CollectionMagnitude,
    H: Hasher<K>,
{
    partial_eq_fn!();
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
{
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        let pairs = self.table.entries.iter().map(|x| (&x.0, &x.1));
        f.debug_map().entries(pairs).finish()
    }
}

#[cfg(feature = "serde")]
impl<K, V, CM, H> Serialize for HashMap<K, V, CM, H>
where
    K: Serialize,
    V: Serialize,
{
    serialize_fn!();
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

        assert!(HashMap::<_, _, SmallCollection, BridgeHasher>::with_hasher(
            input,
            BridgeHasher::default()
        )
        .is_ok());

        let mut input: Vec<(i32, i32)> = Vec::new();
        for i in 0..256 {
            input.push((i, i));
        }

        assert!(HashMap::<_, _, SmallCollection, BridgeHasher>::with_hasher(
            input,
            BridgeHasher::default()
        )
        .is_err());
    }
}
