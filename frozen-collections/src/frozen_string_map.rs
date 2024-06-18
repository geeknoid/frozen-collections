use std::fmt::{Debug, Formatter, Result};
use std::hash::BuildHasher;
use std::ops::Index;
use std::ops::IndexMut;

use ahash::RandomState;

use frozen_collections_core::analyzers::{analyze_slice_keys, SliceKeyAnalysisResult};
use frozen_collections_core::traits::Map;

use crate::specialized_maps::*;
use crate::Len;

/// The different implementations available for use, depending on the payload.
#[derive(Clone)]
enum MapTypes<V, BH> {
    LeftSlice(LeftSliceMap<String, V, usize, BH>),
    RightSlice(RightSliceMap<String, V, usize, BH>),
    Common(CommonMap<String, V, usize, BH>),
}

/// A map optimized for fast read access using string keys.
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
/// # use frozen_collections::FrozenStringMap;
/// # use frozen_collections::Len;
/// #
/// let book_reviews = FrozenStringMap::new(vec![
///     ("Adventures of Huckleberry Finn".to_string(), "My favorite book."),
///     ("Grimms' Fairy Tales".to_string(), "Masterpiece."),
///     ("Pride and Prejudice".to_string(), "Very enjoyable."),
///     ("The Adventures of Sherlock Holmes".to_string(), "I liked it a lot."),
/// ]).unwrap();
///
/// // Check for a specific one.
/// if !book_reviews.contains_key(&"Les Misérables") {
///     println!("We've got {} reviews, but Les Misérables ain't one.",
///              book_reviews.len());
/// }
///
/// // Look up the values associated with some keys.
/// let to_find = ["Pride and Prejudice", "Alice's Adventure in Wonderland"];
/// for &book in &to_find {
///     match book_reviews.get(&book) {
///         Some(review) => println!("{book}: {review}"),
///         None => println!("{book} is unreviewed.")
///     }
/// }
///
/// // Look up the value for a key (will panic if the key is not found).
/// println!("Review for Jane: {}", book_reviews[&"Pride and Prejudice"]);
///
/// // Iterate over everything.
/// for (book, review) in &book_reviews {
///     println!("{book}: \"{review}\"");
/// }
/// ```
#[derive(Clone)]
#[allow(clippy::module_name_repetitions)]
pub struct FrozenStringMap<V, BH = RandomState> {
    map_impl: MapTypes<V, BH>,
}

impl<V> FrozenStringMap<V, RandomState> {
    /// Creates a frozen map which will use the [`RandomState`] hasher to hash
    /// keys.
    ///
    /// # Errors
    ///
    /// This fails if there are duplicate keys within the vector.
    ///
    /// # Examples
    ///
    /// ```
    /// # use frozen_collections::FrozenStringMap;
    /// # use std::hash::RandomState;
    /// #
    /// let map = FrozenStringMap::new(vec![("One".to_string(), 1), ("Two".to_string(), 2)]).unwrap();
    /// ```
    pub fn new(payload: Vec<(String, V)>) -> std::result::Result<Self, &'static str> {
        Self::with_hasher(payload, RandomState::new())
    }
}

