use crate::sets::decl_macros::set_iteration_trait_funcs;
use core::hash::BuildHasher;

/// Common iteration abstractions for sets.
pub trait SetIteration<T>: IntoIterator<Item = T> {
    /// The type of the iterator returned by [`Self::iter`].
    type Iterator<'a>: Iterator<Item = &'a T>
    where
        Self: 'a,
        T: 'a;

    #[doc = include_str!("../doc_snippets/iter.md")]
    #[must_use]
    fn iter(&self) -> Self::Iterator<'_>;
}

#[cfg(feature = "std")]
impl<T, BH> SetIteration<T> for std::collections::HashSet<T, BH>
where
    BH: BuildHasher,
{
    type Iterator<'a>
        = std::collections::hash_set::Iter<'a, T>
    where
        T: 'a,
        BH: 'a;

    set_iteration_trait_funcs!();
}

#[cfg(feature = "std")]
impl<T> SetIteration<T> for alloc::collections::BTreeSet<T> {
    type Iterator<'a>
        = std::collections::btree_set::Iter<'a, T>
    where
        T: 'a;

    set_iteration_trait_funcs!();
}

impl<T, BH> SetIteration<T> for hashbrown::hash_set::HashSet<T, BH>
where
    BH: BuildHasher,
{
    type Iterator<'a>
        = hashbrown::hash_set::Iter<'a, T>
    where
        T: 'a,
        BH: 'a;

    set_iteration_trait_funcs!();
}
