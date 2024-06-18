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
/// # Type Parameters
///
/// - `K`: The key type.
/// - `V`: The value type.
/// - `SZ`: The number of entries in the map.
///
/// # Compatibility Note
///
/// This type is an implementation detail of the `frozen-collections` crate.
/// This API is therefore not stable and may change at any time.
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
    #[allow(clippy::missing_errors_doc)]
    pub fn new(mut entries: Vec<(K, V)>) -> core::result::Result<Self, String> {
        entries.sort_by_key(|x| x.0);
        dedup_by_keep_last(&mut entries, |x, y| x.0.eq(&y.0));

        if SZ == 0 {
            let len = entries.len();
            return Ok(Self::new_raw(
                entries
                    .try_into()
                    .map_err(|_| format!("incorrect number of entries, expected 0, got {len}"))?,
                1,
                0,
            ));
        }

        let min = entries[0].0.index();
        let max = entries[entries.len() - 1].0.index();

        if entries.len() == max - min + 1 {
            let len = entries.len();
            Ok(Self::new_raw(
                entries
                    .try_into()
                    .map_err(|_| format!("Expected {SZ} entries, got {len}"))?,
                min,
                max,
            ))
        } else {
            Err("keys must be in a contiguous range <= usize::MAX in size".to_string())
        }
    }

    pub const fn new_raw(processed_entries: [(K, V); SZ], min: usize, max: usize) -> Self {
        Self {
            min,
            max,
            entries: processed_entries,
        }
    }
}

impl<K, V, const SZ: usize> InlineDenseScalarLookupMap<K, V, SZ> {
    dense_scalar_lookup_core!();
}

impl<K, V, const SZ: usize> Len for InlineDenseScalarLookupMap<K, V, SZ> {
    fn len(&self) -> usize {
        SZ
    }
}

impl<K, V, const SZ: usize> Debug for InlineDenseScalarLookupMap<K, V, SZ>
where
    K: Debug,
    V: Debug,
{
    debug_fn!();
}

impl<Q, K, V, const SZ: usize> Index<&Q> for InlineDenseScalarLookupMap<K, V, SZ>
where
    K: Borrow<Q>,
    Q: Scalar,
{
    index_fn!();
}

impl<K, V, const SZ: usize> IntoIterator for InlineDenseScalarLookupMap<K, V, SZ> {
    into_iter_fn_for_slice!(entries);
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

impl<K, V, const SZ: usize> MapIterator<K, V> for InlineDenseScalarLookupMap<K, V, SZ> {
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

impl<K, V, const SZ: usize> Map<K, V> for InlineDenseScalarLookupMap<K, V, SZ>
where
    K: Scalar,
{
    map_boilerplate_for_slice!(entries);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_empty_map() {
        let empty_map: InlineDenseScalarLookupMap<u32, u32, 0> =
            InlineDenseScalarLookupMap::new(Vec::new()).unwrap();
        assert_eq!(empty_map.len(), 0);
    }
}
