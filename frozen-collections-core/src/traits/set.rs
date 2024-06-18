use std::collections::hash_set::Iter;
use std::collections::{BTreeSet, HashSet};
use std::hash::{BuildHasher, Hash};

use crate::specialized_sets::{Difference, Intersection, SymmetricDifference, Union};
use crate::traits::Len;

/// Common abstractions for sets.
pub trait Set<T>: Len {
    type Iterator<'a>: Iterator<Item = &'a T>
    where
        Self: 'a,
        T: 'a;

    /// An iterator visiting all elements in arbitrary order.
    /// The iterator element type is `&'a T`.
    fn iter(&self) -> Self::Iterator<'_>;

    /// Checks whether a particular value is present in the set.
    fn contains(&self, value: &T) -> bool;

    /// Visits the values representing the union,
    /// i.e., all the values in `self` or `other`, without duplicates.
    fn union<'a, ST>(&'a self, other: &'a ST) -> Union<'a, Self, ST, T>
    where
        ST: Set<T>,
        Self: Sized,
    {
        Union::new(self, other)
    }

    /// Visits the values representing the symmetric difference,
    /// i.e., the values that are in `self` or in `other` but not in both.
    fn symmetric_difference<'a, ST>(&'a self, other: &'a ST) -> SymmetricDifference<'a, Self, ST, T>
    where
        ST: Set<T>,
        Self: Sized,
    {
        SymmetricDifference::new(self, other)
    }

    /// Visits the values representing the difference,
    /// i.e., the values that are in `self` but not in `other`.
    fn difference<'a, ST>(&'a self, other: &'a ST) -> Difference<'a, Self, ST, T>
    where
        ST: Set<T>,
        Self: Sized,
    {
        Difference::new(self, other)
    }

    /// Visits the values representing the intersection,
    /// i.e., the values that are both in `self` and `other`.
    ///
    /// When an equal element is present in `self` and `other`
    /// then the resulting `Intersection` may yield references to
    /// one or the other. This can be relevant if `T` contains fields which
    /// are not compared by its `Eq` implementation, and may hold different
    /// value between the two equal copies of `T` in the two sets.
    fn intersection<'a, ST>(&'a self, other: &'a ST) -> Intersection<'a, Self, ST, T>
    where
        ST: Set<T>,
        Self: Sized,
    {
        Intersection::new(self, other)
    }

    /// Returns `true` if `self` has no elements in common with `other`.
    /// This is equivalent to checking for an empty intersection.
    fn is_disjoint<'a, ST>(&'a self, other: &'a ST) -> bool
    where
        ST: Set<T>,
        Self: Sized,
    {
        if self.len() <= self.len() {
            self.iter().all(|v| !other.contains(v))
        } else {
            other.iter().all(|v| !self.contains(v))
        }
    }

    /// Returns `true` if the set is a subset of another,
    /// i.e., `other` contains at least all the values in `self`.
    fn is_subset<'a, ST>(&'a self, other: &'a ST) -> bool
    where
        ST: Set<T>,
        Self: Sized,
    {
        if self.len() <= other.len() {
            self.iter().all(|v| other.contains(v))
        } else {
            false
        }
    }

    /// Returns `true` if the set is a superset of another,
    /// i.e., `self` contains at least all the values in `other`.
    fn is_superset<'a, ST>(&'a self, other: &'a ST) -> bool
    where
        ST: Set<T>,
        Self: Sized,
    {
        if other.len() <= self.len() {
            other.iter().all(|v| self.contains(v))
        } else {
            false
        }
    }
}

impl<T, BH> Set<T> for HashSet<T, BH>
where
    T: Hash + Eq,
    BH: BuildHasher,
{
    type Iterator<'a> = Iter<'a, T>
    where
        T: 'a,
        BH: 'a;

    fn iter(&self) -> Self::Iterator<'_> {
        Self::iter(self)
    }

    fn contains(&self, value: &T) -> bool {
        Self::contains(self, value)
    }
}

impl<T> Set<T> for BTreeSet<T>
where
    T: Ord,
{
    type Iterator<'a> = std::collections::btree_set::Iter<'a, T>
    where
        T: 'a;

    fn iter(&self) -> Self::Iterator<'_> {
        Self::iter(self)
    }

    fn contains(&self, value: &T) -> bool {
        Self::contains(self, value)
    }
}
