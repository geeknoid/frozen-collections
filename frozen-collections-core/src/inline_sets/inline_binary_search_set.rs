use core::borrow::Borrow;
use core::fmt::Debug;
use core::hash::Hash;
use core::ops::{BitAnd, BitOr, BitXor, Sub};

use crate::inline_maps::InlineBinarySearchMap;
use crate::sets::decl_macros::{
    bitand_fn, bitor_fn, bitxor_fn, contains_fn, debug_fn, get_fn, into_iter_fn, into_iter_ref_fn,
    partial_eq_fn, set_boilerplate, set_iterator_boilerplate, sub_fn,
};
use crate::sets::{IntoIter, Iter};
use crate::traits::{Len, MapIterator, Set, SetIterator};

/// A general purpose set implement using binary search.
///
/// # Type Parameters
///
/// - `T`: The value type.
/// - `SZ`: The number of entries in the set.
///
/// # Compatibility Note
///
/// This type is an implementation detail of the `frozen-collections` crate.
/// This API is therefore not stable and may change at any time.
#[derive(Clone)]
pub struct InlineBinarySearchSet<T, const SZ: usize> {
    map: InlineBinarySearchMap<T, (), SZ>,
}

impl<T, const SZ: usize> InlineBinarySearchSet<T, SZ>
where
    T: Ord,
{
    #[allow(clippy::missing_errors_doc)]
    pub fn new(entries: Vec<T>) -> Result<Self, String> {
        Ok(Self {
            map: InlineBinarySearchMap::new(entries.into_iter().map(|x| (x, ())).collect())?,
        })
    }

    pub const fn new_raw(map: InlineBinarySearchMap<T, (), SZ>) -> Self {
        Self { map }
    }

    get_fn!(Ord);
    contains_fn!(Ord);
}

impl<T, const SZ: usize> Len for InlineBinarySearchSet<T, SZ> {
    fn len(&self) -> usize {
        SZ
    }
}

impl<T, const SZ: usize> Debug for InlineBinarySearchSet<T, SZ>
where
    T: Debug,
{
    debug_fn!();
}

impl<T, const SZ: usize> IntoIterator for InlineBinarySearchSet<T, SZ> {
    into_iter_fn!();
}

impl<'a, T, const SZ: usize> IntoIterator for &'a InlineBinarySearchSet<T, SZ>
where
    T: Ord,
{
    into_iter_ref_fn!();
}

impl<T, const SZ: usize> SetIterator<T> for InlineBinarySearchSet<T, SZ> {
    type Iterator<'a>
        = Iter<'a, T>
    where
        T: 'a;

    set_iterator_boilerplate!();
}

impl<T, const SZ: usize> Set<T> for InlineBinarySearchSet<T, SZ>
where
    T: Ord,
{
    set_boilerplate!();
}

impl<T, ST, const SZ: usize> BitOr<&ST> for &InlineBinarySearchSet<T, SZ>
where
    T: Hash + Ord + Clone,
    ST: Set<T>,
{
    bitor_fn!(RandomState);
}

impl<T, ST, const SZ: usize> BitAnd<&ST> for &InlineBinarySearchSet<T, SZ>
where
    T: Hash + Ord + Clone,
    ST: Set<T>,
{
    bitand_fn!(RandomState);
}

impl<T, ST, const SZ: usize> BitXor<&ST> for &InlineBinarySearchSet<T, SZ>
where
    T: Hash + Ord + Clone,
    ST: Set<T>,
{
    bitxor_fn!(RandomState);
}

impl<T, ST, const SZ: usize> Sub<&ST> for &InlineBinarySearchSet<T, SZ>
where
    T: Hash + Ord + Clone,
    ST: Set<T>,
{
    sub_fn!(RandomState);
}

impl<T, ST, const SZ: usize> PartialEq<ST> for InlineBinarySearchSet<T, SZ>
where
    T: Ord,
    ST: Set<T>,
{
    partial_eq_fn!();
}

impl<T, const SZ: usize> Eq for InlineBinarySearchSet<T, SZ> where T: Ord {}
