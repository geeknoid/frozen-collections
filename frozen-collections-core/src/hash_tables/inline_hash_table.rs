use crate::hash_tables::HashTableSlot;
use crate::traits::{CollectionMagnitude, SmallCollection};
use core::ops::Range;

/// A hash table that stores its entries inline.
///
#[doc = include_str!("../doc_snippets/private_api_warning.md")]
///
/// # Type Parameters
///
/// - `T`: The data held in the hash table.
/// - `CM`: The magnitude of the collection.
/// - `SZ`: The number of entries in the hash table.
/// - `NHS`: The number of hash table slots. This must be a power of two.
///
/// This implementation always has a power-of-two number of hash slots. This speeds up
/// lookups by avoiding the need to perform a modulo operation.
#[derive(Clone)]
pub struct InlineHashTable<T, const SZ: usize, const NHS: usize, CM = SmallCollection> {
    mask: u64,
    slots: [HashTableSlot<CM>; NHS],
    pub(crate) entries: [T; SZ],
}

impl<T, const SZ: usize, const NHS: usize, CM> InlineHashTable<T, SZ, NHS, CM> {
    /// Creates a new hash table.
    ///
    /// This function assumes that the slots and processed entries are in proper order.
    pub const fn new_raw(slots: [HashTableSlot<CM>; NHS], processed_entries: [T; SZ]) -> Self {
        Self {
            mask: (NHS - 1) as u64,
            slots,
            entries: processed_entries,
        }
    }
}

impl<T, const SZ: usize, const NHS: usize, CM> InlineHashTable<T, SZ, NHS, CM>
where
    CM: CollectionMagnitude,
{
    #[inline]
    #[allow(clippy::cast_possible_truncation)]
    pub(crate) fn find(&self, hash_code: u64, mut eq: impl FnMut(&T) -> bool) -> Option<&T> {
        let hash_slot_index = (hash_code & self.mask) as usize;
        let hash_slot = unsafe { self.slots.get_unchecked(hash_slot_index) };
        let range: Range<usize> = hash_slot.min_index.into()..hash_slot.max_index.into();
        let entries = unsafe { self.entries.get_unchecked(range) };
        entries.iter().find(|entry| eq(entry))
    }

    #[inline]
    #[allow(clippy::cast_possible_truncation)]
    pub(crate) fn find_mut(
        &mut self,
        hash_code: u64,
        mut eq: impl FnMut(&T) -> bool,
    ) -> Option<&mut T> {
        let hash_slot_index = (hash_code & self.mask) as usize;
        let hash_slot = unsafe { self.slots.get_unchecked(hash_slot_index) };
        let range: Range<usize> = hash_slot.min_index.into()..hash_slot.max_index.into();
        let entries = unsafe { self.entries.get_unchecked_mut(range) };
        entries.iter_mut().find(|entry| eq(entry))
    }
}
