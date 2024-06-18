use std::borrow::Borrow;
use std::collections::HashSet;
use std::fmt::{Debug, Formatter, Result};
use std::hash::{Hash, RandomState};
use std::ops::{BitAnd, BitOr, BitXor, Sub};

use num_traits::{PrimInt, Unsigned};

use crate::specialized_maps::LengthMap;
use crate::specialized_sets::{IntoIter, Iter};
use crate::traits::Len;
use crate::traits::Set;

/// A set that uses value lengths as hash codes.
///
/// # Capacity Constraints
///
/// The `S` generic argument controls the maximum capacity
/// of the set. A `u8` will allow up to 255 elements, `u16`
/// will allow up to 65,535 elements, and `usize` will allow
/// up to `usize::MAX` elements.
///
/// # Important Note
///
/// This type is not intended to be used directly by
/// application code. Instead, applications are expected
/// to use the `FrozenSet` type or the `frozen_set!` macro.
#[derive(Clone)]
pub struct LengthSet<T, S = u8> {
    map: LengthMap<T, (), S>,
}

impl<T, S> LengthSet<T, S>
where
    S: PrimInt + Unsigned,
{
    #[inline]
    #[must_use]
    pub fn get<Q>(&self, value: &Q) -> Option<&T>
    where
        T: Borrow<Q>,
        Q: ?Sized + Len + Eq,
    {
        Some(self.map.get_key_value(value)?.0)
    }

    #[inline]
    #[must_use]
    pub fn contains<Q>(&self, value: &Q) -> bool
    where
        T: Borrow<Q>,
        Q: ?Sized + Len + Eq,
    {
        self.get(value).is_some()
    }
}

impl<T, S> LengthSet<T, S> {
    #[must_use]
    pub const fn iter(&self) -> Iter<T> {
        Iter::new(&self.map.table.entries)
    }
}

impl<T, S> Len for LengthSet<T, S> {
    fn len(&self) -> usize {
        self.map.len()
    }
}

impl<T, S> Debug for LengthSet<T, S>
where
    T: Debug,
{
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        f.debug_set().entries(self.iter()).finish()
    }
}

impl<T, S> Default for LengthSet<T, S>
where
    S: PrimInt + Unsigned,
{
    fn default() -> Self {
        Self {
            map: LengthMap::default(),
        }
    }
}

impl<T, S> IntoIterator for LengthSet<T, S> {
    type Item = T;
    type IntoIter = IntoIter<T>;

    fn into_iter(self) -> Self::IntoIter {
        IntoIter::new(self.map.table.entries)
    }
}

impl<'a, T, S> IntoIterator for &'a LengthSet<T, S> {
    type Item = &'a T;
    type IntoIter = Iter<'a, T>;

    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}

impl<T, S> TryFrom<Vec<T>> for LengthSet<T, S>
where
    T: Len + Hash + Eq,
    S: PrimInt + Unsigned,
{
    type Error = &'static str;

    #[allow(clippy::from_iter_instead_of_collect)]
    fn try_from(payload: Vec<T>) -> std::result::Result<Self, Self::Error> {
        Ok(Self {
            map: LengthMap::try_from(Vec::from_iter(payload.into_iter().map(|x| (x, ()))))?,
        })
    }
}

impl<T, S, const N: usize> TryFrom<[T; N]> for LengthSet<T, S>
where
    T: Len + Hash + Eq,
    S: PrimInt + Unsigned,
{
    type Error = &'static str;

    #[allow(clippy::from_iter_instead_of_collect)]
    fn try_from(payload: [T; N]) -> std::result::Result<Self, Self::Error> {
        Ok(Self {
            map: LengthMap::try_from(Vec::from_iter(payload.into_iter().map(|x| (x, ()))))?,
        })
    }
}

impl<T, S> FromIterator<T> for LengthSet<T, S>
where
    T: Len + Hash + Eq,
    S: PrimInt + Unsigned,
{
    fn from_iter<I: IntoIterator<Item = T>>(iter: I) -> Self {
        Self {
            map: iter.into_iter().map(|x| (x, ())).collect(),
        }
    }
}

impl<T, S> Set<T> for LengthSet<T, S>
where
    T: Len + Eq,
    S: PrimInt + Unsigned,
{
    type Iterator<'a> = Iter<'a, T>
    where
        T: 'a,
        S: 'a;

    fn iter(&self) -> Iter<'_, T> {
        self.iter()
    }

    fn contains(&self, value: &T) -> bool {
        self.contains(value)
    }
}

impl<T, S, ST> BitOr<&ST> for &LengthSet<T, S>
where
    T: Hash + Eq + Len + Clone,
    S: PrimInt + Unsigned,
    ST: Set<T>,
{
    type Output = HashSet<T, RandomState>;

    fn bitor(self, rhs: &ST) -> Self::Output {
        self.union(rhs).cloned().collect()
    }
}

impl<T, S, ST> BitAnd<&ST> for &LengthSet<T, S>
where
    T: Hash + Eq + Len + Clone,
    S: PrimInt + Unsigned,
    ST: Set<T>,
{
    type Output = HashSet<T, RandomState>;

    fn bitand(self, rhs: &ST) -> Self::Output {
        self.intersection(rhs).cloned().collect()
    }
}

impl<T, S, ST> BitXor<&ST> for &LengthSet<T, S>
where
    T: Hash + Eq + Len + Clone,
    S: PrimInt + Unsigned,
    ST: Set<T>,
{
    type Output = HashSet<T, RandomState>;

    fn bitxor(self, rhs: &ST) -> Self::Output {
        self.symmetric_difference(rhs).cloned().collect()
    }
}

impl<T, S, ST> Sub<&ST> for &LengthSet<T, S>
where
    T: Hash + Eq + Len + Clone,
    S: PrimInt + Unsigned,
    ST: Set<T>,
{
    type Output = HashSet<T, RandomState>;

    fn sub(self, rhs: &ST) -> Self::Output {
        self.difference(rhs).cloned().collect()
    }
}

impl<T, S, ST> PartialEq<ST> for LengthSet<T, S>
where
    T: Hash + Eq + Len,
    S: PrimInt + Unsigned,
    ST: Set<T>,
{
    fn eq(&self, other: &ST) -> bool {
        if self.len() != other.len() {
            return false;
        }

        self.iter().all(|value| other.contains(value))
    }
}

impl<T, S> Eq for LengthSet<T, S>
where
    T: Hash + Eq + Len,
    S: PrimInt + Unsigned,
{
}
