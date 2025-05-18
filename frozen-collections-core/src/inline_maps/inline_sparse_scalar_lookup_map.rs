use crate::maps::decl_macros::{
    debug_fn, get_disjoint_mut_fn, get_disjoint_unchecked_mut_body, get_disjoint_unchecked_mut_fn,
    index_fn, into_iter_fn, into_iter_mut_ref_fn, into_iter_ref_fn, map_iteration_funcs,
    partial_eq_fn, sparse_scalar_lookup_query_funcs,
};
use crate::maps::{IntoIter, IntoKeys, IntoValues, Iter, IterMut, Keys, Values, ValuesMut};
use crate::traits::{
    CollectionMagnitude, Len, Map, MapIteration, MapQuery, Scalar, SmallCollection,
};
use alloc::vec::Vec;
use core::fmt::{Debug, Formatter, Result};
use core::ops::Index;

#[cfg(feature = "serde")]
use {
    crate::maps::decl_macros::serialize_fn,
    serde::ser::SerializeMap,
    serde::{Serialize, Serializer},
};

/// A map whose keys are a sparse range of integers.
///
#[doc = include_str!("../doc_snippets/private_api_warning.md")]
#[doc = include_str!("../doc_snippets/about.md")]
///
/// # Type Parameters
///
/// - `K`: The key type.
/// - `V`: The value type.
/// - `CM`: The magnitude of the map, one of [`SmallCollection`](SmallCollection), [`MediumCollection`](crate::traits::MediumCollection), or [`LargeCollection`](crate::traits::LargeCollection).
/// - `SZ`: The number of entries in the map.
/// - `LTSZ`: The number of entries in the lookup table.
#[derive(Clone)]
pub struct InlineSparseScalarLookupMap<
    K,
    V,
    const SZ: usize,
    const LTSZ: usize,
    CM = SmallCollection,
> {
    min: usize,
    max: usize,
    lookup: [CM; LTSZ],
    entries: [(K, V); SZ],
}

impl<K, V, const SZ: usize, const LTSZ: usize, CM> InlineSparseScalarLookupMap<K, V, SZ, LTSZ, CM>
where
    K: Scalar,
    CM: CollectionMagnitude,
{
    /// Creates a frozen map.
    #[must_use]
    pub const fn new_raw(
        processed_entries: [(K, V); SZ],
        lookup: [CM; LTSZ],
        min: usize,
        max: usize,
    ) -> Self {
        Self {
            min,
            max,
            lookup,
            entries: processed_entries,
        }
    }
}

impl<K, V, const SZ: usize, const LTSZ: usize, CM> Map<K, V, K>
    for InlineSparseScalarLookupMap<K, V, SZ, LTSZ, CM>
where
    CM: CollectionMagnitude,
    K: Scalar,
{
    get_disjoint_mut_fn!("Scalar");
    get_disjoint_unchecked_mut_fn!("Scalar");
}

impl<K, V, const SZ: usize, const LTSZ: usize, CM> MapQuery<K, V, K>
    for InlineSparseScalarLookupMap<K, V, SZ, LTSZ, CM>
where
    CM: CollectionMagnitude,
    K: Scalar,
{
    sparse_scalar_lookup_query_funcs!();
}

impl<K, V, const SZ: usize, const LTSZ: usize, CM> MapIteration<K, V>
    for InlineSparseScalarLookupMap<K, V, SZ, LTSZ, CM>
{
    type Iterator<'a>
        = Iter<'a, K, V>
    where
        K: 'a,
        V: 'a,
        CM: 'a;

    type KeyIterator<'a>
        = Keys<'a, K, V>
    where
        K: 'a,
        V: 'a,
        CM: 'a;

    type ValueIterator<'a>
        = Values<'a, K, V>
    where
        K: 'a,
        V: 'a,
        CM: 'a;

    type MutIterator<'a>
        = IterMut<'a, K, V>
    where
        K: 'a,
        V: 'a,
        CM: 'a;

    type ValueMutIterator<'a>
        = ValuesMut<'a, K, V>
    where
        K: 'a,
        V: 'a,
        CM: 'a;

    map_iteration_funcs!(entries);
}

impl<K, V, const SZ: usize, const LTSZ: usize, CM> Len
    for InlineSparseScalarLookupMap<K, V, SZ, LTSZ, CM>
{
    fn len(&self) -> usize {
        SZ
    }
}

impl<Q, V, const SZ: usize, const LTSZ: usize, CM> Index<&Q>
    for InlineSparseScalarLookupMap<Q, V, SZ, LTSZ, CM>
where
    Q: Scalar,
    CM: CollectionMagnitude,
{
    index_fn!();
}

impl<K, V, const SZ: usize, const LTSZ: usize, CM> IntoIterator
    for InlineSparseScalarLookupMap<K, V, SZ, LTSZ, CM>
{
    into_iter_fn!(entries);
}

impl<'a, K, V, const SZ: usize, const LTSZ: usize, CM> IntoIterator
    for &'a InlineSparseScalarLookupMap<K, V, SZ, LTSZ, CM>
{
    into_iter_ref_fn!();
}

impl<'a, K, V, const SZ: usize, const LTSZ: usize, CM> IntoIterator
    for &'a mut InlineSparseScalarLookupMap<K, V, SZ, LTSZ, CM>
{
    into_iter_mut_ref_fn!();
}

impl<K, V, MT, const SZ: usize, const LTSZ: usize, CM> PartialEq<MT>
    for InlineSparseScalarLookupMap<K, V, SZ, LTSZ, CM>
where
    K: Scalar,
    V: PartialEq,
    MT: Map<K, V>,
    CM: CollectionMagnitude,
{
    partial_eq_fn!();
}

impl<K, V, const SZ: usize, const LTSZ: usize, CM> Eq
    for InlineSparseScalarLookupMap<K, V, SZ, LTSZ, CM>
where
    K: Scalar,
    V: Eq,
    CM: CollectionMagnitude,
{
}

impl<K, V, const SZ: usize, const LTSZ: usize, CM> Debug
    for InlineSparseScalarLookupMap<K, V, SZ, LTSZ, CM>
where
    K: Debug,
    V: Debug,
{
    debug_fn!();
}

#[cfg(feature = "serde")]
impl<K, V, const SZ: usize, const LTSZ: usize, CM> Serialize
    for InlineSparseScalarLookupMap<K, V, SZ, LTSZ, CM>
where
    K: Serialize,
    V: Serialize,
{
    serialize_fn!();
}
