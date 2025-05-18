use crate::maps::DenseScalarLookupMap;
use crate::sets::decl_macros::{
    bitand_fn, bitor_fn, bitxor_fn, debug_fn, get_fn, into_iter_fn, into_iter_ref_fn,
    partial_eq_fn, set_iteration_funcs, sub_fn,
};
use crate::sets::{IntoIter, Iter};
use crate::traits::{Len, MapIteration, MapQuery, Scalar, Set, SetIteration, SetOps, SetQuery};
use core::fmt::Debug;
use core::hash::Hash;
use core::ops::{BitAnd, BitOr, BitXor, Sub};

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
#[derive(Clone)]
pub struct DenseScalarLookupSet<T> {
    map: DenseScalarLookupMap<T, ()>,
}

impl<T> DenseScalarLookupSet<T>
where
    T: Scalar,
{
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
}

impl<T> Default for DenseScalarLookupSet<T>
where
    T: Scalar,
{
    fn default() -> Self {
        Self {
            map: DenseScalarLookupMap::default(),
        }
    }
}

impl<T> Set<T, T> for DenseScalarLookupSet<T> where T: Scalar {}

impl<T> SetQuery<T, T> for DenseScalarLookupSet<T>
where
    T: Scalar,
{
    get_fn!("Scalar");
}

impl<T> SetIteration<T> for DenseScalarLookupSet<T> {
    type Iterator<'a>
        = Iter<'a, T>
    where
        T: 'a;

    set_iteration_funcs!();
}

impl<T> Len for DenseScalarLookupSet<T> {
    fn len(&self) -> usize {
        self.map.len()
    }
}

impl<T, ST> BitOr<&ST> for &DenseScalarLookupSet<T>
where
    T: Scalar + Hash,
    ST: Set<T>,
{
    bitor_fn!();
}

impl<T, ST> BitAnd<&ST> for &DenseScalarLookupSet<T>
where
    T: Scalar + Hash,
    ST: Set<T>,
{
    bitand_fn!();
}

impl<T, ST> BitXor<&ST> for &DenseScalarLookupSet<T>
where
    T: Scalar + Hash,
    ST: Set<T>,
{
    bitxor_fn!();
}

impl<T, ST> Sub<&ST> for &DenseScalarLookupSet<T>
where
    T: Scalar + Hash,
    ST: Set<T>,
{
    sub_fn!();
}

impl<T> IntoIterator for DenseScalarLookupSet<T> {
    into_iter_fn!();
}

impl<'a, T> IntoIterator for &'a DenseScalarLookupSet<T> {
    into_iter_ref_fn!();
}

impl<T, ST> PartialEq<ST> for DenseScalarLookupSet<T>
where
    T: Scalar,
    ST: Set<T>,
{
    partial_eq_fn!();
}

impl<T> Eq for DenseScalarLookupSet<T> where T: Scalar {}

impl<T> Debug for DenseScalarLookupSet<T>
where
    T: Debug,
{
    debug_fn!();
}

#[cfg(feature = "serde")]
impl<T> Serialize for DenseScalarLookupSet<T>
where
    T: Serialize,
{
    serialize_fn!();
}
