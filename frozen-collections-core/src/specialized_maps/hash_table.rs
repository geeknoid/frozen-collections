use std::fmt::{Debug, Formatter, Result};
use std::num::{NonZeroU64, NonZeroUsize};
use std::ops::Range;

use bitvec::macros::internal::funty::Fundamental;
use num_traits::{PrimInt, Unsigned};

#[derive(Clone)]
pub struct HashTable<K, V, S> {
    num_slots: NonZeroU64,
    slots: Box<[HashTableSlot<S>]>,
    pub entries: Box<[(K, V)]>,
}

#[derive(Clone)]
struct HashTableSlot<S> {
    min_index: S,
    max_index: S,
}

struct PrepItem<K, V> {
    hash_slot_index: usize,
    entry: (K, V), // TODO: Try to use a different approach so we don't copy around so much data
}

impl<K, V, S> HashTable<K, V, S>
where
    S: PrimInt + Unsigned,
{
    pub fn new<F>(payload: Vec<(K, V)>, num_hash_slots: usize, hash: F) -> Self
    where
        F: Fn(&K) -> u64,
    {
        if payload.is_empty() {
            return Self {
                num_slots: NonZeroU64::try_from(1).unwrap(),
                slots: Box::new([HashTableSlot {
                    min_index: S::zero(),
                    max_index: S::zero(),
                }]),
                entries: Box::new([]),
            };
        } else if payload.len() > S::max_value().to_usize().unwrap() {
            panic!("Too many payload entries for the map size S")
        }

        let mut prep_items = Vec::new();
        for entry in payload {
            let hash_code = hash(&entry.0);
            let hash_slot_index = (hash_code % num_hash_slots as u64).as_usize();

            prep_items.push(PrepItem {
                hash_slot_index,
                entry,
            });
        }

        // sort items so hash collisions are contiguous
        prep_items.sort_unstable_by(|x, y| x.hash_slot_index.cmp(&y.hash_slot_index));

        let mut entry_index = 0;
        let mut slots = Vec::with_capacity(num_hash_slots);
        let mut entries = Vec::with_capacity(prep_items.len());

        slots.resize_with(num_hash_slots, || HashTableSlot {
            min_index: S::zero(),
            max_index: S::zero(),
        });

        while let Some(mut item) = prep_items.pop() {
            let hash_slot_index = item.hash_slot_index;
            let mut num_entries = 0;

            loop {
                entries.push(item.entry);
                num_entries += 1;

                if prep_items.is_empty()
                    || prep_items.last().unwrap().hash_slot_index != hash_slot_index
                {
                    break;
                }

                item = prep_items.pop().unwrap();
            }

            slots[hash_slot_index] = HashTableSlot {
                min_index: S::from(entry_index).unwrap(),
                max_index: S::from(entry_index).unwrap() + S::from(num_entries).unwrap(),
            };

            entry_index += num_entries;
        }

        Self {
            num_slots: NonZeroU64::try_from(NonZeroUsize::try_from(slots.len()).unwrap()).unwrap(),
            slots: slots.into_boxed_slice(),
            entries: entries.into_boxed_slice(),
        }
    }

    #[inline]
    pub fn get_hash_info(&self, hash_code: u64) -> Range<usize> {
        let hash_slot_index = (hash_code % self.num_slots).as_usize();
        let hash_slot = unsafe { self.slots.get_unchecked(hash_slot_index) };

        hash_slot.min_index.to_usize().unwrap()..hash_slot.max_index.to_usize().unwrap()
    }
}

impl<K, V, S> HashTable<K, V, S> {
    #[inline]
    pub const fn len(&self) -> usize {
        self.entries.len()
    }
}

impl<K, V, S> Debug for HashTable<K, V, S>
where
    K: Debug,
    V: Debug,
{
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        let pairs = self.entries.iter().map(|x| (&x.0, &x.1));
        f.debug_map().entries(pairs).finish()
    }
}
