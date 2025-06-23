use crate::DefaultHashBuilder;
use crate::hashers::BridgeHasher;
use crate::maps::decl_macros::{
    debug_trait_funcs, index_trait_funcs, into_iterator_trait_funcs, into_iterator_trait_mut_ref_funcs, into_iterator_trait_ref_funcs,
    len_trait_funcs, map_extras_trait_funcs, map_iteration_trait_funcs, map_query_trait_funcs, partial_eq_trait_funcs,
};
use crate::maps::{HashMap, IntoIter, IntoKeys, IntoValues, Iter, IterMut, Keys, Values, ValuesMut};
use crate::traits::{LargeCollection, Len, Map, MapExtras, MapIteration, MapQuery};
use crate::utils::dedup_by_hash_keep_last;
use core::fmt::{Debug, Formatter, Result};
use core::hash::{BuildHasher, Hash};
use core::ops::Index;
use equivalent::Equivalent;

#[cfg(not(feature = "std"))]
use alloc::vec::Vec;

#[cfg(feature = "serde")]
use {
    crate::maps::decl_macros::serialize_trait_funcs,
    core::marker::PhantomData,
    serde::de::{MapAccess, Visitor},
    serde::ser::SerializeMap,
    serde::{Deserialize, Deserializer, Serialize, Serializer},
};

/// A map optimized for fast read access with hashable keys.
///
#[doc = include_str!("../doc_snippets/about.md")]
#[doc = include_str!("../doc_snippets/hash_warning.md")]
///
/// # Alternate Choices
///
/// If your keys are integers or enum variants, you should use the [`FzScalarMap`](crate::fz_maps::FzScalarMap) type instead.
/// If your keys are strings, you should use the [`FzStringMap`](crate::fz_maps::FzStringMap) type instead. Both of these will
/// deliver better performance since they are specifically optimized for those key types.
///
/// If your keys are known at compile time, consider using the various `fz_*_map` macros instead of
/// this type as they generally perform better.
#[derive(Clone)]
pub struct FzHashMap<K, V, BH = DefaultHashBuilder> {
    map_impl: HashMap<K, V, LargeCollection, BridgeHasher<BH>>,
}

impl<K, V> FzHashMap<K, V, DefaultHashBuilder> {
    /// Creates a frozen map.
    #[must_use]
    pub fn new(entries: Vec<(K, V)>) -> Self
    where
        K: Eq + Hash,
    {
        Self::with_hasher(entries, DefaultHashBuilder::default())
    }
}

