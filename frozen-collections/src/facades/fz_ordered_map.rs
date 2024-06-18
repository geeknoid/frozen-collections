use alloc::vec::Vec;
use core::borrow::Borrow;
use core::fmt::{Debug, Formatter, Result};
use core::ops::Index;
use core::ops::IndexMut;

use frozen_collections_core::maps::{
    BinarySearchMap, IntoIter, IntoKeys, IntoValues, Iter, IterMut, Keys, OrderedScanMap, Values,
    ValuesMut,
};
use frozen_collections_core::traits::{Len, Map, MapIterator};

/// The different implementations available for use, depending on the entries.
#[derive(Clone)]
enum MapTypes<K, V> {
    BinarySearch(BinarySearchMap<K, V>),
    Scanning(OrderedScanMap<K, V>),
}

/// A map optimized for fast read access.
///
/// A frozen map differs from the traditional [`HashMap`](std::collections::HashMap) type in three key ways. First, creating
/// a mew frozen map can take a relatively long time, especially for very large maps. Second,
/// once created, the keys in frozen maps are immutable. And third, probing a frozen map is
/// typically considerably faster, which is the whole point.
///
/// The reason creating a frozen map can take some time is due to the extensive analysis that is
/// performed on the map's keys in order to determine the best implementation strategy and data
/// layout to use. This analysis is what enables frozen maps to be faster later when
/// reading from the map.
///
/// Frozen maps are intended for long-lived maps, where the cost of creating the map is made up
/// over time by the faster read performance.
///
/// A `FrozenMap` requires that the elements
/// implement the [`Eq`] and [`Hash`] traits. This can frequently be achieved by
/// using `#[derive(PartialEq, Eq, Hash)]`. If you implement these yourself,
/// it is important that the following property holds:
///
/// ```text
/// k1 == k2 -> hash(k1) == hash(k2)
/// ```
///
/// In other words, if two keys are equal, their hashes must be equal.
/// Violating this property is a logic error.
///
/// It is also a logic error for a key to be modified in such a way that the key's
/// hash, as determined by the [`Hash`] trait, or its equality, as determined by
/// the [`Eq`] trait, changes while it is in the map. This is normally only
/// possible through [`core::cell::Cell`], [`core::cell::RefCell`], global state, I/O, or unsafe code.
///
/// The behavior resulting from either logic error is not specified, but will
/// be encapsulated to the `FrozenMap` that observed the logic error and not
/// result in undefined behavior. This could include panics, incorrect results,
/// aborts, memory leaks, and non-termination.
///
/// # Macros are Faster
///
/// If all your keys are known at compile time, you are much better off using the
/// `frozen_map!` macro rather than this type. This will result in considerably
/// better performance.
///
/// # Integer and String Keys
///
/// If you can't use the `frozen_map!` macro, but you know at compile time that your keys are integers or strings, you should use the
/// [`crate::FzSequenceMap`] and [`crate::FzStringMap`] types respectively for better performance.
///
/// # Examples
///
/// The easiest way to use `FrozenMap` with a custom key type is to derive [`Eq`] and [`Hash`].
/// We must also derive [`PartialEq`].
///
/// ```
/// # use frozen_collections::FzHashMap;
/// #
/// #[derive(Hash, Eq, PartialEq, Debug)]
/// struct Viking {
///     name: String,
///     country: String,
/// }
///
/// impl Viking {
///     /// Creates a new Viking.
///     fn new(name: &str, country: &str) -> Viking {
///         Viking { name: name.to_string(), country: country.to_string() }
///     }
/// }
///
/// // Use a FrozenMap to store the vikings' health points.
/// let vikings = FzHashMap::new(vec![
///     (Viking::new("Einar", "Norway"), 25),
///     (Viking::new("Olaf", "Denmark"), 24),
///     (Viking::new("Harald", "Iceland"), 12),
/// ]);
///
/// assert_eq!(Some(&24), vikings.get(&Viking::new("Olaf", "Denmark")));
/// assert_ne!(Some(&24), vikings.get(&Viking::new("Olaf", "Hawaii")));
///
/// // Prints the status of the vikings in a non-deterministic order.
/// for (viking, health) in &vikings {
///     println!("{viking:?} has {health} hp");
/// }
/// ```
#[derive(Clone)]
#[allow(clippy::module_name_repetitions)]
pub struct FzOrderedMap<K, V> {
    map_impl: MapTypes<K, V>,
}

