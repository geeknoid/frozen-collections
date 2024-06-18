use crate::maps::ScanMap;
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
///
#[derive(Clone)]
pub struct ScanSet<T> {
    map: ScanMap<T, ()>,
}

impl<T> ScanSet<T>
where
    T: Eq,
{
    /// Creates a frozen set.
    #[must_use]
    pub const fn new(map: ScanMap<T, ()>) -> Self {
        Self { map }
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
    type Iterator<'a>
        = Iter<'a, T>
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
