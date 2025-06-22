use crate::fz_maps::FzScalarMap;
use crate::maps::decl_macros::len_trait_funcs;
use crate::sets::decl_macros::{
    bitand_trait_funcs, bitor_trait_funcs, bitxor_trait_funcs, debug_trait_funcs, into_iterator_ref_trait_funcs, into_iterator_trait_funcs,
    partial_eq_trait_funcs, set_extras_trait_funcs, set_iteration_trait_funcs, set_query_trait_funcs, sub_trait_funcs,
};
use crate::sets::{IntoIter, Iter};
use crate::traits::{Len, Scalar, Set, SetExtras, SetIteration, SetOps, SetQuery};
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

/// A set optimized for fast read access with integer or enum values.
///
#[doc = include_str!("../doc_snippets/about.md")]
///
/// # Alternate Choices
///
/// If your values are known at compile time, consider using the various `fz_*_set` macros instead of
/// this type as they generally perform better.
#[derive(Clone)]
pub struct FzScalarSet<T> {
    map: FzScalarMap<T, ()>,
}

impl<T> FzScalarSet<T> {
    /// Creates a new frozen set.
    #[must_use]
    pub fn new(entries: Vec<T>) -> Self
    where
        T: Scalar,
    {
        Self {
            map: FzScalarMap::new(entries.into_iter().map(|x| (x, ())).collect()),
        }
    }

    #[doc = include_str!("../doc_snippets/get_from_set.md")]
    #[inline]
    pub fn get<Q>(&self, value: &Q) -> Option<&T>
    where
        Q: Scalar + Comparable<T>,
    {
        Some(self.map.get_key_value(value)?.0)
    }

    #[doc = include_str!("../doc_snippets/contains.md")]
    #[inline]
    #[must_use]
    pub fn contains<Q>(&self, value: &Q) -> bool
    where
        Q: Scalar + Comparable<T>,
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

impl<T> Default for FzScalarSet<T> {
    fn default() -> Self {
        Self {
            map: FzScalarMap::default(),
        }
    }
}

impl<T> From<FzScalarMap<T, ()>> for FzScalarSet<T>
where
    T: Scalar,
{
    fn from(map: FzScalarMap<T, ()>) -> Self {
        Self { map }
    }
}

impl<T, const N: usize> From<[T; N]> for FzScalarSet<T>
where
    T: Scalar,
{
    fn from(entries: [T; N]) -> Self {
        Self::from(FzScalarMap::from_iter(entries.into_iter().map(|x| (x, ()))))
    }
}

impl<T> FromIterator<T> for FzScalarSet<T>
where
    T: Scalar,
{
    fn from_iter<IT: IntoIterator<Item = T>>(iter: IT) -> Self {
        Self::from(FzScalarMap::from_iter(iter.into_iter().map(|x| (x, ()))))
    }
}

impl<T> Set<T, T> for FzScalarSet<T> where T: Scalar {}

impl<T, Q> SetExtras<T, Q> for FzScalarSet<T>
where
    Q: Scalar + Comparable<T>,
{
    set_extras_trait_funcs!();
}

impl<T, Q> SetQuery<Q> for FzScalarSet<T>
where
    Q: Scalar + Comparable<T>,
{
    set_query_trait_funcs!();
}

impl<T> SetIteration<T> for FzScalarSet<T> {
    type Iterator<'a>
        = Iter<'a, T>
    where
        T: 'a;

    set_iteration_trait_funcs!();
}

impl<T> Len for FzScalarSet<T> {
    len_trait_funcs!();
}

impl<T, ST> BitOr<&ST> for &FzScalarSet<T>
where
    T: Hash + Scalar + Clone,
    ST: Set<T>,
{
    bitor_trait_funcs!();
}

impl<T, ST> BitAnd<&ST> for &FzScalarSet<T>
where
    T: Hash + Scalar + Clone,
    ST: Set<T>,
{
    bitand_trait_funcs!();
}

impl<T, ST> BitXor<&ST> for &FzScalarSet<T>
where
    T: Hash + Scalar + Clone,
    ST: Set<T>,
{
    bitxor_trait_funcs!();
}

impl<T, ST> Sub<&ST> for &FzScalarSet<T>
where
    T: Hash + Scalar + Clone,
    ST: Set<T>,
{
    sub_trait_funcs!();
}

impl<T> IntoIterator for FzScalarSet<T> {
    into_iterator_trait_funcs!();
}

impl<'a, T> IntoIterator for &'a FzScalarSet<T> {
    into_iterator_ref_trait_funcs!();
}

impl<T, ST> PartialEq<ST> for FzScalarSet<T>
where
    T: Scalar,
    ST: SetQuery<T>,
{
    partial_eq_trait_funcs!();
}

impl<T> Eq for FzScalarSet<T> where T: Scalar {}

impl<T> Debug for FzScalarSet<T>
where
    T: Debug,
{
    debug_trait_funcs!();
}

#[cfg(feature = "serde")]
impl<T> Serialize for FzScalarSet<T>
where
    T: Serialize,
{
    serialize_trait_funcs!();
}

#[cfg(feature = "serde")]
impl<'de, T> Deserialize<'de> for FzScalarSet<T>
where
    T: Deserialize<'de> + Scalar,
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
    T: Deserialize<'de> + Scalar,
{
    type Value = FzScalarSet<T>;

    fn expecting(&self, formatter: &mut Formatter) -> core::fmt::Result {
        formatter.write_str("a set with scalar values")
    }

    fn visit_seq<M>(self, mut seq: M) -> Result<Self::Value, M::Error>
    where
        M: SeqAccess<'de>,
    {
        let mut v = Vec::with_capacity(seq.size_hint().unwrap_or(0));
        while let Some(x) = seq.next_element()? {
            v.push((x, ()));
        }

        Ok(FzScalarSet::from(FzScalarMap::new(v)))
    }
}
