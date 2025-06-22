use crate::sets::decl_macros::set_query_trait_funcs;
use crate::traits::Len;
use core::hash::{BuildHasher, Hash};

#[cfg(feature = "std")]
use core::borrow::Borrow;

/// Common query abstractions for sets.
pub trait SetQuery<Q: ?Sized>: Len {
    #[doc = include_str!("../doc_snippets/contains.md")]
    #[must_use]
    fn contains(&self, value: &Q) -> bool;
}

#[cfg(feature = "std")]
impl<T, Q, BH> SetQuery<Q> for std::collections::HashSet<T, BH>
where
    T: Hash + Eq + Borrow<Q>,
    Q: ?Sized + Hash + Eq,
    BH: BuildHasher,
{
    set_query_trait_funcs!();
}

#[cfg(feature = "std")]
impl<T, Q> SetQuery<Q> for std::collections::BTreeSet<T>
where
    T: Ord + Borrow<Q>,
    Q: Ord,
{
    set_query_trait_funcs!();
}

impl<T, Q, BH> SetQuery<Q> for hashbrown::hash_set::HashSet<T, BH>
where
    T: Hash + Eq,
    Q: ?Sized + Hash + hashbrown::Equivalent<T>,
    BH: BuildHasher,
{
    set_query_trait_funcs!();
}

#[cfg(test)]
mod tests {
    use super::*;
    use hashbrown::HashSet;

    #[test]
    fn test_object_safe() {
        let s: &dyn SetQuery<_> = &HashSet::from([1, 2, 3]);

        assert!(s.contains(&1));
    }
}
