use std::borrow::Borrow;
use std::collections::HashSet;
use std::fmt::{Debug, Formatter, Result};
use std::hash::{BuildHasher, Hash};
use std::ops::{BitAnd, BitOr, BitXor, Sub};

use ahash::RandomState;
use bitvec::macros::internal::funty::Fundamental;

use crate::specialized_sets::{CommonSet, IntoIter, Iter};
use crate::Len;
use crate::Set;

/// The different implementations available for use, depending on the payload.
#[derive(Clone)]
enum SetTypes<T, BH> {
    CommonSmall(CommonSet<T, u8, BH>),
    CommonLarge(CommonSet<T, usize, BH>),
}

/// A set optimized for fast read access.
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
/// A `FrozenSet` requires that the elements
/// implement the [`Eq`] and [`Hash`] traits. This can frequently be achieved by
/// using `#[derive(PartialEq, Eq, Hash)]`. If you implement these yourself,
/// it is important that the following property holds:
///
/// ```text
/// k1 == k2 -> hash(k1) == hash(k2)
/// ```
///
/// In other words, if two values are equal, their hashes must be equal.
/// Violating this property is a logic error.
///
/// It is also a logic error for a value to be modified in such a way that the value's
/// hash, as determined by the [`Hash`] trait, or its equality, as determined by
/// the [`Eq`] trait, changes while it is in the set. This is normally only
/// possible through [`std::cell::Cell`], [`std::cell::RefCell`], global state, I/O, or unsafe code.
///
/// The behavior resulting from either logic error is not specified, but will
/// be encapsulated to the `FrozenSet` that observed the logic error and not
/// result in undefined behavior. This could include panics, incorrect results,
/// aborts, memory leaks, and non-termination.
///
/// # Macros are Faster
///
/// If all your values are known at compile time, you are much better off using the
/// [`frozen_set!`](crate::frozen_set!) macro rather than this type. This will result in considerably
/// better performance.
///
/// # Integer and String Values
///
/// If you can't use the [`frozen_set!`](crate::frozen_set!), but you know at compile time that your values are integers or strings, you should use the
/// [`crate::FrozenIntSet`] and [`crate::FrozenStringSet`] types respectively for better performance.
///
/// # Examples
///
/// The easiest way to use `FrozenSet` with a custom type is to derive
/// [`Eq`] and [`Hash`]. We must also derive [`PartialEq`],
/// which is required if [`Eq`] is derived.
///
/// ```
/// # use frozen_collections::FrozenSet;
/// #
/// #[derive(Hash, Eq, PartialEq, Debug)]
/// struct Viking {
///     name: String,
///     power: usize,
/// }
///
/// let vikings = FrozenSet::new(vec![
///     Viking {name: "Einar".to_string(), power: 9 },
///     Viking { name: "Olaf".to_string(), power: 4 },
///     Viking { name: "Harald".to_string(), power: 8 }]).unwrap();
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
pub struct FrozenSet<T, BH = RandomState> {
    set_impl: SetTypes<T, BH>,
}

impl<T> FrozenSet<T, RandomState>
where
    T: Hash + Eq,
{
    /// Creates a new frozen set which will use the [`RandomState`] hasher to hash values.
    ///
    /// # Errors
    ///
    /// This fails if there are duplicate items within the vector.
    ///
    /// # Examples
    ///
    /// ```
    /// # use frozen_collections::FrozenSet;
    /// # use std::hash::RandomState;
    /// # use frozen_collections::Len;
    /// #
    /// let set = FrozenSet::new(vec![1, 2, 3]).unwrap();
    ///
    /// assert_eq!(set.len(), 3);
    /// assert!(set.contains(&1));
    /// ```
    pub fn new(payload: Vec<T>) -> std::result::Result<Self, &'static str> {
        Self::with_hasher(payload, RandomState::new())
    }
}

impl<T, BH> FrozenSet<T, BH>
where
    T: Hash + Eq,
    BH: BuildHasher,
{
    /// Creates a new frozen set which will use the given hasher to hash values.
    ///
    /// # Errors
    ///
    /// This fails if there are duplicate items within the vector.
    ///
    /// # Examples
    ///
    /// ```
    /// # use frozen_collections::FrozenSet;
    /// # use std::hash::RandomState;
    /// # use frozen_collections::Len;
    /// #
    /// let set = FrozenSet::with_hasher(vec![1, 2, 3], RandomState::new()).unwrap();
    ///
    /// assert_eq!(set.len(), 3);
    /// assert!(set.contains(&1));
    /// ```
    pub fn with_hasher(payload: Vec<T>, bh: BH) -> std::result::Result<Self, &'static str> {
        Ok(Self {
            set_impl: if payload.len() <= u8::MAX.as_usize() {
                SetTypes::CommonSmall(CommonSet::with_hasher(payload, bh)?)
            } else {
                SetTypes::CommonLarge(CommonSet::with_hasher(payload, bh)?)
            },
        })
    }
}

