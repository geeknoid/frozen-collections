use crate::DefaultBuildHasher;
use crate::analyzers::{SliceKeyAnalysisResult, analyze_slice_keys};
use crate::hashers::{BridgeHasher, LeftRangeHasher, RightRangeHasher};
use crate::maps::decl_macros::{debug_trait_funcs, index_trait_funcs, len_trait_funcs, map_query_trait_funcs, partial_eq_trait_funcs};
use crate::maps::{HashMap, IntoIter, IntoKeys, IntoValues, Iter, IterMut, Keys, Values, ValuesMut};
use crate::traits::{LargeCollection, Len, Map, MapExtras, MapIteration, MapQuery};
use crate::utils::dedup_by_keep_last;
use core::array;
use core::fmt::{Debug, Formatter, Result};
use core::hash::BuildHasher;
use core::marker::PhantomData;
use core::ops::Index;

#[cfg(not(feature = "std"))]
use {alloc::boxed::Box, alloc::string::ToString, alloc::vec::Vec};

#[cfg(feature = "serde")]
use {
    crate::maps::decl_macros::serialize_trait_funcs,
    serde::de::{MapAccess, Visitor},
    serde::ser::SerializeMap,
    serde::{Deserialize, Deserializer, Serialize, Serializer},
};

#[derive(Clone)]
enum MapTypes<V, BH> {
    LeftRange(HashMap<Box<str>, V, LargeCollection, LeftRangeHasher<BH>>),
    RightRange(HashMap<Box<str>, V, LargeCollection, RightRangeHasher<BH>>),
    Hash(HashMap<Box<str>, V, LargeCollection, BridgeHasher<BH>>),
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
pub struct FzStringMap<K, V, BH = DefaultBuildHasher> {
    map_impl: MapTypes<V, BH>,
    _0: PhantomData<K>,
}

impl<V> FzStringMap<Box<str>, V, DefaultBuildHasher> {
    /// Creates a frozen map.
    #[must_use]
    pub fn new(entries: Vec<(impl AsRef<str>, V)>) -> Self {
        Self::with_hasher(entries, DefaultBuildHasher::default())
    }
}

impl<V, BH> FzStringMap<Box<str>, V, BH> {
    /// Creates a frozen map which uses the given hash builder to hash keys.
    #[must_use]
    #[expect(
        clippy::missing_panics_doc,
        reason = "Guaranteed not to panic because the map is a LargeCollection"
    )]
    pub fn with_hasher(mut entries: Vec<(impl AsRef<str>, V)>, bh: BH) -> Self
    where
        BH: BuildHasher,
    {
        entries.sort_by(|x, y| x.0.as_ref().cmp(y.0.as_ref()));
        dedup_by_keep_last(&mut entries, |x, y| x.0.as_ref().eq(y.0.as_ref()));

        let entries: Vec<(Box<str>, V)> = entries
            .into_iter()
            .map(|(k, v)| (k.as_ref().to_string().into_boxed_str(), v))
            .collect();

        Self {
            map_impl: {
                match analyze_slice_keys(entries.iter().map(|x| x.0.as_ref().as_bytes()), &bh) {
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
            _0: PhantomData,
        }
    }

    #[doc = include_str!("../doc_snippets/get.md")]
    #[inline]
    pub fn get(&self, key: impl AsRef<str>) -> Option<&V>
    where
        BH: BuildHasher,
    {
        let key = key.as_ref();
        match &self.map_impl {
            MapTypes::LeftRange(m) => m.get(key),
            MapTypes::RightRange(m) => m.get(key),
            MapTypes::Hash(m) => m.get(key),
        }
    }

    #[doc = include_str!("../doc_snippets/get_mut.md")]
    #[inline]
    pub fn get_mut(&mut self, key: impl AsRef<str>) -> Option<&mut V>
    where
        BH: BuildHasher,
    {
        let key = key.as_ref();
        match &mut self.map_impl {
            MapTypes::LeftRange(m) => m.get_mut(key),
            MapTypes::RightRange(m) => m.get_mut(key),
            MapTypes::Hash(m) => m.get_mut(key),
        }
    }

    #[doc = include_str!("../doc_snippets/get_key_value.md")]
    #[inline]
    #[expect(clippy::borrowed_box, reason = "By design")]
    pub fn get_key_value(&self, key: impl AsRef<str>) -> Option<(&Box<str>, &V)>
    where
        BH: BuildHasher,
    {
        let key = key.as_ref();
        match &self.map_impl {
            MapTypes::LeftRange(m) => m.get_key_value(key),
            MapTypes::RightRange(m) => m.get_key_value(key),
            MapTypes::Hash(m) => m.get_key_value(key),
        }
    }

    #[doc = include_str!("../doc_snippets/contains_key.md")]
    #[inline]
    #[must_use]
    pub fn contains_key(&self, key: impl AsRef<str>) -> bool
    where
        BH: BuildHasher,
    {
        let key = key.as_ref();
        match &self.map_impl {
            MapTypes::LeftRange(m) => m.contains_key(key),
            MapTypes::RightRange(m) => m.contains_key(key),
            MapTypes::Hash(m) => m.contains_key(key),
        }
    }

    #[doc = include_str!("../doc_snippets/get_disjoint_mut.md")]
    #[expect(clippy::needless_pass_by_value, reason = "By design")]
    pub fn get_disjoint_mut<const N: usize>(&mut self, keys: [impl AsRef<str>; N]) -> [Option<&mut V>; N]
    where
        BH: BuildHasher,
    {
        let keys: [&str; N] = array::from_fn(|i| keys[i].as_ref());
        match &mut self.map_impl {
            MapTypes::LeftRange(m) => m.get_disjoint_mut(keys),
            MapTypes::RightRange(m) => m.get_disjoint_mut(keys),
            MapTypes::Hash(m) => m.get_disjoint_mut(keys),
        }
    }

    #[doc = include_str!("../doc_snippets/get_disjoint_unchecked_mut.md")]
    #[expect(clippy::needless_pass_by_value, reason = "By design")]
    pub unsafe fn get_disjoint_unchecked_mut<const N: usize>(&mut self, keys: [impl AsRef<str>; N]) -> [Option<&mut V>; N]
    where
        BH: BuildHasher,
    {
        let keys: [&str; N] = array::from_fn(|i| keys[i].as_ref());
        match &mut self.map_impl {
            // SAFETY: The caller must ensure that the keys are disjoint.
            MapTypes::LeftRange(m) => unsafe { m.get_disjoint_unchecked_mut(keys) },

            // SAFETY: The caller must ensure that the keys are disjoint.
            MapTypes::RightRange(m) => unsafe { m.get_disjoint_unchecked_mut(keys) },

            // SAFETY: The caller must ensure that the keys are disjoint.
            MapTypes::Hash(m) => unsafe { m.get_disjoint_unchecked_mut(keys) },
        }
    }

    #[doc = include_str!("../doc_snippets/len.md")]
    #[inline]
    #[must_use]
    pub fn len(&self) -> usize {
        match &self.map_impl {
            MapTypes::LeftRange(m) => m.len(),
            MapTypes::RightRange(m) => m.len(),
            MapTypes::Hash(m) => m.len(),
        }
    }

    #[doc = include_str!("../doc_snippets/is_empty.md")]
    #[inline]
    #[must_use]
    pub fn is_empty(&self) -> bool {
        match &self.map_impl {
            MapTypes::LeftRange(m) => m.is_empty(),
            MapTypes::RightRange(m) => m.is_empty(),
            MapTypes::Hash(m) => m.is_empty(),
        }
    }

    #[doc = include_str!("../doc_snippets/iter.md")]
    #[must_use]
    pub fn iter(&self) -> Iter<'_, Box<str>, V> {
        match &self.map_impl {
            MapTypes::LeftRange(m) => m.iter(),
            MapTypes::RightRange(m) => m.iter(),
            MapTypes::Hash(m) => m.iter(),
        }
    }

    #[doc = include_str!("../doc_snippets/iter_mut.md")]
    #[must_use]
    pub fn iter_mut(&mut self) -> IterMut<'_, Box<str>, V> {
        match &mut self.map_impl {
            MapTypes::LeftRange(m) => m.iter_mut(),
            MapTypes::RightRange(m) => m.iter_mut(),
            MapTypes::Hash(m) => m.iter_mut(),
        }
    }

    #[must_use]
    fn into_iter(self) -> IntoIter<Box<str>, V> {
        match self.map_impl {
            MapTypes::LeftRange(m) => m.into_iter(),
            MapTypes::RightRange(m) => m.into_iter(),
            MapTypes::Hash(m) => m.into_iter(),
        }
    }

    #[doc = include_str!("../doc_snippets/keys.md")]
    #[must_use]
    pub fn keys(&self) -> Keys<'_, Box<str>, V> {
        match &self.map_impl {
            MapTypes::LeftRange(m) => m.keys(),
            MapTypes::RightRange(m) => m.keys(),
            MapTypes::Hash(m) => m.keys(),
        }
    }

    #[doc = include_str!("../doc_snippets/into_keys.md")]
    #[must_use]
    pub fn into_keys(self) -> IntoKeys<Box<str>, V> {
        match self.map_impl {
            MapTypes::LeftRange(m) => m.into_keys(),
            MapTypes::RightRange(m) => m.into_keys(),
            MapTypes::Hash(m) => m.into_keys(),
        }
    }

    #[doc = include_str!("../doc_snippets/values.md")]
    #[must_use]
    pub fn values(&self) -> Values<'_, Box<str>, V> {
        match &self.map_impl {
            MapTypes::LeftRange(m) => m.values(),
            MapTypes::RightRange(m) => m.values(),
            MapTypes::Hash(m) => m.values(),
        }
    }

    #[doc = include_str!("../doc_snippets/values_mut.md")]
    #[must_use]
    pub fn values_mut(&mut self) -> ValuesMut<'_, Box<str>, V> {
        match &mut self.map_impl {
            MapTypes::LeftRange(m) => m.values_mut(),
            MapTypes::RightRange(m) => m.values_mut(),
            MapTypes::Hash(m) => m.values_mut(),
        }
    }

    #[doc = include_str!("../doc_snippets/into_values.md")]
    #[must_use]
    pub fn into_values(self) -> IntoValues<Box<str>, V> {
        match self.map_impl {
            MapTypes::LeftRange(m) => m.into_values(),
            MapTypes::RightRange(m) => m.into_values(),
            MapTypes::Hash(m) => m.into_values(),
        }
    }
}

