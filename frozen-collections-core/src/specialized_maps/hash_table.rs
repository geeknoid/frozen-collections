use std::fmt::{Debug, Formatter, Result};
use std::ops::Range;

use bitvec::macros::internal::funty::Fundamental;
use num_traits::{PrimInt, Unsigned};
use quickdiv::DivisorU64;

#[derive(Clone)]
pub struct HashTable<K, V, S> {
    hash_divisor: DivisorU64,
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
    entry: (K, V),
}

impl<K, V, S> HashTable<K, V, S>
where
    S: PrimInt + Unsigned,
{
    pub fn new<F>(
        mut payload: Vec<(K, V)>,
        num_hash_slots: usize,
        hash: F,
    ) -> std::result::Result<Self, &'static str>
    where
        F: Fn(&K) -> u64,
    {
        if payload.is_empty() {
            return Ok(Self::default());
        } else if payload.len() > S::max_value().to_usize().unwrap() {
            return Err("too many payload entries for the given collection size S");
        }

        let mut prep_items = Vec::with_capacity(payload.len());
        while let Some(entry) = payload.pop() {
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
        let mut entries = payload;

        slots.resize_with(num_hash_slots, || HashTableSlot {
            min_index: S::zero(),
            max_index: S::zero(),
        });

        while let Some(mut item) = prep_items.pop() {
            let hash_slot_index = item.hash_slot_index;
            let mut num_entries_in_hash_slot = 0;

            loop {
                entries.push(item.entry);
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
                min_index: S::from(entry_index).unwrap(),
                max_index: S::from(entry_index).unwrap()
                    + S::from(num_entries_in_hash_slot).unwrap(),
            };

            entry_index += num_entries_in_hash_slot;
        }

        Ok(Self {
            hash_divisor: DivisorU64::new(slots.len() as u64),
            slots: slots.into_boxed_slice(),
            entries: entries.into_boxed_slice(),
        })
    }

    #[inline]
    pub fn get_hash_info(&self, hash_code: u64) -> Range<usize> {
        let hash_slot_index = self.hash_divisor.rem_of(hash_code).as_usize();
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

impl<K, V, S> Default for HashTable<K, V, S>
where
    S: PrimInt + Unsigned,
{
    fn default() -> Self {
        Self {
            hash_divisor: DivisorU64::new(1),
            slots: Box::new([HashTableSlot {
                min_index: S::zero(),
                max_index: S::zero(),
            }]),
            entries: Box::new([]),
        }
    }
}
