use crate::DefaultBuildHasher;
use crate::analyzers::{SliceKeyAnalysisResult, analyze_slice_keys};
use crate::hashers::{BridgeHasher, LeftRangeHasher, LengthHasher, RightRangeHasher};
use crate::maps::HashMap;
use crate::maps::decl_macros::{
    debug_trait_funcs, index_trait_funcs, len_trait_funcs, map_iteration_trait_funcs, map_query_trait_funcs, partial_eq_trait_funcs,
};
use crate::maps::{IntoIter, IntoKeys, IntoValues, Iter, IterMut, Keys, Values, ValuesMut};
use crate::traits::{CollectionMagnitude, LargeCollection, Len, Map, MapExtras, MapIteration, MapQuery, MediumCollection, SmallCollection};
use crate::utils::DeduppedVec;
use core::array;
use core::fmt::{Debug, Formatter, Result};
use core::hash::{BuildHasher, Hash};
use core::ops::Index;
use equivalent::Equivalent;

#[cfg(not(feature = "std"))]
use {alloc::boxed::Box, alloc::vec::Vec};

#[cfg(feature = "compact_str")]
use compact_str::CompactString;

#[cfg(feature = "serde")]
use {
    crate::maps::decl_macros::serialize_trait_funcs,
    core::marker::PhantomData,
    serde::de::{MapAccess, Visitor},
    serde::ser::SerializeMap,
    serde::{Deserialize, Deserializer, Serialize, Serializer},
};

#[derive(Clone)]
enum MapTypes<K, V, BH> {
    BridgeSmall(HashMap<K, V, SmallCollection, BridgeHasher<BH>>),
    LengthSmall(HashMap<K, V, SmallCollection, LengthHasher>),
    LeftRangeSmall(HashMap<K, V, SmallCollection, LeftRangeHasher<BH>>),
    RightRangeSmall(HashMap<K, V, SmallCollection, RightRangeHasher<BH>>),

    BridgeMedium(HashMap<K, V, MediumCollection, BridgeHasher<BH>>),
    LengthMedium(HashMap<K, V, MediumCollection, LengthHasher>),
    LeftRangeMedium(HashMap<K, V, MediumCollection, LeftRangeHasher<BH>>),
    RightRangeMedium(HashMap<K, V, MediumCollection, RightRangeHasher<BH>>),

    BridgeLarge(HashMap<K, V, LargeCollection, BridgeHasher<BH>>),
    LengthLarge(HashMap<K, V, LargeCollection, LengthHasher>),
    LeftRangeLarge(HashMap<K, V, LargeCollection, LeftRangeHasher<BH>>),
    RightRangeLarge(HashMap<K, V, LargeCollection, RightRangeHasher<BH>>),
}

macro_rules! match_map_types {
    ($self:expr, $m:ident => $body:expr) => {
        match $self {
            MapTypes::BridgeSmall($m) => $body,
            MapTypes::BridgeMedium($m) => $body,
            MapTypes::BridgeLarge($m) => $body,
            MapTypes::LengthSmall($m) => $body,
            MapTypes::LengthMedium($m) => $body,
            MapTypes::LengthLarge($m) => $body,
            MapTypes::LeftRangeSmall($m) => $body,
            MapTypes::LeftRangeMedium($m) => $body,
            MapTypes::LeftRangeLarge($m) => $body,
            MapTypes::RightRangeSmall($m) => $body,
            MapTypes::RightRangeMedium($m) => $body,
            MapTypes::RightRangeLarge($m) => $body,
        }
    };
}

/// A map optimized for fast read access with string keys.
///
#[doc = include_str!("../doc_snippets/about.md")]
#[doc = include_str!("../doc_snippets/hash_warning.md")]
#[derive(Clone)]
pub struct FzStringMap<K, V, BH = DefaultBuildHasher> {
    map_impl: MapTypes<K, V, BH>,
}

