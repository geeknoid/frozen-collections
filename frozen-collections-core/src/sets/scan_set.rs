use crate::maps::ScanMap;
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
pub struct ScanSet<T> {
    map: ScanMap<T, ()>,
}

impl<T> ScanSet<T>
where
    T: Eq,
{
    #[must_use]
    pub fn new(entries: Vec<T>) -> Self {
        Self {
            map: ScanMap::new(entries.into_iter().map(|x| (x, ())).collect()),
        }
    }
}

impl<T> ScanSet<T> {
    get_fn!(Eq);
    contains_fn!(Eq);
}

impl<T> Len for ScanSet<T> {
    fn len(&self) -> usize {
        self.map.len()
    }
}

impl<T> Debug for ScanSet<T>
where
    T: Debug,
{
    debug_fn!();
}

impl<T> Default for ScanSet<T> {
    fn default() -> Self {
        Self {
            map: ScanMap::default(),
        }
    }
}

impl<T> IntoIterator for ScanSet<T> {
    into_iter_fn!();
}

impl<'a, T> IntoIterator for &'a ScanSet<T> {
    into_iter_ref_fn!();
}

impl<T> SetIterator<T> for ScanSet<T> {
    type Iterator<'a> = Iter<'a, T>
    where
        T: 'a;

    set_iterator_boilerplate!();
}

impl<T> Set<T> for ScanSet<T>
where
    T: Eq,
{
    set_boilerplate!();
}

impl<T, ST> BitOr<&ST> for &ScanSet<T>
where
    T: Hash + Eq + Clone,
    ST: Set<T>,
{
    bitor_fn!(RandomState);
}

impl<T, ST> BitAnd<&ST> for &ScanSet<T>
where
    T: Hash + Eq + Clone,
    ST: Set<T>,
{
    bitand_fn!(RandomState);
}

impl<T, ST> BitXor<&ST> for &ScanSet<T>
where
    T: Hash + Eq + Clone,
    ST: Set<T>,
{
    bitxor_fn!(RandomState);
}

impl<T, ST> Sub<&ST> for &ScanSet<T>
where
    T: Hash + Eq + Clone,
    ST: Set<T>,
{
    sub_fn!(RandomState);
}

impl<T, ST> PartialEq<ST> for ScanSet<T>
where
    T: Eq,
    ST: Set<T>,
{
    partial_eq_fn!();
}

impl<T> Eq for ScanSet<T> where T: Eq {}

#[cfg(test)]
mod tests {
    use super::ScanSet;
    use crate::sets::set_tests::{test_misc_trait_impl, test_set_trait_impl};
    use hashbrown::HashSet as HashbrownSet;

    #[test]
    fn test_ordered_scan_set() {
        let set = ScanSet::new(vec![1, 2, 3]);
        let reference = HashbrownSet::from([1, 2, 3]);
        let other = HashbrownSet::from([3, 4, 5]);
        test_set_trait_impl(&set, &reference, &other);
        test_misc_trait_impl(&set, &reference, &other);

        let set = ScanSet::new(vec![]);
        let reference = HashbrownSet::from([]);
        let other = HashbrownSet::from([3, 4, 5]);
        test_set_trait_impl(&set, &reference, &other);
        test_misc_trait_impl(&set, &reference, &other);

        let set = ScanSet::new(vec![3, 1, 2, 3, 3]);
        let reference = HashbrownSet::from([3, 1, 2, 3, 3]);
        let other = HashbrownSet::from([3, 4, 5]);
        test_set_trait_impl(&set, &reference, &other);
        test_misc_trait_impl(&set, &reference, &other);

        let set = ScanSet::new(vec![1, 2, 3]);
        let reference = HashbrownSet::from([1, 2, 3]);
        let other = HashbrownSet::from([1, 2, 3, 4, 5]);
        test_set_trait_impl(&set, &reference, &other);
        test_misc_trait_impl(&set, &reference, &other);

        let set = ScanSet::new(vec![1, 2, 3]);
        let reference = HashbrownSet::from([1, 2, 3]);
        let other = HashbrownSet::from([1, 2]);
        test_set_trait_impl(&set, &reference, &other);
        test_misc_trait_impl(&set, &reference, &other);

        let set = ScanSet::new(vec![1, 2, 3]);
        let reference = HashbrownSet::from([1, 2, 3]);
        let other = HashbrownSet::from([]);
        test_set_trait_impl(&set, &reference, &other);
        test_misc_trait_impl(&set, &reference, &other);

        assert_ne!(&set, &HashbrownSet::<i32>::from_iter(0..1234));
    }

    test_misc_trait_impl!(ScanSet<i32>, i32);
}
