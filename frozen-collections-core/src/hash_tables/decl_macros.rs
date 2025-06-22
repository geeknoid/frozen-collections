macro_rules! hash_table_funcs {
    () => {
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
    };
}

pub(crate) use hash_table_funcs;
