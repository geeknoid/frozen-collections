use std::borrow::Borrow;
use std::collections::HashSet;
use std::fmt::{Debug, Formatter, Result};
use std::hash::Hash;
use std::ops::{BitAnd, BitOr, BitXor, Sub};

use bitvec::macros::internal::funty::Fundamental;
use num_traits::{AsPrimitive, PrimInt};

use frozen_collections_core::analyzers::{analyze_int_keys, IntKeyAnalysisResult};

use crate::specialized_sets::{IntegerRangeSet, IntegerSet, IntoIter, Iter};
use crate::Len;
use crate::Set;

/// The different implementations available for use, depending on the payload.
#[derive(Clone)]
enum SetTypes<T> {
    Small(IntegerSet<T, u8>),
    Large(IntegerSet<T, usize>),
    Range(IntegerRangeSet<T>),
}

/// A set optimized for fast read access of integer values.
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
/// # Macros are Faster
///
/// If all your values are known at compile time, you are much better off using the
/// [`frozen_set!`](crate::frozen_set!) macro rather than this type. This will result in considerably
/// better performance.
///
/// # Examples
///
/// ```
/// # use frozen_collections::FrozenIntSet;
/// # use frozen_collections::Len;
/// #
/// let set = FrozenIntSet::try_from([1, 2, 3]).unwrap();
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
pub struct FrozenIntSet<T> {
    set_impl: SetTypes<T>,
}

impl<T> FrozenIntSet<T>
where
    T: PrimInt + AsPrimitive<u64> + Hash,
{
    /// Creates a new frozen set.
    ///
    /// # Errors
    ///
    /// This fails if there are duplicate items within the vector.
    ///
    /// # Examples
    ///
    /// ```
    /// # use frozen_collections::FrozenIntSet;
    /// # use frozen_collections::Len;
    /// #
    /// let set = FrozenIntSet::new(vec![1, 2, 3]).unwrap();
    ///
    /// assert_eq!(set.len(), 3);
    /// assert!(set.contains(&1));
    /// ```
    pub fn new(payload: Vec<T>) -> std::result::Result<Self, &'static str> {
        let key_analysis = analyze_int_keys(payload.iter().copied());

        Ok(Self {
            set_impl: match key_analysis {
                IntKeyAnalysisResult::Range => SetTypes::Range(IntegerRangeSet::try_from(payload)?),

                IntKeyAnalysisResult::Normal => {
                    if payload.len() <= u8::MAX.as_usize() {
                        SetTypes::Small(IntegerSet::try_from(payload)?)
                    } else {
                        SetTypes::Large(IntegerSet::try_from(payload)?)
                    }
                }
            },
        })
    }
}

impl<T> FrozenIntSet<T> {
    /// Returns `true` if the set contains a value.
    ///
    /// # Examples
    ///
    /// ```
    /// # use frozen_collections::FrozenIntSet;
    /// #
    /// let set = FrozenIntSet::try_from([1, 2, 3]).unwrap();
    ///
    /// assert!(set.contains(&1));
    /// assert!(!set.contains(&4));
    /// ```
    #[inline(always)]
    #[must_use]
    pub fn contains<Q>(&self, value: &Q) -> bool
    where
        T: Borrow<Q>,
        Q: PrimInt + AsPrimitive<u64>,
    {
        match &self.set_impl {
            SetTypes::Small(s) => s.contains(value),
            SetTypes::Large(s) => s.contains(value),
            SetTypes::Range(s) => s.contains(value),
        }
    }

    /// Returns a reference to the value in the set, if any, that is equal to the given value.
    ///
    /// # Examples
    ///
    /// ```
    /// # use frozen_collections::FrozenIntSet;
    /// #
    /// let set = FrozenIntSet::try_from([1, 2, 3]).unwrap();
    ///
    /// assert_eq!(set.get(&2), Some(&2));
    /// assert_eq!(set.get(&4), None);
    /// ```
    #[inline]
    #[must_use]
    pub fn get<Q>(&self, value: &Q) -> Option<&T>
    where
        T: Borrow<Q>,
        Q: PrimInt + AsPrimitive<u64>,
    {
        match &self.set_impl {
            SetTypes::Small(s) => s.get(value),
            SetTypes::Large(s) => s.get(value),
            SetTypes::Range(s) => s.get(value),
        }
    }
}

