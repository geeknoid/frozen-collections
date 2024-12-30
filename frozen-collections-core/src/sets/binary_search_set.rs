use core::fmt::Debug;
use core::hash::Hash;
use core::ops::{BitAnd, BitOr, BitXor, Sub};

use crate::maps::BinarySearchMap;
use crate::sets::decl_macros::{
    bitand_fn, bitor_fn, bitxor_fn, debug_fn, get_fn, into_iter_fn, into_iter_ref_fn,
    partial_eq_fn, set_iteration_funcs, sub_fn,
};
use crate::sets::{IntoIter, Iter};
use crate::traits::{Len, MapIteration, MapQuery, Set, SetIteration, SetOps, SetQuery};

#[cfg(feature = "serde")]
use {
    crate::sets::decl_macros::serialize_fn,
    serde::ser::SerializeSeq,
    serde::{Serialize, Serializer},
};

/// A general purpose set implemented using binary search.
///
#[doc = include_str!("../doc_snippets/private_api_warning.md")]
#[doc = include_str!("../doc_snippets/about.md")]
#[doc = include_str!("../doc_snippets/ord_warning.md")]
///
#[derive(Clone)]
pub struct BinarySearchSet<T> {
    map: BinarySearchMap<T, ()>,
}

impl<T> BinarySearchSet<T>
where
    T: Ord,
{
    /// Creates a frozen set.
    #[must_use]
    pub const fn new(map: BinarySearchMap<T, ()>) -> Self {
        Self { map }
    }
}

impl<T> Default for BinarySearchSet<T> {
    fn default() -> Self {
        Self {
            map: BinarySearchMap::default(),
        }
    }
}

impl<T> Set<T, T> for BinarySearchSet<T> where T: Ord {}

impl<T> SetQuery<T, T> for BinarySearchSet<T>
where
    T: Ord,
{
    get_fn!("Scalar");
}

impl<T> SetIteration<T> for BinarySearchSet<T> {
    type Iterator<'a>
        = Iter<'a, T>
    where
        T: 'a;

    set_iteration_funcs!();
}

impl<T> Len for BinarySearchSet<T> {
    fn len(&self) -> usize {
        self.map.len()
    }
}

impl<T, ST> BitOr<&ST> for &BinarySearchSet<T>
where
    T: Hash + Ord + Clone,
    ST: Set<T>,
{
    bitor_fn!();
}

impl<T, ST> BitAnd<&ST> for &BinarySearchSet<T>
where
    T: Hash + Ord + Clone,
    ST: Set<T>,
{
    bitand_fn!();
}

impl<T, ST> BitXor<&ST> for &BinarySearchSet<T>
where
    T: Hash + Ord + Clone,
    ST: Set<T>,
{
    bitxor_fn!();
}

impl<T, ST> Sub<&ST> for &BinarySearchSet<T>
where
    T: Hash + Ord + Clone,
    ST: Set<T>,
{
    sub_fn!();
}

impl<T> IntoIterator for BinarySearchSet<T> {
    into_iter_fn!();
}

impl<'a, T> IntoIterator for &'a BinarySearchSet<T> {
    into_iter_ref_fn!();
}

impl<T, ST> PartialEq<ST> for BinarySearchSet<T>
where
    T: Ord,
    ST: Set<T>,
{
    partial_eq_fn!();
}

impl<T> Eq for BinarySearchSet<T> where T: Ord {}

impl<T> Debug for BinarySearchSet<T>
where
    T: Debug,
{
    debug_fn!();
}

#[cfg(feature = "serde")]
impl<T> Serialize for BinarySearchSet<T>
where
    T: Serialize,
{
    serialize_fn!();
}
