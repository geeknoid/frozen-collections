use proc_macro2::Literal;
use quote::{quote, ToTokens};

pub struct HashTable<T> {
    pub slots: Box<[HashTableSlot]>,
    pub entries: Box<[T]>,
}

#[allow(clippy::module_name_repetitions)]
pub struct HashTableSlot {
    min_index: usize,
    max_index: usize,
}

impl ToTokens for HashTableSlot {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        let min_index = Literal::usize_unsuffixed(self.min_index);
        let max_index = Literal::usize_unsuffixed(self.max_index);

        tokens.extend(
            quote!(::frozen_collections::inline_maps::HashTableSlot::new(#min_index, #max_index)),
        );
    }
}

struct PrepItem<T> {
    hash_slot_index: usize,
    entry: T,
}

#[allow(clippy::cast_possible_truncation)]
impl<T> HashTable<T> {
    pub fn new<F>(mut payload: Vec<T>, num_hash_slots: usize, hash: F) -> Self
    where
        F: Fn(&T) -> u64,
    {
        let mut prep_items = Vec::with_capacity(payload.len());
        while let Some(entry) = payload.pop() {
            let hash_code = hash(&entry);
            let hash_slot_index = (hash_code % num_hash_slots as u64) as usize;

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
            min_index: 0,
            max_index: 0,
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
                min_index: entry_index,
                max_index: entry_index + num_entries_in_hash_slot,
            };

            entry_index += num_entries_in_hash_slot;
        }

        Self {
            slots: slots.into_boxed_slice(),
            entries: entries.into_boxed_slice(),
        }
    }
}
