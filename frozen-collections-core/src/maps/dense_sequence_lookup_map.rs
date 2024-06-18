use alloc::boxed::Box;
use alloc::string::{String, ToString};
use alloc::vec::Vec;
use core::borrow::Borrow;
use core::fmt::{Debug, Formatter, Result};
use core::ops::{Index, IndexMut};

use crate::maps::decl_macros::{
    contains_key_fn, debug_fn, dense_sequence_lookup_core, get_many_mut_body, get_many_mut_fn,
    index_fn, index_mut_fn, into_iter_fn_for_slice, into_iter_mut_ref_fn, into_iter_ref_fn,
    map_boilerplate_for_slice, map_iterator_boilerplate_for_slice, partial_eq_fn,
};
use crate::maps::{IntoIter, IntoKeys, IntoValues, Iter, IterMut, Keys, Values, ValuesMut};
use crate::traits::{Len, Map, MapIterator, Sequence};
use crate::utils::dedup_by_keep_last;

/// A map whose keys are a continuous range in a sequence.
///
/// # Compatibility Note
///
/// This type is an implementation detail of the `frozen-collections` crate.
/// This API is therefore not stable and may change at any time.
#[derive(Clone)]
pub struct DenseSequenceLookupMap<K, V> {
    min: K,
    max: K,
    entries: Box<[(K, V)]>,
}

impl<K, V> DenseSequenceLookupMap<K, V>
where
    K: Sequence,
{
    #[allow(clippy::missing_errors_doc)]
    pub fn new(mut entries: Vec<(K, V)>) -> core::result::Result<Self, String> {
        if entries.is_empty() {
            return Ok(Self::default());
        }

        entries.sort_by_key(|x| x.0);
        dedup_by_keep_last(&mut entries, |x, y| x.0.eq(&y.0));

        let min = entries[0].0;
        let max = entries[entries.len() - 1].0;

        if let Some(count) = K::count(&min, &max) {
            if count == entries.len() {
                return Ok(Self::new_internal(entries));
            }
        }

        Err("keys must be in a contiguous range <= usize::MAX in size".to_string())
    }

    #[must_use]
    #[allow(clippy::missing_errors_doc)]
    pub fn new_internal(entries: Vec<(K, V)>) -> Self {
        Self {
            min: entries[0].0,
            max: entries[entries.len() - 1].0,
            entries: entries.into_boxed_slice(),
        }
    }
}

impl<K, V> DenseSequenceLookupMap<K, V> {
    dense_sequence_lookup_core!();
}

impl<K, V> Len for DenseSequenceLookupMap<K, V> {
    fn len(&self) -> usize {
        self.entries.len()
    }
}

impl<K, V> Debug for DenseSequenceLookupMap<K, V>
where
    K: Debug,
    V: Debug,
{
    debug_fn!();
}

impl<K, V> Default for DenseSequenceLookupMap<K, V>
where
    K: Sequence,
{
    fn default() -> Self {
        Self {
            min: K::MAX,
            max: K::MIN,
            entries: Box::new([]),
        }
    }
}

impl<Q, K, V> Index<&Q> for DenseSequenceLookupMap<K, V>
where
    K: Borrow<Q>,
    Q: Sequence,
{
    index_fn!();
}

impl<Q, K, V> IndexMut<&Q> for DenseSequenceLookupMap<K, V>
where
    K: Borrow<Q>,
    Q: Sequence,
{
    index_mut_fn!();
}

impl<K, V> IntoIterator for DenseSequenceLookupMap<K, V> {
    into_iter_fn_for_slice!(entries);
}

impl<'a, K, V> IntoIterator for &'a DenseSequenceLookupMap<K, V> {
    into_iter_ref_fn!();
}

impl<'a, K, V> IntoIterator for &'a mut DenseSequenceLookupMap<K, V> {
    into_iter_mut_ref_fn!();
}

impl<K, V, MT> PartialEq<MT> for DenseSequenceLookupMap<K, V>
where
    K: Sequence,
    V: PartialEq,
    MT: Map<K, V>,
{
    partial_eq_fn!();
}

impl<K, V> Eq for DenseSequenceLookupMap<K, V>
where
    K: Sequence,
    V: Eq,
{
}

impl<K, V> MapIterator<K, V> for DenseSequenceLookupMap<K, V> {
    type Iterator<'a> = Iter<'a, K, V>
    where
        K: 'a,
        V: 'a;

    type KeyIterator<'a> = Keys<'a, K, V>
    where
        K: 'a,
        V: 'a;

    type ValueIterator<'a> = Values<'a, K, V>
    where
        K: 'a,
        V: 'a;

    type MutIterator<'a> = IterMut<'a, K, V>
    where
        K: 'a,
        V: 'a;

    type ValueMutIterator<'a> = ValuesMut<'a, K, V>
    where
        K: 'a,
        V: 'a;

    map_iterator_boilerplate_for_slice!(entries);
}

impl<K, V> Map<K, V> for DenseSequenceLookupMap<K, V>
where
    K: Sequence,
{
    map_boilerplate_for_slice!(entries);
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::maps::map_tests::test_map_trait_impl;
    use std::collections::HashMap as StdHashMap;

    #[test]
    fn test_dense_sequence_lookup_map() {
        let map = DenseSequenceLookupMap::new(vec![(1, 1), (2, 2), (3, 3)]).unwrap();
        let reference = StdHashMap::from([(1, 1), (2, 2), (3, 3)]);
        let other = StdHashMap::from([(1, 1), (2, 2), (3, 3), (4, 4)]);
        test_map_trait_impl(&map, &reference, &other);

        let map = DenseSequenceLookupMap::new(vec![]).unwrap();
        let reference = StdHashMap::from([]);
        let other = StdHashMap::from([(1, 1), (2, 2), (3, 3), (4, 4)]);
        test_map_trait_impl(&map, &reference, &other);

        let map = DenseSequenceLookupMap::new(vec![(1, 1), (2, 2), (3, 3), (1, 4)]).unwrap();
        let reference = StdHashMap::from([(2, 2), (3, 3), (1, 4)]);
        let other = StdHashMap::from([(1, 1), (2, 2), (3, 3), (4, 4)]);
        test_map_trait_impl(&map, &reference, &other);
    }

    #[test]
    fn test_error_in_new() {
        let map = DenseSequenceLookupMap::<u8, u8>::new(vec![(1, 1), (2, 2), (4, 3)]);
        assert_eq!(
            map,
            Err("keys must be in a contiguous range <= usize::MAX in size".to_string())
        );
    }

    #[test]
    fn test_get_many_mut() {
        let mut map = DenseSequenceLookupMap::new(vec![(1, 1), (2, 2), (3, 3)]).unwrap();

        let values = map.get_many_mut([&1, &2, &3]);
        assert_eq!(values, Some([&mut 1, &mut 2, &mut 3]));

        let values = map.get_many_mut([]);
        assert_eq!(values, Some([]));

        let values = map.get_many_mut([&1, &2, &3, &4]);
        assert_eq!(values, None);
    }

    #[test]
    fn test_into_iter() {
        let map = &DenseSequenceLookupMap::new(vec![(1, 1), (2, 2), (3, 3)]).unwrap();
        let into_map: StdHashMap<_, _> = map.into_iter().map(|(k, v)| (*k, *v)).collect();
        assert_eq!(map, &into_map);

        let map = &mut DenseSequenceLookupMap::new(vec![(1, 1), (2, 2), (3, 3)]).unwrap();
        let into_map: StdHashMap<_, _> = map.into_iter().map(|(k, v)| (*k, *v)).collect();
        assert_eq!(map, &into_map);
    }
}