enum Magnitude {
    Small,
    Medium,
    Large,
}

fn make_map<K, V, BH>(
    entries: DeduppedVec<(K, V)>,
    analysis: SliceKeyAnalysisResult,
    bh: BH,
) -> MapTypes<K, V, BH>
where
    BH: BuildHasher,
    K: AsRef<str> + Hash + Len,
{
    let magnitude = if entries.len() <= SmallCollection::MAX_CAPACITY {
        Magnitude::Small
    } else if entries.len() <= MediumCollection::MAX_CAPACITY {
        Magnitude::Medium
    } else {
        Magnitude::Large
    };

    match (analysis, magnitude) {
        (SliceKeyAnalysisResult::General, Magnitude::Small) => {
            MapTypes::BridgeSmall(HashMap::from_dedupped(entries, BridgeHasher::new(bh)).expect("failed to create hash map"))
        }
        (SliceKeyAnalysisResult::General, Magnitude::Medium) => {
            MapTypes::BridgeMedium(HashMap::from_dedupped(entries, BridgeHasher::new(bh)).expect("failed to create hash map"))
        }
        (SliceKeyAnalysisResult::General, Magnitude::Large) => {
            MapTypes::BridgeLarge(HashMap::from_dedupped(entries, BridgeHasher::new(bh)).expect("failed to create hash map"))
        }
        (SliceKeyAnalysisResult::Length, Magnitude::Small) => {
            MapTypes::LengthSmall(HashMap::from_dedupped(entries, LengthHasher).expect("failed to create hash map"))
        }
        (SliceKeyAnalysisResult::Length, Magnitude::Medium) => {
            MapTypes::LengthMedium(HashMap::from_dedupped(entries, LengthHasher).expect("failed to create hash map"))
        }
        (SliceKeyAnalysisResult::Length, Magnitude::Large) => {
            MapTypes::LengthLarge(HashMap::from_dedupped(entries, LengthHasher).expect("failed to create hash map"))
        }
        (SliceKeyAnalysisResult::LeftHandSubslice(range), Magnitude::Small) => {
            MapTypes::LeftRangeSmall(
                HashMap::from_dedupped(entries, LeftRangeHasher::new(bh, range)).expect("failed to create hash map"),
            )
        }
        (SliceKeyAnalysisResult::LeftHandSubslice(range), Magnitude::Medium) => {
            MapTypes::LeftRangeMedium(
                HashMap::from_dedupped(entries, LeftRangeHasher::new(bh, range)).expect("failed to create hash map"),
            )
        }
        (SliceKeyAnalysisResult::LeftHandSubslice(range), Magnitude::Large) => {
            MapTypes::LeftRangeLarge(
                HashMap::from_dedupped(entries, LeftRangeHasher::new(bh, range)).expect("failed to create hash map"),
            )
        }
        (SliceKeyAnalysisResult::RightHandSubslice(range), Magnitude::Small) => {
            MapTypes::RightRangeSmall(
                HashMap::from_dedupped(entries, RightRangeHasher::new(bh, range)).expect("failed to create hash map"),
            )
        }
        (SliceKeyAnalysisResult::RightHandSubslice(range), Magnitude::Medium) => {
            MapTypes::RightRangeMedium(
                HashMap::from_dedupped(entries, RightRangeHasher::new(bh, range)).expect("failed to create hash map"),
            )
        }
        (SliceKeyAnalysisResult::RightHandSubslice(range), Magnitude::Large) => {
            MapTypes::RightRangeLarge(
                HashMap::from_dedupped(entries, RightRangeHasher::new(bh, range)).expect("failed to create hash map"),
            )
        }
    }
}

