use crate::hashers::BridgeHasher;
use crate::inline_maps::InlineHashMap;
use crate::sets::decl_macros::{
    bitand_fn, bitor_fn, bitxor_fn, debug_fn, into_iter_fn, into_iter_ref_fn, partial_eq_fn,
    set_iteration_funcs, sub_fn,
};
use crate::sets::{IntoIter, Iter};
use crate::traits::{
    CollectionMagnitude, Hasher, Len, MapIteration, MapQuery, Set, SetIteration, SetOps, SetQuery,
    SmallCollection,
};
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

/// A general purpose set implemented using a hash table.
///
#[doc = include_str!("../doc_snippets/type_compat_warning.md")]
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
pub struct InlineHashSet<
    T,
    const SZ: usize,
    const NHS: usize,
    CM = SmallCollection,
    H = BridgeHasher,
> {
    map: InlineHashMap<T, (), SZ, NHS, CM, H>,
}

impl<T, const SZ: usize, const NHS: usize, CM, H> InlineHashSet<T, SZ, NHS, CM, H>
where
    CM: CollectionMagnitude,
    H: Hasher<T>,
{
    /// Creates a frozen set.
    #[must_use]
    pub const fn new(map: InlineHashMap<T, (), SZ, NHS, CM, H>) -> Self {
        Self { map }
    }
}

impl<T, Q, const SZ: usize, const NHS: usize, CM, H> Set<T, Q> for InlineHashSet<T, SZ, NHS, CM, H>
where
    Q: Eq + Equivalent<T>,
    CM: CollectionMagnitude,
    H: Hasher<Q>,
{
}

impl<T, Q, const SZ: usize, const NHS: usize, CM, H> SetQuery<T, Q>
    for InlineHashSet<T, SZ, NHS, CM, H>
where
    Q: Eq + Equivalent<T>,
    CM: CollectionMagnitude,
    H: Hasher<Q>,
{
    #[inline]
    fn get(&self, value: &Q) -> Option<&T> {
        Some(self.map.get_key_value(value)?.0)
    }
}

impl<T, const SZ: usize, const NHS: usize, CM, H> SetIteration<T>
    for InlineHashSet<T, SZ, NHS, CM, H>
{
    type Iterator<'a>
        = Iter<'a, T>
    where
        T: 'a,
        CM: 'a,
        H: 'a;

    set_iteration_funcs!();
}

impl<T, const SZ: usize, const NHS: usize, CM, H> Len for InlineHashSet<T, SZ, NHS, CM, H> {
    fn len(&self) -> usize {
        SZ
    }
}

impl<T, ST, const SZ: usize, const NHS: usize, CM, H> BitOr<&ST>
    for &InlineHashSet<T, SZ, NHS, CM, H>
where
    T: Hash + Eq + Clone,
    ST: Set<T>,
    CM: CollectionMagnitude,
    H: Hasher<T>,
{
    bitor_fn!(H);
}

impl<T, ST, const SZ: usize, const NHS: usize, CM, H> BitAnd<&ST>
    for &InlineHashSet<T, SZ, NHS, CM, H>
where
    T: Hash + Eq + Clone,
    ST: Set<T>,
    CM: CollectionMagnitude,
    H: Hasher<T>,
{
    bitand_fn!(H);
}

impl<T, ST, const SZ: usize, const NHS: usize, CM, H> BitXor<&ST>
    for &InlineHashSet<T, SZ, NHS, CM, H>
where
    T: Hash + Eq + Clone,
    ST: Set<T>,
    CM: CollectionMagnitude,
    H: Hasher<T>,
{
    bitxor_fn!(H);
}

impl<T, ST, const SZ: usize, const NHS: usize, CM, H> Sub<&ST> for &InlineHashSet<T, SZ, NHS, CM, H>
where
    T: Hash + Eq + Clone,
    ST: Set<T>,
    CM: CollectionMagnitude,
    H: Hasher<T>,
{
    sub_fn!(H);
}

impl<T, const SZ: usize, const NHS: usize, CM, H> IntoIterator
    for InlineHashSet<T, SZ, NHS, CM, H>
{
    into_iter_fn!();
}

impl<'a, T, const SZ: usize, const NHS: usize, CM, H> IntoIterator
    for &'a InlineHashSet<T, SZ, NHS, CM, H>
{
    into_iter_ref_fn!();
}

impl<T, ST, const SZ: usize, const NHS: usize, CM, H> PartialEq<ST>
    for InlineHashSet<T, SZ, NHS, CM, H>
where
    T: Eq,
    ST: Set<T>,
    CM: CollectionMagnitude,
    H: Hasher<T>,
{
    partial_eq_fn!();
}

impl<T, const SZ: usize, const NHS: usize, CM, H> Eq for InlineHashSet<T, SZ, NHS, CM, H>
where
    T: Eq,
    CM: CollectionMagnitude,
    H: Hasher<T>,
{
}

impl<T, const SZ: usize, const NHS: usize, CM, H> Debug for InlineHashSet<T, SZ, NHS, CM, H>
where
    T: Eq + Debug,
    CM: CollectionMagnitude,
    H: Hasher<T>,
{
    debug_fn!();
}

#[cfg(feature = "serde")]
impl<T, const SZ: usize, const NHS: usize, CM, H> Serialize for InlineHashSet<T, SZ, NHS, CM, H>
where
    T: Serialize,
{
    serialize_fn!();
}
