use alloc::vec::Vec;
use core::borrow::Borrow;
use core::fmt::{Debug, Formatter, Result};
use core::hash::{BuildHasher, Hash};
use core::ops::{BitAnd, BitOr, BitXor, Sub};

use ahash::RandomState;
use frozen_collections_core::hashers::BridgeHasher;
use frozen_collections_core::sets::{HashSet, IntoIter, Iter, ScanSet};
use frozen_collections_core::traits::{LargeCollection, Len, Set, SetIterator};
use hashbrown::HashSet as HashbrownSet;

/// The different implementations available for use, depending on the entries.
#[derive(Clone)]
enum SetTypes<T, BH> {
    Hash(HashSet<T, LargeCollection, BridgeHasher<BH>>),
    Scanning(ScanSet<T>),
}

/// A hash set optimized for fast read access.
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
/// A [`FzHashSet`] requires that the elements
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
/// possible through [`core::cell::Cell`], [`core::cell::RefCell`], global state, I/O, or unsafe code.
///
/// The behavior resulting from either logic error is not specified, but will
/// be encapsulated to the `FzHashSet` that observed the logic error and not
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
/// [`fz_hash_set!`](crate::fz_hash_set) macro rather than this type. This will result in considerably
/// better performance.
///
/// # Examples
///
/// The easiest way to use `FzHashSet` with a custom type is to derive
/// [`Eq`] and [`Hash`]. We must also derive [`PartialEq`],
/// which is required if [`Eq`] is derived.
///
/// ```
/// # use frozen_collections::FzHashSet;
/// #
/// #[derive(Hash, Eq, PartialEq, Debug)]
/// struct Viking {
///     name: String,
///     power: usize,
/// }
///
/// let vikings = FzHashSet::new(vec![
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
pub struct FzHashSet<T, BH = RandomState> {
    set_impl: SetTypes<T, BH>,
}

impl<T> FzHashSet<T, RandomState>
where
    T: Hash + Eq,
{
    /// Creates a new frozen set which will use the [`RandomState`] hasher to hash values.
    ///
    /// # Examples
    ///
    /// ```
    /// # use frozen_collections::FzHashSet;
    /// # use frozen_collections::Len;
    /// #
    /// let set = FzHashSet::new(vec![1, 2, 3]);
    ///
    /// assert_eq!(set.len(), 3);
    /// assert!(set.contains(&1));
    /// ```
    #[must_use]
    pub fn new(entries: Vec<T>) -> Self {
        Self::with_hasher(entries, RandomState::new())
    }
}

impl<T, BH> FzHashSet<T, BH>
where
    T: Hash + Eq,
    BH: BuildHasher,
{
    /// Creates a new frozen set which will use the given hasher to hash values.
    ///
    /// # Examples
    ///
    /// ```
    /// # use frozen_collections::FzHashSet;
    /// # use frozen_collections::Len;
    /// # use ahash::RandomState;
    /// #
    /// let set = FzHashSet::with_hasher(vec![1, 2, 3], RandomState::new());
    ///
    /// assert_eq!(set.len(), 3);
    /// assert!(set.contains(&1));
    /// ```
    #[must_use]
    #[allow(clippy::missing_panics_doc)]
    pub fn with_hasher(entries: Vec<T>, bh: BH) -> Self {
        Self {
            set_impl: if entries.len() <= 3 {
                SetTypes::Scanning(ScanSet::new(entries))
            } else {
                let h = BridgeHasher::new(bh);
                SetTypes::Hash(HashSet::new(entries, h).unwrap())
            },
        }
    }
}

impl<T, BH> FzHashSet<T, BH>
where
    BH: BuildHasher,
{
    /// Returns `true` if the set contains a value.
    ///
    /// # Examples
    ///
    /// ```
    /// # use frozen_collections::FzHashSet;
    /// #
    /// let set = FzHashSet::new(vec![1, 2, 3]);
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
            SetTypes::Hash(s) => s.contains(value),
            SetTypes::Scanning(s) => s.contains(value),
        }
    }

    /// Returns a reference to the value in the set, if any, that is equal to the given value.
    ///
    /// # Examples
    ///
    /// ```
    /// # use frozen_collections::FzHashSet;
    /// #
    /// let set = FzHashSet::new(vec![1, 2, 3]);
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
            SetTypes::Hash(s) => s.get(value),
            SetTypes::Scanning(s) => s.get(value),
        }
    }
}

impl<T, BH> From<Vec<T>> for FzHashSet<T, BH>
where
    T: Hash + Eq,
    BH: BuildHasher + Default,
{
    fn from(entries: Vec<T>) -> Self {
        Self::with_hasher(entries, BH::default())
    }
}

impl<T, const N: usize, BH> From<[T; N]> for FzHashSet<T, BH>
where
    T: Hash + Eq,
    BH: BuildHasher + Default,
{
    fn from(entries: [T; N]) -> Self {
        Self::with_hasher(Vec::from_iter(entries), BH::default())
    }
}

impl<T, BH> FromIterator<T> for FzHashSet<T, BH>
where
    T: Hash + Eq,
    BH: BuildHasher + Default,
{
    fn from_iter<U: IntoIterator<Item = T>>(iter: U) -> Self {
        Self::with_hasher(Vec::from_iter(iter), BH::default())
    }
}

