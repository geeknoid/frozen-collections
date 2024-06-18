use core::hash::{BuildHasher, Hash};

/// Common query abstractions for maps.
pub trait MapQuery<K, V, Q: ?Sized = K> {
    /// Checks whether a particular value is present in the map.
    #[inline]
    #[must_use]
    fn contains_key(&self, key: &Q) -> bool {
        self.get(key).is_some()
    }

    /// Gets a value from the map.
    #[must_use]
    fn get(&self, key: &Q) -> Option<&V>;

    /// Gets a key and value from the map.
    #[must_use]
    fn get_key_value(&self, key: &Q) -> Option<(&K, &V)>;

    /// Gets a mutable value from the map.
    #[must_use]
    fn get_mut(&mut self, key: &Q) -> Option<&mut V>;
}

#[cfg(feature = "std")]
impl<K, V, Q, BH> MapQuery<K, V, Q> for std::collections::HashMap<K, V, BH>
where
    K: Hash + Eq + core::borrow::Borrow<Q>,
    Q: ?Sized + Hash + Eq,
    BH: BuildHasher,
{
    #[inline]
    fn contains_key(&self, key: &Q) -> bool {
        Self::contains_key(self, key)
    }

    #[inline]
    fn get(&self, key: &Q) -> Option<&V> {
        Self::get(self, key)
    }

    #[inline]
    fn get_key_value(&self, key: &Q) -> Option<(&K, &V)> {
        Self::get_key_value(self, key)
    }

    #[inline]
    fn get_mut(&mut self, key: &Q) -> Option<&mut V> {
        Self::get_mut(self, key)
    }
}

#[cfg(feature = "std")]
impl<K, V, Q> MapQuery<K, V, Q> for std::collections::BTreeMap<K, V>
where
    K: Ord + core::borrow::Borrow<Q>,
    Q: Ord,
{
    #[inline]
    fn contains_key(&self, key: &Q) -> bool {
        Self::contains_key(self, key)
    }

    #[inline]
    fn get(&self, key: &Q) -> Option<&V> {
        Self::get(self, key)
    }

    #[inline]
    fn get_key_value(&self, key: &Q) -> Option<(&K, &V)> {
        Self::get_key_value(self, key)
    }

    #[inline]
    fn get_mut(&mut self, key: &Q) -> Option<&mut V> {
        Self::get_mut(self, key)
    }
}

impl<K, V, Q, BH> MapQuery<K, V, Q> for hashbrown::HashMap<K, V, BH>
where
    K: Hash + Eq + core::borrow::Borrow<Q>,
    Q: Hash + Eq,
    BH: BuildHasher,
{
    #[inline]
    fn contains_key(&self, key: &Q) -> bool {
        Self::contains_key(self, key)
    }

    #[inline]
    fn get(&self, key: &Q) -> Option<&V> {
        Self::get(self, key)
    }

    #[inline]
    fn get_key_value(&self, key: &Q) -> Option<(&K, &V)> {
        Self::get_key_value(self, key)
    }

    #[inline]
    fn get_mut(&mut self, key: &Q) -> Option<&mut V> {
        Self::get_mut(self, key)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use hashbrown::HashMap;

    #[test]
    fn test_object_safe() {
        let m: &dyn MapQuery<_, _, _> = &HashMap::from([(1, 1), (2, 2), (3, 3)]);

        assert!(m.contains_key(&1));
    }
}