impl<V, BH> Default for FzStringMap<Box<str>, V, BH>
where
    BH: Default,
{
    fn default() -> Self {
        Self {
            map_impl: MapTypes::Hash(HashMap::default()),
            _0: PhantomData,
        }
    }
}

impl<K, V, const N: usize, BH> From<[(K, V); N]> for FzStringMap<Box<str>, V, BH>
where
    K: AsRef<str>,
    BH: BuildHasher + Default,
{
    fn from(entries: [(K, V); N]) -> Self {
        Self::with_hasher(Vec::from(entries), BH::default())
    }
}

impl<K, V, BH> FromIterator<(K, V)> for FzStringMap<Box<str>, V, BH>
where
    K: AsRef<str>,
    BH: BuildHasher + Default,
{
    fn from_iter<T: IntoIterator<Item = (K, V)>>(iter: T) -> Self {
        Self::with_hasher(iter.into_iter().collect(), BH::default())
    }
}

impl<V, Q, BH> Map<Box<str>, V, Q> for FzStringMap<Box<str>, V, BH>
where
    Q: AsRef<str>,
    BH: BuildHasher,
{
}

impl<V, Q, BH> MapExtras<Box<str>, V, Q> for FzStringMap<Box<str>, V, BH>
where
    Q: AsRef<str>,
    BH: BuildHasher,
{
    fn get_key_value(&self, key: &Q) -> Option<(&Box<str>, &V)> {
        self.get_key_value(key)
    }

    fn get_disjoint_mut<const N: usize>(&mut self, keys: [&Q; N]) -> [Option<&mut V>; N]
    where
        Q: Eq,
    {
        self.get_disjoint_mut(keys)
    }

    unsafe fn get_disjoint_unchecked_mut<const N: usize>(&mut self, keys: [&Q; N]) -> [Option<&mut V>; N] {
        // SAFETY: The caller must ensure that the keys are disjoint.
        unsafe { self.get_disjoint_unchecked_mut(keys) }
    }
}

