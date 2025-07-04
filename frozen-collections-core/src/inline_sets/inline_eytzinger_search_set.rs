use crate::inline_maps::InlineEytzingerSearchMap;
use crate::sets::decl_macros::{
    bitand_trait_funcs, bitor_trait_funcs, bitxor_trait_funcs, common_primary_funcs, debug_trait_funcs, into_iterator_ref_trait_funcs,
    into_iterator_trait_funcs, partial_eq_trait_funcs, set_extras_trait_funcs, set_iteration_trait_funcs, set_query_trait_funcs,
    sub_trait_funcs,
};
use crate::sets::{IntoIter, Iter};
use crate::traits::{Len, Set, SetExtras, SetIteration, SetOps, SetQuery};
use core::fmt::Debug;
use core::hash::Hash;
use core::ops::{BitAnd, BitOr, BitXor, Sub};
use equivalent::Comparable;

use crate::maps::decl_macros::len_trait_funcs;
#[cfg(feature = "serde")]
use {
    crate::sets::decl_macros::serialize_trait_funcs,
    serde::ser::SerializeSeq,
    serde::{Serialize, Serializer},
};

/// A general-purpose set implemented using Eytzinger search.
///
#[doc = include_str!("../doc_snippets/private_api_warning.md")]
#[doc = include_str!("../doc_snippets/about.md")]
#[doc = include_str!("../doc_snippets/ord_warning.md")]
///
/// # Type Parameters
///
/// - `T`: The value type.
/// - `SZ`: The number of entries in the set.
#[derive(Clone)]
pub struct InlineEytzingerSearchSet<T, const SZ: usize> {
    map: InlineEytzingerSearchMap<T, (), SZ>,
}

impl<T, const SZ: usize> InlineEytzingerSearchSet<T, SZ> {
    /// Creates a frozen set.
    #[must_use]
    pub const fn new(map: InlineEytzingerSearchMap<T, (), SZ>) -> Self {
        Self { map }
    }

    #[doc = include_str!("../doc_snippets/get_from_set.md")]
    #[inline]
    #[must_use]
    pub fn get<Q>(&self, value: &Q) -> Option<&T>
    where
        Q: ?Sized + Comparable<T>,
    {
        Some(self.map.get_key_value(value)?.0)
    }

    #[doc = include_str!("../doc_snippets/contains.md")]
    #[inline]
    #[must_use]
    pub fn contains<Q>(&self, value: &Q) -> bool
    where
        Q: ?Sized + Comparable<T>,
    {
        self.get(value).is_some()
    }

    common_primary_funcs!(const_len);
}

impl<T, Q, const SZ: usize> Set<T, Q> for InlineEytzingerSearchSet<T, SZ> where Q: ?Sized + Comparable<T> {}

impl<T, Q, const SZ: usize> SetExtras<T, Q> for InlineEytzingerSearchSet<T, SZ>
where
    Q: ?Sized + Comparable<T>,
{
    set_extras_trait_funcs!();
}

impl<T, Q, const SZ: usize> SetQuery<Q> for InlineEytzingerSearchSet<T, SZ>
where
    Q: ?Sized + Comparable<T>,
{
    set_query_trait_funcs!();
}

impl<T, const SZ: usize> SetIteration<T> for InlineEytzingerSearchSet<T, SZ> {
    type Iterator<'a>
        = Iter<'a, T>
    where
        T: 'a;

    set_iteration_trait_funcs!();
}

impl<T, const SZ: usize> Len for InlineEytzingerSearchSet<T, SZ> {
    len_trait_funcs!();
}

impl<T, ST, const SZ: usize> BitOr<&ST> for &InlineEytzingerSearchSet<T, SZ>
where
    T: Hash + Ord + Clone,
    ST: Set<T>,
{
    bitor_trait_funcs!();
}

impl<T, ST, const SZ: usize> BitAnd<&ST> for &InlineEytzingerSearchSet<T, SZ>
where
    T: Hash + Ord + Clone,
    ST: Set<T>,
{
    bitand_trait_funcs!();
}

impl<T, ST, const SZ: usize> BitXor<&ST> for &InlineEytzingerSearchSet<T, SZ>
where
    T: Hash + Ord + Clone,
    ST: Set<T>,
{
    bitxor_trait_funcs!();
}

impl<T, ST, const SZ: usize> Sub<&ST> for &InlineEytzingerSearchSet<T, SZ>
where
    T: Hash + Ord + Clone,
    ST: Set<T>,
{
    sub_trait_funcs!();
}

impl<T, const SZ: usize> IntoIterator for InlineEytzingerSearchSet<T, SZ> {
    into_iterator_trait_funcs!();
}

impl<'a, T, const SZ: usize> IntoIterator for &'a InlineEytzingerSearchSet<T, SZ> {
    into_iterator_ref_trait_funcs!();
}

impl<T, ST, const SZ: usize> PartialEq<ST> for InlineEytzingerSearchSet<T, SZ>
where
    T: Ord,
    ST: SetQuery<T>,
{
    partial_eq_trait_funcs!();
}

impl<T, const SZ: usize> Eq for InlineEytzingerSearchSet<T, SZ> where T: Ord {}

impl<T, const SZ: usize> Debug for InlineEytzingerSearchSet<T, SZ>
where
    T: Debug,
{
    debug_trait_funcs!();
}

#[cfg(feature = "serde")]
impl<T, const SZ: usize> Serialize for InlineEytzingerSearchSet<T, SZ>
where
    T: Serialize,
{
    serialize_trait_funcs!();
}
