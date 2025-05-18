use crate::hashers::BridgeHasher;
use crate::maps::HashMap;
use crate::sets::decl_macros::{
    bitand_fn, bitor_fn, bitxor_fn, debug_fn, into_iter_fn, into_iter_ref_fn, partial_eq_fn, set_iteration_funcs, sub_fn,
};
use crate::sets::{IntoIter, Iter};
use crate::traits::{CollectionMagnitude, Len, SetOps, SetQuery, SmallCollection};
use crate::traits::{Hasher, MapIteration, MapQuery, Set, SetIteration};
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

/// A general-purpose set implemented using a hash table.
///
#[doc = include_str!("../doc_snippets/private_api_warning.md")]
#[doc = include_str!("../doc_snippets/about.md")]
#[doc = include_str!("../doc_snippets/hash_warning.md")]
///
#[derive(Clone)]
pub struct HashSet<T, CM = SmallCollection, H = BridgeHasher> {
    map: HashMap<T, (), CM, H>,
}

impl<T, CM, H> HashSet<T, CM, H>
where
    T: Eq,
    H: Hasher<T>,
{
    /// Creates a frozen set.
    ///
    /// # Errors
    ///
    /// Fails if the number of entries in the vector, after deduplication, exceeds the
    /// magnitude of the collection as specified by the `CM` generic argument.
    #[must_use]
    pub const fn new(map: HashMap<T, (), CM, H>) -> Self {
        Self { map }
    }
}

impl<T, CM, H> Default for HashSet<T, CM, H>
where
    CM: CollectionMagnitude,
    H: Default,
{
    fn default() -> Self {
        Self { map: HashMap::default() }
    }
}

impl<T, Q, CM, H> Set<T, Q> for HashSet<T, CM, H>
where
    Q: ?Sized + Eq + Equivalent<T>,
    CM: CollectionMagnitude,
    H: Hasher<Q>,
{
}

impl<T, Q, CM, H> SetQuery<T, Q> for HashSet<T, CM, H>
where
    Q: ?Sized + Eq + Equivalent<T>,
    CM: CollectionMagnitude,
    H: Hasher<Q>,
{
    #[inline]
    fn get(&self, value: &Q) -> Option<&T> {
        Some(self.map.get_key_value(value)?.0)
    }
}

impl<T, CM, H> SetIteration<T> for HashSet<T, CM, H> {
    type Iterator<'a>
        = Iter<'a, T>
    where
        T: 'a,
        CM: 'a,
        H: 'a;

    set_iteration_funcs!();
}

impl<T, CM, H> Len for HashSet<T, CM, H> {
    fn len(&self) -> usize {
        self.map.len()
    }
}

impl<T, ST, CM, H> BitOr<&ST> for &HashSet<T, CM, H>
where
    T: Hash + Eq + Clone,
    ST: Set<T>,
    CM: CollectionMagnitude,
    H: Hasher<T>,
{
    bitor_fn!();
}

impl<T, ST, CM, H> BitAnd<&ST> for &HashSet<T, CM, H>
where
    T: Hash + Eq + Clone,
    ST: Set<T>,
    CM: CollectionMagnitude,
    H: Hasher<T>,
{
    bitand_fn!();
}

impl<T, ST, CM, H> BitXor<&ST> for &HashSet<T, CM, H>
where
    T: Hash + Eq + Clone,
    ST: Set<T>,
    CM: CollectionMagnitude,
    H: Hasher<T>,
{
    bitxor_fn!();
}

impl<T, ST, CM, H> Sub<&ST> for &HashSet<T, CM, H>
where
    T: Hash + Eq + Clone,
    ST: Set<T>,
    CM: CollectionMagnitude,
    H: Hasher<T>,
{
    sub_fn!();
}

impl<T, CM, H> IntoIterator for HashSet<T, CM, H> {
    into_iter_fn!();
}

impl<'a, T, CM, H> IntoIterator for &'a HashSet<T, CM, H> {
    into_iter_ref_fn!();
}

impl<T, ST, CM, H> PartialEq<ST> for HashSet<T, CM, H>
where
    T: Eq,
    ST: Set<T>,
    CM: CollectionMagnitude,
    H: Hasher<T>,
{
    partial_eq_fn!();
}

impl<T, CM, H> Eq for HashSet<T, CM, H>
where
    T: Eq,
    CM: CollectionMagnitude,
    H: Hasher<T>,
{
}

impl<T, CM, H> Debug for HashSet<T, CM, H>
where
    T: Debug,
{
    debug_fn!();
}

#[cfg(feature = "serde")]
impl<T, CM, H> Serialize for HashSet<T, CM, H>
where
    T: Serialize,
{
    serialize_fn!();
}
