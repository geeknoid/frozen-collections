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
/// # Compatibility Note
///
/// This type is an implementation detail of the `frozen-collections` crate.
/// This API is therefore not stable and may change at any time.
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
    #[allow(clippy::missing_errors_doc)]
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

    #[must_use]
    pub fn new_raw(processed_entries: Vec<(K, V)>) -> Self {
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

impl<K, V> Default for DenseScalarLookupMap<K, V>
where
    K: Scalar,
{
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
    use crate::traits::map_trait_tests::test_map_trait_impl;
    use std::collections::HashMap as StdHashMap;

    #[test]
    fn test_dense_scalar_lookup_map() {
        let map = DenseScalarLookupMap::new(vec![(1, 1), (2, 2), (3, 3)]).unwrap();
        let reference = StdHashMap::from([(1, 1), (2, 2), (3, 3)]);
        let other = StdHashMap::from([(1, 1), (2, 2), (3, 3), (4, 4)]);
        test_map_trait_impl(&map, &reference, &other);

        let map = DenseScalarLookupMap::new(vec![]).unwrap();
        let reference = StdHashMap::from([]);
        let other = StdHashMap::from([(1, 1), (2, 2), (3, 3), (4, 4)]);
        test_map_trait_impl(&map, &reference, &other);

        let map = DenseScalarLookupMap::new(vec![(1, 1), (2, 2), (3, 3), (1, 4)]).unwrap();
        let reference = StdHashMap::from([(2, 2), (3, 3), (1, 4)]);
        let other = StdHashMap::from([(1, 1), (2, 2), (3, 3), (4, 4)]);
        test_map_trait_impl(&map, &reference, &other);
    }

    #[test]
    fn test_error_in_new() {
        let map = DenseScalarLookupMap::<u8, u8>::new(vec![(1, 1), (2, 2), (4, 3)]);
        assert_eq!(
            map,
            Err("keys must be in a contiguous range <= usize::MAX in size".to_string())
        );
    }

    #[test]
    fn test_get_many_mut() {
        let mut map = DenseScalarLookupMap::new(vec![(1, 1), (2, 2), (3, 3)]).unwrap();

        let values = map.get_many_mut([&1, &2, &3]);
        assert_eq!(values, Some([&mut 1, &mut 2, &mut 3]));

        let values = map.get_many_mut([]);
        assert_eq!(values, Some([]));

        let values = map.get_many_mut([&1, &2, &3, &4]);
        assert_eq!(values, None);
    }

    #[test]
    fn test_into_iter() {
        let map = &DenseScalarLookupMap::new(vec![(1, 1), (2, 2), (3, 3)]).unwrap();
        let into_map: StdHashMap<_, _> = map.into_iter().map(|(k, v)| (*k, *v)).collect();
        assert_eq!(map, &into_map);

        let map = &mut DenseScalarLookupMap::new(vec![(1, 1), (2, 2), (3, 3)]).unwrap();
        let into_map: StdHashMap<_, _> = map.into_iter().map(|(k, v)| (*k, *v)).collect();
        assert_eq!(map, &into_map);
    }
}
