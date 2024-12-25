use crate::maps::{
    BinarySearchMap, EytzingerSearchMap, IntoIter, IntoKeys, IntoValues, Iter, IterMut, Keys,
    OrderedScanMap, Values, ValuesMut,
};
use crate::traits::{Len, Map, MapIteration, MapQuery};
use crate::utils::{dedup_by_keep_last, eytzinger_sort};
use alloc::vec::Vec;
use core::fmt::{Debug, Formatter, Result};
use core::iter::FromIterator;
use core::ops::Index;
use equivalent::Comparable;

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
    BinarySearch(BinarySearchMap<K, V>),
    EytzingerSearch(EytzingerSearchMap<K, V>),
    Scanning(OrderedScanMap<K, V>),
}

/// A map optimized for fast read access with ordered keys.
///
#[doc = include_str!("../doc_snippets/about.md")]
#[doc = include_str!("../doc_snippets/order_warning.md")]
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
    map_impl: MapTypes<K, V>,
}

impl<K, V> FzOrderedMap<K, V>
where
    K: Ord + Eq,
{
    /// Creates a frozen ordered map.
    #[must_use]
    pub fn new(mut entries: Vec<(K, V)>) -> Self {
        entries.sort_by(|x, y| x.0.cmp(&y.0));
        dedup_by_keep_last(&mut entries, |x, y| x.0.eq(&y.0));

        Self {
            map_impl: if entries.len() < 5 {
                MapTypes::Scanning(OrderedScanMap::new_raw(entries))
            } else if entries.len() < 64 {
                MapTypes::BinarySearch(BinarySearchMap::new_raw(entries))
            } else {
                eytzinger_sort(&mut entries);
                MapTypes::EytzingerSearch(EytzingerSearchMap::new_raw(entries))
            },
        }
    }
}

impl<K, V> Default for FzOrderedMap<K, V> {
    fn default() -> Self {
        Self {
            map_impl: MapTypes::Scanning(OrderedScanMap::default()),
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

impl<K, V, Q> Map<K, V, Q> for FzOrderedMap<K, V>
where
    Q: ?Sized + Eq + Comparable<K>,
{
    #[must_use]
    fn get_many_mut<const N: usize>(&mut self, keys: [&Q; N]) -> Option<[&mut V; N]> {
        match &mut self.map_impl {
            MapTypes::BinarySearch(m) => m.get_many_mut(keys),
            MapTypes::EytzingerSearch(m) => m.get_many_mut(keys),
            MapTypes::Scanning(m) => m.get_many_mut(keys),
        }
    }
}

impl<K, V, Q> MapQuery<K, V, Q> for FzOrderedMap<K, V>
where
    Q: ?Sized + Eq + Comparable<K>,
{
    #[inline]
    #[must_use]
    fn get(&self, key: &Q) -> Option<&V> {
        match &self.map_impl {
            MapTypes::BinarySearch(m) => m.get(key),
            MapTypes::EytzingerSearch(m) => m.get(key),
            MapTypes::Scanning(m) => m.get(key),
        }
    }

    #[inline]
    #[must_use]
    fn get_key_value(&self, key: &Q) -> Option<(&K, &V)> {
        match &self.map_impl {
            MapTypes::BinarySearch(m) => m.get_key_value(key),
            MapTypes::EytzingerSearch(m) => m.get_key_value(key),
            MapTypes::Scanning(m) => m.get_key_value(key),
        }
    }

    #[inline]
    #[must_use]
    fn get_mut(&mut self, key: &Q) -> Option<&mut V> {
        match &mut self.map_impl {
            MapTypes::BinarySearch(m) => m.get_mut(key),
            MapTypes::EytzingerSearch(m) => m.get_mut(key),
            MapTypes::Scanning(m) => m.get_mut(key),
        }
    }
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
            MapTypes::BinarySearch(m) => m.iter(),
            MapTypes::EytzingerSearch(m) => m.iter(),
            MapTypes::Scanning(m) => m.iter(),
        }
    }

    fn keys(&self) -> Self::KeyIterator<'_> {
        match &self.map_impl {
            MapTypes::BinarySearch(m) => m.keys(),
            MapTypes::EytzingerSearch(m) => m.keys(),
            MapTypes::Scanning(m) => m.keys(),
        }
    }

