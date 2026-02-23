use crate::DefaultBuildHasher;
use crate::fz_maps::FzStringMap;
use crate::maps::decl_macros::len_trait_funcs;
use crate::sets::decl_macros::{debug_trait_funcs, partial_eq_trait_funcs, set_query_trait_funcs};
use crate::sets::{IntoIter, Iter};
use crate::traits::{Len, Set, SetExtras, SetIteration, SetOps, SetQuery};
use core::fmt::Debug;
use core::hash::{BuildHasher, Hash};
use core::ops::{BitAnd, BitOr, BitXor, Sub};
use equivalent::Equivalent;

#[cfg(not(feature = "std"))]
use alloc::vec::Vec;

#[cfg(feature = "serde")]
use {
    crate::sets::decl_macros::serialize_trait_funcs,
    core::fmt::Formatter,
    core::marker::PhantomData,
    serde::de::{SeqAccess, Visitor},
    serde::ser::SerializeSeq,
    serde::{Deserialize, Deserializer, Serialize, Serializer},
};

#[cfg(not(feature = "std"))]
use alloc::boxed::Box;

/// A set optimized for fast read access with string values.
///
#[doc = include_str!("../doc_snippets/about.md")]
#[doc = include_str!("../doc_snippets/hash_warning.md")]
#[derive(Clone)]
pub struct FzStringSet<T, BH = DefaultBuildHasher> {
    map: FzStringMap<T, (), BH>,
}

impl FzStringSet<Box<str>, DefaultBuildHasher> {
    /// Creates a new frozen set.
    #[doc = include_str!("../doc_snippets/duplicate_values.md")]
    #[must_use]
    pub fn new(entries: Vec<impl AsRef<str>>) -> Self {
        Self::with_hasher(entries, DefaultBuildHasher::default())
    }
}

impl<BH> FzStringSet<Box<str>, BH> {
    /// Creates a new frozen set which uses the given hash builder to hash values.
    #[doc = include_str!("../doc_snippets/duplicate_values.md")]
    #[must_use]
    pub fn with_hasher(entries: Vec<impl AsRef<str>>, bh: BH) -> Self
    where
        BH: BuildHasher + Clone + Send + Sync + 'static,
    {
        Self {
            map: FzStringMap::with_hasher(entries.into_iter().map(|x| (x, ())).collect(), bh),
        }
    }
}

impl<'a> FzStringSet<&'a str, DefaultBuildHasher> {
    /// Creates a new frozen set.
    #[doc = include_str!("../doc_snippets/duplicate_values.md")]
    #[must_use]
    pub fn new_for_str(entries: Vec<&'a str>) -> Self {
        Self::with_hasher_for_str(entries, DefaultBuildHasher::default())
    }
}

impl<'a, BH> FzStringSet<&'a str, BH> {
    /// Creates a new frozen set which uses the given hash builder to hash values.
    #[doc = include_str!("../doc_snippets/duplicate_values.md")]
    #[must_use]
    pub fn with_hasher_for_str(entries: Vec<&'a str>, bh: BH) -> Self
    where
        BH: BuildHasher + Clone + Send + Sync + 'static,
    {
        Self {
            map: FzStringMap::with_hasher_for_str(entries.into_iter().map(|x| (x, ())).collect(), bh),
        }
    }
}

impl<T, BH> FzStringSet<T, BH> {
    #[doc = include_str!("../doc_snippets/get_from_set.md")]
    #[inline]
    pub fn get(&self, value: impl AsRef<str>) -> Option<&T>
    where
        BH: BuildHasher,
        str: Equivalent<T>,
    {
        Some(self.map.get_key_value(value)?.0)
    }

    #[doc = include_str!("../doc_snippets/contains.md")]
    #[inline]
    #[must_use]
    pub fn contains(&self, value: impl AsRef<str>) -> bool
    where
        BH: BuildHasher,
        str: Equivalent<T>,
    {
        self.map.contains_key(value)
    }

