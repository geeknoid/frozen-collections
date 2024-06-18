use alloc::vec::Vec;
use core::borrow::Borrow;
use core::fmt::{Debug, Formatter, Result};
use core::ops::Index;

use crate::hashers::BridgeHasher;
use crate::maps::{
    HashMap, IntoIter, IntoKeys, IntoValues, Iter, IterMut, Keys, ScanMap, Values, ValuesMut,
};
use crate::traits::{Hasher, LargeCollection, Len, Map, MapIterator};
use crate::utils::dedup_by_hash_keep_last;

#[derive(Clone)]
enum MapTypes<K, V, H> {
    Hash(HashMap<K, V, LargeCollection, H>),
    Scanning(ScanMap<K, V>),
}

/// A map optimized for fast read access with hashable keys.
///
#[doc = include_str!("../doc_snippets/type_compat_warning.md")]
#[doc = include_str!("../doc_snippets/about.md")]
#[doc = include_str!("../doc_snippets/hash_warning.md")]
///
/// # Alternate Choices
///
/// If your keys are integers or enum variants, you should use the [`FacadeScalarMap`](crate::facade_maps::FacadeScalarMap) type instead.
/// If your keys are strings, you should use the [`FacadeStringMap`](crate::facade_maps::FacadeStringMap) type instead. Both of these will
/// deliver better performance since they are specifically optimized for those key types.
#[derive(Clone)]
#[allow(clippy::module_name_repetitions)]
pub struct FacadeHashMap<K, V, H = BridgeHasher> {
    map_impl: MapTypes<K, V, H>,
}

impl<K, V, H> FacadeHashMap<K, V, H>
where
    K: Eq,
    H: Hasher<K>,
{
    /// Creates a frozen map which uses the given hash builder to hash keys.
    #[must_use]
    #[allow(clippy::missing_panics_doc)]
    pub fn new(mut entries: Vec<(K, V)>, hasher: H) -> Self {
        dedup_by_hash_keep_last(&mut entries, &hasher);

        Self {
            map_impl: if entries.len() < 3 {
                MapTypes::Scanning(ScanMap::new_raw(entries))
            } else {
                MapTypes::Hash(HashMap::new_half_baked(entries, hasher).unwrap())
            },
        }
    }
}

impl<K, V, H> FacadeHashMap<K, V, H> {
    #[doc = include_str!("../doc_snippets/get_method.md")]
    #[inline(always)]
    #[must_use]
    pub fn get<Q>(&self, key: &Q) -> Option<&V>
    where
        K: Borrow<Q>,
        Q: ?Sized + Eq,
        H: Hasher<Q>,
    {
        match &self.map_impl {
            MapTypes::Hash(m) => m.get(key),
            MapTypes::Scanning(m) => m.get(key),
        }
    }

    #[doc = include_str!("../doc_snippets/get_key_value_method.md")]
    #[inline]
    #[must_use]
    pub fn get_key_value<Q>(&self, key: &Q) -> Option<(&K, &V)>
    where
        K: Borrow<Q>,
        Q: ?Sized + Eq,
        H: Hasher<Q>,
    {
        match &self.map_impl {
            MapTypes::Hash(m) => m.get_key_value(key),
            MapTypes::Scanning(m) => m.get_key_value(key),
        }
    }

    #[doc = include_str!("../doc_snippets/get_mut_method.md")]
    #[inline]
    #[must_use]
    pub fn get_mut<Q>(&mut self, key: &Q) -> Option<&mut V>
    where
        K: Borrow<Q>,
        Q: ?Sized + Eq,
        H: Hasher<Q>,
    {
        match &mut self.map_impl {
            MapTypes::Hash(m) => m.get_mut(key),
            MapTypes::Scanning(m) => m.get_mut(key),
        }
    }

    #[doc = include_str!("../doc_snippets/get_many_mut_method.md")]
    #[must_use]
    pub fn get_many_mut<const N: usize, Q>(&mut self, keys: [&Q; N]) -> Option<[&mut V; N]>
    where
        K: Borrow<Q>,
        Q: ?Sized + Eq,
        H: Hasher<Q>,
    {
        match &mut self.map_impl {
            MapTypes::Hash(m) => m.get_many_mut(keys),
            MapTypes::Scanning(m) => m.get_many_mut(keys),
        }
    }

    #[doc = include_str!("../doc_snippets/contains_key_method.md")]
    #[inline]
    #[must_use]
    pub fn contains_key<Q>(&self, key: &Q) -> bool
    where
        K: Borrow<Q>,
        Q: ?Sized + Eq,
        H: Hasher<Q>,
    {
        self.get(key).is_some()
    }
}

impl<K, V, H> Len for FacadeHashMap<K, V, H> {
    fn len(&self) -> usize {
        match &self.map_impl {
            MapTypes::Hash(m) => m.len(),
            MapTypes::Scanning(m) => m.len(),
        }
    }
}