impl<V, BH> FrozenStringMap<V, BH>
where
    BH: BuildHasher,
{
    /// Creates a frozen map which will use the given hash builder to hash
    /// keys.
    ///
    /// # Errors
    ///
    /// This fails if there are duplicate keys within the vector.
    ///
    /// # Examples
    ///
    /// ```
    /// # use frozen_collections::FrozenStringMap;
    /// # use std::hash::RandomState;
    /// #
    /// let map = FrozenStringMap::with_hasher(vec![("One".to_string(), 1), ("Two".to_string(), 2)], RandomState::new()).unwrap();
    /// ```
    pub fn with_hasher(
        payload: Vec<(String, V)>,
        bh: BH,
    ) -> std::result::Result<Self, &'static str> {
        Ok(Self {
            map_impl: {
                let key_analysis = analyze_slice_keys(payload.iter().map(|x| x.0.as_bytes()), &bh);

                match key_analysis {
                    SliceKeyAnalysisResult::Normal | SliceKeyAnalysisResult::Length => {
                        MapTypes::Common(CommonMap::with_hasher(payload, bh)?)
                    }

                    SliceKeyAnalysisResult::LeftHandSubslice(range) => {
                        MapTypes::LeftSlice(LeftSliceMap::with_hasher(payload, range, bh)?)
                    }

                    SliceKeyAnalysisResult::RightHandSubslice(range) => {
                        MapTypes::RightSlice(RightSliceMap::with_hasher(payload, range, bh)?)
                    }
                }
            },
        })
    }

    /// Returns a reference to the value corresponding to the key.
    ///
    /// # Examples
    ///
    /// ```
    /// # use frozen_collections::FrozenStringMap;
    /// #
    /// let map = FrozenStringMap::new(vec![("a".to_string(), 1)]).unwrap();
    ///
    /// assert_eq!(map.get(&"a"), Some(&1));
    /// assert_eq!(map.get(&"b"), None);
    /// ```
    #[inline(always)]
    #[must_use]
    pub fn get(&self, key: &str) -> Option<&V> {
        match &self.map_impl {
            MapTypes::LeftSlice(m) => m.get(key),
            MapTypes::RightSlice(m) => m.get(key),
            MapTypes::Common(m) => m.get(key),
        }
    }

    /// Returns the key-value pair corresponding to the supplied key.
    ///
    /// # Examples
    ///
    /// ```
    /// # use frozen_collections::FrozenStringMap;
    /// #
    /// let map = FrozenStringMap::new(vec![("a".to_string(), 1)]).unwrap();
    ///
    /// assert_eq!(map.get_key_value(&"a"), Some((&"a".to_string(), &1)));
    /// assert_eq!(map.get_key_value(&"b"), None);
    /// ```
    #[inline]
    #[must_use]
    pub fn get_key_value(&self, key: &str) -> Option<(&String, &V)> {
        match &self.map_impl {
            MapTypes::LeftSlice(m) => m.get_key_value(key),
            MapTypes::RightSlice(m) => m.get_key_value(key),
            MapTypes::Common(m) => m.get_key_value(key),
        }
    }

    /// Returns a mutable reference to the value corresponding to the key.
    ///
    /// # Examples
    ///
    /// ```
    /// # use frozen_collections::FrozenStringMap;
    /// #
    /// let mut map = FrozenStringMap::new(vec![("a".to_string(), 1)]).unwrap();
    ///
    /// assert_eq!(map.get_mut(&"a"), Some(&mut 1));
    /// assert_eq!(map.get_mut(&"b"), None);
    /// ```
    #[inline]
    #[must_use]
    pub fn get_mut(&mut self, key: &str) -> Option<&mut V> {
        match &mut self.map_impl {
            MapTypes::LeftSlice(m) => m.get_mut(key),
            MapTypes::RightSlice(m) => m.get_mut(key),
            MapTypes::Common(m) => m.get_mut(key),
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
    /// # use frozen_collections::FrozenStringMap;
    /// #
    /// let mut libraries = FrozenStringMap::new(vec![
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
    pub fn get_many_mut<const N: usize>(&mut self, keys: [&str; N]) -> Option<[&mut V; N]> {
        match &mut self.map_impl {
            MapTypes::LeftSlice(m) => m.get_many_mut(keys),
            MapTypes::RightSlice(m) => m.get_many_mut(keys),
            MapTypes::Common(m) => m.get_many_mut(keys),
        }
    }

    /// Returns `true` if the map contains a value for the specified key.
    ///
    /// # Examples
    ///
    /// ```
    /// # use frozen_collections::FrozenStringMap;
    /// #
    /// let map = FrozenStringMap::new(vec![("a".to_string(), 1)]).unwrap();
    ///
    /// assert_eq!(map.contains_key(&"a"), true);
    /// assert_eq!(map.contains_key(&"b"), false);
    /// ```
    #[inline]
    #[must_use]
    pub fn contains_key(&self, key: &str) -> bool {
        self.get(key).is_some()
    }
}

impl<V, BH> Len for FrozenStringMap<V, BH> {
    fn len(&self) -> usize {
        match &self.map_impl {
            MapTypes::LeftSlice(m) => m.len(),
            MapTypes::RightSlice(m) => m.len(),
            MapTypes::Common(m) => m.len(),
        }
    }
}

impl<V, BH> TryFrom<Vec<(String, V)>> for FrozenStringMap<V, BH>
where
    BH: BuildHasher + Default,
{
    type Error = &'static str;

    fn try_from(payload: Vec<(String, V)>) -> std::result::Result<Self, Self::Error> {
        Self::with_hasher(payload, BH::default())
    }
}

impl<V, BH> TryFrom<Vec<(&str, V)>> for FrozenStringMap<V, BH>
where
    BH: BuildHasher + Default,
{
    type Error = &'static str;

    fn try_from(payload: Vec<(&str, V)>) -> std::result::Result<Self, Self::Error> {
        Self::with_hasher(
            Vec::from_iter(payload.into_iter().map(|x| (x.0.to_string(), x.1))),
            BH::default(),
        )
    }
}

impl<V, const N: usize, BH> TryFrom<[(String, V); N]> for FrozenStringMap<V, BH>
where
    BH: BuildHasher + Default,
{
    type Error = &'static str;

    fn try_from(payload: [(String, V); N]) -> std::result::Result<Self, Self::Error> {
        Self::try_from(Vec::from_iter(payload))
    }
}

impl<V, const N: usize, BH> TryFrom<[(&str, V); N]> for FrozenStringMap<V, BH>
where
    BH: BuildHasher + Default,
{
    type Error = &'static str;

    fn try_from(payload: [(&str, V); N]) -> std::result::Result<Self, Self::Error> {
        Self::try_from(Vec::from_iter(
            payload.into_iter().map(|x| (x.0.to_string(), x.1)),
        ))
    }
}

impl<V, BH> FromIterator<(String, V)> for FrozenStringMap<V, BH>
where
    BH: BuildHasher + Default,
{
    fn from_iter<T: IntoIterator<Item = (String, V)>>(iter: T) -> Self {
        Self::try_from(Vec::from_iter(iter)).unwrap()
    }
}

impl<'a, V, BH> FromIterator<(&'a str, V)> for FrozenStringMap<V, BH>
where
    BH: BuildHasher + Default,
{
    fn from_iter<T: IntoIterator<Item = (&'a str, V)>>(iter: T) -> Self {
        Self::try_from(Vec::from_iter(
            iter.into_iter().map(|x| (x.0.to_string(), x.1)),
        ))
        .unwrap()
    }
}