impl<T, BH> FrozenSet<T, BH>
where
    BH: BuildHasher,
{
    /// Returns `true` if the set contains a value.
    ///
    /// # Examples
    ///
    /// ```
    /// # use frozen_collections::FrozenSet;
    /// #
    /// let set = FrozenSet::new(vec![1, 2, 3]).unwrap();
    ///
    /// assert!(set.contains(&1));
    /// assert!(!set.contains(&4));
    /// ```
    #[inline(always)]
    #[must_use]
    pub fn contains<Q>(&self, value: &Q) -> bool
    where
        T: Borrow<Q>,
        Q: Hash + Eq + ?Sized,
    {
        match &self.set_impl {
            SetTypes::CommonSmall(s) => s.contains(value),
            SetTypes::CommonLarge(s) => s.contains(value),
        }
    }

    /// Returns a reference to the value in the set, if any, that is equal to the given value.
    ///
    /// # Examples
    ///
    /// ```
    /// # use frozen_collections::FrozenSet;
    /// #
    /// let set = FrozenSet::new(vec![1, 2, 3]).unwrap();
    ///
    /// assert_eq!(set.get(&2), Some(&2));
    /// assert_eq!(set.get(&4), None);
    /// ```
    #[inline]
    #[must_use]
    pub fn get<Q>(&self, value: &Q) -> Option<&T>
    where
        T: Borrow<Q>,
        Q: Hash + Eq + ?Sized,
    {
        match &self.set_impl {
            SetTypes::CommonSmall(s) => s.get(value),
            SetTypes::CommonLarge(s) => s.get(value),
        }
    }
}

impl<T, BH> TryFrom<Vec<T>> for FrozenSet<T, BH>
where
    T: Hash + Eq,
    BH: BuildHasher + Default,
{
    type Error = &'static str;

    fn try_from(payload: Vec<T>) -> std::result::Result<Self, Self::Error> {
        Self::with_hasher(payload, BH::default())
    }
}

impl<T, const N: usize, BH> TryFrom<[T; N]> for FrozenSet<T, BH>
where
    T: Hash + Eq,
    BH: BuildHasher + Default,
{
    type Error = &'static str;

    fn try_from(payload: [T; N]) -> std::result::Result<Self, Self::Error> {
        Self::try_from(Vec::from_iter(payload))
    }
}

impl<T, BH> FromIterator<T> for FrozenSet<T, BH>
where
    T: Hash + Eq,
    BH: BuildHasher + Default,
{
    fn from_iter<U: IntoIterator<Item = T>>(iter: U) -> Self {
        Self::try_from(Vec::from_iter(iter)).unwrap()
    }
}

impl<T, BH> Default for FrozenSet<T, BH>
where
    T: Hash + Eq + Default,
    BH: BuildHasher + Default,
{
    fn default() -> Self {
        Self {
            set_impl: SetTypes::CommonSmall(CommonSet::default()),
        }
    }
}

impl<T, BH> Debug for FrozenSet<T, BH>
where
    T: Hash + Eq + Debug,
    BH: BuildHasher,
{
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        match &self.set_impl {
            SetTypes::CommonSmall(s) => s.fmt(f),
            SetTypes::CommonLarge(s) => s.fmt(f),
        }
    }
}

impl<T, ST, BH> PartialEq<ST> for FrozenSet<T, BH>
where
    T: Hash + Eq,
    ST: Set<T>,
    BH: BuildHasher,
{
    fn eq(&self, other: &ST) -> bool {
        if self.len() != other.len() {
            return false;
        }

        self.iter().all(|value| other.contains(value))
    }
}

impl<T, BH> Eq for FrozenSet<T, BH>
where
    T: Hash + Eq,
    BH: BuildHasher,
{
}

impl<T, ST, BH> BitOr<&ST> for &FrozenSet<T, BH>
where
    T: Hash + Eq + Clone,
    ST: Set<T>,
    BH: BuildHasher + Default,
{
    type Output = HashSet<T, BH>;

    fn bitor(self, rhs: &ST) -> Self::Output {
        self.union(rhs).cloned().collect()
    }
}

