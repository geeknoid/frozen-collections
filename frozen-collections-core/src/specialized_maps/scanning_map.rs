use std::borrow::Borrow;
use std::fmt::{Debug, Formatter, Result};
use std::hash::Hash;
use std::ops::{Index, IndexMut};

use crate::analyzers::check_duplicate_keys;
use crate::specialized_maps::utils::{any_duplicate_keys, get_many_mut, partial_eq};
use crate::specialized_maps::{
    IntoIter, IntoKeys, IntoValues, Iter, IterMut, Keys, Values, ValuesMut,
};
use crate::traits::{Len, Map};

/// A general purpose map that uses linear scan of entries rather than a hash table.
///
/// # Important Note
///
/// This type is not intended to be used directly by
/// application code. Instead, applications are expected
/// to use the `FrozenMap` type or the `frozen_map!` macro.
#[derive(Clone)]
pub struct ScanningMap<K, V> {
    pub(crate) entries: Box<[(K, V)]>,
}

impl<K, V> ScanningMap<K, V>
where
    K: Hash + Eq,
{
    #[allow(clippy::missing_errors_doc)]
    pub fn new(payload: Vec<(K, V)>) -> std::result::Result<Self, &'static str> {
        check_duplicate_keys(payload.iter().map(|entry| &entry.0))?;

        Ok(Self {
            entries: payload.into_boxed_slice(),
        })
    }
}

impl<K, V> ScanningMap<K, V> {
    #[inline]
    #[must_use]
    pub fn get<Q>(&self, key: &Q) -> Option<&V>
    where
        K: Borrow<Q>,
        Q: ?Sized + Eq,
    {
        for entry in self.entries.iter() {
            if key.eq(entry.0.borrow()) {
                return Some(&entry.1);
            }
        }

        None
    }

    #[inline]
    #[must_use]
    pub fn get_mut<Q>(&mut self, key: &Q) -> Option<&mut V>
    where
        K: Borrow<Q>,
        Q: ?Sized + Eq,
    {
        for entry in self.entries.iter_mut() {
            if key.eq(entry.0.borrow()) {
                return Some(&mut entry.1);
            }
        }

        None
    }

    #[inline]
    #[must_use]
    pub fn get_key_value<Q>(&self, key: &Q) -> Option<(&K, &V)>
    where
        K: Borrow<Q>,
        Q: ?Sized + Eq,
    {
        for entry in self.entries.iter() {
            if key.eq(entry.0.borrow()) {
                return Some((&entry.0, &entry.1));
            }
        }

        None
    }

    #[must_use]
    pub fn get_many_mut<Q, const N: usize>(&mut self, keys: [&Q; N]) -> Option<[&mut V; N]>
    where
        K: Borrow<Q>,
        Q: ?Sized + Eq,
    {
        get_many_mut!(self, keys);
    }

    #[inline]
    #[must_use]
    pub fn contains_key<Q>(&self, key: &Q) -> bool
    where
        K: Borrow<Q>,
        Q: ?Sized + Eq,
    {
        self.get(key).is_some()
    }
}

impl<K, V> Len for ScanningMap<K, V> {
    fn len(&self) -> usize {
        self.entries.len()
    }
}

impl<K, V> Debug for ScanningMap<K, V>
where
    K: Debug,
    V: Debug,
{
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        let pairs = self.entries.iter().map(|x| (&x.0, &x.1));
        f.debug_map().entries(pairs).finish()
    }
}

impl<K, V> Default for ScanningMap<K, V> {
    fn default() -> Self {
        Self {
            entries: Box::default(),
        }
    }
}

impl<Q, K, V> Index<&Q> for ScanningMap<K, V>
where
    K: Borrow<Q>,
    Q: ?Sized + Eq,
{
    type Output = V;

    fn index(&self, index: &Q) -> &Self::Output {
        self.get(index).unwrap()
    }
}

impl<Q, K, V> IndexMut<&Q> for ScanningMap<K, V>
where
    K: Borrow<Q>,
    Q: ?Sized + Eq,
{
    fn index_mut(&mut self, index: &Q) -> &mut V {
        self.get_mut(index).unwrap()
    }
}

impl<'a, K, V> IntoIterator for &'a ScanningMap<K, V>
where
    K: Eq,
{
    type Item = (&'a K, &'a V);
    type IntoIter = Iter<'a, K, V>;

    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}

