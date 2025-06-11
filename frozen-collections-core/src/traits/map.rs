use crate::traits::{MapExtras, MapIteration, MapQuery};
use core::hash::{BuildHasher, Hash};

#[cfg(feature = "std")]
use core::borrow::Borrow;

/// Common abstractions for maps.
pub trait Map<K, V, Q: ?Sized = K>: MapQuery<Q, V> + MapIteration<K, V> + MapExtras<K, V, Q> {}

#[cfg(feature = "std")]
impl<K, V, Q, BH> Map<K, V, Q> for std::collections::HashMap<K, V, BH>
where
    K: Hash + Eq + Borrow<Q>,
    Q: ?Sized + Hash + Eq,
    BH: BuildHasher,
{
}

#[cfg(feature = "std")]
impl<K, V, Q> Map<K, V, Q> for alloc::collections::BTreeMap<K, V>
where
    K: Ord + Borrow<Q>,
    Q: Ord,
{
}

impl<K, V, Q, BH> Map<K, V, Q> for hashbrown::HashMap<K, V, BH>
where
    K: Hash + Eq,
    Q: ?Sized + Hash + Eq + hashbrown::Equivalent<K>,
    BH: BuildHasher,
{
}
