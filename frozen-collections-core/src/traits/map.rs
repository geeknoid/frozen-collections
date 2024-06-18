use std::collections::{BTreeMap, HashMap};
use std::hash::{BuildHasher, Hash};

use crate::traits::Len;

/// Common abstractions for maps.
pub trait Map<K, V>: Len {
    type Iterator<'a>: Iterator<Item = (&'a K, &'a V)>
    where
        Self: 'a,
        K: 'a,
        V: 'a;

    type KeyIterator<'a>: Iterator<Item = &'a K>
    where
        Self: 'a,
        K: 'a;

    type ValueIterator<'a>: Iterator<Item = &'a V>
    where
        Self: 'a,
        V: 'a;

    type IntoKeyIterator: Iterator<Item = K>;
    type IntoValueIterator: Iterator<Item = V>;

    type MutIterator<'a>: Iterator<Item = (&'a K, &'a mut V)>
    where
        Self: 'a,
        K: 'a,
        V: 'a;

    type ValueMutIterator<'a>: Iterator<Item = &'a mut V>
    where
        Self: 'a,
        V: 'a;

    /// An iterator visiting all elements in arbitrary order.
    #[must_use]
    fn iter(&self) -> Self::Iterator<'_>;

    /// An iterator visiting all keys in arbitrary order.
    fn keys(&self) -> Self::KeyIterator<'_>;

    /// An iterator visiting all values in arbitrary order.
    fn values(&self) -> Self::ValueIterator<'_>;

    /// A consuming iterator visiting all keys in arbitrary order.
    #[must_use]
    fn into_keys(self) -> Self::IntoKeyIterator;

    /// A consuming iterator visiting all values in arbitrary order.
    #[must_use]
    fn into_values(self) -> Self::IntoValueIterator;

    /// An iterator producing mutable references to all entries in arbitrary order.
    #[must_use]
    fn iter_mut(&mut self) -> Self::MutIterator<'_>;

    /// An iterator visiting all values mutably in arbitrary order.
    #[must_use]
    fn values_mut(&mut self) -> Self::ValueMutIterator<'_>;

    /// Checks whether a particular value is present in the map.
    #[must_use]
    fn contains_key(&self, key: &K) -> bool;

    /// Gets a value from the map.
    #[must_use]
    fn get(&self, key: &K) -> Option<&V>;
}

impl<K, V, BH> Map<K, V> for HashMap<K, V, BH>
where
    K: Hash + Eq,
    BH: BuildHasher,
{
    type Iterator<'a> = std::collections::hash_map::Iter<'a, K, V>
    where
        K: 'a,
        V: 'a,
        BH: 'a;

    type KeyIterator<'a> = std::collections::hash_map::Keys<'a, K, V>
    where
        K: 'a,
        V: 'a,
        BH: 'a;

    type ValueIterator<'a> = std::collections::hash_map::Values<'a, K, V>
    where
        K: 'a,
        V: 'a,
        BH: 'a;

    type IntoKeyIterator = std::collections::hash_map::IntoKeys<K, V>;
    type IntoValueIterator = std::collections::hash_map::IntoValues<K, V>;
    type MutIterator<'a> = std::collections::hash_map::IterMut<'a, K, V>
    where
        K: 'a,
        V: 'a,
        BH: 'a;

    type ValueMutIterator<'a> = std::collections::hash_map::ValuesMut<'a, K, V>
    where
        K: 'a,
        V: 'a,
        BH: 'a;

    #[inline]
    fn iter(&self) -> Self::Iterator<'_> {
        Self::iter(self)
    }

    #[inline]
    fn keys(&self) -> Self::KeyIterator<'_> {
        Self::keys(self)
    }

    #[inline]
    fn values(&self) -> Self::ValueIterator<'_> {
        Self::values(self)
    }

    fn into_keys(self) -> Self::IntoKeyIterator {
        Self::into_keys(self)
    }

    fn into_values(self) -> Self::IntoValueIterator {
        Self::into_values(self)
    }

    fn iter_mut(&mut self) -> Self::MutIterator<'_> {
        Self::iter_mut(self)
    }

    fn values_mut(&mut self) -> Self::ValueMutIterator<'_> {
        self.values_mut()
    }

    #[inline]
    fn contains_key(&self, key: &K) -> bool {
        Self::contains_key(self, key)
    }

    #[inline]
    fn get(&self, key: &K) -> Option<&V> {
        Self::get(self, key)
    }
}

impl<K, V> Map<K, V> for BTreeMap<K, V>
where
    K: Ord,
{
    type Iterator<'a> = std::collections::btree_map::Iter<'a, K, V>
    where
        K: 'a,
        V: 'a;

    type KeyIterator<'a> = std::collections::btree_map::Keys<'a, K, V>
    where
        K: 'a,
        V: 'a;

    type ValueIterator<'a> = std::collections::btree_map::Values<'a, K, V>
    where
        K: 'a,
        V: 'a;

    type IntoKeyIterator = std::collections::btree_map::IntoKeys<K, V>;
    type IntoValueIterator = std::collections::btree_map::IntoValues<K, V>;
    type MutIterator<'a> = std::collections::btree_map::IterMut<'a, K, V>
    where
        K: 'a,
        V: 'a;

    type ValueMutIterator<'a> = std::collections::btree_map::ValuesMut<'a, K, V>
    where
        K: 'a,
        V: 'a;

    #[inline]
    fn iter(&self) -> Self::Iterator<'_> {
        Self::iter(self)
    }

    #[inline]
    fn keys(&self) -> Self::KeyIterator<'_> {
        Self::keys(self)
    }

    #[inline]
    fn values(&self) -> Self::ValueIterator<'_> {
        Self::values(self)
    }

    fn into_keys(self) -> Self::IntoKeyIterator {
        Self::into_keys(self)
    }

    fn into_values(self) -> Self::IntoValueIterator {
        Self::into_values(self)
    }

    fn iter_mut(&mut self) -> Self::MutIterator<'_> {
        Self::iter_mut(self)
    }

    fn values_mut(&mut self) -> Self::ValueMutIterator<'_> {
        self.values_mut()
    }

    #[inline]
    fn contains_key(&self, key: &K) -> bool {
        Self::contains_key(self, key)
    }

    #[inline]
    fn get(&self, key: &K) -> Option<&V> {
        Self::get(self, key)
    }
}
