use core::fmt::Debug;
use core::hash::Hash;
use core::ops::{BitAnd, BitOr, BitXor, Sub};

use crate::inline_maps::InlineDenseScalarLookupMap;
use crate::sets::decl_macros::{
    bitand_fn, bitor_fn, bitxor_fn, debug_fn, get_fn, into_iter_fn, into_iter_ref_fn,
    partial_eq_fn, set_iteration_funcs, sub_fn,
};
use crate::sets::{IntoIter, Iter};
use crate::traits::{Len, MapIteration, MapQuery, Scalar, Set, SetIteration, SetOps, SetQuery};

#[cfg(feature = "serde")]
use {
    crate::sets::decl_macros::serialize_fn,
    serde::ser::SerializeSeq,
    serde::{Serialize, Serializer},
};

/// A set whose values are a continuous range in a sequence of scalar values.
///
#[doc = include_str!("../doc_snippets/private_api_warning.md")]
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
}

impl<T, const SZ: usize> Set<T, T> for InlineDenseScalarLookupSet<T, SZ> where T: Scalar {}

impl<T, const SZ: usize> SetQuery<T, T> for InlineDenseScalarLookupSet<T, SZ>
where
    T: Scalar,
{
    get_fn!("Scalar");
}

impl<T, const SZ: usize> SetIteration<T> for InlineDenseScalarLookupSet<T, SZ> {
    type Iterator<'a>
        = Iter<'a, T>
    where
        T: 'a;

    set_iteration_funcs!();
}

impl<T, const SZ: usize> Len for InlineDenseScalarLookupSet<T, SZ> {
    fn len(&self) -> usize {
        SZ
    }
}

impl<T, ST, const SZ: usize> BitOr<&ST> for &InlineDenseScalarLookupSet<T, SZ>
where
    T: Scalar + Hash,
    ST: Set<T>,
{
    bitor_fn!();
}

impl<T, ST, const SZ: usize> BitAnd<&ST> for &InlineDenseScalarLookupSet<T, SZ>
where
    T: Scalar + Hash,
    ST: Set<T>,
{
    bitand_fn!();
}

impl<T, ST, const SZ: usize> BitXor<&ST> for &InlineDenseScalarLookupSet<T, SZ>
where
    T: Scalar + Hash,
    ST: Set<T>,
{
    bitxor_fn!();
}

impl<T, ST, const SZ: usize> Sub<&ST> for &InlineDenseScalarLookupSet<T, SZ>
where
    T: Scalar + Hash,
    ST: Set<T>,
{
    sub_fn!();
}

impl<T, const SZ: usize> IntoIterator for InlineDenseScalarLookupSet<T, SZ> {
    into_iter_fn!();
}

impl<'a, T, const SZ: usize> IntoIterator for &'a InlineDenseScalarLookupSet<T, SZ> {
    into_iter_ref_fn!();
}

impl<T, ST, const SZ: usize> PartialEq<ST> for InlineDenseScalarLookupSet<T, SZ>
where
    T: Scalar,
    ST: Set<T>,
{
    partial_eq_fn!();
}

impl<T, const SZ: usize> Eq for InlineDenseScalarLookupSet<T, SZ> where T: Scalar {}

impl<T, const SZ: usize> Debug for InlineDenseScalarLookupSet<T, SZ>
where
    T: Debug,
{
    debug_fn!();
}

#[cfg(feature = "serde")]
impl<T, const SZ: usize> Serialize for InlineDenseScalarLookupSet<T, SZ>
where
    T: Serialize,
{
    serialize_fn!();
}