    fn values(&self) -> Self::ValueIterator<'_> {
        match &self.map_impl {
            MapTypes::BinarySearch(m) => m.values(),
            MapTypes::EytzingerSearch(m) => m.values(),
            MapTypes::Scanning(m) => m.values(),
        }
    }

    fn into_keys(self) -> Self::IntoKeyIterator {
        match self.map_impl {
            MapTypes::BinarySearch(m) => m.into_keys(),
            MapTypes::EytzingerSearch(m) => m.into_keys(),
            MapTypes::Scanning(m) => m.into_keys(),
        }
    }

    fn into_values(self) -> Self::IntoValueIterator {
        match self.map_impl {
            MapTypes::BinarySearch(m) => m.into_values(),
            MapTypes::EytzingerSearch(m) => m.into_values(),
            MapTypes::Scanning(m) => m.into_values(),
        }
    }

    fn iter_mut(&mut self) -> Self::MutIterator<'_> {
        match &mut self.map_impl {
            MapTypes::BinarySearch(m) => m.iter_mut(),
            MapTypes::EytzingerSearch(m) => m.iter_mut(),
            MapTypes::Scanning(m) => m.iter_mut(),
        }
    }

    fn values_mut(&mut self) -> Self::ValueMutIterator<'_> {
        match &mut self.map_impl {
            MapTypes::BinarySearch(m) => m.values_mut(),
            MapTypes::EytzingerSearch(m) => m.values_mut(),
            MapTypes::Scanning(m) => m.values_mut(),
        }
    }
}

impl<K, V> Len for FzOrderedMap<K, V> {
    fn len(&self) -> usize {
        match &self.map_impl {
            MapTypes::BinarySearch(m) => m.len(),
            MapTypes::EytzingerSearch(m) => m.len(),
            MapTypes::Scanning(m) => m.len(),
        }
    }
}

impl<K, V, Q> Index<&Q> for FzOrderedMap<K, V>
where
    Q: ?Sized + Eq + Comparable<K>,
{
    type Output = V;

    fn index(&self, index: &Q) -> &Self::Output {
        self.get(index).expect("index should be valid")
    }
}

impl<'a, K, V> IntoIterator for &'a FzOrderedMap<K, V> {
    type Item = (&'a K, &'a V);
    type IntoIter = Iter<'a, K, V>;

    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}

impl<'a, K, V> IntoIterator for &'a mut FzOrderedMap<K, V> {
    type Item = (&'a K, &'a mut V);
    type IntoIter = IterMut<'a, K, V>;

    fn into_iter(self) -> Self::IntoIter {
        self.iter_mut()
    }
}

impl<K, V> IntoIterator for FzOrderedMap<K, V> {
    type Item = (K, V);
    type IntoIter = IntoIter<K, V>;

    fn into_iter(self) -> Self::IntoIter {
        match self.map_impl {
            MapTypes::BinarySearch(m) => m.into_iter(),
            MapTypes::EytzingerSearch(m) => m.into_iter(),
            MapTypes::Scanning(m) => m.into_iter(),
        }
    }
}

impl<K, V, MT> PartialEq<MT> for FzOrderedMap<K, V>
where
    K: Ord + Eq,
    V: PartialEq,
    MT: Map<K, V>,
{
    fn eq(&self, other: &MT) -> bool {
        if self.len() != other.len() {
            return false;
        }

        self.iter()
            .all(|(key, value)| other.get(key).map_or(false, |v| *value == *v))
    }
}

impl<K, V> Eq for FzOrderedMap<K, V>
where
    K: Ord + Eq,
    V: Eq,
{
}

impl<K, V> Debug for FzOrderedMap<K, V>
where
    K: Debug,
    V: Debug,
{
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        match &self.map_impl {
            MapTypes::BinarySearch(m) => m.fmt(f),
            MapTypes::EytzingerSearch(m) => m.fmt(f),
            MapTypes::Scanning(m) => m.fmt(f),
        }
    }
}

#[cfg(feature = "serde")]
impl<K, V> Serialize for FzOrderedMap<K, V>
where
    K: Serialize,
    V: Serialize,
{
    serialize_fn!();
}

#[cfg(feature = "serde")]
impl<'de, K, V> Deserialize<'de> for FzOrderedMap<K, V>
where
    K: Deserialize<'de> + Ord,
    V: Deserialize<'de>,
{
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
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
    K: Deserialize<'de> + Ord,
    V: Deserialize<'de>,
{
    type Value = FzOrderedMap<K, V>;

    fn expecting(&self, formatter: &mut Formatter) -> Result {
        formatter.write_str("a map with ordered keys")
    }

    fn visit_map<M>(self, mut access: M) -> core::result::Result<Self::Value, M::Error>
    where
        M: MapAccess<'de>,
    {
        let mut v = Vec::with_capacity(access.size_hint().unwrap_or(0));
        while let Some(x) = access.next_entry()? {
            v.push(x);
        }

        Ok(FzOrderedMap::new(v))
    }
}
