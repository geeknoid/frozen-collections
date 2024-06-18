use alloc::vec::Vec;
use core::borrow::Borrow;
use core::fmt::{Debug, Formatter, Result};
use core::hash::Hash;
use core::ops::{BitAnd, BitOr, BitXor, Sub};
use frozen_collections_core::sets::{BinarySearchSet, IntoIter, Iter, OrderedScanSet};
use frozen_collections_core::traits::{Len, Set, SetIterator};
use hashbrown::HashSet as HashbrownSet;

/// The different implementations available for use, depending on the entries.
#[derive(Clone)]
enum SetTypes<T> {
    BinarySearch(BinarySearchSet<T>),
    Scanning(OrderedScanSet<T>),
}

/// A set optimized for fast read access.
///
/// A frozen set differs from the traditional [`HashSet`](std::collections::HashSet) type in three key ways. First, creating
/// a mew frozen set can take a relatively long time, especially for very large sets. Second,
/// once created, instances of frozen sets are immutable. And third, probing a frozen set is
/// typically considerably faster, which is the whole point
///
/// The reason creating a frozen set can take some time is due to the extensive analysis that is
/// performed on the set's values in order to determine the best set implementation strategy and
/// data layout to use. This analysis is what enables frozen sets to be faster later when
/// probing the set.
///
/// Frozen sets are intended for long-lived sets, where the cost of creating the set is made up
/// over time by the faster probing performance.
///
/// A [`FzOrderedSet`] requires that the elements
/// implement the [`Eq`] and [`Ord`] traits. This can frequently be achieved by
/// using `#[derive(PartialEq, Eq, Ord)]`.
///
/// It is a logic error for a value to be modified in such a way that the value's
/// order, as determined by the [`Ord`] trait, or its equality, as determined by
/// the [`Eq`] trait, changes while it is in the set. This is normally only
/// possible through [`core::cell::Cell`], [`core::cell::RefCell`], global state, I/O, or unsafe code.
///
/// The behavior resulting from the above logic error is not specified, but will
/// be encapsulated to the `FrozenSet` that observed the logic error and not
/// result in undefined behavior. This could include panics, incorrect results,
/// aborts, memory leaks, and non-termination.
///
/// # Alternate Choices
///
/// If your values are integer or strings, you should use the [`FzScalarSet`](crate::FzScalarSet) and
/// [`FzStringSet`](crate::FzStringSet)
/// types instead, you'll get better performance.
///
/// If all your values are known at compile time, you are much better off using the
/// [`fz_ordered_set!`](crate::fz_ordered_set) macro rather than this type. This will result in considerably
/// better performance.
///
/// # Examples
///
/// The easiest way to use `FzOrderedSet` with a custom type is to derive
/// [`Eq`] and [`Ord`]. We must also derive [`PartialEq`],
/// which is required if [`Eq`] is derived.
///
/// ```
/// # use frozen_collections::FzOrderedSet;
/// #
/// #[derive(PartialOrd, Ord, Eq, PartialEq, Debug)]
/// struct Viking {
///     name: String,
///     power: usize,
/// }
///
/// let vikings = FzOrderedSet::new(vec![
///     Viking {name: "Einar".to_string(), power: 9 },
///     Viking { name: "Olaf".to_string(), power: 4 },
///     Viking { name: "Harald".to_string(), power: 8 }]);
///
/// assert!(vikings.contains(&Viking {name: "Einar".to_string(), power: 9 }));
/// assert!(!vikings.contains(&Viking {name: "Einar".to_string(), power: 10 }));
///
/// // Print out all the vikings in non-deterministic order.
/// for x in &vikings {
///     println!("{x:?}");
/// }
/// ```
#[derive(Clone)]
#[allow(clippy::module_name_repetitions)]
pub struct FzOrderedSet<T> {
    set_impl: SetTypes<T>,
}

impl<T> FzOrderedSet<T>
where
    T: Ord + Eq,
{
    /// Creates a new frozen ordered set.
    ///
    /// # Examples
    ///
    /// ```
    /// # use frozen_collections::FzOrderedSet;
    /// # use frozen_collections::Len;
    ///
    /// let set = FzOrderedSet::new(vec![1, 2, 3]);
    ///
    /// assert_eq!(set.len(), 3);
    /// assert!(set.contains(&1));
    /// ```
    #[must_use]
    pub fn new(entries: Vec<T>) -> Self {
        Self {
            set_impl: if entries.len() <= 5 {
                SetTypes::Scanning(OrderedScanSet::new(entries))
            } else {
                SetTypes::BinarySearch(BinarySearchSet::new(entries))
            },
        }
    }
}

