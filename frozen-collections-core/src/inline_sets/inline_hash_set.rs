use core::borrow::Borrow;
use core::fmt::Debug;
use core::hash::Hash;
use core::ops::{BitAnd, BitOr, BitXor, Sub};

use crate::inline_maps::InlineHashMap;
use crate::sets::decl_macros::{
    bitand_fn, bitor_fn, bitxor_fn, debug_fn, into_iter_fn, into_iter_ref_fn, partial_eq_fn,
    set_boilerplate, set_iterator_boilerplate, sub_fn,
};
use crate::sets::{IntoIter, Iter};
use crate::traits::{CollectionMagnitude, Hasher, Len, MapIterator, Set, SetIterator};

/// A general purpose set implemented using a hash table.
///
/// # Type Parameters
///
/// - `T`: The value type.
/// - `CM`: The magnitude of the set, one of [`SmallCollection`](crate::traits::SmallCollection), [`MediumCollection`](crate::traits::MediumCollection), or [`LargeCollection`](crate::traits::LargeCollection).
/// - `SZ`: The number of entries in the set.
/// - `NHS`: The number of hash table slots.
/// - `H`: The hasher to generate hash codes.
///
/// # Compatibility Note
///
/// This type is an implementation detail of the `frozen-collections` crate.
/// This API is therefore not stable and may change at any time.
#[derive(Clone)]
pub struct InlineHashSet<T, CM, const SZ: usize, const NHS: usize, H> {
    map: InlineHashMap<T, (), CM, SZ, NHS, H>,
}

impl<T, CM, const SZ: usize, const NHS: usize, H> InlineHashSet<T, CM, SZ, NHS, H> {
    pub const fn new(map: InlineHashMap<T, (), CM, SZ, NHS, H>) -> Self {
        Self { map }
    }
}

impl<T, CM, const SZ: usize, const NHS: usize, H> InlineHashSet<T, CM, SZ, NHS, H>
where
    CM: CollectionMagnitude,
{
    #[inline]
    #[must_use]
    pub fn get<Q>(&self, value: &Q) -> Option<&T>
    where
        T: Borrow<Q>,
        Q: ?Sized + Eq,
        H: Hasher<Q>,
    {
        Some(self.map.get_key_value(value)?.0)
    }

    #[inline]
    #[must_use]
    pub fn contains<Q>(&self, value: &Q) -> bool
    where
        T: Borrow<Q>,
        Q: ?Sized + Eq,
        H: Hasher<Q>,
    {
        self.get(value).is_some()
    }

    /// Returns the hasher for this set.
    #[must_use]
    pub const fn hasher(&self) -> &H {
        self.map.hasher()
    }
}

impl<T, CM, const SZ: usize, const NHS: usize, H> Len for InlineHashSet<T, CM, SZ, NHS, H> {
    fn len(&self) -> usize {
        SZ
    }
}

impl<T, CM, const SZ: usize, const NHS: usize, H> Debug for InlineHashSet<T, CM, SZ, NHS, H>
where
    T: Eq + Debug,
    CM: CollectionMagnitude,
    H: Hasher<T>,
{
    debug_fn!();
}

impl<T, CM, const SZ: usize, const NHS: usize, H> IntoIterator
    for InlineHashSet<T, CM, SZ, NHS, H>
{
    into_iter_fn!();
}

impl<'a, T, CM, const SZ: usize, const NHS: usize, H> IntoIterator
    for &'a InlineHashSet<T, CM, SZ, NHS, H>
where
    T: Eq,
    CM: CollectionMagnitude,
    H: Hasher<T>,
{
    into_iter_ref_fn!();
}

impl<T, CM, const SZ: usize, const NHS: usize, H> SetIterator<T>
    for InlineHashSet<T, CM, SZ, NHS, H>
{
    type Iterator<'a> = Iter<'a, T>
    where
        T: 'a,
        CM: 'a,
        H: 'a;

    set_iterator_boilerplate!();
}

impl<T, CM, const SZ: usize, const NHS: usize, H> Set<T> for InlineHashSet<T, CM, SZ, NHS, H>
where
    T: Eq,
    CM: CollectionMagnitude,
    H: Hasher<T>,
{
    set_boilerplate!();
}

impl<T, ST, CM, const SZ: usize, const NHS: usize, H> BitOr<&ST>
    for &InlineHashSet<T, CM, SZ, NHS, H>
where
    T: Hash + Eq + Clone,
    ST: Set<T>,
    CM: CollectionMagnitude,
    H: Hasher<T> + Default,
{
    bitor_fn!(H);
}

impl<T, ST, CM, const SZ: usize, const NHS: usize, H> BitAnd<&ST>
    for &InlineHashSet<T, CM, SZ, NHS, H>
where
    T: Hash + Eq + Clone,
    ST: Set<T>,
    CM: CollectionMagnitude,
    H: Hasher<T> + Default,
{
    bitand_fn!(H);
}

impl<T, ST, CM, const SZ: usize, const NHS: usize, H> BitXor<&ST>
    for &InlineHashSet<T, CM, SZ, NHS, H>
where
    T: Hash + Eq + Clone,
    ST: Set<T>,
    CM: CollectionMagnitude,
    H: Hasher<T> + Default,
{
    bitxor_fn!(H);
}

impl<T, ST, CM, const SZ: usize, const NHS: usize, H> Sub<&ST> for &InlineHashSet<T, CM, SZ, NHS, H>
where
    T: Hash + Eq + Clone,
    ST: Set<T>,
    CM: CollectionMagnitude,
    H: Hasher<T> + Default,
{
    sub_fn!(H);
}

impl<T, ST, CM, const SZ: usize, const NHS: usize, H> PartialEq<ST>
    for InlineHashSet<T, CM, SZ, NHS, H>
where
    T: Eq,
    ST: Set<T>,
    CM: CollectionMagnitude,
    H: Hasher<T>,
{
    partial_eq_fn!();
}

impl<T, CM, const SZ: usize, const NHS: usize, H> Eq for InlineHashSet<T, CM, SZ, NHS, H>
where
    T: Eq,
    CM: CollectionMagnitude,
    H: Hasher<T>,
{
}
