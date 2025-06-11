use crate::traits::{Len, MapIteration, MapQuery};
use core::borrow::Borrow;
use core::hash::{BuildHasher, Hash};

#[cfg(feature = "std")]
use core::mem::MaybeUninit;

#[cfg(feature = "std")]
use crate::utils::cold;

/// Common abstractions for maps.
pub trait Map<K, V, Q: ?Sized = K>: MapQuery<K, V, Q> + MapIteration<K, V> + Len {
    #[doc = include_str!("../doc_snippets/get_disjoint_mut.md")]
    #[must_use]
    fn get_disjoint_mut<const N: usize>(&mut self, keys: [&Q; N]) -> [Option<&mut V>; N]
    where
        Q: Eq;

    #[doc = include_str!("../doc_snippets/get_disjoint_unchecked_mut.md")]
    #[must_use]
    unsafe fn get_disjoint_unchecked_mut<const N: usize>(&mut self, keys: [&Q; N]) -> [Option<&mut V>; N];
}

#[cfg(feature = "std")]
impl<K, V, Q, BH> Map<K, V, Q> for std::collections::HashMap<K, V, BH>
where
    K: Hash + Eq + Borrow<Q>,
    Q: ?Sized + Hash + Eq,
    BH: BuildHasher,
{
    fn get_disjoint_mut<const N: usize>(&mut self, keys: [&Q; N]) -> [Option<&mut V>; N] {
        self.get_disjoint_mut(keys)
    }

    unsafe fn get_disjoint_unchecked_mut<const N: usize>(&mut self, keys: [&Q; N]) -> [Option<&mut V>; N] {
        // SAFETY: This method is unsafe because it assumes that the keys are disjoint and valid.
        unsafe { self.get_disjoint_unchecked_mut(keys) }
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

        // SAFETY: We've validated that the keys are disjoint.
        unsafe { self.get_disjoint_unchecked_mut(keys) }
    }

    unsafe fn get_disjoint_unchecked_mut<const N: usize>(&mut self, keys: [&Q; N]) -> [Option<&mut V>; N] {
        let mut result: MaybeUninit<[Option<&mut V>; N]> = MaybeUninit::uninit();
        let p = result.as_mut_ptr();
        let x: *mut Self = self;

        for (i, key) in keys.iter().enumerate() {
            // SAFETY: keys are known valid
            let v = unsafe { (*x).get_mut(*key) };

            // SAFETY: p is known valid since it was allocated above
            unsafe {
                (*p)[i] = v;
            }
        }

        // SAFETY: We have filled the MaybeUninit with values, so it is now initialized
        unsafe { result.assume_init() }
    }
}

impl<K, V, Q, BH> Map<K, V, Q> for hashbrown::HashMap<K, V, BH>
where
    K: Hash + Eq + Borrow<Q>,
    Q: Hash + Eq,
    BH: BuildHasher,
{
    fn get_disjoint_mut<const N: usize>(&mut self, keys: [&Q; N]) -> [Option<&mut V>; N] {
        self.get_many_mut(keys)
    }

    unsafe fn get_disjoint_unchecked_mut<const N: usize>(&mut self, keys: [&Q; N]) -> [Option<&mut V>; N] {
        // SAFETY: This method is unsafe because it assumes that the keys are disjoint and valid.
        unsafe { self.get_many_unchecked_mut(keys) }
    }
}
