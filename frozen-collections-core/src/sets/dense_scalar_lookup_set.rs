use crate::maps::DenseScalarLookupMap;
use crate::sets::decl_macros::{
    bitand_fn, bitor_fn, bitxor_fn, contains_fn, debug_fn, get_fn, into_iter_fn, into_iter_ref_fn,
    partial_eq_fn, set_boilerplate, set_iterator_boilerplate, sub_fn,
};
use crate::sets::{IntoIter, Iter};
use crate::traits::{Len, MapIterator, Scalar, Set, SetIterator};
use alloc::string::String;
use alloc::vec::Vec;
use core::borrow::Borrow;
use core::fmt::Debug;
use core::hash::Hash;
use core::ops::{BitAnd, BitOr, BitXor, Sub};

/// A set whose values are a continuous range in a sequence of scalar values.
///
/// # Compatibility Note
///
/// This type is an implementation detail of the `frozen-collections` crate.
/// This API is therefore not stable and may change at any time.
#[derive(Clone)]
pub struct DenseScalarLookupSet<T> {
    map: DenseScalarLookupMap<T, ()>,
}

impl<T> DenseScalarLookupSet<T>
where
    T: Scalar,
{
    #[allow(clippy::missing_errors_doc)]
    pub fn new(entries: Vec<T>) -> Result<Self, String> {
        Ok(Self {
            map: DenseScalarLookupMap::new(entries.into_iter().map(|x| (x, ())).collect())?,
        })
    }

    #[must_use]
    pub fn new_raw(processed_entries: Vec<T>) -> Self {
        Self {
            map: DenseScalarLookupMap::new_raw(
                processed_entries.into_iter().map(|x| (x, ())).collect(),
            ),
        }
    }
}

impl<T> DenseScalarLookupSet<T> {
    get_fn!(Scalar);
    contains_fn!(Scalar);
}

impl<T> Len for DenseScalarLookupSet<T> {
    fn len(&self) -> usize {
        self.map.len()
    }
}

impl<T> Debug for DenseScalarLookupSet<T>
where
    T: Debug,
{
    debug_fn!();
}

impl<T> Default for DenseScalarLookupSet<T>
where
    T: Scalar,
{
    fn default() -> Self {
        Self {
            map: DenseScalarLookupMap::default(),
        }
    }
}

impl<T> IntoIterator for DenseScalarLookupSet<T> {
    into_iter_fn!();
}

impl<'a, T> IntoIterator for &'a DenseScalarLookupSet<T> {
    into_iter_ref_fn!();
}

impl<T> SetIterator<T> for DenseScalarLookupSet<T> {
    type Iterator<'a>
        = Iter<'a, T>
    where
        T: 'a;

    set_iterator_boilerplate!();
}

impl<T> Set<T> for DenseScalarLookupSet<T>
where
    T: Scalar,
{
    set_boilerplate!();
}

impl<T, ST> BitOr<&ST> for &DenseScalarLookupSet<T>
where
    T: Scalar + Hash,
    ST: Set<T>,
{
    bitor_fn!(RandomState);
}

impl<T, ST> BitAnd<&ST> for &DenseScalarLookupSet<T>
where
    T: Scalar + Hash,
    ST: Set<T>,
{
    bitand_fn!(RandomState);
}

impl<T, ST> BitXor<&ST> for &DenseScalarLookupSet<T>
where
    T: Scalar + Hash,
    ST: Set<T>,
{
    bitxor_fn!(RandomState);
}

impl<T, ST> Sub<&ST> for &DenseScalarLookupSet<T>
where
    T: Scalar + Hash,
    ST: Set<T>,
{
    sub_fn!(RandomState);
}

impl<T, ST> PartialEq<ST> for DenseScalarLookupSet<T>
where
    T: Scalar,
    ST: Set<T>,
{
    partial_eq_fn!();
}

impl<T> Eq for DenseScalarLookupSet<T> where T: Scalar {}

#[cfg(test)]
mod tests {
    use super::DenseScalarLookupSet;
    use crate::traits::set_trait_tests::{test_misc_trait_impl, test_set_trait_impl};
    use hashbrown::HashSet as HashbrownSet;

    #[test]
    fn test_bad_new_args() {
        assert!(DenseScalarLookupSet::new(vec![1, 3]).is_err());
    }

    #[test]
    fn test_dense_scalar_lookup_set() -> Result<(), String> {
        let set = DenseScalarLookupSet::new_raw(vec![1, 2, 3]);
        let reference = HashbrownSet::from([1, 2, 3]);
        let other = HashbrownSet::from([3, 4, 5]);
        test_set_trait_impl(&set, &reference, &other);
        test_misc_trait_impl(&set, &reference, &other);

        let set = DenseScalarLookupSet::new(vec![])?;
        let reference = HashbrownSet::from([]);
        let other = HashbrownSet::from([3, 4, 5]);
        test_set_trait_impl(&set, &reference, &other);
        test_misc_trait_impl(&set, &reference, &other);

        let set = DenseScalarLookupSet::new(vec![3, 1, 2, 3, 3])?;
        let reference = HashbrownSet::from([3, 1, 2, 3, 3]);
        let other = HashbrownSet::from([3, 4, 5]);
        test_set_trait_impl(&set, &reference, &other);
        test_misc_trait_impl(&set, &reference, &other);

        let set = DenseScalarLookupSet::new(vec![1, 2, 3])?;
        let reference = HashbrownSet::from([1, 2, 3]);
        let other = HashbrownSet::from([1, 2, 3, 4, 5]);
        test_set_trait_impl(&set, &reference, &other);
        test_misc_trait_impl(&set, &reference, &other);

        let set = DenseScalarLookupSet::new(vec![1, 2, 3])?;
        let reference = HashbrownSet::from([1, 2, 3]);
        let other = HashbrownSet::from([1, 2]);
        test_set_trait_impl(&set, &reference, &other);
        test_misc_trait_impl(&set, &reference, &other);

        let set = DenseScalarLookupSet::new(vec![1, 2, 3])?;
        let reference = HashbrownSet::from([1, 2, 3]);
        let other = HashbrownSet::from([]);
        test_set_trait_impl(&set, &reference, &other);
        test_misc_trait_impl(&set, &reference, &other);

        assert_ne!(&set, &HashbrownSet::<i32>::from_iter(0..1234));

        Ok(())
    }

    test_misc_trait_impl!(DenseScalarLookupSet<i32>, i32);
}
