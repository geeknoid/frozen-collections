use crate::DefaultBuildHasher;
use crate::fz_maps::FzStringMap;
use crate::maps::decl_macros::len_trait_funcs;
use crate::sets::decl_macros::{debug_trait_funcs, partial_eq_trait_funcs};
use crate::sets::{IntoIter, Iter};
use crate::traits::{Len, Set, SetExtras, SetIteration, SetOps, SetQuery};
use core::fmt::Debug;
use core::hash::BuildHasher;
use core::ops::{BitAnd, BitOr, BitXor, Sub};

#[cfg(not(feature = "std"))]
use {alloc::boxed::Box, alloc::vec::Vec};

#[cfg(feature = "serde")]
use {
    crate::sets::decl_macros::serialize_trait_funcs,
    core::fmt::Formatter,
    core::marker::PhantomData,
    serde::de::{SeqAccess, Visitor},
    serde::ser::SerializeSeq,
    serde::{Deserialize, Deserializer, Serialize, Serializer},
};

/// A set optimized for fast read access with string values.
///
#[doc = include_str!("../doc_snippets/about.md")]
#[doc = include_str!("../doc_snippets/hash_warning.md")]
///
/// # Alternate Choices
///
/// If your values are known at compile time, consider using the various `fz_*_set` macros instead of
/// this type as they generally perform better.
#[derive(Clone)]
pub struct FzStringSet<K, BH = DefaultBuildHasher> {
    map: FzStringMap<K, (), BH>,
}

impl FzStringSet<Box<str>, DefaultBuildHasher> {
    /// Creates a new frozen set.
    #[must_use]
    pub fn new(entries: Vec<impl AsRef<str>>) -> Self {
        Self::with_hasher(entries, DefaultBuildHasher::default())
    }
}

impl<BH> FzStringSet<Box<str>, BH> {
    /// Creates a new frozen set which uses the given hash builder to hash values.
    #[must_use]
    pub fn with_hasher(entries: Vec<impl AsRef<str>>, bh: BH) -> Self
    where
        BH: BuildHasher,
    {
        Self {
            map: FzStringMap::with_hasher(entries.into_iter().map(|x| (x, ())).collect(), bh),
        }
    }

    #[doc = include_str!("../doc_snippets/get_from_set.md")]
    #[inline]
    #[expect(clippy::borrowed_box, reason = "By design")]
    pub fn get(&self, value: impl AsRef<str>) -> Option<&Box<str>>
    where
        BH: BuildHasher,
    {
        Some(self.map.get_key_value(value)?.0)
    }

    #[doc = include_str!("../doc_snippets/contains.md")]
    #[inline]
    #[must_use]
    pub fn contains(&self, value: impl AsRef<str>) -> bool
    where
        BH: BuildHasher,
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
    pub fn iter(&self) -> Iter<'_, Box<str>> {
        Iter::new(self.map.iter())
    }

    fn into_iter(self) -> IntoIter<Box<str>> {
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

impl<BH> From<FzStringMap<Box<str>, (), BH>> for FzStringSet<Box<str>, BH> {
    fn from(map: FzStringMap<Box<str>, (), BH>) -> Self {
        Self { map }
    }
}

impl<T, const N: usize, BH> From<[T; N]> for FzStringSet<Box<str>, BH>
where
    T: AsRef<str>,
    BH: BuildHasher + Default,
{
    fn from(entries: [T; N]) -> Self {
        Self::from(FzStringMap::from_iter(entries.into_iter().map(|x| (x, ()))))
    }
}

impl<T, BH> FromIterator<T> for FzStringSet<Box<str>, BH>
where
    T: AsRef<str>,
    BH: BuildHasher + Default,
{
    fn from_iter<IT: IntoIterator<Item = T>>(iter: IT) -> Self {
        Self::from(FzStringMap::from_iter(iter.into_iter().map(|x| (x, ()))))
    }
}

impl<Q, BH> Set<Box<str>, Q> for FzStringSet<Box<str>, BH>
where
    Q: AsRef<str>,
    BH: BuildHasher,
{
}

impl<Q, BH> SetExtras<Box<str>, Q> for FzStringSet<Box<str>, BH>
where
    Q: AsRef<str>,
    BH: BuildHasher,
{
    #[inline]
    fn get(&self, value: &Q) -> Option<&Box<str>> {
        self.get(value)
    }
}

impl<Q, BH> SetQuery<Q> for FzStringSet<Box<str>, BH>
where
    Q: AsRef<str>,
    BH: BuildHasher,
{
    #[inline]
    fn contains(&self, value: &Q) -> bool {
        self.contains(value)
    }
}

impl<BH> SetIteration<Box<str>> for FzStringSet<Box<str>, BH> {
    type Iterator<'a>
        = Iter<'a, Box<str>>
    where
        BH: 'a;

    fn iter(&self) -> Iter<'_, Box<str>> {
        self.iter()
    }
}

