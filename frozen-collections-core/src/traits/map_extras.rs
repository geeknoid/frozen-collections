use core::hash::{BuildHasher, Hash};

#[cfg(feature = "std")]
use {
    crate::maps::decl_macros::{get_disjoint_mut_funcs, map_extras_trait_funcs},
    core::borrow::Borrow,
};

/// Extra abstractions for maps.
pub trait MapExtras<K, V, Q: ?Sized = K> {
    #[doc = include_str!("../doc_snippets/get_key_value.md")]
    #[must_use]
    fn get_key_value(&self, key: &Q) -> Option<(&K, &V)>;

    #[doc = include_str!("../doc_snippets/get_disjoint_mut.md")]
    #[must_use]
    fn get_disjoint_mut<const N: usize>(&mut self, keys: [&Q; N]) -> [Option<&mut V>; N]
    where
        Q: Eq;

    #[doc = include_str!("../doc_snippets/get_disjoint_unchecked_mut.md")]
    #[must_use]
    unsafe fn get_disjoint_unchecked_mut<const N: usize>(&mut self, keys: [&Q; N]) -> [Option<&mut V>; N];
}

#[cfg(feature = "std")]
impl<K, V, Q, BH> MapExtras<K, V, Q> for std::collections::HashMap<K, V, BH>
where
    K: Hash + Eq + Borrow<Q>,
    Q: ?Sized + Hash + Eq,
    BH: BuildHasher,
{
    map_extras_trait_funcs!();
}

#[cfg(feature = "std")]
impl<K, V, Q> MapExtras<K, V, Q> for alloc::collections::BTreeMap<K, V>
where
    K: Ord + Borrow<Q>,
    Q: ?Sized + Ord,
{
    fn get_key_value(&self, key: &Q) -> Option<(&K, &V)> {
        self.get_key_value(key)
    }

    fn get_disjoint_mut<const N: usize>(&mut self, keys: [&Q; N]) -> [Option<&mut V>; N] {
        get_disjoint_mut_funcs!(@safe_body, self, keys);
    }

    unsafe fn get_disjoint_unchecked_mut<const N: usize>(&mut self, keys: [&Q; N]) -> [Option<&mut V>; N] {
        get_disjoint_mut_funcs!(@unsafe_body, self, keys);
    }
}

impl<K, V, Q, BH> MapExtras<K, V, Q> for hashbrown::HashMap<K, V, BH>
where
    K: Hash + Eq,
    Q: ?Sized + Hash + hashbrown::Equivalent<K>,
    BH: BuildHasher,
{
    fn get_key_value(&self, key: &Q) -> Option<(&K, &V)> {
        self.get_key_value(key)
    }

    fn get_disjoint_mut<const N: usize>(&mut self, keys: [&Q; N]) -> [Option<&mut V>; N] {
        self.get_many_mut(keys)
    }

    unsafe fn get_disjoint_unchecked_mut<const N: usize>(&mut self, keys: [&Q; N]) -> [Option<&mut V>; N] {
        // SAFETY: This method is unsafe because it assumes that the keys are disjoint and valid.
        unsafe { self.get_many_unchecked_mut(keys) }
    }
}