impl<T> FzOrderedSet<T> {
    /// Returns `true` if the set contains a value.
    ///
    /// # Examples
    ///
    /// ```
    /// # use frozen_collections::FzOrderedSet;
    /// #
    /// let set = FzOrderedSet::new(vec![1, 2, 3]);
    ///
    /// assert!(set.contains(&1));
    /// assert!(!set.contains(&4));
    /// ```
    #[inline(always)]
    #[must_use]
    pub fn contains<Q>(&self, value: &Q) -> bool
    where
        T: Borrow<Q>,
        Q: Ord + Eq + ?Sized,
    {
        match &self.set_impl {
            SetTypes::BinarySearch(s) => s.contains(value),
            SetTypes::Scanning(s) => s.contains(value),
        }
    }

    /// Returns a reference to the value in the set, if any, that is equal to the given value.
    ///
    /// # Examples
    ///
    /// ```
    /// # use frozen_collections::FzOrderedSet;
    /// #
    /// let set = FzOrderedSet::new(vec![1, 2, 3]);
    ///
    /// assert_eq!(set.get(&2), Some(&2));
    /// assert_eq!(set.get(&4), None);
    /// ```
    #[inline]
    #[must_use]
    pub fn get<Q>(&self, value: &Q) -> Option<&T>
    where
        T: Borrow<Q>,
        Q: Ord + Eq + ?Sized,
    {
        match &self.set_impl {
            SetTypes::BinarySearch(s) => s.get(value),
            SetTypes::Scanning(s) => s.get(value),
        }
    }
}

impl<T> From<Vec<T>> for FzOrderedSet<T>
where
    T: Ord + Eq,
{
    fn from(entries: Vec<T>) -> Self {
        Self::new(entries)
    }
}

impl<T, const N: usize> From<[T; N]> for FzOrderedSet<T>
where
    T: Ord + Eq,
{
    fn from(entries: [T; N]) -> Self {
        Self::new(Vec::from_iter(entries))
    }
}

impl<T> FromIterator<T> for FzOrderedSet<T>
where
    T: Ord + Eq,
{
    fn from_iter<U: IntoIterator<Item = T>>(iter: U) -> Self {
        Self::new(Vec::from_iter(iter))
    }
}

impl<T> Len for FzOrderedSet<T> {
    fn len(&self) -> usize {
        match &self.set_impl {
            SetTypes::BinarySearch(s) => Len::len(s),
            SetTypes::Scanning(s) => Len::len(s),
        }
    }
}

impl<T> Default for FzOrderedSet<T> {
    fn default() -> Self {
        Self {
            set_impl: SetTypes::Scanning(OrderedScanSet::default()),
        }
    }
}

impl<T> Debug for FzOrderedSet<T>
where
    T: Debug,
{
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        match &self.set_impl {
            SetTypes::BinarySearch(s) => s.fmt(f),
            SetTypes::Scanning(s) => s.fmt(f),
        }
    }
}

impl<T, ST> PartialEq<ST> for FzOrderedSet<T>
where
    T: Ord + Eq,
    ST: Set<T>,
{
    fn eq(&self, other: &ST) -> bool {
        if self.len() != other.len() {
            return false;
        }

        self.iter().all(|value| other.contains(value))
    }
}

impl<T> Eq for FzOrderedSet<T> where T: Ord + Eq {}

impl<T, ST> BitOr<&ST> for &FzOrderedSet<T>
where
    T: Hash + Ord + Eq + Clone,
    ST: Set<T>,
{
    type Output = HashbrownSet<T>;

    fn bitor(self, rhs: &ST) -> Self::Output {
        self.union(rhs).cloned().collect()
    }
}

impl<T, ST> BitAnd<&ST> for &FzOrderedSet<T>
where
    T: Hash + Ord + Eq + Clone,
    ST: Set<T>,
{
    type Output = HashbrownSet<T>;

    fn bitand(self, rhs: &ST) -> Self::Output {
        self.intersection(rhs).cloned().collect()
    }
}

impl<T, ST> BitXor<&ST> for &FzOrderedSet<T>
where
    T: Hash + Ord + Eq + Clone,
    ST: Set<T>,
{
    type Output = HashbrownSet<T>;

    fn bitxor(self, rhs: &ST) -> Self::Output {
        self.symmetric_difference(rhs).cloned().collect()
    }
}

impl<T, ST> Sub<&ST> for &FzOrderedSet<T>
where
    T: Hash + Ord + Eq + Clone,
    ST: Set<T>,
{
    type Output = HashbrownSet<T>;

    fn sub(self, rhs: &ST) -> Self::Output {
        self.difference(rhs).cloned().collect()
    }
}

impl<T> IntoIterator for FzOrderedSet<T> {
    type Item = T;
    type IntoIter = IntoIter<T>;

