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
    mask: u64,
    slots: [CM; NHS],
    pub(crate) entries: [T; SZ],
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
    pub(crate) fn contains(&self, hash_code: u64, eq: impl Fn(&T) -> bool) -> bool {
        #[expect(clippy::cast_possible_truncation, reason = "Truncation on 32 bit systems is fine")]
        let hash_slot_index = (hash_code & self.mask) as usize;

        // SAFETY: The hash slot index is guaranteed to be within bounds because of the modulo above
        let index_in_entries: usize = unsafe { (*self.slots.get_unchecked(hash_slot_index)).into() };

        if index_in_entries > 0 {
            // SAFETY: The range is guaranteed to be valid by construction
            let entry = unsafe { self.entries.get_unchecked(index_in_entries - 1) };
            return eq(entry);
        }

        false
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn contains_returns_false_for_empty_slot() {
        // 2 entries, 4 slots (power-of-two). Slots at indices 2 and 3 are empty (0).
        let table: InlineHashTableNoCollisions<i32, 2, 4, SmallCollection> = InlineHashTableNoCollisions::new_raw([1, 2, 0, 0], [10, 20]);

        // Hash code 0 → slot 0 → entry index 1 → entries[0] = 10
        assert!(table.contains(0, |e| *e == 10));
        // Hash code 1 → slot 1 → entry index 2 → entries[1] = 20
        assert!(table.contains(1, |e| *e == 20));
        // Hash code 2 → slot 2 → entry index 0 → empty, must return false
        assert!(!table.contains(2, |_| true));
        // Hash code 3 → slot 3 → entry index 0 → empty, must return false
        assert!(!table.contains(3, |_| true));
    }
}
