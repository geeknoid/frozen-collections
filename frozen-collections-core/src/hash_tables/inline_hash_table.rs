use crate::hash_tables::HashTableSlot;
use crate::hash_tables::decl_macros::hash_table_funcs;
use crate::traits::{CollectionMagnitude, SmallCollection};

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
    slots: [HashTableSlot<CM>; NHS],
    pub(crate) entries: [T; SZ],
    mask: u64,
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
    hash_table_funcs!();

    #[inline]
    pub(crate) const fn len(&self) -> usize {
        self.entries.len()
    }
}