impl<T> TryFrom<Vec<T>> for FrozenIntSet<T>
where
    T: PrimInt + AsPrimitive<u64> + Hash,
{
    type Error = &'static str;

    fn try_from(payload: Vec<T>) -> std::result::Result<Self, Self::Error> {
        Self::new(payload)
    }
}

impl<T, const N: usize> TryFrom<[T; N]> for FrozenIntSet<T>
where
    T: PrimInt + AsPrimitive<u64> + Hash,
{
    type Error = &'static str;

    fn try_from(payload: [T; N]) -> std::result::Result<Self, Self::Error> {
        Self::new(Vec::from_iter(payload))
    }
}

impl<T> FromIterator<T> for FrozenIntSet<T>
where
    T: PrimInt + AsPrimitive<u64> + Hash,
{
    fn from_iter<U: IntoIterator<Item = T>>(iter: U) -> Self {
        Self::new(Vec::from_iter(iter)).unwrap()
    }
}

impl<T> Default for FrozenIntSet<T>
where
    T: PrimInt + AsPrimitive<u64> + Hash,
{
    fn default() -> Self {
        Self {
            set_impl: SetTypes::Range(IntegerRangeSet::default()),
        }
    }
}

impl<T> Debug for FrozenIntSet<T>
where
    T: PrimInt + AsPrimitive<u64> + Debug,
{
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        match &self.set_impl {
            SetTypes::Small(s) => s.fmt(f),
            SetTypes::Large(s) => s.fmt(f),
            SetTypes::Range(s) => s.fmt(f),
        }
    }
}

impl<T, ST> PartialEq<ST> for FrozenIntSet<T>
where
    T: PrimInt + AsPrimitive<u64> + Hash,
    ST: Set<T>,
{
    fn eq(&self, other: &ST) -> bool {
        if self.len() != other.len() {
            return false;
        }

        self.iter().all(|value| other.contains(value))
    }
}

impl<T> Eq for FrozenIntSet<T> where T: PrimInt + AsPrimitive<u64> + Hash {}

impl<T, ST> BitOr<&ST> for &FrozenIntSet<T>
where
    T: PrimInt + AsPrimitive<u64> + Hash + Clone,
    ST: Set<T>,
{
    type Output = HashSet<T>;

    fn bitor(self, rhs: &ST) -> Self::Output {
        self.union(rhs).copied().collect()
    }
}

impl<T, ST> BitAnd<&ST> for &FrozenIntSet<T>
where
    T: PrimInt + AsPrimitive<u64> + Hash + Clone,
    ST: Set<T>,
{
    type Output = HashSet<T>;

    fn bitand(self, rhs: &ST) -> Self::Output {
        self.intersection(rhs).copied().collect()
    }
}

impl<T, ST> BitXor<&ST> for &FrozenIntSet<T>
where
    T: PrimInt + AsPrimitive<u64> + Hash + Clone,
    ST: Set<T>,
{
    type Output = HashSet<T>;

    fn bitxor(self, rhs: &ST) -> Self::Output {
        self.symmetric_difference(rhs).copied().collect()
    }
}

impl<T, ST> Sub<&ST> for &FrozenIntSet<T>
where
    T: PrimInt + AsPrimitive<u64> + Hash,
    ST: Set<T>,
{
    type Output = HashSet<T>;

    fn sub(self, rhs: &ST) -> Self::Output {
        self.difference(rhs).copied().collect()
    }
}

impl<T> IntoIterator for FrozenIntSet<T>
where
    T: PrimInt + AsPrimitive<u64> + Hash,
{
    type Item = T;
    type IntoIter = IntoIter<T>;

    fn into_iter(self) -> Self::IntoIter {
        match self.set_impl {
            SetTypes::Small(s) => s.into_iter(),
            SetTypes::Large(s) => s.into_iter(),
            SetTypes::Range(s) => s.into_iter(),
        }
    }
}

