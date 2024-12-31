use core::borrow::Borrow;
use core::hash::{BuildHasher, Hash};

/// Common query abstractions for sets.
pub trait SetQuery<T, Q: ?Sized = T> {
    /// Checks whether a particular value is present in the set.
    #[must_use]
    #[inline]
    fn contains(&self, value: &Q) -> bool {
        self.get(value).is_some()
    }

    /// Gets a reference to a value in the set.
    #[must_use]
    fn get(&self, value: &Q) -> Option<&T>;
}

#[cfg(feature = "std")]
impl<T, Q, BH> SetQuery<T, Q> for std::collections::HashSet<T, BH>
where
    T: Eq + Hash + Borrow<Q>,
    Q: Hash + Eq,
    BH: BuildHasher,
{
    #[inline]
    fn get(&self, value: &Q) -> Option<&T> {
        self.get(value)
    }
}

#[cfg(feature = "std")]
impl<T, Q> SetQuery<T, Q> for std::collections::BTreeSet<T>
where
    T: Ord + Borrow<Q>,
    Q: Ord,
{
    #[inline]
    fn get(&self, value: &Q) -> Option<&T> {
        self.get(value)
    }
}

impl<T, Q, BH> SetQuery<T, Q> for hashbrown::hash_set::HashSet<T, BH>
where
    T: Hash + Eq + Borrow<Q>,
    Q: Hash + Eq,
    BH: BuildHasher,
{
    #[inline]
    fn get(&self, value: &Q) -> Option<&T> {
        self.get(value)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use hashbrown::HashSet;

    #[test]
    fn test_object_safe() {
        let s: &dyn SetQuery<_, _> = &HashSet::from([1, 2, 3]);

        assert!(s.contains(&1));
    }
}
