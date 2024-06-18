use core::borrow::Borrow;
use core::fmt::Debug;
use core::hash::Hash;
use core::ops::{BitAnd, BitOr, BitXor, Sub};

use crate::inline_maps::InlineScanMap;
use crate::sets::decl_macros::{
    bitand_fn, bitor_fn, bitxor_fn, contains_fn, debug_fn, get_fn, into_iter_fn, into_iter_ref_fn,
    partial_eq_fn, set_boilerplate, set_iterator_boilerplate, sub_fn,
};
use crate::sets::{IntoIter, Iter};
use crate::traits::{Len, MapIterator, Set, SetIterator};

/// A general purpose set implemented using linear scanning.
///
#[doc = include_str!("../doc_snippets/type_compat_warning.md")]
#[doc = include_str!("../doc_snippets/about.md")]
///
/// # Type Parameters
///
/// - `T`: The value type.
/// - `SZ`: The number of entries in the set.
#[derive(Clone)]
pub struct InlineScanSet<T, const SZ: usize> {
    map: InlineScanMap<T, (), SZ>,
}

impl<T, const SZ: usize> InlineScanSet<T, SZ> {
    /// Creates a frozen set.
    #[must_use]
    pub const fn new(map: InlineScanMap<T, (), SZ>) -> Self {
        Self { map }
    }
}

impl<T, const SZ: usize> InlineScanSet<T, SZ> {
    get_fn!(Eq);
    contains_fn!(Eq);
}

impl<T, const SZ: usize> Len for InlineScanSet<T, SZ> {
    fn len(&self) -> usize {
        SZ
    }
}

impl<T, const SZ: usize> Debug for InlineScanSet<T, SZ>
where
    T: Debug,
{
    debug_fn!();
}

impl<T, const SZ: usize> IntoIterator for InlineScanSet<T, SZ> {
    into_iter_fn!();
}

impl<'a, T, const SZ: usize> IntoIterator for &'a InlineScanSet<T, SZ> {
    into_iter_ref_fn!();
}

impl<T, const SZ: usize> SetIterator<T> for InlineScanSet<T, SZ> {
    type Iterator<'a>
        = Iter<'a, T>
    where
        T: 'a;

    set_iterator_boilerplate!();
}

impl<T, const SZ: usize> Set<T> for InlineScanSet<T, SZ>
where
    T: Eq,
{
    set_boilerplate!();
}

impl<T, ST, const SZ: usize> BitOr<&ST> for &InlineScanSet<T, SZ>
where
    T: Hash + Eq + Clone,
    ST: Set<T>,
{
    bitor_fn!(RandomState);
}

impl<T, ST, const SZ: usize> BitAnd<&ST> for &InlineScanSet<T, SZ>
where
    T: Hash + Eq + Clone,
    ST: Set<T>,
{
    bitand_fn!(RandomState);
}

impl<T, ST, const SZ: usize> BitXor<&ST> for &InlineScanSet<T, SZ>
where
    T: Hash + Eq + Clone,
    ST: Set<T>,
{
    bitxor_fn!(RandomState);
}

impl<T, ST, const SZ: usize> Sub<&ST> for &InlineScanSet<T, SZ>
where
    T: Hash + Eq + Clone,
    ST: Set<T>,
{
    sub_fn!(RandomState);
}

impl<T, ST, const SZ: usize> PartialEq<ST> for InlineScanSet<T, SZ>
where
    T: Eq,
    ST: Set<T>,
{
    partial_eq_fn!();
}

impl<T, const SZ: usize> Eq for InlineScanSet<T, SZ> where T: Eq {}
