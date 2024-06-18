use crate::hash_tables::HashTable;
use crate::hashers::BridgeHasher;
use crate::maps::decl_macros::{
    hash_core, index_fn, into_iter_fn_for_slice, into_iter_mut_ref_fn, into_iter_ref_fn,
    map_boilerplate_for_slice, map_iterator_boilerplate_for_slice, partial_eq_fn,
};
use crate::maps::{IntoIter, IntoKeys, IntoValues, Iter, IterMut, Keys, Values, ValuesMut};
use crate::traits::{Hasher, LargeCollection, Len, Map, MapIterator};
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
pub struct HashMap<K, V, H = BridgeHasher> {
    table: HashTable<(K, V), LargeCollection>,
    hasher: H,
}

impl<K, V, H> HashMap<K, V, H>
where
    K: Eq,
    H: Hasher<K>,
{
    /// Creates a frozen map.
    pub fn new(mut entries: Vec<(K, V)>, hasher: H) -> Self {
        dedup_by_hash_keep_last(&mut entries, &hasher);
        Self::new_half_baked(entries, hasher)
    }

    /// Creates a frozen map.
    pub(crate) fn new_half_baked(processed_entries: Vec<(K, V)>, hasher: H) -> Self {
        let c = &hasher;
        let h = |entry: &(K, V)| c.hash(&entry.0);
        Self::new_raw(
            HashTable::<(K, V), LargeCollection>::new(processed_entries, h).unwrap(),
            hasher,
        )
    }

    /// Creates a frozen map.
    pub(crate) const fn new_raw(table: HashTable<(K, V), LargeCollection>, hasher: H) -> Self {
        Self { table, hasher }
    }
}

impl<K, V, H> HashMap<K, V, H> {
    hash_core!();
}

impl<K, V, H> Len for HashMap<K, V, H> {
    fn len(&self) -> usize {
        self.table.len()
    }
}

impl<K, V, H> Default for HashMap<K, V, H>
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

impl<K, V, H> Debug for HashMap<K, V, H>
where
    K: Debug,
    V: Debug,
{
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        let pairs = self.table.entries.iter().map(|x| (&x.0, &x.1));
        f.debug_map().entries(pairs).finish()
    }
}

impl<Q, K, V, H> Index<&Q> for HashMap<K, V, H>
where
    K: Borrow<Q>,
    Q: ?Sized + Eq,
    H: Hasher<Q>,
{
    index_fn!();
}

impl<K, V, H> IntoIterator for HashMap<K, V, H> {
    into_iter_fn_for_slice!(table entries);
}

impl<'a, K, V, H> IntoIterator for &'a HashMap<K, V, H> {
    into_iter_ref_fn!();
}

impl<'a, K, V, H> IntoIterator for &'a mut HashMap<K, V, H> {
    into_iter_mut_ref_fn!();
}

impl<K, V, MT, H> PartialEq<MT> for HashMap<K, V, H>
where
    K: Eq,
    V: PartialEq,
    MT: Map<K, V>,
    H: Hasher<K>,
{
    partial_eq_fn!();
}

impl<K, V, H> Eq for HashMap<K, V, H>
where
    K: Eq,
    V: Eq,
    H: Hasher<K>,
{
}

impl<K, V, H> MapIterator<K, V> for HashMap<K, V, H> {
    type Iterator<'a>
        = Iter<'a, K, V>
    where
        K: 'a,
        V: 'a,
        H: 'a;

    type KeyIterator<'a>
        = Keys<'a, K, V>
    where
        K: 'a,
        V: 'a,
        H: 'a;

    type ValueIterator<'a>
        = Values<'a, K, V>
    where
        K: 'a,
        V: 'a,
        H: 'a;

    type MutIterator<'a>
        = IterMut<'a, K, V>
    where
        K: 'a,
        V: 'a,
        H: 'a;

    type ValueMutIterator<'a>
        = ValuesMut<'a, K, V>
    where
        K: 'a,
        V: 'a,
        H: 'a;

    map_iterator_boilerplate_for_slice!(table entries);
}

impl<K, V, H> Map<K, V> for HashMap<K, V, H>
where
    K: Eq,
    H: Hasher<K>,
{
    map_boilerplate_for_slice!(table entries);
}