impl<K, V> FzOrderedMap<K, V>
where
    K: Ord + Eq,
{
    /// Creates a frozen ordered map.
    ///
    /// # Examples
    ///
    /// ```
    /// # use frozen_collections::FzHashMap;
    /// #
    /// let map = FzHashMap::new(vec![(1, 2), (3, 4)]);
    /// ```
    #[must_use]
    pub fn new(entries: Vec<(K, V)>) -> Self {
        Self {
            map_impl: if entries.len() < 5 {
                MapTypes::Scanning(OrderedScanMap::new(entries))
            } else {
                MapTypes::BinarySearch(BinarySearchMap::new(entries))
            },
        }
    }
}

impl<K, V> FzOrderedMap<K, V> {
    /// Returns a reference to the value corresponding to the key.
    ///
    /// # Examples
    ///
    /// ```
    /// # use frozen_collections::FzHashMap;
    /// #
    /// let map = FzHashMap::new(vec![(1, "a".to_string())]);
    ///
    /// assert_eq!(map.get(&1), Some(&"a".to_string()));
    /// assert_eq!(map.get(&2), None);
    /// ```
    #[inline(always)]
    #[must_use]
    pub fn get<Q>(&self, key: &Q) -> Option<&V>
    where
        K: Borrow<Q>,
        Q: Ord + Eq,
    {
        match &self.map_impl {
            MapTypes::BinarySearch(m) => m.get(key),
            MapTypes::Scanning(m) => m.get(key),
        }
    }

    /// Returns the key-value pair corresponding to the supplied key.
    ///
    /// # Examples
    ///
    /// ```
    /// # use frozen_collections::FzHashMap;
    /// #
    /// let map = FzHashMap::new(vec![(1, "a".to_string())]);
    ///
    /// assert_eq!(map.get_key_value(&1), Some((&1, &"a".to_string())));
    /// assert_eq!(map.get_key_value(&2), None);
    /// ```
    #[inline]
    #[must_use]
    pub fn get_key_value<Q>(&self, key: &Q) -> Option<(&K, &V)>
    where
        K: Borrow<Q>,
        Q: Ord + Eq,
    {
        match &self.map_impl {
            MapTypes::BinarySearch(m) => m.get_key_value(key),
            MapTypes::Scanning(m) => m.get_key_value(key),
        }
    }

    /// Returns a mutable reference to the value corresponding to the key.
    ///
    /// # Examples
    ///
    /// ```
    /// # use frozen_collections::FzHashMap;
    /// #
    /// let mut map = FzHashMap::new(vec![(1, "a".to_string())]);
    ///
    /// assert_eq!(map.get_mut(&1), Some(&mut "a".to_string()));
    /// assert_eq!(map.get_mut(&2), None);
    /// ```
    #[inline]
    #[must_use]
    pub fn get_mut<Q>(&mut self, key: &Q) -> Option<&mut V>
    where
        K: Borrow<Q>,
        Q: Ord + Eq,
    {
        match &mut self.map_impl {
            MapTypes::BinarySearch(m) => m.get_mut(key),
            MapTypes::Scanning(m) => m.get_mut(key),
        }
    }

    /// Attempts to get mutable references to `N` values in the map at once.
    ///
    /// Returns an array of length `N` with the results of each query. For soundness, at most one
    /// mutable reference will be returned to any value. `None` will be returned if any of the
    /// keys are duplicates or missing.
    ///
    /// # Examples
    ///
    /// ```
    /// # use frozen_collections::FzHashMap;
    /// #
    /// let mut libraries = FzHashMap::new(vec![
    ///     ("Bodleian Library".to_string(), 1602),
    ///     ("Athenæum".to_string(), 1807),
    ///     ("Herzogin-Anna-Amalia-Bibliothek".to_string(), 1691),
    ///     ("Library of Congress".to_string(), 1800)
    /// ]);
    ///
    /// let got = libraries.get_many_mut([
    ///     &"Athenæum".to_string(),
    ///     &"Library of Congress".to_string(),
    /// ]);
    ///
    /// assert_eq!(
    ///     got,
    ///     Some([
    ///         &mut 1807,
    ///         &mut 1800,
    ///     ]),
    /// );
    ///
    /// // Missing keys result in None
    /// let got = libraries.get_many_mut([
    ///     &"Athenæum".to_string(),
    ///     &"New York Public Library".to_string(),
    /// ]);
    ///
    /// assert_eq!(got, None);
    ///
    /// // Duplicate keys result in None
    /// let got = libraries.get_many_mut([
    ///     &"Athenæum".to_string(),
    ///     &"Athenæum".to_string(),
    /// ]);
    ///
    /// assert_eq!(got, None);
    /// ```
    #[must_use]
    pub fn get_many_mut<const N: usize, Q>(&mut self, keys: [&Q; N]) -> Option<[&mut V; N]>
    where
        K: Borrow<Q>,
        Q: Ord + Eq,
    {
        match &mut self.map_impl {
            MapTypes::BinarySearch(m) => m.get_many_mut(keys),
            MapTypes::Scanning(m) => m.get_many_mut(keys),
        }
    }

