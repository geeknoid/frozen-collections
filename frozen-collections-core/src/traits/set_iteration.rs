use core::hash::BuildHasher;

/// Common iteration abstractions for sets.
pub trait SetIteration<T>: IntoIterator<Item = T> {
    type Iterator<'a>: Iterator<Item = &'a T>
    where
        Self: 'a,
        T: 'a;

    /// An iterator visiting all entries in arbitrary order.
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

    #[inline]
    fn iter(&self) -> Self::Iterator<'_> {
        Self::iter(self)
    }
}

#[cfg(feature = "std")]
impl<T> SetIteration<T> for std::collections::BTreeSet<T> {
    type Iterator<'a>
        = std::collections::btree_set::Iter<'a, T>
    where
        T: 'a;

    #[inline]
    fn iter(&self) -> Self::Iterator<'_> {
        Self::iter(self)
    }
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

    #[inline]
    fn iter(&self) -> Self::Iterator<'_> {
        Self::iter(self)
    }
}
