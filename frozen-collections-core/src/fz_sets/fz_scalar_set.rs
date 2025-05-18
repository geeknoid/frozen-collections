use crate::fz_maps::FzScalarMap;
use crate::sets::decl_macros::{
    bitand_fn, bitor_fn, bitxor_fn, debug_fn, into_iter_fn, into_iter_ref_fn, partial_eq_fn, set_iteration_funcs, sub_fn,
};
use crate::sets::{IntoIter, Iter};
use crate::traits::{Len, MapIteration, MapQuery, Scalar, Set, SetIteration, SetOps, SetQuery};
use alloc::vec::Vec;
use core::fmt::Debug;
use core::hash::Hash;
use core::iter::FromIterator;
use core::ops::{BitAnd, BitOr, BitXor, Sub};

#[cfg(feature = "serde")]
use {
    crate::sets::decl_macros::serialize_fn,
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

impl<T> FzScalarSet<T>
where
    T: Scalar,
{
    /// Creates a new frozen set.
    #[must_use]
    pub fn new(entries: Vec<T>) -> Self {
        Self {
            map: FzScalarMap::new(entries.into_iter().map(|x| (x, ())).collect()),
        }
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

impl<T> SetQuery<T, T> for FzScalarSet<T>
where
    T: Scalar,
{
    #[inline]
    fn get(&self, value: &T) -> Option<&T> {
        Some(self.map.get_key_value(value)?.0)
    }
}

impl<T> SetIteration<T> for FzScalarSet<T> {
    type Iterator<'a>
        = Iter<'a, T>
    where
        T: 'a;

    set_iteration_funcs!();
}

impl<T> Len for FzScalarSet<T> {
    fn len(&self) -> usize {
        self.map.len()
    }
}

impl<T, ST> BitOr<&ST> for &FzScalarSet<T>
where
    T: Hash + Eq + Scalar + Clone,
    ST: Set<T>,
{
    bitor_fn!();
}

impl<T, ST> BitAnd<&ST> for &FzScalarSet<T>
where
    T: Hash + Eq + Scalar + Clone,
    ST: Set<T>,
{
    bitand_fn!();
}

impl<T, ST> BitXor<&ST> for &FzScalarSet<T>
where
    T: Hash + Eq + Scalar + Clone,
    ST: Set<T>,
{
    bitxor_fn!();
}

impl<T, ST> Sub<&ST> for &FzScalarSet<T>
where
    T: Hash + Eq + Scalar + Clone,
    ST: Set<T>,
{
    sub_fn!();
}

impl<T> IntoIterator for FzScalarSet<T> {
    into_iter_fn!();
}

impl<'a, T> IntoIterator for &'a FzScalarSet<T> {
    into_iter_ref_fn!();
}

impl<T, ST> PartialEq<ST> for FzScalarSet<T>
where
    T: Scalar,
    ST: Set<T>,
{
    partial_eq_fn!();
}

impl<T> Eq for FzScalarSet<T> where T: Scalar {}

impl<T> Debug for FzScalarSet<T>
where
    T: Debug,
{
    debug_fn!();
}

#[cfg(feature = "serde")]
impl<T> Serialize for FzScalarSet<T>
where
    T: Serialize,
{
    serialize_fn!();
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

    fn visit_seq<M>(self, mut access: M) -> Result<Self::Value, M::Error>
    where
        M: SeqAccess<'de>,
    {
        let mut v = Vec::with_capacity(access.size_hint().unwrap_or(0));
        while let Some(x) = access.next_element()? {
            v.push((x, ()));
        }

        Ok(FzScalarSet::from(FzScalarMap::new(v)))
    }
}
