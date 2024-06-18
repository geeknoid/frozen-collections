use std::borrow::Borrow;
use std::collections::HashSet;
use std::fmt::{Debug, Formatter, Result};
use std::hash::{Hash, RandomState};
use std::ops::{BitAnd, BitOr, BitXor, Sub};

use num_traits::PrimInt;

use crate::specialized_maps::IntegerRangeMap;
use crate::specialized_sets::{IntoIter, Iter};
use crate::traits::Len;
use crate::traits::Set;

/// A set whose values are a continuous range of integers.
///
/// # Important Note
///
/// This type is not intended to be used directly by
/// application code. Instead, applications are expected
/// to use the `FrozenSet` type or the `frozen_set!` macro.
#[derive(Clone, Default)]
pub struct IntegerRangeSet<T> {
    map: IntegerRangeMap<T, ()>,
}

impl<T> IntegerRangeSet<T> {
    #[inline]
    #[must_use]
    pub fn get<Q>(&self, value: &Q) -> Option<&T>
    where
        T: Borrow<Q>,
        Q: ?Sized + PrimInt,
    {
        Some(self.map.get_key_value(value)?.0)
    }

    #[inline]
    #[must_use]
    pub fn contains<Q>(&self, value: &Q) -> bool
    where
        T: Borrow<Q>,
        Q: ?Sized + PrimInt,
    {
        self.get(value).is_some()
    }

    #[must_use]
    pub const fn iter(&self) -> Iter<T> {
        Iter::new(&self.map.entries)
    }
}

impl<T> Len for IntegerRangeSet<T> {
    fn len(&self) -> usize {
        self.map.len()
    }
}

impl<T> Debug for IntegerRangeSet<T>
where
    T: Debug,
{
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        f.debug_set().entries(self.iter()).finish()
    }
}

impl<T> IntoIterator for IntegerRangeSet<T> {
    type Item = T;
    type IntoIter = IntoIter<T>;

    fn into_iter(self) -> Self::IntoIter {
        IntoIter::new(self.map.entries)
    }
}

impl<'a, T> IntoIterator for &'a IntegerRangeSet<T> {
    type Item = &'a T;
    type IntoIter = Iter<'a, T>;

    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}

impl<T> TryFrom<Vec<T>> for IntegerRangeSet<T>
where
    T: PrimInt + Hash + Eq,
{
    type Error = &'static str;

    #[allow(clippy::from_iter_instead_of_collect)]
    fn try_from(payload: Vec<T>) -> std::result::Result<Self, Self::Error> {
        Ok(Self {
            map: IntegerRangeMap::try_from(Vec::from_iter(payload.into_iter().map(|x| (x, ()))))?,
        })
    }
}

impl<T, const N: usize> TryFrom<[T; N]> for IntegerRangeSet<T>
where
    T: PrimInt + Hash + Eq,
{
    type Error = &'static str;

    #[allow(clippy::from_iter_instead_of_collect)]
    fn try_from(payload: [T; N]) -> std::result::Result<Self, Self::Error> {
        Ok(Self {
            map: IntegerRangeMap::try_from(Vec::from_iter(payload.into_iter().map(|x| (x, ()))))?,
        })
    }
}

impl<T> FromIterator<T> for IntegerRangeSet<T>
where
    T: PrimInt + Hash + Eq,
{
    fn from_iter<I: IntoIterator<Item = T>>(iter: I) -> Self {
        Self {
            map: iter.into_iter().map(|x| (x, ())).collect(),
        }
    }
}

impl<T> Set<T> for IntegerRangeSet<T>
where
    T: PrimInt,
{
    type Iterator<'a> = Iter<'a, T>
    where
        T: 'a;

    fn iter(&self) -> Iter<'_, T> {
        self.iter()
    }

    fn contains(&self, value: &T) -> bool {
        self.contains(value)
    }
}

impl<T, ST> BitOr<&ST> for &IntegerRangeSet<T>
where
    T: PrimInt + Hash,
    ST: Set<T>,
{
    type Output = HashSet<T, RandomState>;

    fn bitor(self, rhs: &ST) -> Self::Output {
        self.union(rhs).copied().collect()
    }
}

impl<T, ST> BitAnd<&ST> for &IntegerRangeSet<T>
where
    T: PrimInt + Hash,
    ST: Set<T>,
{
    type Output = HashSet<T, RandomState>;

    fn bitand(self, rhs: &ST) -> Self::Output {
        self.intersection(rhs).copied().collect()
    }
}

impl<T, ST> BitXor<&ST> for &IntegerRangeSet<T>
where
    T: PrimInt + Hash,
    ST: Set<T>,
{
    type Output = HashSet<T, RandomState>;

    fn bitxor(self, rhs: &ST) -> Self::Output {
        self.symmetric_difference(rhs).copied().collect()
    }
}

impl<T, ST> Sub<&ST> for &IntegerRangeSet<T>
where
    T: PrimInt + Hash,
    ST: Set<T>,
{
    type Output = HashSet<T, RandomState>;

    fn sub(self, rhs: &ST) -> Self::Output {
        self.difference(rhs).copied().collect()
    }
}

impl<T, ST> PartialEq<ST> for IntegerRangeSet<T>
where
    T: PrimInt,
    ST: Set<T>,
{
    fn eq(&self, other: &ST) -> bool {
        if self.len() != other.len() {
            return false;
        }

        self.iter().all(|value| other.contains(value))
    }
}

impl<T> Eq for IntegerRangeSet<T> where T: PrimInt {}
