use crate::analyzers::{ScalarKeyAnalysisResult, analyze_scalar_keys};
use crate::hashers::PassthroughHasher;
use crate::maps::decl_macros::{
    debug_trait_funcs, index_trait_funcs, into_iterator_trait_funcs, into_iterator_trait_mut_ref_funcs, into_iterator_trait_ref_funcs,
    len_trait_funcs, map_extras_trait_funcs, map_iteration_trait_funcs, map_query_trait_funcs, partial_eq_trait_funcs,
};
use crate::maps::{
    DenseScalarLookupMap, HashMap, IntoIter, IntoKeys, IntoValues, Iter, IterMut, Keys, SparseScalarLookupMap, Values, ValuesMut,
};
use crate::traits::{LargeCollection, Len, Map, MapExtras, MapIteration, MapQuery, Scalar};
use crate::utils::dedup_by_keep_last;
use core::fmt::{Debug, Formatter, Result};
use core::ops::Index;
use equivalent::Comparable;

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

impl<K, V> FzScalarMap<K, V> {
    /// Creates a frozen map.
    #[must_use]
    #[expect(clippy::missing_panics_doc, reason = "Guaranteed to work because the map is a LargeCollection")]
    pub fn new(mut entries: Vec<(K, V)>) -> Self
    where
        K: Scalar,
    {
        entries.sort_by(|x, y| x.0.cmp(&y.0));
        dedup_by_keep_last(&mut entries, |x, y| x.0.eq(&y.0));

        Self {
            map_impl: match analyze_scalar_keys(entries.iter().map(|x| x.0)) {
                ScalarKeyAnalysisResult::DenseRange => MapTypes::Dense(DenseScalarLookupMap::new_raw(entries)),
                ScalarKeyAnalysisResult::SparseRange => MapTypes::Sparse(SparseScalarLookupMap::new_raw(entries)),
                ScalarKeyAnalysisResult::General => MapTypes::Hash(HashMap::with_hasher_half_baked(entries, PassthroughHasher {}).unwrap()),
            },
        }
    }

    #[doc = include_str!("../doc_snippets/get.md")]
    #[inline]
    pub fn get<Q>(&self, key: &Q) -> Option<&V>
    where
        Q: Scalar + Comparable<K>,
    {
        match &self.map_impl {
            MapTypes::Hash(m) => m.get(key),
            MapTypes::Dense(m) => m.get(key),
            MapTypes::Sparse(m) => m.get(key),
        }
    }

    #[doc = include_str!("../doc_snippets/get_mut.md")]
    #[inline]
    pub fn get_mut<Q>(&mut self, key: &Q) -> Option<&mut V>
    where
        Q: Scalar + Comparable<K>,
    {
        match &mut self.map_impl {
            MapTypes::Hash(m) => m.get_mut(key),
            MapTypes::Dense(m) => m.get_mut(key),
            MapTypes::Sparse(m) => m.get_mut(key),
        }
    }

    #[doc = include_str!("../doc_snippets/get_key_value.md")]
    #[inline]
    pub fn get_key_value<Q>(&self, key: &Q) -> Option<(&K, &V)>
    where
        Q: Scalar + Comparable<K>,
    {
        match &self.map_impl {
            MapTypes::Hash(m) => m.get_key_value(key),
            MapTypes::Dense(m) => m.get_key_value(key),
            MapTypes::Sparse(m) => m.get_key_value(key),
        }
    }

    #[doc = include_str!("../doc_snippets/contains_key.md")]
    #[inline]
    #[must_use]
    pub fn contains_key<Q>(&self, key: &Q) -> bool
    where
        Q: Scalar + Comparable<K>,
    {
        match &self.map_impl {
            MapTypes::Hash(m) => m.contains_key(key),
            MapTypes::Dense(m) => m.contains_key(key),
            MapTypes::Sparse(m) => m.contains_key(key),
        }
    }

    #[doc = include_str!("../doc_snippets/get_disjoint_mut.md")]
    #[must_use]
    pub fn get_disjoint_mut<Q, const N: usize>(&mut self, keys: [&Q; N]) -> [Option<&mut V>; N]
    where
        Q: Scalar + Comparable<K>,
    {
        match &mut self.map_impl {
            MapTypes::Hash(m) => m.get_disjoint_mut(keys),
            MapTypes::Dense(m) => m.get_disjoint_mut(keys),
            MapTypes::Sparse(m) => m.get_disjoint_mut(keys),
        }
    }

    #[doc = include_str!("../doc_snippets/get_disjoint_unchecked_mut.md")]
    #[must_use]
    pub unsafe fn get_disjoint_unchecked_mut<Q, const N: usize>(&mut self, keys: [&Q; N]) -> [Option<&mut V>; N]
    where
        Q: Scalar + Comparable<K>,
    {
        match &mut self.map_impl {
            MapTypes::Hash(m) => {
                // SAFETY: The caller must ensure that the keys are disjoint and valid for the map.
                unsafe { m.get_disjoint_unchecked_mut(keys) }
            }

            MapTypes::Dense(m) => {
                // SAFETY: The caller must ensure that the keys are disjoint and valid for the map.
                unsafe { m.get_disjoint_unchecked_mut(keys) }
            }

            MapTypes::Sparse(m) => {
                // SAFETY: The caller must ensure that the keys are disjoint and valid for the map.
                unsafe { m.get_disjoint_unchecked_mut(keys) }
            }
        }
    }

