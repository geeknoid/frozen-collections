use crate::DefaultBuildHasher;
use crate::analyzers::{SliceKeyAnalysisResult, analyze_slice_keys};
use crate::hashers::{BridgeHasher, LeftRangeHasher, LengthHasher, RightRangeHasher, StringHasher};
use crate::maps::HashMap;
use crate::maps::decl_macros::{debug_trait_funcs, index_trait_funcs, len_trait_funcs, map_query_trait_funcs, partial_eq_trait_funcs};
use crate::maps::{IntoIter, IntoKeys, IntoValues, Iter, IterMut, Keys, Values, ValuesMut};
use crate::traits::{LargeCollection, Len, Map, MapExtras, MapIteration, MapQuery};
use crate::utils::DeduppedVec;
use core::array;
use core::fmt::{Debug, Formatter, Result};
use core::hash::BuildHasher;
use core::marker::PhantomData;
use core::ops::Index;

#[cfg(not(feature = "std"))]
use {alloc::boxed::Box, alloc::vec::Vec};

#[cfg(feature = "serde")]
use {
    crate::maps::decl_macros::serialize_trait_funcs,
    serde::de::{MapAccess, Visitor},
    serde::ser::SerializeMap,
    serde::{Deserialize, Deserializer, Serialize, Serializer},
};

/// A map optimized for fast read access with string keys.
///
#[doc = include_str!("../doc_snippets/about.md")]
#[doc = include_str!("../doc_snippets/hash_warning.md")]
#[derive(Clone)]
pub struct FzStringMap<V, BH = DefaultBuildHasher> {
    map_impl: HashMap<Box<str>, V, LargeCollection, StringHasher>,
    _bh: PhantomData<BH>,
}

impl<V> FzStringMap<V, DefaultBuildHasher> {
    /// Creates a frozen map.
    #[doc = include_str!("../doc_snippets/duplicate_keys.md")]
    #[must_use]
    pub fn new(entries: Vec<(impl AsRef<str>, V)>) -> Self {
        Self::with_hasher(entries, DefaultBuildHasher::default())
    }
}

impl<V, BH> FzStringMap<V, BH> {
    /// Creates a frozen map which uses the given hash builder to hash keys.
    #[doc = include_str!("../doc_snippets/duplicate_keys.md")]
    #[must_use]
    #[expect(
        clippy::missing_panics_doc,
        reason = "Guaranteed not to panic because the map is a LargeCollection"
    )]
    pub fn with_hasher(entries: Vec<(impl AsRef<str>, V)>, bh: BH) -> Self
    where
        BH: BuildHasher + Clone + Send + Sync + 'static,
    {
        let entries: Vec<(Box<str>, V)> = entries.into_iter().map(|(k, v)| (Box::from(k.as_ref()), v)).collect();

        let entries = DeduppedVec::using_cmp(entries, |x, y| x.0.cmp(&y.0));

        let hasher = match analyze_slice_keys(entries.iter().map(|x| x.0.as_bytes()), &bh) {
            SliceKeyAnalysisResult::General => StringHasher::new(BridgeHasher::new(bh)),
            SliceKeyAnalysisResult::Length => StringHasher::new(LengthHasher),
            SliceKeyAnalysisResult::LeftHandSubslice(range) => StringHasher::new(LeftRangeHasher::new(bh, range)),
            SliceKeyAnalysisResult::RightHandSubslice(range) => StringHasher::new(RightRangeHasher::new(bh, range)),
        };

        Self {
            map_impl: HashMap::from_dedupped(entries, hasher).expect("failed to create hash map"),
            _bh: PhantomData,
        }
    }

    #[doc = include_str!("../doc_snippets/get.md")]
    #[inline]
    pub fn get(&self, key: impl AsRef<str>) -> Option<&V>
    where
        BH: BuildHasher,
    {
        self.map_impl.get(key.as_ref())
    }

    #[doc = include_str!("../doc_snippets/get_mut.md")]
    #[inline]
    pub fn get_mut(&mut self, key: impl AsRef<str>) -> Option<&mut V>
    where
        BH: BuildHasher,
    {
        self.map_impl.get_mut(key.as_ref())
    }

    #[doc = include_str!("../doc_snippets/get_key_value.md")]
    #[inline]
    #[expect(clippy::borrowed_box, reason = "Key type is Box<str>")]
    pub fn get_key_value(&self, key: impl AsRef<str>) -> Option<(&Box<str>, &V)>
    where
        BH: BuildHasher,
    {
        self.map_impl.get_key_value(key.as_ref())
    }

    #[doc = include_str!("../doc_snippets/contains_key.md")]
    #[inline]
    #[must_use]
    pub fn contains_key(&self, key: impl AsRef<str>) -> bool
    where
        BH: BuildHasher,
    {
        self.map_impl.contains_key(key.as_ref())
    }

    #[doc = include_str!("../doc_snippets/get_disjoint_mut.md")]
    #[expect(clippy::needless_pass_by_value, reason = "By design")]
    pub fn get_disjoint_mut<const N: usize>(&mut self, keys: [impl AsRef<str>; N]) -> [Option<&mut V>; N]
    where
        BH: BuildHasher,
    {
        let keys: [&str; N] = array::from_fn(|i| keys[i].as_ref());
        self.map_impl.get_disjoint_mut(keys)
    }

    #[doc = include_str!("../doc_snippets/get_disjoint_unchecked_mut.md")]
    #[expect(clippy::needless_pass_by_value, reason = "By design")]
    pub unsafe fn get_disjoint_unchecked_mut<const N: usize>(&mut self, keys: [impl AsRef<str>; N]) -> [Option<&mut V>; N]
    where
        BH: BuildHasher,
    {
        let keys: [&str; N] = array::from_fn(|i| keys[i].as_ref());
        // SAFETY: The caller must ensure that the keys are disjoint.
        unsafe { self.map_impl.get_disjoint_unchecked_mut(keys) }
    }

    #[doc = include_str!("../doc_snippets/len.md")]
    #[inline]
    #[must_use]
    pub fn len(&self) -> usize {
        self.map_impl.len()
    }

    #[doc = include_str!("../doc_snippets/is_empty.md")]
    #[inline]
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.map_impl.is_empty()
    }

    #[doc = include_str!("../doc_snippets/iter.md")]
    #[must_use]
    pub fn iter(&self) -> Iter<'_, Box<str>, V> {
        self.map_impl.iter()
    }

    #[doc = include_str!("../doc_snippets/iter_mut.md")]
    #[must_use]
    pub fn iter_mut(&mut self) -> IterMut<'_, Box<str>, V> {
        self.map_impl.iter_mut()
    }

    #[must_use]
    fn into_iter(self) -> IntoIter<Box<str>, V> {
        self.map_impl.into_iter()
    }

    #[doc = include_str!("../doc_snippets/keys.md")]
    #[must_use]
    pub fn keys(&self) -> Keys<'_, Box<str>, V> {
        self.map_impl.keys()
    }

    #[doc = include_str!("../doc_snippets/into_keys.md")]
    #[must_use]
    pub fn into_keys(self) -> IntoKeys<Box<str>, V> {
        self.map_impl.into_keys()
    }

    #[doc = include_str!("../doc_snippets/values.md")]
    #[must_use]
    pub fn values(&self) -> Values<'_, Box<str>, V> {
        self.map_impl.values()
    }

    #[doc = include_str!("../doc_snippets/values_mut.md")]
    #[must_use]
    pub fn values_mut(&mut self) -> ValuesMut<'_, Box<str>, V> {
        self.map_impl.values_mut()
    }

    #[doc = include_str!("../doc_snippets/into_values.md")]
    #[must_use]
    pub fn into_values(self) -> IntoValues<Box<str>, V> {
        self.map_impl.into_values()
    }
}

