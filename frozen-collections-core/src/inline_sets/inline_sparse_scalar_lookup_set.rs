use crate::inline_maps::InlineSparseScalarLookupMap;
use crate::sets::decl_macros::{
    bitand_trait_funcs, bitor_trait_funcs, bitxor_trait_funcs, common_primary_funcs, debug_trait_funcs, into_iterator_ref_trait_funcs,
    into_iterator_trait_funcs, partial_eq_trait_funcs, set_extras_trait_funcs, set_iteration_trait_funcs, set_query_trait_funcs,
    sparse_scalar_lookup_primary_funcs, sub_trait_funcs,
};
use crate::sets::{IntoIter, Iter};
use crate::traits::{CollectionMagnitude, Len, Scalar, Set, SetExtras, SetIteration, SetOps, SetQuery, SmallCollection};
use core::fmt::Debug;
use core::hash::Hash;
use core::ops::{BitAnd, BitOr, BitXor, Sub};
use equivalent::Comparable;

use crate::maps::decl_macros::len_trait_funcs;
#[cfg(feature = "serde")]
use {
    crate::sets::decl_macros::serialize_trait_funcs,
    serde::ser::SerializeSeq,
    serde::{Serialize, Serializer},
};

/// A set whose values are scalars.
///
#[doc = include_str!("../doc_snippets/private_api_warning.md")]
#[doc = include_str!("../doc_snippets/about.md")]
///
/// # Type Parameters
///
/// - `T`: The value type.
/// - `CM`: The magnitude of the set, one of [`SmallCollection`](SmallCollection), [`MediumCollection`](crate::traits::MediumCollection), or [`LargeCollection`](crate::traits::LargeCollection).
/// - `SZ`: The number of entries in the set.
/// - `LTSZ`: The number of entries in the lookup table.
#[derive(Clone)]
pub struct InlineSparseScalarLookupSet<T, const SZ: usize, const LTSZ: usize, CM = SmallCollection> {
    map: InlineSparseScalarLookupMap<T, (), SZ, LTSZ, CM>,
}

impl<T, const SZ: usize, const LTSZ: usize, CM> InlineSparseScalarLookupSet<T, SZ, LTSZ, CM>
where
    CM: CollectionMagnitude,
{
    /// Creates a frozen set.
    #[must_use]
    pub const fn new(map: InlineSparseScalarLookupMap<T, (), SZ, LTSZ, CM>) -> Self {
        Self { map }
    }

    sparse_scalar_lookup_primary_funcs!();
    common_primary_funcs!(const_len);
}

impl<T, Q, const SZ: usize, const LTSZ: usize, CM> Set<T, Q> for InlineSparseScalarLookupSet<T, SZ, LTSZ, CM>
where
    CM: CollectionMagnitude,
    Q: Comparable<T> + Scalar,
{
}

impl<T, Q, const SZ: usize, const LTSZ: usize, CM> SetExtras<T, Q> for InlineSparseScalarLookupSet<T, SZ, LTSZ, CM>
where
    CM: CollectionMagnitude,
    Q: Scalar + Comparable<T>,
{
    set_extras_trait_funcs!();
}

impl<T, Q, const SZ: usize, const LTSZ: usize, CM> SetQuery<Q> for InlineSparseScalarLookupSet<T, SZ, LTSZ, CM>
where
    CM: CollectionMagnitude,
    Q: Scalar + Comparable<T>,
{
    set_query_trait_funcs!();
}

impl<T, const SZ: usize, const LTSZ: usize, CM> SetIteration<T> for InlineSparseScalarLookupSet<T, SZ, LTSZ, CM>
where
    CM: CollectionMagnitude,
{
    type Iterator<'a>
        = Iter<'a, T>
    where
        T: 'a,
        CM: 'a;

    set_iteration_trait_funcs!();
}

impl<T, const SZ: usize, const LTSZ: usize, CM> Len for InlineSparseScalarLookupSet<T, SZ, LTSZ, CM>
where
    CM: CollectionMagnitude,
{
    len_trait_funcs!();
}

impl<T, ST, const SZ: usize, const LTSZ: usize, CM> BitOr<&ST> for &InlineSparseScalarLookupSet<T, SZ, LTSZ, CM>
where
    T: Scalar + Hash,
    ST: Set<T>,
    CM: CollectionMagnitude,
{
    bitor_trait_funcs!();
}

impl<T, ST, const SZ: usize, const LTSZ: usize, CM> BitAnd<&ST> for &InlineSparseScalarLookupSet<T, SZ, LTSZ, CM>
where
    T: Scalar + Hash,
    ST: Set<T>,
    CM: CollectionMagnitude,
{
    bitand_trait_funcs!();
}

impl<T, ST, const SZ: usize, const LTSZ: usize, CM> BitXor<&ST> for &InlineSparseScalarLookupSet<T, SZ, LTSZ, CM>
where
    T: Scalar + Hash,
    ST: Set<T>,
    CM: CollectionMagnitude,
{
    bitxor_trait_funcs!();
}

impl<T, ST, const SZ: usize, const LTSZ: usize, CM> Sub<&ST> for &InlineSparseScalarLookupSet<T, SZ, LTSZ, CM>
where
    T: Scalar + Hash,
    ST: Set<T>,
    CM: CollectionMagnitude,
{
    sub_trait_funcs!();
}

impl<T, const SZ: usize, const LTSZ: usize, CM> IntoIterator for InlineSparseScalarLookupSet<T, SZ, LTSZ, CM>
where
    CM: CollectionMagnitude,
{
    into_iterator_trait_funcs!();
}

impl<'a, T, const SZ: usize, const LTSZ: usize, CM> IntoIterator for &'a InlineSparseScalarLookupSet<T, SZ, LTSZ, CM>
where
    CM: CollectionMagnitude,
{
    into_iterator_ref_trait_funcs!();
}

impl<T, ST, const SZ: usize, const LTSZ: usize, CM> PartialEq<ST> for InlineSparseScalarLookupSet<T, SZ, LTSZ, CM>
where
    ST: SetQuery<T>,
    CM: CollectionMagnitude,
{
    partial_eq_trait_funcs!();
}

impl<T, const SZ: usize, const LTSZ: usize, CM> Eq for InlineSparseScalarLookupSet<T, SZ, LTSZ, CM>
where
    T: Scalar,
    CM: CollectionMagnitude,
{
}

impl<T, const SZ: usize, const LTSZ: usize, CM> Debug for InlineSparseScalarLookupSet<T, SZ, LTSZ, CM>
where
    T: Debug,
    CM: CollectionMagnitude,
{
    debug_trait_funcs!();
}

#[cfg(feature = "serde")]
impl<T, const SZ: usize, const LTSZ: usize, CM> Serialize for InlineSparseScalarLookupSet<T, SZ, LTSZ, CM>
where
    T: Serialize,
    CM: CollectionMagnitude,
{
    serialize_trait_funcs!();
}
