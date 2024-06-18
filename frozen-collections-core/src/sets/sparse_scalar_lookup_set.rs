use crate::maps::SparseScalarLookupMap;
use crate::sets::decl_macros::{
    bitand_fn, bitor_fn, bitxor_fn, contains_fn, debug_fn, get_fn, into_iter_fn, into_iter_ref_fn,
    partial_eq_fn, set_boilerplate, set_iterator_boilerplate, sub_fn,
};
use crate::sets::{IntoIter, Iter};
use crate::traits::{Len, MapIterator, Scalar, Set, SetIterator};
use core::borrow::Borrow;
use core::fmt::Debug;
use core::hash::Hash;
use core::ops::{BitAnd, BitOr, BitXor, Sub};

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

impl<T> SparseScalarLookupSet<T> {
    get_fn!(Scalar);
    contains_fn!(Scalar);
}

impl<T> Len for SparseScalarLookupSet<T> {
    fn len(&self) -> usize {
        self.map.len()
    }
}

impl<T> Debug for SparseScalarLookupSet<T>
where
    T: Debug,
{
    debug_fn!();
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

impl<T> IntoIterator for SparseScalarLookupSet<T> {
    into_iter_fn!();
}

impl<'a, T> IntoIterator for &'a SparseScalarLookupSet<T>
where
    T: Scalar,
{
    into_iter_ref_fn!();
}

impl<T> SetIterator<T> for SparseScalarLookupSet<T> {
    type Iterator<'a>
        = Iter<'a, T>
    where
        T: 'a;

    set_iterator_boilerplate!();
}

impl<T> Set<T> for SparseScalarLookupSet<T>
where
    T: Scalar,
{
    set_boilerplate!();
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

impl<T, ST> PartialEq<ST> for SparseScalarLookupSet<T>
where
    T: Scalar,
    ST: Set<T>,
{
    partial_eq_fn!();
}

impl<T> Eq for SparseScalarLookupSet<T> where T: Scalar {}
