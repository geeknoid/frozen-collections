use crate::maps::DenseScalarLookupMap;
use crate::maps::decl_macros::len_trait_funcs;
use crate::sets::decl_macros::{
    bitand_trait_funcs, bitor_trait_funcs, bitxor_trait_funcs, common_primary_funcs, debug_trait_funcs, dense_scalar_lookup_primary_funcs,
    into_iterator_ref_trait_funcs, into_iterator_trait_funcs, partial_eq_trait_funcs, set_extras_trait_funcs, set_iteration_trait_funcs,
    set_query_trait_funcs, sub_trait_funcs,
};
use crate::sets::{IntoIter, Iter};
use crate::traits::{Len, Scalar, Set, SetExtras, SetIteration, SetOps, SetQuery};
use core::fmt::Debug;
use core::hash::Hash;
use core::ops::{BitAnd, BitOr, BitXor, Sub};
use equivalent::Comparable;

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
#[derive(Clone)]
pub struct DenseScalarLookupSet<T> {
    map: DenseScalarLookupMap<T, ()>,
}

impl<T> DenseScalarLookupSet<T> {
    /// Creates a frozen set.
    ///
    /// # Errors
    ///
    /// Fails if all the values in the input vector, after sorting and dedupping,
    /// don't represent a continuous range.
    #[must_use]
    pub const fn new(map: DenseScalarLookupMap<T, ()>) -> Self {
        Self { map }
    }

    dense_scalar_lookup_primary_funcs!();
    common_primary_funcs!(non_const_len);
}

impl<T> Default for DenseScalarLookupSet<T> {
    fn default() -> Self {
        Self {
            map: DenseScalarLookupMap::default(),
        }
    }
}

impl<T, Q> Set<T, Q> for DenseScalarLookupSet<T> where Q: Comparable<T> + Scalar {}

impl<T, Q> SetExtras<T, Q> for DenseScalarLookupSet<T>
where
    Q: Scalar + Comparable<T>,
{
    set_extras_trait_funcs!();
}

impl<T, Q> SetQuery<Q> for DenseScalarLookupSet<T>
where
    Q: Scalar + Comparable<T>,
{
    set_query_trait_funcs!();
}

impl<T> SetIteration<T> for DenseScalarLookupSet<T> {
    type Iterator<'a>
        = Iter<'a, T>
    where
        T: 'a;

    set_iteration_trait_funcs!();
}

impl<T> Len for DenseScalarLookupSet<T> {
    len_trait_funcs!();
}

impl<T, ST> BitOr<&ST> for &DenseScalarLookupSet<T>
where
    T: Scalar + Hash,
    ST: Set<T>,
{
    bitor_trait_funcs!();
}

impl<T, ST> BitAnd<&ST> for &DenseScalarLookupSet<T>
where
    T: Scalar + Hash,
    ST: Set<T>,
{
    bitand_trait_funcs!();
}

impl<T, ST> BitXor<&ST> for &DenseScalarLookupSet<T>
where
    T: Scalar + Hash,
    ST: Set<T>,
{
    bitxor_trait_funcs!();
}

impl<T, ST> Sub<&ST> for &DenseScalarLookupSet<T>
where
    T: Scalar + Hash,
    ST: Set<T>,
{
    sub_trait_funcs!();
}

impl<T> IntoIterator for DenseScalarLookupSet<T> {
    into_iterator_trait_funcs!();
}

impl<'a, T> IntoIterator for &'a DenseScalarLookupSet<T> {
    into_iterator_ref_trait_funcs!();
}

impl<T, ST> PartialEq<ST> for DenseScalarLookupSet<T>
where
    T: Scalar,
    ST: SetQuery<T>,
{
    partial_eq_trait_funcs!();
}

impl<T> Eq for DenseScalarLookupSet<T> where T: Scalar {}

impl<T> Debug for DenseScalarLookupSet<T>
where
    T: Debug,
{
    debug_trait_funcs!();
}

#[cfg(feature = "serde")]
impl<T> Serialize for DenseScalarLookupSet<T>
where
    T: Serialize,
{
    serialize_trait_funcs!();
}
