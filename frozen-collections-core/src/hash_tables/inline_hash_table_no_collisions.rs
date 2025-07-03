use crate::traits::{CollectionMagnitude, SmallCollection};

/// A specialized hash table that stores its entries inline and doesn't tolerate hash collisions.
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
#[derive(Clone, Debug)]
pub struct InlineHashTableNoCollisions<T, const SZ: usize, const NHS: usize, CM = SmallCollection> {
    pub(crate) entries: [T; SZ],
    slots: [CM; NHS],
    mask: u64,
}

impl<T, const SZ: usize, const NHS: usize, CM> InlineHashTableNoCollisions<T, SZ, NHS, CM> {
    /// Creates a new hash table.
    ///
    /// This function assumes that the slots and processed entries are in proper order.
    pub const fn new_raw(slots: [CM; NHS], processed_entries: [T; SZ]) -> Self {
        Self {
            mask: (NHS - 1) as u64,
            slots,
            entries: processed_entries,
        }
    }
}

impl<T, const SZ: usize, const NHS: usize, CM> InlineHashTableNoCollisions<T, SZ, NHS, CM>
where
    CM: CollectionMagnitude,
{
    #[inline]
    pub(crate) fn find(&self, hash_code: u64, eq: impl Fn(&T) -> bool) -> Option<&T> {
        #[expect(clippy::cast_possible_truncation, reason = "Truncation on 32 bit systems is fine")]
        let hash_slot_index = (hash_code & self.mask) as usize;

        // SAFETY: The hash slot index is guaranteed to be within bounds because of the modulo above
        let index_in_entries: usize = unsafe { (*self.slots.get_unchecked(hash_slot_index)).into() };

        if index_in_entries > 0 {
            // SAFETY: The range is guaranteed to be valid by construction
            let entry = unsafe { self.entries.get_unchecked(index_in_entries - 1) };

            if eq(entry) {
                return Some(entry);
            }
        }

        None
    }

    #[inline]
    pub(crate) fn find_mut(&mut self, hash_code: u64, eq: impl Fn(&T) -> bool) -> Option<&mut T> {
        #[expect(clippy::cast_possible_truncation, reason = "Truncation on 32 bit systems is fine")]
        let hash_slot_index = (hash_code & self.mask) as usize;

        // SAFETY: The hash slot index is guaranteed to be within bounds because of the modulo above
        let index_in_entries: usize = unsafe { (*self.slots.get_unchecked(hash_slot_index)).into() };

        if index_in_entries > 0 {
            // SAFETY: The range is guaranteed to be valid by construction
            let entry = unsafe { self.entries.get_unchecked_mut(index_in_entries - 1) };

            if eq(entry) {
                return Some(entry);
            }
        }

        None
    }

    #[inline]
    pub(crate) const fn len(&self) -> usize {
        self.entries.len()
    }
}
