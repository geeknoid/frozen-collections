use std::borrow::Borrow;
use std::fmt::{Debug, Formatter, Result};
use std::hash::{BuildHasher, Hash};
use std::ops::Index;
use std::ops::IndexMut;

use ahash::RandomState;
use bitvec::macros::internal::funty::Fundamental;

use frozen_collections_core::traits::Map;

use crate::specialized_maps::*;
use crate::Len;

/// The different implementations available for use, depending on the payload.
#[derive(Clone)]
enum MapTypes<K, V, BH> {
    CommonSmall(CommonMap<K, V, u8, BH>),
    CommonLarge(CommonMap<K, V, usize, BH>),
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
/// possible through [`std::cell::Cell`], [`std::cell::RefCell`], global state, I/O, or unsafe code.
///
/// The behavior resulting from either logic error is not specified, but will
/// be encapsulated to the `FrozenMap` that observed the logic error and not
/// result in undefined behavior. This could include panics, incorrect results,
/// aborts, memory leaks, and non-termination.
///
/// # Macros are Faster
///
/// If all your keys are known at compile time, you are much better off using the
/// [`frozen_map!`](crate::frozen_map!) macro rather than this type. This will result in considerably
/// better performance.
///
/// # Integer and String Keys
///
/// If you can't use the [`frozen_map!`](crate::frozen_map!) macro, but you know at compile time that your keys are integers or strings, you should use the
/// [`crate::FrozenIntMap`] and [`crate::FrozenStringMap`] types respectively for better performance.
///
/// # Examples
///
/// The easiest way to use `FrozenMap` with a custom key type is to derive [`Eq`] and [`Hash`].
/// We must also derive [`PartialEq`].
///
/// ```
/// # use frozen_collections::FrozenMap;
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
/// let vikings = FrozenMap::new(vec![
///     (Viking::new("Einar", "Norway"), 25),
///     (Viking::new("Olaf", "Denmark"), 24),
///     (Viking::new("Harald", "Iceland"), 12),
/// ]).unwrap();
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
pub struct FrozenMap<K, V, BH = RandomState> {
    map_impl: MapTypes<K, V, BH>,
}

impl<K, V> FrozenMap<K, V, RandomState>
where
    K: Hash + Eq,
{
    /// Creates a frozen map which will use the [`RandomState`] hasher builder to hash keys.
    ///
    /// # Errors
    ///
    /// This fails if there are duplicate keys within the vector.
    ///
    /// # Examples
    ///
    /// ```
    /// # use frozen_collections::FrozenMap;
    /// # use std::hash::RandomState;
    /// #
    /// let map = FrozenMap::new(vec![(1, 2), (3, 4)]).unwrap();
    /// ```
    pub fn new(payload: Vec<(K, V)>) -> std::result::Result<Self, &'static str> {
        Self::with_hasher(payload, RandomState::new())
    }
}

impl<K, V, BH> FrozenMap<K, V, BH>
where
    K: Hash + Eq,
    BH: BuildHasher,
{
    /// Creates a frozen map which will use the given hash builder to hash keys.
    ///
    /// # Errors
    ///
    /// This fails if there are duplicate keys within the vector.
    ///
    /// # Examples
    ///
    /// ```
    /// # use frozen_collections::FrozenMap;
    /// # use std::hash::RandomState;
    /// #
    /// let map = FrozenMap::with_hasher(vec![(1, 2), (3, 4)], RandomState::new()).unwrap();
    /// ```
    pub fn with_hasher(payload: Vec<(K, V)>, bh: BH) -> std::result::Result<Self, &'static str> {
        Ok(Self {
            map_impl: if payload.len() <= u8::MAX.as_usize() {
                MapTypes::CommonSmall(CommonMap::with_hasher(payload, bh)?)
            } else {
                MapTypes::CommonLarge(CommonMap::with_hasher(payload, bh)?)
            },
        })
    }
}

