use crate::inline_maps::InlineScanMap;
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

/// A general-purpose set implemented using linear scanning.
///
#[doc = include_str!("../doc_snippets/private_api_warning.md")]
#[doc = include_str!("../doc_snippets/about.md")]
///
/// # Type Parameters
///
/// - `T`: The value type.
/// - `SZ`: The number of entries in the set.
#[derive(Clone)]
pub struct InlineScanSet<T, const SZ: usize> {
    map: InlineScanMap<T, (), SZ>,
}

impl<T, const SZ: usize> InlineScanSet<T, SZ> {
    /// Creates a frozen set.
    #[must_use]
    pub const fn new(map: InlineScanMap<T, (), SZ>) -> Self {
        Self { map }
    }

    scan_primary_funcs!();
    common_primary_funcs!(const_len);
}

impl<T, Q, const SZ: usize> Set<T, Q> for InlineScanSet<T, SZ> where Q: ?Sized + Equivalent<T> {}

impl<T, Q, const SZ: usize> SetExtras<T, Q> for InlineScanSet<T, SZ>
where
    Q: ?Sized + Equivalent<T>,
{
    set_extras_trait_funcs!();
}

impl<T, Q, const SZ: usize> SetQuery<Q> for InlineScanSet<T, SZ>
where
    Q: ?Sized + Equivalent<T>,
{
    set_query_trait_funcs!();
}

impl<T, const SZ: usize> SetIteration<T> for InlineScanSet<T, SZ> {
    type Iterator<'a>
        = Iter<'a, T>
    where
        T: 'a;

    set_iteration_trait_funcs!();
}

impl<T, const SZ: usize> Len for InlineScanSet<T, SZ> {
    fn len(&self) -> usize {
        SZ
    }
}

impl<T, ST, const SZ: usize> BitOr<&ST> for &InlineScanSet<T, SZ>
where
    T: Hash + Eq + Clone,
    ST: Set<T>,
{
    bitor_trait_funcs!();
}

impl<T, ST, const SZ: usize> BitAnd<&ST> for &InlineScanSet<T, SZ>
where
    T: Hash + Eq + Clone,
    ST: Set<T>,
{
    bitand_trait_funcs!();
}

impl<T, ST, const SZ: usize> BitXor<&ST> for &InlineScanSet<T, SZ>
where
    T: Hash + Eq + Clone,
    ST: Set<T>,
{
    bitxor_trait_funcs!();
}

impl<T, ST, const SZ: usize> Sub<&ST> for &InlineScanSet<T, SZ>
where
    T: Hash + Eq + Clone,
    ST: Set<T>,
{
    sub_trait_funcs!();
}

impl<T, const SZ: usize> IntoIterator for InlineScanSet<T, SZ> {
    into_iterator_trait_funcs!();
}

impl<'a, T, const SZ: usize> IntoIterator for &'a InlineScanSet<T, SZ> {
    into_iterator_ref_trait_funcs!();
}

impl<T, ST, const SZ: usize> PartialEq<ST> for InlineScanSet<T, SZ>
where
    T: PartialEq,
    ST: SetQuery<T>,
{
    partial_eq_trait_funcs!();
}

impl<T, const SZ: usize> Eq for InlineScanSet<T, SZ> where T: Eq {}

impl<T, const SZ: usize> Debug for InlineScanSet<T, SZ>
where
    T: Debug,
{
    debug_trait_funcs!();
}

#[cfg(feature = "serde")]
impl<T, const SZ: usize> Serialize for InlineScanSet<T, SZ>
where
    T: Serialize,
{
    serialize_trait_funcs!();
}
