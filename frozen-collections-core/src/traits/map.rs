use crate::traits::{Len, MapIterator};
use crate::utils::{find_duplicate, slow_find_duplicate};

#[cfg(feature = "std")]
use core::hash::{BuildHasher, Hash};
use core::mem::MaybeUninit;

/// Common abstractions for maps.
pub trait Map<K, V>: Len + MapIterator<K, V> {
    /// Checks whether a particular value is present in the map.
    #[must_use]
    fn contains_key(&self, key: &K) -> bool;

    /// Gets a value from the map.
    #[must_use]
    fn get(&self, key: &K) -> Option<&V>;

    /// Gets a key and value from the map.
    #[must_use]
    fn get_key_value(&self, key: &K) -> Option<(&K, &V)>;

    /// Gets a mutable value from the map.
    #[must_use]
    fn get_mut(&mut self, key: &K) -> Option<&mut V>;

    /// Gets multiple mutable values from the map.
    #[must_use]
    fn get_many_mut<const N: usize>(&mut self, keys: [&K; N]) -> Option<[&mut V; N]>;
}

#[cfg(feature = "std")]
impl<K, V, BH> Map<K, V> for std::collections::HashMap<K, V, BH>
where
    K: Hash + Eq,
    BH: BuildHasher,
{
    #[inline]
    fn contains_key(&self, key: &K) -> bool {
        Self::contains_key(self, key)
    }

    #[inline]
    fn get(&self, key: &K) -> Option<&V> {
        Self::get(self, key)
    }

    #[inline]
    fn get_key_value(&self, key: &K) -> Option<(&K, &V)> {
        Self::get_key_value(self, key)
    }

    #[inline]
    fn get_mut(&mut self, key: &K) -> Option<&mut V> {
        Self::get_mut(self, key)
    }

    fn get_many_mut<const N: usize>(&mut self, keys: [&K; N]) -> Option<[&mut V; N]> {
        if find_duplicate(keys.iter()).is_some() {
            return None;
        }

        let mut result: MaybeUninit<[&mut V; N]> = MaybeUninit::uninit();
        let p = result.as_mut_ptr();
        let x: *mut Self = self;
        unsafe {
            for (i, key) in keys.iter().enumerate() {
                (*p)[i] = (*x).get_mut(key)?;
            }

            Some(result.assume_init())
        }
    }
}

#[cfg(feature = "std")]
impl<K, V> Map<K, V> for std::collections::BTreeMap<K, V>
where
    K: Ord,
{
    #[inline]
    fn contains_key(&self, key: &K) -> bool {
        Self::contains_key(self, key)
    }

    #[inline]
    fn get(&self, key: &K) -> Option<&V> {
        Self::get(self, key)
    }

    #[inline]
    fn get_key_value(&self, key: &K) -> Option<(&K, &V)> {
        Self::get_key_value(self, key)
    }

    #[inline]
    fn get_mut(&mut self, key: &K) -> Option<&mut V> {
        Self::get_mut(self, key)
    }

    fn get_many_mut<const N: usize>(&mut self, keys: [&K; N]) -> Option<[&mut V; N]> {
        if slow_find_duplicate(&keys).is_some() {
            return None;
        }

        let mut result: MaybeUninit<[&mut V; N]> = MaybeUninit::uninit();
        let p = result.as_mut_ptr();
        let x: *mut Self = self;
        unsafe {
            for (i, key) in keys.iter().enumerate() {
                (*p)[i] = (*x).get_mut(key)?;
            }

            Some(result.assume_init())
        }
    }
}

impl<K, V, BH> Map<K, V> for hashbrown::HashMap<K, V, BH>
where
    K: Hash + Eq,
    BH: BuildHasher,
{
    #[inline]
    fn contains_key(&self, key: &K) -> bool {
        Self::contains_key(self, key)
    }

    #[inline]
    fn get(&self, key: &K) -> Option<&V> {
        Self::get(self, key)
    }

    #[inline]
    fn get_key_value(&self, key: &K) -> Option<(&K, &V)> {
        Self::get_key_value(self, key)
    }

    #[inline]
    fn get_mut(&mut self, key: &K) -> Option<&mut V> {
        Self::get_mut(self, key)
    }

    fn get_many_mut<const N: usize>(&mut self, keys: [&K; N]) -> Option<[&mut V; N]> {
        if find_duplicate(keys.iter()).is_some() {
            return None;
        }

        let mut result: MaybeUninit<[&mut V; N]> = MaybeUninit::uninit();
        let p = result.as_mut_ptr();
        let x: *mut Self = self;
        unsafe {
            for (i, key) in keys.iter().enumerate() {
                (*p)[i] = (*x).get_mut(*key)?;
            }

            Some(result.assume_init())
        }
    }
}