impl<V, Q, BH> MapQuery<Q, V> for FzStringMap<Box<str>, V, BH>
where
    Q: AsRef<str>,
    BH: BuildHasher,
{
    map_query_trait_funcs!();
}

impl<V, BH> MapIteration<Box<str>, V> for FzStringMap<Box<str>, V, BH>
where
    BH: BuildHasher,
{
    type Iterator<'a>
        = Iter<'a, Box<str>, V>
    where
        V: 'a,
        BH: 'a;

    type KeyIterator<'a>
        = Keys<'a, Box<str>, V>
    where
        V: 'a,
        BH: 'a;

    type ValueIterator<'a>
        = Values<'a, Box<str>, V>
    where
        V: 'a,
        BH: 'a;

    type IntoKeyIterator = IntoKeys<Box<str>, V>;
    type IntoValueIterator = IntoValues<Box<str>, V>;

    type MutIterator<'a>
        = IterMut<'a, Box<str>, V>
    where
        V: 'a,
        BH: 'a;
    type ValueMutIterator<'a>
        = ValuesMut<'a, Box<str>, V>
    where
        V: 'a,
        BH: 'a;

    fn iter(&self) -> Self::Iterator<'_> {
        self.iter()
    }

    fn iter_mut(&mut self) -> Self::MutIterator<'_> {
        self.iter_mut()
    }

    fn keys(&self) -> Self::KeyIterator<'_> {
        self.keys()
    }

    fn into_keys(self) -> Self::IntoKeyIterator {
        self.into_keys()
    }

    fn values(&self) -> Self::ValueIterator<'_> {
        self.values()
    }

    fn values_mut(&mut self) -> Self::ValueMutIterator<'_> {
        self.values_mut()
    }

    fn into_values(self) -> Self::IntoValueIterator {
        self.into_values()
    }
}

