use crate::hashers::BridgeHasher;
use crate::inline_maps::InlineHashMapNoCollisions;
use crate::sets::decl_macros::{
    bitand_trait_funcs, bitor_trait_funcs, bitxor_trait_funcs, common_primary_funcs, debug_trait_funcs, hash_primary_funcs,
    into_iterator_ref_trait_funcs, into_iterator_trait_funcs, partial_eq_trait_funcs, set_extras_trait_funcs, set_iteration_trait_funcs,
    set_query_trait_funcs, sub_trait_funcs,
};
use crate::sets::{IntoIter, Iter};
use crate::traits::{CollectionMagnitude, Hasher, Len, Set, SetExtras, SetIteration, SetOps, SetQuery, SmallCollection};
use core::fmt::Debug;
use core::hash::Hash;
use core::ops::{BitAnd, BitOr, BitXor, Sub};
use equivalent::Equivalent;

use crate::maps::decl_macros::len_trait_funcs;
#[cfg(feature = "serde")]
use {
    crate::sets::decl_macros::serialize_trait_funcs,
    serde::ser::SerializeSeq,
    serde::{Serialize, Serializer},
};

/// A general-purpose set implemented using a hash table which doesn't tolerate hash collisions.
///
#[doc = include_str!("../doc_snippets/private_api_warning.md")]
#[doc = include_str!("../doc_snippets/about.md")]
#[doc = include_str!("../doc_snippets/hash_warning.md")]
///
/// # Type Parameters
///
/// - `T`: The value type.
/// - `CM`: The magnitude of the set, one of [`SmallCollection`](SmallCollection), [`MediumCollection`](crate::traits::MediumCollection), or [`LargeCollection`](crate::traits::LargeCollection).
/// - `SZ`: The number of entries in the set.
/// - `NHS`: The number of hash table slots.
/// - `H`: The hasher to generate hash codes.
#[derive(Clone)]
pub struct InlineHashSetNoCollisions<T, const SZ: usize, const NHS: usize, CM = SmallCollection, H = BridgeHasher> {
    map: InlineHashMapNoCollisions<T, (), SZ, NHS, CM, H>,
}

impl<T, const SZ: usize, const NHS: usize, CM, H> InlineHashSetNoCollisions<T, SZ, NHS, CM, H>
where
    CM: CollectionMagnitude,
{
    /// Creates a frozen set.
    #[must_use]
    pub const fn new(map: InlineHashMapNoCollisions<T, (), SZ, NHS, CM, H>) -> Self {
        Self { map }
    }

    hash_primary_funcs!();
    common_primary_funcs!(const_len);
}

impl<T, Q, const SZ: usize, const NHS: usize, CM, H> Set<T, Q> for InlineHashSetNoCollisions<T, SZ, NHS, CM, H>
where
    Q: ?Sized + Equivalent<T>,
    CM: CollectionMagnitude,
    H: Hasher<Q>,
{
}

impl<T, Q, const SZ: usize, const NHS: usize, CM, H> SetExtras<T, Q> for InlineHashSetNoCollisions<T, SZ, NHS, CM, H>
where
    Q: ?Sized + Equivalent<T>,
    CM: CollectionMagnitude,
    H: Hasher<Q>,
{
    set_extras_trait_funcs!();
}

impl<T, Q, const SZ: usize, const NHS: usize, CM, H> SetQuery<Q> for InlineHashSetNoCollisions<T, SZ, NHS, CM, H>
where
    Q: ?Sized + Equivalent<T>,
    CM: CollectionMagnitude,
    H: Hasher<Q>,
{
    set_query_trait_funcs!();
}

impl<T, const SZ: usize, const NHS: usize, CM, H> SetIteration<T> for InlineHashSetNoCollisions<T, SZ, NHS, CM, H>
where
    CM: CollectionMagnitude,
{
    type Iterator<'a>
        = Iter<'a, T>
    where
        T: 'a,
        CM: 'a,
        H: 'a;

    set_iteration_trait_funcs!();
}

impl<T, const SZ: usize, const NHS: usize, CM, H> Len for InlineHashSetNoCollisions<T, SZ, NHS, CM, H>
where
    CM: CollectionMagnitude,
{
    len_trait_funcs!();
}

impl<T, ST, const SZ: usize, const NHS: usize, CM, H> BitOr<&ST> for &InlineHashSetNoCollisions<T, SZ, NHS, CM, H>
where
    T: Hash + Eq + Clone,
    ST: Set<T>,
    CM: CollectionMagnitude,
    H: Hasher<T>,
{
    bitor_trait_funcs!();
}

impl<T, ST, const SZ: usize, const NHS: usize, CM, H> BitAnd<&ST> for &InlineHashSetNoCollisions<T, SZ, NHS, CM, H>
where
    T: Hash + Eq + Clone,
    ST: Set<T>,
    CM: CollectionMagnitude,
    H: Hasher<T>,
{
    bitand_trait_funcs!();
}

impl<T, ST, const SZ: usize, const NHS: usize, CM, H> BitXor<&ST> for &InlineHashSetNoCollisions<T, SZ, NHS, CM, H>
where
    T: Hash + Eq + Clone,
    ST: Set<T>,
    CM: CollectionMagnitude,
    H: Hasher<T>,
{
    bitxor_trait_funcs!();
}

impl<T, ST, const SZ: usize, const NHS: usize, CM, H> Sub<&ST> for &InlineHashSetNoCollisions<T, SZ, NHS, CM, H>
where
    T: Hash + Eq + Clone,
    ST: Set<T>,
    CM: CollectionMagnitude,
    H: Hasher<T>,
{
    sub_trait_funcs!();
}

impl<T, const SZ: usize, const NHS: usize, CM, H> IntoIterator for InlineHashSetNoCollisions<T, SZ, NHS, CM, H>
where
    CM: CollectionMagnitude,
{
    into_iterator_trait_funcs!();
}

impl<'a, T, const SZ: usize, const NHS: usize, CM, H> IntoIterator for &'a InlineHashSetNoCollisions<T, SZ, NHS, CM, H>
where
    CM: CollectionMagnitude,
{
    into_iterator_ref_trait_funcs!();
}

impl<T, ST, const SZ: usize, const NHS: usize, CM, H> PartialEq<ST> for InlineHashSetNoCollisions<T, SZ, NHS, CM, H>
where
    T: PartialEq,
    ST: SetQuery<T>,
    CM: CollectionMagnitude,
    H: Hasher<T>,
{
    partial_eq_trait_funcs!();
}

impl<T, const SZ: usize, const NHS: usize, CM, H> Eq for InlineHashSetNoCollisions<T, SZ, NHS, CM, H>
where
    T: Eq,
    CM: CollectionMagnitude,
    H: Hasher<T>,
{
}

impl<T, const SZ: usize, const NHS: usize, CM, H> Debug for InlineHashSetNoCollisions<T, SZ, NHS, CM, H>
where
    T: Debug,
    CM: CollectionMagnitude,
    H: Hasher<T>,
{
    debug_trait_funcs!();
}

#[cfg(feature = "serde")]
impl<T, const SZ: usize, const NHS: usize, CM, H> Serialize for InlineHashSetNoCollisions<T, SZ, NHS, CM, H>
where
    T: Serialize,
    CM: CollectionMagnitude,
{
    serialize_trait_funcs!();
}
