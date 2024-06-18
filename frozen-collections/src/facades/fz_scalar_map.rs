use alloc::vec::Vec;
use core::borrow::Borrow;
use core::fmt::{Debug, Formatter, Result};
use core::ops::Index;

use frozen_collections_core::analyzers::{analyze_scalar_keys, ScalarKeyAnalysisResult};
use frozen_collections_core::hashers::PassthroughHasher;
use frozen_collections_core::maps::{
    DenseScalarLookupMap, HashMap, IntoIter, IntoKeys, IntoValues, Iter, IterMut, Keys,
    SparseScalarLookupMap, Values, ValuesMut,
};
use frozen_collections_core::traits::{LargeCollection, Len, Map, MapIterator, Scalar};
use frozen_collections_core::utils::dedup_by_keep_last;

/// The different implementations available for use, depending on the entries.
#[derive(Clone)]
enum MapTypes<K, V> {
    Hash(HashMap<K, V, LargeCollection, PassthroughHasher>),
    Dense(DenseScalarLookupMap<K, V>),
    Sparse(SparseScalarLookupMap<K, V, LargeCollection>),
}

/// A map optimized for fast read access using scalar keys.
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
/// # Alternate Choices
///
/// If all your keys are known at compile time, you are much better off using the
/// [`fz_scalar_map!`](crate::fz_scalar_map!) macro rather than this type. This will result in considerably
/// better performance.
///
/// # Examples
///
/// ```
/// # use frozen_collections::FzScalarMap;
/// # use frozen_collections::Len;
/// #
/// let map = FzScalarMap::from([(1, "One"), (2, "Two"), (3, "Three")]);
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
pub struct FzScalarMap<K, V> {
    map_impl: MapTypes<K, V>,
}

impl<K, V> FzScalarMap<K, V>
where
    K: Scalar,
{
    /// Creates a frozen map.
    ///
    /// # Examples
    ///
    /// ```
    /// # use frozen_collections::FzScalarMap;
    /// # use frozen_collections::Len;
    /// #
    /// let map = FzScalarMap::new(vec![(1, 2), (3, 4)]);
    ///
    /// assert_eq!(2, map.len());
    /// assert_eq!(Some(&2), map.get(&1));
    /// ```
    #[must_use]
    #[allow(clippy::missing_panics_doc)]
    pub fn new(mut entries: Vec<(K, V)>) -> Self {
        entries.sort_by(|x, y| x.0.cmp(&y.0));
        dedup_by_keep_last(&mut entries, |x, y| x.0.eq(&y.0));

        Self {
            map_impl: match analyze_scalar_keys(entries.iter().map(|x| x.0)) {
                ScalarKeyAnalysisResult::DenseRange => {
                    MapTypes::Dense(DenseScalarLookupMap::new_raw(entries))
                }
                ScalarKeyAnalysisResult::SparseRange => {
                    MapTypes::Sparse(SparseScalarLookupMap::new_raw(entries))
                }
                ScalarKeyAnalysisResult::General => {
                    let h = PassthroughHasher::new();
                    MapTypes::Hash(HashMap::new(entries, h).unwrap())
                }
            },
        }
    }
}

impl<K, V> FzScalarMap<K, V> {
    /// Returns a reference to the value corresponding to the key.
    ///
    /// # Examples
    ///
    /// ```
    /// # use frozen_collections::FzScalarMap;
    /// #
    /// let map = FzScalarMap::from([(1, "a")]);
    ///
    /// assert_eq!(map.get(&1), Some(&"a"));
    /// assert_eq!(map.get(&2), None);
    /// ```
    #[inline(always)]
    #[must_use]
    pub fn get<Q>(&self, key: &Q) -> Option<&V>
    where
        K: Borrow<Q>,
        Q: Scalar,
    {
        match &self.map_impl {
            MapTypes::Hash(m) => m.get(key),
            MapTypes::Dense(m) => m.get(key),
            MapTypes::Sparse(m) => m.get(key),
        }
    }

