use alloc::boxed::Box;
use alloc::vec::Vec;
use core::borrow::Borrow;
use core::fmt::{Debug, Formatter, Result};
use core::ops::{Index, IndexMut};

use crate::maps::decl_macros::{
    contains_key_fn, debug_fn, get_many_mut_body, get_many_mut_fn, index_fn, index_mut_fn,
    into_iter_fn_for_slice, into_iter_mut_ref_fn, into_iter_ref_fn, map_boilerplate_for_slice,
    map_iterator_boilerplate_for_slice, partial_eq_fn, scan_core,
};
use crate::maps::{IntoIter, IntoKeys, IntoValues, Iter, IterMut, Keys, Values, ValuesMut};
use crate::traits::{Len, Map, MapIterator};
use crate::utils::slow_dedup_by_keep_last;

/// A general purpose map implemented using linear scanning.
///
/// # Compatibility Note
///
/// This type is an implementation detail of the `frozen-collections` crate.
/// This API is therefore not stable and may change at any time.
#[derive(Clone, Eq)]
pub struct ScanMap<K, V> {
    entries: Box<[(K, V)]>,
}

impl<K, V> ScanMap<K, V>
where
    K: Eq,
{
    #[must_use]
    pub fn new(mut entries: Vec<(K, V)>) -> Self {
        if entries.is_empty() {
            return Self::default();
        }

        slow_dedup_by_keep_last(&mut entries, |x, y| x.0.eq(&y.0));

        Self::new_internal(entries)
    }

    #[must_use]
    pub fn new_internal(entries: Vec<(K, V)>) -> Self {
        Self {
            entries: entries.into_boxed_slice(),
        }
    }
}

impl<K, V> ScanMap<K, V> {
    scan_core!();
}

impl<K, V> Len for ScanMap<K, V> {
    fn len(&self) -> usize {
        self.entries.len()
    }
}

impl<K, V> Debug for ScanMap<K, V>
where
    K: Debug,
    V: Debug,
{
    debug_fn!();
}

impl<K, V> Default for ScanMap<K, V> {
    fn default() -> Self {
        Self {
            entries: Box::default(),
        }
    }
}

impl<Q, K, V> Index<&Q> for ScanMap<K, V>
where
    K: Borrow<Q>,
    Q: ?Sized + Eq,
{
    index_fn!();
}

impl<Q, K, V> IndexMut<&Q> for ScanMap<K, V>
where
    K: Borrow<Q>,
    Q: ?Sized + Eq,
{
    index_mut_fn!();
}

impl<K, V> IntoIterator for ScanMap<K, V> {
    into_iter_fn_for_slice!(entries);
}

impl<'a, K, V> IntoIterator for &'a ScanMap<K, V> {
    into_iter_ref_fn!();
}

impl<'a, K, V> IntoIterator for &'a mut ScanMap<K, V> {
    into_iter_mut_ref_fn!();
}

impl<K, V, MT> PartialEq<MT> for ScanMap<K, V>
where
    K: Eq,
    V: PartialEq,
    MT: Map<K, V>,
{
    partial_eq_fn!();
}

impl<K, V> MapIterator<K, V> for ScanMap<K, V> {
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

impl<K, V> Map<K, V> for ScanMap<K, V>
where
    K: Eq,
{
    map_boilerplate_for_slice!(entries);
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::maps::map_tests::test_map_trait_impl;
    use std::collections::HashMap as StdHashMap;

    #[test]
    fn test_scan_map() {
        let map = ScanMap::new(vec![(1, 1), (2, 2), (3, 3)]);
        let reference = StdHashMap::from([(1, 1), (2, 2), (3, 3)]);
        let other = StdHashMap::from([(1, 1), (2, 2), (3, 3), (4, 4)]);
        test_map_trait_impl(&map, &reference, &other);

        let map = ScanMap::new(vec![]);
        let reference = StdHashMap::from([]);
        let other = StdHashMap::from([(1, 1), (2, 2), (3, 3), (4, 4)]);
        test_map_trait_impl(&map, &reference, &other);

        let map = ScanMap::new(vec![(1, 1), (2, 2), (3, 3), (1, 4)]);
        let reference = StdHashMap::from([(2, 2), (3, 3), (1, 4)]);
        let other = StdHashMap::from([(1, 1), (2, 2), (3, 3), (4, 4)]);
        test_map_trait_impl(&map, &reference, &other);
    }

    #[test]
    fn test_get_many_mut() {
        let mut map = ScanMap::new(vec![(1, 1), (2, 2), (3, 3)]);

        let values = map.get_many_mut([&1, &2, &3]);
        assert_eq!(values, Some([&mut 1, &mut 2, &mut 3]));

        let values = map.get_many_mut([]);
        assert_eq!(values, Some([]));

        let values = map.get_many_mut([&1, &2, &3, &4]);
        assert_eq!(values, None);
    }

    #[test]
    fn test_into_iter() {
        let map = &ScanMap::new(vec![(1, 1), (2, 2), (3, 3)]);
        let into_map: StdHashMap<_, _> = map.into_iter().map(|(k, v)| (*k, *v)).collect();
        assert_eq!(map, &into_map);

        let map = &mut ScanMap::new(vec![(1, 1), (2, 2), (3, 3)]);
        let into_map: StdHashMap<_, _> = map.into_iter().map(|(k, v)| (*k, *v)).collect();
        assert_eq!(map, &into_map);
    }
}
