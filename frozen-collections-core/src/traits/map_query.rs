use core::hash::{BuildHasher, Hash};

/// Common query abstractions for maps.
pub trait MapQuery<K, V, Q: ?Sized = K> {
    #[doc = include_str!("../doc_snippets/get.md")]
    #[must_use]
    fn get(&self, key: &Q) -> Option<&V>;

    #[doc = include_str!("../doc_snippets/get_mut.md")]
    #[must_use]
    fn get_mut(&mut self, key: &Q) -> Option<&mut V>;

    #[doc = include_str!("../doc_snippets/get_key_value.md")]
    #[must_use]
    fn get_key_value(&self, key: &Q) -> Option<(&K, &V)>;

    #[doc = include_str!("../doc_snippets/contains_key.md")]
    #[must_use]
    fn contains_key(&self, key: &Q) -> bool {
        self.get(key).is_some()
    }
}

#[cfg(feature = "std")]
impl<K, V, Q, BH> MapQuery<K, V, Q> for std::collections::HashMap<K, V, BH>
where
    K: Hash + Eq + core::borrow::Borrow<Q>,
    Q: ?Sized + Hash + Eq,
    BH: BuildHasher,
{
    #[inline]
    fn get(&self, key: &Q) -> Option<&V> {
        self.get(key)
    }

    #[inline]
    fn get_mut(&mut self, key: &Q) -> Option<&mut V> {
        self.get_mut(key)
    }

    #[inline]
    fn get_key_value(&self, key: &Q) -> Option<(&K, &V)> {
        self.get_key_value(key)
    }

    #[inline]
    fn contains_key(&self, key: &Q) -> bool {
        self.contains_key(key)
    }
}

#[cfg(feature = "std")]
impl<K, V, Q> MapQuery<K, V, Q> for std::collections::BTreeMap<K, V>
where
    K: Ord + core::borrow::Borrow<Q>,
    Q: Ord,
{
    #[inline]
    fn get(&self, key: &Q) -> Option<&V> {
        self.get(key)
    }

    #[inline]
    fn get_mut(&mut self, key: &Q) -> Option<&mut V> {
        self.get_mut(key)
    }

    #[inline]
    fn get_key_value(&self, key: &Q) -> Option<(&K, &V)> {
        self.get_key_value(key)
    }

    #[inline]
    fn contains_key(&self, key: &Q) -> bool {
        self.contains_key(key)
    }
}

impl<K, V, Q, BH> MapQuery<K, V, Q> for hashbrown::HashMap<K, V, BH>
where
    K: Hash + Eq + core::borrow::Borrow<Q>,
    Q: Hash + Eq,
    BH: BuildHasher,
{
    #[inline]
    fn get(&self, key: &Q) -> Option<&V> {
        self.get(key)
    }

    #[inline]
    fn get_mut(&mut self, key: &Q) -> Option<&mut V> {
        self.get_mut(key)
    }

    #[inline]
    fn get_key_value(&self, key: &Q) -> Option<(&K, &V)> {
        self.get_key_value(key)
    }

    #[inline]
    fn contains_key(&self, key: &Q) -> bool {
        self.contains_key(key)
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
