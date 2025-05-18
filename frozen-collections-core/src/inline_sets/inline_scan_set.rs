use crate::inline_maps::InlineScanMap;
use crate::sets::decl_macros::{
    bitand_fn, bitor_fn, bitxor_fn, debug_fn, get_fn, into_iter_fn, into_iter_ref_fn, partial_eq_fn, set_iteration_funcs, sub_fn,
};
use crate::sets::{IntoIter, Iter};
use crate::traits::{Len, MapIteration, MapQuery, Set, SetIteration, SetOps, SetQuery};
use core::fmt::Debug;
use core::hash::Hash;
use core::ops::{BitAnd, BitOr, BitXor, Sub};
use equivalent::Equivalent;

#[cfg(feature = "serde")]
use {
    crate::sets::decl_macros::serialize_fn,
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
}

impl<T, Q, const SZ: usize> Set<T, Q> for InlineScanSet<T, SZ> where Q: ?Sized + Eq + Equivalent<T> {}

impl<T, Q, const SZ: usize> SetQuery<T, Q> for InlineScanSet<T, SZ>
where
    Q: ?Sized + Eq + Equivalent<T>,
{
    get_fn!();
}

impl<T, const SZ: usize> SetIteration<T> for InlineScanSet<T, SZ> {
    type Iterator<'a>
        = Iter<'a, T>
    where
        T: 'a;

    set_iteration_funcs!();
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
    bitor_fn!();
}

impl<T, ST, const SZ: usize> BitAnd<&ST> for &InlineScanSet<T, SZ>
where
    T: Hash + Eq + Clone,
    ST: Set<T>,
{
    bitand_fn!();
}

impl<T, ST, const SZ: usize> BitXor<&ST> for &InlineScanSet<T, SZ>
where
    T: Hash + Eq + Clone,
    ST: Set<T>,
{
    bitxor_fn!();
}

impl<T, ST, const SZ: usize> Sub<&ST> for &InlineScanSet<T, SZ>
where
    T: Hash + Eq + Clone,
    ST: Set<T>,
{
    sub_fn!();
}

impl<T, const SZ: usize> IntoIterator for InlineScanSet<T, SZ> {
    into_iter_fn!();
}

impl<'a, T, const SZ: usize> IntoIterator for &'a InlineScanSet<T, SZ> {
    into_iter_ref_fn!();
}

impl<T, ST, const SZ: usize> PartialEq<ST> for InlineScanSet<T, SZ>
where
    T: Eq,
    ST: Set<T>,
{
    partial_eq_fn!();
}

impl<T, const SZ: usize> Eq for InlineScanSet<T, SZ> where T: Eq {}

impl<T, const SZ: usize> Debug for InlineScanSet<T, SZ>
where
    T: Debug,
{
    debug_fn!();
}

#[cfg(feature = "serde")]
impl<T, const SZ: usize> Serialize for InlineScanSet<T, SZ>
where
    T: Serialize,
{
    serialize_fn!();
}
