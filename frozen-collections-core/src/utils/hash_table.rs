use std::fmt::{Debug, Formatter, Result};
use std::mem::MaybeUninit;
use std::ops::Range;

use crate::analyzers::analyze_hash_codes;
use crate::traits::{CollectionMagnitude, Len};
use bitvec::macros::internal::funty::Fundamental;

/// A general purpose hash table.
///
/// The `CM` type parameter is the collection magnitude, which
/// determines the maximum number of entries that can be stored in the hash table.
///
/// This implementation always has a power-of-two number of hash slots. This speeds up
/// lookups by avoiding the need to perform a modulo operation.
#[derive(Clone)]
pub struct HashTable<T, CM> {
    pub mask: u64,
    pub slots: Box<[HashTableSlot<CM>]>,
    pub entries: Box<[T]>,
}

/// An individual slot in the hash table. This slot contains the range of indices in the
/// `entries` array that contain entries that hash to this slot.
#[derive(Clone)]
#[allow(clippy::module_name_repetitions)]
pub struct HashTableSlot<CM> {
    pub min_index: CM,
    pub max_index: CM,
}

/// Used while preparing the entries for the hash table.
pub struct PrepItem<T> {
    pub hash_slot_index: usize,
    pub entry: T,
}

impl<T, CM> HashTable<T, CM>
where
    CM: CollectionMagnitude,
{
    #[allow(clippy::missing_errors_doc)]
    #[allow(clippy::missing_panics_doc)]
    pub fn new<F>(mut entries: Vec<T>, hash: F) -> std::result::Result<Self, String>
    where
        F: Fn(&T) -> u64,
    {
        if entries.is_empty() {
            return Ok(Self::default());
        } else if entries.len() > CM::MAX_CAPACITY {
            // TODO: check done before duplicates are removed.
            return Err("too many entries for the selected collection magnitude".to_string());
        }

        // TODO: this analysis is done before duplicates are removed.
        let num_hash_slots = analyze_hash_codes(entries.iter().map(&hash)).num_hash_slots;

        let mut prep_items = Vec::with_capacity(entries.len());
        while let Some(entry) = entries.pop() {
            let hash_code = hash(&entry);
            let hash_slot_index = (hash_code % num_hash_slots as u64).as_usize();

            prep_items.push(PrepItem {
                hash_slot_index,
                entry,
            });
        }

        // sort items so hash collisions are contiguous. We use a stable sort to ensure
        // that the order of entries with the same hash code is preserved, so that when
        // multiple equal keys are inserted, the last one is the one that is found on query.
        prep_items.sort_by(|x, y| x.hash_slot_index.cmp(&y.hash_slot_index));
        prep_items.reverse();

        let mut entry_index = 0;
        let mut slots = Vec::with_capacity(num_hash_slots);
        let mut final_entries = entries;

        slots.resize_with(num_hash_slots, || HashTableSlot {
            min_index: CM::ZERO,
            max_index: CM::ZERO,
        });

        while let Some(mut item) = prep_items.pop() {
            let hash_slot_index = item.hash_slot_index;
            let mut num_entries_in_hash_slot = 0;

            loop {
                final_entries.push(item.entry);
                num_entries_in_hash_slot += 1;

                if let Some(last) = prep_items.last() {
                    if last.hash_slot_index == hash_slot_index {
                        item = prep_items.pop().unwrap();
                        continue;
                    }
                }

                break;
            }

            slots[hash_slot_index] = HashTableSlot {
                min_index: CM::try_from(entry_index).unwrap_or(CM::ZERO),
                max_index: CM::try_from(entry_index + num_entries_in_hash_slot).unwrap_or(CM::ZERO),
            };

            entry_index += num_entries_in_hash_slot;
        }

        Ok(Self {
            mask: (slots.len() - 1) as u64,
            slots: slots.into_boxed_slice(),
            entries: final_entries.into_boxed_slice(),
        })
    }
}

impl<T, CM> HashTable<T, CM>
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

impl<T, CM> Len for HashTable<T, CM> {
    #[inline]
    fn len(&self) -> usize {
        self.entries.len()
    }
}

impl<T, CM> Debug for HashTable<T, CM>
where
    T: Debug,
{
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        f.debug_set().entries(&self.entries).finish()
    }
}

impl<T, CM> Default for HashTable<T, CM> {
    fn default() -> Self {
        Self {
            mask: 0,
            slots: Box::new([]),
            entries: Box::new([]),
        }
    }
}
