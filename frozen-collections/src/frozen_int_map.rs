use std::borrow::Borrow;
use std::fmt::{Debug, Formatter, Result};
use std::hash::Hash;
use std::ops::Index;
use std::ops::IndexMut;

use bitvec::macros::internal::funty::Fundamental;
use num_traits::{AsPrimitive, PrimInt};

use frozen_collections_core::analyzers::{analyze_int_keys, IntKeyAnalysisResult};
use frozen_collections_core::traits::Map;

use crate::specialized_maps::*;
use crate::Len;

/// The different implementations available for use, depending on the payload.
#[derive(Clone)]
enum MapTypes<K, V> {
    Small(IntegerMap<K, V, u8>),
    Large(IntegerMap<K, V, usize>),
    Range(IntegerRangeMap<K, V>),
}

/// A map optimized for fast read access using integer keys.
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
/// # Macros are Faster
///
/// If all your keys are known at compile time, you are much better off using the
/// [`frozen_map!`](crate::frozen_map!) macro rather than this type. This will result in considerably
/// better performance.
///
/// # Examples
///
/// ```
/// # use frozen_collections::FrozenIntMap;
/// # use frozen_collections::Len;
/// #
/// let map = FrozenIntMap::try_from([(1, "One"), (2, "Two"), (3, "Three")]).unwrap();
///
/// assert_eq!(3, map.len());
/// assert_eq!(&"Three", map.get(&3).unwrap());
///
/// // Iterate over everything.
/// for (key, value) in &map {
///     println!("{key}: \"{value}\"");
/// }
/// ```
#[derive(Clone)]
#[allow(clippy::module_name_repetitions)]
pub struct FrozenIntMap<K, V> {
    map_impl: MapTypes<K, V>,
}

impl<K, V> FrozenIntMap<K, V>
where
    K: PrimInt + AsPrimitive<u64> + Hash,
{
    /// Creates a frozen map.
    ///
    /// # Errors
    ///
    /// This fails if there are duplicate keys within the vector.
    ///
    /// # Examples
    ///
    /// ```
    /// # use frozen_collections::FrozenIntMap;
    /// #
    /// let map = FrozenIntMap::new(vec![(1, 2), (3, 4)]).unwrap();
    /// ```
    pub fn new(payload: Vec<(K, V)>) -> std::result::Result<Self, &'static str> {
        let key_analysis = analyze_int_keys(payload.iter().map(|x| x.0));

        Ok(Self {
            map_impl: match key_analysis {
                IntKeyAnalysisResult::Range => MapTypes::Range(IntegerRangeMap::try_from(payload)?),

                IntKeyAnalysisResult::Normal => {
                    if payload.len() <= u8::MAX.as_usize() {
                        MapTypes::Small(IntegerMap::try_from(payload)?)
                    } else {
                        MapTypes::Large(IntegerMap::try_from(payload)?)
                    }
                }
            },
        })
    }
}

impl<K, V> FrozenIntMap<K, V> {
    /// Returns a reference to the value corresponding to the key.
    ///
    /// # Examples
    ///
    /// ```
    /// # use frozen_collections::FrozenIntMap;
    /// #
    /// let map = FrozenIntMap::try_from([(1, "a")]).unwrap();
    ///
    /// assert_eq!(map.get(&1), Some(&"a"));
    /// assert_eq!(map.get(&2), None);
    /// ```
    #[inline(always)]
    #[must_use]
    pub fn get<Q>(&self, key: &Q) -> Option<&V>
    where
        K: Borrow<Q>,
        Q: PrimInt + AsPrimitive<u64> + Hash,
    {
        match &self.map_impl {
            MapTypes::Small(m) => m.get(key),
            MapTypes::Large(m) => m.get(key),
            MapTypes::Range(m) => m.get(key),
        }
    }

    /// Returns the key-value pair corresponding to the supplied key.
    ///
    /// # Examples
    ///
    /// ```
    /// # use frozen_collections::FrozenIntMap;
    /// #
    /// let map = FrozenIntMap::try_from([(1, "a")]).unwrap();
    ///
    /// assert_eq!(map.get_key_value(&1), Some((&1, &"a")));
    /// assert_eq!(map.get_key_value(&2), None);
    /// ```
    #[inline]
    #[must_use]
    pub fn get_key_value<Q>(&self, key: &Q) -> Option<(&K, &V)>
    where
        K: Borrow<Q>,
        Q: PrimInt + AsPrimitive<u64> + Hash,
    {
        match &self.map_impl {
            MapTypes::Small(m) => m.get_key_value(key),
            MapTypes::Large(m) => m.get_key_value(key),
            MapTypes::Range(m) => m.get_key_value(key),
        }
    }