impl<'a, V> FzStringMap<&'a str, V, DefaultBuildHasher> {
    /// Creates a frozen map.
    #[doc = include_str!("../doc_snippets/duplicate_keys.md")]
    #[must_use]
    pub fn new_for_str(entries: Vec<(&'a str, V)>) -> Self {
        Self::with_hasher_for_str(entries, DefaultBuildHasher::default())
    }
}

impl<'a, V, BH> FzStringMap<&'a str, V, BH> {
    /// Creates a frozen map which uses the given hash builder to hash keys.
    #[doc = include_str!("../doc_snippets/duplicate_keys.md")]
    #[must_use]
    pub fn with_hasher_for_str(entries: Vec<(&'a str, V)>, bh: BH) -> Self
    where
        BH: BuildHasher + Clone + Send + Sync + 'static,
    {
        let entries = DeduppedVec::using_cmp(entries, |x, y| x.0.cmp(y.0));

        let analysis = analyze_slice_keys(entries.iter().map(|x| x.0.as_bytes()), &bh);

        Self {
            map_impl: make_map(entries, analysis, bh),
        }
    }
}

impl<V> FzStringMap<Box<str>, V, DefaultBuildHasher> {
    /// Creates a frozen map.
    #[doc = include_str!("../doc_snippets/duplicate_keys.md")]
    #[must_use]
    pub fn new(entries: Vec<(impl AsRef<str>, V)>) -> Self {
        Self::with_hasher(entries, DefaultBuildHasher::default())
    }
}

impl<V, BH> FzStringMap<Box<str>, V, BH> {
    /// Creates a frozen map which uses the given hash builder to hash keys.
    #[doc = include_str!("../doc_snippets/duplicate_keys.md")]
    #[must_use]
    pub fn with_hasher(entries: Vec<(impl AsRef<str>, V)>, bh: BH) -> Self
    where
        BH: BuildHasher + Clone + Send + Sync + 'static,
    {
        let entries: Vec<(Box<str>, V)> = entries.into_iter().map(|(k, v)| (Box::from(k.as_ref()), v)).collect();

        let entries = DeduppedVec::using_cmp(entries, |x, y| x.0.cmp(&y.0));

        let analysis = analyze_slice_keys(entries.iter().map(|x| x.0.as_bytes()), &bh);

        Self {
            map_impl: make_map(entries, analysis, bh),
        }
    }
}

#[cfg(feature = "compact_str")]
impl<V> FzStringMap<CompactString, V, DefaultBuildHasher> {
    /// Creates a frozen map with `CompactString` keys.
    #[doc = include_str!("../doc_snippets/duplicate_keys.md")]
    #[must_use]
    pub fn new_compact(entries: Vec<(impl AsRef<str>, V)>) -> Self {
        Self::with_hasher_compact(entries, DefaultBuildHasher::default())
    }
}

#[cfg(feature = "compact_str")]
impl<V, BH> FzStringMap<CompactString, V, BH> {
    /// Creates a frozen map with `CompactString` keys which uses the given hash builder to hash keys.
    #[doc = include_str!("../doc_snippets/duplicate_keys.md")]
    #[must_use]
    pub fn with_hasher_compact(entries: Vec<(impl AsRef<str>, V)>, bh: BH) -> Self
    where
        BH: BuildHasher + Clone + Send + Sync + 'static,
    {
        let entries: Vec<(CompactString, V)> = entries
            .into_iter()
            .map(|(k, v)| {
                let mut cs = CompactString::from(k.as_ref());
                cs.shrink_to_fit();
                (cs, v)
            })
            .collect();

        let entries = DeduppedVec::using_cmp(entries, |x, y| x.0.cmp(&y.0));

        let analysis = analyze_slice_keys(entries.iter().map(|x| x.0.as_bytes()), &bh);

        Self {
            map_impl: make_map(entries, analysis, bh),
        }
    }
}

impl<K, V, BH> FzStringMap<K, V, BH> {
    #[doc = include_str!("../doc_snippets/get.md")]
    #[inline]
    pub fn get(&self, key: impl AsRef<str>) -> Option<&V>
    where
        BH: BuildHasher,
        str: Equivalent<K>,
    {
        match_map_types!(&self.map_impl, m => m.get(key.as_ref()))
    }

    #[doc = include_str!("../doc_snippets/get_mut.md")]
    #[inline]
    pub fn get_mut(&mut self, key: impl AsRef<str>) -> Option<&mut V>
    where
        BH: BuildHasher,
        str: Equivalent<K>,
    {
        match_map_types!(&mut self.map_impl, m => m.get_mut(key.as_ref()))
    }

    #[doc = include_str!("../doc_snippets/get_key_value.md")]
    #[inline]
    pub fn get_key_value(&self, key: impl AsRef<str>) -> Option<(&K, &V)>
    where
        BH: BuildHasher,
        str: Equivalent<K>,
    {
        match_map_types!(&self.map_impl, m => m.get_key_value(key.as_ref()))
    }

    #[doc = include_str!("../doc_snippets/contains_key.md")]
    #[inline]
    #[must_use]
    pub fn contains_key(&self, key: impl AsRef<str>) -> bool
    where
        BH: BuildHasher,
        str: Equivalent<K>,
    {
        match_map_types!(&self.map_impl, m => m.contains_key(key.as_ref()))
    }

    #[doc = include_str!("../doc_snippets/get_disjoint_mut.md")]
    #[expect(clippy::needless_pass_by_value, reason = "By design")]
    pub fn get_disjoint_mut<const N: usize>(&mut self, keys: [impl AsRef<str>; N]) -> [Option<&mut V>; N]
    where
        BH: BuildHasher,
        str: Equivalent<K>,
    {
        let keys: [&str; N] = array::from_fn(|i| keys[i].as_ref());
        match_map_types!(&mut self.map_impl, m => m.get_disjoint_mut(keys))
    }

    #[doc = include_str!("../doc_snippets/get_disjoint_unchecked_mut.md")]
    #[expect(clippy::needless_pass_by_value, reason = "By design")]
    pub unsafe fn get_disjoint_unchecked_mut<const N: usize>(&mut self, keys: [impl AsRef<str>; N]) -> [Option<&mut V>; N]
    where
        BH: BuildHasher,
        str: Equivalent<K>,
    {
        let keys: [&str; N] = array::from_fn(|i| keys[i].as_ref());
        // SAFETY: The caller must ensure that the keys are disjoint.
        match_map_types!(&mut self.map_impl, m => unsafe { m.get_disjoint_unchecked_mut(keys) })
    }

    #[doc = include_str!("../doc_snippets/len.md")]
    #[inline]
    #[must_use]
    pub fn len(&self) -> usize {
        match_map_types!(&self.map_impl, m => m.len())
    }

    #[doc = include_str!("../doc_snippets/is_empty.md")]
    #[inline]
    #[must_use]
    pub fn is_empty(&self) -> bool {
        match_map_types!(&self.map_impl, m => m.is_empty())
    }

    #[doc = include_str!("../doc_snippets/iter.md")]
    #[must_use]
    pub fn iter(&self) -> Iter<'_, K, V> {
        match_map_types!(&self.map_impl, m => m.iter())
    }

    #[doc = include_str!("../doc_snippets/iter_mut.md")]
    #[must_use]
    pub fn iter_mut(&mut self) -> IterMut<'_, K, V> {
        match_map_types!(&mut self.map_impl, m => m.iter_mut())
    }

