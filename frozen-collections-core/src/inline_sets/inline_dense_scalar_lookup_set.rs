use core::borrow::Borrow;
use core::fmt::Debug;
use core::hash::Hash;
use core::ops::{BitAnd, BitOr, BitXor, Sub};

use crate::inline_maps::InlineDenseScalarLookupMap;
use crate::sets::decl_macros::{
    bitand_fn, bitor_fn, bitxor_fn, contains_fn, debug_fn, get_fn, into_iter_fn, into_iter_ref_fn,
    partial_eq_fn, set_boilerplate, set_iterator_boilerplate, sub_fn,
};
use crate::sets::{IntoIter, Iter};
use crate::traits::{Len, MapIterator, Scalar, Set, SetIterator};

/// A set whose values are a continuous range in a sequence of scalar values.
///
#[doc = include_str!("../doc_snippets/type_compat_warning.md")]
#[doc = include_str!("../doc_snippets/about.md")]
///
/// # Type Parameters
///
/// - `T`: The value type.
/// - `SZ`: The number of entries in the set.
#[derive(Clone)]
pub struct InlineDenseScalarLookupSet<T, const SZ: usize> {
    map: InlineDenseScalarLookupMap<T, (), SZ>,
}

impl<T, const SZ: usize> InlineDenseScalarLookupSet<T, SZ>
where
    T: Scalar,
{
    /// Creates a frozen set.
    #[must_use]
    pub const fn new(map: InlineDenseScalarLookupMap<T, (), SZ>) -> Self {
        Self { map }
    }

    get_fn!(Scalar);
    contains_fn!(Scalar);
}

impl<T, const SZ: usize> Len for InlineDenseScalarLookupSet<T, SZ> {
    fn len(&self) -> usize {
        SZ
    }
}

impl<T, const SZ: usize> Debug for InlineDenseScalarLookupSet<T, SZ>
where
    T: Debug,
{
    debug_fn!();
}

impl<T, const SZ: usize> IntoIterator for InlineDenseScalarLookupSet<T, SZ> {
    into_iter_fn!();
}

impl<'a, T, const SZ: usize> IntoIterator for &'a InlineDenseScalarLookupSet<T, SZ> {
    into_iter_ref_fn!();
}

impl<T, const SZ: usize> SetIterator<T> for InlineDenseScalarLookupSet<T, SZ> {
    type Iterator<'a>
        = Iter<'a, T>
    where
        T: 'a;

    set_iterator_boilerplate!();
}

impl<T, const SZ: usize> Set<T> for InlineDenseScalarLookupSet<T, SZ>
where
    T: Scalar,
{
    set_boilerplate!();
}

impl<T, ST, const SZ: usize> BitOr<&ST> for &InlineDenseScalarLookupSet<T, SZ>
where
    T: Scalar + Hash,
    ST: Set<T>,
{
    bitor_fn!(RandomState);
}

impl<T, ST, const SZ: usize> BitAnd<&ST> for &InlineDenseScalarLookupSet<T, SZ>
where
    T: Scalar + Hash,
    ST: Set<T>,
{
    bitand_fn!(RandomState);
}

impl<T, ST, const SZ: usize> BitXor<&ST> for &InlineDenseScalarLookupSet<T, SZ>
where
    T: Scalar + Hash,
    ST: Set<T>,
{
    bitxor_fn!(RandomState);
}

impl<T, ST, const SZ: usize> Sub<&ST> for &InlineDenseScalarLookupSet<T, SZ>
where
    T: Scalar + Hash,
    ST: Set<T>,
{
    sub_fn!(RandomState);
}

impl<T, ST, const SZ: usize> PartialEq<ST> for InlineDenseScalarLookupSet<T, SZ>
where
    T: Scalar,
    ST: Set<T>,
{
    partial_eq_fn!();
}

impl<T, const SZ: usize> Eq for InlineDenseScalarLookupSet<T, SZ> where T: Scalar {}