impl<K, V, BH> FzHashMap<K, V, BH>
where
    BH: BuildHasher,
{
    /// Creates a frozen map which uses the given hash builder to hash keys.
    #[must_use]
    #[expect(
        clippy::missing_panics_doc,
        reason = "Guaranteed not to panic because the map is a LargeCollection"
    )]
    pub fn with_hasher(mut entries: Vec<(K, V)>, bh: BH) -> Self
    where
        K: Eq + Hash,
    {
        dedup_by_hash_keep_last(&mut entries, |x| bh.hash_one(&x.0), |x, y| x.0 == y.0);

        Self {
            map_impl: HashMap::with_hasher_half_baked(entries, BridgeHasher::new(bh)).unwrap(),
        }
    }

    #[doc = include_str!("../doc_snippets/get.md")]
    #[inline]
    #[must_use]
    pub fn get<Q>(&self, key: &Q) -> Option<&V>
    where
        Q: ?Sized + Hash + Equivalent<K>,
    {
        self.map_impl.get(key)
    }

    #[doc = include_str!("../doc_snippets/get_mut.md")]
    #[inline]
    #[must_use]
    pub fn get_mut<Q>(&mut self, key: &Q) -> Option<&mut V>
    where
        Q: ?Sized + Hash + Equivalent<K>,
    {
        self.map_impl.get_mut(key)
    }

    #[doc = include_str!("../doc_snippets/get_key_value.md")]
    #[inline]
    #[must_use]
    pub fn get_key_value<Q>(&self, key: &Q) -> Option<(&K, &V)>
    where
        Q: ?Sized + Hash + Equivalent<K>,
    {
        self.map_impl.get_key_value(key)
    }

    #[doc = include_str!("../doc_snippets/contains_key.md")]
    #[inline]
    #[must_use]
    pub fn contains_key<Q>(&self, key: &Q) -> bool
    where
        Q: ?Sized + Hash + Equivalent<K>,
    {
        self.map_impl.contains_key(key)
    }

    #[doc = include_str!("../doc_snippets/get_disjoint_mut.md")]
    #[must_use]
    pub fn get_disjoint_mut<Q, const N: usize>(&mut self, keys: [&Q; N]) -> [Option<&mut V>; N]
    where
        Q: ?Sized + Hash + Eq + Equivalent<K>,
    {
        self.map_impl.get_disjoint_mut(keys)
    }

    #[doc = include_str!("../doc_snippets/get_disjoint_unchecked_mut.md")]
    #[must_use]
    pub unsafe fn get_disjoint_unchecked_mut<Q, const N: usize>(&mut self, keys: [&Q; N]) -> [Option<&mut V>; N]
    where
        Q: ?Sized + Hash + Equivalent<K>,
    {
        // SAFETY: The caller must ensure that the keys are disjoint and valid for the map.
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
    pub fn iter(&self) -> Iter<'_, K, V> {
        self.map_impl.iter()
    }

    #[doc = include_str!("../doc_snippets/iter_mut.md")]
    #[must_use]
    pub fn iter_mut(&mut self) -> IterMut<'_, K, V> {
        self.map_impl.iter_mut()
    }

    #[must_use]
    fn into_iter(self) -> IntoIter<K, V> {
        self.map_impl.into_iter()
    }

    #[doc = include_str!("../doc_snippets/keys.md")]
    #[must_use]
    pub fn keys(&self) -> Keys<'_, K, V> {
        self.map_impl.keys()
    }

    #[doc = include_str!("../doc_snippets/into_keys.md")]
    #[must_use]
    pub fn into_keys(self) -> IntoKeys<K, V> {
        self.map_impl.into_keys()
    }

    #[doc = include_str!("../doc_snippets/values.md")]
    #[must_use]
    pub fn values(&self) -> Values<'_, K, V> {
        self.map_impl.values()
    }

    #[doc = include_str!("../doc_snippets/values_mut.md")]
    #[must_use]
    pub fn values_mut(&mut self) -> ValuesMut<'_, K, V> {
        self.map_impl.values_mut()
    }

    #[doc = include_str!("../doc_snippets/into_values.md")]
    #[must_use]
    pub fn into_values(self) -> IntoValues<K, V> {
        self.map_impl.into_values()
    }
}

impl<K, V, BH> Default for FzHashMap<K, V, BH>
where
    BH: Default,
{
    fn default() -> Self {
        Self {
            map_impl: HashMap::default(),
        }
    }
}

impl<K, V, const N: usize, BH> From<[(K, V); N]> for FzHashMap<K, V, BH>
where
    K: Eq + Hash,
    BH: BuildHasher + Default,
{
    fn from(entries: [(K, V); N]) -> Self {
        Self::with_hasher(Vec::from(entries), BH::default())
    }
}

impl<K, V, BH> FromIterator<(K, V)> for FzHashMap<K, V, BH>
where
    K: Eq + Hash,
    BH: BuildHasher + Default,
{
    fn from_iter<T: IntoIterator<Item = (K, V)>>(iter: T) -> Self {
        Self::with_hasher(iter.into_iter().collect(), BH::default())
    }
}

impl<K, V, Q, BH> Map<K, V, Q> for FzHashMap<K, V, BH>
where
    Q: ?Sized + Hash + Equivalent<K>,
    BH: BuildHasher,
{
}

