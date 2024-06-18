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
use crate::traits::{
    CollectionMagnitude, Len, MapIterator, Scalar, Set, SetIterator, SmallCollection,
};

/// A set whose values are scalars.
///
#[doc = include_str!("../doc_snippets/type_compat_warning.md")]
#[doc = include_str!("../doc_snippets/about.md")]
///
/// # Type Parameters
///
/// - `T`: The value type.
/// - `CM`: The magnitude of the set, one of [`SmallCollection`](crate::traits::SmallCollection), [`MediumCollection`](crate::traits::MediumCollection), or [`LargeCollection`](crate::traits::LargeCollection).
/// - `SZ`: The number of entries in the set.
/// - `LTSZ`: The number of entries in the lookup table.
#[derive(Clone)]
pub struct InlineSparseScalarLookupSet<T, const SZ: usize, const LTSZ: usize, CM = SmallCollection>
{
    map: InlineSparseScalarLookupMap<T, (), SZ, LTSZ, CM>,
}

impl<T, const SZ: usize, const LTSZ: usize, CM> InlineSparseScalarLookupSet<T, SZ, LTSZ, CM> {
    /// Creates a frozen set.
    #[must_use]
    pub const fn new(map: InlineSparseScalarLookupMap<T, (), SZ, LTSZ, CM>) -> Self {
        Self { map }
    }
}

impl<T, const SZ: usize, const LTSZ: usize, CM> InlineSparseScalarLookupSet<T, SZ, LTSZ, CM>
where
    CM: CollectionMagnitude,
{
    get_fn!(Scalar);
    contains_fn!(Scalar);
}

impl<T, const SZ: usize, const LTSZ: usize, CM> Len
    for InlineSparseScalarLookupSet<T, SZ, LTSZ, CM>
{
    fn len(&self) -> usize {
        SZ
    }
}

impl<T, const SZ: usize, const LTSZ: usize, CM> Debug
    for InlineSparseScalarLookupSet<T, SZ, LTSZ, CM>
where
    T: Debug,
{
    debug_fn!();
}

impl<T, const SZ: usize, const LTSZ: usize, CM> IntoIterator
    for InlineSparseScalarLookupSet<T, SZ, LTSZ, CM>
{
    into_iter_fn!();
}

impl<'a, T, const SZ: usize, const LTSZ: usize, CM> IntoIterator
    for &'a InlineSparseScalarLookupSet<T, SZ, LTSZ, CM>
{
    into_iter_ref_fn!();
}

impl<T, const SZ: usize, const LTSZ: usize, CM> SetIterator<T>
    for InlineSparseScalarLookupSet<T, SZ, LTSZ, CM>
{
    type Iterator<'a>
        = Iter<'a, T>
    where
        T: 'a,
        CM: 'a;

    set_iterator_boilerplate!();
}

impl<T, const SZ: usize, const LTSZ: usize, CM> Set<T>
    for InlineSparseScalarLookupSet<T, SZ, LTSZ, CM>
where
    T: Scalar,
    CM: CollectionMagnitude,
{
    set_boilerplate!();
}

impl<T, ST, const SZ: usize, const LTSZ: usize, CM> BitOr<&ST>
    for &InlineSparseScalarLookupSet<T, SZ, LTSZ, CM>
where
    T: Scalar + Hash,
    ST: Set<T>,
    CM: CollectionMagnitude,
{
    bitor_fn!(RandomState);
}

impl<T, ST, const SZ: usize, const LTSZ: usize, CM> BitAnd<&ST>
    for &InlineSparseScalarLookupSet<T, SZ, LTSZ, CM>
where
    T: Scalar + Hash,
    ST: Set<T>,
    CM: CollectionMagnitude,
{
    bitand_fn!(RandomState);
}

impl<T, ST, const SZ: usize, const LTSZ: usize, CM> BitXor<&ST>
    for &InlineSparseScalarLookupSet<T, SZ, LTSZ, CM>
where
    T: Scalar + Hash,
    ST: Set<T>,
    CM: CollectionMagnitude,
{
    bitxor_fn!(RandomState);
}

impl<T, ST, const SZ: usize, const LTSZ: usize, CM> Sub<&ST>
    for &InlineSparseScalarLookupSet<T, SZ, LTSZ, CM>
where
    T: Scalar + Hash,
    ST: Set<T>,
    CM: CollectionMagnitude,
{
    sub_fn!(RandomState);
}

impl<T, ST, const SZ: usize, const LTSZ: usize, CM> PartialEq<ST>
    for InlineSparseScalarLookupSet<T, SZ, LTSZ, CM>
where
    T: Scalar,
    ST: Set<T>,
    CM: CollectionMagnitude,
{
    partial_eq_fn!();
}

impl<T, const SZ: usize, const LTSZ: usize, CM> Eq for InlineSparseScalarLookupSet<T, SZ, LTSZ, CM>
where
    T: Scalar,
    CM: CollectionMagnitude,
{
}