impl<V, BH> Index<&str> for FrozenStringMap<V, BH>
where
    BH: BuildHasher,
{
    type Output = V;

    fn index(&self, index: &str) -> &Self::Output {
        self.get(index).unwrap()
    }
}

impl<V, BH> IndexMut<&str> for FrozenStringMap<V, BH>
where
    BH: BuildHasher,
{
    fn index_mut(&mut self, index: &str) -> &mut V {
        self.get_mut(index).unwrap()
    }
}

impl<V, BH> Default for FrozenStringMap<V, BH>
where
    BH: BuildHasher + Default,
{
    fn default() -> Self {
        Self {
            map_impl: MapTypes::Common(CommonMap::default()),
        }
    }
}

impl<V, BH> Debug for FrozenStringMap<V, BH>
where
    V: Debug,
{
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        match &self.map_impl {
            MapTypes::LeftSlice(m) => m.fmt(f),
            MapTypes::RightSlice(m) => m.fmt(f),
            MapTypes::Common(m) => m.fmt(f),
        }
    }
}

impl<V, MT, BH> PartialEq<MT> for FrozenStringMap<V, BH>
where
    V: PartialEq,
    MT: Map<String, V>,
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

impl<V, BH> Eq for FrozenStringMap<V, BH>
where
    V: Eq,
    BH: BuildHasher,
{
}

