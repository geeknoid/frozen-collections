use crate::sets::{Difference, Intersection, SymmetricDifference, Union};
use crate::traits::Set;

/// Common operations on sets.
pub trait SetOps<T> {
    /// Visits the values representing the union,
    /// i.e., all the values in `self` or `other`, without duplicates.
    #[must_use]
    fn union<'a, ST>(&'a self, other: &'a ST) -> Union<'a, Self, ST, T>
    where
        ST: Set<T>,
        Self: Sized + Set<T>,
    {
        Union::new(self, other)
    }

    /// Visits the values representing the symmetric difference,
    /// i.e., the values that are in `self` or in `other` but not in both.
    #[must_use]
    fn symmetric_difference<'a, ST>(&'a self, other: &'a ST) -> SymmetricDifference<'a, Self, ST, T>
    where
        ST: Set<T>,
        Self: Sized + Set<T>,
    {
        SymmetricDifference::new(self, other)
    }

    /// Visits the values representing the difference,
    /// i.e., the values that are in `self` but not in `other`.
    #[must_use]
    fn difference<'a, ST>(&'a self, other: &'a ST) -> Difference<'a, Self, ST, T>
    where
        ST: Set<T>,
        Self: Sized + Set<T>,
    {
        Difference::new(self, other)
    }

    /// Visits the values representing the intersection,
    /// i.e., the values that are both in `self` and `other`.
    ///
    /// When an equal element is present in `self` and `other`,
    /// then the resulting `Intersection` may yield references to
    /// one or the other. This can be relevant if `T` contains fields which
    /// are not compared by its `Eq` implementation and may hold different
    /// values between the two equal copies of `T` in the two sets.
    #[must_use]
    fn intersection<'a, ST>(&'a self, other: &'a ST) -> Intersection<'a, Self, ST, T>
    where
        ST: Set<T>,
        Self: Sized + Set<T>,
    {
        Intersection::new(self, other)
    }

    /// Returns `true` if `self` has no entries in common with `other`.
    /// This is equivalent to checking for an empty intersection.
    #[must_use]
    #[mutants::skip]
    fn is_disjoint<'a, ST>(&'a self, other: &'a ST) -> bool
    where
        ST: Set<T>,
        Self: Sized + Set<T>,
    {
        if self.len() <= other.len() {
            self.iter().all(|v| !other.contains(v))
        } else {
            other.iter().all(|v| !self.contains(v))
        }
    }

    /// Returns `true` if the set is a subset of another,
    /// i.e., `other` contains at least all the values in `self`.
    #[must_use]
    fn is_subset<'a, ST>(&'a self, other: &'a ST) -> bool
    where
        ST: Set<T>,
        Self: Sized + Set<T>,
    {
        if self.len() <= other.len() {
            self.iter().all(|v| other.contains(v))
        } else {
            false
        }
    }

    /// Returns `true` if the set is a superset of another,
    /// i.e., `self` contains at least all the values in `other`.
    #[must_use]
    fn is_superset<'a, ST>(&'a self, other: &'a ST) -> bool
    where
        ST: Set<T>,
        Self: Sized + Set<T>,
    {
        if other.len() <= self.len() {
            other.iter().all(|v| self.contains(v))
        } else {
            false
        }
    }
}

impl<ST, T> SetOps<T> for ST where ST: Set<T> {}
