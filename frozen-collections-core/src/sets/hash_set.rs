use crate::hashers::BridgeHasher;
use crate::maps::HashMap;
use crate::maps::decl_macros::len_trait_funcs;
use crate::sets::decl_macros::{
    bitand_trait_funcs, bitor_trait_funcs, bitxor_trait_funcs, common_primary_funcs, debug_trait_funcs, hash_primary_funcs,
    into_iterator_ref_trait_funcs, into_iterator_trait_funcs, partial_eq_trait_funcs, set_extras_trait_funcs, set_iteration_trait_funcs,
    set_query_trait_funcs, sub_trait_funcs,
};
use crate::sets::{IntoIter, Iter};
use crate::traits::{CollectionMagnitude, Len, SetExtras, SetOps, SetQuery, SmallCollection};
use crate::traits::{Hasher, Set, SetIteration};
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
    CM: CollectionMagnitude,
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

    hash_primary_funcs!();
    common_primary_funcs!(non_const_len);
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

impl<T, Q, CM, H> SetExtras<T, Q> for HashSet<T, CM, H>
where
    Q: ?Sized + Equivalent<T>,
    CM: CollectionMagnitude,
    H: Hasher<Q>,
{
    set_extras_trait_funcs!();
}

impl<T, Q, CM, H> SetQuery<Q> for HashSet<T, CM, H>
where
    Q: ?Sized + Equivalent<T>,
    CM: CollectionMagnitude,
    H: Hasher<Q>,
{
    set_query_trait_funcs!();
}

impl<T, CM, H> SetIteration<T> for HashSet<T, CM, H>
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

impl<T, CM, H> Len for HashSet<T, CM, H>
where
    CM: CollectionMagnitude,
{
    len_trait_funcs!();
}

impl<T, ST, CM, H> BitOr<&ST> for &HashSet<T, CM, H>
where
    T: Hash + Eq + Clone,
    ST: Set<T>,
    CM: CollectionMagnitude,
    H: Hasher<T>,
{
    bitor_trait_funcs!();
}

impl<T, ST, CM, H> BitAnd<&ST> for &HashSet<T, CM, H>
where
    T: Hash + Eq + Clone,
    ST: Set<T>,
    CM: CollectionMagnitude,
    H: Hasher<T>,
{
    bitand_trait_funcs!();
}

impl<T, ST, CM, H> BitXor<&ST> for &HashSet<T, CM, H>
where
    T: Hash + Eq + Clone,
    ST: Set<T>,
    CM: CollectionMagnitude,
    H: Hasher<T>,
{
    bitxor_trait_funcs!();
}

impl<T, ST, CM, H> Sub<&ST> for &HashSet<T, CM, H>
where
    T: Hash + Eq + Clone,
    ST: Set<T>,
    CM: CollectionMagnitude,
    H: Hasher<T>,
{
    sub_trait_funcs!();
}

impl<T, CM, H> IntoIterator for HashSet<T, CM, H>
where
    CM: CollectionMagnitude,
{
    into_iterator_trait_funcs!();
}

impl<'a, T, CM, H> IntoIterator for &'a HashSet<T, CM, H>
where
    CM: CollectionMagnitude,
{
    into_iterator_ref_trait_funcs!();
}

impl<T, ST, CM, H> PartialEq<ST> for HashSet<T, CM, H>
where
    T: PartialEq,
    ST: SetQuery<T>,
    CM: CollectionMagnitude,
    H: Hasher<T>,
{
    partial_eq_trait_funcs!();
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
    CM: CollectionMagnitude,
{
    debug_trait_funcs!();
}

#[cfg(feature = "serde")]
impl<T, CM, H> Serialize for HashSet<T, CM, H>
where
    T: Serialize,
    CM: CollectionMagnitude,
{
    serialize_trait_funcs!();
}