impl<K, V, Q, H> Index<&Q> for FacadeHashMap<K, V, H>
where
    K: Borrow<Q>,
    Q: ?Sized + Eq,
    H: Hasher<Q>,
{
    type Output = V;

    fn index(&self, index: &Q) -> &Self::Output {
        self.get(index).expect("index should be valid")
    }
}

impl<K, V, H> Default for FacadeHashMap<K, V, H>
where
    H: Default,
{
    fn default() -> Self {
        Self {
            map_impl: MapTypes::Hash(HashMap::<K, V, LargeCollection, H>::default()),
        }
    }
}

impl<K, V, H> Debug for FacadeHashMap<K, V, H>
where
    K: Debug,
    V: Debug,
{
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        match &self.map_impl {
            MapTypes::Hash(m) => m.fmt(f),
            MapTypes::Scanning(m) => m.fmt(f),
        }
    }
}

impl<K, V, MT, H> PartialEq<MT> for FacadeHashMap<K, V, H>
where
    K: Eq,
    V: PartialEq,
    MT: Map<K, V>,
    H: Hasher<K>,
{
    fn eq(&self, other: &MT) -> bool {
        if self.len() != other.len() {
            return false;
        }

        self.iter()
            .all(|(key, value)| other.get(key).map_or(false, |v| *value == *v))
    }
}

impl<K, V, H> Eq for FacadeHashMap<K, V, H>
where
    K: Eq,
    V: Eq,
    H: Hasher<K>,
{
}

impl<'a, K, V, H> IntoIterator for &'a FacadeHashMap<K, V, H> {
    type Item = (&'a K, &'a V);
    type IntoIter = Iter<'a, K, V>;

    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}

impl<'a, K, V, H> IntoIterator for &'a mut FacadeHashMap<K, V, H> {
    type Item = (&'a K, &'a mut V);
    type IntoIter = IterMut<'a, K, V>;

    fn into_iter(self) -> Self::IntoIter {
        self.iter_mut()
    }
}

impl<K, V, H> IntoIterator for FacadeHashMap<K, V, H> {
    type Item = (K, V);
    type IntoIter = IntoIter<K, V>;

    fn into_iter(self) -> Self::IntoIter {
        match self.map_impl {
            MapTypes::Hash(m) => m.into_iter(),
            MapTypes::Scanning(m) => m.into_iter(),
        }
    }
}

impl<K, V, H> MapIterator<K, V> for FacadeHashMap<K, V, H> {
    type Iterator<'a>
        = Iter<'a, K, V>
    where
        K: 'a,
        V: 'a,
        H: 'a;

    type KeyIterator<'a>
        = Keys<'a, K, V>
    where
        K: 'a,
        V: 'a,
        H: 'a;

    type ValueIterator<'a>
        = Values<'a, K, V>
    where
        K: 'a,
        V: 'a,
        H: 'a;

    type IntoKeyIterator = IntoKeys<K, V>;
    type IntoValueIterator = IntoValues<K, V>;

    type MutIterator<'a>
        = IterMut<'a, K, V>
    where
        K: 'a,
        V: 'a,
        H: 'a;

    type ValueMutIterator<'a>
        = ValuesMut<'a, K, V>
    where
        K: 'a,
        V: 'a,
        H: 'a;

    fn iter(&self) -> Self::Iterator<'_> {
        match &self.map_impl {
            MapTypes::Hash(m) => m.iter(),
            MapTypes::Scanning(m) => m.iter(),
        }
    }

    fn keys(&self) -> Self::KeyIterator<'_> {
        match &self.map_impl {
            MapTypes::Hash(m) => m.keys(),
            MapTypes::Scanning(m) => m.keys(),
        }
    }

    fn values(&self) -> Self::ValueIterator<'_> {
        match &self.map_impl {
            MapTypes::Hash(m) => m.values(),
            MapTypes::Scanning(m) => m.values(),
        }
    }

    fn into_keys(self) -> Self::IntoKeyIterator {
        match self.map_impl {
            MapTypes::Hash(m) => m.into_keys(),
            MapTypes::Scanning(m) => m.into_keys(),
        }
    }

    fn into_values(self) -> Self::IntoValueIterator {
        match self.map_impl {
            MapTypes::Hash(m) => m.into_values(),
            MapTypes::Scanning(m) => m.into_values(),
        }
    }

    fn iter_mut(&mut self) -> Self::MutIterator<'_> {
        match &mut self.map_impl {
            MapTypes::Hash(m) => m.iter_mut(),
            MapTypes::Scanning(m) => m.iter_mut(),
        }
    }

    fn values_mut(&mut self) -> Self::ValueMutIterator<'_> {
        match &mut self.map_impl {
            MapTypes::Hash(m) => m.values_mut(),
            MapTypes::Scanning(m) => m.values_mut(),
        }
    }
}

impl<K, V, H> Map<K, V> for FacadeHashMap<K, V, H>
where
    K: Eq,
    H: Hasher<K>,
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
