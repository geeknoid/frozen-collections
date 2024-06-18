use crate::hashers::BridgeHasher;
use crate::maps::decl_macros::{
    hash_core, index_fn, index_mut_fn, into_iter_fn_for_slice, into_iter_mut_ref_fn,
    into_iter_ref_fn, map_boilerplate_for_slice, map_iterator_boilerplate_for_slice, partial_eq_fn,
};
use crate::maps::hash_table::HashTable;
use crate::maps::{IntoIter, IntoKeys, IntoValues, Iter, IterMut, Keys, Values, ValuesMut};
use crate::traits::{CollectionMagnitude, Hasher, Len, Map, MapIterator};
use ahash::RandomState;
use alloc::vec::Vec;
use core::borrow::Borrow;
use core::fmt::{Debug, Formatter, Result};
use core::hash::Hash;
use core::ops::{Index, IndexMut};

/// A general purpose map implemented using a hash table.
///
/// # Compatibility Note
///
/// This type is an implementation detail of the `frozen-collections` crate.
/// This API is therefore not stable and may change at any time.
#[derive(Clone)]
pub struct HashMap<K, V, CM, H = BridgeHasher<RandomState>> {
    table: HashTable<(K, V), CM>,
    hasher: H,
}

impl<K, V, CM> HashMap<K, V, CM, BridgeHasher<RandomState>>
where
    K: Hash + Eq,
    CM: CollectionMagnitude,
{
    #[allow(clippy::missing_errors_doc)]
    pub fn new(entries: Vec<(K, V)>, num_hash_slots: usize) -> core::result::Result<Self, String> {
        Self::with_hasher(
            entries,
            num_hash_slots,
            BridgeHasher::new(RandomState::new()),
        )
    }
}

impl<K, V, CM, H> HashMap<K, V, CM, H>
where
    K: Eq,
    CM: CollectionMagnitude,
    H: Hasher<K>,
{
    #[allow(clippy::missing_errors_doc)]
    pub fn with_hasher(
        entries: Vec<(K, V)>,
        num_hash_slots: usize,
        hasher: H,
    ) -> core::result::Result<Self, String> {
        let c = &hasher;
        let h = |entry: &(K, V)| c.hash(&entry.0);
        let table = HashTable::<(K, V), CM>::new(entries, num_hash_slots, h)?;

        Ok(Self { table, hasher })
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

impl<Q, K, V, CM, H> IndexMut<&Q> for HashMap<K, V, CM, H>
where
    K: Borrow<Q>,
    Q: ?Sized + Eq,
    CM: CollectionMagnitude,
    H: Hasher<Q>,
{
    index_mut_fn!();
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
    type Iterator<'a> = Iter<'a, K, V>
    where
        K: 'a,
        V: 'a,
        CM: 'a,
        H: 'a;

    type KeyIterator<'a> = Keys<'a, K, V>
    where
        K: 'a,
        V: 'a,
        CM: 'a,
        H: 'a;

    type ValueIterator<'a> = Values<'a, K, V>
    where
        K: 'a,
        V: 'a,
        CM: 'a,
        H: 'a;

    type MutIterator<'a> = IterMut<'a, K, V>
    where
        K: 'a,
        V: 'a,
        CM: 'a,
        H: 'a;

    type ValueMutIterator<'a> = ValuesMut<'a, K, V>
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
    use crate::traits::SmallCollection;

    #[test]
    fn test_latest_key_appears_in_final_map() {
        let entries = vec![
            ("key1", "value1"),
            ("key2", "value2"),
            ("key1", "latest_value1"),
        ];
        let num_hash_slots = 16;
        let map = HashMap::<&str, &str, SmallCollection>::new(entries, num_hash_slots).unwrap();

        assert_eq!(map.get(&"key1"), Some(&"latest_value1"));
        assert_eq!(map.get(&"key2"), Some(&"value2"));
    }
}
