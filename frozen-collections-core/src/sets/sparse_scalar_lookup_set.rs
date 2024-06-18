use crate::maps::SparseScalarLookupMap;
use crate::sets::decl_macros::{
    bitand_fn, bitor_fn, bitxor_fn, contains_fn, debug_fn, get_fn, into_iter_fn, into_iter_ref_fn,
    partial_eq_fn, set_boilerplate, set_iterator_boilerplate, sub_fn,
};
use crate::sets::{IntoIter, Iter};
use crate::traits::{
    CollectionMagnitude, Len, MapIterator, Scalar, Set, SetIterator, SmallCollection,
};
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
pub struct SparseScalarLookupSet<T, CM = SmallCollection> {
    map: SparseScalarLookupMap<T, (), CM>,
}

impl<T, CM> SparseScalarLookupSet<T, CM>
where
    T: Scalar,
    CM: CollectionMagnitude,
    <CM as TryFrom<usize>>::Error: Debug,
{
    /// Creates a frozen set.
    ///
    /// # Errors
    ///
    /// Fails if the number of entries in the vector, after deduplication, exceeds the
    /// magnitude of the collection as specified by the `CM` generic argument.
    #[must_use]
    pub const fn new(map: SparseScalarLookupMap<T, (), CM>) -> Self {
        Self { map }
    }
}

impl<T, CM> SparseScalarLookupSet<T, CM>
where
    CM: CollectionMagnitude,
{
    get_fn!(Scalar);
    contains_fn!(Scalar);
}

impl<T, CM> Len for SparseScalarLookupSet<T, CM> {
    fn len(&self) -> usize {
        self.map.len()
    }
}

impl<T, CM> Debug for SparseScalarLookupSet<T, CM>
where
    T: Debug,
{
    debug_fn!();
}

impl<T, CM> Default for SparseScalarLookupSet<T, CM>
where
    T: Scalar,
{
    fn default() -> Self {
        Self {
            map: SparseScalarLookupMap::default(),
        }
    }
}

impl<T, CM> IntoIterator for SparseScalarLookupSet<T, CM> {
    into_iter_fn!();
}

impl<'a, T, CM> IntoIterator for &'a SparseScalarLookupSet<T, CM>
where
    T: Scalar,
    CM: CollectionMagnitude,
{
    into_iter_ref_fn!();
}

impl<T, CM> SetIterator<T> for SparseScalarLookupSet<T, CM> {
    type Iterator<'a>
        = Iter<'a, T>
    where
        T: 'a,
        CM: 'a;

    set_iterator_boilerplate!();
}

impl<T, CM> Set<T> for SparseScalarLookupSet<T, CM>
where
    T: Scalar,
    CM: CollectionMagnitude,
{
    set_boilerplate!();
}

impl<T, ST, CM> BitOr<&ST> for &SparseScalarLookupSet<T, CM>
where
    T: Scalar + Hash,
    ST: Set<T>,
    CM: CollectionMagnitude,
{
    bitor_fn!(RandomState);
}

impl<T, ST, CM> BitAnd<&ST> for &SparseScalarLookupSet<T, CM>
where
    T: Scalar + Hash,
    ST: Set<T>,
    CM: CollectionMagnitude,
{
    bitand_fn!(RandomState);
}

impl<T, ST, CM> BitXor<&ST> for &SparseScalarLookupSet<T, CM>
where
    T: Scalar + Hash,
    ST: Set<T>,
    CM: CollectionMagnitude,
{
    bitxor_fn!(RandomState);
}

impl<T, ST, CM> Sub<&ST> for &SparseScalarLookupSet<T, CM>
where
    T: Scalar + Hash,
    ST: Set<T>,
    CM: CollectionMagnitude,
{
    sub_fn!(RandomState);
}

impl<T, ST, CM> PartialEq<ST> for SparseScalarLookupSet<T, CM>
where
    T: Scalar,
    ST: Set<T>,
    CM: CollectionMagnitude,
{
    partial_eq_fn!();
}

impl<T, CM> Eq for SparseScalarLookupSet<T, CM>
where
    T: Scalar,
    CM: CollectionMagnitude,
{
}
