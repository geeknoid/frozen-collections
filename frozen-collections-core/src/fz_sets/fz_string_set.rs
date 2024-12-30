use crate::fz_maps::FzStringMap;
use crate::hashers::{LeftRangeHasher, RightRangeHasher};
use crate::sets::decl_macros::{
    bitand_fn, bitor_fn, bitxor_fn, debug_fn, into_iter_fn, into_iter_ref_fn, partial_eq_fn,
    set_iteration_funcs, sub_fn,
};
use crate::sets::{IntoIter, Iter};
use crate::traits::{Hasher, Len, MapIteration, MapQuery, Set, SetIteration, SetOps, SetQuery};
use crate::DefaultHashBuilder;
use alloc::vec::Vec;
use core::fmt::Debug;
use core::hash::BuildHasher;
use core::hash::Hash;
use core::iter::FromIterator;
use core::ops::{BitAnd, BitOr, BitXor, Sub};
use equivalent::Equivalent;
use foldhash::fast::RandomState;
#[cfg(feature = "serde")]
use {
    crate::sets::decl_macros::serialize_fn,
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
pub struct FzStringSet<T, BH = DefaultHashBuilder> {
    map: FzStringMap<T, (), BH>,
}

impl<'a> FzStringSet<&'a str, DefaultHashBuilder> {
    /// Creates a new frozen set.
    #[must_use]
    pub fn new(entries: Vec<&'a str>) -> Self {
        Self::with_hasher(entries, RandomState::default())
    }
}

impl<'a, BH> FzStringSet<&'a str, BH>
where
    BH: BuildHasher,
{
    /// Creates a new frozen set which uses the given hash builder to hash values.
    #[must_use]
    pub fn with_hasher(entries: Vec<&'a str>, bh: BH) -> Self {
        Self {
            map: FzStringMap::with_hasher(entries.into_iter().map(|x| (x, ())).collect(), bh),
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

impl<'a, BH> From<FzStringMap<&'a str, (), BH>> for FzStringSet<&'a str, BH> {
    fn from(map: FzStringMap<&'a str, (), BH>) -> Self {
        Self { map }
    }
}

impl<'a, const N: usize, BH> From<[&'a str; N]> for FzStringSet<&'a str, BH>
where
    BH: BuildHasher + Default,
{
    fn from(entries: [&'a str; N]) -> Self {
        Self::from(FzStringMap::from_iter(entries.into_iter().map(|x| (x, ()))))
    }
}

impl<'a, BH> FromIterator<&'a str> for FzStringSet<&'a str, BH>
where
    BH: BuildHasher + Default,
{
    fn from_iter<IT: IntoIterator<Item = &'a str>>(iter: IT) -> Self {
        Self::from(FzStringMap::from_iter(iter.into_iter().map(|x| (x, ()))))
    }
}

impl<T, Q, BH> Set<T, Q> for FzStringSet<T, BH>
where
    Q: Hash + Eq + Len + Equivalent<T>,
    BH: BuildHasher,
    LeftRangeHasher<BH>: Hasher<Q>,
    RightRangeHasher<BH>: Hasher<Q>,
{
}

impl<T, Q, BH> SetQuery<T, Q> for FzStringSet<T, BH>
where
    Q: Hash + Eq + Len + Equivalent<T>,
    BH: BuildHasher,
    LeftRangeHasher<BH>: Hasher<Q>,
    RightRangeHasher<BH>: Hasher<Q>,
{
    #[inline]
    fn get(&self, value: &Q) -> Option<&T> {
        Some(self.map.get_key_value(value)?.0)
    }
}

impl<T, BH> SetIteration<T> for FzStringSet<T, BH> {
    type Iterator<'a>
        = Iter<'a, T>
    where
        T: 'a,
        BH: 'a;

    set_iteration_funcs!();
}

impl<T, BH> Len for FzStringSet<T, BH> {
    fn len(&self) -> usize {
        self.map.len()
    }
}

impl<T, ST, BH> BitOr<&ST> for &FzStringSet<T, BH>
where
    T: Hash + Eq + Len + Clone,
    ST: Set<T>,
    BH: BuildHasher + Default,
    LeftRangeHasher<BH>: Hasher<T>,
    RightRangeHasher<BH>: Hasher<T>,
{
    bitor_fn!();
}

impl<T, ST, BH> BitAnd<&ST> for &FzStringSet<T, BH>
where
    T: Hash + Eq + Len + Clone,
    ST: Set<T>,
    BH: BuildHasher + Default,
    LeftRangeHasher<BH>: Hasher<T>,
    RightRangeHasher<BH>: Hasher<T>,
{
    bitand_fn!();
}

impl<T, ST, BH> BitXor<&ST> for &FzStringSet<T, BH>
where
    T: Hash + Eq + Len + Clone,
    ST: Set<T>,
    BH: BuildHasher + Default,
    LeftRangeHasher<BH>: Hasher<T>,
    RightRangeHasher<BH>: Hasher<T>,
{
    bitxor_fn!();
}

impl<T, ST, BH> Sub<&ST> for &FzStringSet<T, BH>
where
    T: Hash + Eq + Len + Clone,
    ST: Set<T>,
    BH: BuildHasher + Default,
    LeftRangeHasher<BH>: Hasher<T>,
    RightRangeHasher<BH>: Hasher<T>,
{
    sub_fn!();
}

impl<T, BH> IntoIterator for FzStringSet<T, BH> {
    into_iter_fn!();
}

impl<'a, T, BH> IntoIterator for &'a FzStringSet<T, BH> {
    into_iter_ref_fn!();
}

impl<T, ST, BH> PartialEq<ST> for FzStringSet<T, BH>
where
    T: Hash + Eq + Len,
    ST: Set<T>,
    BH: BuildHasher + Default,
    LeftRangeHasher<BH>: Hasher<T>,
    RightRangeHasher<BH>: Hasher<T>,
{
    partial_eq_fn!();
}

impl<T, BH> Eq for FzStringSet<T, BH>
where
    T: Hash + Eq + Len,
    BH: BuildHasher + Default,
    LeftRangeHasher<BH>: Hasher<T>,
    RightRangeHasher<BH>: Hasher<T>,
{
}

impl<T, BH> Debug for FzStringSet<T, BH>
where
    T: Debug,
{
    debug_fn!();
}

#[cfg(feature = "serde")]
impl<T> Serialize for FzStringSet<T>
where
    T: Serialize,
{
    serialize_fn!();
}

#[cfg(feature = "serde")]
impl<'de, BH> Deserialize<'de> for FzStringSet<&'de str, BH>
where
    BH: BuildHasher + Default,
{
    fn deserialize<D>(deserializer: D) -> core::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer.deserialize_seq(SetVisitor {
            marker: PhantomData,
        })
    }
}

#[cfg(feature = "serde")]
struct SetVisitor<BH> {
    marker: PhantomData<BH>,
}

#[cfg(feature = "serde")]
impl<'de, BH> Visitor<'de> for SetVisitor<BH>
where
    BH: BuildHasher + Default,
{
    type Value = FzStringSet<&'de str, BH>;

    fn expecting(&self, formatter: &mut Formatter) -> core::fmt::Result {
        formatter.write_str("a set with string values")
    }

    fn visit_seq<M>(self, mut access: M) -> core::result::Result<Self::Value, M::Error>
    where
        M: SeqAccess<'de>,
    {
        let mut v = Vec::with_capacity(access.size_hint().unwrap_or(0));
        while let Some(x) = access.next_element()? {
            v.push((x, ()));
        }

        Ok(FzStringSet::from(FzStringMap::with_hasher(
            v,
            BH::default(),
        )))
    }
}
