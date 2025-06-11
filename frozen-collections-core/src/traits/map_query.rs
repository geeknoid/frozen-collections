use crate::maps::decl_macros::map_query_trait_funcs;
use crate::traits::Len;
use core::hash::{BuildHasher, Hash};

#[cfg(feature = "std")]
use core::borrow::Borrow;

/// Common query abstractions for maps.
pub trait MapQuery<Q: ?Sized, V>: Len {
    #[doc = include_str!("../doc_snippets/get.md")]
    #[must_use]
    fn get(&self, key: &Q) -> Option<&V>;

    #[doc = include_str!("../doc_snippets/get_mut.md")]
    #[must_use]
    fn get_mut(&mut self, key: &Q) -> Option<&mut V>;

    #[doc = include_str!("../doc_snippets/contains_key.md")]
    #[must_use]
    fn contains_key(&self, key: &Q) -> bool;
}

#[cfg(feature = "std")]
impl<K, V, Q, BH> MapQuery<Q, V> for std::collections::HashMap<K, V, BH>
where
    K: Hash + Eq + Borrow<Q>,
    Q: ?Sized + Hash + Eq,
    BH: BuildHasher,
{
    map_query_trait_funcs!();
}

#[cfg(feature = "std")]
impl<K, V, Q> MapQuery<Q, V> for alloc::collections::BTreeMap<K, V>
where
    K: Ord + Borrow<Q>,
    Q: ?Sized + Ord,
{
    map_query_trait_funcs!();
}

impl<K, V, Q, BH> MapQuery<Q, V> for hashbrown::HashMap<K, V, BH>
where
    K: Hash + Eq,
    Q: ?Sized + Hash + Eq + hashbrown::Equivalent<K>,
    BH: BuildHasher,
{
    map_query_trait_funcs!();
}

#[cfg(test)]
mod tests {
    use super::*;
    use hashbrown::HashMap;

    #[test]
    fn test_object_safe() {
        let m: &dyn MapQuery<_, _> = &HashMap::from([(1, 1), (2, 2), (3, 3)]);

        assert!(m.contains_key(&1));
    }
}