    /// Returns a mutable reference to the value corresponding to the key.
    ///
    /// # Examples
    ///
    /// ```
    /// # use frozen_collections::FrozenIntMap;
    /// #
    /// let mut map = FrozenIntMap::try_from([(1, "a")]).unwrap();
    ///
    /// assert_eq!(map.get_mut(&1), Some(&mut "a"));
    /// assert_eq!(map.get_mut(&2), None);
    /// ```
    #[inline]
    #[must_use]
    pub fn get_mut<Q>(&mut self, key: &Q) -> Option<&mut V>
    where
        K: Borrow<Q>,
        Q: PrimInt + AsPrimitive<u64> + Hash,
    {
        match &mut self.map_impl {
            MapTypes::Small(m) => m.get_mut(key),
            MapTypes::Large(m) => m.get_mut(key),
            MapTypes::Range(m) => m.get_mut(key),
        }
    }

    /// Attempts to get mutable references to `N` values in the map at once.
    ///
    /// Returns an array of length `N` with the results of each query. For soundness, at most one
    /// mutable reference will be returned to any value. `None` will be returned if any of the
    /// keys are duplicates or missing.
    #[must_use]
    pub fn get_many_mut<Q, const N: usize>(&mut self, keys: [&Q; N]) -> Option<[&mut V; N]>
    where
        K: Borrow<Q>,
        Q: PrimInt + AsPrimitive<u64> + Hash,
    {
        match &mut self.map_impl {
            MapTypes::Small(m) => m.get_many_mut(keys),
            MapTypes::Large(m) => m.get_many_mut(keys),
            MapTypes::Range(m) => m.get_many_mut(keys),
        }
    }

    /// Returns `true` if the map contains a value for the specified key.
    ///
    /// # Examples
    ///
    /// ```
    /// # use frozen_collections::FrozenIntMap;
    /// #
    /// let map = FrozenIntMap::try_from([(1, "a")]).unwrap();
    ///
    /// assert_eq!(map.contains_key(&1), true);
    /// assert_eq!(map.contains_key(&2), false);
    /// ```
    #[inline]
    #[must_use]
    pub fn contains_key<Q>(&self, key: &Q) -> bool
    where
        K: Borrow<Q>,
        Q: PrimInt + AsPrimitive<u64> + Hash,
    {
        self.get(key).is_some()
    }
}

impl<K, V> Len for FrozenIntMap<K, V> {
    fn len(&self) -> usize {
        match &self.map_impl {
            MapTypes::Small(m) => m.len(),
            MapTypes::Large(m) => m.len(),
            MapTypes::Range(m) => m.len(),
        }
    }
}

impl<K, V> TryFrom<Vec<(K, V)>> for FrozenIntMap<K, V>
where
    K: PrimInt + AsPrimitive<u64> + Hash,
{
    type Error = &'static str;

    fn try_from(payload: Vec<(K, V)>) -> std::result::Result<Self, Self::Error> {
        Self::new(payload)
    }
}

impl<K, V, const N: usize> TryFrom<[(K, V); N]> for FrozenIntMap<K, V>
where
    K: PrimInt + AsPrimitive<u64> + Hash,
{
    type Error = &'static str;

    fn try_from(payload: [(K, V); N]) -> std::result::Result<Self, Self::Error> {
        Self::new(Vec::from_iter(payload))
    }
}

impl<K, V> FromIterator<(K, V)> for FrozenIntMap<K, V>
where
    K: PrimInt + AsPrimitive<u64> + Hash,
{
    fn from_iter<T: IntoIterator<Item = (K, V)>>(iter: T) -> Self {
        Self::new(Vec::from_iter(iter)).unwrap()
    }
}

impl<K, V> Index<&K> for FrozenIntMap<K, V>
where
    K: PrimInt + AsPrimitive<u64> + Hash,
{
    type Output = V;

    fn index(&self, index: &K) -> &Self::Output {
        self.get(index).unwrap()
    }
}

