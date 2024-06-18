use alloc::string::String;
use alloc::vec::Vec;
use core::fmt::{Debug, Formatter, Result};
use core::hash::BuildHasher;
use core::ops::Index;

use ahash::RandomState;

use crate::analyzers::{analyze_slice_keys, SliceKeyAnalysisResult};
use crate::hashers::{BridgeHasher, LeftRangeHasher, RightRangeHasher};
use crate::maps::{
    HashMap, IntoIter, IntoKeys, IntoValues, Iter, IterMut, Keys, Values, ValuesMut,
};
use crate::traits::{LargeCollection, Len, Map, MapIterator};
use crate::utils::dedup_by_keep_last;

#[derive(Clone)]
enum MapTypes<V, BH> {
    LeftRange(HashMap<String, V, LargeCollection, LeftRangeHasher<BH>>),
    RightRange(HashMap<String, V, LargeCollection, RightRangeHasher<BH>>),
    Hash(HashMap<String, V, LargeCollection, BridgeHasher<BH>>),
}

/// A map optimized for fast read access using string keys.
///
#[doc = include_str!("../doc_snippets/type_compat_warning.md")]
#[doc = include_str!("../doc_snippets/about.md")]
#[derive(Clone)]
#[allow(clippy::module_name_repetitions)]
pub struct FacadeStringMap<V, BH = RandomState> {
    map_impl: MapTypes<V, BH>,
}

impl<V, BH> FacadeStringMap<V, BH>
where
    BH: BuildHasher,
{
    /// Creates a frozen map which will use the given hash builder to hash
    /// keys.
    #[must_use]
    #[allow(clippy::missing_panics_doc)]
    pub fn new(mut entries: Vec<(String, V)>, bh: BH) -> Self {
        entries.sort_by(|x, y| x.0.cmp(&y.0));
        dedup_by_keep_last(&mut entries, |x, y| x.0.eq(&y.0));

        Self {
            map_impl: {
                match analyze_slice_keys(entries.iter().map(|x| x.0.as_bytes()), &bh) {
                    SliceKeyAnalysisResult::General | SliceKeyAnalysisResult::Length => {
                        let h = BridgeHasher::new(bh);
                        MapTypes::Hash(HashMap::new_half_baked(entries, h).unwrap())
                    }

                    SliceKeyAnalysisResult::LeftHandSubslice(range) => {
                        let h = LeftRangeHasher::new(bh, range);
                        MapTypes::LeftRange(HashMap::new_half_baked(entries, h).unwrap())
                    }

                    SliceKeyAnalysisResult::RightHandSubslice(range) => {
                        let h = RightRangeHasher::new(bh, range);
                        MapTypes::RightRange(HashMap::new_half_baked(entries, h).unwrap())
                    }
                }
            },
        }
    }

    #[doc = include_str!("../doc_snippets/get_method.md")]
    #[inline(always)]
    #[must_use]
    pub fn get(&self, key: &str) -> Option<&V> {
        match &self.map_impl {
            MapTypes::LeftRange(m) => m.get(key),
            MapTypes::RightRange(m) => m.get(key),
            MapTypes::Hash(m) => m.get(key),
        }
    }

    #[doc = include_str!("../doc_snippets/get_key_value_method.md")]
    #[inline]
    #[must_use]
    pub fn get_key_value(&self, key: &str) -> Option<(&String, &V)> {
        match &self.map_impl {
            MapTypes::LeftRange(m) => m.get_key_value(key),
            MapTypes::RightRange(m) => m.get_key_value(key),
            MapTypes::Hash(m) => m.get_key_value(key),
        }
    }

    #[doc = include_str!("../doc_snippets/get_mut_method.md")]
    #[inline]
    #[must_use]
    pub fn get_mut(&mut self, key: &str) -> Option<&mut V> {
        match &mut self.map_impl {
            MapTypes::LeftRange(m) => m.get_mut(key),
            MapTypes::RightRange(m) => m.get_mut(key),
            MapTypes::Hash(m) => m.get_mut(key),
        }
    }

    #[doc = include_str!("../doc_snippets/get_many_mut_method.md")]
    #[must_use]
    pub fn get_many_mut<const N: usize>(&mut self, keys: [&str; N]) -> Option<[&mut V; N]> {
        match &mut self.map_impl {
            MapTypes::LeftRange(m) => m.get_many_mut(keys),
            MapTypes::RightRange(m) => m.get_many_mut(keys),
            MapTypes::Hash(m) => m.get_many_mut(keys),
        }
    }

    #[doc = include_str!("../doc_snippets/contains_key_method.md")]
    #[inline]
    #[must_use]
    pub fn contains_key(&self, key: &str) -> bool {
        self.get(key).is_some()
    }
}

impl<V, BH> Len for FacadeStringMap<V, BH> {
    fn len(&self) -> usize {
        match &self.map_impl {
            MapTypes::LeftRange(m) => m.len(),
            MapTypes::RightRange(m) => m.len(),
            MapTypes::Hash(m) => m.len(),
        }
    }
}

impl<V, BH> Index<&String> for FacadeStringMap<V, BH>
where
    BH: BuildHasher,
{
    type Output = V;

    fn index(&self, index: &String) -> &Self::Output {
        self.get(index).expect("index should be valid")
    }
}

impl<V, BH> Default for FacadeStringMap<V, BH>
where
    BH: Default,
{
    fn default() -> Self {
        Self {
            map_impl: MapTypes::Hash(HashMap::default()),
        }
    }
}

