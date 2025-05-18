use crate::maps::ScanMap;
use crate::sets::decl_macros::{
    bitand_fn, bitor_fn, bitxor_fn, debug_fn, get_fn, into_iter_fn, into_iter_ref_fn,
    partial_eq_fn, set_iteration_funcs, sub_fn,
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

/// A general-purpose set implemented with linear scanning.
///
#[doc = include_str!("../doc_snippets/private_api_warning.md")]
#[doc = include_str!("../doc_snippets/about.md")]
///
#[derive(Clone)]
pub struct ScanSet<T> {
    map: ScanMap<T, ()>,
}

impl<T> ScanSet<T>
where
    T: Eq,
{
    /// Creates a frozen set.
    #[must_use]
    pub const fn new(map: ScanMap<T, ()>) -> Self {
        Self { map }
    }
}

impl<T> Default for ScanSet<T> {
    fn default() -> Self {
        Self {
            map: ScanMap::default(),
        }
    }
}

impl<T, Q> Set<T, Q> for ScanSet<T> where Q: ?Sized + Eq + Equivalent<T> {}

impl<T, Q> SetQuery<T, Q> for ScanSet<T>
where
    Q: ?Sized + Eq + Equivalent<T>,
{
    get_fn!();
}

impl<T> SetIteration<T> for ScanSet<T> {
    type Iterator<'a>
        = Iter<'a, T>
    where
        T: 'a;

    set_iteration_funcs!();
}

impl<T> Len for ScanSet<T> {
    fn len(&self) -> usize {
        self.map.len()
    }
}

impl<T, ST> BitOr<&ST> for &ScanSet<T>
where
    T: Hash + Eq + Clone,
    ST: Set<T>,
{
    bitor_fn!();
}

impl<T, ST> BitAnd<&ST> for &ScanSet<T>
where
    T: Hash + Eq + Clone,
    ST: Set<T>,
{
    bitand_fn!();
}

impl<T, ST> BitXor<&ST> for &ScanSet<T>
where
    T: Hash + Eq + Clone,
    ST: Set<T>,
{
    bitxor_fn!();
}

impl<T, ST> Sub<&ST> for &ScanSet<T>
where
    T: Hash + Eq + Clone,
    ST: Set<T>,
{
    sub_fn!();
}

impl<T> IntoIterator for ScanSet<T> {
    into_iter_fn!();
}

impl<'a, T> IntoIterator for &'a ScanSet<T> {
    into_iter_ref_fn!();
}

impl<T, ST> PartialEq<ST> for ScanSet<T>
where
    T: Eq,
    ST: Set<T>,
{
    partial_eq_fn!();
}

impl<T> Eq for ScanSet<T> where T: Eq {}

impl<T> Debug for ScanSet<T>
where
    T: Debug,
{
    debug_fn!();
}

#[cfg(feature = "serde")]
impl<T> Serialize for ScanSet<T>
where
    T: Serialize,
{
    serialize_fn!();
}
