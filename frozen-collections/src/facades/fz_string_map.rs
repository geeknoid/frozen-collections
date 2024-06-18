use alloc::string::{String, ToString};
use alloc::vec::Vec;
use core::fmt::{Debug, Formatter, Result};
use core::hash::BuildHasher;
use core::ops::Index;

use ahash::RandomState;

use frozen_collections_core::analyzers::{analyze_slice_keys, SliceKeyAnalysisResult};
use frozen_collections_core::hashers::{BridgeHasher, LeftRangeHasher, RightRangeHasher};
use frozen_collections_core::maps::{
    HashMap, IntoIter, IntoKeys, IntoValues, Iter, IterMut, Keys, Values, ValuesMut,
};
use frozen_collections_core::traits::{LargeCollection, Len, Map, MapIterator};
use frozen_collections_core::utils::dedup_by_keep_last;

/// The different implementations available for use, depending on the entries.
#[derive(Clone)]
enum MapTypes<V, BH> {
    LeftRange(HashMap<String, V, LargeCollection, LeftRangeHasher<BH>>),
    RightRange(HashMap<String, V, LargeCollection, RightRangeHasher<BH>>),
    Hash(HashMap<String, V, LargeCollection, BridgeHasher<BH>>),
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
/// # Alternate Choices
///
/// If all your keys are known at compile time, you are much better off using the
/// [`fz_string_map!`](crate::fz_string_map!) macro rather than this type. This will result in considerably
/// better performance.
///
/// # Examples
///
/// ```
/// # use frozen_collections::FzStringMap;
/// # use frozen_collections::Len;
/// #
/// let book_reviews = FzStringMap::new(vec![
///     ("Adventures of Huckleberry Finn".to_string(), "My favorite book."),
///     ("Grimms' Fairy Tales".to_string(), "Masterpiece."),
///     ("Pride and Prejudice".to_string(), "Very enjoyable."),
///     ("The Adventures of Sherlock Holmes".to_string(), "I liked it a lot."),
/// ]);
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
pub struct FzStringMap<V, BH = RandomState> {
    map_impl: MapTypes<V, BH>,
}

impl<V> FzStringMap<V, RandomState> {
    /// Creates a frozen map which will use the [`RandomState`] hasher to hash
    /// keys.
    ///
    /// # Examples
    ///
    /// ```
    /// # use frozen_collections::FzStringMap;
    /// #
    /// let map = FzStringMap::new(vec![("One".to_string(), 1), ("Two".to_string(), 2)]);
    /// ```
    #[must_use]
    pub fn new(entries: Vec<(String, V)>) -> Self {
        Self::with_hasher(entries, RandomState::new())
    }
}

impl<V, BH> FzStringMap<V, BH>
where
    BH: BuildHasher,
{
    /// Creates a frozen map which will use the given hash builder to hash
    /// keys.
    ///
    /// # Examples
    ///
    /// ```
    /// # use frozen_collections::FzStringMap;
    /// # use ahash::RandomState;
    /// #
    /// let map = FzStringMap::with_hasher(vec![("One".to_string(), 1), ("Two".to_string(), 2)], RandomState::new());
    /// ```
    #[must_use]
    #[allow(clippy::missing_panics_doc)]
    pub fn with_hasher(mut entries: Vec<(String, V)>, bh: BH) -> Self {
        entries.sort_by(|x, y| x.0.cmp(&y.0));
        dedup_by_keep_last(&mut entries, |x, y| x.0.eq(&y.0));

        Self {
            map_impl: {
                match analyze_slice_keys(entries.iter().map(|x| x.0.as_bytes()), &bh) {
                    SliceKeyAnalysisResult::General | SliceKeyAnalysisResult::Length => {
                        let h = BridgeHasher::new(bh);
                        MapTypes::Hash(HashMap::new(entries, h).unwrap())
                    }

                    SliceKeyAnalysisResult::LeftHandSubslice(range) => {
                        let h = LeftRangeHasher::new(bh, range);
                        MapTypes::LeftRange(HashMap::new(entries, h).unwrap())
                    }

                    SliceKeyAnalysisResult::RightHandSubslice(range) => {
                        let h = RightRangeHasher::new(bh, range);
                        MapTypes::RightRange(HashMap::new(entries, h).unwrap())
                    }
                }
            },
        }
    }

    /// Returns a reference to the value corresponding to the key.
    ///
    /// # Examples
    ///
    /// ```
    /// # use frozen_collections::FzStringMap;
    /// #
    /// let map = FzStringMap::new(vec![("a".to_string(), 1)]);
    ///
    /// assert_eq!(map.get(&"a"), Some(&1));
    /// assert_eq!(map.get(&"b"), None);
    /// ```
    #[inline(always)]
    #[must_use]
    pub fn get(&self, key: &str) -> Option<&V> {
        match &self.map_impl {
            MapTypes::LeftRange(m) => m.get(key),
            MapTypes::RightRange(m) => m.get(key),
            MapTypes::Hash(m) => m.get(key),
        }
    }

    /// Returns the key-value pair corresponding to the supplied key.
    ///
    /// # Examples
    ///
    /// ```
    /// # use frozen_collections::FzStringMap;
    /// #
    /// let map = FzStringMap::new(vec![("a".to_string(), 1)]);
    ///
    /// assert_eq!(map.get_key_value(&"a"), Some((&"a".to_string(), &1)));
    /// assert_eq!(map.get_key_value(&"b"), None);
    /// ```
    #[inline]
    #[must_use]
    pub fn get_key_value(&self, key: &str) -> Option<(&String, &V)> {
        match &self.map_impl {
            MapTypes::LeftRange(m) => m.get_key_value(key),
            MapTypes::RightRange(m) => m.get_key_value(key),
            MapTypes::Hash(m) => m.get_key_value(key),
        }
    }

    /// Returns a mutable reference to the value corresponding to the key.
    ///
    /// # Examples
    ///
    /// ```
    /// # use frozen_collections::FzStringMap;
    /// #
    /// let mut map = FzStringMap::new(vec![("a".to_string(), 1)]);
    ///
    /// assert_eq!(map.get_mut(&"a"), Some(&mut 1));
    /// assert_eq!(map.get_mut(&"b"), None);
    /// ```
    #[inline]
    #[must_use]
    pub fn get_mut(&mut self, key: &str) -> Option<&mut V> {
        match &mut self.map_impl {
            MapTypes::LeftRange(m) => m.get_mut(key),
            MapTypes::RightRange(m) => m.get_mut(key),
            MapTypes::Hash(m) => m.get_mut(key),
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
    /// # use frozen_collections::FzStringMap;
    /// #
    /// let mut libraries = FzStringMap::new(vec![
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
    pub fn get_many_mut<const N: usize>(&mut self, keys: [&str; N]) -> Option<[&mut V; N]> {
        match &mut self.map_impl {
            MapTypes::LeftRange(m) => m.get_many_mut(keys),
            MapTypes::RightRange(m) => m.get_many_mut(keys),
            MapTypes::Hash(m) => m.get_many_mut(keys),
        }
    }

    /// Returns `true` if the map contains a value for the specified key.
    ///
    /// # Examples
    ///
    /// ```
    /// # use frozen_collections::FzStringMap;
    /// #
    /// let map = FzStringMap::new(vec![("a".to_string(), 1)]);
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

impl<V, BH> Len for FzStringMap<V, BH> {
    fn len(&self) -> usize {
        match &self.map_impl {
            MapTypes::LeftRange(m) => m.len(),
            MapTypes::RightRange(m) => m.len(),
            MapTypes::Hash(m) => m.len(),
        }
    }
}

impl<V, BH> From<Vec<(String, V)>> for FzStringMap<V, BH>
where
    BH: BuildHasher + Default,
{
    fn from(entries: Vec<(String, V)>) -> Self {
        Self::with_hasher(entries, BH::default())
    }
}

impl<V, BH> From<Vec<(&str, V)>> for FzStringMap<V, BH>
where
    BH: BuildHasher + Default,
{
    fn from(entries: Vec<(&str, V)>) -> Self {
        Self::with_hasher(
            Vec::from_iter(entries.into_iter().map(|x| (x.0.to_string(), x.1))),
            BH::default(),
        )
    }
}

impl<V, const N: usize, BH> From<[(String, V); N]> for FzStringMap<V, BH>
where
    BH: BuildHasher + Default,
{
    fn from(entries: [(String, V); N]) -> Self {
        Self::with_hasher(Vec::from_iter(entries), BH::default())
    }
}

impl<V, const N: usize, BH> From<[(&str, V); N]> for FzStringMap<V, BH>
where
    BH: BuildHasher + Default,
{
    fn from(entries: [(&str, V); N]) -> Self {
        Self::with_hasher(
            Vec::from_iter(entries.into_iter().map(|x| (x.0.to_string(), x.1))),
            BH::default(),
        )
    }
}

impl<V, BH> FromIterator<(String, V)> for FzStringMap<V, BH>
where
    BH: BuildHasher + Default,
{
    fn from_iter<T: IntoIterator<Item = (String, V)>>(iter: T) -> Self {
        Self::with_hasher(Vec::from_iter(iter), BH::default())
    }
}

impl<'a, V, BH> FromIterator<(&'a str, V)> for FzStringMap<V, BH>
where
    BH: BuildHasher + Default,
{
    fn from_iter<T: IntoIterator<Item = (&'a str, V)>>(iter: T) -> Self {
        Self::with_hasher(
            Vec::from_iter(iter.into_iter().map(|x| (x.0.to_string(), x.1))),
            BH::default(),
        )
    }
}

impl<V, BH> Index<&str> for FzStringMap<V, BH>
where
    BH: BuildHasher,
{
    type Output = V;

    fn index(&self, index: &str) -> &Self::Output {
        self.get(index).expect("index should be valid")
    }
}

impl<V, BH> Default for FzStringMap<V, BH>
where
    BH: Default,
{
    fn default() -> Self {
        Self {
            map_impl: MapTypes::Hash(HashMap::default()),
        }
    }
}

impl<V, BH> Debug for FzStringMap<V, BH>
where
    V: Debug,
{
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        match &self.map_impl {
            MapTypes::LeftRange(m) => m.fmt(f),
            MapTypes::RightRange(m) => m.fmt(f),
            MapTypes::Hash(m) => m.fmt(f),
        }
    }
}

impl<V, MT, BH> PartialEq<MT> for FzStringMap<V, BH>
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

impl<V, BH> Eq for FzStringMap<V, BH>
where
    V: Eq,
    BH: BuildHasher,
{
}

impl<'a, V, BH> IntoIterator for &'a FzStringMap<V, BH> {
    type Item = (&'a String, &'a V);
    type IntoIter = Iter<'a, String, V>;

    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}

impl<'a, V, BH> IntoIterator for &'a mut FzStringMap<V, BH> {
    type Item = (&'a String, &'a mut V);
    type IntoIter = IterMut<'a, String, V>;

    fn into_iter(self) -> Self::IntoIter {
        self.iter_mut()
    }
}

impl<V, BH> IntoIterator for FzStringMap<V, BH> {
    type Item = (String, V);
    type IntoIter = IntoIter<String, V>;

    fn into_iter(self) -> Self::IntoIter {
        match self.map_impl {
            MapTypes::LeftRange(m) => m.into_iter(),
            MapTypes::RightRange(m) => m.into_iter(),
            MapTypes::Hash(m) => m.into_iter(),
        }
    }
}

impl<V, BH> MapIterator<String, V> for FzStringMap<V, BH> {
    type Iterator<'a>
        = Iter<'a, String, V>
    where
        V: 'a,
        BH: 'a;

    type KeyIterator<'a>
        = Keys<'a, String, V>
    where
        V: 'a,
        BH: 'a;

    type ValueIterator<'a>
        = Values<'a, String, V>
    where
        V: 'a,
        BH: 'a;

    type IntoKeyIterator = IntoKeys<String, V>;
    type IntoValueIterator = IntoValues<String, V>;

    type MutIterator<'a>
        = IterMut<'a, String, V>
    where
        V: 'a,
        BH: 'a;

    type ValueMutIterator<'a>
        = ValuesMut<'a, String, V>
    where
        V: 'a,
        BH: 'a;

    fn iter(&self) -> Self::Iterator<'_> {
        match &self.map_impl {
            MapTypes::LeftRange(m) => m.iter(),
            MapTypes::RightRange(m) => m.iter(),
            MapTypes::Hash(m) => m.iter(),
        }
    }

    fn keys(&self) -> Self::KeyIterator<'_> {
        match &self.map_impl {
            MapTypes::LeftRange(m) => m.keys(),
            MapTypes::RightRange(m) => m.keys(),
            MapTypes::Hash(m) => m.keys(),
        }
    }

