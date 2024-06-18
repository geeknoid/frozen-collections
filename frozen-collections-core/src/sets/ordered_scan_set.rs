use crate::maps::OrderedScanMap;
use crate::sets::decl_macros::{
    bitand_fn, bitor_fn, bitxor_fn, contains_fn, debug_fn, get_fn, into_iter_fn, into_iter_ref_fn,
    partial_eq_fn, set_boilerplate, set_iterator_boilerplate, sub_fn,
};
use crate::sets::{IntoIter, Iter};
use crate::traits::{Len, MapIterator, Set, SetIterator};
use core::borrow::Borrow;
use core::fmt::Debug;
use core::hash::Hash;
use core::ops::{BitAnd, BitOr, BitXor, Sub};

/// A general purpose set implemented with linear scanning.
///
#[doc = include_str!("../doc_snippets/type_compat_warning.md")]
#[doc = include_str!("../doc_snippets/about.md")]
#[doc = include_str!("../doc_snippets/order_warning.md")]
///
#[derive(Clone)]
pub struct OrderedScanSet<T> {
    map: OrderedScanMap<T, ()>,
}

impl<T> OrderedScanSet<T>
where
    T: Ord,
{
    /// Creates a frozen set.
    #[must_use]
    pub const fn new(map: OrderedScanMap<T, ()>) -> Self {
        Self { map }
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
