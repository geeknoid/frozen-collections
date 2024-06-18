use core::borrow::Borrow;
use core::fmt::Debug;
use core::hash::Hash;
use core::ops::{BitAnd, BitOr, BitXor, Sub};

use crate::inline_maps::InlineDenseSequenceLookupMap;
use crate::sets::decl_macros::{
    bitand_fn, bitor_fn, bitxor_fn, contains_fn, debug_fn, get_fn, into_iter_fn, into_iter_ref_fn,
    partial_eq_fn, set_boilerplate, set_iterator_boilerplate, sub_fn,
};
use crate::sets::{IntoIter, Iter};
use crate::traits::{Len, MapIterator, Sequence, Set, SetIterator};

/// A set whose values are a continuous range in a sequence.
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
pub struct InlineDenseSequenceLookupSet<T, const SZ: usize> {
    map: InlineDenseSequenceLookupMap<T, (), SZ>,
}

impl<T, const SZ: usize> InlineDenseSequenceLookupSet<T, SZ> {
    pub const fn new(map: InlineDenseSequenceLookupMap<T, (), SZ>) -> Self {
        Self { map }
    }
}

impl<T, const SZ: usize> InlineDenseSequenceLookupSet<T, SZ> {
    get_fn!(Sequence);
    contains_fn!(Sequence);
}

impl<T, const SZ: usize> Len for InlineDenseSequenceLookupSet<T, SZ> {
    fn len(&self) -> usize {
        SZ
    }
}

impl<T, const SZ: usize> Debug for InlineDenseSequenceLookupSet<T, SZ>
where
    T: Debug,
{
    debug_fn!();
}

impl<T, const SZ: usize> IntoIterator for InlineDenseSequenceLookupSet<T, SZ> {
    into_iter_fn!();
}

impl<'a, T, const SZ: usize> IntoIterator for &'a InlineDenseSequenceLookupSet<T, SZ> {
    into_iter_ref_fn!();
}

impl<T, const SZ: usize> SetIterator<T> for InlineDenseSequenceLookupSet<T, SZ> {
    type Iterator<'a> = Iter<'a, T>
    where
        T: 'a;

    set_iterator_boilerplate!();
}

impl<T, const SZ: usize> Set<T> for InlineDenseSequenceLookupSet<T, SZ>
where
    T: Sequence,
{
    set_boilerplate!();
}

impl<T, ST, const SZ: usize> BitOr<&ST> for &InlineDenseSequenceLookupSet<T, SZ>
where
    T: Sequence + Hash,
    ST: Set<T>,
{
    bitor_fn!(RandomState);
}

impl<T, ST, const SZ: usize> BitAnd<&ST> for &InlineDenseSequenceLookupSet<T, SZ>
where
    T: Sequence + Hash,
    ST: Set<T>,
{
    bitand_fn!(RandomState);
}

impl<T, ST, const SZ: usize> BitXor<&ST> for &InlineDenseSequenceLookupSet<T, SZ>
where
    T: Sequence + Hash,
    ST: Set<T>,
{
    bitxor_fn!(RandomState);
}

impl<T, ST, const SZ: usize> Sub<&ST> for &InlineDenseSequenceLookupSet<T, SZ>
where
    T: Sequence + Hash,
    ST: Set<T>,
{
    sub_fn!(RandomState);
}

impl<T, ST, const SZ: usize> PartialEq<ST> for InlineDenseSequenceLookupSet<T, SZ>
where
    T: Sequence,
    ST: Set<T>,
{
    partial_eq_fn!();
}

impl<T, const SZ: usize> Eq for InlineDenseSequenceLookupSet<T, SZ> where T: Sequence {}
