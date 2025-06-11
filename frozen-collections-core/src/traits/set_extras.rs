use crate::sets::decl_macros::set_extras_trait_funcs;
use core::hash::{BuildHasher, Hash};

#[cfg(feature = "std")]
use core::borrow::Borrow;

/// Common query abstractions for sets.
pub trait SetExtras<T, Q: ?Sized = T> {
    #[doc = include_str!("../doc_snippets/get_from_set.md")]
    #[must_use]
    fn get(&self, value: &Q) -> Option<&T>;
}

#[cfg(feature = "std")]
impl<T, Q, BH> SetExtras<T, Q> for std::collections::HashSet<T, BH>
where
    T: Eq + Hash + Borrow<Q>,
    Q: ?Sized + Hash + Eq,
    BH: BuildHasher,
{
    set_extras_trait_funcs!();
}

#[cfg(feature = "std")]
impl<T, Q> SetExtras<T, Q> for alloc::collections::BTreeSet<T>
where
    T: Ord + Borrow<Q>,
    Q: Ord,
{
    set_extras_trait_funcs!();
}

impl<T, Q, BH> SetExtras<T, Q> for hashbrown::hash_set::HashSet<T, BH>
where
    T: Hash + Eq,
    Q: ?Sized + Hash + hashbrown::Equivalent<T>,
    BH: BuildHasher,
{
    set_extras_trait_funcs!();
}
