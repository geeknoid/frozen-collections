macro_rules! hash_table_funcs {
    () => {
        #[inline]
        pub(crate) fn find(&self, hash_code: u64, eq: impl Fn(&T) -> bool) -> Option<&T> {
            #[expect(clippy::cast_possible_truncation, reason = "Truncation ok on 32 bit systems")]
            let hash_slot_index = (hash_code & self.mask) as usize;

            // SAFETY: The hash slot index is guaranteed to be within bounds because of the masking above
            let hash_slot = unsafe { self.slots.get_unchecked(hash_slot_index) };
            let min: usize = hash_slot.min_index.into();
            let max: usize = hash_slot.max_index.into();

            // SAFETY: The range is guaranteed to be within bounds by construction
            let entries = unsafe { self.entries.get_unchecked(min..max) };
            for entry in entries {
                if eq(entry) {
                    return Some(entry);
                }
            }

            None
        }

        #[inline]
        pub(crate) fn contains(&self, hash_code: u64, eq: impl Fn(&T) -> bool) -> bool {
            #[expect(clippy::cast_possible_truncation, reason = "Truncation ok on 32 bit systems")]
            let hash_slot_index = (hash_code & self.mask) as usize;

            // SAFETY: The hash slot index is guaranteed to be within bounds because of the masking above
            let hash_slot = unsafe { self.slots.get_unchecked(hash_slot_index) };
            let min: usize = hash_slot.min_index.into();
            let max: usize = hash_slot.max_index.into();

            // SAFETY: The range is guaranteed to be within bounds by construction
            let entries = unsafe { self.entries.get_unchecked(min..max) };
            for entry in entries {
                if eq(entry) {
                    return true;
                }
            }

            false
        }

        #[inline]
        pub(crate) fn find_mut(&mut self, hash_code: u64, eq: impl Fn(&T) -> bool) -> Option<&mut T> {
            #[expect(clippy::cast_possible_truncation, reason = "Truncation on 32 bit systems is fine")]
            let hash_slot_index = (hash_code & self.mask) as usize;

            // SAFETY: The hash slot index is guaranteed to be within bounds because of the masking above
            let hash_slot = unsafe { self.slots.get_unchecked(hash_slot_index) };
            let min: usize = hash_slot.min_index.into();
            let max: usize = hash_slot.max_index.into();

            // SAFETY: The range is guaranteed to be within bounds by construction
            let entries = unsafe { self.entries.get_unchecked_mut(min..max) };
            for entry in entries {
                if eq(entry) {
                    return Some(entry);
                }
            }

            None
        }
    };
}

pub(crate) use hash_table_funcs;
