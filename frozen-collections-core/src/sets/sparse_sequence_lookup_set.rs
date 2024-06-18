use crate::maps::SparseSequenceLookupMap;
use crate::sets::decl_macros::{
    bitand_fn, bitor_fn, bitxor_fn, contains_fn, debug_fn, get_fn, into_iter_fn, into_iter_ref_fn,
    partial_eq_fn, set_boilerplate, set_iterator_boilerplate, sub_fn,
};
use crate::sets::{IntoIter, Iter};
use crate::traits::{CollectionMagnitude, Len, MapIterator, Sequence, Set, SetIterator};
use alloc::string::String;
use alloc::vec::Vec;
use core::borrow::Borrow;
use core::fmt::Debug;
use core::hash::Hash;
use core::ops::{BitAnd, BitOr, BitXor, Sub};

/// A set whose values are a sparse range of values from a sequence.
///
/// # Compatibility Note
///
/// This type is an implementation detail of the `frozen-collections` crate.
/// This API is therefore not stable and may change at any time.
#[derive(Clone)]
pub struct SparseSequenceLookupSet<T, CM> {
    map: SparseSequenceLookupMap<T, (), CM>,
}

impl<T, CM> SparseSequenceLookupSet<T, CM>
where
    T: Sequence,
    CM: CollectionMagnitude,
    <CM as TryFrom<usize>>::Error: Debug,
{
    #[allow(clippy::missing_errors_doc)]
    pub fn new(entries: Vec<T>) -> core::result::Result<Self, String> {
        Ok(Self {
            map: SparseSequenceLookupMap::new(entries.into_iter().map(|x| (x, ())).collect())?,
        })
    }

    #[must_use]
    #[allow(clippy::missing_errors_doc)]
    pub fn new_internal(entries: Vec<T>) -> Self {
        Self {
            map: SparseSequenceLookupMap::new_internal(
                entries.into_iter().map(|x| (x, ())).collect(),
            ),
        }
    }
}

impl<T, CM> SparseSequenceLookupSet<T, CM>
where
    CM: CollectionMagnitude,
{
    get_fn!(Sequence);
    contains_fn!(Sequence);
}

impl<T, CM> Len for SparseSequenceLookupSet<T, CM> {
    fn len(&self) -> usize {
        self.map.len()
    }
}

impl<T, CM> Debug for SparseSequenceLookupSet<T, CM>
where
    T: Debug,
{
    debug_fn!();
}

impl<T, CM> Default for SparseSequenceLookupSet<T, CM>
where
    T: Sequence,
{
    fn default() -> Self {
        Self {
            map: SparseSequenceLookupMap::default(),
        }
    }
}

impl<T, CM> IntoIterator for SparseSequenceLookupSet<T, CM> {
    into_iter_fn!();
}

impl<'a, T, CM> IntoIterator for &'a SparseSequenceLookupSet<T, CM>
where
    T: Sequence,
    CM: CollectionMagnitude,
{
    into_iter_ref_fn!();
}

impl<T, CM> SetIterator<T> for SparseSequenceLookupSet<T, CM> {
    type Iterator<'a> = Iter<'a, T>
    where
        T: 'a,
        CM: 'a;

    set_iterator_boilerplate!();
}

impl<T, CM> Set<T> for SparseSequenceLookupSet<T, CM>
where
    T: Sequence,
    CM: CollectionMagnitude,
{
    set_boilerplate!();
}

impl<T, ST, CM> BitOr<&ST> for &SparseSequenceLookupSet<T, CM>
where
    T: Sequence + Hash,
    ST: Set<T>,
    CM: CollectionMagnitude,
{
    bitor_fn!(RandomState);
}

impl<T, ST, CM> BitAnd<&ST> for &SparseSequenceLookupSet<T, CM>
where
    T: Sequence + Hash,
    ST: Set<T>,
    CM: CollectionMagnitude,
{
    bitand_fn!(RandomState);
}