    fn into_iter(self) -> Self::IntoIter {
        match self.set_impl {
            SetTypes::BinarySearch(s) => s.into_iter(),
            SetTypes::Scanning(s) => s.into_iter(),
        }
    }
}

impl<'a, T> IntoIterator for &'a FzOrderedSet<T> {
    type Item = &'a T;
    type IntoIter = Iter<'a, T>;

    fn into_iter(self) -> Iter<'a, T> {
        self.iter()
    }
}

impl<T> SetIterator<T> for FzOrderedSet<T> {
    type Iterator<'a>
        = Iter<'a, T>
    where
        T: 'a;

    fn iter(&self) -> Iter<'_, T> {
        match &self.set_impl {
            SetTypes::BinarySearch(s) => s.iter(),
            SetTypes::Scanning(s) => s.iter(),
        }
    }
}

impl<T> Set<T> for FzOrderedSet<T>
where
    T: Ord + Eq,
{
    fn contains(&self, value: &T) -> bool {
        self.contains(value)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use frozen_collections_core::hashers::BridgeHasher;
    use frozen_collections_core::sets::HashSet;
    use frozen_collections_core::traits::LargeCollection;
    use std::hash::RandomState;

    #[test]
    fn little() {
        run_tests((0..2).collect());
    }

    #[test]
    fn medium() {
        run_tests((0..10).collect());
    }

    #[test]
    fn large() {
        run_tests((0..300).collect());
    }

    fn run_tests(entries: Vec<i32>) {
        misc(entries.clone());
        iter(entries.clone());
        fmt(entries.clone());
        bits(entries);
    }

    fn misc(entries: Vec<i32>) {
        let set = FzOrderedSet::new(entries.clone());
        let mut hs = HashbrownSet::<i32>::from_iter(entries.iter().copied());

        assert_eq!(entries.len(), set.len());

        for value in &entries {
            assert!(set.contains(value));
            assert_eq!(value, set.get(value).unwrap());
        }

        assert!(!set.contains(&-1));
        assert_eq!(entries.len(), set.iter().count());
        assert_eq!(set, hs);

        hs.insert(-1);
        assert_ne!(set, hs);

        let set3 = FzOrderedSet::<i32>::from_iter(entries.clone());
        assert_eq!(set, set3);

        let set4 = FzOrderedSet::<i32>::from(entries);
        assert_eq!(set, set4);

        let set5 = FzOrderedSet::<i32>::from([0, 1, 2, 3]);
        assert_eq!(4, set5.len());
    }

    fn iter(entries: Vec<i32>) {
        let set = FzOrderedSet::new(entries);

        let hs = HashSet::<i32, LargeCollection, _>::new(
            HashSet::<&i32, LargeCollection, _>::new(
                set.iter().collect(),
                BridgeHasher::new(RandomState::default()),
            )
            .unwrap()
            .iter()
            .map(|x| **x)
            .collect(),
            BridgeHasher::new(RandomState::default()),
        )
        .unwrap();

        assert_eq!(set, hs);

        let other_hs = HashbrownSet::<i32, RandomState>::from_iter(set.clone());
        assert_eq!(hs, other_hs);

        let set_ref = &set.clone();
        let other_hs = HashbrownSet::<i32, RandomState>::from_iter(set_ref.into_iter().copied());
        assert_eq!(hs, other_hs);

        let other_hs = HashbrownSet::<i32, RandomState>::from_iter(set);
        assert_eq!(hs, other_hs);
    }

    fn fmt(entries: Vec<i32>) {
        let set = FzOrderedSet::new(entries.clone());

        let formatted_set = format!("{set:?}");
        for value in entries {
            let value_str = format!("{value:?}");
            assert!(
                formatted_set.contains(&value_str),
                "Formatted string does not contain value: {value:?}"
            );
        }
    }

    fn bits(entries: Vec<i32>) {
        let fs1 = FzOrderedSet::new(entries.clone());
        let fs2 = FzOrderedSet::new(entries.iter().map(|x| x + 1).collect());

        let hs1 = HashbrownSet::<i32>::from_iter(entries.clone());
        let hs2 = HashbrownSet::<i32>::from_iter(entries.into_iter().map(|x| x + 1));

        assert_eq!(&fs1 | &fs2, &hs1 | &hs2);
        assert_eq!(&fs1 & &fs2, &hs1 & &hs2);
        assert_eq!(&fs1 ^ &fs2, &hs1 ^ &hs2);
        assert_eq!(&fs1 - &fs2, &hs1 - &hs2);
    }

    #[test]
    fn default() {
        let fs = FzOrderedSet::<i32>::default();
        assert_eq!(0, fs.len());
    }
}
