use crate::fz_maps::FzHashMap;
use crate::sets::decl_macros::{
    bitand_fn, bitor_fn, bitxor_fn, debug_fn, into_iter_fn, into_iter_ref_fn, partial_eq_fn,
    set_iteration_funcs, sub_fn,
};
use crate::sets::{IntoIter, Iter};
use crate::traits::{Len, MapIteration, MapQuery, Set, SetIteration, SetOps, SetQuery};
use alloc::vec::Vec;
use core::fmt::Debug;
use core::hash::BuildHasher;
use core::hash::Hash;
use core::iter::FromIterator;
use core::ops::{BitAnd, BitOr, BitXor, Sub};
use equivalent::Equivalent;

use crate::DefaultHashBuilder;
#[cfg(feature = "serde")]
use {
    crate::sets::decl_macros::serialize_fn,
    core::fmt::Formatter,
    core::marker::PhantomData,
    serde::de::{SeqAccess, Visitor},
    serde::ser::SerializeSeq,
    serde::{Deserialize, Deserializer, Serialize, Serializer},
};

/// A set optimized for fast read access with hashable values.
///
#[doc = include_str!("../doc_snippets/about.md")]
#[doc = include_str!("../doc_snippets/hash_warning.md")]
///
/// # Alternate Choices
///
/// If your values are integers or enum variants, you should use the [`FzScalarSet`](crate::fz_sets::FzScalarSet) type instead.
/// If your values are strings, you should use the [`FzStringSet`](crate::fz_sets::FzStringSet) type instead. Both of these will
/// deliver better performance since they are specifically optimized for those value types.
///
/// If your values are known at compile time, consider using the various `fz_*_set` macros instead of
/// this type as they generally perform better.
#[derive(Clone)]
pub struct FzHashSet<T, BH = DefaultHashBuilder> {
    map: FzHashMap<T, (), BH>,
}

impl<T> FzHashSet<T, DefaultHashBuilder>
where
    T: Hash + Eq,
{
    /// Creates a new frozen set.
    #[must_use]
    pub fn new(entries: Vec<T>) -> Self {
        Self::with_hasher(entries, foldhash::fast::RandomState::default())
    }
}

impl<T, BH> FzHashSet<T, BH>
where
    T: Hash + Eq,
    BH: BuildHasher,
{
    /// Creates a new frozen set which uses the given hash builder to hash values.
    #[must_use]
    pub fn with_hasher(entries: Vec<T>, bh: BH) -> Self {
        Self {
            map: FzHashMap::with_hasher(entries.into_iter().map(|x| (x, ())).collect(), bh),
        }
    }
}

impl<T, BH> Default for FzHashSet<T, BH>
where
    BH: Default,
{
    fn default() -> Self {
        Self {
            map: FzHashMap::default(),
        }
    }
}

impl<T, BH> From<FzHashMap<T, (), BH>> for FzHashSet<T, BH>
where
    T: Hash + Eq,
    BH: BuildHasher + Default,
{
    fn from(map: FzHashMap<T, (), BH>) -> Self {
        Self { map }
    }
}

impl<T, const N: usize, BH> From<[T; N]> for FzHashSet<T, BH>
where
    T: Hash + Eq,
    BH: BuildHasher + Default,
{
    fn from(entries: [T; N]) -> Self {
        Self::from(FzHashMap::from_iter(entries.into_iter().map(|x| (x, ()))))
    }
}

impl<T, BH> FromIterator<T> for FzHashSet<T, BH>
where
    T: Hash + Eq,
    BH: BuildHasher + Default,
{
    fn from_iter<IT: IntoIterator<Item = T>>(iter: IT) -> Self {
        Self::from(FzHashMap::from_iter(iter.into_iter().map(|x| (x, ()))))
    }
}

impl<T, Q, BH> Set<T, Q> for FzHashSet<T, BH>
where
    Q: Hash + Eq + Equivalent<T>,
    BH: BuildHasher,
{
}

impl<T, Q, BH> SetQuery<T, Q> for FzHashSet<T, BH>
where
    Q: Hash + Eq + Equivalent<T>,
    BH: BuildHasher,
{
    #[inline]
    fn get(&self, value: &Q) -> Option<&T> {
        Some(self.map.get_key_value(value)?.0)
    }
}

impl<T, BH> SetIteration<T> for FzHashSet<T, BH> {
    type Iterator<'a>
        = Iter<'a, T>
    where
        T: 'a,
        BH: 'a;

    set_iteration_funcs!();
}

impl<T, BH> Len for FzHashSet<T, BH> {
    fn len(&self) -> usize {
        self.map.len()
    }
}

impl<T, ST, BH> BitOr<&ST> for &FzHashSet<T, BH>
where
    T: Hash + Eq + Clone,
    ST: Set<T>,
    BH: BuildHasher,
{
    bitor_fn!(H);
}

impl<T, ST, BH> BitAnd<&ST> for &FzHashSet<T, BH>
where
    T: Hash + Eq + Clone,
    ST: Set<T>,
    BH: BuildHasher,
{
    bitand_fn!(H);
}

impl<T, ST, BH> BitXor<&ST> for &FzHashSet<T, BH>
where
    T: Hash + Eq + Clone,
    ST: Set<T>,
    BH: BuildHasher,
{
    bitxor_fn!(H);
}

impl<T, ST, BH> Sub<&ST> for &FzHashSet<T, BH>
where
    T: Hash + Eq + Clone,
    ST: Set<T>,
    BH: BuildHasher,
{
    sub_fn!(H);
}

impl<T, BH> IntoIterator for FzHashSet<T, BH> {
    into_iter_fn!();
}

impl<'a, T, BH> IntoIterator for &'a FzHashSet<T, BH> {
    into_iter_ref_fn!();
}

impl<T, ST, BH> PartialEq<ST> for FzHashSet<T, BH>
where
    T: Hash + Eq,
    ST: Set<T>,
    BH: BuildHasher,
{
    partial_eq_fn!();
}

impl<T, BH> Eq for FzHashSet<T, BH>
where
    T: Hash + Eq,
    BH: BuildHasher,
{
}

impl<T, BH> Debug for FzHashSet<T, BH>
where
    T: Debug,
{
    debug_fn!();
}

#[cfg(feature = "serde")]
impl<T, BH> Serialize for FzHashSet<T, BH>
where
    T: Serialize,
{
    serialize_fn!();
}

#[cfg(feature = "serde")]
impl<'de, T, BH> Deserialize<'de> for FzHashSet<T, BH>
where
    T: Deserialize<'de> + Hash + Eq,
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
struct SetVisitor<T, BH> {
    marker: PhantomData<(T, BH)>,
}

#[cfg(feature = "serde")]
impl<'de, T, BH> Visitor<'de> for SetVisitor<T, BH>
where
    T: Deserialize<'de> + Hash + Eq,
    BH: BuildHasher + Default,
{
    type Value = FzHashSet<T, BH>;

    fn expecting(&self, formatter: &mut Formatter) -> core::fmt::Result {
        formatter.write_str("a set with hashable values")
    }

    fn visit_seq<M>(self, mut access: M) -> core::result::Result<Self::Value, M::Error>
    where
        M: SeqAccess<'de>,
    {
        let mut v = Vec::with_capacity(access.size_hint().unwrap_or(0));
        while let Some(x) = access.next_element()? {
            v.push((x, ()));
        }

        Ok(FzHashSet::from(FzHashMap::with_hasher(v, BH::default())))
    }
}
