use crate::maps::decl_macros::{
    common_primary_funcs, debug_trait_funcs, get_disjoint_mut_funcs, index_trait_funcs, into_iterator_trait_funcs,
    into_iterator_trait_mut_ref_funcs, into_iterator_trait_ref_funcs, len_trait_funcs, map_extras_trait_funcs, map_iteration_trait_funcs,
    map_query_trait_funcs, partial_eq_trait_funcs, sparse_scalar_lookup_primary_funcs,
};
use crate::maps::{IntoIter, IntoKeys, IntoValues, Iter, IterMut, Keys, Values, ValuesMut};
use crate::traits::{CollectionMagnitude, Len, Map, MapExtras, MapIteration, MapQuery, Scalar, SmallCollection};
use core::fmt::{Debug, Formatter, Result};
use core::ops::Index;
use equivalent::Comparable;

#[cfg(feature = "serde")]
use {
    crate::maps::decl_macros::serialize_trait_funcs,
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
pub struct InlineSparseScalarLookupMap<K, V, const SZ: usize, const LTSZ: usize, CM = SmallCollection> {
    min: usize,
    max: usize,
    lookup: [CM; LTSZ],
    entries: [(K, V); SZ],
}

impl<K, V, const SZ: usize, const LTSZ: usize, CM> InlineSparseScalarLookupMap<K, V, SZ, LTSZ, CM>
where
    CM: CollectionMagnitude,
{
    /// Creates a frozen map.
    #[must_use]
    pub const fn new_raw(sorted_and_dedupped_entries: [(K, V); SZ], lookup: [CM; LTSZ], min: usize, max: usize) -> Self {
        Self {
            min,
            max,
            lookup,
            entries: sorted_and_dedupped_entries,
        }
    }

    sparse_scalar_lookup_primary_funcs!();
    common_primary_funcs!(const_len, entries);
}

impl<K, V, Q, const SZ: usize, const LTSZ: usize, CM> Map<K, V, Q> for InlineSparseScalarLookupMap<K, V, SZ, LTSZ, CM>
where
    CM: CollectionMagnitude,
    Q: Scalar + Comparable<K>,
{
}

impl<K, V, Q, const SZ: usize, const LTSZ: usize, CM> MapExtras<K, V, Q> for InlineSparseScalarLookupMap<K, V, SZ, LTSZ, CM>
where
    CM: CollectionMagnitude,
    Q: Scalar + Comparable<K>,
{
    map_extras_trait_funcs!();
}

impl<K, V, Q, const SZ: usize, const LTSZ: usize, CM> MapQuery<Q, V> for InlineSparseScalarLookupMap<K, V, SZ, LTSZ, CM>
where
    CM: CollectionMagnitude,
    Q: Scalar + Comparable<K>,
{
    map_query_trait_funcs!();
}

impl<K, V, const SZ: usize, const LTSZ: usize, CM> MapIteration<K, V> for InlineSparseScalarLookupMap<K, V, SZ, LTSZ, CM>
where
    CM: CollectionMagnitude,
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

    map_iteration_trait_funcs!();
}

impl<K, V, const SZ: usize, const LTSZ: usize, CM> Len for InlineSparseScalarLookupMap<K, V, SZ, LTSZ, CM>
where
    CM: CollectionMagnitude,
{
    len_trait_funcs!();
}

impl<K, V, Q, const SZ: usize, const LTSZ: usize, CM> Index<&Q> for InlineSparseScalarLookupMap<K, V, SZ, LTSZ, CM>
where
    Q: Comparable<K> + Scalar,
    CM: CollectionMagnitude,
{
    index_trait_funcs!();
}

impl<K, V, const SZ: usize, const LTSZ: usize, CM> IntoIterator for InlineSparseScalarLookupMap<K, V, SZ, LTSZ, CM>
where
    CM: CollectionMagnitude,
{
    into_iterator_trait_funcs!();
}

impl<'a, K, V, const SZ: usize, const LTSZ: usize, CM> IntoIterator for &'a InlineSparseScalarLookupMap<K, V, SZ, LTSZ, CM>
where
    CM: CollectionMagnitude,
{
    into_iterator_trait_ref_funcs!();
}

impl<'a, K, V, const SZ: usize, const LTSZ: usize, CM> IntoIterator for &'a mut InlineSparseScalarLookupMap<K, V, SZ, LTSZ, CM>
where
    CM: CollectionMagnitude,
{
    into_iterator_trait_mut_ref_funcs!();
}

impl<K, V, MT, const SZ: usize, const LTSZ: usize, CM> PartialEq<MT> for InlineSparseScalarLookupMap<K, V, SZ, LTSZ, CM>
where
    K: Scalar,
    V: PartialEq,
    MT: MapQuery<K, V>,
    CM: CollectionMagnitude,
{
    partial_eq_trait_funcs!();
}

impl<K, V, const SZ: usize, const LTSZ: usize, CM> Eq for InlineSparseScalarLookupMap<K, V, SZ, LTSZ, CM>
where
    K: Scalar,
    V: Eq,
    CM: CollectionMagnitude,
{
}

impl<K, V, const SZ: usize, const LTSZ: usize, CM> Debug for InlineSparseScalarLookupMap<K, V, SZ, LTSZ, CM>
where
    K: Debug,
    V: Debug,
    CM: CollectionMagnitude,
{
    debug_trait_funcs!();
}

#[cfg(feature = "serde")]
impl<K, V, const SZ: usize, const LTSZ: usize, CM> Serialize for InlineSparseScalarLookupMap<K, V, SZ, LTSZ, CM>
where
    K: Serialize,
    V: Serialize,
    CM: CollectionMagnitude,
{
    serialize_trait_funcs!();
}
