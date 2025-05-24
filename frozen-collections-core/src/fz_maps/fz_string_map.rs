use crate::DefaultHashBuilder;
use crate::analyzers::{SliceKeyAnalysisResult, analyze_slice_keys};
use crate::hashers::{BridgeHasher, LeftRangeHasher, RightRangeHasher};
use crate::maps::{
    HashMap, IntoIter, IntoKeys, IntoValues, Iter, IterMut, Keys, Values, ValuesMut,
};
use crate::traits::{Hasher, LargeCollection, Len, Map, MapIteration, MapQuery};
use crate::utils::dedup_by_keep_last;
use alloc::string::String;
use alloc::vec::Vec;
use core::fmt::{Debug, Formatter, Result};
use core::hash::{BuildHasher, Hash};
use core::iter::FromIterator;
use core::ops::Index;
use equivalent::Equivalent;
use foldhash::fast::RandomState;
#[cfg(feature = "serde")]
use {
    crate::maps::decl_macros::serialize_fn,
    core::marker::PhantomData,
    serde::de::{MapAccess, Visitor},
    serde::ser::SerializeMap,
    serde::{Deserialize, Deserializer, Serialize, Serializer},
};

#[derive(Clone)]
enum MapTypes<K, V, BH> {
    LeftRange(HashMap<K, V, LargeCollection, LeftRangeHasher<BH>>),
    RightRange(HashMap<K, V, LargeCollection, RightRangeHasher<BH>>),
    Hash(HashMap<K, V, LargeCollection, BridgeHasher<BH>>),
}

/// A map optimized for fast read access with string keys.
///
#[doc = include_str!("../doc_snippets/about.md")]
#[doc = include_str!("../doc_snippets/hash_warning.md")]
///
/// # Alternate Choices
///
/// If your keys are known at compile time, consider using the various `fz_*_map` macros instead of
/// this type as they generally perform better.
#[derive(Clone)]
pub struct FzStringMap<K, V, BH = DefaultHashBuilder> {
    map_impl: MapTypes<K, V, BH>,
}