impl<T, ST, CM> BitXor<&ST> for &SparseSequenceLookupSet<T, CM>
where
    T: Sequence + Hash,
    ST: Set<T>,
    CM: CollectionMagnitude,
{
    bitxor_fn!(RandomState);
}

impl<T, ST, CM> Sub<&ST> for &SparseSequenceLookupSet<T, CM>
where
    T: Sequence + Hash,
    ST: Set<T>,
    CM: CollectionMagnitude,
{
    sub_fn!(RandomState);
}

impl<T, ST, CM> PartialEq<ST> for SparseSequenceLookupSet<T, CM>
where
    T: Sequence,
    ST: Set<T>,
    CM: CollectionMagnitude,
{
    partial_eq_fn!();
}

impl<T, CM> Eq for SparseSequenceLookupSet<T, CM>
where
    T: Sequence,
    CM: CollectionMagnitude,
{
}

#[cfg(test)]
mod tests {
    use super::SparseSequenceLookupSet;
    use crate::sets::set_tests::{test_misc_trait_impl, test_set_trait_impl};
    use crate::traits::SmallCollection;
    use hashbrown::HashSet as HashbrownSet;

    #[test]
    fn test_bad_new_args() {
        assert!(SparseSequenceLookupSet::<_, SmallCollection>::new(vec![0, 256]).is_err());
        assert!(
            SparseSequenceLookupSet::<_, SmallCollection>::new(vec![u128::MIN, u128::MAX]).is_err()
        );
    }

    #[test]
    fn test_sparse_sequence_lookup_set() -> Result<(), String> {
        let set = SparseSequenceLookupSet::new_internal(vec![1, 2, 3, 7]);
        let reference = HashbrownSet::from([1, 2, 3, 7]);
        let other = HashbrownSet::from([3, 4, 5]);
        test_set_trait_impl(&set, &reference, &other);
        test_misc_trait_impl(&set, &reference, &other);

        let set = SparseSequenceLookupSet::new((0..255).collect())?;
        let reference = HashbrownSet::from_iter(0..255);
        let other = HashbrownSet::from([3, 4, 5]);
        test_set_trait_impl(&set, &reference, &other);
        test_misc_trait_impl(&set, &reference, &other);

        let set = SparseSequenceLookupSet::new(vec![])?;
        let reference = HashbrownSet::from([]);
        let other = HashbrownSet::from([3, 4, 5]);
        test_set_trait_impl(&set, &reference, &other);
        test_misc_trait_impl(&set, &reference, &other);

        let set = SparseSequenceLookupSet::new(vec![3, 1, 2, 3, 3])?;
        let reference = HashbrownSet::from([3, 1, 2, 3, 3]);
        let other = HashbrownSet::from([3, 4, 5]);
        test_set_trait_impl(&set, &reference, &other);
        test_misc_trait_impl(&set, &reference, &other);

        let set = SparseSequenceLookupSet::new(vec![1, 2, 3])?;
        let reference = HashbrownSet::from([1, 2, 3]);
        let other = HashbrownSet::from([1, 2, 3, 4, 5]);
        test_set_trait_impl(&set, &reference, &other);
        test_misc_trait_impl(&set, &reference, &other);

        let set = SparseSequenceLookupSet::new(vec![1, 2, 3])?;
        let reference = HashbrownSet::from([1, 2, 3]);
        let other = HashbrownSet::from([1, 2]);
        test_set_trait_impl(&set, &reference, &other);
        test_misc_trait_impl(&set, &reference, &other);

        let set = SparseSequenceLookupSet::new(vec![1, 2, 3])?;
        let reference = HashbrownSet::from([1, 2, 3]);
        let other = HashbrownSet::from([]);
        test_set_trait_impl(&set, &reference, &other);
        test_misc_trait_impl(&set, &reference, &other);

        assert_ne!(&set, &HashbrownSet::<i32>::from_iter(0..1234));

        Ok(())
    }

    test_misc_trait_impl!(SparseSequenceLookupSet<i32, SmallCollection>, i32);
}
