use crate::maps::decl_macros::{
    binary_search_query_funcs, debug_fn, get_disjoint_mut_fn, get_disjoint_unchecked_mut_body, get_disjoint_unchecked_mut_fn, index_fn,
    into_iter_fn, into_iter_mut_ref_fn, into_iter_ref_fn, map_iteration_funcs, partial_eq_fn,
};
use crate::maps::{IntoIter, IntoKeys, IntoValues, Iter, IterMut, Keys, Values, ValuesMut};
use crate::traits::{Len, Map, MapIteration, MapQuery};
use crate::utils::dedup_by_keep_last;
use alloc::boxed::Box;
use alloc::vec::Vec;
use core::fmt::{Debug, Formatter, Result};
use core::ops::Index;
use equivalent::Comparable;

#[cfg(feature = "serde")]
use {
    crate::maps::decl_macros::serialize_fn,
    serde::ser::SerializeMap,
    serde::{Serialize, Serializer},
};

/// A general-purpose map implemented using binary search.
///
#[doc = include_str!("../doc_snippets/private_api_warning.md")]
#[doc = include_str!("../doc_snippets/about.md")]
#[doc = include_str!("../doc_snippets/ord_warning.md")]
///
#[derive(Clone)]
pub struct BinarySearchMap<K, V> {
    entries: Box<[(K, V)]>,
}

impl<K, V> BinarySearchMap<K, V>
where
    K: Ord,
{
    /// Creates a frozen map.
    #[must_use]
    pub fn new(mut entries: Vec<(K, V)>) -> Self {
        entries.sort_by(|x, y| x.0.cmp(&y.0));
        dedup_by_keep_last(&mut entries, |x, y| x.0.eq(&y.0));
        Self::new_raw(entries)
    }

    /// Creates a frozen map.
    #[must_use]
    pub(crate) fn new_raw(processed_entries: Vec<(K, V)>) -> Self {
        Self {
            entries: processed_entries.into_boxed_slice(),
        }
    }
}

impl<K, V> Default for BinarySearchMap<K, V> {
    fn default() -> Self {
        Self { entries: Box::default() }
    }
}

impl<K, V, Q> Map<K, V, Q> for BinarySearchMap<K, V>
where
    Q: ?Sized + Eq + Comparable<K>,
{
    get_disjoint_mut_fn!();
    get_disjoint_unchecked_mut_fn!();
}

impl<K, V, Q> MapQuery<K, V, Q> for BinarySearchMap<K, V>
where
    Q: ?Sized + Eq + Comparable<K>,
{
    binary_search_query_funcs!();
}

impl<K, V> MapIteration<K, V> for BinarySearchMap<K, V> {
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

impl<K, V> Len for BinarySearchMap<K, V> {
    fn len(&self) -> usize {
        self.entries.len()
    }
}

impl<Q, K, V> Index<&Q> for BinarySearchMap<K, V>
where
    Q: ?Sized + Eq + Comparable<K>,
{
    index_fn!();
}

impl<K, V> IntoIterator for BinarySearchMap<K, V> {
    into_iter_fn!(entries);
}

impl<'a, K, V> IntoIterator for &'a BinarySearchMap<K, V> {
    into_iter_ref_fn!();
}

impl<'a, K, V> IntoIterator for &'a mut BinarySearchMap<K, V> {
    into_iter_mut_ref_fn!();
}

impl<K, V, MT> PartialEq<MT> for BinarySearchMap<K, V>
where
    K: Ord,
    V: PartialEq,
    MT: Map<K, V>,
{
    partial_eq_fn!();
}

impl<K, V> Eq for BinarySearchMap<K, V>
where
    K: Ord,
    V: Eq,
{
}

impl<K, V> Debug for BinarySearchMap<K, V>
where
    K: Debug,
    V: Debug,
{
    debug_fn!();
}

#[cfg(feature = "serde")]
impl<K, V> Serialize for BinarySearchMap<K, V>
where
    K: Serialize,
    V: Serialize,
{
    serialize_fn!();
}
