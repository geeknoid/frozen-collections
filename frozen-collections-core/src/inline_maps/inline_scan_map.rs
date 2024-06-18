use crate::maps::decl_macros::{
    contains_key_fn, debug_fn, get_many_mut_body, get_many_mut_fn, index_fn,
    into_iter_fn_for_slice, into_iter_mut_ref_fn, into_iter_ref_fn, map_boilerplate_for_slice,
    map_iterator_boilerplate_for_slice, partial_eq_fn, scan_core,
};
use crate::maps::{IntoIter, IntoKeys, IntoValues, Iter, IterMut, Keys, Values, ValuesMut};
use crate::traits::{Len, Map, MapIterator};
use crate::utils::slow_dedup_by_keep_last;
use alloc::vec::Vec;
use core::borrow::Borrow;
use core::fmt::{Debug, Formatter, Result};
use core::ops::Index;

/// A general purpose map implemented using linear scanning.
///
#[doc = include_str!("../doc_snippets/type_compat_warning.md")]
#[doc = include_str!("../doc_snippets/about.md")]
///
/// # Type Parameters
///
/// - `K`: The key type.
/// - `V`: The value type.
/// - `SZ`: The number of entries in the map.
#[derive(Clone)]
pub struct InlineScanMap<K, V, const SZ: usize> {
    entries: [(K, V); SZ],
}

impl<K, V, const SZ: usize> InlineScanMap<K, V, SZ>
where
    K: Eq,
{
    /// Creates a frozen map.
    ///
    /// # Errors
    ///
    /// Fails if the length of the vector, after removing duplicates, isn't equal to the generic parameter `SZ`.
    pub fn new(mut entries: Vec<(K, V)>) -> core::result::Result<Self, String> {
        slow_dedup_by_keep_last(&mut entries, |x, y| x.0.eq(&y.0));

        let len = entries.len();
        Ok(Self::new_raw(entries.try_into().map_err(|_| {
            format!("incorrect # of entries: got {len} but SZ={SZ}")
        })?))
    }

    /// Creates a frozen map.
    #[must_use]
    pub const fn new_raw(processed_entries: [(K, V); SZ]) -> Self {
        Self {
            entries: processed_entries,
        }
    }
}

impl<K, V, const SZ: usize> InlineScanMap<K, V, SZ> {
    scan_core!();
}

impl<K, V, const SZ: usize> Len for InlineScanMap<K, V, SZ> {
    fn len(&self) -> usize {
        SZ
    }
}

impl<K, V, const SZ: usize> Debug for InlineScanMap<K, V, SZ>
where
    K: Debug,
    V: Debug,
{
    debug_fn!();
}

impl<Q, K, V, const SZ: usize> Index<&Q> for InlineScanMap<K, V, SZ>
where
    K: Borrow<Q>,
    Q: ?Sized + Eq,
{
    index_fn!();
}

impl<K, V, const SZ: usize> IntoIterator for InlineScanMap<K, V, SZ> {
    into_iter_fn_for_slice!(entries);
}

impl<'a, K, V, const SZ: usize> IntoIterator for &'a InlineScanMap<K, V, SZ> {
    into_iter_ref_fn!();
}

impl<'a, K, V, const SZ: usize> IntoIterator for &'a mut InlineScanMap<K, V, SZ> {
    into_iter_mut_ref_fn!();
}

impl<K, V, MT, const N: usize> PartialEq<MT> for InlineScanMap<K, V, N>
where
    K: Eq,
    V: PartialEq,
    MT: Map<K, V>,
{
    partial_eq_fn!();
}

impl<K, V, const N: usize> Eq for InlineScanMap<K, V, N>
where
    K: Eq,
    V: PartialEq,
{
}

impl<K, V, const SZ: usize> MapIterator<K, V> for InlineScanMap<K, V, SZ> {
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

impl<K, V, const SZ: usize> Map<K, V> for InlineScanMap<K, V, SZ>
where
    K: Eq,
{
    map_boilerplate_for_slice!(entries);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_fails_when_bad_range() {
        let mut entries = Vec::new();
        for i in 0..3 {
            entries.push((i, 42));
        }

        let map = InlineScanMap::<_, _, 2>::new(entries);
        assert_eq!(
            map,
            Err("incorrect # of entries: got 3 but SZ=2".to_string())
        );
    }
}
