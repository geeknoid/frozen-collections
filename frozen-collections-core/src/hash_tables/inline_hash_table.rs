use crate::hash_tables::HashTableSlot;
use crate::hash_tables::decl_macros::hash_table_funcs;
use crate::traits::{CollectionMagnitude, SmallCollection, cm_to_usize};

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
#[derive(Clone, Debug)]
pub struct InlineHashTable<T, const SZ: usize, const NHS: usize, CM = SmallCollection> {
    mask: u64,
    slots: [HashTableSlot<CM>; NHS],
    pub(crate) entries: [T; SZ],
}

impl<T, const SZ: usize, const NHS: usize, CM: CollectionMagnitude> InlineHashTable<T, SZ, NHS, CM> {
    /// Creates a new hash table.
    ///
    /// # Panics
    ///
    /// Panics if `NHS` is zero or not a power of two, or if any slot has
    /// `min_index > max_index` or `max_index > SZ`.
    pub const fn new_raw(slots: [HashTableSlot<CM>; NHS], processed_entries: [T; SZ]) -> Self {
        assert!(NHS > 0, "number of hash slots must be greater than zero");
        assert!(NHS.is_power_of_two(), "number of hash slots must be a power of two");

        let mut i = 0;
        while i < NHS {
            let min_index = cm_to_usize(slots[i].min_index);
            let max_index = cm_to_usize(slots[i].max_index);
            assert!(min_index <= max_index, "slot min_index exceeds max_index");
            assert!(max_index <= SZ, "slot max_index exceeds number of entries");
            i += 1;
        }

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
    hash_table_funcs!();

    #[inline]
    pub(crate) const fn len(&self) -> usize {
        self.entries.len()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[should_panic(expected = "slot max_index exceeds number of entries")]
    fn new_raw_panics_on_max_index_exceeding_entries() {
        // max_index 3 exceeds SZ=2
        let _table: InlineHashTable<i32, 2, 4, SmallCollection> = InlineHashTable::new_raw(
            [
                HashTableSlot::new(0, 1),
                HashTableSlot::new(0, 3),
                HashTableSlot::new(0, 0),
                HashTableSlot::new(0, 0),
            ],
            [10, 20],
        );
    }

    #[test]
    #[should_panic(expected = "slot min_index exceeds max_index")]
    fn new_raw_panics_on_min_index_exceeding_max_index() {
        let _table: InlineHashTable<i32, 2, 4, SmallCollection> = InlineHashTable::new_raw(
            [
                HashTableSlot::new(2, 1),
                HashTableSlot::new(0, 0),
                HashTableSlot::new(0, 0),
                HashTableSlot::new(0, 0),
            ],
            [10, 20],
        );
    }

    #[test]
    fn new_raw_valid_construction() {
        let table: InlineHashTable<i32, 2, 4, SmallCollection> = InlineHashTable::new_raw(
            [
                HashTableSlot::new(0, 1),
                HashTableSlot::new(1, 2),
                HashTableSlot::new(0, 0),
                HashTableSlot::new(0, 0),
            ],
            [10, 20],
        );
        assert_eq!(table.len(), 2);
    }
}
