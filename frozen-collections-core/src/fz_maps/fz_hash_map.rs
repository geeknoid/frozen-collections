use crate::DefaultHashBuilder;
use crate::hashers::BridgeHasher;
use crate::maps::{HashMap, IntoIter, IntoKeys, IntoValues, Iter, IterMut, Keys, Values, ValuesMut};
use crate::traits::{LargeCollection, Len, Map, MapIteration, MapQuery};
use crate::utils::dedup_by_hash_keep_last;
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

impl<K, V> FzHashMap<K, V, DefaultHashBuilder>
where
    K: Eq + Hash,
{
    /// Creates a frozen map.
    #[must_use]
    pub fn new(entries: Vec<(K, V)>) -> Self {
        Self::with_hasher(entries, RandomState::default())
    }
}

impl<K, V, BH> FzHashMap<K, V, BH>
where
    K: Eq + Hash,
    BH: BuildHasher,
{
    /// Creates a frozen map which uses the given hash builder to hash keys.
    #[must_use]
    #[allow(clippy::missing_panics_doc)]
    pub fn with_hasher(mut entries: Vec<(K, V)>, bh: BH) -> Self {
        dedup_by_hash_keep_last(&mut entries, |x| bh.hash_one(&x.0), |x, y| x.0 == y.0);

        Self {
            map_impl: HashMap::with_hasher_half_baked(entries, BridgeHasher::new(bh)).unwrap(),
        }
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
    Q: ?Sized + Hash + Eq + Equivalent<K>,
    BH: BuildHasher,
{
    fn get_disjoint_mut<const N: usize>(&mut self, keys: [&Q; N]) -> [Option<&mut V>; N] {
        self.map_impl.get_disjoint_mut(keys)
    }

    unsafe fn get_disjoint_unchecked_mut<const N: usize>(&mut self, keys: [&Q; N]) -> [Option<&mut V>; N] {
        unsafe { self.map_impl.get_disjoint_unchecked_mut(keys) }
    }
}

impl<K, V, Q, BH> MapQuery<K, V, Q> for FzHashMap<K, V, BH>
where
    Q: ?Sized + Hash + Eq + Equivalent<K>,
    BH: BuildHasher,
{
    #[inline]
    fn get(&self, key: &Q) -> Option<&V> {
        self.map_impl.get(key)
    }

    #[inline]
    fn get_key_value(&self, key: &Q) -> Option<(&K, &V)> {
        self.map_impl.get_key_value(key)
    }

    #[inline]
    fn get_mut(&mut self, key: &Q) -> Option<&mut V> {
        self.map_impl.get_mut(key)
    }
}

impl<K, V, BH> MapIteration<K, V> for FzHashMap<K, V, BH> {
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
        self.map_impl.iter()
    }

    fn keys(&self) -> Self::KeyIterator<'_> {
        self.map_impl.keys()
    }

    fn values(&self) -> Self::ValueIterator<'_> {
        self.map_impl.values()
    }

    fn into_keys(self) -> Self::IntoKeyIterator {
        self.map_impl.into_keys()
    }

    fn into_values(self) -> Self::IntoValueIterator {
        self.map_impl.into_values()
    }

    fn iter_mut(&mut self) -> Self::MutIterator<'_> {
        self.map_impl.iter_mut()
    }

    fn values_mut(&mut self) -> Self::ValueMutIterator<'_> {
        self.map_impl.values_mut()
    }
}

impl<K, V, BH> Len for FzHashMap<K, V, BH> {
    fn len(&self) -> usize {
        self.map_impl.len()
    }
}

impl<K, V, Q, BH> Index<&Q> for FzHashMap<K, V, BH>
where
    Q: ?Sized + Hash + Eq + Equivalent<K>,
    BH: BuildHasher,
{
    type Output = V;

    fn index(&self, index: &Q) -> &Self::Output {
        self.get(index).expect("index should be valid")
    }
}

impl<'a, K, V, BH> IntoIterator for &'a FzHashMap<K, V, BH> {
    type Item = (&'a K, &'a V);
    type IntoIter = Iter<'a, K, V>;

    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}

impl<'a, K, V, BH> IntoIterator for &'a mut FzHashMap<K, V, BH> {
    type Item = (&'a K, &'a mut V);
    type IntoIter = IterMut<'a, K, V>;

    fn into_iter(self) -> Self::IntoIter {
        self.iter_mut()
    }
}

impl<K, V, BH> IntoIterator for FzHashMap<K, V, BH> {
    type Item = (K, V);
    type IntoIter = IntoIter<K, V>;

    fn into_iter(self) -> Self::IntoIter {
        self.map_impl.into_iter()
    }
}

impl<K, V, MT, BH> PartialEq<MT> for FzHashMap<K, V, BH>
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

        self.iter().all(|(key, value)| other.get(key).is_some_and(|v| *value == *v))
    }
}

impl<K, V, BH> Eq for FzHashMap<K, V, BH>
where
    K: Hash + Eq,
    V: Eq,
    BH: BuildHasher,
{
}

impl<K, V, BH> Debug for FzHashMap<K, V, BH>
where
    K: Debug,
    V: Debug,
{
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        self.map_impl.fmt(f)
    }
}

#[cfg(feature = "serde")]
impl<K, V, BH> Serialize for FzHashMap<K, V, BH>
where
    K: Serialize,
    V: Serialize,
{
    serialize_fn!();
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

    fn visit_map<M>(self, mut access: M) -> core::result::Result<Self::Value, M::Error>
    where
        M: MapAccess<'de>,
    {
        let mut v = Vec::with_capacity(access.size_hint().unwrap_or(0));
        while let Some(x) = access.next_entry()? {
            v.push(x);
        }

        Ok(FzHashMap::with_hasher(v, BH::default()))
    }
}
