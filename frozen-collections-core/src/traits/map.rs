use crate::traits::{Len, MapIteration, MapQuery};
use core::borrow::Borrow;
use core::hash::{BuildHasher, Hash};

#[cfg(feature = "std")]
use core::mem::MaybeUninit;

#[cfg(feature = "std")]
use crate::utils::cold;

/// Common abstractions for maps.
pub trait Map<K, V, Q: ?Sized = K>: MapQuery<K, V, Q> + MapIteration<K, V> + Len {
    /// Gets multiple mutable values from the map.
    ///
    /// # Panics
    ///
    /// Panics if the same key is specified multiple times.
    #[must_use]
    fn get_disjoint_mut<const N: usize>(&mut self, keys: [&Q; N]) -> [Option<&mut V>; N];

    /// Gets multiple mutable values from the map.
    ///
    /// # Safety
    ///     
    /// Calling this method with overlapping keys is [undefined behavior](https://doc.rust-lang.org/reference/behavior-considered-undefined.html)
    /// even if the resulting references are not used.
    #[must_use]
    unsafe fn get_disjoint_unchecked_mut<const N: usize>(
        &mut self,
        keys: [&Q; N],
    ) -> [Option<&mut V>; N];
}

#[cfg(feature = "std")]
impl<K, V, Q, BH> Map<K, V, Q> for std::collections::HashMap<K, V, BH>
where
    K: Hash + Eq + Borrow<Q>,
    Q: ?Sized + Hash + Eq,
    BH: BuildHasher,
{
    fn get_disjoint_mut<const N: usize>(&mut self, keys: [&Q; N]) -> [Option<&mut V>; N] {
        Self::get_disjoint_mut(self, keys)
    }

    unsafe fn get_disjoint_unchecked_mut<const N: usize>(
        &mut self,
        keys: [&Q; N],
    ) -> [Option<&mut V>; N] {
        unsafe { Self::get_disjoint_unchecked_mut(self, keys) }
    }
}

#[cfg(feature = "std")]
impl<K, V, Q> Map<K, V, Q> for std::collections::BTreeMap<K, V>
where
    K: Ord + Borrow<Q>,
    Q: Ord,
{
    fn get_disjoint_mut<const N: usize>(&mut self, keys: [&Q; N]) -> [Option<&mut V>; N] {
        if crate::utils::has_duplicates_slow(&keys) {
            cold();
            panic!("duplicate keys found");
        }

        unsafe { self.get_disjoint_unchecked_mut(keys) }
    }

    unsafe fn get_disjoint_unchecked_mut<const N: usize>(
        &mut self,
        keys: [&Q; N],
    ) -> [Option<&mut V>; N] {
        let mut result: MaybeUninit<[Option<&mut V>; N]> = MaybeUninit::uninit();
        let p = result.as_mut_ptr();
        let x: *mut Self = self;
        unsafe {
            for (i, key) in keys.iter().enumerate() {
                (*p)[i] = (*x).get_mut(key);
            }

            result.assume_init()
        }
    }
}

impl<K, V, Q, BH> Map<K, V, Q> for hashbrown::HashMap<K, V, BH>
where
    K: Hash + Eq + Borrow<Q>,
    Q: Hash + Eq,
    BH: BuildHasher,
{
    fn get_disjoint_mut<const N: usize>(&mut self, keys: [&Q; N]) -> [Option<&mut V>; N] {
        Self::get_many_mut(self, keys)
    }

    unsafe fn get_disjoint_unchecked_mut<const N: usize>(
        &mut self,
        keys: [&Q; N],
    ) -> [Option<&mut V>; N] {
        unsafe { Self::get_many_unchecked_mut(self, keys) }
    }
}
