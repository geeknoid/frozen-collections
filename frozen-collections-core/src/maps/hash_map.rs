use crate::hash_tables::HashTable;
use crate::hashers::BridgeHasher;
use crate::maps::decl_macros::{
    hash_core, index_fn, into_iter_fn_for_slice, into_iter_mut_ref_fn, into_iter_ref_fn,
    map_boilerplate_for_slice, map_iterator_boilerplate_for_slice, partial_eq_fn,
};
use crate::maps::{IntoIter, IntoKeys, IntoValues, Iter, IterMut, Keys, Values, ValuesMut};
use crate::traits::{CollectionMagnitude, Hasher, Len, Map, MapIterator, SmallCollection};
use crate::utils::dedup_by_hash_keep_last;
use alloc::vec::Vec;
use core::borrow::Borrow;
use core::fmt::{Debug, Formatter, Result};
use core::ops::Index;

/// A general purpose map implemented using a hash table.
///
#[doc = include_str!("../doc_snippets/type_compat_warning.md")]
#[doc = include_str!("../doc_snippets/about.md")]
#[doc = include_str!("../doc_snippets/hash_warning.md")]
///
#[derive(Clone)]
pub struct HashMap<K, V, CM = SmallCollection, H = BridgeHasher> {
    table: HashTable<(K, V), CM>,
    hasher: H,
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
    pub fn new(mut entries: Vec<(K, V)>, hasher: H) -> core::result::Result<Self, String> {
        dedup_by_hash_keep_last(&mut entries, &hasher);
        Self::new_half_baked(entries, hasher)
    }

    /// Creates a frozen map.
    pub(crate) fn new_half_baked(
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

impl<K, V, CM, H> HashMap<K, V, CM, H>
where
    CM: CollectionMagnitude,
{
    hash_core!();
}

impl<K, V, CM, H> Len for HashMap<K, V, CM, H> {
    fn len(&self) -> usize {
        self.table.len()
    }
}

impl<K, V, CM, H> Default for HashMap<K, V, CM, H>
where
    H: Default,
    CM: CollectionMagnitude,
{
    fn default() -> Self {
        Self {
            table: HashTable::default(),
            hasher: H::default(),
        }
    }
}

impl<K, V, CM, H> Debug for HashMap<K, V, CM, H>
where
    K: Debug,
    V: Debug,
{
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        self.table.fmt(f)
    }
}

impl<Q, K, V, CM, H> Index<&Q> for HashMap<K, V, CM, H>
where
    K: Borrow<Q>,
    Q: ?Sized + Eq,
    CM: CollectionMagnitude,
    H: Hasher<Q>,
{
    index_fn!();
}

impl<K, V, CM, H> IntoIterator for HashMap<K, V, CM, H> {
    into_iter_fn_for_slice!(table entries);
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
    CM: CollectionMagnitude,
    V: PartialEq,
    MT: Map<K, V>,
    H: Hasher<K>,
{
    partial_eq_fn!();
}

impl<K, V, CM, H> Eq for HashMap<K, V, CM, H>
where
    K: Eq,
    CM: CollectionMagnitude,
    V: Eq,
    H: Hasher<K>,
{
}

impl<K, V, CM, H> MapIterator<K, V> for HashMap<K, V, CM, H> {
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

impl<K, V, CM, H> Map<K, V> for HashMap<K, V, CM, H>
where
    K: Eq,
    CM: CollectionMagnitude,
    H: Hasher<K>,
{
    map_boilerplate_for_slice!(table entries);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_fails_when_entries_exceed_collection_magnitude() {
        let mut entries = Vec::new();
        for i in 0..=SmallCollection::MAX_CAPACITY {
            entries.push((i, 42));
        }

        let map = HashMap::<_, _, SmallCollection>::new(entries, BridgeHasher::default());
        assert_eq!(
            map,
            Err("too many entries for the selected collection magnitude".to_string())
        );
    }
}