    #[must_use]
    fn into_iter(self) -> IntoIter<K, V> {
        match_map_types!(self.map_impl, m => m.into_iter())
    }

    #[doc = include_str!("../doc_snippets/keys.md")]
    #[must_use]
    pub fn keys(&self) -> Keys<'_, K, V> {
        match_map_types!(&self.map_impl, m => m.keys())
    }

    #[doc = include_str!("../doc_snippets/into_keys.md")]
    #[must_use]
    pub fn into_keys(self) -> IntoKeys<K, V> {
        match_map_types!(self.map_impl, m => m.into_keys())
    }

    #[doc = include_str!("../doc_snippets/values.md")]
    #[must_use]
    pub fn values(&self) -> Values<'_, K, V> {
        match_map_types!(&self.map_impl, m => m.values())
    }

    #[doc = include_str!("../doc_snippets/values_mut.md")]
    #[must_use]
    pub fn values_mut(&mut self) -> ValuesMut<'_, K, V> {
        match_map_types!(&mut self.map_impl, m => m.values_mut())
    }

    #[doc = include_str!("../doc_snippets/into_values.md")]
    #[must_use]
    pub fn into_values(self) -> IntoValues<K, V> {
        match_map_types!(self.map_impl, m => m.into_values())
    }
}

impl<V, BH> Default for FzStringMap<Box<str>, V, BH>
where
    BH: Default,
{
    fn default() -> Self {
        Self {
            map_impl: MapTypes::BridgeSmall(HashMap::default()),
        }
    }
}