impl<V, BH> Len for FzStringMap<Box<str>, V, BH> {
    len_trait_funcs!();
}

impl<V, Q, BH> Index<&Q> for FzStringMap<Box<str>, V, BH>
where
    Q: AsRef<str>,
    BH: BuildHasher,
{
    index_trait_funcs!();
}

impl<V, BH> IntoIterator for FzStringMap<Box<str>, V, BH> {
    type Item = (Box<str>, V);
    type IntoIter = IntoIter<Box<str>, V>;

    fn into_iter(self) -> Self::IntoIter {
        self.into_iter()
    }
}

impl<'a, V, BH> IntoIterator for &'a FzStringMap<Box<str>, V, BH> {
    type Item = (&'a Box<str>, &'a V);
    type IntoIter = Iter<'a, Box<str>, V>;

    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}

impl<'a, V, BH> IntoIterator for &'a mut FzStringMap<Box<str>, V, BH> {
    type Item = (&'a Box<str>, &'a mut V);
    type IntoIter = IterMut<'a, Box<str>, V>;

    fn into_iter(self) -> Self::IntoIter {
        self.iter_mut()
    }
}

impl<V, MT, BH> PartialEq<MT> for FzStringMap<Box<str>, V, BH>
where
    V: PartialEq,
    MT: MapQuery<Box<str>, V>,
    BH: BuildHasher,
{
    partial_eq_trait_funcs!();
}

impl<V, BH> Eq for FzStringMap<Box<str>, V, BH>
where
    V: Eq,
    BH: BuildHasher,
{
}

impl<V, BH> Debug for FzStringMap<Box<str>, V, BH>
where
    V: Debug,
{
    debug_trait_funcs!();
}

#[cfg(feature = "serde")]
impl<V, BH> Serialize for FzStringMap<Box<str>, V, BH>
where
    V: Serialize,
{
    serialize_trait_funcs!();
}

#[cfg(feature = "serde")]
impl<'de, V> Deserialize<'de> for FzStringMap<Box<str>, V>
where
    V: Deserialize<'de>,
{
    fn deserialize<D>(deserializer: D) -> core::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer.deserialize_map(StrMapVisitor { marker: PhantomData })
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
    type Value = FzStringMap<Box<str>, V, BH>;

    fn expecting(&self, formatter: &mut Formatter) -> Result {
        formatter.write_str("a map with string keys")
    }

    fn visit_map<M>(self, mut map: M) -> core::result::Result<Self::Value, M::Error>
    where
        M: MapAccess<'de>,
    {
        let mut v: Vec<(&'de str, _)> = Vec::with_capacity(map.size_hint().unwrap_or(0));
        while let Some(x) = map.next_entry()? {
            v.push(x);
        }

        Ok(FzStringMap::with_hasher(v, BH::default()))
    }
}