    #[doc = include_str!("../doc_snippets/len.md")]
    #[inline]
    #[must_use]
    pub fn len(&self) -> usize {
        self.map.len()
    }

    #[doc = include_str!("../doc_snippets/is_empty.md")]
    #[inline]
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.map.is_empty()
    }

    #[doc = include_str!("../doc_snippets/iter.md")]
    #[must_use]
    pub fn iter(&self) -> Iter<'_, T> {
        Iter::new(self.map.iter())
    }

    fn into_iter(self) -> IntoIter<T> {
        IntoIter::new(self.map.into_iter())
    }
}

impl<BH> Default for FzStringSet<Box<str>, BH>
where
    BH: Default,
{
    fn default() -> Self {
        Self {
            map: FzStringMap::default(),
        }
    }
}

impl<BH> Default for FzStringSet<&str, BH>
where
    BH: Default,
{
    fn default() -> Self {
        Self {
            map: FzStringMap::default(),
        }
    }
}

impl<T, BH> From<FzStringMap<T, (), BH>> for FzStringSet<T, BH> {
    fn from(map: FzStringMap<T, (), BH>) -> Self {
        Self { map }
    }
}

impl<T, const N: usize, BH> From<[T; N]> for FzStringSet<Box<str>, BH>
where
    T: AsRef<str>,
    BH: BuildHasher + Default + Clone + Send + Sync + 'static,
{
    fn from(entries: [T; N]) -> Self {
        Self::from(FzStringMap::from_iter(entries.into_iter().map(|x| (x, ()))))
    }
}

impl<'a, const N: usize, BH> From<[&'a str; N]> for FzStringSet<&'a str, BH>
where
    BH: BuildHasher + Default + Clone + Send + Sync + 'static,
{
    fn from(entries: [&'a str; N]) -> Self {
        Self::from(FzStringMap::from_iter(entries.into_iter().map(|x| (x, ()))))
    }
}

impl<T, BH> FromIterator<T> for FzStringSet<Box<str>, BH>
where
    T: AsRef<str>,
    BH: BuildHasher + Default + Clone + Send + Sync + 'static,
{
    fn from_iter<IT: IntoIterator<Item = T>>(iter: IT) -> Self {
        Self::from(FzStringMap::from_iter(iter.into_iter().map(|x| (x, ()))))
    }
}

impl<'a, BH> FromIterator<&'a str> for FzStringSet<&'a str, BH>
where
    BH: BuildHasher + Default + Clone + Send + Sync + 'static,
{
    fn from_iter<IT: IntoIterator<Item = &'a str>>(iter: IT) -> Self {
        Self::from(FzStringMap::from_iter(iter.into_iter().map(|x| (x, ()))))
    }
}

impl<T, Q, BH> Set<T, Q> for FzStringSet<T, BH>
where
    Q: AsRef<str>,
    BH: BuildHasher,
    str: Equivalent<T>,
{
}

impl<T, Q, BH> SetExtras<T, Q> for FzStringSet<T, BH>
where
    Q: AsRef<str>,
    BH: BuildHasher,
    str: Equivalent<T>,
{
    #[inline]
    fn get(&self, value: &Q) -> Option<&T> {
        self.get(value)
    }
}

impl<T, Q, BH> SetQuery<Q> for FzStringSet<T, BH>
where
    Q: AsRef<str>,
    BH: BuildHasher,
    str: Equivalent<T>,
{
    set_query_trait_funcs!();
}

impl<T, BH> SetIteration<T> for FzStringSet<T, BH> {
    type Iterator<'a>
        = Iter<'a, T>
    where
        T: 'a,
        BH: 'a;

    fn iter(&self) -> Iter<'_, T> {
        self.iter()
    }
}

impl<T, BH> Len for FzStringSet<T, BH> {
    len_trait_funcs!();
}

