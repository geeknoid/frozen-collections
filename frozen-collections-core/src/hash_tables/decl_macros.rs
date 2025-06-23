macro_rules! hash_table_funcs {
    () => {
        #[inline]
        pub(crate) fn find(&self, hash_code: u64, mut eq: impl FnMut(&T) -> bool) -> Option<&T> {
            #[expect(clippy::cast_possible_truncation, reason = "Truncation ok on 32 bit systems")]
            let hash_slot_index = (hash_code & self.mask) as usize;

            // SAFETY: The hash slot index is guaranteed to be within bounds because of the masking above
            let hash_slot = unsafe { self.slots.get_unchecked(hash_slot_index) };
            let min: usize = hash_slot.min_index.into();
            let mut max: usize = hash_slot.max_index.into();

            // SAFETY: The range is guaranteed to be within bounds by construction
            let entries = unsafe { self.entries.get_unchecked(min..max) };
            let mut ptr = entries.as_ptr();

            while max > min {
                // SAFETY: The pointer is guaranteed to be valid because of the range above
                if eq(unsafe { &*ptr }) {
                    // SAFETY: The pointer is guaranteed to be valid because of the range above
                    return Some(unsafe { &*ptr });
                }

                // SAFETY: The pointer is guaranteed to be valid because of the range above
                unsafe { ptr = ptr.add(1) };
                max -= 1;
            }

            None
        }

        #[inline]
        pub(crate) fn find_mut(&mut self, hash_code: u64, mut eq: impl FnMut(&T) -> bool) -> Option<&mut T> {
            #[expect(clippy::cast_possible_truncation, reason = "Truncation on 32 bit systems is fine")]
            let hash_slot_index = (hash_code & self.mask) as usize;

            // SAFETY: The hash slot index is guaranteed to be within bounds because of the masking above
            let hash_slot = unsafe { self.slots.get_unchecked(hash_slot_index) };
            let min: usize = hash_slot.min_index.into();
            let mut max: usize = hash_slot.max_index.into();

            // SAFETY: The range is guaranteed to be within bounds by construction
            let entries = unsafe { self.entries.get_unchecked_mut(min..max) };
            let mut ptr = entries.as_mut_ptr();

            while max > min {
                // SAFETY: The pointer is guaranteed to be valid because of the range above
                if eq(unsafe { &*ptr }) {
                    // SAFETY: The pointer is guaranteed to be valid because of the range above
                    return Some(unsafe { &mut *ptr });
                }

                // SAFETY: The pointer is guaranteed to be valid because of the range above
                unsafe { ptr = ptr.add(1) };
                max -= 1;
            }

            None
        }
    };
}

pub(crate) use hash_table_funcs;