impl<'a, V> FzStringMap<&'a str, V, DefaultHashBuilder> {
    /// Creates a frozen map.
    #[must_use]
    pub fn new(entries: Vec<(&'a str, V)>) -> Self {
        Self::with_hasher(entries, RandomState::default())
    }
}

impl<V> FzStringMap<String, V, DefaultHashBuilder> {
    /// Creates a frozen map.
    #[must_use]
    pub fn new_with_strings(entries: Vec<(String, V)>) -> Self {
        Self::with_strings_and_hasher(entries, RandomState::default())
    }
}

impl<'a, V, BH> FzStringMap<&'a str, V, BH>
where
    BH: BuildHasher,
{
    /// Creates a frozen map which uses the given hash builder to hash keys.
    #[must_use]
    #[allow(clippy::missing_panics_doc)]
    pub fn with_hasher(mut entries: Vec<(&'a str, V)>, bh: BH) -> Self {
        entries.sort_by(|x, y| x.0.cmp(y.0));
        dedup_by_keep_last(&mut entries, |x, y| x.0.eq(y.0));

        Self {
            map_impl: {
                match analyze_slice_keys(entries.iter().map(|x| x.0.as_bytes()), &bh) {
                    SliceKeyAnalysisResult::General | SliceKeyAnalysisResult::Length => {
                        let h = BridgeHasher::new(bh);
                        MapTypes::Hash(HashMap::with_hasher_half_baked(entries, h).unwrap())
                    }

                    SliceKeyAnalysisResult::LeftHandSubslice(range) => {
                        let h = LeftRangeHasher::new(bh, range);
                        MapTypes::LeftRange(HashMap::with_hasher_half_baked(entries, h).unwrap())
                    }

                    SliceKeyAnalysisResult::RightHandSubslice(range) => {
                        let h = RightRangeHasher::new(bh, range);
                        MapTypes::RightRange(HashMap::with_hasher_half_baked(entries, h).unwrap())
                    }
                }
            },
        }
    }
}

impl<V, BH> FzStringMap<String, V, BH>
where
    BH: BuildHasher,
{
    /// Creates a frozen map which uses the given hash builder to hash keys.
    #[must_use]
    #[allow(clippy::missing_panics_doc)]
    pub fn with_strings_and_hasher(mut entries: Vec<(String, V)>, bh: BH) -> Self {
        entries.sort_by(|x, y| x.0.cmp(&y.0));
        dedup_by_keep_last(&mut entries, |x, y| x.0.eq(&y.0));

        Self {
            map_impl: {
                match analyze_slice_keys(entries.iter().map(|x| x.0.as_bytes()), &bh) {
                    SliceKeyAnalysisResult::General | SliceKeyAnalysisResult::Length => {
                        let h = BridgeHasher::new(bh);
                        MapTypes::Hash(HashMap::with_hasher_half_baked(entries, h).unwrap())
                    }

                    SliceKeyAnalysisResult::LeftHandSubslice(range) => {
                        let h = LeftRangeHasher::new(bh, range);
                        MapTypes::LeftRange(HashMap::with_hasher_half_baked(entries, h).unwrap())
                    }

                    SliceKeyAnalysisResult::RightHandSubslice(range) => {
                        let h = RightRangeHasher::new(bh, range);
                        MapTypes::RightRange(HashMap::with_hasher_half_baked(entries, h).unwrap())
                    }
                }
            },
        }
    }
}

impl<V, BH> Default for FzStringMap<&str, V, BH>
where
    BH: Default,
{
    fn default() -> Self {
        Self {
            map_impl: MapTypes::Hash(
                HashMap::<&str, V, LargeCollection, BridgeHasher<BH>>::default(),
            ),
        }
    }
}

impl<V, BH> Default for FzStringMap<String, V, BH>
where
    BH: Default,
{
    fn default() -> Self {
        Self {
            map_impl: MapTypes::Hash(
                HashMap::<String, V, LargeCollection, BridgeHasher<BH>>::default(),
            ),
        }
    }
}

impl<'a, V, const N: usize, BH> From<[(&'a str, V); N]> for FzStringMap<&'a str, V, BH>
where
    BH: BuildHasher + Default,
{
    fn from(entries: [(&'a str, V); N]) -> Self {
        Self::with_hasher(Vec::from(entries), BH::default())
    }
}

impl<V, const N: usize, BH> From<[(String, V); N]> for FzStringMap<String, V, BH>
where
    BH: BuildHasher + Default,
{
    fn from(entries: [(String, V); N]) -> Self {
        Self::with_strings_and_hasher(Vec::from(entries), BH::default())
    }
}

impl<'a, V, BH> FromIterator<(&'a str, V)> for FzStringMap<&'a str, V, BH>
where
    BH: BuildHasher + Default,
{
    fn from_iter<T: IntoIterator<Item = (&'a str, V)>>(iter: T) -> Self {
        Self::with_hasher(iter.into_iter().collect(), BH::default())
    }
}

impl<V, BH> FromIterator<(String, V)> for FzStringMap<String, V, BH>
where
    BH: BuildHasher + Default,
{
    fn from_iter<T: IntoIterator<Item = (String, V)>>(iter: T) -> Self {
        Self::with_strings_and_hasher(iter.into_iter().collect(), BH::default())
    }
}

impl<K, V, Q, BH> Map<K, V, Q> for FzStringMap<K, V, BH>
where
    Q: ?Sized + Hash + Eq + Len + Equivalent<K>,
    BH: BuildHasher,
    LeftRangeHasher<BH>: Hasher<Q>,
    RightRangeHasher<BH>: Hasher<Q>,
{
    fn get_disjoint_mut<const N: usize>(&mut self, keys: [&Q; N]) -> [Option<&mut V>; N] {
        match &mut self.map_impl {
            MapTypes::LeftRange(m) => m.get_disjoint_mut(keys),
            MapTypes::RightRange(m) => m.get_disjoint_mut(keys),
            MapTypes::Hash(m) => m.get_disjoint_mut(keys),
        }
    }

    unsafe fn get_disjoint_unchecked_mut<const N: usize>(
        &mut self,
        keys: [&Q; N],
    ) -> [Option<&mut V>; N] {
        unsafe {
            match &mut self.map_impl {
                MapTypes::LeftRange(m) => m.get_disjoint_unchecked_mut(keys),
                MapTypes::RightRange(m) => m.get_disjoint_unchecked_mut(keys),
                MapTypes::Hash(m) => m.get_disjoint_unchecked_mut(keys),
            }
        }
    }
}

impl<K, V, Q, BH> MapQuery<K, V, Q> for FzStringMap<K, V, BH>
where
    Q: ?Sized + Hash + Eq + Len + Equivalent<K>,
    BH: BuildHasher,
    LeftRangeHasher<BH>: Hasher<Q>,
    RightRangeHasher<BH>: Hasher<Q>,
{
    #[inline]
    fn get(&self, key: &Q) -> Option<&V> {
        match &self.map_impl {
            MapTypes::LeftRange(m) => m.get(key),
            MapTypes::RightRange(m) => m.get(key),
            MapTypes::Hash(m) => m.get(key),
        }
    }

    #[inline]
    fn get_key_value(&self, key: &Q) -> Option<(&K, &V)> {
        match &self.map_impl {
            MapTypes::LeftRange(m) => m.get_key_value(key),
            MapTypes::RightRange(m) => m.get_key_value(key),
            MapTypes::Hash(m) => m.get_key_value(key),
        }
    }

    #[inline]
    fn get_mut(&mut self, key: &Q) -> Option<&mut V> {
        match &mut self.map_impl {
            MapTypes::LeftRange(m) => m.get_mut(key),
            MapTypes::RightRange(m) => m.get_mut(key),
            MapTypes::Hash(m) => m.get_mut(key),
        }
    }
}

impl<K, V, BH> MapIteration<K, V> for FzStringMap<K, V, BH> {
    type Iterator<'a>
        = Iter<'a, K, V>
    where
        K: 'a,
        V: 'a,
        BH: 'a;

    type KeyIterator<'a>
        = Keys<'a, K, V>
    where
        K: 'a,
        V: 'a,
        BH: 'a;

    type ValueIterator<'a>
        = Values<'a, K, V>
    where
        K: 'a,
        V: 'a,
        BH: 'a;

    type IntoKeyIterator = IntoKeys<K, V>;
    type IntoValueIterator = IntoValues<K, V>;

    type MutIterator<'a>
        = IterMut<'a, K, V>
    where
        K: 'a,
        V: 'a,
        BH: 'a;

    type ValueMutIterator<'a>
        = ValuesMut<'a, K, V>
    where
        K: 'a,
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

impl<K, V, BH> Len for FzStringMap<K, V, BH> {
    fn len(&self) -> usize {
        match &self.map_impl {
            MapTypes::LeftRange(m) => m.len(),
            MapTypes::RightRange(m) => m.len(),
            MapTypes::Hash(m) => m.len(),
        }
    }
}

impl<K, V, Q, BH> Index<&Q> for FzStringMap<K, V, BH>
where
    Q: ?Sized + Hash + Eq + Len + Equivalent<K>,
    BH: BuildHasher,
    LeftRangeHasher<BH>: Hasher<Q>,
    RightRangeHasher<BH>: Hasher<Q>,
{
    type Output = V;

    fn index(&self, index: &Q) -> &Self::Output {
        self.get(index).expect("index should be valid")
    }
}

impl<'a, K, V, BH> IntoIterator for &'a FzStringMap<K, V, BH> {
    type Item = (&'a K, &'a V);
    type IntoIter = Iter<'a, K, V>;

    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}

impl<'a, K, V, BH> IntoIterator for &'a mut FzStringMap<K, V, BH> {
    type Item = (&'a K, &'a mut V);
    type IntoIter = IterMut<'a, K, V>;

    fn into_iter(self) -> Self::IntoIter {
        self.iter_mut()
    }
}

impl<K, V, BH> IntoIterator for FzStringMap<K, V, BH> {
    type Item = (K, V);
    type IntoIter = IntoIter<K, V>;

    fn into_iter(self) -> Self::IntoIter {
        match self.map_impl {
            MapTypes::LeftRange(m) => m.into_iter(),
            MapTypes::RightRange(m) => m.into_iter(),
            MapTypes::Hash(m) => m.into_iter(),
        }
    }
}

impl<K, V, MT, BH> PartialEq<MT> for FzStringMap<K, V, BH>
where
    K: Hash + Eq + Len + Equivalent<K>,
    V: PartialEq,
    MT: Map<K, V>,
    BH: BuildHasher,
{
    fn eq(&self, other: &MT) -> bool {
        if self.len() != other.len() {
            return false;
        }

        self.iter()
            .all(|(key, value)| other.get(key).is_some_and(|v| *value == *v))
    }
}

impl<K, V, BH> Eq for FzStringMap<K, V, BH>
where
    K: Hash + Eq + Len + Equivalent<K>,
    V: Eq,
    BH: BuildHasher,
    LeftRangeHasher<BH>: Hasher<K>,
    RightRangeHasher<BH>: Hasher<K>,
{
}

impl<K, V, BH> Debug for FzStringMap<K, V, BH>
where
    K: Debug,
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

#[cfg(feature = "serde")]
impl<K, V, BH> Serialize for FzStringMap<K, V, BH>
where
    K: Serialize,
    V: Serialize,
{
    serialize_fn!();
}

#[cfg(feature = "serde")]
impl<'de, V> Deserialize<'de> for FzStringMap<&'de str, V>
where
    V: Deserialize<'de>,
{
    fn deserialize<D>(deserializer: D) -> core::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer.deserialize_map(StrMapVisitor {
            marker: PhantomData,
        })
    }
}

#[cfg(feature = "serde")]
impl<'de, V> Deserialize<'de> for FzStringMap<String, V>
where
    V: Deserialize<'de>,
{
    fn deserialize<D>(deserializer: D) -> core::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer.deserialize_map(StringMapVisitor {
            marker: PhantomData,
        })
    }
}

