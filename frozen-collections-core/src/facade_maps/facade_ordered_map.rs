use crate::maps::{
    BinarySearchMap, EytzingerSearchMap, IntoIter, IntoKeys, IntoValues, Iter, IterMut, Keys,
    OrderedScanMap, Values, ValuesMut,
};
use crate::traits::{Len, Map, MapIteration, MapQuery};
use crate::utils::{dedup_by_keep_last, eytzinger_sort};
use alloc::vec::Vec;
use core::fmt::{Debug, Formatter, Result};
use core::ops::Index;
use equivalent::Comparable;

#[derive(Clone)]
enum MapTypes<K, V> {
    BinarySearch(BinarySearchMap<K, V>),
    EytzingerSearch(EytzingerSearchMap<K, V>),
    Scanning(OrderedScanMap<K, V>),
}

/// A map optimized for fast read access with ordered keys.
///
#[doc = include_str!("../doc_snippets/type_compat_warning.md")]
#[doc = include_str!("../doc_snippets/about.md")]
#[doc = include_str!("../doc_snippets/order_warning.md")]
///
/// # Alternate Choices
///
/// If your keys are integers or enum variants, you should use the [`FacadeScalarMap`](crate::facade_maps::FacadeScalarMap) type instead.
/// If your keys are strings, you should use the [`FacadeStringMap`](crate::facade_maps::FacadeStringMap) type instead. Both of these will
/// deliver better performance since they are specifically optimized for those key types.
#[derive(Clone)]
pub struct FacadeOrderedMap<K, V> {
    map_impl: MapTypes<K, V>,
}

impl<K, V> FacadeOrderedMap<K, V>
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

impl<K, V> Default for FacadeOrderedMap<K, V> {
    fn default() -> Self {
        Self {
            map_impl: MapTypes::Scanning(OrderedScanMap::default()),
        }
    }
}

impl<K, V, Q> Map<K, V, Q> for FacadeOrderedMap<K, V>
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

impl<K, V, Q> MapQuery<K, V, Q> for FacadeOrderedMap<K, V>
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

impl<K, V> MapIteration<K, V> for FacadeOrderedMap<K, V> {
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

impl<K, V> Len for FacadeOrderedMap<K, V> {
    fn len(&self) -> usize {
        match &self.map_impl {
            MapTypes::BinarySearch(m) => m.len(),
            MapTypes::EytzingerSearch(m) => m.len(),
            MapTypes::Scanning(m) => m.len(),
        }
    }
}

impl<K, V, Q> Index<&Q> for FacadeOrderedMap<K, V>
where
    Q: ?Sized + Eq + Comparable<K>,
{
    type Output = V;

    fn index(&self, index: &Q) -> &Self::Output {
        self.get(index).expect("index should be valid")
    }
}

impl<'a, K, V> IntoIterator for &'a FacadeOrderedMap<K, V> {
    type Item = (&'a K, &'a V);
    type IntoIter = Iter<'a, K, V>;

    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}

impl<'a, K, V> IntoIterator for &'a mut FacadeOrderedMap<K, V> {
    type Item = (&'a K, &'a mut V);
    type IntoIter = IterMut<'a, K, V>;

    fn into_iter(self) -> Self::IntoIter {
        self.iter_mut()
    }
}

impl<K, V> IntoIterator for FacadeOrderedMap<K, V> {
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

impl<K, V, MT> PartialEq<MT> for FacadeOrderedMap<K, V>
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

impl<K, V> Eq for FacadeOrderedMap<K, V>
where
    K: Ord + Eq,
    V: Eq,
{
}

impl<K, V> Debug for FacadeOrderedMap<K, V>
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