impl<K, V> IntoIterator for ScanningMap<K, V> {
    type Item = (K, V);
    type IntoIter = IntoIter<K, V>;

    fn into_iter(self) -> Self::IntoIter {
        IntoIter::new(self.entries)
    }
}

impl<'a, K, V> IntoIterator for &'a mut ScanningMap<K, V>
where
    K: Eq,
{
    type Item = (&'a K, &'a mut V);
    type IntoIter = IterMut<'a, K, V>;

    fn into_iter(self) -> Self::IntoIter {
        self.iter_mut()
    }
}

impl<K, V, MT> PartialEq<MT> for ScanningMap<K, V>
where
    K: Eq,
    V: PartialEq,
    MT: Map<K, V>,
{
    partial_eq!();
}

impl<K, V> Eq for ScanningMap<K, V>
where
    K: Eq,
    V: Eq,
{
}

impl<K, V> TryFrom<Vec<(K, V)>> for ScanningMap<K, V>
where
    K: Hash + Eq,
{
    type Error = &'static str;

    fn try_from(payload: Vec<(K, V)>) -> std::result::Result<Self, Self::Error> {
        Self::new(payload)
    }
}

impl<K, V, const N: usize> TryFrom<[(K, V); N]> for ScanningMap<K, V>
where
    K: Hash + Eq,
{
    type Error = &'static str;

    fn try_from(payload: [(K, V); N]) -> std::result::Result<Self, Self::Error> {
        Self::new(Vec::from_iter(payload))
    }
}

impl<K, V> FromIterator<(K, V)> for ScanningMap<K, V>
where
    K: Hash + Eq,
{
    fn from_iter<T: IntoIterator<Item = (K, V)>>(iter: T) -> Self {
        Self::new(Vec::from_iter(iter)).unwrap()
    }
}

impl<K, V> Map<K, V> for ScanningMap<K, V>
where
    K: Eq,
{
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

    type IntoKeyIterator = IntoKeys<K, V>;
    type IntoValueIterator = IntoValues<K, V>;

    type MutIterator<'a> = IterMut<'a, K, V>
    where
        K: 'a,
        V: 'a;

    type ValueMutIterator<'a> = ValuesMut<'a, K, V>
    where
        K: 'a,
        V: 'a;

    #[inline]
    fn iter(&self) -> Self::Iterator<'_> {
        Iter::new(&self.entries)
    }

    #[inline]
    fn keys(&self) -> Self::KeyIterator<'_> {
        Keys::new(&self.entries)
    }

    #[inline]
    fn values(&self) -> Self::ValueIterator<'_> {
        Values::new(&self.entries)
    }

    #[inline]
    fn into_keys(self) -> Self::IntoKeyIterator {
        IntoKeys::new(self.entries)
    }

    #[inline]
    fn into_values(self) -> Self::IntoValueIterator {
        IntoValues::new(self.entries)
    }

    #[inline]
    fn iter_mut(&mut self) -> Self::MutIterator<'_> {
        IterMut::new(self.entries.as_mut())
    }

    #[inline]
    fn values_mut(&mut self) -> Self::ValueMutIterator<'_> {
        ValuesMut::new(self.entries.as_mut())
    }

    #[inline]
    fn contains_key(&self, key: &K) -> bool {
        self.contains_key(key)
    }

    #[inline]
    fn get(&self, key: &K) -> Option<&V> {
        Self::get(self, key)
    }
}

#[cfg(test)]
mod tests {
    use crate::traits::{Len, Map};

    use super::ScanningMap;

    #[test]
    fn new_creates_scanning_map_with_given_payload() {
        let payload = vec![(10, 20), (30, 40), (50, 60)];
        let map = ScanningMap::<i32, i32>::try_from(payload.clone()).unwrap();
        assert_eq!(payload.len(), map.len());
    }

    #[test]
    fn get_returns_some_for_existing_keys() {
        let payload = vec![(10, 20), (30, 40), (50, 60)];
        let map = ScanningMap::<i32, i32>::try_from(payload).unwrap();
        assert_eq!(&20, map.get(&10).unwrap());
        assert_eq!(&40, map.get(&30).unwrap());
        assert_eq!(&60, map.get(&50).unwrap());
    }

