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
#[doc = include_str!("../doc_snippets/type_compat_warning.md")]
#[doc = include_str!("../doc_snippets/about.md")]
#[doc = include_str!("../doc_snippets/order_warning.md")]
///
#[derive(Clone)]
pub struct BinarySearchSet<T> {
    map: BinarySearchMap<T, ()>,
}

impl<T> BinarySearchSet<T>
where
    T: Ord,
{
    /// Creates a frozen set.
    #[must_use]
    pub const fn new(map: BinarySearchMap<T, ()>) -> Self {
        Self { map }
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
    type Iterator<'a>
        = Iter<'a, T>
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
