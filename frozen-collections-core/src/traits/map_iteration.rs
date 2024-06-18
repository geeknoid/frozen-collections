use core::hash::BuildHasher;

/// Common iteration abstractions for maps.
pub trait MapIteration<K, V>: IntoIterator<Item = (K, V)> {
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

    /// An iterator visiting all entries in arbitrary order.
    #[must_use]
    fn iter(&self) -> Self::Iterator<'_>;

    /// An iterator visiting all keys in arbitrary order.
    #[must_use]
    fn keys(&self) -> Self::KeyIterator<'_>;

    /// An iterator visiting all values in arbitrary order.
    #[must_use]
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
}

#[cfg(feature = "std")]
impl<K, V, BH> MapIteration<K, V> for std::collections::HashMap<K, V, BH>
where
    BH: BuildHasher,
{
    type Iterator<'a>
        = std::collections::hash_map::Iter<'a, K, V>
    where
        K: 'a,
        V: 'a,
        BH: 'a;

    type KeyIterator<'a>
        = std::collections::hash_map::Keys<'a, K, V>
    where
        K: 'a,
        V: 'a,
        BH: 'a;

    type ValueIterator<'a>
        = std::collections::hash_map::Values<'a, K, V>
    where
        K: 'a,
        V: 'a,
        BH: 'a;

    type IntoKeyIterator = std::collections::hash_map::IntoKeys<K, V>;
    type IntoValueIterator = std::collections::hash_map::IntoValues<K, V>;
    type MutIterator<'a>
        = std::collections::hash_map::IterMut<'a, K, V>
    where
        K: 'a,
        V: 'a,
        BH: 'a;

    type ValueMutIterator<'a>
        = std::collections::hash_map::ValuesMut<'a, K, V>
    where
        K: 'a,
        V: 'a,
        BH: 'a;

    fn iter(&self) -> Self::Iterator<'_> {
        Self::iter(self)
    }

    fn keys(&self) -> Self::KeyIterator<'_> {
        Self::keys(self)
    }

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
}

#[cfg(feature = "std")]
impl<K, V> MapIteration<K, V> for std::collections::BTreeMap<K, V> {
    type Iterator<'a>
        = std::collections::btree_map::Iter<'a, K, V>
    where
        K: 'a,
        V: 'a;

    type KeyIterator<'a>
        = std::collections::btree_map::Keys<'a, K, V>
    where
        K: 'a,
        V: 'a;

    type ValueIterator<'a>
        = std::collections::btree_map::Values<'a, K, V>
    where
        K: 'a,
        V: 'a;

    type IntoKeyIterator = std::collections::btree_map::IntoKeys<K, V>;
    type IntoValueIterator = std::collections::btree_map::IntoValues<K, V>;
    type MutIterator<'a>
        = std::collections::btree_map::IterMut<'a, K, V>
    where
        K: 'a,
        V: 'a;

    type ValueMutIterator<'a>
        = std::collections::btree_map::ValuesMut<'a, K, V>
    where
        K: 'a,
        V: 'a;

    fn iter(&self) -> Self::Iterator<'_> {
        Self::iter(self)
    }

    fn keys(&self) -> Self::KeyIterator<'_> {
        Self::keys(self)
    }

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
}

impl<K, V, BH> MapIteration<K, V> for hashbrown::HashMap<K, V, BH>
where
    BH: BuildHasher,
{
    type Iterator<'a>
        = hashbrown::hash_map::Iter<'a, K, V>
    where
        K: 'a,
        V: 'a,
        BH: 'a;

    type KeyIterator<'a>
        = hashbrown::hash_map::Keys<'a, K, V>
    where
        K: 'a,
        V: 'a,
        BH: 'a;

    type ValueIterator<'a>
        = hashbrown::hash_map::Values<'a, K, V>
    where
        K: 'a,
        V: 'a,
        BH: 'a;

    type IntoKeyIterator = hashbrown::hash_map::IntoKeys<K, V>;
    type IntoValueIterator = hashbrown::hash_map::IntoValues<K, V>;
    type MutIterator<'a>
        = hashbrown::hash_map::IterMut<'a, K, V>
    where
        K: 'a,
        V: 'a,
        BH: 'a;

    type ValueMutIterator<'a>
        = hashbrown::hash_map::ValuesMut<'a, K, V>
    where
        K: 'a,
        V: 'a,
        BH: 'a;

    fn iter(&self) -> Self::Iterator<'_> {
        Self::iter(self)
    }

    fn keys(&self) -> Self::KeyIterator<'_> {
        Self::keys(self)
    }

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
}
