use alloc::vec;

use crate::analyzers::analyze_hash_codes;
use crate::hash_tables::HashTableSlot;
use crate::traits::{CollectionMagnitude, Len, SmallCollection};

use crate::hash_tables::decl_macros::hash_table_funcs;
use crate::utils::DeduppedVec;
#[cfg(not(feature = "std"))]
use {alloc::boxed::Box, alloc::string::String, alloc::string::ToString, alloc::vec::Vec};

/// A general-purpose hash table.
///
#[doc = include_str!("../doc_snippets/private_api_warning.md")]
///
/// The `CM` type parameter is the collection magnitude, which
/// determines the maximum number of entries that can be stored in the hash table.
///
/// This implementation always has a power-of-two number of hash slots. This speeds up
/// lookups by avoiding the need to perform a modulo operation.
#[derive(Clone)]
pub struct HashTable<T, CM = SmallCollection> {
    pub(crate) slots: Box<[HashTableSlot<CM>]>,
    pub(crate) entries: Box<[T]>,
    mask: u64,
}

struct PrepItem<T> {
    pub hash_slot_index: usize,
    pub entry: T,
}

impl<T, CM> HashTable<T, CM>
where
    CM: CollectionMagnitude,
{
    /// Creates a new hash table.
    ///
    /// This function assumes that there are no duplicates in the input vector.
    #[expect(clippy::unwrap_in_result, reason = "Guaranteed not to happen")]
    pub(crate) fn new(entries: DeduppedVec<T>, hash: impl Fn(&T) -> u64) -> Result<Self, String> {
        if entries.is_empty() {
            return Ok(Self::default());
        } else if entries.len() > CM::MAX_CAPACITY {
            return Err("too many entries for the selected collection magnitude".to_string());
        }

        let num_hash_slots = analyze_hash_codes(entries.iter().map(&hash)).num_hash_slots;

        let mut prep_items = Vec::with_capacity(entries.len());
        let mut entries: Vec<T> = entries.into();
        while let Some(entry) = entries.pop() {
            let hash_code = hash(&entry);

            #[expect(clippy::cast_possible_truncation, reason = "Truncation ok on 32 bit systems")]
            let hash_slot_index = (hash_code % num_hash_slots as u64) as usize;

            prep_items.push(PrepItem { hash_slot_index, entry });
        }

        // sort items so hash collisions are contiguous.
        prep_items.sort_unstable_by(|x, y| x.hash_slot_index.cmp(&y.hash_slot_index));

        let mut entry_index = 0;
        let mut slots = Vec::with_capacity(num_hash_slots);
        let mut final_entries = entries;

        slots.resize_with(num_hash_slots, || HashTableSlot::new(CM::ZERO, CM::ZERO));

        while let Some(mut item) = prep_items.pop() {
            let hash_slot_index = item.hash_slot_index;
            let mut num_entries_in_hash_slot = 0;

            loop {
                final_entries.push(item.entry);
                num_entries_in_hash_slot += 1;

                if let Some(last) = prep_items.last() {
                    if last.hash_slot_index == hash_slot_index {
                        item = prep_items.pop().expect("Ensure by the call to last() above");
                        continue;
                    }
                }

                break;
            }

            slots[hash_slot_index] = HashTableSlot::new(
                CM::try_from(entry_index).unwrap_or(CM::ZERO),
                CM::try_from(entry_index + num_entries_in_hash_slot).unwrap_or(CM::ZERO),
            );

            entry_index += num_entries_in_hash_slot;
        }

        Ok(Self {
            mask: (slots.len() - 1) as u64,
            slots: slots.into_boxed_slice(),
            entries: final_entries.into_boxed_slice(),
        })
    }

    hash_table_funcs!();

    #[inline]
    pub(crate) fn len(&self) -> usize {
        self.entries.len()
    }

    #[cfg(any(feature = "emit", feature = "macros"))]
    pub(crate) fn has_collisions(&self) -> bool {
        self.slots.iter().any(|slot| {
            let min: usize = slot.min_index.into();
            let max: usize = slot.max_index.into();
            max - min > 1
        })
    }
}

impl<T, CM> Default for HashTable<T, CM>
where
    CM: CollectionMagnitude,
{
    fn default() -> Self {
        Self {
            mask: 0,
            slots: vec![HashTableSlot::new(CM::ZERO, CM::ZERO)].into_boxed_slice(),
            entries: Box::new([]),
        }
    }
}
