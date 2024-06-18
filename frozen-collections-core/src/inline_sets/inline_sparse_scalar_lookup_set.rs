use core::borrow::Borrow;
use core::fmt::Debug;
use core::hash::Hash;
use core::ops::{BitAnd, BitOr, BitXor, Sub};

use crate::inline_maps::InlineSparseScalarLookupMap;
use crate::sets::decl_macros::{
    bitand_fn, bitor_fn, bitxor_fn, contains_fn, debug_fn, get_fn, into_iter_fn, into_iter_ref_fn,
    partial_eq_fn, set_boilerplate, set_iterator_boilerplate, sub_fn,
};
use crate::sets::{IntoIter, Iter};
use crate::traits::{CollectionMagnitude, Len, MapIterator, Scalar, Set, SetIterator};

/// A set whose values are scalars.
///
/// # Type Parameters
///
/// - `T`: The value type.
/// - `CM`: The magnitude of the set, one of [`SmallCollection`](crate::traits::SmallCollection), [`MediumCollection`](crate::traits::MediumCollection), or [`LargeCollection`](crate::traits::LargeCollection).
/// - `SZ`: The number of entries in the set.
/// - `LTSZ`: The number of entries in the lookup table.
///
/// # Compatibility Note
///
/// This type is an implementation detail of the `frozen-collections` crate.
/// This API is therefore not stable and may change at any time.
#[derive(Clone)]
pub struct InlineSparseScalarLookupSet<T, CM, const SZ: usize, const LTSZ: usize> {
    map: InlineSparseScalarLookupMap<T, (), CM, SZ, LTSZ>,
}

impl<T, CM, const SZ: usize, const LTSZ: usize> InlineSparseScalarLookupSet<T, CM, SZ, LTSZ> {
    pub const fn new_raw(map: InlineSparseScalarLookupMap<T, (), CM, SZ, LTSZ>) -> Self {
        Self { map }
    }
}

impl<T, CM, const SZ: usize, const LTSZ: usize> InlineSparseScalarLookupSet<T, CM, SZ, LTSZ>
where
    CM: CollectionMagnitude,
{
    get_fn!(Scalar);
    contains_fn!(Scalar);
}

impl<T, CM, const SZ: usize, const LTSZ: usize> Len
    for InlineSparseScalarLookupSet<T, CM, SZ, LTSZ>
{
    fn len(&self) -> usize {
        SZ
    }
}

impl<T, CM, const SZ: usize, const LTSZ: usize> Debug
    for InlineSparseScalarLookupSet<T, CM, SZ, LTSZ>
where
    T: Debug,
{
    debug_fn!();
}

impl<T, CM, const SZ: usize, const LTSZ: usize> IntoIterator
    for InlineSparseScalarLookupSet<T, CM, SZ, LTSZ>
{
    into_iter_fn!();
}

impl<'a, T, CM, const SZ: usize, const LTSZ: usize> IntoIterator
    for &'a InlineSparseScalarLookupSet<T, CM, SZ, LTSZ>
{
    into_iter_ref_fn!();
}

impl<T, CM, const SZ: usize, const LTSZ: usize> SetIterator<T>
    for InlineSparseScalarLookupSet<T, CM, SZ, LTSZ>
{
    type Iterator<'a>
        = Iter<'a, T>
    where
        T: 'a,
        CM: 'a;

    set_iterator_boilerplate!();
}

impl<T, CM, const SZ: usize, const LTSZ: usize> Set<T>
    for InlineSparseScalarLookupSet<T, CM, SZ, LTSZ>
where
    T: Scalar,
    CM: CollectionMagnitude,
{
    set_boilerplate!();
}

impl<T, CM, ST, const SZ: usize, const LTSZ: usize> BitOr<&ST>
    for &InlineSparseScalarLookupSet<T, CM, SZ, LTSZ>
where
    T: Scalar + Hash,
    ST: Set<T>,
    CM: CollectionMagnitude,
{
    bitor_fn!(RandomState);
}

impl<T, CM, ST, const SZ: usize, const LTSZ: usize> BitAnd<&ST>
    for &InlineSparseScalarLookupSet<T, CM, SZ, LTSZ>
where
    T: Scalar + Hash,
    ST: Set<T>,
    CM: CollectionMagnitude,
{
    bitand_fn!(RandomState);
}

impl<T, CM, ST, const SZ: usize, const LTSZ: usize> BitXor<&ST>
    for &InlineSparseScalarLookupSet<T, CM, SZ, LTSZ>
where
    T: Scalar + Hash,
    ST: Set<T>,
    CM: CollectionMagnitude,
{
    bitxor_fn!(RandomState);
}

impl<T, CM, ST, const SZ: usize, const LTSZ: usize> Sub<&ST>
    for &InlineSparseScalarLookupSet<T, CM, SZ, LTSZ>
where
    T: Scalar + Hash,
    ST: Set<T>,
    CM: CollectionMagnitude,
{
    sub_fn!(RandomState);
}

impl<T, CM, ST, const SZ: usize, const LTSZ: usize> PartialEq<ST>
    for InlineSparseScalarLookupSet<T, CM, SZ, LTSZ>
where
    T: Scalar,
    ST: Set<T>,
    CM: CollectionMagnitude,
{
    partial_eq_fn!();
}

impl<T, CM, const SZ: usize, const LTSZ: usize> Eq for InlineSparseScalarLookupSet<T, CM, SZ, LTSZ>
where
    T: Scalar,
    CM: CollectionMagnitude,
{
}
