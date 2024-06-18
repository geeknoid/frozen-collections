use alloc::vec::Vec;
use core::borrow::Borrow;
use core::fmt::Debug;
use core::hash::Hash;
use core::ops::{BitAnd, BitOr, BitXor, Sub};

use crate::maps::BinarySearchMap;
use crate::sets::decl_macros::{
    bitand_fn, bitor_fn, bitxor_fn, contains_fn, debug_fn, get_fn, into_iter_fn, into_iter_ref_fn,
    partial_eq_fn, set_boilerplate, set_iterator_boilerplate, sub_fn,
};
use crate::sets::{IntoIter, Iter};
use crate::traits::{Len, MapIterator, Set, SetIterator};

/// A general purpose set implemented using binary search.
///
/// # Compatibility Note
///
/// This type is an implementation detail of the `frozen-collections` crate.
/// This API is therefore not stable and may change at any time.
#[derive(Clone)]
pub struct BinarySearchSet<T> {
    map: BinarySearchMap<T, ()>,
}

impl<T> BinarySearchSet<T>
where
    T: Ord,
{
    #[must_use]
    pub fn new(entries: Vec<T>) -> Self {
        Self {
            map: BinarySearchMap::new(entries.into_iter().map(|x| (x, ())).collect()),
        }
    }

    #[must_use]
    pub fn new_internal(entries: Vec<T>) -> Self {
        Self {
            map: BinarySearchMap::new_internal(entries.into_iter().map(|x| (x, ())).collect()),
        }
    }
}

impl<T> BinarySearchSet<T> {
    get_fn!(Ord);
    contains_fn!(Ord);
}

impl<T> Len for BinarySearchSet<T> {
    fn len(&self) -> usize {
        self.map.len()
    }
}

impl<T> Debug for BinarySearchSet<T>
where
    T: Debug,
{
    debug_fn!();
}

impl<T> Default for BinarySearchSet<T> {
    fn default() -> Self {
        Self {
            map: BinarySearchMap::default(),
        }
    }
}

impl<T> IntoIterator for BinarySearchSet<T> {
    into_iter_fn!();
}

impl<'a, T> IntoIterator for &'a BinarySearchSet<T> {
    into_iter_ref_fn!();
}

impl<T> SetIterator<T> for BinarySearchSet<T> {
    type Iterator<'a> = Iter<'a, T>
    where
        T: 'a;

    set_iterator_boilerplate!();
}

impl<T> Set<T> for BinarySearchSet<T>
where
    T: Ord,
{
    set_boilerplate!();
}

impl<T, ST> BitOr<&ST> for &BinarySearchSet<T>
where
    T: Hash + Ord + Clone,
    ST: Set<T>,
{
    bitor_fn!(RandomState);
}

impl<T, ST> BitAnd<&ST> for &BinarySearchSet<T>
where
    T: Hash + Ord + Clone,
    ST: Set<T>,
{
    bitand_fn!(RandomState);
}

impl<T, ST> BitXor<&ST> for &BinarySearchSet<T>
where
    T: Hash + Ord + Clone,
    ST: Set<T>,
{
    bitxor_fn!(RandomState);
}

impl<T, ST> Sub<&ST> for &BinarySearchSet<T>
where
    T: Hash + Ord + Clone,
    ST: Set<T>,
{
    sub_fn!(RandomState);
}

impl<T, ST> PartialEq<ST> for BinarySearchSet<T>
where
    T: Ord,
    ST: Set<T>,
{
    partial_eq_fn!();
}

impl<T> Eq for BinarySearchSet<T> where T: Ord {}

#[cfg(test)]
mod tests {
    use super::BinarySearchSet;
    use crate::sets::set_tests::{test_misc_trait_impl, test_set_trait_impl};
    use hashbrown::HashSet as HashbrownSet;

    #[test]
    fn test_binary_search_set() {
        let set = BinarySearchSet::new(vec![1, 2, 3]);
        let reference = HashbrownSet::from([1, 2, 3]);
        let other = HashbrownSet::from([3, 4, 5]);
        test_set_trait_impl(&set, &reference, &other);
        test_misc_trait_impl(&set, &reference, &other);

        let set = BinarySearchSet::new(vec![]);
        let reference = HashbrownSet::from([]);
        let other = HashbrownSet::from([3, 4, 5]);
        test_set_trait_impl(&set, &reference, &other);
        test_misc_trait_impl(&set, &reference, &other);

        let set = BinarySearchSet::new(vec![3, 1, 2, 3, 3]);
        let reference = HashbrownSet::from([3, 1, 2, 3, 3]);
        let other = HashbrownSet::from([3, 4, 5]);
        test_set_trait_impl(&set, &reference, &other);
        test_misc_trait_impl(&set, &reference, &other);

        let set = BinarySearchSet::new(vec![1, 2, 3]);
        let reference = HashbrownSet::from([1, 2, 3]);
        let other = HashbrownSet::from([1, 2, 3, 4, 5]);
        test_set_trait_impl(&set, &reference, &other);
        test_misc_trait_impl(&set, &reference, &other);

        let set = BinarySearchSet::new(vec![1, 2, 3]);
        let reference = HashbrownSet::from([1, 2, 3]);
        let other = HashbrownSet::from([1, 2]);
        test_set_trait_impl(&set, &reference, &other);
        test_misc_trait_impl(&set, &reference, &other);

        let set = BinarySearchSet::new(vec![1, 2, 3]);
        let reference = HashbrownSet::from([1, 2, 3]);
        let other = HashbrownSet::from([]);
        test_set_trait_impl(&set, &reference, &other);
        test_misc_trait_impl(&set, &reference, &other);

        assert_ne!(&set, &HashbrownSet::<i32>::from_iter(0..1234));
    }

    test_misc_trait_impl!(BinarySearchSet<i32>, i32);
}