impl<V, BH> Default for FzStringMap<V, BH>
where
    BH: Default,
{
    fn default() -> Self {
        Self {
            map_impl: HashMap::default(),
            _bh: PhantomData,
        }
    }
}

impl<K, V, const N: usize, BH> From<[(K, V); N]> for FzStringMap<V, BH>
where
    K: AsRef<str>,
    BH: BuildHasher + Default + Clone + Send + Sync + 'static,
{
    fn from(entries: [(K, V); N]) -> Self {
        Self::with_hasher(Vec::from(entries), BH::default())
    }
}

impl<K, V, BH> FromIterator<(K, V)> for FzStringMap<V, BH>
where
    K: AsRef<str>,
    BH: BuildHasher + Default + Clone + Send + Sync + 'static,
{
    fn from_iter<T: IntoIterator<Item = (K, V)>>(iter: T) -> Self {
        Self::with_hasher(iter.into_iter().collect(), BH::default())
    }
}

impl<V, Q, BH> Map<Box<str>, V, Q> for FzStringMap<V, BH>
where
    Q: AsRef<str>,
    BH: BuildHasher,
{
}

impl<V, Q, BH> MapExtras<Box<str>, V, Q> for FzStringMap<V, BH>
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

impl<V, Q, BH> MapQuery<Q, V> for FzStringMap<V, BH>
where
    Q: AsRef<str>,
    BH: BuildHasher,
{
    map_query_trait_funcs!();
}

impl<V, BH> MapIteration<Box<str>, V> for FzStringMap<V, BH>
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

impl<V, BH> Len for FzStringMap<V, BH> {
    len_trait_funcs!();
}

impl<V, Q, BH> Index<&Q> for FzStringMap<V, BH>
where
    Q: AsRef<str>,
    BH: BuildHasher,
{
    index_trait_funcs!();
}

impl<V, BH> IntoIterator for FzStringMap<V, BH> {
    type Item = (Box<str>, V);
    type IntoIter = IntoIter<Box<str>, V>;

    fn into_iter(self) -> Self::IntoIter {
        self.into_iter()
    }
}

impl<'a, V, BH> IntoIterator for &'a FzStringMap<V, BH> {
    type Item = (&'a Box<str>, &'a V);
    type IntoIter = Iter<'a, Box<str>, V>;

    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}

impl<'a, V, BH> IntoIterator for &'a mut FzStringMap<V, BH> {
    type Item = (&'a Box<str>, &'a mut V);
    type IntoIter = IterMut<'a, Box<str>, V>;

    fn into_iter(self) -> Self::IntoIter {
        self.iter_mut()
    }
}

impl<V, MT, BH> PartialEq<MT> for FzStringMap<V, BH>
where
    V: PartialEq,
    MT: MapQuery<Box<str>, V>,
    BH: BuildHasher,
{
    partial_eq_trait_funcs!();
}

impl<V, BH> Eq for FzStringMap<V, BH>
where
    V: Eq,
    BH: BuildHasher,
{
}

impl<V, BH> Debug for FzStringMap<V, BH>
where
    V: Debug,
{
    debug_trait_funcs!();
}

#[cfg(feature = "serde")]
impl<V, BH> Serialize for FzStringMap<V, BH>
where
    V: Serialize,
{
    serialize_trait_funcs!();
}

#[cfg(feature = "serde")]
impl<'de, V, BH> Deserialize<'de> for FzStringMap<V, BH>
where
    V: Deserialize<'de>,
    BH: BuildHasher + Default + Clone + Send + Sync + 'static,
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
    BH: BuildHasher + Default + Clone + Send + Sync + 'static,
{
    type Value = FzStringMap<V, BH>;

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