    fn values(&self) -> Self::ValueIterator<'_> {
        match &self.map_impl {
            MapTypes::LeftRange(m) => m.values(),
            MapTypes::RightRange(m) => m.values(),
            MapTypes::Hash(m) => m.values(),
        }
    }

    fn into_keys(self) -> Self::IntoKeyIterator {
        match self.map_impl {
            MapTypes::LeftRange(m) => m.into_keys(),
            MapTypes::RightRange(m) => m.into_keys(),
            MapTypes::Hash(m) => m.into_keys(),
        }
    }

    fn into_values(self) -> Self::IntoValueIterator {
        match self.map_impl {
            MapTypes::LeftRange(m) => m.into_values(),
            MapTypes::RightRange(m) => m.into_values(),
            MapTypes::Hash(m) => m.into_values(),
        }
    }

    fn iter_mut(&mut self) -> Self::MutIterator<'_> {
        match &mut self.map_impl {
            MapTypes::LeftRange(m) => m.iter_mut(),
            MapTypes::RightRange(m) => m.iter_mut(),
            MapTypes::Hash(m) => m.iter_mut(),
        }
    }

    fn values_mut(&mut self) -> Self::ValueMutIterator<'_> {
        match &mut self.map_impl {
            MapTypes::LeftRange(m) => m.values_mut(),
            MapTypes::RightRange(m) => m.values_mut(),
            MapTypes::Hash(m) => m.values_mut(),
        }
    }
}

impl<V, BH> Map<String, V> for FzStringMap<V, BH>
where
    BH: BuildHasher,
{
    #[inline]
    fn contains_key(&self, key: &String) -> bool {
        self.contains_key(key)
    }

    #[inline]
    fn get(&self, key: &String) -> Option<&V> {
        self.get(key)
    }

    #[inline]
    fn get_key_value(&self, key: &String) -> Option<(&String, &V)> {
        self.get_key_value(key)
    }

    #[inline]
    fn get_mut(&mut self, key: &String) -> Option<&mut V> {
        self.get_mut(key)
    }

    #[inline]
    fn get_many_mut<const N: usize>(&mut self, keys: [&String; N]) -> Option<[&mut V; N]> {
        self.get_many_mut(keys.map(String::as_str))
    }
}