impl<K, V, BH> FrozenMap<K, V, BH>
where
    BH: BuildHasher,
{
    /// Returns a reference to the value corresponding to the key.
    ///
    /// # Examples
    ///
    /// ```
    /// # use frozen_collections::FrozenMap;
    /// #
    /// let map = FrozenMap::new(vec![(1, "a".to_string())]).unwrap();
    ///
    /// assert_eq!(map.get(&1), Some(&"a".to_string()));
    /// assert_eq!(map.get(&2), None);
    /// ```
    #[inline(always)]
    #[must_use]
    pub fn get<Q>(&self, key: &Q) -> Option<&V>
    where
        K: Borrow<Q>,
        Q: Hash + Eq,
    {
        match &self.map_impl {
            MapTypes::CommonSmall(m) => m.get(key),
            MapTypes::CommonLarge(m) => m.get(key),
        }
    }

    /// Returns the key-value pair corresponding to the supplied key.
    ///
    /// # Examples
    ///
    /// ```
    /// # use frozen_collections::FrozenMap;
    /// #
    /// let map = FrozenMap::new(vec![(1, "a".to_string())]).unwrap();
    ///
    /// assert_eq!(map.get_key_value(&1), Some((&1, &"a".to_string())));
    /// assert_eq!(map.get_key_value(&2), None);
    /// ```
    #[inline]
    #[must_use]
    pub fn get_key_value<Q>(&self, key: &Q) -> Option<(&K, &V)>
    where
        K: Borrow<Q>,
        Q: Hash + Eq,
    {
        match &self.map_impl {
            MapTypes::CommonSmall(m) => m.get_key_value(key),
            MapTypes::CommonLarge(m) => m.get_key_value(key),
        }
    }

    /// Returns a mutable reference to the value corresponding to the key.
    ///
    /// # Examples
    ///
    /// ```
    /// # use frozen_collections::FrozenMap;
    /// #
    /// let mut map = FrozenMap::new(vec![(1, "a".to_string())]).unwrap();
    ///
    /// assert_eq!(map.get_mut(&1), Some(&mut "a".to_string()));
    /// assert_eq!(map.get_mut(&2), None);
    /// ```
    #[inline]
    #[must_use]
    pub fn get_mut<Q>(&mut self, key: &Q) -> Option<&mut V>
    where
        K: Borrow<Q>,
        Q: Hash + Eq,
    {
        match &mut self.map_impl {
            MapTypes::CommonSmall(m) => m.get_mut(key),
            MapTypes::CommonLarge(m) => m.get_mut(key),
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
    /// # use frozen_collections::FrozenMap;
    /// #
    /// let mut libraries = FrozenMap::new(vec![
    ///     ("Bodleian Library".to_string(), 1602),
    ///     ("Athenæum".to_string(), 1807),
    ///     ("Herzogin-Anna-Amalia-Bibliothek".to_string(), 1691),
    ///     ("Library of Congress".to_string(), 1800)
    /// ]).unwrap();
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
        Q: Hash + Eq,
    {
        match &mut self.map_impl {
            MapTypes::CommonSmall(m) => m.get_many_mut(keys),
            MapTypes::CommonLarge(m) => m.get_many_mut(keys),
        }
    }

    /// Returns `true` if the map contains a value for the specified key.
    ///
    /// # Examples
    ///
    /// ```
    /// # use frozen_collections::FrozenMap;
    /// #
    /// let map = FrozenMap::new(vec![(1, "a".to_string())]).unwrap();
    ///
    /// assert_eq!(map.contains_key(&1), true);
    /// assert_eq!(map.contains_key(&2), false);
    /// ```
    #[inline]
    #[must_use]
    pub fn contains_key<Q>(&self, key: &Q) -> bool
    where
        K: Borrow<Q>,
        Q: Hash + Eq,
    {
        self.get(key).is_some()
    }
}

impl<K, V, BH> Len for FrozenMap<K, V, BH> {
    fn len(&self) -> usize {
        match &self.map_impl {
            MapTypes::CommonSmall(m) => m.len(),
            MapTypes::CommonLarge(m) => m.len(),
        }
    }
}

impl<K, V, BH> TryFrom<Vec<(K, V)>> for FrozenMap<K, V, BH>
where
    K: Hash + Eq,
    BH: BuildHasher + Default,
{
    type Error = &'static str;

    fn try_from(payload: Vec<(K, V)>) -> std::result::Result<Self, Self::Error> {
        Self::with_hasher(payload, BH::default())
    }
}

impl<K, V, const N: usize, BH> TryFrom<[(K, V); N]> for FrozenMap<K, V, BH>
where
    K: Hash + Eq,
    BH: BuildHasher + Default,
{
    type Error = &'static str;

    fn try_from(payload: [(K, V); N]) -> std::result::Result<Self, Self::Error> {
        Self::try_from(Vec::from_iter(payload))
    }
}

impl<K, V, BH> FromIterator<(K, V)> for FrozenMap<K, V, BH>
where
    K: Hash + Eq,
    BH: BuildHasher + Default,
{
    fn from_iter<T: IntoIterator<Item = (K, V)>>(iter: T) -> Self {
        Self::try_from(Vec::from_iter(iter)).unwrap()
    }
}

impl<K, V, BH> Index<&K> for FrozenMap<K, V, BH>
where
    K: Hash + Eq,
    BH: BuildHasher,
{
    type Output = V;

    fn index(&self, index: &K) -> &Self::Output {
        self.get(index).unwrap()
    }
}

impl<K, V, BH> IndexMut<&K> for FrozenMap<K, V, BH>
where
    K: Hash + Eq,
    BH: BuildHasher,
{
    fn index_mut(&mut self, index: &K) -> &mut V {
        self.get_mut(index).unwrap()
    }
}

impl<K, V, BH> Default for FrozenMap<K, V, BH>
where
    K: Hash + Eq + Default,
    V: Default,
    BH: BuildHasher + Default,
{
    fn default() -> Self {
        Self {
            map_impl: MapTypes::CommonSmall(CommonMap::default()),
        }
    }
}

impl<K, V, BH> Debug for FrozenMap<K, V, BH>
where
    K: Debug,
    V: Debug,
{
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        match &self.map_impl {
            MapTypes::CommonSmall(m) => m.fmt(f),
            MapTypes::CommonLarge(m) => m.fmt(f),
        }
    }
}

impl<K, V, MT, BH> PartialEq<MT> for FrozenMap<K, V, BH>
where
    K: Hash + Eq,
    V: PartialEq,
    MT: Map<K, V>,
    BH: BuildHasher,
{
    fn eq(&self, other: &MT) -> bool {
        if self.len() != other.len() {
            return false;
        }

        self.iter()
            .all(|(key, value)| other.get(key).map_or(false, |v| *value == *v))
    }
}

impl<K, V, BH> Eq for FrozenMap<K, V, BH>
where
    K: Hash + Eq,
    V: Eq,
    BH: BuildHasher,
{
}

impl<'a, K, V, BH> IntoIterator for &'a FrozenMap<K, V, BH>
where
    K: Hash + Eq,
    BH: BuildHasher,
{
    type Item = (&'a K, &'a V);
    type IntoIter = Iter<'a, K, V>;

    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}

impl<'a, K, V, BH> IntoIterator for &'a mut FrozenMap<K, V, BH>
where
    K: Hash + Eq,
    BH: BuildHasher,
{
    type Item = (&'a K, &'a mut V);
    type IntoIter = IterMut<'a, K, V>;

    fn into_iter(self) -> Self::IntoIter {
        self.iter_mut()
    }
}

impl<K, V, BH> IntoIterator for FrozenMap<K, V, BH>
where
    K: Hash + Eq,
    BH: BuildHasher,
{
    type Item = (K, V);
    type IntoIter = IntoIter<K, V>;

    fn into_iter(self) -> Self::IntoIter {
        match self.map_impl {
            MapTypes::CommonSmall(m) => m.into_iter(),
            MapTypes::CommonLarge(m) => m.into_iter(),
        }
    }
}

impl<K, V, BH> Map<K, V> for FrozenMap<K, V, BH>
where
    K: Hash + Eq,
    BH: BuildHasher,
{
    type Iterator<'a> = Iter<'a, K, V>
    where
        K: 'a,
        V: 'a,
        BH: 'a;

    type KeyIterator<'a> = Keys<'a, K, V>
    where
        K: 'a,
        V: 'a,
        BH: 'a;

    type ValueIterator<'a> = Values<'a, K, V>
    where
        K: 'a,
        V: 'a,
        BH: 'a;

    type IntoKeyIterator = IntoKeys<K, V>;
    type IntoValueIterator = IntoValues<K, V>;

    type MutIterator<'a> = IterMut<'a, K, V>
    where
        K: 'a,
        V: 'a,
        BH: 'a;

    type ValueMutIterator<'a> = ValuesMut<'a, K, V>
    where
        K: 'a,
        V: 'a,
        BH: 'a;

    fn iter(&self) -> Self::Iterator<'_> {
        match &self.map_impl {
            MapTypes::CommonSmall(m) => m.iter(),
            MapTypes::CommonLarge(m) => m.iter(),
        }
    }

    fn keys(&self) -> Self::KeyIterator<'_> {
        match &self.map_impl {
            MapTypes::CommonSmall(m) => m.keys(),
            MapTypes::CommonLarge(m) => m.keys(),
        }
    }

    fn values(&self) -> Self::ValueIterator<'_> {
        match &self.map_impl {
            MapTypes::CommonSmall(m) => m.values(),
            MapTypes::CommonLarge(m) => m.values(),
        }
    }

    fn into_keys(self) -> Self::IntoKeyIterator {
        match self.map_impl {
            MapTypes::CommonSmall(m) => m.into_keys(),
            MapTypes::CommonLarge(m) => m.into_keys(),
        }
    }

    fn into_values(self) -> Self::IntoValueIterator {
        match self.map_impl {
            MapTypes::CommonSmall(m) => m.into_values(),
            MapTypes::CommonLarge(m) => m.into_values(),
        }
    }

    fn iter_mut(&mut self) -> Self::MutIterator<'_> {
        match &mut self.map_impl {
            MapTypes::CommonSmall(m) => m.iter_mut(),
            MapTypes::CommonLarge(m) => m.iter_mut(),
        }
    }

    fn values_mut(&mut self) -> Self::ValueMutIterator<'_> {
        match &mut self.map_impl {
            MapTypes::CommonSmall(m) => m.values_mut(),
            MapTypes::CommonLarge(m) => m.values_mut(),
        }
    }

    #[inline]
    fn contains_key(&self, key: &K) -> bool {
        self.contains_key(key)
    }

    #[inline]
    fn get(&self, key: &K) -> Option<&V> {
        self.get(key)
    }
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use super::*;

    #[test]
    fn test_empty_map() {
        type FM = FrozenMap<i32, i32>;

        let m = FM::default();
        assert_eq!(m.len(), 0);
    }

    #[test]
    fn test_i32_map() {
        let m = FrozenMap::<i32, i32>::try_from([(1, 1), (2, 2), (3, 3), (4, 4), (5, 5), (6, 6)])
            .unwrap();
        assert_eq!(m.get(&6), Some(&6));
    }

    #[test]
    fn basic_u32_map() {
        let max_entries = [1, 2, 3, 4, 5, 6, 255, 256, 65535, 65536];

        for max in max_entries {
            let mut m = HashMap::<u32, String>::new();
            for i in 0..max {
                m.insert(i, format!("V{i}"));
            }

            let fm = m
                .iter()
                .map(|x| (*x.0, x.1.clone()))
                .collect::<FrozenMap<_, _>>();
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

            let mut m = HashMap::<u32, String>::new();
            for i in (0..max).map(|x| x * 2) {
                m.insert(i, "V{i}".to_string());
            }

            let fd = m
                .iter()
                .map(|x| (*x.0, x.1.clone()))
                .collect::<FrozenMap<_, _>>();
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

    #[test]
    fn test_iter() {
        let mut m = HashMap::new();
        m.insert(1, 10);
        m.insert(2, 20);
        m.insert(3, 30);
        m.insert(4, 40);
        let m = m.iter().collect::<FrozenMap<_, _>>();

        let mut iter = m.iter();
        println!("{iter:?}");
        iter.next();
        println!("{iter:?}");
    }
}
