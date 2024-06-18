use bitvec::macros::internal::funty::Fundamental;
use core::fmt::{Debug, Formatter, Result};
use core::mem::MaybeUninit;
use core::ops::Range;

use crate::traits::{CollectionMagnitude, Len};

/// A hash table that stores its entries inline.
///
/// # Type Parameters
///
/// - `T`: The data held in the hash table.
/// - `CM`: The magnitude of the collection.
/// - `SZ`: The length of the map.
/// - `NHS`: The number of hash table slots.
///
/// # Compatibility Note
///
/// This type is an implementation detail of the `frozen-collections` crate.
/// This API is therefore not stable and may change at any time.
#[derive(Clone)]
pub struct InlineHashTable<T, CM, const SZ: usize, const NHS: usize> {
    mask: u64,
    slots: [HashTableSlot<CM>; NHS],
    pub(crate) entries: [T; SZ],
}

impl<T, CM, const SZ: usize, const NHS: usize> InlineHashTable<T, CM, SZ, NHS> {
    pub const fn new(slots: [HashTableSlot<CM>; NHS], entries: [T; SZ]) -> Self {
        Self {
            mask: (NHS - 1) as u64,
            slots,
            entries,
        }
    }
}

#[derive(Clone, Copy)]
pub struct HashTableSlot<CM> {
    min_index: CM,
    max_index: CM,
}

impl<CM> HashTableSlot<CM> {
    pub const fn new(min_index: CM, max_index: CM) -> Self {
        Self {
            min_index,
            max_index,
        }
    }
}

impl<T, CM, const SZ: usize, const NHS: usize> InlineHashTable<T, CM, SZ, NHS>
where
    CM: CollectionMagnitude,
{
    #[inline]
    pub fn find(&self, hash_code: u64, mut eq: impl FnMut(&T) -> bool) -> Option<&T> {
        let hash_slot_index = (hash_code & self.mask).as_usize();
        let hash_slot = unsafe { self.slots.get_unchecked(hash_slot_index) };
        let range: Range<usize> = hash_slot.min_index.into()..hash_slot.max_index.into();
        let entries = unsafe { self.entries.get_unchecked(range) };
        entries.iter().find(|entry| eq(entry))
    }

    #[inline]
    pub fn find_mut(&mut self, hash_code: u64, mut eq: impl FnMut(&T) -> bool) -> Option<&mut T> {
        let hash_slot_index = (hash_code & self.mask).as_usize();
        let hash_slot = unsafe { self.slots.get_unchecked(hash_slot_index) };
        let range: Range<usize> = hash_slot.min_index.into()..hash_slot.max_index.into();
        let entries = unsafe { self.entries.get_unchecked_mut(range) };
        entries.iter_mut().find(|entry| eq(entry))
    }

    pub fn get_many_mut<const N: usize>(
        &mut self,
        hashes: [u64; N],
        mut eq: impl FnMut(usize, &T) -> bool,
    ) -> Option<[&mut T; N]> {
        let mut result: MaybeUninit<[&mut T; N]> = MaybeUninit::uninit();
        let p = result.as_mut_ptr();
        let x: *mut Self = self;

        for (i, hash_code) in hashes.into_iter().enumerate() {
            unsafe {
                (*p)[i] = (*x).find_mut(hash_code, |entry| eq(i, entry))?;
            }
        }

        let result = unsafe { result.assume_init() };

        // make sure there are no duplicates
        for i in 0..result.len() {
            for j in 0..i {
                let p0 = result[i] as *const T;
                let p1 = result[j] as *const T;

                if p0 == p1 {
                    return None;
                }
            }
        }

        Some(result)
    }
}

impl<T, CM, const SZ: usize, const NHS: usize> Len for InlineHashTable<T, CM, SZ, NHS> {
    #[inline]
    fn len(&self) -> usize {
        self.entries.len()
    }
}

impl<T, CM, const SZ: usize, const NHS: usize> Debug for InlineHashTable<T, CM, SZ, NHS>
where
    T: Debug,
{
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        f.debug_set().entries(&self.entries).finish()
    }
}