impl<V, BH> Debug for FacadeStringMap<V, BH>
where
    V: Debug,
{
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        match &self.map_impl {
            MapTypes::LeftRange(m) => m.fmt(f),
            MapTypes::RightRange(m) => m.fmt(f),
            MapTypes::Hash(m) => m.fmt(f),
        }
    }
}

impl<V, MT, BH> PartialEq<MT> for FacadeStringMap<V, BH>
where
    V: PartialEq,
    MT: Map<String, V>,
    BH: BuildHasher,
{
    fn eq(&self, other: &MT) -> bool {
        if self.len() != other.len() {
            return false;
        }

        self.iter()
            .all(|(key, value)| other.get(key).map_or(false, |v| *value == *v))
    }
}

impl<V, BH> Eq for FacadeStringMap<V, BH>
where
    V: Eq,
    BH: BuildHasher,
{
}

impl<'a, V, BH> IntoIterator for &'a FacadeStringMap<V, BH> {
    type Item = (&'a String, &'a V);
    type IntoIter = Iter<'a, String, V>;

    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}

impl<'a, V, BH> IntoIterator for &'a mut FacadeStringMap<V, BH> {
    type Item = (&'a String, &'a mut V);
    type IntoIter = IterMut<'a, String, V>;

    fn into_iter(self) -> Self::IntoIter {
        self.iter_mut()
    }
}

impl<V, BH> IntoIterator for FacadeStringMap<V, BH> {
    type Item = (String, V);
    type IntoIter = IntoIter<String, V>;

    fn into_iter(self) -> Self::IntoIter {
        match self.map_impl {
            MapTypes::LeftRange(m) => m.into_iter(),
            MapTypes::RightRange(m) => m.into_iter(),
            MapTypes::Hash(m) => m.into_iter(),
        }
    }
}

impl<V, BH> MapIterator<String, V> for FacadeStringMap<V, BH> {
    type Iterator<'a>
        = Iter<'a, String, V>
    where
        V: 'a,
        BH: 'a;

    type KeyIterator<'a>
        = Keys<'a, String, V>
    where
        V: 'a,
        BH: 'a;

    type ValueIterator<'a>
        = Values<'a, String, V>
    where
        V: 'a,
        BH: 'a;

    type IntoKeyIterator = IntoKeys<String, V>;
    type IntoValueIterator = IntoValues<String, V>;

    type MutIterator<'a>
        = IterMut<'a, String, V>
    where
        V: 'a,
        BH: 'a;

    type ValueMutIterator<'a>
        = ValuesMut<'a, String, V>
    where
        V: 'a,
        BH: 'a;

    fn iter(&self) -> Self::Iterator<'_> {
        match &self.map_impl {
            MapTypes::LeftRange(m) => m.iter(),
            MapTypes::RightRange(m) => m.iter(),
            MapTypes::Hash(m) => m.iter(),
        }
    }

    fn keys(&self) -> Self::KeyIterator<'_> {
        match &self.map_impl {
            MapTypes::LeftRange(m) => m.keys(),
            MapTypes::RightRange(m) => m.keys(),
            MapTypes::Hash(m) => m.keys(),
        }
    }

    fn values(&self) -> Self::ValueIterator<'_> {
        match &self.map_impl {
            MapTypes::LeftRange(m) => m.values(),
            MapTypes::RightRange(m) => m.values(),
            MapTypes::Hash(m) => m.values(),
        }
    }

    fn into_keys(self) -> Self::IntoKeyIterator {
        match self.map_impl {
            MapTypes::LeftRange(m) => m.into_keys(),
            MapTypes::RightRange(m) => m.into_keys(),
            MapTypes::Hash(m) => m.into_keys(),
        }
    }

    fn into_values(self) -> Self::IntoValueIterator {
        match self.map_impl {
            MapTypes::LeftRange(m) => m.into_values(),
            MapTypes::RightRange(m) => m.into_values(),
            MapTypes::Hash(m) => m.into_values(),
        }
    }

    fn iter_mut(&mut self) -> Self::MutIterator<'_> {
        match &mut self.map_impl {
            MapTypes::LeftRange(m) => m.iter_mut(),
            MapTypes::RightRange(m) => m.iter_mut(),
            MapTypes::Hash(m) => m.iter_mut(),
        }
    }

    fn values_mut(&mut self) -> Self::ValueMutIterator<'_> {
        match &mut self.map_impl {
            MapTypes::LeftRange(m) => m.values_mut(),
            MapTypes::RightRange(m) => m.values_mut(),
            MapTypes::Hash(m) => m.values_mut(),
        }
    }
}

impl<V, BH> Map<String, V> for FacadeStringMap<V, BH>
where
    BH: BuildHasher,
{
    #[inline]
    fn contains_key(&self, key: &String) -> bool {
        self.contains_key(key)
    }

    #[inline]
    fn get(&self, key: &String) -> Option<&V> {
        self.get(key)
    }

    #[inline]
    fn get_key_value(&self, key: &String) -> Option<(&String, &V)> {
        self.get_key_value(key)
    }

    #[inline]
    fn get_mut(&mut self, key: &String) -> Option<&mut V> {
        self.get_mut(key)
    }

    #[inline]
    fn get_many_mut<const N: usize>(&mut self, keys: [&String; N]) -> Option<[&mut V; N]> {
        self.get_many_mut(keys.map(String::as_str))
    }
}
