use crate::traits::{Len, MapIteration, MapQuery};
use crate::utils::has_duplicates;
use core::borrow::Borrow;
use core::hash::{BuildHasher, Hash};
use core::mem::MaybeUninit;

/// Common abstractions for maps.
pub trait Map<K, V, Q: ?Sized = K>: MapQuery<K, V, Q> + MapIteration<K, V> + Len {
    /// Gets multiple mutable values from the map.
    #[must_use]
    fn get_many_mut<const N: usize>(&mut self, keys: [&Q; N]) -> Option<[&mut V; N]>;
}

#[cfg(feature = "std")]
impl<K, V, Q, BH> Map<K, V, Q> for std::collections::HashMap<K, V, BH>
where
    K: Hash + Eq + Borrow<Q>,
    Q: ?Sized + Hash + Eq,
    BH: BuildHasher,
{
    fn get_many_mut<const N: usize>(&mut self, keys: [&Q; N]) -> Option<[&mut V; N]> {
        if has_duplicates(keys.iter()) {
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
impl<K, V, Q> Map<K, V, Q> for std::collections::BTreeMap<K, V>
where
    K: Ord + Borrow<Q>,
    Q: Ord,
{
    fn get_many_mut<const N: usize>(&mut self, keys: [&Q; N]) -> Option<[&mut V; N]> {
        if crate::utils::has_duplicates_slow(&keys) {
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

impl<K, V, Q, BH> Map<K, V, Q> for hashbrown::HashMap<K, V, BH>
where
    K: Hash + Eq + Borrow<Q>,
    Q: Hash + Eq,
    BH: BuildHasher,
{
    fn get_many_mut<const N: usize>(&mut self, keys: [&Q; N]) -> Option<[&mut V; N]> {
        if has_duplicates(keys.iter()) {
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