impl<V, BH> Default for FzStringMap<&str, V, BH>
where
    BH: Default,
{
    fn default() -> Self {
        Self {
            map_impl: MapTypes::BridgeSmall(HashMap::default()),
        }
    }
}

#[cfg(feature = "compact_str")]
impl<V, BH> Default for FzStringMap<CompactString, V, BH>
where
    BH: Default,
{
    fn default() -> Self {
        Self {
            map_impl: MapTypes::BridgeSmall(HashMap::default()),
        }
    }
}

impl<KI, V, const N: usize, BH> From<[(KI, V); N]> for FzStringMap<Box<str>, V, BH>
where
    KI: AsRef<str>,
    BH: BuildHasher + Default + Clone + Send + Sync + 'static,
{
    fn from(entries: [(KI, V); N]) -> Self {
        Self::with_hasher(Vec::from(entries), BH::default())
    }
}

impl<'a, V, const N: usize, BH> From<[(&'a str, V); N]> for FzStringMap<&'a str, V, BH>
where
    BH: BuildHasher + Default + Clone + Send + Sync + 'static,
{
    fn from(entries: [(&'a str, V); N]) -> Self {
        Self::with_hasher_for_str(Vec::from(entries), BH::default())
    }
}

#[cfg(feature = "compact_str")]
impl<KI, V, const N: usize, BH> From<[(KI, V); N]> for FzStringMap<CompactString, V, BH>
where
    KI: AsRef<str>,
    BH: BuildHasher + Default + Clone + Send + Sync + 'static,
{
    fn from(entries: [(KI, V); N]) -> Self {
        Self::with_hasher_compact(Vec::from(entries), BH::default())
    }
}

impl<KI, V, BH> FromIterator<(KI, V)> for FzStringMap<Box<str>, V, BH>
where
    KI: AsRef<str>,
    BH: BuildHasher + Default + Clone + Send + Sync + 'static,
{
    fn from_iter<T: IntoIterator<Item = (KI, V)>>(iter: T) -> Self {
        Self::with_hasher(iter.into_iter().collect(), BH::default())
    }
}

impl<'a, V, BH> FromIterator<(&'a str, V)> for FzStringMap<&'a str, V, BH>
where
    BH: BuildHasher + Default + Clone + Send + Sync + 'static,
{
    fn from_iter<T: IntoIterator<Item = (&'a str, V)>>(iter: T) -> Self {
        Self::with_hasher_for_str(iter.into_iter().collect(), BH::default())
    }
}

#[cfg(feature = "compact_str")]
impl<KI, V, BH> FromIterator<(KI, V)> for FzStringMap<CompactString, V, BH>
where
    KI: Into<CompactString>,
    BH: BuildHasher + Default + Clone + Send + Sync + 'static,
{
    fn from_iter<T: IntoIterator<Item = (KI, V)>>(iter: T) -> Self {
        let entries: Vec<(CompactString, V)> = iter
            .into_iter()
            .map(|(k, v)| {
                let mut cs = k.into();
                CompactString::shrink_to_fit(&mut cs);
                (cs, v)
            })
            .collect();
        Self::with_hasher_compact(entries, BH::default())
    }
}

impl<K, V, Q, BH> Map<K, V, Q> for FzStringMap<K, V, BH>
where
    Q: AsRef<str>,
    BH: BuildHasher,
    str: Equivalent<K>,
{
}

impl<K, V, Q, BH> MapExtras<K, V, Q> for FzStringMap<K, V, BH>
where
    Q: AsRef<str>,
    BH: BuildHasher,
    str: Equivalent<K>,
{
    fn get_key_value(&self, key: &Q) -> Option<(&K, &V)> {
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

impl<K, V, Q, BH> MapQuery<Q, V> for FzStringMap<K, V, BH>
where
    Q: AsRef<str>,
    BH: BuildHasher,
    str: Equivalent<K>,
{
    map_query_trait_funcs!();
}

impl<K, V, BH> MapIteration<K, V> for FzStringMap<K, V, BH>
where
    BH: BuildHasher,
{
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

    map_iteration_trait_funcs!();
}

impl<K, V, BH> Len for FzStringMap<K, V, BH> {
    len_trait_funcs!();
}

impl<K, V, Q, BH> Index<&Q> for FzStringMap<K, V, BH>
where
    Q: AsRef<str>,
    BH: BuildHasher,
    str: Equivalent<K>,
{
    index_trait_funcs!();
}

impl<K, V, BH> IntoIterator for FzStringMap<K, V, BH> {
    type Item = (K, V);
    type IntoIter = IntoIter<K, V>;

    fn into_iter(self) -> Self::IntoIter {
        self.into_iter()
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

impl<K, V, MT, BH> PartialEq<MT> for FzStringMap<K, V, BH>
where
    V: PartialEq,
    MT: MapQuery<K, V>,
    BH: BuildHasher,
{
    partial_eq_trait_funcs!();
}

impl<K, V, BH> Eq for FzStringMap<K, V, BH>
where
    K: AsRef<str>,
    V: Eq,
    BH: BuildHasher,
    str: Equivalent<K>,
{
}

impl<K, V, BH> Debug for FzStringMap<K, V, BH>
where
    K: Debug,
    V: Debug,
{
    debug_trait_funcs!();
}

#[cfg(feature = "serde")]
impl<K, V, BH> Serialize for FzStringMap<K, V, BH>
where
    K: Serialize,
    V: Serialize,
{
    serialize_trait_funcs!();
}

#[cfg(feature = "serde")]
impl<'de, V, BH> Deserialize<'de> for FzStringMap<Box<str>, V, BH>
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

#[cfg(all(feature = "serde", feature = "compact_str"))]
impl<'de, V, BH> Deserialize<'de> for FzStringMap<CompactString, V, BH>
where
    V: Deserialize<'de>,
    BH: BuildHasher + Default + Clone + Send + Sync + 'static,
{
    fn deserialize<D>(deserializer: D) -> core::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer.deserialize_map(CompactStrMapVisitor { marker: PhantomData })
    }
}

#[cfg(all(feature = "serde", feature = "compact_str"))]
struct CompactStrMapVisitor<V, BH> {
    marker: PhantomData<(V, BH)>,
}

#[cfg(all(feature = "serde", feature = "compact_str"))]
impl<'de, V, BH> Visitor<'de> for CompactStrMapVisitor<V, BH>
where
    V: Deserialize<'de>,
    BH: BuildHasher + Default + Clone + Send + Sync + 'static,
{
    type Value = FzStringMap<CompactString, V, BH>;

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

        Ok(FzStringMap::with_hasher_compact(v, BH::default()))
    }
}
