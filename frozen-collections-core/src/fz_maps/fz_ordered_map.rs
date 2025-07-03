use crate::maps::decl_macros::{
    debug_trait_funcs, index_trait_funcs, into_iterator_trait_funcs, into_iterator_trait_mut_ref_funcs, into_iterator_trait_ref_funcs,
    len_trait_funcs, map_extras_trait_funcs, map_iteration_trait_funcs, map_query_trait_funcs, partial_eq_trait_funcs,
};
use crate::maps::{EytzingerSearchMap, IntoIter, IntoKeys, IntoValues, Iter, IterMut, Keys, Values, ValuesMut};
use crate::traits::{Len, Map, MapExtras, MapIteration, MapQuery};
use crate::utils::SortedAndDeduppedVec;
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

/// A map optimized for fast read access with ordered keys.
///
#[doc = include_str!("../doc_snippets/about.md")]
#[doc = include_str!("../doc_snippets/ord_warning.md")]
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
pub struct FzOrderedMap<K, V> {
    map_impl: EytzingerSearchMap<K, V>,
}

impl<K, V> FzOrderedMap<K, V> {
    /// Creates a frozen ordered map.
    #[must_use]
    pub fn new(entries: Vec<(K, V)>) -> Self
    where
        K: Ord + Eq,
    {
        let entries = SortedAndDeduppedVec::new(entries, |x, y| x.0.cmp(&y.0));

        Self {
            map_impl: { EytzingerSearchMap::from_sorted_and_dedupped(entries) },
        }
    }

    #[doc = include_str!("../doc_snippets/get.md")]
    #[inline]
    pub fn get<Q>(&self, key: &Q) -> Option<&V>
    where
        Q: ?Sized + Comparable<K>,
    {
        self.map_impl.get(key)
    }

    #[doc = include_str!("../doc_snippets/get_mut.md")]
    #[inline]
    pub fn get_mut<Q>(&mut self, key: &Q) -> Option<&mut V>
    where
        Q: ?Sized + Comparable<K>,
    {
        self.map_impl.get_mut(key)
    }

    #[doc = include_str!("../doc_snippets/get_key_value.md")]
    #[inline]
    pub fn get_key_value<Q>(&self, key: &Q) -> Option<(&K, &V)>
    where
        Q: ?Sized + Comparable<K>,
    {
        self.map_impl.get_key_value(key)
    }

    #[doc = include_str!("../doc_snippets/contains_key.md")]
    #[inline]
    #[must_use]
    pub fn contains_key<Q>(&self, key: &Q) -> bool
    where
        Q: ?Sized + Comparable<K>,
    {
        self.map_impl.contains_key(key)
    }

    #[doc = include_str!("../doc_snippets/get_disjoint_mut.md")]
    #[must_use]
    pub fn get_disjoint_mut<Q, const N: usize>(&mut self, keys: [&Q; N]) -> [Option<&mut V>; N]
    where
        Q: ?Sized + Eq + Comparable<K>,
    {
        self.map_impl.get_disjoint_mut(keys)
    }

    #[doc = include_str!("../doc_snippets/get_disjoint_unchecked_mut.md")]
    #[must_use]
    pub unsafe fn get_disjoint_unchecked_mut<Q, const N: usize>(&mut self, keys: [&Q; N]) -> [Option<&mut V>; N]
    where
        Q: ?Sized + Comparable<K>,
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

impl<K, V> Default for FzOrderedMap<K, V> {
    fn default() -> Self {
        Self {
            map_impl: EytzingerSearchMap::default(),
        }
    }
}

impl<K, V, const N: usize> From<[(K, V); N]> for FzOrderedMap<K, V>
where
    K: Ord,
{
    fn from(entries: [(K, V); N]) -> Self {
        Self::new(Vec::from(entries))
    }
}

impl<K, V> FromIterator<(K, V)> for FzOrderedMap<K, V>
where
    K: Ord,
{
    fn from_iter<T: IntoIterator<Item = (K, V)>>(iter: T) -> Self {
        Self::new(iter.into_iter().collect())
    }
}

impl<K, V, Q> Map<K, V, Q> for FzOrderedMap<K, V> where Q: ?Sized + Comparable<K> {}

impl<K, V, Q> MapExtras<K, V, Q> for FzOrderedMap<K, V>
where
    Q: ?Sized + Comparable<K>,
{
    map_extras_trait_funcs!();
}

impl<K, V, Q> MapQuery<Q, V> for FzOrderedMap<K, V>
where
    Q: ?Sized + Comparable<K>,
{
    map_query_trait_funcs!();
}

impl<K, V> MapIteration<K, V> for FzOrderedMap<K, V> {
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

impl<K, V> Len for FzOrderedMap<K, V> {
    len_trait_funcs!();
}

impl<K, V, Q> Index<&Q> for FzOrderedMap<K, V>
where
    Q: ?Sized + Comparable<K>,
{
    index_trait_funcs!();
}

impl<K, V> IntoIterator for FzOrderedMap<K, V> {
    into_iterator_trait_funcs!();
}

impl<'a, K, V> IntoIterator for &'a FzOrderedMap<K, V> {
    into_iterator_trait_ref_funcs!();
}

impl<'a, K, V> IntoIterator for &'a mut FzOrderedMap<K, V> {
    into_iterator_trait_mut_ref_funcs!();
}

impl<K, V, MT> PartialEq<MT> for FzOrderedMap<K, V>
where
    K: Ord,
    V: PartialEq,
    MT: MapQuery<K, V>,
{
    partial_eq_trait_funcs!();
}

impl<K, V> Eq for FzOrderedMap<K, V>
where
    K: Ord,
    V: Eq,
{
}

impl<K, V> Debug for FzOrderedMap<K, V>
where
    K: Debug,
    V: Debug,
{
    debug_trait_funcs!();
}

#[cfg(feature = "serde")]
impl<K, V> Serialize for FzOrderedMap<K, V>
where
    K: Serialize,
    V: Serialize,
{
    serialize_trait_funcs!();
}

#[cfg(feature = "serde")]
impl<'de, K, V> Deserialize<'de> for FzOrderedMap<K, V>
where
    K: Deserialize<'de> + Ord,
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
    K: Deserialize<'de> + Ord,
    V: Deserialize<'de>,
{
    type Value = FzOrderedMap<K, V>;

    fn expecting(&self, formatter: &mut Formatter) -> Result {
        formatter.write_str("a map with ordered keys")
    }

    fn visit_map<M>(self, mut map: M) -> core::result::Result<Self::Value, M::Error>
    where
        M: MapAccess<'de>,
    {
        let mut v = Vec::with_capacity(map.size_hint().unwrap_or(0));
        while let Some(x) = map.next_entry()? {
            v.push(x);
        }

        Ok(FzOrderedMap::new(v))
    }
}