    /// Returns the key-value pair corresponding to the supplied key.
    ///
    /// # Examples
    ///
    /// ```
    /// # use frozen_collections::FzScalarMap;
    /// #
    /// let map = FzScalarMap::from([(1, "a")]);
    ///
    /// assert_eq!(map.get_key_value(&1), Some((&1, &"a")));
    /// assert_eq!(map.get_key_value(&2), None);
    /// ```
    #[inline]
    #[must_use]
    pub fn get_key_value<Q>(&self, key: &Q) -> Option<(&K, &V)>
    where
        K: Borrow<Q>,
        Q: Scalar,
    {
        match &self.map_impl {
            MapTypes::Hash(m) => m.get_key_value(key),
            MapTypes::Dense(m) => m.get_key_value(key),
            MapTypes::Sparse(m) => m.get_key_value(key),
        }
    }

    /// Returns a mutable reference to the value corresponding to the key.
    ///
    /// # Examples
    ///
    /// ```
    /// # use frozen_collections::FzScalarMap;
    /// #
    /// let mut map = FzScalarMap::from([(1, "a")]);
    ///
    /// assert_eq!(map.get_mut(&1), Some(&mut "a"));
    /// assert_eq!(map.get_mut(&2), None);
    /// ```
    #[inline]
    #[must_use]
    pub fn get_mut<Q>(&mut self, key: &Q) -> Option<&mut V>
    where
        K: Borrow<Q>,
        Q: Scalar,
    {
        match &mut self.map_impl {
            MapTypes::Hash(m) => m.get_mut(key),
            MapTypes::Dense(m) => m.get_mut(key),
            MapTypes::Sparse(m) => m.get_mut(key),
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
        Q: Scalar,
    {
        match &mut self.map_impl {
            MapTypes::Hash(m) => m.get_many_mut(keys),
            MapTypes::Dense(m) => m.get_many_mut(keys),
            MapTypes::Sparse(m) => m.get_many_mut(keys),
        }
    }

    /// Returns `true` if the map contains a value for the specified key.
    ///
    /// # Examples
    ///
    /// ```
    /// # use frozen_collections::FzScalarMap;
    /// #
    /// let map = FzScalarMap::from([(1, "a")]);
    ///
    /// assert_eq!(map.contains_key(&1), true);
    /// assert_eq!(map.contains_key(&2), false);
    /// ```
    #[inline]
    #[must_use]
    pub fn contains_key<Q>(&self, key: &Q) -> bool
    where
        K: Borrow<Q>,
        Q: Scalar,
    {
        self.get(key).is_some()
    }
}

impl<K, V> Len for FzScalarMap<K, V> {
    fn len(&self) -> usize {
        match &self.map_impl {
            MapTypes::Hash(m) => m.len(),
            MapTypes::Dense(m) => m.len(),
            MapTypes::Sparse(m) => m.len(),
        }
    }
}

impl<K, V> From<Vec<(K, V)>> for FzScalarMap<K, V>
where
    K: Scalar,
{
    fn from(entries: Vec<(K, V)>) -> Self {
        Self::new(entries)
    }
}

impl<K, V, const N: usize> From<[(K, V); N]> for FzScalarMap<K, V>
where
    K: Scalar,
{
    fn from(entries: [(K, V); N]) -> Self {
        Self::new(Vec::from_iter(entries))
    }
}

impl<K, V> FromIterator<(K, V)> for FzScalarMap<K, V>
where
    K: Scalar,
{
    fn from_iter<T: IntoIterator<Item = (K, V)>>(iter: T) -> Self {
        Self::new(Vec::from_iter(iter))
    }
}

impl<K, V, Q> Index<&Q> for FzScalarMap<K, V>
where
    K: Borrow<Q>,
    Q: Scalar,
{
    type Output = V;

    fn index(&self, index: &Q) -> &Self::Output {
        self.get(index).expect("index should be valid")
    }
}

impl<K, V> Default for FzScalarMap<K, V>
where
    K: Scalar,
{
    fn default() -> Self {
        Self {
            map_impl: MapTypes::Dense(DenseScalarLookupMap::default()),
        }
    }
}

impl<K, V> Debug for FzScalarMap<K, V>
where
    K: Debug,
    V: Debug,
{
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        match &self.map_impl {
            MapTypes::Hash(m) => m.fmt(f),
            MapTypes::Dense(m) => m.fmt(f),
            MapTypes::Sparse(m) => m.fmt(f),
        }
    }
}

impl<K, V, MT> PartialEq<MT> for FzScalarMap<K, V>
where
    K: Scalar,
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

impl<K, V> Eq for FzScalarMap<K, V>
where
    K: Scalar,
    V: Eq,
{
}

impl<'a, K, V> IntoIterator for &'a FzScalarMap<K, V> {
    type Item = (&'a K, &'a V);
    type IntoIter = Iter<'a, K, V>;

    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}

impl<'a, K, V> IntoIterator for &'a mut FzScalarMap<K, V> {
    type Item = (&'a K, &'a mut V);
    type IntoIter = IterMut<'a, K, V>;

