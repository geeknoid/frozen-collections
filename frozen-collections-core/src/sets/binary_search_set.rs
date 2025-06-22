use crate::maps::BinarySearchMap;
use crate::maps::decl_macros::len_trait_funcs;
use crate::sets::decl_macros::{
    binary_search_primary_funcs, bitand_trait_funcs, bitor_trait_funcs, bitxor_trait_funcs, common_primary_funcs, debug_trait_funcs,
    into_iterator_ref_trait_funcs, into_iterator_trait_funcs, partial_eq_trait_funcs, set_extras_trait_funcs, set_iteration_trait_funcs,
    set_query_trait_funcs, sub_trait_funcs,
};
use crate::sets::{IntoIter, Iter};
use crate::traits::{Len, Set, SetExtras, SetIteration, SetOps, SetQuery};
use core::fmt::Debug;
use core::hash::Hash;
use core::ops::{BitAnd, BitOr, BitXor, Sub};
use equivalent::Comparable;

#[cfg(feature = "serde")]
use {
    crate::sets::decl_macros::serialize_trait_funcs,
    serde::ser::SerializeSeq,
    serde::{Serialize, Serializer},
};

/// A general-purpose set implemented using binary search.
///
#[doc = include_str!("../doc_snippets/private_api_warning.md")]
#[doc = include_str!("../doc_snippets/about.md")]
#[doc = include_str!("../doc_snippets/ord_warning.md")]
///
#[derive(Clone)]
pub struct BinarySearchSet<T> {
    map: BinarySearchMap<T, ()>,
}

impl<T> BinarySearchSet<T> {
    /// Creates a frozen set.
    #[must_use]
    pub const fn new(map: BinarySearchMap<T, ()>) -> Self {
        Self { map }
    }

    binary_search_primary_funcs!();
    common_primary_funcs!(non_const_len);
}

impl<T> Default for BinarySearchSet<T> {
    fn default() -> Self {
        Self {
            map: BinarySearchMap::default(),
        }
    }
}

impl<T, Q> Set<T, Q> for BinarySearchSet<T> where Q: ?Sized + Comparable<T> {}

impl<T, Q> SetExtras<T, Q> for BinarySearchSet<T>
where
    Q: ?Sized + Comparable<T>,
{
    set_extras_trait_funcs!();
}

impl<T, Q> SetQuery<Q> for BinarySearchSet<T>
where
    Q: ?Sized + Comparable<T>,
{
    set_query_trait_funcs!();
}

impl<T> SetIteration<T> for BinarySearchSet<T> {
    type Iterator<'a>
        = Iter<'a, T>
    where
        T: 'a;

    set_iteration_trait_funcs!();
}

impl<T> Len for BinarySearchSet<T> {
    len_trait_funcs!();
}

impl<T, ST> BitOr<&ST> for &BinarySearchSet<T>
where
    T: Hash + Ord + Clone,
    ST: Set<T>,
{
    bitor_trait_funcs!();
}

impl<T, ST> BitAnd<&ST> for &BinarySearchSet<T>
where
    T: Hash + Ord + Clone,
    ST: Set<T>,
{
    bitand_trait_funcs!();
}

impl<T, ST> BitXor<&ST> for &BinarySearchSet<T>
where
    T: Hash + Ord + Clone,
    ST: Set<T>,
{
    bitxor_trait_funcs!();
}

impl<T, ST> Sub<&ST> for &BinarySearchSet<T>
where
    T: Hash + Ord + Clone,
    ST: Set<T>,
{
    sub_trait_funcs!();
}

impl<T> IntoIterator for BinarySearchSet<T> {
    into_iterator_trait_funcs!();
}

impl<'a, T> IntoIterator for &'a BinarySearchSet<T> {
    into_iterator_ref_trait_funcs!();
}

impl<T, ST> PartialEq<ST> for BinarySearchSet<T>
where
    T: Ord,
    ST: SetQuery<T>,
{
    partial_eq_trait_funcs!();
}

impl<T> Eq for BinarySearchSet<T> where T: Ord {}

impl<T> Debug for BinarySearchSet<T>
where
    T: Debug,
{
    debug_trait_funcs!();
}

#[cfg(feature = "serde")]
impl<T> Serialize for BinarySearchSet<T>
where
    T: Serialize,
{
    serialize_trait_funcs!();
}