    #[test]
    fn get_returns_none_for_non_existing_keys() {
        let payload = vec![(10, 20), (30, 40), (50, 60)];
        let map = ScanningMap::<i32, i32>::try_from(payload).unwrap();
        assert_eq!(None, map.get(&0));
    }

    #[test]
    fn get_mut_returns_some_for_existing_keys() {
        let payload = vec![(10, 20), (30, 40), (50, 60)];
        let mut map = ScanningMap::<i32, i32>::try_from(payload).unwrap();
        assert_eq!(&20, map.get_mut(&10).unwrap());
        assert_eq!(&40, map.get_mut(&30).unwrap());
        assert_eq!(&60, map.get_mut(&50).unwrap());
    }

    #[test]
    fn get_mut_returns_none_for_non_existing_keys() {
        let payload = vec![(10, 20), (30, 40), (50, 60)];
        let mut map = ScanningMap::<i32, i32>::try_from(payload).unwrap();
        assert_eq!(None, map.get_mut(&0));
    }

    #[test]
    fn get_key_value_returns_some_for_existing_keys() {
        let payload = vec![(10, 20), (30, 40), (50, 60)];
        let map = ScanningMap::<i32, i32>::try_from(payload).unwrap();
        assert_eq!((&10, &20), map.get_key_value(&10).unwrap());
        assert_eq!((&30, &40), map.get_key_value(&30).unwrap());
        assert_eq!((&50, &60), map.get_key_value(&50).unwrap());
    }

    #[test]
    fn get_key_value_returns_none_for_non_existing_keys() {
        let payload = vec![(10, 20), (30, 40), (50, 60)];
        let map = ScanningMap::<i32, i32>::try_from(payload).unwrap();
        assert_eq!(None, map.get_key_value(&0));
    }

    #[test]
    fn debug_format_is_correct() {
        let payload = vec![(10, 20)];
        let map = ScanningMap::<i32, i32>::try_from(payload).unwrap();
        assert_eq!("{10: 20}", format!("{map:?}"));
    }

    #[test]
    fn get_many_mut_success() {
        let mut map = ScanningMap::from_iter(vec![(1, "a"), (2, "b"), (3, "c")]);
        if let Some([a, b]) = map.get_many_mut([&1, &2]) {
            *a = "alpha";
            *b = "beta";
        }
        assert_eq!(map.get(&1), Some(&"alpha"));
        assert_eq!(map.get(&2), Some(&"beta"));
    }

    #[test]
    fn get_many_mut_with_duplicate_keys() {
        let mut map = ScanningMap::from_iter(vec![(1, "a"), (2, "b")]);
        assert_eq!(map.get_many_mut([&1, &1]), None);
    }

    #[test]
    fn get_many_mut_with_non_existing_keys() {
        let mut map = ScanningMap::from_iter(vec![(1, "a"), (2, "b")]);
        assert_eq!(map.get_many_mut([&3, &4]), None);
    }

    #[test]
    fn test_creation() {
        let map = ScanningMap::new(vec![(1, "a"), (2, "b")]).unwrap();
        assert_eq!(map.len(), 2);
    }

    #[test]
    fn test_get() {
        let map = ScanningMap::new(vec![(1, "a"), (2, "b")]).unwrap();
        assert_eq!(map.get(&1), Some(&"a"));
        assert_eq!(map.get(&3), None);
    }

    #[test]
    fn test_get_mut() {
        let mut map = ScanningMap::new(vec![(1, "a"), (2, "b")]).unwrap();
        if let Some(value) = map.get_mut(&1) {
            *value = "alpha";
        }
        assert_eq!(map.get(&1), Some(&"alpha"));
        assert_eq!(map.get_mut(&3), None);
    }

    #[test]
    fn test_get_key_value() {
        let map = ScanningMap::new(vec![(1, "a"), (2, "b")]).unwrap();
        assert_eq!(map.get_key_value(&1), Some((&1, &"a")));
        assert_eq!(map.get_key_value(&3), None);
    }

    #[test]
    fn test_contains_key() {
        let map = ScanningMap::new(vec![(1, "a"), (2, "b")]).unwrap();
        assert!(map.contains_key(&1));
        assert!(!map.contains_key(&3));
    }

