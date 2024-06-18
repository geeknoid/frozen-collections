use crate::maps::decl_macros::{
    contains_key_fn, debug_fn, get_many_mut_body, get_many_mut_fn, index_fn,
    into_iter_fn_for_slice, into_iter_mut_ref_fn, into_iter_ref_fn, map_boilerplate_for_slice,
    map_iterator_boilerplate_for_slice, partial_eq_fn, sparse_scalar_lookup_core,
};
use crate::maps::{IntoIter, IntoKeys, IntoValues, Iter, IterMut, Keys, Values, ValuesMut};
use crate::traits::{CollectionMagnitude, Len, Map, MapIterator, Scalar};
use crate::utils::dedup_by_keep_last;
use alloc::boxed::Box;
use alloc::string::String;
use alloc::vec::Vec;
use core::borrow::Borrow;
use core::fmt::{Debug, Formatter, Result};
use core::ops::Index;

/// A map whose keys are a sparse range of values from a scalar.
///
/// # Compatibility Note
///
/// This type is an implementation detail of the `frozen-collections` crate.
/// This API is therefore not stable and may change at any time.
#[derive(Clone)]
pub struct SparseScalarLookupMap<K, V, CM> {
    min: usize,
    max: usize,
    lookup: Box<[CM]>,
    entries: Box<[(K, V)]>,
}

impl<K, V, CM> SparseScalarLookupMap<K, V, CM>
where
    K: Scalar,
    CM: CollectionMagnitude,
    <CM as TryFrom<usize>>::Error: Debug,
{
    /// Creates a new `IntegerSparseLookupMap` from a list of entries.
    ///
    /// Note that this supports 1 less entry relative to the maximum capacity of the collection scale
    /// since 0 is used as a sentinel value within the lookup table.
    #[allow(clippy::missing_errors_doc)]
    pub fn new(mut entries: Vec<(K, V)>) -> core::result::Result<Self, String> {
        entries.sort_by_key(|x| x.0);
        dedup_by_keep_last(&mut entries, |x, y| x.0.eq(&y.0));

        if entries.is_empty() {
            return Ok(Self::default());
        }

        let min = entries[0].0.index();
        let max = entries[entries.len() - 1].0.index();

        if max - min + 1 >= CM::MAX_CAPACITY {
            Err("the range of keys is too large for the selected collection magnitude".to_string())
        } else {
            Ok(Self::new_raw(entries))
        }
    }

    #[must_use]
    #[allow(clippy::missing_panics_doc)]
    pub fn new_raw(processed_entries: Vec<(K, V)>) -> Self {
        let min = processed_entries[0].0.index();
        let max = processed_entries[processed_entries.len() - 1].0.index();
        let count = max - min + 1;

        let mut lookup = Vec::<CM>::with_capacity(count);
        lookup.resize(lookup.capacity(), CM::ZERO);

        for (i, entry) in processed_entries.iter().enumerate() {
            let index_in_lookup = entry.0.index() - min;
            let index_in_entries = CM::try_from(i + 1).expect("less than CM::MAX");
            lookup[index_in_lookup] = index_in_entries;
        }

        Self {
            min,
            max,
            lookup: lookup.into_boxed_slice(),
            entries: processed_entries.into_boxed_slice(),
        }
    }
}

impl<K, V, CM> SparseScalarLookupMap<K, V, CM>
where
    CM: CollectionMagnitude,
{
    sparse_scalar_lookup_core!();
}

impl<K, V, CM> Len for SparseScalarLookupMap<K, V, CM> {
    fn len(&self) -> usize {
        self.entries.len()
    }
}

impl<K, V, CM> Debug for SparseScalarLookupMap<K, V, CM>
where
    K: Debug,
    V: Debug,
{
    debug_fn!();
}

impl<K, V, CM> Default for SparseScalarLookupMap<K, V, CM> {
    fn default() -> Self {
        Self {
            min: 1,
            max: 0,
            lookup: Box::new([]),
            entries: Box::new([]),
        }
    }
}

impl<Q, K, V, CM> Index<&Q> for SparseScalarLookupMap<K, V, CM>
where
    K: Borrow<Q>,
    Q: Scalar,
    CM: CollectionMagnitude,
{
    index_fn!();
}

impl<K, V, CM> IntoIterator for SparseScalarLookupMap<K, V, CM> {
    into_iter_fn_for_slice!(entries);
}

impl<'a, K, V, CM> IntoIterator for &'a SparseScalarLookupMap<K, V, CM> {
    into_iter_ref_fn!();
}

