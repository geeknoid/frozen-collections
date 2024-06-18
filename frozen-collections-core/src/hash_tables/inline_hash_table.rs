use crate::analyzers::analyze_hash_codes;
use crate::hash_tables::HashTableSlot;
use crate::traits::{CollectionMagnitude, Len, SmallCollection};
use bitvec::macros::internal::funty::Fundamental;
use core::fmt::{Debug, Formatter, Result};
use core::mem::MaybeUninit;
use core::ops::Range;

/// A hash table that stores its entries inline.
///
#[doc = include_str!("../doc_snippets/type_compat_warning.md")]
///
/// # Type Parameters
///
/// - `T`: The data held in the hash table.
/// - `CM`: The magnitude of the collection.
/// - `SZ`: The length of the map.
/// - `NHS`: The number of hash table slots.
///
/// This implementation always has a power-of-two number of hash slots. This speeds up
/// lookups by avoiding the need to perform a modulo operation.
#[derive(Clone)]
pub struct InlineHashTable<T, const SZ: usize, const NHS: usize, CM = SmallCollection> {
    mask: u64,
    slots: [HashTableSlot<CM>; NHS],
    pub(crate) entries: [T; SZ],
}

struct PrepItem<T> {
    pub hash_slot_index: usize,
    pub entry: T,
}

impl<T, const SZ: usize, const NHS: usize, CM> InlineHashTable<T, SZ, NHS, CM> {
    /// Creates a new hash table.
    ///
    /// This function assumes that there are no duplicates in the input vector.
    ///
    /// # Errors
    ///
    /// Fails if the length of the vector, after removing duplicates, isn't equal to the generic parameter `SZ`,
    /// if there are too many entries for the specified collection magnitude as indicated by the generic
    /// parameter `CM`, or if the required number of hash slots isn't equal to the generic parameter `NHS`.
    #[allow(clippy::missing_panics_doc)]
    pub fn new<F>(mut entries: Vec<T>, hash: F) -> std::result::Result<Self, String>
    where
        CM: CollectionMagnitude,
        F: Fn(&T) -> u64,
    {
        if entries.is_empty() {
            return Err(
                "must have at least one entry to create an instance of this collection".to_string(),
            );
        } else if entries.len() != SZ {
            let len = entries.len();
            return Err(format!("incorrect # of entries: got {len} but SZ={SZ}"));
        } else if entries.len() > CM::MAX_CAPACITY {
            return Err("too many entries for the selected collection magnitude".to_string());
        }

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

        slots.resize_with(num_hash_slots, || HashTableSlot::new(CM::ZERO, CM::ZERO));

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

            slots[hash_slot_index] = HashTableSlot::new(
                CM::try_from(entry_index).unwrap_or(CM::ZERO),
                CM::try_from(entry_index + num_entries_in_hash_slot).unwrap_or(CM::ZERO),
            );

            entry_index += num_entries_in_hash_slot;
        }

        let len = final_entries.len();
        let slen = slots.len();
        Ok(Self::new_raw(
            slots
                .try_into()
                .map_err(|_| format!("incorrect # of hash slots: needs {slen} but NHS={NHS}"))?,
            final_entries
                .try_into()
                .map_err(|_| format!("incorrect # of entries: got {len} but SZ={SZ}"))?,
        ))
    }

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
    pub(crate) fn find(&self, hash_code: u64, mut eq: impl FnMut(&T) -> bool) -> Option<&T> {
        let hash_slot_index = (hash_code & self.mask).as_usize();
        let hash_slot = unsafe { self.slots.get_unchecked(hash_slot_index) };
        let range: Range<usize> = hash_slot.min_index.into()..hash_slot.max_index.into();
        let entries = unsafe { self.entries.get_unchecked(range) };
        entries.iter().find(|entry| eq(entry))
    }

    #[inline]
    pub(crate) fn find_mut(
        &mut self,
        hash_code: u64,
        mut eq: impl FnMut(&T) -> bool,
    ) -> Option<&mut T> {
        let hash_slot_index = (hash_code & self.mask).as_usize();
        let hash_slot = unsafe { self.slots.get_unchecked(hash_slot_index) };
        let range: Range<usize> = hash_slot.min_index.into()..hash_slot.max_index.into();
        let entries = unsafe { self.entries.get_unchecked_mut(range) };
        entries.iter_mut().find(|entry| eq(entry))
    }

    pub(crate) fn get_many_mut<const N: usize>(
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

impl<T, const SZ: usize, const NHS: usize, CM> Len for InlineHashTable<T, SZ, NHS, CM> {
    #[inline]
    fn len(&self) -> usize {
        self.entries.len()
    }
}

impl<T, const SZ: usize, const NHS: usize, CM> Debug for InlineHashTable<T, SZ, NHS, CM>
where
    T: Debug,
{
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        f.debug_set().entries(&self.entries).finish()
    }
}
