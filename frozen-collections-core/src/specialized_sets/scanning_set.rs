use std::borrow::Borrow;
use std::collections::HashSet;
use std::fmt::{Debug, Formatter, Result};
use std::hash::{Hash, RandomState};
use std::ops::{BitAnd, BitOr, BitXor, Sub};

use crate::specialized_maps::ScanningMap;
use crate::specialized_sets::utils::partial_eq;
use crate::specialized_sets::{IntoIter, Iter};
use crate::traits::Len;
use crate::traits::Set;

/// A general purpose set that uses linear scan of values rather than a hash table.
///
/// # Capacity Constraints
///
/// The `S` generic argument controls the maximum capacity
/// of the set. A `u8` will allow up to 255 elements, `u16`
/// will allow up to 65,535 elements, and `usize` will allow
/// up to [`usize::MAX`] elements.
///
/// # Important Note
///
/// This type is not intended to be used directly by
/// application code. Instead, applications are expected
/// to use the `FrozenSet` type or the `frozen_set!` macro.
#[derive(Clone)]
pub struct ScanningSet<T> {
    map: ScanningMap<T, ()>,
}

impl<T> ScanningSet<T>
where
    T: Hash + Eq,
{
    #[allow(clippy::missing_errors_doc)]
    pub fn new(payload: Vec<T>) -> std::result::Result<Self, &'static str> {
        Ok(Self {
            map: ScanningMap::try_from(Vec::from_iter(payload.into_iter().map(|x| (x, ()))))?,
        })
    }
}

impl<T> ScanningSet<T> {
    #[inline]
    #[must_use]
    pub fn get<Q>(&self, value: &Q) -> Option<&T>
    where
        T: Borrow<Q>,
        Q: ?Sized + Eq,
    {
        Some(self.map.get_key_value(value)?.0)
    }

    #[inline]
    #[must_use]
    pub fn contains<Q>(&self, value: &Q) -> bool
    where
        T: Borrow<Q>,
        Q: ?Sized + Eq,
    {
        self.get(value).is_some()
    }
}

impl<T> Len for ScanningSet<T> {
    fn len(&self) -> usize {
        self.map.len()
    }
}

impl<T> Debug for ScanningSet<T>
where
    T: Eq + Debug,
{
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        f.debug_set().entries(self.iter()).finish()
    }
}

impl<T> Default for ScanningSet<T> {
    fn default() -> Self {
        Self {
            map: ScanningMap::default(),
        }
    }
}

impl<T> IntoIterator for ScanningSet<T> {
    type Item = T;
    type IntoIter = IntoIter<T>;

    fn into_iter(self) -> Self::IntoIter {
        IntoIter::new(self.map.entries)
    }
}

impl<'a, T> IntoIterator for &'a ScanningSet<T>
where
    T: Eq,
{
    type Item = &'a T;
    type IntoIter = Iter<'a, T>;

    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}

impl<T> TryFrom<Vec<T>> for ScanningSet<T>
where
    T: Hash + Eq,
{
    type Error = &'static str;

    fn try_from(payload: Vec<T>) -> std::result::Result<Self, Self::Error> {
        Self::new(payload)
    }
}

impl<T, const N: usize> TryFrom<[T; N]> for ScanningSet<T>
where
    T: Hash + Eq,
{
    type Error = &'static str;

    fn try_from(payload: [T; N]) -> std::result::Result<Self, Self::Error> {
        Ok(Self {
            map: ScanningMap::try_from(Vec::from_iter(payload.into_iter().map(|x| (x, ()))))?,
        })
    }
}

impl<T> FromIterator<T> for ScanningSet<T>
where
    T: Hash + Eq,
{
    fn from_iter<I: IntoIterator<Item = T>>(iter: I) -> Self {
        Self {
            map: iter.into_iter().map(|x| (x, ())).collect(),
        }
    }
}

