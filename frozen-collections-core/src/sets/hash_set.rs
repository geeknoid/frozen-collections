use crate::hashers::BridgeHasher;
use crate::maps::HashMap;
use crate::sets::decl_macros::{
    bitand_fn, bitor_fn, bitxor_fn, debug_fn, into_iter_fn, into_iter_ref_fn, partial_eq_fn,
    set_boilerplate, set_iterator_boilerplate, sub_fn,
};
use crate::sets::{IntoIter, Iter};
use crate::traits::{CollectionMagnitude, Len};
use crate::traits::{Hasher, MapIterator, Set, SetIterator};
use ahash::RandomState;
use alloc::vec::Vec;
use core::borrow::Borrow;
use core::fmt::Debug;
use core::hash::Hash;
use core::ops::{BitAnd, BitOr, BitXor, Sub};

/// A general purpose set implemented using a hash table.
///
/// # Compatibility Note
///
/// This type is an implementation detail of the `frozen-collections` crate.
/// This API is therefore not stable and may change at any time.
#[derive(Clone)]
pub struct HashSet<T, CM, H = BridgeHasher<RandomState>> {
    map: HashMap<T, (), CM, H>,
}

impl<T, CM, H> HashSet<T, CM, H>
where
    T: Eq,
    CM: CollectionMagnitude,
    H: Hasher<T>,
{
    #[allow(clippy::missing_errors_doc)]
    pub fn new(entries: Vec<T>, hasher: H) -> Result<Self, String> {
        Ok(Self {
            map: HashMap::new(entries.into_iter().map(|x| (x, ())).collect(), hasher)?,
        })
    }
}

impl<T, CM, H> HashSet<T, CM, H>
where
    CM: CollectionMagnitude,
{
    #[inline]
    #[must_use]
    pub fn get<Q>(&self, value: &Q) -> Option<&T>
    where
        T: Borrow<Q>,
        H: Hasher<Q>,
        Q: ?Sized + Eq,
    {
        Some(self.map.get_key_value(value)?.0)
    }

    #[inline]
    #[must_use]
    pub fn contains<Q>(&self, value: &Q) -> bool
    where
        T: Borrow<Q>,
        H: Hasher<Q>,
        Q: ?Sized + Eq,
    {
        self.get(value).is_some()
    }

    /// Returns the hasher for this set.
    #[must_use]
    pub const fn hasher(&self) -> &H {
        self.map.hasher()
    }
}

impl<T, CM, H> Len for HashSet<T, CM, H> {
    fn len(&self) -> usize {
        self.map.len()
    }
}

impl<T, CM, H> Debug for HashSet<T, CM, H>
where
    T: Debug,
{
    debug_fn!();
}

impl<T, CM, H> Default for HashSet<T, CM, H>
where
    H: Default,
{
    fn default() -> Self {
        Self {
            map: HashMap::default(),
        }
    }
}

impl<T, CM, H> IntoIterator for HashSet<T, CM, H> {
    into_iter_fn!();
}

impl<'a, T, CM, H> IntoIterator for &'a HashSet<T, CM, H> {
    into_iter_ref_fn!();
}

impl<T, CM, H> SetIterator<T> for HashSet<T, CM, H> {
    type Iterator<'a>
        = Iter<'a, T>
    where
        T: 'a,
        CM: 'a,
        H: 'a;

    set_iterator_boilerplate!();
}

impl<T, CM, H> Set<T> for HashSet<T, CM, H>
where
    T: Eq,
    CM: CollectionMagnitude,
    H: Hasher<T>,
{
    set_boilerplate!();
}

impl<T, ST, CM, H> BitOr<&ST> for &HashSet<T, CM, H>
where
    T: Hash + Eq + Clone,
    ST: Set<T>,
    CM: CollectionMagnitude,
    H: Hasher<T> + Default,
{
    bitor_fn!(H);
}

impl<T, ST, CM, H> BitAnd<&ST> for &HashSet<T, CM, H>
where
    T: Hash + Eq + Clone,
    ST: Set<T>,
    CM: CollectionMagnitude,
    H: Hasher<T> + Default,
{
    bitand_fn!(H);
}

impl<T, ST, CM, H> BitXor<&ST> for &HashSet<T, CM, H>
where
    T: Hash + Eq + Clone,
    ST: Set<T>,
    CM: CollectionMagnitude,
    H: Hasher<T> + Default,
{
    bitxor_fn!(H);
}

impl<T, ST, CM, H> Sub<&ST> for &HashSet<T, CM, H>
where
    T: Hash + Eq + Clone,
    ST: Set<T>,
    CM: CollectionMagnitude,
    H: Hasher<T> + Default,
{
    sub_fn!(H);
}

impl<T, ST, CM, H> PartialEq<ST> for HashSet<T, CM, H>
where
    T: Eq,
    ST: Set<T>,
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
