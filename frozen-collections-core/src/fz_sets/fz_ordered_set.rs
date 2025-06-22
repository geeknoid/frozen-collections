use crate::fz_maps::FzOrderedMap;
use crate::maps::decl_macros::len_trait_funcs;
use crate::sets::decl_macros::{
    bitand_trait_funcs, bitor_trait_funcs, bitxor_trait_funcs, debug_trait_funcs, into_iterator_ref_trait_funcs, into_iterator_trait_funcs,
    partial_eq_trait_funcs, set_extras_trait_funcs, set_iteration_trait_funcs, set_query_trait_funcs, sub_trait_funcs,
};
use crate::sets::{IntoIter, Iter};
use crate::traits::{Len, Set, SetExtras, SetIteration, SetOps, SetQuery};
use core::fmt::Debug;
use core::hash::Hash;
use core::ops::{BitAnd, BitOr, BitXor, Sub};
use equivalent::Comparable;

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

impl<T> FzOrderedSet<T> {
    /// Creates a new frozen ordered set.
    #[must_use]
    pub fn new(entries: Vec<T>) -> Self
    where
        T: Ord,
    {
        Self {
            map: FzOrderedMap::new(entries.into_iter().map(|x| (x, ())).collect()),
        }
    }

    #[doc = include_str!("../doc_snippets/get_from_set.md")]
    #[inline]
    pub fn get<Q>(&self, value: &Q) -> Option<&T>
    where
        Q: ?Sized + Comparable<T>,
    {
        Some(self.map.get_key_value(value)?.0)
    }

    #[doc = include_str!("../doc_snippets/contains.md")]
    #[inline]
    #[must_use]
    pub fn contains<Q>(&self, value: &Q) -> bool
    where
        Q: ?Sized + Comparable<T>,
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

    #[must_use]
    fn into_iter(self) -> IntoIter<T> {
        IntoIter::new(self.map.into_iter())
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
        Self::from(FzOrderedMap::from_iter(entries.into_iter().map(|x| (x, ()))))
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

impl<T, Q> Set<T, Q> for FzOrderedSet<T> where Q: ?Sized + Comparable<T> {}

impl<T, Q> SetExtras<T, Q> for FzOrderedSet<T>
where
    Q: ?Sized + Comparable<T>,
{
    set_extras_trait_funcs!();
}

impl<T, Q> SetQuery<Q> for FzOrderedSet<T>
where
    Q: ?Sized + Comparable<T>,
{
    set_query_trait_funcs!();
}

impl<T> SetIteration<T> for FzOrderedSet<T> {
    type Iterator<'a>
        = Iter<'a, T>
    where
        T: 'a;

    set_iteration_trait_funcs!();
}

impl<T> Len for FzOrderedSet<T> {
    len_trait_funcs!();
}

impl<T, ST> BitOr<&ST> for &FzOrderedSet<T>
where
    T: Hash + Ord + Clone,
    ST: Set<T>,
{
    bitor_trait_funcs!();
}

impl<T, ST> BitAnd<&ST> for &FzOrderedSet<T>
where
    T: Hash + Ord + Clone,
    ST: Set<T>,
{
    bitand_trait_funcs!();
}

impl<T, ST> BitXor<&ST> for &FzOrderedSet<T>
where
    T: Hash + Ord + Clone,
    ST: Set<T>,
{
    bitxor_trait_funcs!();
}

impl<T, ST> Sub<&ST> for &FzOrderedSet<T>
where
    T: Hash + Ord + Clone,
    ST: Set<T>,
{
    sub_trait_funcs!();
}

impl<T> IntoIterator for FzOrderedSet<T> {
    into_iterator_trait_funcs!();
}

impl<'a, T> IntoIterator for &'a FzOrderedSet<T> {
    into_iterator_ref_trait_funcs!();
}

impl<T, ST> PartialEq<ST> for FzOrderedSet<T>
where
    T: Ord,
    ST: SetQuery<T>,
{
    partial_eq_trait_funcs!();
}

impl<T> Eq for FzOrderedSet<T> where T: Ord {}

impl<T> Debug for FzOrderedSet<T>
where
    T: Debug,
{
    debug_trait_funcs!();
}

#[cfg(feature = "serde")]
impl<T> Serialize for FzOrderedSet<T>
where
    T: Serialize,
{
    serialize_trait_funcs!();
}

#[cfg(feature = "serde")]
impl<'de, T> Deserialize<'de> for FzOrderedSet<T>
where
    T: Deserialize<'de> + Ord,
{
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer.deserialize_seq(SetVisitor { marker: PhantomData })
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

    fn visit_seq<M>(self, mut seq: M) -> Result<Self::Value, M::Error>
    where
        M: SeqAccess<'de>,
    {
        let mut v = Vec::with_capacity(seq.size_hint().unwrap_or(0));
        while let Some(x) = seq.next_element()? {
            v.push((x, ()));
        }

        Ok(FzOrderedSet::from(FzOrderedMap::new(v)))
    }
}
