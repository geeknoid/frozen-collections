use crate::traits::{SetExtras, SetIteration, SetQuery};
use core::hash::{BuildHasher, Hash};

#[cfg(feature = "std")]
use core::borrow::Borrow;

/// Common abstractions for sets.
pub trait Set<T, Q: ?Sized = T>: SetQuery<Q> + SetIteration<T> + SetExtras<T, Q> {}

#[cfg(feature = "std")]
impl<T, Q, BH> Set<T, Q> for std::collections::HashSet<T, BH>
where
    T: Eq + Hash + Borrow<Q>,
    Q: Hash + Eq,
    BH: BuildHasher,
{
}

#[cfg(feature = "std")]
impl<T, Q> Set<T, Q> for alloc::collections::BTreeSet<T>
where
    T: Ord + Borrow<Q>,
    Q: Ord,
{
}

impl<T, Q, BH> Set<T, Q> for hashbrown::hash_set::HashSet<T, BH>
where
    T: Hash + Eq,
    Q: ?Sized + Hash + hashbrown::Equivalent<T>,
    BH: BuildHasher,
{
}
