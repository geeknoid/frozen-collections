use crate::maps::OrderedScanMap;
use crate::sets::decl_macros::{
    bitand_fn, bitor_fn, bitxor_fn, contains_fn, debug_fn, get_fn, into_iter_fn, into_iter_ref_fn,
    partial_eq_fn, set_boilerplate, set_iterator_boilerplate, sub_fn,
};
use crate::sets::{IntoIter, Iter};
use crate::traits::{Len, MapIterator, Set, SetIterator};
use alloc::vec::Vec;
use core::borrow::Borrow;
use core::fmt::Debug;
use core::hash::Hash;
use core::ops::{BitAnd, BitOr, BitXor, Sub};

/// A general purpose set implemented with linear scanning.
///
/// # Compatibility Note
///
/// This type is an implementation detail of the `frozen-collections` crate.
/// This API is therefore not stable and may change at any time.
#[derive(Clone)]
pub struct OrderedScanSet<T> {
    map: OrderedScanMap<T, ()>,
}

impl<T> OrderedScanSet<T>
where
    T: Ord,
{
    #[must_use]
    pub fn new(entries: Vec<T>) -> Self {
        Self {
            map: OrderedScanMap::new(entries.into_iter().map(|x| (x, ())).collect()),
        }
    }
}

impl<T> OrderedScanSet<T> {
    get_fn!(Ord);
    contains_fn!(Ord);
}

impl<T> Len for OrderedScanSet<T> {
    fn len(&self) -> usize {
        self.map.len()
    }
}

impl<T> Debug for OrderedScanSet<T>
where
    T: Debug,
{
    debug_fn!();
}

impl<T> Default for OrderedScanSet<T> {
    fn default() -> Self {
        Self {
            map: OrderedScanMap::default(),
        }
    }
}

impl<T> IntoIterator for OrderedScanSet<T> {
    into_iter_fn!();
}

impl<'a, T> IntoIterator for &'a OrderedScanSet<T> {
    into_iter_ref_fn!();
}

impl<T> SetIterator<T> for OrderedScanSet<T> {
    type Iterator<'a>
        = Iter<'a, T>
    where
        T: 'a;

    set_iterator_boilerplate!();
}

impl<T> Set<T> for OrderedScanSet<T>
where
    T: Ord,
{
    set_boilerplate!();
}

impl<T, ST> BitOr<&ST> for &OrderedScanSet<T>
where
    T: Hash + Eq + Ord + Clone,
    ST: Set<T>,
{
    bitor_fn!(RandomState);
}

impl<T, ST> BitAnd<&ST> for &OrderedScanSet<T>
where
    T: Hash + Eq + Ord + Clone,
    ST: Set<T>,
{
    bitand_fn!(RandomState);
}

impl<T, ST> BitXor<&ST> for &OrderedScanSet<T>
where
    T: Hash + Eq + Ord + Clone,
    ST: Set<T>,
{
    bitxor_fn!(RandomState);
}

impl<T, ST> Sub<&ST> for &OrderedScanSet<T>
where
    T: Hash + Eq + Ord + Clone,
    ST: Set<T>,
{
    sub_fn!(RandomState);
}

impl<T, ST> PartialEq<ST> for OrderedScanSet<T>
where
    T: Ord,
    ST: Set<T>,
{
    partial_eq_fn!();
}

impl<T> Eq for OrderedScanSet<T> where T: Ord {}

#[cfg(test)]
mod tests {
    use super::OrderedScanSet;
    use crate::traits::set_trait_tests::{test_misc_trait_impl, test_set_trait_impl};
    use hashbrown::HashSet as HashbrownSet;

    #[test]
    fn test_ordered_scan_set() {
        let set = OrderedScanSet::new(vec![1, 2, 3]);
        let reference = HashbrownSet::from([1, 2, 3]);
        let other = HashbrownSet::from([3, 4, 5]);
        test_set_trait_impl(&set, &reference, &other);
        test_misc_trait_impl(&set, &reference, &other);

        let set = OrderedScanSet::new(vec![]);
        let reference = HashbrownSet::from([]);
        let other = HashbrownSet::from([3, 4, 5]);
        test_set_trait_impl(&set, &reference, &other);
        test_misc_trait_impl(&set, &reference, &other);

        let set = OrderedScanSet::new(vec![3, 1, 2, 3, 3]);
        let reference = HashbrownSet::from([3, 1, 2, 3, 3]);
        let other = HashbrownSet::from([3, 4, 5]);
        test_set_trait_impl(&set, &reference, &other);
        test_misc_trait_impl(&set, &reference, &other);

        let set = OrderedScanSet::new(vec![1, 2, 3]);
        let reference = HashbrownSet::from([1, 2, 3]);
        let other = HashbrownSet::from([1, 2, 3, 4, 5]);
        test_set_trait_impl(&set, &reference, &other);
        test_misc_trait_impl(&set, &reference, &other);

        let set = OrderedScanSet::new(vec![1, 2, 3]);
        let reference = HashbrownSet::from([1, 2, 3]);
        let other = HashbrownSet::from([1, 2]);
        test_set_trait_impl(&set, &reference, &other);
        test_misc_trait_impl(&set, &reference, &other);

        let set = OrderedScanSet::new(vec![1, 2, 3]);
        let reference = HashbrownSet::from([1, 2, 3]);
        let other = HashbrownSet::from([]);
        test_set_trait_impl(&set, &reference, &other);
        test_misc_trait_impl(&set, &reference, &other);

        assert_ne!(&set, &HashbrownSet::<i32>::from_iter(0..1234));
    }

    test_misc_trait_impl!(OrderedScanSet<i32>, i32);
}
