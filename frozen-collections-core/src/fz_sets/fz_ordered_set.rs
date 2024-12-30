use crate::fz_maps::FzOrderedMap;
use crate::sets::decl_macros::{
    bitand_fn, bitor_fn, bitxor_fn, debug_fn, get_fn, into_iter_fn, into_iter_ref_fn,
    partial_eq_fn, set_iteration_funcs, sub_fn,
};
use crate::sets::{IntoIter, Iter};
use crate::traits::{Len, MapIteration, MapQuery, Set, SetIteration, SetOps, SetQuery};
use alloc::vec::Vec;
use core::fmt::Debug;
use core::hash::Hash;
use core::iter::FromIterator;
use core::ops::{BitAnd, BitOr, BitXor, Sub};
use equivalent::Comparable;

#[cfg(feature = "serde")]
use {
    crate::sets::decl_macros::serialize_fn,
    core::fmt::Formatter,
    core::marker::PhantomData,
    serde::de::{SeqAccess, Visitor},
    serde::ser::SerializeSeq,
    serde::{Deserialize, Deserializer, Serialize, Serializer},
};

/// A set optimized for fast read access with ordered values.
///
#[doc = include_str!("../doc_snippets/about.md")]
#[doc = include_str!("../doc_snippets/ord_warning.md")]
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
pub struct FzOrderedSet<T> {
    map: FzOrderedMap<T, ()>,
}

impl<T> FzOrderedSet<T>
where
    T: Ord + Eq,
{
    /// Creates a new frozen ordered set.
    #[must_use]
    pub fn new(entries: Vec<T>) -> Self {
        Self {
            map: FzOrderedMap::new(entries.into_iter().map(|x| (x, ())).collect()),
        }
    }
}

impl<T> Default for FzOrderedSet<T> {
    fn default() -> Self {
        Self {
            map: FzOrderedMap::default(),
        }
    }
}

impl<T> From<FzOrderedMap<T, ()>> for FzOrderedSet<T>
where
    T: Ord,
{
    fn from(map: FzOrderedMap<T, ()>) -> Self {
        Self { map }
    }
}

impl<T, const N: usize> From<[T; N]> for FzOrderedSet<T>
where
    T: Ord,
{
    fn from(entries: [T; N]) -> Self {
        Self::from(FzOrderedMap::from_iter(
            entries.into_iter().map(|x| (x, ())),
        ))
    }
}

impl<T> FromIterator<T> for FzOrderedSet<T>
where
    T: Ord,
{
    fn from_iter<IT: IntoIterator<Item = T>>(iter: IT) -> Self {
        Self::from(FzOrderedMap::from_iter(iter.into_iter().map(|x| (x, ()))))
    }
}

impl<T, Q> Set<T, Q> for FzOrderedSet<T> where Q: ?Sized + Ord + Comparable<T> {}

impl<T, Q> SetQuery<T, Q> for FzOrderedSet<T>
where
    Q: ?Sized + Ord + Comparable<T>,
{
    get_fn!();
}

impl<T> SetIteration<T> for FzOrderedSet<T> {
    type Iterator<'a>
        = Iter<'a, T>
    where
        T: 'a;

    set_iteration_funcs!();
}

impl<T> Len for FzOrderedSet<T> {
    fn len(&self) -> usize {
        self.map.len()
    }
}

impl<T, ST> BitOr<&ST> for &FzOrderedSet<T>
where
    T: Hash + Eq + Ord + Clone,
    ST: Set<T>,
{
    bitor_fn!();
}

impl<T, ST> BitAnd<&ST> for &FzOrderedSet<T>
where
    T: Hash + Eq + Ord + Clone,
    ST: Set<T>,
{
    bitand_fn!();
}

impl<T, ST> BitXor<&ST> for &FzOrderedSet<T>
where
    T: Hash + Eq + Ord + Clone,
    ST: Set<T>,
{
    bitxor_fn!();
}

impl<T, ST> Sub<&ST> for &FzOrderedSet<T>
where
    T: Hash + Eq + Ord + Clone,
    ST: Set<T>,
{
    sub_fn!();
}

impl<T> IntoIterator for FzOrderedSet<T> {
    into_iter_fn!();
}

impl<'a, T> IntoIterator for &'a FzOrderedSet<T> {
    into_iter_ref_fn!();
}

impl<T, ST> PartialEq<ST> for FzOrderedSet<T>
where
    T: Ord,
    ST: Set<T>,
{
    partial_eq_fn!();
}

impl<T> Eq for FzOrderedSet<T> where T: Ord {}

impl<T> Debug for FzOrderedSet<T>
where
    T: Debug,
{
    debug_fn!();
}

#[cfg(feature = "serde")]
impl<T> Serialize for FzOrderedSet<T>
where
    T: Serialize,
{
    serialize_fn!();
}

#[cfg(feature = "serde")]
impl<'de, T> Deserialize<'de> for FzOrderedSet<T>
where
    T: Deserialize<'de> + Ord,
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
struct SetVisitor<T> {
    marker: PhantomData<T>,
}

#[cfg(feature = "serde")]
impl<'de, T> Visitor<'de> for SetVisitor<T>
where
    T: Deserialize<'de> + Ord,
{
    type Value = FzOrderedSet<T>;

    fn expecting(&self, formatter: &mut Formatter) -> core::fmt::Result {
        formatter.write_str("a set with ordered values")
    }

    fn visit_seq<M>(self, mut access: M) -> core::result::Result<Self::Value, M::Error>
    where
        M: SeqAccess<'de>,
    {
        let mut v = Vec::with_capacity(access.size_hint().unwrap_or(0));
        while let Some(x) = access.next_element()? {
            v.push((x, ()));
        }

        Ok(FzOrderedSet::from(FzOrderedMap::new(v)))
    }
}
