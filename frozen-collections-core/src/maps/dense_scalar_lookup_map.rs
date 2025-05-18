use crate::maps::decl_macros::{
    debug_fn, dense_scalar_lookup_query_funcs, get_disjoint_mut_fn, get_disjoint_unchecked_mut_body, get_disjoint_unchecked_mut_fn,
    index_fn, into_iter_fn, into_iter_mut_ref_fn, into_iter_ref_fn, map_iteration_funcs, partial_eq_fn,
};
use crate::maps::{IntoIter, IntoKeys, IntoValues, Iter, IterMut, Keys, Values, ValuesMut};
use crate::traits::{Len, Map, MapIteration, MapQuery, Scalar};
use crate::utils::dedup_by_keep_last;
use alloc::boxed::Box;
use alloc::string::{String, ToString};
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
#[derive(Clone)]
pub struct DenseScalarLookupMap<K, V> {
    min: usize,
    max: usize,
    entries: Box<[(K, V)]>,
}

impl<K, V> DenseScalarLookupMap<K, V>
where
    K: Scalar,
{
    /// Creates a frozen map.
    ///
    /// # Errors
    ///
    /// Fails if all the keys in the input vector, after sorting and dedupping,
    /// don't represent a continuous range of values.
    pub fn new(mut entries: Vec<(K, V)>) -> core::result::Result<Self, String> {
        entries.sort_by_key(|x| x.0);
        dedup_by_keep_last(&mut entries, |x, y| x.0.eq(&y.0));

        if entries.is_empty() {
            return Ok(Self::default());
        }

        let min = entries[0].0.index();
        let max = entries[entries.len() - 1].0.index();

        if entries.len() == max - min + 1 {
            Ok(Self::new_raw(entries))
        } else {
            Err("keys must be in a contiguous range <= usize::MAX in size".to_string())
        }
    }

    /// Creates a new frozen map.
    ///
    /// This function assumes that `min` <= `max` and that the vector is sorted according to the
    /// order of the [`Ord`] trait.
    #[must_use]
    pub(crate) fn new_raw(processed_entries: Vec<(K, V)>) -> Self {
        Self {
            min: processed_entries[0].0.index(),
            max: processed_entries[processed_entries.len() - 1].0.index(),
            entries: processed_entries.into_boxed_slice(),
        }
    }
}

impl<K, V> Default for DenseScalarLookupMap<K, V> {
    fn default() -> Self {
        Self {
            min: 1,
            max: 0,
            entries: Box::new([]),
        }
    }
}

impl<K, V> Map<K, V, K> for DenseScalarLookupMap<K, V>
where
    K: Scalar,
{
    get_disjoint_mut_fn!("Scalar");
    get_disjoint_unchecked_mut_fn!("Scalar");
}

impl<K, V> MapQuery<K, V, K> for DenseScalarLookupMap<K, V>
where
    K: Scalar,
{
    dense_scalar_lookup_query_funcs!();
}

impl<K, V> MapIteration<K, V> for DenseScalarLookupMap<K, V> {
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

impl<K, V> Len for DenseScalarLookupMap<K, V> {
    fn len(&self) -> usize {
        self.entries.len()
    }
}

impl<Q, V> Index<&Q> for DenseScalarLookupMap<Q, V>
where
    Q: Scalar,
{
    index_fn!();
}

impl<K, V> IntoIterator for DenseScalarLookupMap<K, V> {
    into_iter_fn!(entries);
}

impl<'a, K, V> IntoIterator for &'a DenseScalarLookupMap<K, V> {
    into_iter_ref_fn!();
}

impl<'a, K, V> IntoIterator for &'a mut DenseScalarLookupMap<K, V> {
    into_iter_mut_ref_fn!();
}

impl<K, V, MT> PartialEq<MT> for DenseScalarLookupMap<K, V>
where
    K: Scalar,
    V: PartialEq,
    MT: Map<K, V>,
{
    partial_eq_fn!();
}

impl<K, V> Eq for DenseScalarLookupMap<K, V>
where
    K: Scalar,
    V: Eq,
{
}

impl<K, V> Debug for DenseScalarLookupMap<K, V>
where
    K: Debug,
    V: Debug,
{
    debug_fn!();
}

#[cfg(feature = "serde")]
impl<K, V> Serialize for DenseScalarLookupMap<K, V>
where
    K: Serialize,
    V: Serialize,
{
    serialize_fn!();
}

#[cfg(test)]
mod tests {
    use super::*;
    use alloc::vec;

    #[test]
    fn error_in_new() {
        let map = DenseScalarLookupMap::<u8, u8>::new(vec![(1, 1), (2, 2), (4, 3)]);
        assert_eq!(map, Err("keys must be in a contiguous range <= usize::MAX in size".to_string()));
    }
}
