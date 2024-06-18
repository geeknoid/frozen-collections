use crate::traits::{Len, SetIteration, SetQuery};
use core::borrow::Borrow;
use core::hash::{BuildHasher, Hash};

/// Common abstractions for sets.
pub trait Set<T, Q: ?Sized = T>: SetQuery<T, Q> + SetIteration<T> + Len {}

#[cfg(feature = "std")]
impl<T, Q, BH> Set<T, Q> for std::collections::HashSet<T, BH>
where
    T: Eq + Hash + Borrow<Q>,
    Q: Hash + Eq,
    BH: BuildHasher,
{
}

#[cfg(feature = "std")]
impl<T, Q> Set<T, Q> for std::collections::BTreeSet<T>
where
    T: Ord + Borrow<Q>,
    Q: Ord,
{
}

impl<T, Q, BH> Set<T, Q> for hashbrown::hash_set::HashSet<T, BH>
where
    T: Hash + Eq + Borrow<Q>,
    Q: Hash + Eq,
    BH: BuildHasher,
{
}