impl<T, ST, BH> BitOr<&ST> for &FzStringSet<T, BH>
where
    T: Hash + Eq + Clone + AsRef<str>,
    ST: Set<T>,
    BH: BuildHasher + Default,
    str: Equivalent<T>,
{
    type Output = hashbrown::HashSet<T>;

    fn bitor(self, rhs: &ST) -> Self::Output {
        Self::Output::from_iter(self.union(rhs).cloned())
    }
}

impl<T, ST, BH> BitAnd<&ST> for &FzStringSet<T, BH>
where
    T: Hash + Eq + Clone + AsRef<str>,
    ST: Set<T>,
    BH: BuildHasher + Default,
    str: Equivalent<T>,
{
    type Output = hashbrown::HashSet<T>;

    fn bitand(self, rhs: &ST) -> Self::Output {
        Self::Output::from_iter(self.intersection(rhs).cloned())
    }
}

impl<T, ST, BH> BitXor<&ST> for &FzStringSet<T, BH>
where
    T: Hash + Eq + Clone + AsRef<str>,
    ST: Set<T>,
    BH: BuildHasher + Default,
    str: Equivalent<T>,
{
    type Output = hashbrown::HashSet<T>;

    fn bitxor(self, rhs: &ST) -> Self::Output {
        self.symmetric_difference(rhs).cloned().collect()
    }
}

impl<T, ST, BH> Sub<&ST> for &FzStringSet<T, BH>
where
    T: Hash + Eq + Clone + AsRef<str>,
    ST: Set<T>,
    BH: BuildHasher + Default,
    str: Equivalent<T>,
{
    type Output = hashbrown::HashSet<T>;

    fn sub(self, rhs: &ST) -> Self::Output {
        self.difference(rhs).cloned().collect()
    }
}

impl<T, BH> IntoIterator for FzStringSet<T, BH> {
    type Item = T;
    type IntoIter = IntoIter<T>;

    fn into_iter(self) -> Self::IntoIter {
        self.into_iter()
    }
}

impl<'a, T, BH> IntoIterator for &'a FzStringSet<T, BH> {
    type Item = &'a T;
    type IntoIter = Iter<'a, T>;

    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}

impl<T, ST, BH> PartialEq<ST> for FzStringSet<T, BH>
where
    ST: SetQuery<T>,
    BH: BuildHasher + Default,
{
    partial_eq_trait_funcs!();
}

impl<T, BH> Eq for FzStringSet<T, BH>
where
    T: AsRef<str>,
    BH: BuildHasher + Default,
    str: Equivalent<T>,
{
}

impl<T, BH> Debug for FzStringSet<T, BH>
where
    T: Debug,
{
    debug_trait_funcs!();
}

#[cfg(feature = "serde")]
impl<T, BH> Serialize for FzStringSet<T, BH>
where
    T: Serialize,
{
    serialize_trait_funcs!();
}

#[cfg(feature = "serde")]
impl<'de, BH> Deserialize<'de> for FzStringSet<Box<str>, BH>
where
    BH: BuildHasher + Default + Clone + Send + Sync + 'static,
{
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer.deserialize_seq(StrSetVisitor { marker: PhantomData })
    }
}

#[cfg(feature = "serde")]
struct StrSetVisitor<BH> {
    marker: PhantomData<BH>,
}

#[cfg(feature = "serde")]
impl<'de, BH> Visitor<'de> for StrSetVisitor<BH>
where
    BH: BuildHasher + Default + Clone + Send + Sync + 'static,
{
    type Value = FzStringSet<Box<str>, BH>;

    fn expecting(&self, formatter: &mut Formatter) -> core::fmt::Result {
        formatter.write_str("a set with string values")
    }

    fn visit_seq<M>(self, mut seq: M) -> Result<Self::Value, M::Error>
    where
        M: SeqAccess<'de>,
    {
        let mut v: Vec<(&str, ())> = Vec::with_capacity(seq.size_hint().unwrap_or(0));
        while let Some(x) = seq.next_element()? {
            v.push((x, ()));
        }

        Ok(FzStringSet::from(FzStringMap::with_hasher(v, BH::default())))
    }
}