    /// Returns `true` if the map contains a value for the specified key.
    ///
    /// # Examples
    ///
    /// ```
    /// # use frozen_collections::FzHashMap;
    /// #
    /// let map = FzHashMap::new(vec![(1, "a".to_string())]);
    ///
    /// assert_eq!(map.contains_key(&1), true);
    /// assert_eq!(map.contains_key(&2), false);
    /// ```
    #[inline]
    #[must_use]
    pub fn contains_key<Q>(&self, key: &Q) -> bool
    where
        K: Borrow<Q>,
        Q: Ord + Eq,
    {
        self.get(key).is_some()
    }
}

impl<K, V> Len for FzOrderedMap<K, V> {
    fn len(&self) -> usize {
        match &self.map_impl {
            MapTypes::BinarySearch(m) => m.len(),
            MapTypes::Scanning(m) => m.len(),
        }
    }
}

impl<K, V> From<Vec<(K, V)>> for FzOrderedMap<K, V>
where
    K: Ord + Eq,
{
    fn from(entries: Vec<(K, V)>) -> Self {
        Self::new(entries)
    }
}

impl<K, V, const N: usize> From<[(K, V); N]> for FzOrderedMap<K, V>
where
    K: Ord + Eq,
{
    fn from(entries: [(K, V); N]) -> Self {
        Self::new(Vec::from_iter(entries))
    }
}

impl<K, V> FromIterator<(K, V)> for FzOrderedMap<K, V>
where
    K: Ord + Eq,
{
    fn from_iter<T: IntoIterator<Item = (K, V)>>(iter: T) -> Self {
        Self::new(Vec::from_iter(iter))
    }
}

impl<K, V> Index<&K> for FzOrderedMap<K, V>
where
    K: Ord + Eq,
{
    type Output = V;

    fn index(&self, index: &K) -> &Self::Output {
        self.get(index).expect("index should be valid")
    }
}

impl<K, V> IndexMut<&K> for FzOrderedMap<K, V>
where
    K: Ord + Eq,
{
    fn index_mut(&mut self, index: &K) -> &mut V {
        self.get_mut(index).expect("index should be valid")
    }
}

impl<K, V> Default for FzOrderedMap<K, V> {
    fn default() -> Self {
        Self {
            map_impl: MapTypes::Scanning(OrderedScanMap::default()),
        }
    }
}

impl<K, V> Debug for FzOrderedMap<K, V>
where
    K: Debug,
    V: Debug,
{
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        match &self.map_impl {
            MapTypes::BinarySearch(m) => m.fmt(f),
            MapTypes::Scanning(m) => m.fmt(f),
        }
    }
}

impl<K, V, MT> PartialEq<MT> for FzOrderedMap<K, V>
where
    K: Ord + Eq,
    V: PartialEq,
    MT: Map<K, V>,
{
    fn eq(&self, other: &MT) -> bool {
        if self.len() != other.len() {
            return false;
        }

        self.iter()
            .all(|(key, value)| other.get(key).map_or(false, |v| *value == *v))
    }
}

impl<K, V> Eq for FzOrderedMap<K, V>
where
    K: Ord + Eq,
    V: Eq,
{
}

impl<'a, K, V> IntoIterator for &'a FzOrderedMap<K, V> {
    type Item = (&'a K, &'a V);
    type IntoIter = Iter<'a, K, V>;

    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}

impl<'a, K, V> IntoIterator for &'a mut FzOrderedMap<K, V> {
    type Item = (&'a K, &'a mut V);
    type IntoIter = IterMut<'a, K, V>;

    fn into_iter(self) -> Self::IntoIter {
        self.iter_mut()
    }
}

impl<K, V> IntoIterator for FzOrderedMap<K, V> {
    type Item = (K, V);
    type IntoIter = IntoIter<K, V>;

    fn into_iter(self) -> Self::IntoIter {
        match self.map_impl {
            MapTypes::BinarySearch(m) => m.into_iter(),
            MapTypes::Scanning(m) => m.into_iter(),
        }
    }
}

impl<K, V> MapIterator<K, V> for FzOrderedMap<K, V> {
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