impl<K, V> IndexMut<&K> for FrozenIntMap<K, V>
where
    K: PrimInt + AsPrimitive<u64> + Hash,
{
    fn index_mut(&mut self, index: &K) -> &mut V {
        self.get_mut(index).unwrap()
    }
}

impl<K, V> Default for FrozenIntMap<K, V>
where
    K: PrimInt + AsPrimitive<u64> + Hash,
{
    fn default() -> Self {
        Self {
            map_impl: MapTypes::Range(IntegerRangeMap::default()),
        }
    }
}

impl<K, V> Debug for FrozenIntMap<K, V>
where
    K: Debug,
    V: Debug,
{
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        match &self.map_impl {
            MapTypes::Small(m) => m.fmt(f),
            MapTypes::Large(m) => m.fmt(f),
            MapTypes::Range(m) => m.fmt(f),
        }
    }
}

impl<K, V, MT> PartialEq<MT> for FrozenIntMap<K, V>
where
    K: PrimInt + AsPrimitive<u64> + Hash,
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

impl<K, V> Eq for FrozenIntMap<K, V>
where
    K: PrimInt + AsPrimitive<u64> + Hash,
    V: Eq,
{
}

impl<'a, K, V> IntoIterator for &'a FrozenIntMap<K, V>
where
    K: PrimInt + AsPrimitive<u64> + Hash,
{
    type Item = (&'a K, &'a V);
    type IntoIter = Iter<'a, K, V>;

    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}

impl<'a, K, V> IntoIterator for &'a mut FrozenIntMap<K, V>
where
    K: PrimInt + AsPrimitive<u64> + Hash,
{
    type Item = (&'a K, &'a mut V);
    type IntoIter = IterMut<'a, K, V>;

    fn into_iter(self) -> Self::IntoIter {
        self.iter_mut()
    }
}

impl<K, V> IntoIterator for FrozenIntMap<K, V>
where
    K: PrimInt + AsPrimitive<u64> + Hash,
{
    type Item = (K, V);
    type IntoIter = IntoIter<K, V>;

    fn into_iter(self) -> Self::IntoIter {
        match self.map_impl {
            MapTypes::Small(m) => m.into_iter(),
            MapTypes::Large(m) => m.into_iter(),
            MapTypes::Range(m) => m.into_iter(),
        }
    }
}

impl<K, V> Map<K, V> for FrozenIntMap<K, V>
where
    K: PrimInt + AsPrimitive<u64> + Hash,
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

    fn iter(&self) -> Self::Iterator<'_> {
        match &self.map_impl {
            MapTypes::Small(m) => m.iter(),
            MapTypes::Large(m) => m.iter(),
            MapTypes::Range(m) => m.iter(),
        }
    }

    fn keys(&self) -> Self::KeyIterator<'_> {
        match &self.map_impl {
            MapTypes::Small(m) => m.keys(),
            MapTypes::Large(m) => m.keys(),
            MapTypes::Range(m) => m.keys(),
        }
    }

    fn values(&self) -> Self::ValueIterator<'_> {
        match &self.map_impl {
            MapTypes::Small(m) => m.values(),
            MapTypes::Large(m) => m.values(),
            MapTypes::Range(m) => m.values(),
        }
    }

    fn into_keys(self) -> Self::IntoKeyIterator {
        match self.map_impl {
            MapTypes::Small(m) => m.into_keys(),
            MapTypes::Large(m) => m.into_keys(),
            MapTypes::Range(m) => m.into_keys(),
        }
    }

    fn into_values(self) -> Self::IntoValueIterator {
        match self.map_impl {
            MapTypes::Small(m) => m.into_values(),
            MapTypes::Large(m) => m.into_values(),
            MapTypes::Range(m) => m.into_values(),
        }
    }

    fn iter_mut(&mut self) -> Self::MutIterator<'_> {
        match &mut self.map_impl {
            MapTypes::Small(m) => m.iter_mut(),
            MapTypes::Large(m) => m.iter_mut(),
            MapTypes::Range(m) => m.iter_mut(),
        }
    }

    fn values_mut(&mut self) -> Self::ValueMutIterator<'_> {
        match &mut self.map_impl {
            MapTypes::Small(m) => m.values_mut(),
            MapTypes::Large(m) => m.values_mut(),
            MapTypes::Range(m) => m.values_mut(),
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