impl<BH> Len for FzStringSet<Box<str>, BH> {
    len_trait_funcs!();
}

impl<ST, BH> BitOr<&ST> for &FzStringSet<Box<str>, BH>
where
    ST: Set<Box<str>>,
    BH: BuildHasher + Default,
{
    type Output = hashbrown::HashSet<Box<str>>;

    fn bitor(self, rhs: &ST) -> Self::Output {
        Self::Output::from_iter(self.union(rhs).cloned())
    }
}

impl<ST, BH> BitAnd<&ST> for &FzStringSet<Box<str>, BH>
where
    ST: Set<Box<str>>,
    BH: BuildHasher + Default,
{
    type Output = hashbrown::HashSet<Box<str>>;

    fn bitand(self, rhs: &ST) -> Self::Output {
        Self::Output::from_iter(self.intersection(rhs).cloned())
    }
}

impl<ST, BH> BitXor<&ST> for &FzStringSet<Box<str>, BH>
where
    ST: Set<Box<str>>,
    BH: BuildHasher + Default,
{
    type Output = hashbrown::HashSet<Box<str>>;

    fn bitxor(self, rhs: &ST) -> Self::Output {
        self.symmetric_difference(rhs).cloned().collect()
    }
}

impl<ST, BH> Sub<&ST> for &FzStringSet<Box<str>, BH>
where
    ST: Set<Box<str>>,
    BH: BuildHasher + Default,
{
    type Output = hashbrown::HashSet<Box<str>>;

    fn sub(self, rhs: &ST) -> Self::Output {
        self.difference(rhs).cloned().collect()
    }
}

impl<BH> IntoIterator for FzStringSet<Box<str>, BH> {
    type Item = Box<str>;
    type IntoIter = IntoIter<Box<str>>;

    fn into_iter(self) -> Self::IntoIter {
        self.into_iter()
    }
}

impl<'a, BH> IntoIterator for &'a FzStringSet<Box<str>, BH> {
    type Item = &'a Box<str>;
    type IntoIter = Iter<'a, Box<str>>;

    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}

impl<ST, BH> PartialEq<ST> for FzStringSet<Box<str>, BH>
where
    ST: SetQuery<Box<str>>,
    BH: BuildHasher + Default,
{
    partial_eq_trait_funcs!();
}

impl<BH> Eq for FzStringSet<Box<str>, BH> where BH: BuildHasher + Default {}

impl<BH> Debug for FzStringSet<Box<str>, BH> {
    debug_trait_funcs!();
}

#[cfg(feature = "serde")]
impl Serialize for FzStringSet<Box<str>> {
    serialize_trait_funcs!();
}

#[cfg(feature = "serde")]
impl<'de, BH> Deserialize<'de> for FzStringSet<Box<str>, BH>
where
    BH: BuildHasher + Default,
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
    BH: BuildHasher + Default,
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