impl<K, V, Q, BH> MapExtras<K, V, Q> for FzHashMap<K, V, BH>
where
    Q: ?Sized + Hash + Equivalent<K>,
    BH: BuildHasher,
{
    map_extras_trait_funcs!();
}

impl<K, V, Q, BH> MapQuery<Q, V> for FzHashMap<K, V, BH>
where
    Q: ?Sized + Hash + Equivalent<K>,
    BH: BuildHasher,
{
    map_query_trait_funcs!();
}

impl<K, V, BH> MapIteration<K, V> for FzHashMap<K, V, BH>
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

impl<K, V, BH> Len for FzHashMap<K, V, BH>
where
    BH: BuildHasher,
{
    len_trait_funcs!();
}

impl<K, V, Q, BH> Index<&Q> for FzHashMap<K, V, BH>
where
    Q: ?Sized + Hash + Equivalent<K>,
    BH: BuildHasher,
{
    index_trait_funcs!();
}

impl<K, V, BH> IntoIterator for FzHashMap<K, V, BH>
where
    BH: BuildHasher,
{
    into_iterator_trait_funcs!();
}

impl<'a, K, V, BH> IntoIterator for &'a FzHashMap<K, V, BH>
where
    BH: BuildHasher,
{
    into_iterator_trait_ref_funcs!();
}

impl<'a, K, V, BH> IntoIterator for &'a mut FzHashMap<K, V, BH>
where
    BH: BuildHasher,
{
    into_iterator_trait_mut_ref_funcs!();
}

impl<K, V, MT, BH> PartialEq<MT> for FzHashMap<K, V, BH>
where
    K: PartialEq + Hash,
    V: PartialEq,
    MT: MapQuery<K, V>,
    BH: BuildHasher,
{
    partial_eq_trait_funcs!();
}

impl<K, V, BH> Eq for FzHashMap<K, V, BH>
where
    K: Eq + Hash,
    V: Eq,
    BH: BuildHasher,
{
}

impl<K, V, BH> Debug for FzHashMap<K, V, BH>
where
    K: Debug,
    V: Debug,
    BH: BuildHasher,
{
    debug_trait_funcs!();
}

#[cfg(feature = "serde")]
impl<K, V, BH> Serialize for FzHashMap<K, V, BH>
where
    K: Serialize,
    V: Serialize,
    BH: BuildHasher,
{
    serialize_trait_funcs!();
}

#[cfg(feature = "serde")]
impl<'de, K, V, BH> Deserialize<'de> for FzHashMap<K, V, BH>
where
    K: Deserialize<'de> + Hash + Eq,
    V: Deserialize<'de>,
    BH: BuildHasher + Default,
{
    fn deserialize<D>(deserializer: D) -> core::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer.deserialize_map(MapVisitor { marker: PhantomData })
    }
}

#[cfg(feature = "serde")]
struct MapVisitor<K, V, BH> {
    marker: PhantomData<(K, V, BH)>,
}

#[cfg(feature = "serde")]
impl<'de, K, V, BH> Visitor<'de> for MapVisitor<K, V, BH>
where
    K: Deserialize<'de> + Hash + Eq,
    V: Deserialize<'de>,
    BH: BuildHasher + Default,
{
    type Value = FzHashMap<K, V, BH>;

    fn expecting(&self, formatter: &mut Formatter) -> Result {
        formatter.write_str("a map with hashable keys")
    }

    fn visit_map<M>(self, mut map: M) -> core::result::Result<Self::Value, M::Error>
    where
        M: MapAccess<'de>,
    {
        let mut v = Vec::with_capacity(map.size_hint().unwrap_or(0));
        while let Some(x) = map.next_entry()? {
            v.push(x);
        }

        Ok(FzHashMap::with_hasher(v, BH::default()))
    }
}