impl<T> Set<T> for ScanningSet<T>
where
    T: Eq,
{
    type Iterator<'a> = Iter<'a, T>
    where
        T: 'a;

    fn iter(&self) -> Iter<'_, T> {
        Iter::new(&self.map.entries)
    }

    fn contains(&self, value: &T) -> bool {
        self.contains(value)
    }
}

impl<T, ST> BitOr<&ST> for &ScanningSet<T>
where
    T: Hash + Eq + Clone,
    ST: Set<T>,
{
    type Output = HashSet<T, RandomState>;

    fn bitor(self, rhs: &ST) -> Self::Output {
        self.union(rhs).cloned().collect()
    }
}

impl<T, ST> BitAnd<&ST> for &ScanningSet<T>
where
    T: Hash + Eq + Clone,
    ST: Set<T>,
{
    type Output = HashSet<T, RandomState>;

    fn bitand(self, rhs: &ST) -> Self::Output {
        self.intersection(rhs).cloned().collect()
    }
}

impl<T, ST> BitXor<&ST> for &ScanningSet<T>
where
    T: Hash + Eq + Clone,
    ST: Set<T>,
{
    type Output = HashSet<T, RandomState>;

    fn bitxor(self, rhs: &ST) -> Self::Output {
        self.symmetric_difference(rhs).cloned().collect()
    }
}

impl<T, ST> Sub<&ST> for &ScanningSet<T>
where
    T: Hash + Eq + Clone,
    ST: Set<T>,
{
    type Output = HashSet<T, RandomState>;

    fn sub(self, rhs: &ST) -> Self::Output {
        self.difference(rhs).cloned().collect()
    }
}

impl<T, ST> PartialEq<ST> for ScanningSet<T>
where
    T: Hash + Eq,
    ST: Set<T>,
{
    partial_eq!();
}

impl<T> Eq for ScanningSet<T> where T: Hash + Eq {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_creates_set_correctly() {
        let set = ScanningSet::new(vec![1, 2, 3]).unwrap();
        assert_eq!(set.len(), 3);
    }

    #[test]
    fn get_returns_correct_item() {
        let set = ScanningSet::new(vec![1, 2, 3]).unwrap();
        assert!(set.get(&2).is_some());
        assert!(set.get(&4).is_none());
    }

    #[test]
    fn contains_checks_for_item_existence() {
        let set = ScanningSet::new(vec![1, 2, 3]).unwrap();
        assert!(set.contains(&1));
        assert!(!set.contains(&4));
    }

    #[test]
    fn iter_iterates_over_all_items() {
        let set = ScanningSet::new(vec![1, 2, 3]).unwrap();
        let mut iter = set.iter();
        assert_eq!(iter.next().copied(), Some(1));
        assert_eq!(iter.next().copied(), Some(2));
        assert_eq!(iter.next().copied(), Some(3));
        assert!(iter.next().is_none());
    }

    #[test]
    fn len_returns_correct_length() {
        let set = ScanningSet::new(vec![1, 2, 3]).unwrap();
        assert_eq!(set.len(), 3);
    }

    #[test]
    fn default_creates_empty_set() {
        let set: ScanningSet<i32> = ScanningSet::default();
        assert_eq!(set.len(), 0);
    }

    #[test]
    fn into_iter_works_for_set() {
        let set = ScanningSet::new(vec![1, 2, 3]).unwrap();
        let iter = set.into_iter();
        let mut v = Vec::from_iter(iter);
        v.sort_unstable();

        let mut iter = v.iter();
        assert_eq!(iter.next(), Some(&1));
        assert_eq!(iter.next(), Some(&2));
        assert_eq!(iter.next(), Some(&3));
        assert!(iter.next().is_none());
    }

    #[test]
    fn into_iter_works_for_set_ref() {
        let set = ScanningSet::new(vec![4, 5, 6]).unwrap();
        let iter = (&set).into_iter();
        let mut v = Vec::from_iter(iter);
        v.sort_unstable();

        let mut iter = v.iter();
        assert_eq!(iter.next(), Some(&&4));
        assert_eq!(iter.next(), Some(&&5));
        assert_eq!(iter.next(), Some(&&6));
        assert!(iter.next().is_none());
    }

