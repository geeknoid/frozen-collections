use crate::maps::SparseScalarLookupMap;
use crate::sets::decl_macros::{
    bitand_fn, bitor_fn, bitxor_fn, contains_fn, debug_fn, get_fn, into_iter_fn, into_iter_ref_fn,
    partial_eq_fn, set_boilerplate, set_iterator_boilerplate, sub_fn,
};
use crate::sets::{IntoIter, Iter};
use crate::traits::{CollectionMagnitude, Len, MapIterator, Scalar, Set, SetIterator};
use alloc::string::String;
use alloc::vec::Vec;
use core::borrow::Borrow;
use core::fmt::Debug;
use core::hash::Hash;
use core::ops::{BitAnd, BitOr, BitXor, Sub};

/// A set whose values are a sparse range of values from a scalar.
///
/// # Compatibility Note
///
/// This type is an implementation detail of the `frozen-collections` crate.
/// This API is therefore not stable and may change at any time.
#[derive(Clone)]
pub struct SparseScalarLookupSet<T, CM> {
    map: SparseScalarLookupMap<T, (), CM>,
}

impl<T, CM> SparseScalarLookupSet<T, CM>
where
    T: Scalar,
    CM: CollectionMagnitude,
    <CM as TryFrom<usize>>::Error: Debug,
{
    #[allow(clippy::missing_errors_doc)]
    pub fn new(entries: Vec<T>) -> Result<Self, String> {
        Ok(Self {
            map: SparseScalarLookupMap::new(entries.into_iter().map(|x| (x, ())).collect())?,
        })
    }

    #[must_use]
    pub fn new_raw(processed_entries: Vec<T>) -> Self {
        Self {
            map: SparseScalarLookupMap::new_raw(
                processed_entries.into_iter().map(|x| (x, ())).collect(),
            ),
        }
    }
}

impl<T, CM> SparseScalarLookupSet<T, CM>
where
    CM: CollectionMagnitude,
{
    get_fn!(Scalar);
    contains_fn!(Scalar);
}

impl<T, CM> Len for SparseScalarLookupSet<T, CM> {
    fn len(&self) -> usize {
        self.map.len()
    }
}

impl<T, CM> Debug for SparseScalarLookupSet<T, CM>
where
    T: Debug,
{
    debug_fn!();
}

impl<T, CM> Default for SparseScalarLookupSet<T, CM>
where
    T: Scalar,
{
    fn default() -> Self {
        Self {
            map: SparseScalarLookupMap::default(),
        }
    }
}

impl<T, CM> IntoIterator for SparseScalarLookupSet<T, CM> {
    into_iter_fn!();
}

impl<'a, T, CM> IntoIterator for &'a SparseScalarLookupSet<T, CM>
where
    T: Scalar,
    CM: CollectionMagnitude,
{
    into_iter_ref_fn!();
}

impl<T, CM> SetIterator<T> for SparseScalarLookupSet<T, CM> {
    type Iterator<'a>
        = Iter<'a, T>
    where
        T: 'a,
        CM: 'a;

    set_iterator_boilerplate!();
}

impl<T, CM> Set<T> for SparseScalarLookupSet<T, CM>
where
    T: Scalar,
    CM: CollectionMagnitude,
{
    set_boilerplate!();
}

impl<T, ST, CM> BitOr<&ST> for &SparseScalarLookupSet<T, CM>
where
    T: Scalar + Hash,
    ST: Set<T>,
    CM: CollectionMagnitude,
{
    bitor_fn!(RandomState);
}

impl<T, ST, CM> BitAnd<&ST> for &SparseScalarLookupSet<T, CM>
where
    T: Scalar + Hash,
    ST: Set<T>,
    CM: CollectionMagnitude,
{
    bitand_fn!(RandomState);
}

impl<T, ST, CM> BitXor<&ST> for &SparseScalarLookupSet<T, CM>
where
    T: Scalar + Hash,
    ST: Set<T>,
    CM: CollectionMagnitude,
{
    bitxor_fn!(RandomState);
}

impl<T, ST, CM> Sub<&ST> for &SparseScalarLookupSet<T, CM>
where
    T: Scalar + Hash,
    ST: Set<T>,
    CM: CollectionMagnitude,
{
    sub_fn!(RandomState);
}

impl<T, ST, CM> PartialEq<ST> for SparseScalarLookupSet<T, CM>
where
    T: Scalar,
    ST: Set<T>,
    CM: CollectionMagnitude,
{
    partial_eq_fn!();
}

impl<T, CM> Eq for SparseScalarLookupSet<T, CM>
where
    T: Scalar,
    CM: CollectionMagnitude,
{
}

#[cfg(test)]
mod tests {
    use super::SparseScalarLookupSet;
    use crate::traits::set_trait_tests::{test_misc_trait_impl, test_set_trait_impl};
    use crate::traits::{MediumCollection, SmallCollection};
    use hashbrown::HashSet as HashbrownSet;

    #[test]
    fn test_bad_new_args() {
        assert!(SparseScalarLookupSet::<_, SmallCollection>::new(vec![0, 256]).is_err());
    }

    #[test]
    fn test_sparse_scalar_lookup_set() -> Result<(), String> {
        let set = SparseScalarLookupSet::new_raw(vec![1, 2, 3, 7]);
        let reference = HashbrownSet::from([1, 2, 3, 7]);
        let other = HashbrownSet::from([3, 4, 5]);
        test_set_trait_impl(&set, &reference, &other);
        test_misc_trait_impl(&set, &reference, &other);

        let set = SparseScalarLookupSet::new((0..255).collect())?;
        let reference = HashbrownSet::from_iter(0..255);
        let other = HashbrownSet::from([3, 4, 5]);
        test_set_trait_impl(&set, &reference, &other);
        test_misc_trait_impl(&set, &reference, &other);

        let set = SparseScalarLookupSet::new(vec![])?;
        let reference = HashbrownSet::from([]);
        let other = HashbrownSet::from([3, 4, 5]);
        test_set_trait_impl(&set, &reference, &other);
        test_misc_trait_impl(&set, &reference, &other);

        let set = SparseScalarLookupSet::new(vec![3, 1, 2, 3, 3])?;
        let reference = HashbrownSet::from([3, 1, 2, 3, 3]);
        let other = HashbrownSet::from([3, 4, 5]);
        test_set_trait_impl(&set, &reference, &other);
        test_misc_trait_impl(&set, &reference, &other);

        let set = SparseScalarLookupSet::new(vec![1, 2, 3])?;
        let reference = HashbrownSet::from([1, 2, 3]);
        let other = HashbrownSet::from([1, 2, 3, 4, 5]);
        test_set_trait_impl(&set, &reference, &other);
        test_misc_trait_impl(&set, &reference, &other);

        let set = SparseScalarLookupSet::new(vec![1, 2, 3])?;
        let reference = HashbrownSet::from([1, 2, 3]);
        let other = HashbrownSet::from([1, 2]);
        test_set_trait_impl(&set, &reference, &other);
        test_misc_trait_impl(&set, &reference, &other);

        let set = SparseScalarLookupSet::new(vec![1, 2, 3])?;
        let reference = HashbrownSet::from([1, 2, 3]);
        let other = HashbrownSet::from([]);
        test_set_trait_impl(&set, &reference, &other);
        test_misc_trait_impl(&set, &reference, &other);

        assert_ne!(&set, &HashbrownSet::<i32>::from_iter(0..1234));

        Ok(())
    }

    test_misc_trait_impl!(SparseScalarLookupSet<i32, MediumCollection>, i32);
}