impl<T, BH> Len for FzHashSet<T, BH> {
    fn len(&self) -> usize {
        match &self.set_impl {
            SetTypes::Hash(s) => Len::len(s),
            SetTypes::Scanning(s) => Len::len(s),
        }
    }
}

impl<T, BH> Default for FzHashSet<T, BH>
where
    BH: BuildHasher + Default,
{
    fn default() -> Self {
        Self {
            set_impl: SetTypes::Hash(HashSet::default()),
        }
    }
}

impl<T, BH> Debug for FzHashSet<T, BH>
where
    T: Debug,
{
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        match &self.set_impl {
            SetTypes::Hash(s) => s.fmt(f),
            SetTypes::Scanning(s) => s.fmt(f),
        }
    }
}

impl<T, ST, BH> PartialEq<ST> for FzHashSet<T, BH>
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

impl<T, BH> Eq for FzHashSet<T, BH>
where
    T: Hash + Eq,
    BH: BuildHasher,
{
}

impl<T, ST, BH> BitOr<&ST> for &FzHashSet<T, BH>
where
    T: Hash + Eq + Clone,
    ST: Set<T>,
    BH: BuildHasher + Default,
{
    type Output = HashbrownSet<T>;

    fn bitor(self, rhs: &ST) -> Self::Output {
        self.union(rhs).cloned().collect()
    }
}

impl<T, ST, BH> BitAnd<&ST> for &FzHashSet<T, BH>
where
    T: Hash + Eq + Clone,
    ST: Set<T>,
    BH: BuildHasher + Default,
{
    type Output = HashbrownSet<T>;

    fn bitand(self, rhs: &ST) -> Self::Output {
        self.intersection(rhs).cloned().collect()
    }
}

impl<T, ST, BH> BitXor<&ST> for &FzHashSet<T, BH>
where
    T: Hash + Eq + Clone,
    ST: Set<T>,
    BH: BuildHasher + Default,
{
    type Output = HashbrownSet<T>;

    fn bitxor(self, rhs: &ST) -> Self::Output {
        self.symmetric_difference(rhs).cloned().collect()
    }
}

impl<T, ST, BH> Sub<&ST> for &FzHashSet<T, BH>
where
    T: Hash + Eq + Clone,
    ST: Set<T>,
    BH: BuildHasher + Default,
{
    type Output = HashbrownSet<T>;

    fn sub(self, rhs: &ST) -> Self::Output {
        self.difference(rhs).cloned().collect()
    }
}

impl<T, BH> IntoIterator for FzHashSet<T, BH> {
    type Item = T;
    type IntoIter = IntoIter<T>;

    fn into_iter(self) -> Self::IntoIter {
        match self.set_impl {
            SetTypes::Hash(s) => s.into_iter(),
            SetTypes::Scanning(s) => s.into_iter(),
        }
    }
}

impl<'a, T, BH> IntoIterator for &'a FzHashSet<T, BH> {
    type Item = &'a T;
    type IntoIter = Iter<'a, T>;

    fn into_iter(self) -> Iter<'a, T> {
        self.iter()
    }
}

impl<T, BH> SetIterator<T> for FzHashSet<T, BH> {
    type Iterator<'a>
        = Iter<'a, T>
    where
        T: 'a,
        BH: 'a;

    fn iter(&self) -> Iter<'_, T> {
        match &self.set_impl {
            SetTypes::Hash(s) => s.iter(),
            SetTypes::Scanning(s) => s.iter(),
        }
    }
}

impl<T, BH> Set<T> for FzHashSet<T, BH>
where
    T: Hash + Eq,
    BH: BuildHasher,
{
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

    fn run_tests(entries: Vec<i32>) {
        misc(entries.clone());
        iter(entries.clone());
        fmt(entries.clone());
        bits(entries);
    }

    fn misc(entries: Vec<i32>) {
        let set = FzHashSet::new(entries.clone());
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

        let set3 = FzHashSet::<i32, RandomState>::from_iter(entries.clone());
        assert_eq!(set, set3);

        let set4 = FzHashSet::<i32, RandomState>::from(entries);
        assert_eq!(set, set4);

        let set5 = FzHashSet::<i32, RandomState>::from([0, 1, 2, 3]);
        assert_eq!(4, set5.len());
    }

    fn iter(entries: Vec<i32>) {
        let set = FzHashSet::new(entries);

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
        let set = FzHashSet::new(entries.clone());

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
        let fs1 = FzHashSet::new(entries.clone());
        let fs2 = FzHashSet::new(entries.iter().map(|x| x + 1).collect());

        let hs1 = HashbrownSet::<i32>::from_iter(entries.clone());
        let hs2 = HashbrownSet::<i32>::from_iter(entries.into_iter().map(|x| x + 1));

        assert_eq!(&fs1 | &fs2, &hs1 | &hs2);
        assert_eq!(&fs1 & &fs2, &hs1 & &hs2);
        assert_eq!(&fs1 ^ &fs2, &hs1 ^ &hs2);
        assert_eq!(&fs1 - &fs2, &hs1 - &hs2);
    }

    #[test]
    fn default() {
        let fs = FzHashSet::<i32, RandomState>::default();
        assert_eq!(0, fs.len());
    }
}