#[cfg(feature = "serde")]
struct StrMapVisitor<V, BH> {
    marker: PhantomData<(V, BH)>,
}

#[cfg(feature = "serde")]
impl<'de, V, BH> Visitor<'de> for StrMapVisitor<V, BH>
where
    V: Deserialize<'de>,
    BH: BuildHasher + Default,
{
    type Value = FzStringMap<&'de str, V, BH>;

    fn expecting(&self, formatter: &mut Formatter) -> Result {
        formatter.write_str("a map with string keys")
    }

    fn visit_map<M>(self, mut access: M) -> core::result::Result<Self::Value, M::Error>
    where
        M: MapAccess<'de>,
    {
        let mut v = Vec::with_capacity(access.size_hint().unwrap_or(0));
        while let Some(x) = access.next_entry()? {
            v.push(x);
        }

        Ok(FzStringMap::with_hasher(v, BH::default()))
    }
}

#[cfg(feature = "serde")]
struct StringMapVisitor<V, BH> {
    marker: PhantomData<(V, BH)>,
}

#[cfg(feature = "serde")]
impl<'de, V, BH> Visitor<'de> for StringMapVisitor<V, BH>
where
    V: Deserialize<'de>,
    BH: BuildHasher + Default,
{
    type Value = FzStringMap<String, V, BH>;

    fn expecting(&self, formatter: &mut Formatter) -> Result {
        formatter.write_str("a map with string keys")
    }

    fn visit_map<M>(self, mut access: M) -> core::result::Result<Self::Value, M::Error>
    where
        M: MapAccess<'de>,
    {
        let mut v = Vec::with_capacity(access.size_hint().unwrap_or(0));
        while let Some(x) = access.next_entry::<&str, V>()? {
            v.push((::alloc::string::String::from(x.0), x.1));
        }

        Ok(FzStringMap::with_strings_and_hasher(v, BH::default()))
    }
}