impl<'a, T> IntoIterator for &'a FrozenIntSet<T>
where
    T: PrimInt + AsPrimitive<u64> + Hash,
{
    type Item = &'a T;
    type IntoIter = Iter<'a, T>;

    fn into_iter(self) -> Iter<'a, T> {
        self.iter()
    }
}

impl<T> Len for FrozenIntSet<T> {
    fn len(&self) -> usize {
        match &self.set_impl {
            SetTypes::Small(s) => Len::len(s),
            SetTypes::Large(s) => Len::len(s),
            SetTypes::Range(s) => Len::len(s),
        }
    }
}

impl<T> Set<T> for FrozenIntSet<T>
where
    T: PrimInt + AsPrimitive<u64> + Hash,
{
    type Iterator<'a> = Iter<'a, T>
    where
        T: 'a;

    fn iter(&self) -> Iter<'_, T> {
        match &self.set_impl {
            SetTypes::Small(s) => s.iter(),
            SetTypes::Large(s) => s.iter(),
            SetTypes::Range(s) => s.iter(),
        }
    }

    fn contains(&self, value: &T) -> bool {
        self.contains(value)
    }
}

#[cfg(test)]
mod tests {
    use ahash::RandomState;

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

    fn run_tests(payload: Vec<i32>) {
        misc(payload.clone());
        iter(payload.clone());
        fmt(payload.clone());
        bits(payload);
    }

    fn misc(payload: Vec<i32>) {
        let set = FrozenIntSet::new(payload.clone()).unwrap();
        let mut hs = HashSet::<i32, RandomState>::from_iter(payload.iter().copied());

        assert_eq!(payload.len(), set.len());

        for value in &payload {
            assert!(set.contains(value));
            assert_eq!(value, set.get(value).unwrap());
        }

        assert!(!set.contains(&-1));
        assert_eq!(payload.len(), set.iter().count());
        assert_eq!(set, hs);

        hs.insert(-1);
        assert_ne!(set, hs);

        let set3 = FrozenIntSet::<i32>::from_iter(payload.clone());
        assert_eq!(set, set3);

        let set4 = FrozenIntSet::<i32>::try_from(payload).unwrap();
        assert_eq!(set, set4);

        let set5 = FrozenIntSet::<i32>::try_from([0, 1, 2, 3]).unwrap();
        assert_eq!(4, set5.len());
    }

    fn iter(payload: Vec<i32>) {
        let set = FrozenIntSet::new(payload).unwrap();

        let hs = HashSet::<i32, RandomState>::from_iter(
            HashSet::<&i32, RandomState>::from_iter(set.iter())
                .iter()
                .map(|x| **x),
        );

        assert_eq!(set, hs);

        let other_hs = HashSet::from_iter(set.clone());
        assert_eq!(hs, other_hs);

        let set_ref = &set.clone();
        let other_hs = HashSet::from_iter(set_ref.into_iter().copied());
        assert_eq!(hs, other_hs);

        let other_hs = HashSet::from_iter(set);
        assert_eq!(hs, other_hs);
    }

    fn fmt(payload: Vec<i32>) {
        let set = FrozenIntSet::new(payload.clone()).unwrap();

        let formatted_set = format!("{set:?}");
        for value in payload {
            let value_str = format!("{value:?}");
            assert!(
                formatted_set.contains(&value_str),
                "Formatted string does not contain value: {value:?}"
            );
        }
    }

    fn bits(payload: Vec<i32>) {
        let fs1 = FrozenIntSet::new(payload.clone()).unwrap();
        let fs2 = FrozenIntSet::new(payload.iter().map(|x| x + 1).collect()).unwrap();

        let hs1 = HashSet::from_iter(payload.clone());
        let hs2 = HashSet::from_iter(payload.into_iter().map(|x| x + 1));

        assert_eq!(&fs1 | &fs2, &hs1 | &hs2);
        assert_eq!(&fs1 & &fs2, &hs1 & &hs2);
        assert_eq!(&fs1 ^ &fs2, &hs1 ^ &hs2);
        assert_eq!(&fs1 - &fs2, &hs1 - &hs2);
    }

    #[test]
    fn default() {
        let fs = FrozenIntSet::<i32>::default();
        assert_eq!(0, fs.len());
    }
}