impl<'a, K, V, CM> IntoIterator for &'a mut SparseScalarLookupMap<K, V, CM> {
    into_iter_mut_ref_fn!();
}

impl<K, V, MT, CM> PartialEq<MT> for SparseScalarLookupMap<K, V, CM>
where
    K: Scalar,
    V: PartialEq,
    MT: Map<K, V>,
    CM: CollectionMagnitude,
{
    partial_eq_fn!();
}

impl<K, V, CM> Eq for SparseScalarLookupMap<K, V, CM>
where
    K: Scalar,
    V: Eq,
    CM: CollectionMagnitude,
{
}

impl<K, V, CM> MapIterator<K, V> for SparseScalarLookupMap<K, V, CM> {
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

    map_iterator_boilerplate_for_slice!(entries);
}

impl<K, V, CM> Map<K, V> for SparseScalarLookupMap<K, V, CM>
where
    K: Scalar,
    CM: CollectionMagnitude,
{
    map_boilerplate_for_slice!(entries);
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::traits::map_trait_tests::test_map_trait_impl;
    use crate::traits::SmallCollection;
    use std::collections::HashMap as StdHashMap;

    #[test]
    fn test_sparse_scalar_lookup_map() {
        let map = SparseScalarLookupMap::<_, _, SmallCollection>::new(vec![(1, 1), (2, 2), (3, 3)])
            .unwrap();
        let reference = StdHashMap::from([(1, 1), (2, 2), (3, 3)]);
        let other = StdHashMap::from([(1, 1), (2, 2), (3, 3), (4, 4)]);
        test_map_trait_impl(&map, &reference, &other);

        let map = SparseScalarLookupMap::<_, _, SmallCollection>::new(vec![]).unwrap();
        let reference = StdHashMap::from([]);
        let other = StdHashMap::from([(1, 1), (2, 2), (3, 3), (4, 4)]);
        test_map_trait_impl(&map, &reference, &other);

        let map = SparseScalarLookupMap::<_, _, SmallCollection>::new(vec![
            (1, 1),
            (2, 2),
            (3, 3),
            (1, 4),
        ])
        .unwrap();
        let reference = StdHashMap::from([(2, 2), (3, 3), (1, 4)]);
        let other = StdHashMap::from([(1, 1), (2, 2), (3, 3), (4, 4)]);
        test_map_trait_impl(&map, &reference, &other);

        let map = SparseScalarLookupMap::<_, _, SmallCollection>::new(vec![
            (1, 1),
            (2, 2),
            (3, 3),
            (99, 4),
        ])
        .unwrap();
        let reference = StdHashMap::from([(1, 1), (2, 2), (3, 3), (99, 4)]);
        let other = StdHashMap::from([(1, 1), (2, 2), (3, 3), (77, 4)]);
        test_map_trait_impl(&map, &reference, &other);
    }

    #[test]
    fn test_error_in_new() {
        let map =
            SparseScalarLookupMap::<u32, u8, SmallCollection>::new(vec![(1, 1), (2, 2), (256, 3)]);
        assert_eq!(
            map,
            Err("the range of keys is too large for the selected collection magnitude".to_string())
        );
    }

    #[test]
    fn test_get_many_mut() {
        let mut map =
            SparseScalarLookupMap::<_, _, SmallCollection>::new(vec![(1, 1), (2, 2), (4, 4)])
                .unwrap();

        let values = map.get_many_mut([&1, &2, &4]);
        assert_eq!(values, Some([&mut 1, &mut 2, &mut 4]));

        let values = map.get_many_mut([]);
        assert_eq!(values, Some([]));

        let values = map.get_many_mut([&1, &2, &3, &4]);
        assert_eq!(values, None);
    }

    #[test]
    fn test_into_iter() {
        let map =
            &SparseScalarLookupMap::<_, _, SmallCollection>::new(vec![(1, 1), (2, 2), (3, 3)])
                .unwrap();
        let into_map: StdHashMap<_, _> = map.into_iter().map(|(k, v)| (*k, *v)).collect();
        assert_eq!(map, &into_map);

        let map =
            &mut SparseScalarLookupMap::<_, _, SmallCollection>::new(vec![(1, 1), (2, 2), (3, 3)])
                .unwrap();
        let into_map: StdHashMap<_, _> = map.into_iter().map(|(k, v)| (*k, *v)).collect();
        assert_eq!(map, &into_map);
    }
}
