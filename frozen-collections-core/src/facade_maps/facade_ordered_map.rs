use crate::maps::{
    BinarySearchMap, IntoIter, IntoKeys, IntoValues, Iter, IterMut, Keys, OrderedScanMap, Values,
    ValuesMut,
};
use crate::traits::{Len, Map, MapIterator};
use crate::utils::dedup_by_keep_last;
use alloc::vec::Vec;
use core::borrow::Borrow;
use core::fmt::{Debug, Formatter, Result};
use core::ops::Index;

#[derive(Clone)]
enum MapTypes<K, V> {
    BinarySearch(BinarySearchMap<K, V>),
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
#[allow(clippy::module_name_repetitions)]
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
            } else {
                MapTypes::BinarySearch(BinarySearchMap::new_raw(entries))
            },
        }
    }
}

impl<K, V> FacadeOrderedMap<K, V> {
    #[doc = include_str!("../doc_snippets/get_method.md")]
    #[inline(always)]
    #[must_use]
    pub fn get<Q>(&self, key: &Q) -> Option<&V>
    where
        K: Borrow<Q>,
        Q: ?Sized + Ord + Eq,
    {
        match &self.map_impl {
            MapTypes::BinarySearch(m) => m.get(key),
            MapTypes::Scanning(m) => m.get(key),
        }
    }

    #[doc = include_str!("../doc_snippets/get_key_value_method.md")]
    #[inline]
    #[must_use]
    pub fn get_key_value<Q>(&self, key: &Q) -> Option<(&K, &V)>
    where
        K: Borrow<Q>,
        Q: ?Sized + Ord + Eq,
    {
        match &self.map_impl {
            MapTypes::BinarySearch(m) => m.get_key_value(key),
            MapTypes::Scanning(m) => m.get_key_value(key),
        }
    }

    #[doc = include_str!("../doc_snippets/get_mut_method.md")]
    #[inline]
    #[must_use]
    pub fn get_mut<Q>(&mut self, key: &Q) -> Option<&mut V>
    where
        K: Borrow<Q>,
        Q: ?Sized + Ord + Eq,
    {
        match &mut self.map_impl {
            MapTypes::BinarySearch(m) => m.get_mut(key),
            MapTypes::Scanning(m) => m.get_mut(key),
        }
    }

    #[doc = include_str!("../doc_snippets/get_many_mut_method.md")]
    #[must_use]
    pub fn get_many_mut<const N: usize, Q>(&mut self, keys: [&Q; N]) -> Option<[&mut V; N]>
    where
        K: Borrow<Q>,
        Q: ?Sized + Ord + Eq,
    {
        match &mut self.map_impl {
            MapTypes::BinarySearch(m) => m.get_many_mut(keys),
            MapTypes::Scanning(m) => m.get_many_mut(keys),
        }
    }

    #[doc = include_str!("../doc_snippets/contains_key_method.md")]
    #[inline]
    #[must_use]
    pub fn contains_key<Q>(&self, key: &Q) -> bool
    where
        K: Borrow<Q>,
        Q: ?Sized + Ord + Eq,
    {
        self.get(key).is_some()
    }
}

impl<K, V> Len for FacadeOrderedMap<K, V> {
    fn len(&self) -> usize {
        match &self.map_impl {
            MapTypes::BinarySearch(m) => m.len(),
            MapTypes::Scanning(m) => m.len(),
        }
    }
}

impl<K, V, Q> Index<&Q> for FacadeOrderedMap<K, V>
where
    K: Borrow<Q>,
    Q: ?Sized + Ord + Eq,
{
    type Output = V;

    fn index(&self, index: &Q) -> &Self::Output {
        self.get(index).expect("index should be valid")
    }
}

impl<K, V> Default for FacadeOrderedMap<K, V> {
    fn default() -> Self {
        Self {
            map_impl: MapTypes::Scanning(OrderedScanMap::default()),
        }
    }
}

impl<K, V> Debug for FacadeOrderedMap<K, V>
where
    K: Debug,
    V: Debug,
{
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        match &self.map_impl {
            MapTypes::BinarySearch(m) => m.fmt(f),
            MapTypes::Scanning(m) => m.fmt(f),
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
            MapTypes::Scanning(m) => m.into_iter(),
        }
    }
}

impl<K, V> MapIterator<K, V> for FacadeOrderedMap<K, V> {
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
            MapTypes::Scanning(m) => m.iter(),
        }
    }

    fn keys(&self) -> Self::KeyIterator<'_> {
        match &self.map_impl {
            MapTypes::BinarySearch(m) => m.keys(),
            MapTypes::Scanning(m) => m.keys(),
        }
    }

    fn values(&self) -> Self::ValueIterator<'_> {
        match &self.map_impl {
            MapTypes::BinarySearch(m) => m.values(),
            MapTypes::Scanning(m) => m.values(),
        }
    }

    fn into_keys(self) -> Self::IntoKeyIterator {
        match self.map_impl {
            MapTypes::BinarySearch(m) => m.into_keys(),
            MapTypes::Scanning(m) => m.into_keys(),
        }
    }

    fn into_values(self) -> Self::IntoValueIterator {
        match self.map_impl {
            MapTypes::BinarySearch(m) => m.into_values(),
            MapTypes::Scanning(m) => m.into_values(),
        }
    }

    fn iter_mut(&mut self) -> Self::MutIterator<'_> {
        match &mut self.map_impl {
            MapTypes::BinarySearch(m) => m.iter_mut(),
            MapTypes::Scanning(m) => m.iter_mut(),
        }
    }

    fn values_mut(&mut self) -> Self::ValueMutIterator<'_> {
        match &mut self.map_impl {
            MapTypes::BinarySearch(m) => m.values_mut(),
            MapTypes::Scanning(m) => m.values_mut(),
        }
    }
}

impl<K, V> Map<K, V> for FacadeOrderedMap<K, V>
where
    K: Ord + Eq,
{
    #[inline]
    fn contains_key(&self, key: &K) -> bool {
        self.contains_key(key)
    }

    #[inline]
    fn get(&self, key: &K) -> Option<&V> {
        self.get(key)
    }

    #[inline]
    fn get_key_value(&self, key: &K) -> Option<(&K, &V)> {
        self.get_key_value(key)
    }

    #[inline]
    fn get_mut(&mut self, key: &K) -> Option<&mut V> {
        self.get_mut(key)
    }

    #[inline]
    fn get_many_mut<const N: usize>(&mut self, keys: [&K; N]) -> Option<[&mut V; N]> {
        self.get_many_mut(keys)
    }
}
