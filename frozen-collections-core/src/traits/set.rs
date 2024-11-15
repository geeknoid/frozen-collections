use crate::sets::{Difference, Intersection, SymmetricDifference, Union};
use crate::traits::{Len, SetIterator};

#[cfg(feature = "std")]
use core::hash::{BuildHasher, Hash};

/// Common abstractions for sets.
pub trait Set<T>: Len + SetIterator<T> {
    /// Checks whether a particular value is present in the set.
    #[must_use]
    fn contains(&self, value: &T) -> bool;

    /// Visits the values representing the union,
    /// i.e., all the values in `self` or `other`, without duplicates.
    #[must_use]
    fn union<'a, ST>(&'a self, other: &'a ST) -> Union<'a, Self, ST, T>
    where
        ST: Set<T>,
        Self: Sized,
    {
        Union::new(self, other)
    }

    /// Visits the values representing the symmetric difference,
    /// i.e., the values that are in `self` or in `other` but not in both.
    #[must_use]
    fn symmetric_difference<'a, ST>(&'a self, other: &'a ST) -> SymmetricDifference<'a, Self, ST, T>
    where
        ST: Set<T>,
        Self: Sized,
    {
        SymmetricDifference::new(self, other)
    }

    /// Visits the values representing the difference,
    /// i.e., the values that are in `self` but not in `other`.
    #[must_use]
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
    #[must_use]
    fn intersection<'a, ST>(&'a self, other: &'a ST) -> Intersection<'a, Self, ST, T>
    where
        ST: Set<T>,
        Self: Sized,
    {
        Intersection::new(self, other)
    }

    /// Returns `true` if `self` has no elements in common with `other`.
    /// This is equivalent to checking for an empty intersection.
    #[must_use]
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
    #[must_use]
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
    #[must_use]
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

#[cfg(feature = "std")]
impl<T, BH> Set<T> for std::collections::HashSet<T, BH>
where
    T: Hash + Eq,
    BH: BuildHasher,
{
    #[inline]
    fn contains(&self, value: &T) -> bool {
        Self::contains(self, value)
    }
}

#[cfg(feature = "std")]
impl<T> Set<T> for std::collections::BTreeSet<T>
where
    T: Ord,
{
    #[inline]
    fn contains(&self, value: &T) -> bool {
        Self::contains(self, value)
    }
}

impl<T, BH> Set<T> for hashbrown::hash_set::HashSet<T, BH>
where
    T: Hash + Eq,
    BH: BuildHasher,
{
    #[inline]
    fn contains(&self, value: &T) -> bool {
        Self::contains(self, value)
    }
}

#[cfg(test)]
mod tests {
    use crate::traits::set_trait_tests::test_set_trait_impl;
    use hashbrown::HashSet as HashbrownSet;
    use std::collections::{BTreeSet, HashSet as StdHashSet};

    #[test]
    fn test_stdset() {
        let set = StdHashSet::from([1, 2, 3]);
        let reference = HashbrownSet::from([1, 2, 3]);
        let other = HashbrownSet::from([3, 4, 5]);
        test_set_trait_impl(&set, &reference, &other);

        let set = StdHashSet::from([]);
        let reference = HashbrownSet::from([]);
        let other = HashbrownSet::from([3, 4, 5]);
        test_set_trait_impl(&set, &reference, &other);

        let set = StdHashSet::from([3, 1, 2, 3, 3]);
        let reference = HashbrownSet::from([3, 1, 2, 3, 3]);
        let other = HashbrownSet::from([3, 4, 5]);
        test_set_trait_impl(&set, &reference, &other);

        let set = StdHashSet::from([1, 2, 3]);
        let reference = HashbrownSet::from([1, 2, 3]);
        let other = HashbrownSet::from([1, 2, 3, 4, 5]);
        test_set_trait_impl(&set, &reference, &other);

        let set = StdHashSet::from([1, 2, 3]);
        let reference = HashbrownSet::from([1, 2, 3]);
        let other = HashbrownSet::from([1, 2]);
        test_set_trait_impl(&set, &reference, &other);

        let set = StdHashSet::from([1, 2, 3]);
        let reference = HashbrownSet::from([1, 2, 3]);
        let other = HashbrownSet::from([]);
        test_set_trait_impl(&set, &reference, &other);
    }

    #[test]
    fn test_btreeset() {
        let set = BTreeSet::from([1, 2, 3]);
        let reference = HashbrownSet::from([1, 2, 3]);
        let other = HashbrownSet::from([3, 4, 5]);
        test_set_trait_impl(&set, &reference, &other);

        let set = BTreeSet::from([]);
        let reference = HashbrownSet::from([]);
        let other = HashbrownSet::from([3, 4, 5]);
        test_set_trait_impl(&set, &reference, &other);

        let set = BTreeSet::from([3, 1, 2, 3, 3]);
        let reference = HashbrownSet::from([3, 1, 2, 3, 3]);
        let other = HashbrownSet::from([3, 4, 5]);
        test_set_trait_impl(&set, &reference, &other);

        let set = BTreeSet::from([1, 2, 3]);
        let reference = HashbrownSet::from([1, 2, 3]);
        let other = HashbrownSet::from([1, 2, 3, 4, 5]);
        test_set_trait_impl(&set, &reference, &other);

        let set = BTreeSet::from([1, 2, 3]);
        let reference = HashbrownSet::from([1, 2, 3]);
        let other = HashbrownSet::from([1, 2]);
        test_set_trait_impl(&set, &reference, &other);

        let set = BTreeSet::from([1, 2, 3]);
        let reference = HashbrownSet::from([1, 2, 3]);
        let other = HashbrownSet::from([]);
        test_set_trait_impl(&set, &reference, &other);
    }

    #[test]
    fn test_hashbrownset() {
        let set = HashbrownSet::from([1, 2, 3]);
        let reference = HashbrownSet::from([1, 2, 3]);
        let other = HashbrownSet::from([3, 4, 5]);
        test_set_trait_impl(&set, &reference, &other);

        let set = HashbrownSet::from([]);
        let reference = HashbrownSet::from([]);
        let other = HashbrownSet::from([3, 4, 5]);
        test_set_trait_impl(&set, &reference, &other);

        let set = HashbrownSet::from([3, 1, 2, 3, 3]);
        let reference = HashbrownSet::from([3, 1, 2, 3, 3]);
        let other = HashbrownSet::from([3, 4, 5]);
        test_set_trait_impl(&set, &reference, &other);

        let set = HashbrownSet::from([1, 2, 3]);
        let reference = HashbrownSet::from([1, 2, 3]);
        let other = HashbrownSet::from([1, 2, 3, 4, 5]);
        test_set_trait_impl(&set, &reference, &other);

        let set = HashbrownSet::from([1, 2, 3]);
        let reference = HashbrownSet::from([1, 2, 3]);
        let other = HashbrownSet::from([1, 2]);
        test_set_trait_impl(&set, &reference, &other);

        let set = HashbrownSet::from([1, 2, 3]);
        let reference = HashbrownSet::from([1, 2, 3]);
        let other = HashbrownSet::from([]);
        test_set_trait_impl(&set, &reference, &other);
    }
}