impl<T, ST, BH> BitAnd<&ST> for &FrozenSet<T, BH>
where
    T: Hash + Eq + Clone,
    ST: Set<T>,
    BH: BuildHasher + Default,
{
    type Output = HashSet<T, BH>;

    fn bitand(self, rhs: &ST) -> Self::Output {
        self.intersection(rhs).cloned().collect()
    }
}

impl<T, ST, BH> BitXor<&ST> for &FrozenSet<T, BH>
where
    T: Hash + Eq + Clone,
    ST: Set<T>,
    BH: BuildHasher + Default,
{
    type Output = HashSet<T, BH>;

    fn bitxor(self, rhs: &ST) -> Self::Output {
        self.symmetric_difference(rhs).cloned().collect()
    }
}

impl<T, ST, BH> Sub<&ST> for &FrozenSet<T, BH>
where
    T: Hash + Eq + Clone,
    ST: Set<T>,
    BH: BuildHasher + Default,
{
    type Output = HashSet<T, BH>;

    fn sub(self, rhs: &ST) -> Self::Output {
        self.difference(rhs).cloned().collect()
    }
}

impl<T, BH> IntoIterator for FrozenSet<T, BH>
where
    T: Hash + Eq,
    BH: BuildHasher,
{
    type Item = T;
    type IntoIter = IntoIter<T>;

    fn into_iter(self) -> Self::IntoIter {
        match self.set_impl {
            SetTypes::CommonSmall(s) => s.into_iter(),
            SetTypes::CommonLarge(s) => s.into_iter(),
        }
    }
}

impl<'a, T, BH> IntoIterator for &'a FrozenSet<T, BH>
where
    T: Hash + Eq,
    BH: BuildHasher,
{
    type Item = &'a T;
    type IntoIter = Iter<'a, T>;

    fn into_iter(self) -> Iter<'a, T> {
        self.iter()
    }
}

impl<T, BH> Len for FrozenSet<T, BH> {
    fn len(&self) -> usize {
        match &self.set_impl {
            SetTypes::CommonSmall(s) => Len::len(s),
            SetTypes::CommonLarge(s) => Len::len(s),
        }
    }
}

impl<T, BH> Set<T> for FrozenSet<T, BH>
where
    T: Hash + Eq,
    BH: BuildHasher,
{
    type Iterator<'a> = Iter<'a, T>
    where
        T: 'a,
        BH: 'a;

    fn iter(&self) -> Iter<'_, T> {
        match &self.set_impl {
            SetTypes::CommonSmall(s) => s.iter(),
            SetTypes::CommonLarge(s) => s.iter(),
        }
    }

    fn contains(&self, value: &T) -> bool {
        self.contains(value)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

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

    fn run_tests(payload: Vec<i32>) {
        misc(payload.clone());
        iter(payload.clone());
        fmt(payload.clone());
        bits(payload);
    }

    fn misc(payload: Vec<i32>) {
        let set = FrozenSet::new(payload.clone()).unwrap();
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

        let set3 = FrozenSet::<i32, RandomState>::from_iter(payload.clone());
        assert_eq!(set, set3);

        let set4 = FrozenSet::<i32, RandomState>::try_from(payload).unwrap();
        assert_eq!(set, set4);

        let set5 = FrozenSet::<i32, RandomState>::try_from([0, 1, 2, 3]).unwrap();
        assert_eq!(4, set5.len());
    }

    fn iter(payload: Vec<i32>) {
        let set = FrozenSet::new(payload).unwrap();

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
        let set = FrozenSet::new(payload.clone()).unwrap();

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
        let fs1 = FrozenSet::new(payload.clone()).unwrap();
        let fs2 = FrozenSet::new(payload.iter().map(|x| x + 1).collect()).unwrap();

        let hs1 = HashSet::from_iter(payload.clone());
        let hs2 = HashSet::from_iter(payload.into_iter().map(|x| x + 1));

        assert_eq!(&fs1 | &fs2, &hs1 | &hs2);
        assert_eq!(&fs1 & &fs2, &hs1 & &hs2);
        assert_eq!(&fs1 ^ &fs2, &hs1 ^ &hs2);
        assert_eq!(&fs1 - &fs2, &hs1 - &hs2);
    }

    #[test]
    fn default() {
        let fs = FrozenSet::<i32, RandomState>::default();
        assert_eq!(0, fs.len());
    }
}
