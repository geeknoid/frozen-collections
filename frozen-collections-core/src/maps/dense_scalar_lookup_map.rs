use alloc::boxed::Box;
use alloc::string::{String, ToString};
use alloc::vec::Vec;
use core::borrow::Borrow;
use core::fmt::{Debug, Formatter, Result};
use core::ops::Index;

use crate::maps::decl_macros::{
    contains_key_fn, debug_fn, dense_scalar_lookup_core, get_many_mut_body, get_many_mut_fn,
    index_fn, into_iter_fn_for_slice, into_iter_mut_ref_fn, into_iter_ref_fn,
    map_boilerplate_for_slice, map_iterator_boilerplate_for_slice, partial_eq_fn,
};
use crate::maps::{IntoIter, IntoKeys, IntoValues, Iter, IterMut, Keys, Values, ValuesMut};
use crate::traits::{Len, Map, MapIterator, Scalar};
use crate::utils::dedup_by_keep_last;

/// A map whose keys are a continuous range in a sequence of scalar values.
///
#[doc = include_str!("../doc_snippets/type_compat_warning.md")]
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
    /// Fails if the all the keys in the input vector, after sorting and dedupping,
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

impl<K, V> DenseScalarLookupMap<K, V> {
    dense_scalar_lookup_core!();
}

impl<K, V> Len for DenseScalarLookupMap<K, V> {
    fn len(&self) -> usize {
        self.entries.len()
    }
}

impl<K, V> Debug for DenseScalarLookupMap<K, V>
where
    K: Debug,
    V: Debug,
{
    debug_fn!();
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

impl<Q, K, V> Index<&Q> for DenseScalarLookupMap<K, V>
where
    K: Borrow<Q>,
    Q: Scalar,
{
    index_fn!();
}

impl<K, V> IntoIterator for DenseScalarLookupMap<K, V> {
    into_iter_fn_for_slice!(entries);
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

impl<K, V> MapIterator<K, V> for DenseScalarLookupMap<K, V> {
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

    map_iterator_boilerplate_for_slice!(entries);
}

impl<K, V> Map<K, V> for DenseScalarLookupMap<K, V>
where
    K: Scalar,
{
    map_boilerplate_for_slice!(entries);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn error_in_new() {
        let map = DenseScalarLookupMap::<u8, u8>::new(vec![(1, 1), (2, 2), (4, 3)]);
        assert_eq!(
            map,
            Err("keys must be in a contiguous range <= usize::MAX in size".to_string())
        );
    }
}
