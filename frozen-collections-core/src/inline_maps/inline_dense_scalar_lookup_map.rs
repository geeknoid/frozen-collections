use crate::maps::decl_macros::{
    debug_fn, dense_scalar_lookup_query_funcs, get_disjoint_mut_fn,
    get_disjoint_unchecked_mut_body, get_disjoint_unchecked_mut_fn, index_fn, into_iter_fn,
    into_iter_mut_ref_fn, into_iter_ref_fn, map_iteration_funcs, partial_eq_fn,
};
use crate::maps::{IntoIter, IntoKeys, IntoValues, Iter, IterMut, Keys, Values, ValuesMut};
use crate::traits::{Len, Map, MapIteration, MapQuery, Scalar};
use alloc::vec::Vec;
use core::fmt::{Debug, Formatter, Result};
use core::ops::Index;

#[cfg(feature = "serde")]
use {
    crate::maps::decl_macros::serialize_fn,
    serde::ser::SerializeMap,
    serde::{Serialize, Serializer},
};

/// A map whose keys are a continuous range in a sequence of scalar values.
///
#[doc = include_str!("../doc_snippets/private_api_warning.md")]
#[doc = include_str!("../doc_snippets/about.md")]
///
/// # Type Parameters
///
/// - `K`: The key type.
/// - `V`: The value type.
/// - `SZ`: The number of entries in the map.
#[derive(Clone)]
pub struct InlineDenseScalarLookupMap<K, V, const SZ: usize> {
    min: usize,
    max: usize,
    entries: [(K, V); SZ],
}

impl<K, V, const SZ: usize> InlineDenseScalarLookupMap<K, V, SZ>
where
    K: Scalar,
{
    /// Creates a frozen map.
    ///
    /// This function assumes that `min` <= `max` and that the vector is sorted according to the
    /// order of the [`Ord`] trait.
    #[must_use]
    pub const fn new_raw(processed_entries: [(K, V); SZ], min: usize, max: usize) -> Self {
        Self {
            min,
            max,
            entries: processed_entries,
        }
    }
}

impl<K, V, const SZ: usize> Map<K, V, K> for InlineDenseScalarLookupMap<K, V, SZ>
where
    K: Scalar,
{
    get_disjoint_mut_fn!("Scalar");
    get_disjoint_unchecked_mut_fn!("Scalar");
}

impl<K, V, const SZ: usize> MapQuery<K, V, K> for InlineDenseScalarLookupMap<K, V, SZ>
where
    K: Scalar,
{
    dense_scalar_lookup_query_funcs!();
}

impl<K, V, const SZ: usize> MapIteration<K, V> for InlineDenseScalarLookupMap<K, V, SZ> {
    type Iterator<'a>
        = Iter<'a, K, V>
    where
        K: 'a,
        V: 'a;

    type KeyIterator<'a>
        = Keys<'a, K, V>
    where
        K: 'a,
        V: 'a;

    type ValueIterator<'a>
        = Values<'a, K, V>
    where
        K: 'a,
        V: 'a;

    type MutIterator<'a>
        = IterMut<'a, K, V>
    where
        K: 'a,
        V: 'a;

    type ValueMutIterator<'a>
        = ValuesMut<'a, K, V>
    where
        K: 'a,
        V: 'a;

    map_iteration_funcs!(entries);
}

impl<K, V, const SZ: usize> Len for InlineDenseScalarLookupMap<K, V, SZ> {
    fn len(&self) -> usize {
        SZ
    }
}

impl<Q, V, const SZ: usize> Index<&Q> for InlineDenseScalarLookupMap<Q, V, SZ>
where
    Q: Scalar,
{
    index_fn!();
}

impl<K, V, const SZ: usize> IntoIterator for InlineDenseScalarLookupMap<K, V, SZ> {
    into_iter_fn!(entries);
}

impl<'a, K, V, const SZ: usize> IntoIterator for &'a InlineDenseScalarLookupMap<K, V, SZ> {
    into_iter_ref_fn!();
}

impl<'a, K, V, const SZ: usize> IntoIterator for &'a mut InlineDenseScalarLookupMap<K, V, SZ> {
    into_iter_mut_ref_fn!();
}

impl<K, V, MT, const SZ: usize> PartialEq<MT> for InlineDenseScalarLookupMap<K, V, SZ>
where
    K: Scalar,
    V: PartialEq,
    MT: Map<K, V>,
{
    partial_eq_fn!();
}

impl<K, V, const SZ: usize> Eq for InlineDenseScalarLookupMap<K, V, SZ>
where
    K: Scalar,
    V: Eq,
{
}

impl<K, V, const SZ: usize> Debug for InlineDenseScalarLookupMap<K, V, SZ>
where
    K: Debug,
    V: Debug,
{
    debug_fn!();
}

#[cfg(feature = "serde")]
impl<K, V, const SZ: usize> Serialize for InlineDenseScalarLookupMap<K, V, SZ>
where
    K: Serialize,
    V: Serialize,
{
    serialize_fn!();
}
