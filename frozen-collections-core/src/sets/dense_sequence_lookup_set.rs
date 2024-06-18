use crate::maps::DenseSequenceLookupMap;
use crate::sets::decl_macros::{
    bitand_fn, bitor_fn, bitxor_fn, contains_fn, debug_fn, get_fn, into_iter_fn, into_iter_ref_fn,
    partial_eq_fn, set_boilerplate, set_iterator_boilerplate, sub_fn,
};
use crate::sets::{IntoIter, Iter};
use crate::traits::{Len, MapIterator, Sequence, Set, SetIterator};
use alloc::string::String;
use alloc::vec::Vec;
use core::borrow::Borrow;
use core::fmt::Debug;
use core::hash::Hash;
use core::ops::{BitAnd, BitOr, BitXor, Sub};

/// A set whose values are a continuous range in a sequence.
///
/// # Compatibility Note
///
/// This type is an implementation detail of the `frozen-collections` crate.
/// This API is therefore not stable and may change at any time.
#[derive(Clone)]
pub struct DenseSequenceLookupSet<T> {
    map: DenseSequenceLookupMap<T, ()>,
}

impl<T> DenseSequenceLookupSet<T>
where
    T: Sequence,
{
    #[allow(clippy::missing_errors_doc)]
    pub fn new(entries: Vec<T>) -> core::result::Result<Self, String> {
        Ok(Self {
            map: DenseSequenceLookupMap::new(entries.into_iter().map(|x| (x, ())).collect())?,
        })
    }

    #[must_use]
    #[allow(clippy::missing_errors_doc)]
    pub fn new_internal(entries: Vec<T>) -> Self {
        Self {
            map: DenseSequenceLookupMap::new_internal(
                entries.into_iter().map(|x| (x, ())).collect(),
            ),
        }
    }
}

impl<T> DenseSequenceLookupSet<T> {
    get_fn!(Sequence);
    contains_fn!(Sequence);
}

impl<T> Len for DenseSequenceLookupSet<T> {
    fn len(&self) -> usize {
        self.map.len()
    }
}

impl<T> Debug for DenseSequenceLookupSet<T>
where
    T: Debug,
{
    debug_fn!();
}

impl<T> Default for DenseSequenceLookupSet<T>
where
    T: Sequence,
{
    fn default() -> Self {
        Self {
            map: DenseSequenceLookupMap::default(),
        }
    }
}

impl<T> IntoIterator for DenseSequenceLookupSet<T> {
    into_iter_fn!();
}

impl<'a, T> IntoIterator for &'a DenseSequenceLookupSet<T> {
    into_iter_ref_fn!();
}

impl<T> SetIterator<T> for DenseSequenceLookupSet<T> {
    type Iterator<'a> = Iter<'a, T>
    where
        T: 'a;

    set_iterator_boilerplate!();
}

impl<T> Set<T> for DenseSequenceLookupSet<T>
where
    T: Sequence,
{
    set_boilerplate!();
}

impl<T, ST> BitOr<&ST> for &DenseSequenceLookupSet<T>
where
    T: Sequence + Hash,
    ST: Set<T>,
{
    bitor_fn!(RandomState);
}

impl<T, ST> BitAnd<&ST> for &DenseSequenceLookupSet<T>
where
    T: Sequence + Hash,
    ST: Set<T>,
{
    bitand_fn!(RandomState);
}

impl<T, ST> BitXor<&ST> for &DenseSequenceLookupSet<T>
where
    T: Sequence + Hash,
    ST: Set<T>,
{
    bitxor_fn!(RandomState);
}

impl<T, ST> Sub<&ST> for &DenseSequenceLookupSet<T>
where
    T: Sequence + Hash,
    ST: Set<T>,
{
    sub_fn!(RandomState);
}

impl<T, ST> PartialEq<ST> for DenseSequenceLookupSet<T>
where
    T: Sequence,
    ST: Set<T>,
{
    partial_eq_fn!();
}

impl<T> Eq for DenseSequenceLookupSet<T> where T: Sequence {}

#[cfg(test)]
mod tests {
    use super::DenseSequenceLookupSet;
    use crate::sets::set_tests::{test_misc_trait_impl, test_set_trait_impl};
    use hashbrown::HashSet as HashbrownSet;

    #[test]
    fn test_bad_new_args() {
        assert!(DenseSequenceLookupSet::new(vec![1, 3]).is_err());
        assert!(DenseSequenceLookupSet::new(vec![u128::MIN, u128::MAX]).is_err());
    }

    #[test]
    fn test_dense_sequence_lookup_set() -> Result<(), String> {
        let set = DenseSequenceLookupSet::new_internal(vec![1, 2, 3]);
        let reference = HashbrownSet::from([1, 2, 3]);
        let other = HashbrownSet::from([3, 4, 5]);
        test_set_trait_impl(&set, &reference, &other);
        test_misc_trait_impl(&set, &reference, &other);

        let set = DenseSequenceLookupSet::new(vec![])?;
        let reference = HashbrownSet::from([]);
        let other = HashbrownSet::from([3, 4, 5]);
        test_set_trait_impl(&set, &reference, &other);
        test_misc_trait_impl(&set, &reference, &other);

        let set = DenseSequenceLookupSet::new(vec![3, 1, 2, 3, 3])?;
        let reference = HashbrownSet::from([3, 1, 2, 3, 3]);
        let other = HashbrownSet::from([3, 4, 5]);
        test_set_trait_impl(&set, &reference, &other);
        test_misc_trait_impl(&set, &reference, &other);

        let set = DenseSequenceLookupSet::new(vec![1, 2, 3])?;
        let reference = HashbrownSet::from([1, 2, 3]);
        let other = HashbrownSet::from([1, 2, 3, 4, 5]);
        test_set_trait_impl(&set, &reference, &other);
        test_misc_trait_impl(&set, &reference, &other);

        let set = DenseSequenceLookupSet::new(vec![1, 2, 3])?;
        let reference = HashbrownSet::from([1, 2, 3]);
        let other = HashbrownSet::from([1, 2]);
        test_set_trait_impl(&set, &reference, &other);
        test_misc_trait_impl(&set, &reference, &other);

        let set = DenseSequenceLookupSet::new(vec![1, 2, 3])?;
        let reference = HashbrownSet::from([1, 2, 3]);
        let other = HashbrownSet::from([]);
        test_set_trait_impl(&set, &reference, &other);
        test_misc_trait_impl(&set, &reference, &other);

        assert_ne!(&set, &HashbrownSet::<i32>::from_iter(0..1234));

        Ok(())
    }

    test_misc_trait_impl!(DenseSequenceLookupSet<i32>, i32);
}
