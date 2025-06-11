use crate::maps::ScanMap;
use crate::maps::decl_macros::len_trait_funcs;
use crate::sets::decl_macros::{
    bitand_trait_funcs, bitor_trait_funcs, bitxor_trait_funcs, common_primary_funcs, debug_trait_funcs, into_iterator_ref_trait_funcs,
    into_iterator_trait_funcs, partial_eq_trait_funcs, scan_primary_funcs, set_extras_trait_funcs, set_iteration_trait_funcs,
    set_query_trait_funcs, sub_trait_funcs,
};
use crate::sets::{IntoIter, Iter};
use crate::traits::{Len, Set, SetExtras, SetIteration, SetOps, SetQuery};
use core::fmt::Debug;
use core::hash::Hash;
use core::ops::{BitAnd, BitOr, BitXor, Sub};
use equivalent::Equivalent;

#[cfg(feature = "serde")]
use {
    crate::sets::decl_macros::serialize_trait_funcs,
    serde::ser::SerializeSeq,
    serde::{Serialize, Serializer},
};

/// A general-purpose set implemented with linear scanning.
///
#[doc = include_str!("../doc_snippets/private_api_warning.md")]
#[doc = include_str!("../doc_snippets/about.md")]
///
#[derive(Clone)]
pub struct ScanSet<T> {
    map: ScanMap<T, ()>,
}

impl<T> ScanSet<T> {
    /// Creates a frozen set.
    #[must_use]
    pub const fn new(map: ScanMap<T, ()>) -> Self {
        Self { map }
    }

    scan_primary_funcs!();
    common_primary_funcs!(non_const_len);
}

impl<T> Default for ScanSet<T> {
    fn default() -> Self {
        Self { map: ScanMap::default() }
    }
}

impl<T, Q> Set<T, Q> for ScanSet<T> where Q: ?Sized + Equivalent<T> {}

impl<T, Q> SetExtras<T, Q> for ScanSet<T>
where
    Q: ?Sized + Equivalent<T>,
{
    set_extras_trait_funcs!();
}

impl<T, Q> SetQuery<Q> for ScanSet<T>
where
    Q: ?Sized + Equivalent<T>,
{
    set_query_trait_funcs!();
}

impl<T> SetIteration<T> for ScanSet<T> {
    type Iterator<'a>
        = Iter<'a, T>
    where
        T: 'a;

    set_iteration_trait_funcs!();
}

impl<T> Len for ScanSet<T> {
    len_trait_funcs!();
}

impl<T, ST> BitOr<&ST> for &ScanSet<T>
where
    T: Hash + Eq + Clone,
    ST: Set<T>,
{
    bitor_trait_funcs!();
}

impl<T, ST> BitAnd<&ST> for &ScanSet<T>
where
    T: Hash + Eq + Clone,
    ST: Set<T>,
{
    bitand_trait_funcs!();
}

impl<T, ST> BitXor<&ST> for &ScanSet<T>
where
    T: Hash + Eq + Clone,
    ST: Set<T>,
{
    bitxor_trait_funcs!();
}

impl<T, ST> Sub<&ST> for &ScanSet<T>
where
    T: Hash + Eq + Clone,
    ST: Set<T>,
{
    sub_trait_funcs!();
}

impl<T> IntoIterator for ScanSet<T> {
    into_iterator_trait_funcs!();
}

impl<'a, T> IntoIterator for &'a ScanSet<T> {
    into_iterator_ref_trait_funcs!();
}

impl<T, ST> PartialEq<ST> for ScanSet<T>
where
    T: PartialEq,
    ST: SetQuery<T>,
{
    partial_eq_trait_funcs!();
}

impl<T> Eq for ScanSet<T> where T: Eq {}

impl<T> Debug for ScanSet<T>
where
    T: Debug,
{
    debug_trait_funcs!();
}

#[cfg(feature = "serde")]
impl<T> Serialize for ScanSet<T>
where
    T: Serialize,
{
    serialize_trait_funcs!();
}
