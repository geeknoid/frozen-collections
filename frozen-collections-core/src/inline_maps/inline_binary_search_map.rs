use crate::maps::decl_macros::{
    binary_search_core, contains_key_fn, debug_fn, get_many_mut_body, get_many_mut_fn, index_fn,
    into_iter_fn_for_slice, into_iter_mut_ref_fn, into_iter_ref_fn, map_boilerplate_for_slice,
    map_iterator_boilerplate_for_slice, partial_eq_fn,
};
use crate::maps::{IntoIter, IntoKeys, IntoValues, Iter, IterMut, Keys, Values, ValuesMut};
use crate::traits::{Len, Map, MapIterator};
use crate::utils::dedup_by_keep_last;
use alloc::vec::Vec;
use core::borrow::Borrow;
use core::fmt::{Debug, Formatter, Result};
use core::ops::Index;

/// A general purpose map implemented using binary search.
///
#[doc = include_str!("../doc_snippets/type_compat_warning.md")]
#[doc = include_str!("../doc_snippets/about.md")]
#[doc = include_str!("../doc_snippets/order_warning.md")]
///
/// # Type Parameters
///
/// - `K`: The key type.
/// - `V`: The value type.
/// - `SZ`: The number of entries in the map.
#[derive(Clone)]
pub struct InlineBinarySearchMap<K, V, const SZ: usize> {
    entries: [(K, V); SZ],
}

impl<K, V, const SZ: usize> InlineBinarySearchMap<K, V, SZ>
where
    K: Ord,
{
    /// Creates a frozen map.
    ///
    /// # Errors
    ///
    /// Fails if the length of the vector, after removing duplicates, isn't equal to the generic parameter `SZ`.
    pub fn new(mut entries: Vec<(K, V)>) -> core::result::Result<Self, String> {
        entries.sort_by(|x, y| x.0.cmp(&y.0));
        dedup_by_keep_last(&mut entries, |x, y| x.0.eq(&y.0));

        let len = entries.len();
        Ok(Self::new_raw(entries.try_into().map_err(|_| {
            format!("incorrect # of entries: got {len} but SZ={SZ}")
        })?))
    }

    /// Creates a frozen map.
    ///
    /// This function assumes the vector is sorted according to the ordering of the [`Ord`] trait.
    #[must_use]
    pub const fn new_raw(processed_entries: [(K, V); SZ]) -> Self {
        Self {
            entries: processed_entries,
        }
    }
}

impl<K, V, const SZ: usize> InlineBinarySearchMap<K, V, SZ> {
    binary_search_core!();
}

impl<K, V, const SZ: usize> Len for InlineBinarySearchMap<K, V, SZ> {
    fn len(&self) -> usize {
        SZ
    }
}

impl<K, V, const SZ: usize> Debug for InlineBinarySearchMap<K, V, SZ>
where
    K: Debug,
    V: Debug,
{
    debug_fn!();
}

impl<Q, K, V, const SZ: usize> Index<&Q> for InlineBinarySearchMap<K, V, SZ>
where
    K: Borrow<Q>,
    Q: ?Sized + Ord,
{
    index_fn!();
}

impl<K, V, const SZ: usize> IntoIterator for InlineBinarySearchMap<K, V, SZ> {
    into_iter_fn_for_slice!(entries);
}

impl<'a, K, V, const SZ: usize> IntoIterator for &'a InlineBinarySearchMap<K, V, SZ> {
    into_iter_ref_fn!();
}

impl<'a, K, V, const SZ: usize> IntoIterator for &'a mut InlineBinarySearchMap<K, V, SZ> {
    into_iter_mut_ref_fn!();
}

impl<K, V, MT, const SZ: usize> PartialEq<MT> for InlineBinarySearchMap<K, V, SZ>
where
    K: Ord,
    V: PartialEq,
    MT: Map<K, V>,
{
    partial_eq_fn!();
}

impl<K, V, const SZ: usize> Eq for InlineBinarySearchMap<K, V, SZ>
where
    K: Ord,
    V: Eq,
{
}

impl<K, V, const SZ: usize> MapIterator<K, V> for InlineBinarySearchMap<K, V, SZ> {
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

impl<K, V, const SZ: usize> Map<K, V> for InlineBinarySearchMap<K, V, SZ>
where
    K: Ord,
{
    map_boilerplate_for_slice!(entries);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_fails_when_too_many_entries() {
        let mut entries = Vec::new();
        for i in 0..3 {
            entries.push((i, 42));
        }

        let map = InlineBinarySearchMap::<_, _, 2>::new(entries);
        assert_eq!(
            map,
            Err("incorrect # of entries: got 3 but SZ=2".to_string())
        );
    }
}
