use crate::analyzers::{ScalarKeyAnalysisResult, analyze_scalar_keys};
use crate::hashers::PassthroughHasher;
use crate::maps::{
    DenseScalarLookupMap, HashMap, IntoIter, IntoKeys, IntoValues, Iter, IterMut, Keys,
    SparseScalarLookupMap, Values, ValuesMut,
};
use crate::traits::{LargeCollection, Len, Map, MapIteration, MapQuery, Scalar};
use crate::utils::dedup_by_keep_last;
use alloc::vec::Vec;
use core::fmt::{Debug, Formatter, Result};
use core::iter::FromIterator;
use core::ops::Index;
use equivalent::Equivalent;

#[cfg(feature = "serde")]
use {
    crate::maps::decl_macros::serialize_fn,
    core::marker::PhantomData,
    serde::de::{MapAccess, Visitor},
    serde::ser::SerializeMap,
    serde::{Deserialize, Deserializer, Serialize, Serializer},
};

#[derive(Clone)]
enum MapTypes<K, V> {
    Hash(HashMap<K, V, LargeCollection, PassthroughHasher>),
    Dense(DenseScalarLookupMap<K, V>),
    Sparse(SparseScalarLookupMap<K, V>),
}

/// A map optimized for fast read access using scalar keys.
///
#[doc = include_str!("../doc_snippets/about.md")]
///
/// # Alternate Choices
///
/// If your keys are known at compile time, consider using the various `fz_*_map` macros instead of
/// this type as they generally perform better.
#[derive(Clone)]
pub struct FzScalarMap<K, V> {
    map_impl: MapTypes<K, V>,
}

impl<K, V> FzScalarMap<K, V>
where
    K: Scalar,
{
    /// Creates a frozen map.
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
                    MapTypes::Hash(HashMap::with_hasher_half_baked(entries, h).unwrap())
                }
            },
        }
    }
}

impl<K, V> Default for FzScalarMap<K, V> {
    fn default() -> Self {
        Self {
            map_impl: MapTypes::Dense(DenseScalarLookupMap::default()),
        }
    }
}

impl<K, V, const N: usize> From<[(K, V); N]> for FzScalarMap<K, V>
where
    K: Scalar,
{
    fn from(entries: [(K, V); N]) -> Self {
        Self::new(Vec::from(entries))
    }
}

impl<K, V> FromIterator<(K, V)> for FzScalarMap<K, V>
where
    K: Scalar,
{
    fn from_iter<T: IntoIterator<Item = (K, V)>>(iter: T) -> Self {
        Self::new(iter.into_iter().collect())
    }
}

impl<K, V> Map<K, V, K> for FzScalarMap<K, V>
where
    K: Scalar + Eq + Equivalent<K>,
{
    fn get_disjoint_mut<const N: usize>(&mut self, keys: [&K; N]) -> [Option<&mut V>; N] {
        match &mut self.map_impl {
            MapTypes::Hash(m) => m.get_disjoint_mut(keys),
            MapTypes::Dense(m) => m.get_disjoint_mut(keys),
            MapTypes::Sparse(m) => m.get_disjoint_mut(keys),
        }
    }

    unsafe fn get_disjoint_unchecked_mut<const N: usize>(
        &mut self,
        keys: [&K; N],
    ) -> [Option<&mut V>; N] {
        unsafe {
            match &mut self.map_impl {
                MapTypes::Hash(m) => m.get_disjoint_unchecked_mut(keys),
                MapTypes::Dense(m) => m.get_disjoint_unchecked_mut(keys),
                MapTypes::Sparse(m) => m.get_disjoint_unchecked_mut(keys),
            }
        }
    }
}

impl<K, V> MapQuery<K, V, K> for FzScalarMap<K, V>
where
    K: Scalar + Eq + Equivalent<K>,
{
    #[inline(always)]
    fn get(&self, key: &K) -> Option<&V> {
        match &self.map_impl {
            MapTypes::Hash(m) => m.get(key),
            MapTypes::Dense(m) => m.get(key),
            MapTypes::Sparse(m) => m.get(key),
        }
    }

    #[inline]
    fn get_key_value(&self, key: &K) -> Option<(&K, &V)> {
        match &self.map_impl {
            MapTypes::Hash(m) => m.get_key_value(key),
            MapTypes::Dense(m) => m.get_key_value(key),
            MapTypes::Sparse(m) => m.get_key_value(key),
        }
    }

    #[inline]
    fn get_mut(&mut self, key: &K) -> Option<&mut V> {
        match &mut self.map_impl {
            MapTypes::Hash(m) => m.get_mut(key),
            MapTypes::Dense(m) => m.get_mut(key),
            MapTypes::Sparse(m) => m.get_mut(key),
        }
    }
}

impl<K, V> MapIteration<K, V> for FzScalarMap<K, V> {
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

impl<K, V> Len for FzScalarMap<K, V> {
    fn len(&self) -> usize {
        match &self.map_impl {
            MapTypes::Hash(m) => m.len(),
            MapTypes::Dense(m) => m.len(),
            MapTypes::Sparse(m) => m.len(),
        }
    }
}

impl<Q, V> Index<&Q> for FzScalarMap<Q, V>
where
    Q: Scalar + Eq + Equivalent<Q>,
{
    type Output = V;

    fn index(&self, index: &Q) -> &Self::Output {
        self.get(index).expect("index should be valid")
    }
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
            .all(|(key, value)| other.get(key).is_some_and(|v| *value == *v))
    }
}

impl<K, V> Eq for FzScalarMap<K, V>
where
    K: Scalar,
    V: Eq,
{
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

#[cfg(feature = "serde")]
impl<K, V> Serialize for FzScalarMap<K, V>
where
    K: Serialize,
    V: Serialize,
{
    serialize_fn!();
}

#[cfg(feature = "serde")]
impl<'de, K, V> Deserialize<'de> for FzScalarMap<K, V>
where
    K: Deserialize<'de> + Scalar,
    V: Deserialize<'de>,
{
    fn deserialize<D>(deserializer: D) -> core::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer.deserialize_map(MapVisitor {
            marker: PhantomData,
        })
    }
}

#[cfg(feature = "serde")]
struct MapVisitor<K, V> {
    marker: PhantomData<(K, V)>,
}

#[cfg(feature = "serde")]
impl<'de, K, V> Visitor<'de> for MapVisitor<K, V>
where
    K: Deserialize<'de> + Scalar,
    V: Deserialize<'de>,
{
    type Value = FzScalarMap<K, V>;

    fn expecting(&self, formatter: &mut Formatter) -> Result {
        formatter.write_str("a map with scalar keys")
    }

    fn visit_map<M>(self, mut access: M) -> core::result::Result<Self::Value, M::Error>
    where
        M: MapAccess<'de>,
    {
        let mut v = Vec::with_capacity(access.size_hint().unwrap_or(0));
        while let Some(x) = access.next_entry()? {
            v.push(x);
        }

        Ok(FzScalarMap::new(v))
    }
}