    #[test]
    fn test_iterators() {
        let map = ScanningMap::new(vec![(1, "a"), (2, "b")]).unwrap();
        assert_eq!(map.iter().count(), 2);
        assert_eq!(map.keys().collect::<Vec<_>>(), vec![&1, &2]);
        assert_eq!(map.values().collect::<Vec<_>>(), vec![&"a", &"b"]);
    }

    #[test]
    fn test_debug() {
        let map = ScanningMap::new(vec![(1, "a")]).unwrap();
        assert_eq!(format!("{map:?}"), "{1: \"a\"}");
    }

    #[test]
    fn test_get_many_mut() {
        let mut map = ScanningMap::new(vec![(1, "a"), (2, "b"), (3, "c")]).unwrap();
        if let Some([a, _]) = map.get_many_mut([&1, &2]) {
            *a = "alpha";
        }
        assert_eq!(map.get(&1), Some(&"alpha"));
        assert_eq!(map.get_many_mut([&4, &5]), None);
        assert_eq!(map.get_many_mut([&1, &1]), None);
    }

    #[test]
    fn test_into_keys() {
        let map = ScanningMap::from_iter(vec![(1, "a"), (2, "b")]);
        let keys: Vec<_> = map.into_keys().collect();
        assert_eq!(keys, vec![1, 2]);
    }

    #[test]
    fn test_into_values() {
        let map = ScanningMap::from_iter(vec![(1, "a"), (2, "b")]);
        let values: Vec<_> = map.into_values().collect();
        assert_eq!(values, vec!["a", "b"]);
    }

    #[test]
    fn test_iter_mut() {
        let mut map = ScanningMap::from_iter(vec![(1, "a"), (2, "b")]);
        map.iter_mut().for_each(|(_k, v)| *v = "modified");
        assert_eq!(map.get(&1), Some(&"modified"));
        assert_eq!(map.get(&2), Some(&"modified"));
    }

    #[test]
    fn test_values_mut() {
        let mut map = ScanningMap::from_iter(vec![(1, "a"), (2, "b")]);
        map.values_mut().for_each(|v| *v = "changed");
        assert_eq!(map.get(&1), Some(&"changed"));
        assert_eq!(map.get(&2), Some(&"changed"));
    }

    #[test]
    fn test_equality_of_identical_maps() {
        let map1 = ScanningMap::from_iter(vec![(1, "a"), (2, "b")]);
        let map2 = ScanningMap::from_iter(vec![(1, "a"), (2, "b")]);
        assert_eq!(map1, map2);
    }

    #[test]
    fn test_inequality_of_different_maps() {
        let map1 = ScanningMap::from_iter(vec![(1, "a"), (2, "b")]);
        let map2 = ScanningMap::from_iter(vec![(3, "c"), (4, "d")]);
        assert_ne!(map1, map2);
    }

    #[test]
    fn test_equality_with_self() {
        let map = ScanningMap::from_iter(vec![(1, "a"), (2, "b")]);
        assert_eq!(map, map);
    }

    #[test]
    fn test_inequality_with_different_lengths() {
        let map1 = ScanningMap::from_iter(vec![(1, "a"), (2, "b")]);
        let map2 = ScanningMap::from_iter(vec![(1, "a")]);
        assert_ne!(map1, map2);
    }

    #[test]
    fn default_creates_empty_map() {
        let map: ScanningMap<i32, i32> = ScanningMap::default();
        assert!(map.entries.is_empty());
    }

    #[test]
    #[should_panic(expected = "called `Option::unwrap()` on a `None` value")]
    fn index_retrieves_correct_value_or_panics() {
        let map = ScanningMap::from_iter(vec![(1, "a"), (2, "b")]);
        assert_eq!("a", map[&1]);
        // This line should panic because key 3 does not exist.
        let _ = map[&3];
    }

    #[test]
    fn index_mut_updates_value_correctly() {
        let mut map = ScanningMap::from_iter(vec![(1, "a")]);
        map[&1] = "alpha";
        assert_eq!(&"alpha", map.get(&1).unwrap());
    }

    #[test]
    fn into_iterator_yields_correct_pairs() {
        let map = ScanningMap::from_iter(vec![(1, "a"), (2, "b")]);
        let mut iter: Vec<_> = (&map).into_iter().collect();
        iter.sort_by_key(|&(k, _)| *k); // Ensure the order for comparison
        assert_eq!(vec![(&1, &"a"), (&2, &"b")], iter);
    }
}