    fn into_iter(self) -> Self::IntoIter {
        self.iter_mut()
    }
}

impl<K, V> IntoIterator for FzScalarMap<K, V> {
    type Item = (K, V);
    type IntoIter = IntoIter<K, V>;

    fn into_iter(self) -> Self::IntoIter {
        match self.map_impl {
            MapTypes::Hash(m) => m.into_iter(),
            MapTypes::Dense(m) => m.into_iter(),
            MapTypes::Sparse(m) => m.into_iter(),
        }
    }
}

impl<K, V> MapIterator<K, V> for FzScalarMap<K, V> {
    type Iterator<'a>
        = Iter<'a, K, V>
    where
        K: 'a,
        V: 'a;

    type KeyIterator<'a>
        = Keys<'a, K, V>
    where
        K: 'a,
        V: 'a;

    type ValueIterator<'a>
        = Values<'a, K, V>
    where
        K: 'a,
        V: 'a;

    type IntoKeyIterator = IntoKeys<K, V>;
    type IntoValueIterator = IntoValues<K, V>;

    type MutIterator<'a>
        = IterMut<'a, K, V>
    where
        K: 'a,
        V: 'a;

    type ValueMutIterator<'a>
        = ValuesMut<'a, K, V>
    where
        K: 'a,
        V: 'a;

    fn iter(&self) -> Self::Iterator<'_> {
        match &self.map_impl {
            MapTypes::Hash(m) => m.iter(),
            MapTypes::Dense(m) => m.iter(),
            MapTypes::Sparse(m) => m.iter(),
        }
    }

    fn keys(&self) -> Self::KeyIterator<'_> {
        match &self.map_impl {
            MapTypes::Hash(m) => m.keys(),
            MapTypes::Dense(m) => m.keys(),
            MapTypes::Sparse(m) => m.keys(),
        }
    }

    fn values(&self) -> Self::ValueIterator<'_> {
        match &self.map_impl {
            MapTypes::Hash(m) => m.values(),
            MapTypes::Dense(m) => m.values(),
            MapTypes::Sparse(m) => m.values(),
        }
    }

    fn into_keys(self) -> Self::IntoKeyIterator {
        match self.map_impl {
            MapTypes::Hash(m) => m.into_keys(),
            MapTypes::Dense(m) => m.into_keys(),
            MapTypes::Sparse(m) => m.into_keys(),
        }
    }

    fn into_values(self) -> Self::IntoValueIterator {
        match self.map_impl {
            MapTypes::Hash(m) => m.into_values(),
            MapTypes::Dense(m) => m.into_values(),
            MapTypes::Sparse(m) => m.into_values(),
        }
    }

    fn iter_mut(&mut self) -> Self::MutIterator<'_> {
        match &mut self.map_impl {
            MapTypes::Hash(m) => m.iter_mut(),
            MapTypes::Dense(m) => m.iter_mut(),
            MapTypes::Sparse(m) => m.iter_mut(),
        }
    }

    fn values_mut(&mut self) -> Self::ValueMutIterator<'_> {
        match &mut self.map_impl {
            MapTypes::Hash(m) => m.values_mut(),
            MapTypes::Dense(m) => m.values_mut(),
            MapTypes::Sparse(m) => m.values_mut(),
        }
    }
}

impl<K, V> Map<K, V> for FzScalarMap<K, V>
where
    K: Scalar,
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