    fn iter(&self) -> Self::Iterator<'_> {
        match &self.map_impl {
            MapTypes::BinarySearch(m) => m.iter(),
            MapTypes::Scanning(m) => m.iter(),
        }
    }

    fn keys(&self) -> Self::KeyIterator<'_> {
        match &self.map_impl {
            MapTypes::BinarySearch(m) => m.keys(),
            MapTypes::Scanning(m) => m.keys(),
        }
    }

    fn values(&self) -> Self::ValueIterator<'_> {
        match &self.map_impl {
            MapTypes::BinarySearch(m) => m.values(),
            MapTypes::Scanning(m) => m.values(),
        }
    }

    fn into_keys(self) -> Self::IntoKeyIterator {
        match self.map_impl {
            MapTypes::BinarySearch(m) => m.into_keys(),
            MapTypes::Scanning(m) => m.into_keys(),
        }
    }

    fn into_values(self) -> Self::IntoValueIterator {
        match self.map_impl {
            MapTypes::BinarySearch(m) => m.into_values(),
            MapTypes::Scanning(m) => m.into_values(),
        }
    }

    fn iter_mut(&mut self) -> Self::MutIterator<'_> {
        match &mut self.map_impl {
            MapTypes::BinarySearch(m) => m.iter_mut(),
            MapTypes::Scanning(m) => m.iter_mut(),
        }
    }

    fn values_mut(&mut self) -> Self::ValueMutIterator<'_> {
        match &mut self.map_impl {
            MapTypes::BinarySearch(m) => m.values_mut(),
            MapTypes::Scanning(m) => m.values_mut(),
        }
    }
}

impl<K, V> Map<K, V> for FzOrderedMap<K, V>
where
    K: Ord + Eq,
{
    #[inline]
    fn contains_key(&self, key: &K) -> bool {
        self.contains_key(key)
    }

    #[inline]
    fn get(&self, key: &K) -> Option<&V> {
        self.get(key)
    }

    #[inline]
    fn get_key_value(&self, key: &K) -> Option<(&K, &V)> {
        self.get_key_value(key)
    }

    #[inline]
    fn get_mut(&mut self, key: &K) -> Option<&mut V> {
        self.get_mut(key)
    }

    #[inline]
    fn get_many_mut<const N: usize>(&mut self, keys: [&K; N]) -> Option<[&mut V; N]> {
        self.get_many_mut(keys)
    }
}

#[cfg(test)]
mod tests {
    use hashbrown::HashMap as HashbrownMap;

    use super::*;

    #[test]
    fn test_empty_map() {
        type FM = FzOrderedMap<i32, i32>;

        let m = FM::default();
        assert_eq!(m.len(), 0);
    }

    #[test]
    fn test_i32_map() {
        let m = FzOrderedMap::<i32, i32>::from([(1, 1), (2, 2), (3, 3), (4, 4), (5, 5), (6, 6)]);
        assert_eq!(m.get(&6), Some(&6));
    }

    #[test]
    fn basic_u32_map() {
        let max_entries = [1, 2, 3, 4, 5, 6, 255, 256, 65535, 65536];

        for max in max_entries {
            let mut m = HashbrownMap::<u32, String>::new();
            for i in 0..max {
                m.insert(i, format!("V{i}"));
            }

            let fm = m
                .iter()
                .map(|x| (*x.0, x.1.clone()))
                .collect::<FzOrderedMap<_, _>>();
            assert_eq!(m.len(), fm.len());
            assert_eq!(m.is_empty(), fm.is_empty());

            for pair in &m {
                assert!(fm.contains_key(pair.0));
                assert_eq!(m.get(pair.0).unwrap(), fm.get(pair.0).unwrap());
                assert_eq!(
                    m.get_key_value(pair.0).unwrap(),
                    fm.get_key_value(pair.0).unwrap()
                );
            }

            let mut m = HashbrownMap::<u32, String>::new();
            for i in (0..max).map(|x| x * 2) {
                m.insert(i, "V{i}".to_string());
            }

            let fd = m
                .iter()
                .map(|x| (*x.0, x.1.clone()))
                .collect::<FzOrderedMap<_, _>>();
            assert_eq!(m.len(), fd.len());
            assert_eq!(m.is_empty(), fd.is_empty());

            for pair in &m {
                assert!(fd.contains_key(pair.0));
                assert_eq!(m.get(pair.0).unwrap(), fd.get(pair.0).unwrap());
                assert_eq!(
                    m.get_key_value(pair.0).unwrap(),
                    fd.get_key_value(pair.0).unwrap()
                );
            }
        }
    }
}
