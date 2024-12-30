use crate::inline_maps::InlineOrderedScanMap;
use crate::sets::decl_macros::{
    bitand_fn, bitor_fn, bitxor_fn, debug_fn, get_fn, into_iter_fn, into_iter_ref_fn,
    partial_eq_fn, set_iteration_funcs, sub_fn,
};
use crate::sets::{IntoIter, Iter};
use crate::traits::{Len, MapIteration, MapQuery, Set, SetIteration, SetOps, SetQuery};
use core::fmt::Debug;
use core::hash::Hash;
use core::ops::{BitAnd, BitOr, BitXor, Sub};
use equivalent::Comparable;

#[cfg(feature = "serde")]
use {
    crate::sets::decl_macros::serialize_fn,
    serde::ser::SerializeSeq,
    serde::{Serialize, Serializer},
};

/// A general purpose set implemented using linear scanning.
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
pub struct InlineOrderedScanSet<T, const SZ: usize> {
    map: InlineOrderedScanMap<T, (), SZ>,
}

impl<T, const SZ: usize> InlineOrderedScanSet<T, SZ> {
    /// Creates a frozen set.
    #[must_use]
    pub const fn new(map: InlineOrderedScanMap<T, (), SZ>) -> Self {
        Self { map }
    }
}

impl<T, Q, const SZ: usize> Set<T, Q> for InlineOrderedScanSet<T, SZ> where
    Q: ?Sized + Ord + Comparable<T>
{
}

impl<T, Q, const SZ: usize> SetQuery<T, Q> for InlineOrderedScanSet<T, SZ>
where
    Q: ?Sized + Ord + Comparable<T>,
{
    get_fn!();
}

impl<T, const SZ: usize> SetIteration<T> for InlineOrderedScanSet<T, SZ> {
    type Iterator<'a>
        = Iter<'a, T>
    where
        T: 'a;

    set_iteration_funcs!();
}

impl<T, const SZ: usize> Len for InlineOrderedScanSet<T, SZ> {
    fn len(&self) -> usize {
        SZ
    }
}

impl<T, ST, const SZ: usize> BitOr<&ST> for &InlineOrderedScanSet<T, SZ>
where
    T: Hash + Eq + Ord + Clone,
    ST: Set<T>,
{
    bitor_fn!();
}

impl<T, ST, const SZ: usize> BitAnd<&ST> for &InlineOrderedScanSet<T, SZ>
where
    T: Hash + Eq + Ord + Clone,
    ST: Set<T>,
{
    bitand_fn!();
}

impl<T, ST, const SZ: usize> BitXor<&ST> for &InlineOrderedScanSet<T, SZ>
where
    T: Hash + Eq + Ord + Clone,
    ST: Set<T>,
{
    bitxor_fn!();
}

impl<T, ST, const SZ: usize> Sub<&ST> for &InlineOrderedScanSet<T, SZ>
where
    T: Hash + Eq + Ord + Clone,
    ST: Set<T>,
{
    sub_fn!();
}

impl<T, const SZ: usize> IntoIterator for InlineOrderedScanSet<T, SZ> {
    into_iter_fn!();
}

impl<'a, T, const SZ: usize> IntoIterator for &'a InlineOrderedScanSet<T, SZ> {
    into_iter_ref_fn!();
}

impl<T, ST, const SZ: usize> PartialEq<ST> for InlineOrderedScanSet<T, SZ>
where
    T: Ord,
    ST: Set<T>,
{
    partial_eq_fn!();
}

impl<T, const SZ: usize> Eq for InlineOrderedScanSet<T, SZ> where T: Ord {}

impl<T, const SZ: usize> Debug for InlineOrderedScanSet<T, SZ>
where
    T: Debug,
{
    debug_fn!();
}

#[cfg(feature = "serde")]
impl<T, const SZ: usize> Serialize for InlineOrderedScanSet<T, SZ>
where
    T: Serialize,
{
    serialize_fn!();
}
