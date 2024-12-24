use crate::maps::SparseScalarLookupMap;
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

/// A set whose values are a sparse range of values from a scalar.
///
#[doc = include_str!("../doc_snippets/type_compat_warning.md")]
#[doc = include_str!("../doc_snippets/about.md")]
///
#[derive(Clone)]
pub struct SparseScalarLookupSet<T> {
    map: SparseScalarLookupMap<T, ()>,
}

impl<T> SparseScalarLookupSet<T>
where
    T: Scalar,
{
    /// Creates a frozen set.
    ///
    /// # Errors
    ///
    /// Fails if the number of entries in the vector, after deduplication, exceeds the
    /// magnitude of the collection as specified by the `CM` generic argument.
    #[must_use]
    pub const fn new(map: SparseScalarLookupMap<T, ()>) -> Self {
        Self { map }
    }
}

impl<T> Default for SparseScalarLookupSet<T>
where
    T: Scalar,
{
    fn default() -> Self {
        Self {
            map: SparseScalarLookupMap::default(),
        }
    }
}

impl<T> Set<T, T> for SparseScalarLookupSet<T> where T: Scalar {}

impl<T> SetQuery<T, T> for SparseScalarLookupSet<T>
where
    T: Scalar,
{
    get_fn!("Scalar");
}

impl<T> SetIteration<T> for SparseScalarLookupSet<T> {
    type Iterator<'a>
        = Iter<'a, T>
    where
        T: 'a;

    set_iteration_funcs!();
}

impl<T> Len for SparseScalarLookupSet<T> {
    fn len(&self) -> usize {
        self.map.len()
    }
}

impl<T, ST> BitOr<&ST> for &SparseScalarLookupSet<T>
where
    T: Scalar + Hash,
    ST: Set<T>,
{
    bitor_fn!(RandomState);
}

impl<T, ST> BitAnd<&ST> for &SparseScalarLookupSet<T>
where
    T: Scalar + Hash,
    ST: Set<T>,
{
    bitand_fn!(RandomState);
}

impl<T, ST> BitXor<&ST> for &SparseScalarLookupSet<T>
where
    T: Scalar + Hash,
    ST: Set<T>,
{
    bitxor_fn!(RandomState);
}

impl<T, ST> Sub<&ST> for &SparseScalarLookupSet<T>
where
    T: Scalar + Hash,
    ST: Set<T>,
{
    sub_fn!(RandomState);
}

impl<T> IntoIterator for SparseScalarLookupSet<T> {
    into_iter_fn!();
}

impl<'a, T> IntoIterator for &'a SparseScalarLookupSet<T> {
    into_iter_ref_fn!();
}

impl<T, ST> PartialEq<ST> for SparseScalarLookupSet<T>
where
    T: Scalar,
    ST: Set<T>,
{
    partial_eq_fn!();
}

impl<T> Eq for SparseScalarLookupSet<T> where T: Scalar {}

impl<T> Debug for SparseScalarLookupSet<T>
where
    T: Debug,
{
    debug_fn!();
}

#[cfg(feature = "serde")]
impl<T> Serialize for SparseScalarLookupSet<T>
where
    T: Serialize,
{
    serialize_fn!();
}
