use core::fmt::Debug;
use core::hash::Hash;
use core::ops::{BitAnd, BitOr, BitXor, Sub};
use equivalent::Comparable;

use crate::inline_maps::InlineDenseScalarLookupMap;
use crate::sets::decl_macros::{
    bitand_trait_funcs, bitor_trait_funcs, bitxor_trait_funcs, common_primary_funcs, debug_trait_funcs, dense_scalar_lookup_primary_funcs,
    into_iterator_ref_trait_funcs, into_iterator_trait_funcs, partial_eq_trait_funcs, set_extras_trait_funcs, set_iteration_trait_funcs,
    set_query_trait_funcs, sub_trait_funcs,
};
use crate::sets::{IntoIter, Iter};
use crate::traits::{Len, Scalar, Set, SetExtras, SetIteration, SetOps, SetQuery};

use crate::maps::decl_macros::len_trait_funcs;
#[cfg(feature = "serde")]
use {
    crate::sets::decl_macros::serialize_trait_funcs,
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

impl<T, const SZ: usize> InlineDenseScalarLookupSet<T, SZ> {
    /// Creates a frozen set.
    #[must_use]
    pub const fn new(map: InlineDenseScalarLookupMap<T, (), SZ>) -> Self {
        Self { map }
    }

    dense_scalar_lookup_primary_funcs!();
    common_primary_funcs!(const_len);
}

impl<T, Q, const SZ: usize> Set<T, Q> for InlineDenseScalarLookupSet<T, SZ> where Q: Comparable<T> + Scalar {}

impl<T, Q, const SZ: usize> SetExtras<T, Q> for InlineDenseScalarLookupSet<T, SZ>
where
    Q: Scalar + Comparable<T>,
{
    set_extras_trait_funcs!();
}

impl<T, Q, const SZ: usize> SetQuery<Q> for InlineDenseScalarLookupSet<T, SZ>
where
    Q: Scalar + Comparable<T>,
{
    set_query_trait_funcs!();
}

impl<T, const SZ: usize> SetIteration<T> for InlineDenseScalarLookupSet<T, SZ> {
    type Iterator<'a>
        = Iter<'a, T>
    where
        T: 'a;

    set_iteration_trait_funcs!();
}

impl<T, const SZ: usize> Len for InlineDenseScalarLookupSet<T, SZ> {
    len_trait_funcs!();
}

impl<T, ST, const SZ: usize> BitOr<&ST> for &InlineDenseScalarLookupSet<T, SZ>
where
    T: Hash + Scalar,
    ST: Set<T>,
{
    bitor_trait_funcs!();
}

impl<T, ST, const SZ: usize> BitAnd<&ST> for &InlineDenseScalarLookupSet<T, SZ>
where
    T: Hash + Scalar,
    ST: Set<T>,
{
    bitand_trait_funcs!();
}

impl<T, ST, const SZ: usize> BitXor<&ST> for &InlineDenseScalarLookupSet<T, SZ>
where
    T: Hash + Scalar,
    ST: Set<T>,
{
    bitxor_trait_funcs!();
}

impl<T, ST, const SZ: usize> Sub<&ST> for &InlineDenseScalarLookupSet<T, SZ>
where
    T: Hash + Scalar,
    ST: Set<T>,
{
    sub_trait_funcs!();
}

impl<T, const SZ: usize> IntoIterator for InlineDenseScalarLookupSet<T, SZ> {
    into_iterator_trait_funcs!();
}

impl<'a, T, const SZ: usize> IntoIterator for &'a InlineDenseScalarLookupSet<T, SZ> {
    into_iterator_ref_trait_funcs!();
}

impl<T, ST, const SZ: usize> PartialEq<ST> for InlineDenseScalarLookupSet<T, SZ>
where
    T: Scalar,
    ST: SetQuery<T>,
{
    partial_eq_trait_funcs!();
}

impl<T, const SZ: usize> Eq for InlineDenseScalarLookupSet<T, SZ> where T: Scalar {}

impl<T, const SZ: usize> Debug for InlineDenseScalarLookupSet<T, SZ>
where
    T: Debug,
{
    debug_trait_funcs!();
}

#[cfg(feature = "serde")]
impl<T, const SZ: usize> Serialize for InlineDenseScalarLookupSet<T, SZ>
where
    T: Serialize,
{
    serialize_trait_funcs!();
}
