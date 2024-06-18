use alloc::vec::Vec;
use core::borrow::Borrow;
use core::fmt::{Debug, Formatter, Result};
use core::hash::Hash;
use core::ops::{BitAnd, BitOr, BitXor, Sub};

use frozen_collections_core::analyzers::{analyze_scalar_keys, ScalarKeyAnalysisResult};
use frozen_collections_core::hashers::PassthroughHasher;
use frozen_collections_core::sets::{
    DenseScalarLookupSet, HashSet, IntoIter, Iter, SparseScalarLookupSet,
};
use frozen_collections_core::traits::{LargeCollection, Len, Scalar, Set, SetIterator};
use frozen_collections_core::utils::dedup_keep_last;
use hashbrown::HashSet as HashbrownSet;

/// The different implementations available for use, depending on the entries.
#[derive(Clone)]
enum SetTypes<T> {
    Hash(HashSet<T, LargeCollection, PassthroughHasher>),
    Dense(DenseScalarLookupSet<T>),
    Sparse(SparseScalarLookupSet<T, LargeCollection>),
}

/// A set optimized for fast read access of integer or enum values.
///
/// A frozen set differs from the traditional [`HashSet`] type in three key ways. First, creating
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
/// # Alternate Choices
///
/// If all your values are known at compile time, you are much better off using the
/// [`fz_scalar_set!`](crate::fz_scalar_set) macro rather than this type. This will result in considerably
/// better performance.
///
/// # Examples
///
/// ```
/// # use frozen_collections::FzScalarSet;
/// # use frozen_collections::Len;
/// #
/// let set = FzScalarSet::from([1, 2, 3]);
///
/// assert_eq!(3, set.len());
/// assert!(set.contains(&1));
/// assert!(!set.contains(&4));
///
/// // Iterate over everything.
/// for value in &set {
///     println!("{value}");
/// }
/// ```
#[derive(Clone)]
#[allow(clippy::module_name_repetitions)]
pub struct FzScalarSet<T> {
    set_impl: SetTypes<T>,
}

impl<T> FzScalarSet<T>
where
    T: Scalar,
{
    /// Creates a new frozen set.
    ///
    /// # Examples
    ///
    /// ```
    /// # use frozen_collections::FzScalarSet;
    /// # use frozen_collections::Len;
    /// #
    /// let set = FzScalarSet::new(vec![1, 2, 3]);
    ///
    /// assert_eq!(set.len(), 3);
    /// assert!(set.contains(&1));
    /// ```
    #[must_use]
    #[allow(clippy::missing_panics_doc)]
    pub fn new(mut entries: Vec<T>) -> Self {
        entries.sort();
        dedup_keep_last(&mut entries);

        Self {
            set_impl: match analyze_scalar_keys(entries.iter().copied()) {
                ScalarKeyAnalysisResult::DenseRange => {
                    SetTypes::Dense(DenseScalarLookupSet::new_raw(entries))
                }
                ScalarKeyAnalysisResult::SparseRange => {
                    SetTypes::Sparse(SparseScalarLookupSet::new_raw(entries))
                }
                ScalarKeyAnalysisResult::General => {
                    SetTypes::Hash(HashSet::new(entries, PassthroughHasher::new()).unwrap())
                }
            },
        }
    }
}

impl<T> FzScalarSet<T> {
    /// Returns `true` if the set contains a value.
    ///
    /// # Examples
    ///
    /// ```
    /// # use frozen_collections::FzScalarSet;
    /// #
    /// let set = FzScalarSet::from([1, 2, 3]);
    ///
    /// assert!(set.contains(&1));
    /// assert!(!set.contains(&4));
    /// ```
    #[inline(always)]
    #[must_use]
    pub fn contains<Q>(&self, value: &Q) -> bool
    where
        T: Borrow<Q>,
        Q: Scalar,
    {
        match &self.set_impl {
            SetTypes::Hash(s) => s.contains(value),
            SetTypes::Dense(s) => s.contains(value),
            SetTypes::Sparse(s) => s.contains(value),
        }
    }

    /// Returns a reference to the value in the set, if any, that is equal to the given value.
    ///
    /// # Examples
    ///
    /// ```
    /// # use frozen_collections::FzScalarSet;
    /// #
    /// let set = FzScalarSet::from([1, 2, 3]);
    ///
    /// assert_eq!(set.get(&2), Some(&2));
    /// assert_eq!(set.get(&4), None);
    /// ```
    #[inline]
    #[must_use]
    pub fn get<Q>(&self, value: &Q) -> Option<&T>
    where
        T: Borrow<Q>,
        Q: Scalar,
    {
        match &self.set_impl {
            SetTypes::Hash(s) => s.get(value),
            SetTypes::Dense(s) => s.get(value),
            SetTypes::Sparse(s) => s.get(value),
        }
    }
}

impl<T> From<Vec<T>> for FzScalarSet<T>
where
    T: Scalar,
{
    fn from(entries: Vec<T>) -> Self {
        Self::new(entries)
    }
}

impl<T, const N: usize> From<[T; N]> for FzScalarSet<T>
where
    T: Scalar,
{
    fn from(entries: [T; N]) -> Self {
        Self::new(Vec::from_iter(entries))
    }
}

impl<T> FromIterator<T> for FzScalarSet<T>
where
    T: Scalar,
{
    fn from_iter<U: IntoIterator<Item = T>>(iter: U) -> Self {
        Self::new(Vec::from_iter(iter))
    }
}

impl<T> Len for FzScalarSet<T> {
    fn len(&self) -> usize {
        match &self.set_impl {
            SetTypes::Hash(s) => Len::len(s),
            SetTypes::Dense(s) => Len::len(s),
            SetTypes::Sparse(s) => Len::len(s),
        }
    }
}

