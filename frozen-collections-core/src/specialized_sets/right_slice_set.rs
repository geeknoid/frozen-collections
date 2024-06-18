use std::borrow::Borrow;
use std::collections::HashSet;
use std::fmt::{Debug, Formatter, Result};
use std::hash::{BuildHasher, Hash, RandomState};
use std::ops::{BitAnd, BitOr, BitXor, Range, Sub};

use num_traits::{PrimInt, Unsigned};

use crate::specialized_maps::RightSliceMap;
use crate::specialized_sets::{IntoIter, Iter};
use crate::traits::Len;
use crate::traits::RangeHash;
use crate::traits::Set;

/// A set that hashes right-aligned slices of its values.
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
pub struct RightSliceSet<T, S = u8, BH = RandomState> {
    map: RightSliceMap<T, (), S, BH>,
}

impl<T, S> RightSliceSet<T, S, RandomState>
where
    T: RangeHash + Len + Hash + Eq,
    S: PrimInt + Unsigned,
{
    #[allow(clippy::missing_errors_doc)]
    pub fn try_from(
        payload: Vec<T>,
        range: Range<usize>,
    ) -> std::result::Result<Self, &'static str> {
        Self::with_hasher(payload, range, RandomState::new())
    }
}

impl<T, S, BH> RightSliceSet<T, S, BH>
where
    T: RangeHash + Len + Hash + Eq,
    S: PrimInt + Unsigned,
    BH: BuildHasher,
{
    #[allow(clippy::missing_errors_doc)]
    pub fn with_hasher(
        payload: Vec<T>,
        range: Range<usize>,
        bh: BH,
    ) -> std::result::Result<Self, &'static str> {
        Ok(Self {
            map: RightSliceMap::with_hasher(
                payload.into_iter().map(|x| (x, ())).collect(),
                range,
                bh,
            )?,
        })
    }
}

impl<T, S, BH> RightSliceSet<T, S, BH>
where
    S: PrimInt + Unsigned,
    BH: BuildHasher,
{
    #[inline]
    #[must_use]
    pub fn get<Q>(&self, value: &Q) -> Option<&T>
    where
        T: Borrow<Q>,
        Q: ?Sized + RangeHash + Len + Eq,
    {
        Some(self.map.get_key_value(value)?.0)
    }

    #[inline]
    #[must_use]
    pub fn contains<Q>(&self, value: &Q) -> bool
    where
        T: Borrow<Q>,
        Q: ?Sized + RangeHash + Len + Eq,
    {
        self.get(value).is_some()
    }
}

impl<T, S, BH> RightSliceSet<T, S, BH> {
    #[must_use]
    pub const fn iter(&self) -> Iter<T> {
        Iter::new(&self.map.table.entries)
    }

    #[must_use]
    pub const fn hasher(&self) -> &BH {
        self.map.hasher()
    }
}

impl<T, S, BH> Len for RightSliceSet<T, S, BH> {
    fn len(&self) -> usize {
        self.map.len()
    }
}

impl<T, S, BH> Debug for RightSliceSet<T, S, BH>
where
    T: Debug,
{
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        f.debug_set().entries(self.iter()).finish()
    }
}

impl<T, S, BH> Default for RightSliceSet<T, S, BH>
where
    S: PrimInt + Unsigned,
    BH: BuildHasher + Default,
{
    fn default() -> Self {
        Self {
            map: RightSliceMap::default(),
        }
    }
}

impl<T, S, BH> IntoIterator for RightSliceSet<T, S, BH> {
    type Item = T;
    type IntoIter = IntoIter<T>;

    fn into_iter(self) -> Self::IntoIter {
        IntoIter::new(self.map.table.entries)
    }
}

impl<'a, T, S, BH> IntoIterator for &'a RightSliceSet<T, S, BH> {
    type Item = &'a T;
    type IntoIter = Iter<'a, T>;

    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}

impl<T, S, BH> Set<T> for RightSliceSet<T, S, BH>
where
    T: RangeHash + Len + Eq,
    S: PrimInt + Unsigned,
    BH: BuildHasher,
{
    type Iterator<'a> = Iter<'a, T>
    where
        T: 'a,
        S: 'a,
        BH: 'a;

    fn iter(&self) -> Iter<'_, T> {
        self.iter()
    }

    fn contains(&self, value: &T) -> bool {
        self.contains(value)
    }
}

impl<T, S, ST, BH> BitOr<&ST> for &RightSliceSet<T, S, BH>
where
    T: RangeHash + Hash + Len + Eq + Clone,
    S: PrimInt + Unsigned,
    ST: Set<T>,
    BH: BuildHasher + Default,
{
    type Output = HashSet<T, BH>;

    fn bitor(self, rhs: &ST) -> Self::Output {
        self.union(rhs).cloned().collect()
    }
}

impl<T, S, ST, BH> BitAnd<&ST> for &RightSliceSet<T, S, BH>
where
    T: RangeHash + Hash + Len + Eq + Clone,
    S: PrimInt + Unsigned,
    ST: Set<T>,
    BH: BuildHasher + Default,
{
    type Output = HashSet<T, BH>;

    fn bitand(self, rhs: &ST) -> Self::Output {
        self.intersection(rhs).cloned().collect()
    }
}

impl<T, S, ST, BH> BitXor<&ST> for &RightSliceSet<T, S, BH>
where
    T: RangeHash + Hash + Len + Eq + Clone,
    S: PrimInt + Unsigned,
    ST: Set<T>,
    BH: BuildHasher + Default,
{
    type Output = HashSet<T, BH>;

    fn bitxor(self, rhs: &ST) -> Self::Output {
        self.symmetric_difference(rhs).cloned().collect()
    }
}

impl<T, S, ST, BH> Sub<&ST> for &RightSliceSet<T, S, BH>
where
    T: RangeHash + Hash + Len + Eq + Clone,
    S: PrimInt + Unsigned,
    ST: Set<T>,
    BH: BuildHasher + Default,
{
    type Output = HashSet<T, BH>;

    fn sub(self, rhs: &ST) -> Self::Output {
        self.difference(rhs).cloned().collect()
    }
}

impl<T, S, ST, BH> PartialEq<ST> for RightSliceSet<T, S, BH>
where
    T: RangeHash + Len + Eq,
    S: PrimInt + Unsigned,
    ST: Set<T>,
    BH: BuildHasher + Default,
{
    fn eq(&self, other: &ST) -> bool {
        if self.len() != other.len() {
            return false;
        }

        self.iter().all(|value| other.contains(value))
    }
}

impl<T, S, BH> Eq for RightSliceSet<T, S, BH>
where
    T: RangeHash + Len + Eq,
    S: PrimInt + Unsigned,
    BH: BuildHasher + Default,
{
}
