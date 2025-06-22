use core::hash::BuildHasher;

/// Common iteration abstractions for maps.
pub trait MapIteration<K, V>: IntoIterator<Item = (K, V)> {
    /// The type of the iterator returned by [`Self::iter`].
    type Iterator<'a>: Iterator<Item = (&'a K, &'a V)>
    where
        Self: 'a,
        K: 'a,
        V: 'a;

    /// The type of the iterator returned by [`Self::keys`].
    type KeyIterator<'a>: Iterator<Item = &'a K>
    where
        Self: 'a,
        K: 'a;

    /// The type of the iterator returned by [`Self::values`].
    type ValueIterator<'a>: Iterator<Item = &'a V>
    where
        Self: 'a,
        V: 'a;

    /// The type of the iterator returned by [`Self::into_keys`].
    type IntoKeyIterator: Iterator<Item = K>;

    /// The type of the iterator returned by [`Self::into_values`].
    type IntoValueIterator: Iterator<Item = V>;

    /// The type of the mutable iterator returned by [`Self::iter_mut`].
    type MutIterator<'a>: Iterator<Item = (&'a K, &'a mut V)>
    where
        Self: 'a,
        K: 'a,
        V: 'a;

    /// The type of the mutable iterator returned by [`Self::values_mut`].
    type ValueMutIterator<'a>: Iterator<Item = &'a mut V>
    where
        Self: 'a,
        V: 'a;

    #[doc = include_str!("../doc_snippets/iter.md")]
    #[must_use]
    fn iter(&self) -> Self::Iterator<'_>;

    #[doc = include_str!("../doc_snippets/iter_mut.md")]
    #[must_use]
    fn iter_mut(&mut self) -> Self::MutIterator<'_>;

    #[doc = include_str!("../doc_snippets/keys.md")]
    #[must_use]
    fn keys(&self) -> Self::KeyIterator<'_>;

    #[doc = include_str!("../doc_snippets/into_keys.md")]
    #[must_use]
    fn into_keys(self) -> Self::IntoKeyIterator;

    #[doc = include_str!("../doc_snippets/values.md")]
    #[must_use]
    fn values(&self) -> Self::ValueIterator<'_>;

    #[doc = include_str!("../doc_snippets/values_mut.md")]
    #[must_use]
    fn values_mut(&mut self) -> Self::ValueMutIterator<'_>;

    #[doc = include_str!("../doc_snippets/into_values.md")]
    #[must_use]
    fn into_values(self) -> Self::IntoValueIterator;
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
        self.iter()
    }

    fn iter_mut(&mut self) -> Self::MutIterator<'_> {
        self.iter_mut()
    }

    fn keys(&self) -> Self::KeyIterator<'_> {
        self.keys()
    }

    fn into_keys(self) -> Self::IntoKeyIterator {
        self.into_keys()
    }

    fn values(&self) -> Self::ValueIterator<'_> {
        self.values()
    }

    fn values_mut(&mut self) -> Self::ValueMutIterator<'_> {
        self.values_mut()
    }

    fn into_values(self) -> Self::IntoValueIterator {
        self.into_values()
    }
}

#[cfg(feature = "std")]
impl<K, V> MapIteration<K, V> for alloc::collections::BTreeMap<K, V> {
    type Iterator<'a>
        = alloc::collections::btree_map::Iter<'a, K, V>
    where
        K: 'a,
        V: 'a;

    type KeyIterator<'a>
        = alloc::collections::btree_map::Keys<'a, K, V>
    where
        K: 'a,
        V: 'a;

    type ValueIterator<'a>
        = alloc::collections::btree_map::Values<'a, K, V>
    where
        K: 'a,
        V: 'a;

    type IntoKeyIterator = alloc::collections::btree_map::IntoKeys<K, V>;
    type IntoValueIterator = alloc::collections::btree_map::IntoValues<K, V>;

    type MutIterator<'a>
        = alloc::collections::btree_map::IterMut<'a, K, V>
    where
        K: 'a,
        V: 'a;

    type ValueMutIterator<'a>
        = alloc::collections::btree_map::ValuesMut<'a, K, V>
    where
        K: 'a,
        V: 'a;

    fn iter(&self) -> Self::Iterator<'_> {
        self.iter()
    }

    fn iter_mut(&mut self) -> Self::MutIterator<'_> {
        self.iter_mut()
    }

    fn keys(&self) -> Self::KeyIterator<'_> {
        self.keys()
    }

    fn into_keys(self) -> Self::IntoKeyIterator {
        self.into_keys()
    }

    fn values(&self) -> Self::ValueIterator<'_> {
        self.values()
    }

    fn values_mut(&mut self) -> Self::ValueMutIterator<'_> {
        self.values_mut()
    }

    fn into_values(self) -> Self::IntoValueIterator {
        self.into_values()
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
        self.iter()
    }

    fn iter_mut(&mut self) -> Self::MutIterator<'_> {
        self.iter_mut()
    }

    fn keys(&self) -> Self::KeyIterator<'_> {
        self.keys()
    }

    fn into_keys(self) -> Self::IntoKeyIterator {
        self.into_keys()
    }

    fn values(&self) -> Self::ValueIterator<'_> {
        self.values()
    }

    fn values_mut(&mut self) -> Self::ValueMutIterator<'_> {
        self.values_mut()
    }

    fn into_values(self) -> Self::IntoValueIterator {
        self.into_values()
    }
}