impl<T> Default for FzScalarSet<T>
where
    T: Scalar,
{
    fn default() -> Self {
        Self {
            set_impl: SetTypes::Dense(DenseScalarLookupSet::default()),
        }
    }
}

impl<T> Debug for FzScalarSet<T>
where
    T: Debug,
{
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        match &self.set_impl {
            SetTypes::Hash(s) => s.fmt(f),
            SetTypes::Dense(s) => s.fmt(f),
            SetTypes::Sparse(s) => s.fmt(f),
        }
    }
}

impl<T, ST> PartialEq<ST> for FzScalarSet<T>
where
    T: Scalar,
    ST: Set<T>,
{
    fn eq(&self, other: &ST) -> bool {
        if self.len() != other.len() {
            return false;
        }

        self.iter().all(|value| other.contains(value))
    }
}

impl<T> Eq for FzScalarSet<T> where T: Scalar {}

impl<T, ST> BitOr<&ST> for &FzScalarSet<T>
where
    T: Scalar + Hash,
    ST: Set<T>,
{
    type Output = HashbrownSet<T>;

    fn bitor(self, rhs: &ST) -> Self::Output {
        self.union(rhs).copied().collect()
    }
}

impl<T, ST> BitAnd<&ST> for &FzScalarSet<T>
where
    T: Scalar + Hash,
    ST: Set<T>,
{
    type Output = HashbrownSet<T>;

    fn bitand(self, rhs: &ST) -> Self::Output {
        self.intersection(rhs).copied().collect()
    }
}

impl<T, ST> BitXor<&ST> for &FzScalarSet<T>
where
    T: Scalar + Hash,
    ST: Set<T>,
{
    type Output = HashbrownSet<T>;

    fn bitxor(self, rhs: &ST) -> Self::Output {
        self.symmetric_difference(rhs).copied().collect()
    }
}

impl<T, ST> Sub<&ST> for &FzScalarSet<T>
where
    T: Scalar + Hash,
    ST: Set<T>,
{
    type Output = HashbrownSet<T>;

    fn sub(self, rhs: &ST) -> Self::Output {
        self.difference(rhs).copied().collect()
    }
}

impl<T> IntoIterator for FzScalarSet<T> {
    type Item = T;
    type IntoIter = IntoIter<T>;

    fn into_iter(self) -> Self::IntoIter {
        match self.set_impl {
            SetTypes::Hash(s) => s.into_iter(),
            SetTypes::Dense(s) => s.into_iter(),
            SetTypes::Sparse(s) => s.into_iter(),
        }
    }
}

impl<'a, T> IntoIterator for &'a FzScalarSet<T> {
    type Item = &'a T;
    type IntoIter = Iter<'a, T>;

    fn into_iter(self) -> Iter<'a, T> {
        self.iter()
    }
}

impl<T> SetIterator<T> for FzScalarSet<T> {
    type Iterator<'a>
        = Iter<'a, T>
    where
        T: 'a;

    fn iter(&self) -> Iter<'_, T> {
        match &self.set_impl {
            SetTypes::Hash(s) => s.iter(),
            SetTypes::Dense(s) => s.iter(),
            SetTypes::Sparse(s) => s.iter(),
        }
    }
}

impl<T> Set<T> for FzScalarSet<T>
where
    T: Scalar,
{
    fn contains(&self, value: &T) -> bool {
        self.contains(value)
    }
}

#[cfg(test)]
mod tests {
    use ahash::RandomState;
    use hashbrown::HashSet as HashbrownSet;

    use super::*;

    #[test]
    fn little() {
        run_tests((0..2).collect());
        run_tests((0..2).chain(-10..-9).collect());
    }

    #[test]
    fn medium() {
        run_tests((0..10).collect());
        run_tests((0..10).chain(-10..-9).collect());
    }

    #[test]
    fn large() {
        run_tests((0..300).collect());
        run_tests((0..300).chain(-10..-9).collect());
    }

    fn run_tests(entries: Vec<i32>) {
        misc(entries.clone());
        fmt(entries.clone());
        bits(entries);
    }

    fn misc(entries: Vec<i32>) {
        let set = FzScalarSet::new(entries.clone());
        let mut hs = HashbrownSet::<i32, RandomState>::from_iter(entries.iter().copied());

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

        let set3 = FzScalarSet::<i32>::from_iter(entries.clone());
        assert_eq!(set, set3);

        let set4 = FzScalarSet::<i32>::from(entries);
        assert_eq!(set, set4);

        let set5 = FzScalarSet::<i32>::from([0, 1, 2, 3]);
        assert_eq!(4, set5.len());
    }

    fn fmt(entries: Vec<i32>) {
        let set = FzScalarSet::new(entries.clone());

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
        let fs1 = FzScalarSet::new(entries.clone());
        let fs2 = FzScalarSet::new(entries.iter().map(|x| x + 1).collect());

        let hs1 = HashbrownSet::<i32>::from_iter(entries.clone());
        let hs2 = HashbrownSet::<i32>::from_iter(entries.into_iter().map(|x| x + 1));

        assert_eq!(&fs1 | &fs2, &hs1 | &hs2);
        assert_eq!(&fs1 & &fs2, &hs1 & &hs2);
        assert_eq!(&fs1 ^ &fs2, &hs1 ^ &hs2);
        assert_eq!(&fs1 - &fs2, &hs1 - &hs2);
    }

    #[test]
    fn default() {
        let fs = FzScalarSet::<i32>::default();
        assert_eq!(0, fs.len());
    }
}