    #[test]
    fn test_equality_with_self() {
        let set = ScanningSet::new(vec![1, 2, 3]).unwrap();
        assert_eq!(set, set);
    }

    #[test]
    fn test_equality_with_identical_set() {
        let set1 = ScanningSet::new(vec![1, 2, 3]).unwrap();
        let set2 = ScanningSet::new(vec![1, 2, 3]).unwrap();
        assert_eq!(set1, set2);
    }

    #[test]
    fn test_inequality_with_different_sets() {
        let set1 = ScanningSet::new(vec![1, 2, 3]).unwrap();
        let set2 = ScanningSet::new(vec![4, 5, 6]).unwrap();
        assert_ne!(set1, set2);
    }

    #[test]
    fn test_inequality_with_different_sizes() {
        let set1 = ScanningSet::new(vec![1, 2, 3]).unwrap();
        let set2 = ScanningSet::new(vec![1, 2]).unwrap();
        assert_ne!(set1, set2);
    }

    #[test]
    fn intersection_with_common_elements() {
        let set1 = ScanningSet::new(vec![1, 2, 3]).unwrap();
        let set2 = ScanningSet::new(vec![2, 3, 4]).unwrap();
        let result: HashSet<_, _> = (&set1 & &set2).into_iter().collect();
        let expected: HashSet<_> = [2, 3].iter().copied().collect();
        assert_eq!(result, expected);
    }

    #[test]
    fn intersection_with_no_common_elements() {
        let set1 = ScanningSet::new(vec![1, 2, 3]).unwrap();
        let set2 = ScanningSet::new(vec![4, 5, 6]).unwrap();
        let result: HashSet<_> = (&set1 & &set2).into_iter().collect();
        assert!(result.is_empty());
    }

    #[test]
    fn intersection_with_self() {
        let set = ScanningSet::new(vec![1, 2, 3]).unwrap();
        let result: HashSet<_, _> = (&set & &set).into_iter().collect();
        let expected: HashSet<_> = set.into_iter().collect();
        assert_eq!(result, expected);
    }

    #[test]
    fn intersection_of_non_empty_with_empty_set() {
        let set1 = ScanningSet::new(vec![1, 2, 3]).unwrap();
        let set2: ScanningSet<i32> = ScanningSet::new(vec![]).unwrap();
        let result: HashSet<_> = (&set1 & &set2).into_iter().collect();
        assert!(result.is_empty());
    }

    #[test]
    fn test_debug_output() {
        let set = ScanningSet::new(vec![1, 2, 3]).unwrap();
        let debug_str = format!("{set:?}");
        let expected = "{1, 2, 3}";
        assert_eq!(debug_str, expected);
    }

    #[test]
    fn symmetric_difference_with_common_elements() {
        let set1 = ScanningSet::new(vec![1, 2, 3]).unwrap();
        let set2 = ScanningSet::new(vec![3, 4, 5]).unwrap();
        let result: HashSet<_, _> = (&set1 ^ &set2).into_iter().collect();
        let expected: HashSet<_> = [1, 2, 4, 5].iter().copied().collect();
        assert_eq!(result, expected);
    }

    #[test]
    fn symmetric_difference_with_no_common_elements() {
        let set1 = ScanningSet::new(vec![1, 2, 3]).unwrap();
        let set2 = ScanningSet::new(vec![4, 5, 6]).unwrap();
        let result: HashSet<_, _> = (&set1 ^ &set2).into_iter().collect();
        let expected: HashSet<_> = [1, 2, 3, 4, 5, 6].iter().copied().collect();
        assert_eq!(result, expected);
    }