impl<'a, V, BH> IntoIterator for &'a FrozenStringMap<V, BH>
where
    BH: BuildHasher,
{
    type Item = (&'a String, &'a V);
    type IntoIter = Iter<'a, String, V>;

    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}

impl<'a, V, BH> IntoIterator for &'a mut FrozenStringMap<V, BH>
where
    BH: BuildHasher,
{
    type Item = (&'a String, &'a mut V);
    type IntoIter = IterMut<'a, String, V>;

    fn into_iter(self) -> Self::IntoIter {
        self.iter_mut()
    }
}

impl<V, BH> IntoIterator for FrozenStringMap<V, BH>
where
    BH: BuildHasher,
{
    type Item = (String, V);
    type IntoIter = IntoIter<String, V>;

    fn into_iter(self) -> Self::IntoIter {
        match self.map_impl {
            MapTypes::LeftSlice(m) => m.into_iter(),
            MapTypes::RightSlice(m) => m.into_iter(),
            MapTypes::Common(m) => m.into_iter(),
        }
    }
}

impl<V, BH> Map<String, V> for FrozenStringMap<V, BH>
where
    BH: BuildHasher,
{
    type Iterator<'a> = Iter<'a, String, V>
    where
        V: 'a,
        BH: 'a;

    type KeyIterator<'a> = Keys<'a, String, V>
    where
        V: 'a,
        BH: 'a;

    type ValueIterator<'a> = Values<'a, String, V>
    where
        V: 'a,
        BH: 'a;

    type IntoKeyIterator = IntoKeys<String, V>;
    type IntoValueIterator = IntoValues<String, V>;

    type MutIterator<'a> = IterMut<'a, String, V>
    where
        V: 'a,
        BH: 'a;

    type ValueMutIterator<'a> = ValuesMut<'a, String, V>
    where
        V: 'a,
        BH: 'a;

    fn iter(&self) -> Self::Iterator<'_> {
        match &self.map_impl {
            MapTypes::LeftSlice(m) => m.iter(),
            MapTypes::RightSlice(m) => m.iter(),
            MapTypes::Common(m) => m.iter(),
        }
    }

    fn keys(&self) -> Self::KeyIterator<'_> {
        match &self.map_impl {
            MapTypes::LeftSlice(m) => m.keys(),
            MapTypes::RightSlice(m) => m.keys(),
            MapTypes::Common(m) => m.keys(),
        }
    }

    fn values(&self) -> Self::ValueIterator<'_> {
        match &self.map_impl {
            MapTypes::LeftSlice(m) => m.values(),
            MapTypes::RightSlice(m) => m.values(),
            MapTypes::Common(m) => m.values(),
        }
    }

    fn into_keys(self) -> Self::IntoKeyIterator {
        match self.map_impl {
            MapTypes::LeftSlice(m) => m.into_keys(),
            MapTypes::RightSlice(m) => m.into_keys(),
            MapTypes::Common(m) => m.into_keys(),
        }
    }

    fn into_values(self) -> Self::IntoValueIterator {
        match self.map_impl {
            MapTypes::LeftSlice(m) => m.into_values(),
            MapTypes::RightSlice(m) => m.into_values(),
            MapTypes::Common(m) => m.into_values(),
        }
    }

    fn iter_mut(&mut self) -> Self::MutIterator<'_> {
        match &mut self.map_impl {
            MapTypes::LeftSlice(m) => m.iter_mut(),
            MapTypes::RightSlice(m) => m.iter_mut(),
            MapTypes::Common(m) => m.iter_mut(),
        }
    }

    fn values_mut(&mut self) -> Self::ValueMutIterator<'_> {
        match &mut self.map_impl {
            MapTypes::LeftSlice(m) => m.values_mut(),
            MapTypes::RightSlice(m) => m.values_mut(),
            MapTypes::Common(m) => m.values_mut(),
        }
    }

    #[inline]
    fn contains_key(&self, key: &String) -> bool {
        self.contains_key(key)
    }

    #[inline]
    fn get(&self, key: &String) -> Option<&V> {
        Self::get(self, key)
    }
}
