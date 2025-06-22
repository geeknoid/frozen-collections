use alloc::vec;
use core::ops::Range;

use crate::analyzers::analyze_hash_codes;
use crate::hash_tables::HashTableSlot;
use crate::traits::{CollectionMagnitude, Len, SmallCollection};

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
    mask: u64,
    pub(crate) slots: Box<[HashTableSlot<CM>]>,
    pub(crate) entries: Box<[T]>,
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
    pub(crate) fn new<F>(mut entries: Vec<T>, hash: F) -> Result<Self, String>
    where
        F: Fn(&T) -> u64,
    {
        if entries.is_empty() {
            return Ok(Self::default());
        } else if entries.len() > CM::MAX_CAPACITY {
            return Err("too many entries for the selected collection magnitude".to_string());
        }

        let num_hash_slots = analyze_hash_codes(entries.iter().map(&hash)).num_hash_slots;

        let mut prep_items = Vec::with_capacity(entries.len());
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

    #[inline]
    pub(crate) fn find(&self, hash_code: u64, mut eq: impl FnMut(&T) -> bool) -> Option<&T> {
        #[expect(clippy::cast_possible_truncation, reason = "Truncation ok on 32 bit systems")]
        let hash_slot_index = (hash_code & self.mask) as usize;

        // SAFETY: The hash slot index is guaranteed to be within bounds because of the modulo above
        let hash_slot = unsafe { self.slots.get_unchecked(hash_slot_index) };
        let range: Range<usize> = hash_slot.min_index.into()..hash_slot.max_index.into();

        // SAFETY: The range is guaranteed to be within bounds by construction
        let entries = unsafe { self.entries.get_unchecked(range) };

        let mut result = None;
        for entry in entries {
            if eq(entry) {
                result = Some(entry);
            }
        }

        result
    }

    #[inline]
    pub(crate) fn find_mut(&mut self, hash_code: u64, mut eq: impl FnMut(&T) -> bool) -> Option<&mut T> {
        #[expect(clippy::cast_possible_truncation, reason = "Truncation on 32 bit systems is fine")]
        let hash_slot_index = (hash_code & self.mask) as usize;

        // SAFETY: The hash slot index is guaranteed to be within bounds because of the modulo above
        let hash_slot = unsafe { self.slots.get_unchecked(hash_slot_index) };
        let range: Range<usize> = hash_slot.min_index.into()..hash_slot.max_index.into();

        // SAFETY: The range is guaranteed to be valid by construction
        let entries = unsafe { self.entries.get_unchecked_mut(range) };

        let mut result = None;
        for entry in entries {
            if eq(entry) {
                result = Some(entry);
            }
        }

        result
    }

    pub(crate) fn len(&self) -> usize {
        self.entries.len()
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