    #[doc = include_str!("../doc_snippets/len.md")]
    #[inline]
    #[must_use]
    pub fn len(&self) -> usize {
        match &self.map_impl {
            MapTypes::Hash(m) => m.len(),
            MapTypes::Dense(m) => m.len(),
            MapTypes::Sparse(m) => m.len(),
        }
    }

    #[doc = include_str!("../doc_snippets/is_empty.md")]
    #[inline]
    #[must_use]
    pub fn is_empty(&self) -> bool {
        match &self.map_impl {
            MapTypes::Hash(m) => m.is_empty(),
            MapTypes::Dense(m) => m.is_empty(),
            MapTypes::Sparse(m) => m.is_empty(),
        }
    }

    #[doc = include_str!("../doc_snippets/iter.md")]
    #[must_use]
    pub fn iter(&self) -> Iter<'_, K, V> {
        match &self.map_impl {
            MapTypes::Hash(m) => m.iter(),
            MapTypes::Dense(m) => m.iter(),
            MapTypes::Sparse(m) => m.iter(),
        }
    }

    #[doc = include_str!("../doc_snippets/iter_mut.md")]
    #[must_use]
    pub fn iter_mut(&mut self) -> IterMut<'_, K, V> {
        match &mut self.map_impl {
            MapTypes::Hash(m) => m.iter_mut(),
            MapTypes::Dense(m) => m.iter_mut(),
            MapTypes::Sparse(m) => m.iter_mut(),
        }
    }

    #[must_use]
    fn into_iter(self) -> IntoIter<K, V> {
        match self.map_impl {
            MapTypes::Hash(m) => m.into_iter(),
            MapTypes::Dense(m) => m.into_iter(),
            MapTypes::Sparse(m) => m.into_iter(),
        }
    }

    #[doc = include_str!("../doc_snippets/keys.md")]
    #[must_use]
    pub fn keys(&self) -> Keys<'_, K, V> {
        match &self.map_impl {
            MapTypes::Hash(m) => m.keys(),
            MapTypes::Dense(m) => m.keys(),
            MapTypes::Sparse(m) => m.keys(),
        }
    }

    #[doc = include_str!("../doc_snippets/into_keys.md")]
    #[must_use]
    pub fn into_keys(self) -> IntoKeys<K, V> {
        match self.map_impl {
            MapTypes::Hash(m) => m.into_keys(),
            MapTypes::Dense(m) => m.into_keys(),
            MapTypes::Sparse(m) => m.into_keys(),
        }
    }

    #[doc = include_str!("../doc_snippets/values.md")]
    #[must_use]
    pub fn values(&self) -> Values<'_, K, V> {
        match &self.map_impl {
            MapTypes::Hash(m) => m.values(),
            MapTypes::Dense(m) => m.values(),
            MapTypes::Sparse(m) => m.values(),
        }
    }

    #[doc = include_str!("../doc_snippets/values_mut.md")]
    #[must_use]
    pub fn values_mut(&mut self) -> ValuesMut<'_, K, V> {
        match &mut self.map_impl {
            MapTypes::Hash(m) => m.values_mut(),
            MapTypes::Dense(m) => m.values_mut(),
            MapTypes::Sparse(m) => m.values_mut(),
        }
    }

    #[doc = include_str!("../doc_snippets/into_values.md")]
    #[must_use]
    pub fn into_values(self) -> IntoValues<K, V> {
        match self.map_impl {
            MapTypes::Hash(m) => m.into_values(),
            MapTypes::Dense(m) => m.into_values(),
            MapTypes::Sparse(m) => m.into_values(),
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

impl<K, V, Q> Map<K, V, Q> for FzScalarMap<K, V> where Q: Scalar + Comparable<K> {}

impl<K, V, Q> MapExtras<K, V, Q> for FzScalarMap<K, V>
where
    Q: Scalar + Comparable<K>,
{
    map_extras_trait_funcs!();
}

impl<K, V, Q> MapQuery<Q, V> for FzScalarMap<K, V>
where
    Q: Scalar + Comparable<K>,
{
    map_query_trait_funcs!();
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

    map_iteration_trait_funcs!();
}

impl<K, V> Len for FzScalarMap<K, V> {
    len_trait_funcs!();
}

impl<Q, V> Index<&Q> for FzScalarMap<Q, V>
where
    Q: Scalar + Eq,
{
    index_trait_funcs!();
}

impl<K, V> IntoIterator for FzScalarMap<K, V> {
    into_iterator_trait_funcs!();
}

impl<'a, K, V> IntoIterator for &'a FzScalarMap<K, V> {
    into_iterator_trait_ref_funcs!();
}

impl<'a, K, V> IntoIterator for &'a mut FzScalarMap<K, V> {
    into_iterator_trait_mut_ref_funcs!();
}

impl<K, V, MT> PartialEq<MT> for FzScalarMap<K, V>
where
    K: Scalar,
    V: PartialEq,
    MT: MapQuery<K, V>,
{
    partial_eq_trait_funcs!();
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
    debug_trait_funcs!();
}

#[cfg(feature = "serde")]
impl<K, V> Serialize for FzScalarMap<K, V>
where
    K: Serialize,
    V: Serialize,
{
    serialize_trait_funcs!();
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
        deserializer.deserialize_map(MapVisitor { marker: PhantomData })
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

    fn visit_map<M>(self, mut map: M) -> core::result::Result<Self::Value, M::Error>
    where
        M: MapAccess<'de>,
    {
        let mut v = Vec::with_capacity(map.size_hint().unwrap_or(0));
        while let Some(x) = map.next_entry()? {
            v.push(x);
        }

        Ok(FzScalarMap::new(v))
    }
}