    #[test]
    fn symmetric_difference_with_self() {
        let set = ScanningSet::new(vec![1, 2, 3]).unwrap();
        let result: HashSet<_> = (&set ^ &set).into_iter().collect();
        assert!(result.is_empty());
    }

    #[test]
    fn union_with_common_elements() {
        let set1 = ScanningSet::new(vec![1, 2, 3]).unwrap();
        let set2 = ScanningSet::new(vec![3, 4, 5]).unwrap();
        let result: HashSet<_, _> = (&set1 | &set2).into_iter().collect();
        let expected: HashSet<_> = [1, 2, 3, 4, 5].iter().copied().collect();
        assert_eq!(result, expected);
    }

    #[test]
    fn union_with_self() {
        let set = ScanningSet::new(vec![1, 2, 3]).unwrap();
        let result: HashSet<_, _> = (&set | &set).into_iter().collect();
        let expected: HashSet<_> = set.into_iter().collect();
        assert_eq!(result, expected);
    }

    #[test]
    fn union_of_empty_with_non_empty_set() {
        let set1: ScanningSet<i32> = ScanningSet::new(vec![]).unwrap();
        let set2 = ScanningSet::new(vec![1, 2, 3]).unwrap();
        let result: HashSet<_> = (&set1 | &set2).into_iter().collect();
        let expected: HashSet<_> = [1, 2, 3].iter().copied().collect();
        assert_eq!(result, expected);
    }

    #[test]
    fn try_from_vec_succeeds() {
        let vec = vec![1, 2, 3];
        let set = ScanningSet::try_from(vec).expect("Failed to create ScanningSet from Vec");
        let expected: HashSet<_> = [1, 2, 3].iter().copied().collect();
        assert_eq!(set.into_iter().collect::<HashSet<_>>(), expected);
    }

    #[test]
    fn try_from_array_succeeds() {
        let array = [1, 2, 3];
        let set = ScanningSet::try_from(array).expect("Failed to create ScanningSet from array");
        let expected: HashSet<_> = [1, 2, 3].iter().copied().collect();
        assert_eq!(set.into_iter().collect::<HashSet<_>>(), expected);
    }

    #[test]
    fn from_iter_succeeds() {
        let iter = [1, 2, 3].iter().copied();
        let set: ScanningSet<_> = iter.collect();
        let expected: HashSet<_> = [1, 2, 3].iter().copied().collect();
        assert_eq!(set.into_iter().collect::<HashSet<_>>(), expected);
    }

    #[test]
    fn difference_with_common_elements() {
        let set1 = ScanningSet::new(vec![1, 2, 3]).unwrap();
        let set2 = ScanningSet::new(vec![2, 3, 4]).unwrap();
        let result: HashSet<_> = (&set1 - &set2).into_iter().collect();
        let expected: HashSet<_> = std::iter::once(1).collect();
        assert_eq!(result, expected);
    }

    #[test]
    fn difference_with_no_common_elements() {
        let set1 = ScanningSet::new(vec![1, 2, 3]).unwrap();
        let set2 = ScanningSet::new(vec![4, 5, 6]).unwrap();
        let result: HashSet<_> = (&set1 - &set2).into_iter().collect();
        let expected: HashSet<_> = [1, 2, 3].iter().copied().collect();
        assert_eq!(result, expected);
    }

    #[test]
    fn difference_with_self() {
        let set = ScanningSet::new(vec![1, 2, 3]).unwrap();
        let result: HashSet<_> = (&set - &set).into_iter().collect();
        assert!(result.is_empty());
    }

    #[test]
    fn difference_of_non_empty_with_empty_set() {
        let set1 = ScanningSet::new(vec![1, 2, 3]).unwrap();
        let set2: ScanningSet<i32> = ScanningSet::new(vec![]).unwrap();
        let result: HashSet<_> = (&set1 - &set2).into_iter().collect();
        let expected: HashSet<_> = [1, 2, 3].iter().copied().collect();
        assert_eq!(result, expected);
    }
}
