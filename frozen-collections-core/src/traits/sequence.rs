use std::num::{
    NonZeroI128, NonZeroI16, NonZeroI32, NonZeroI64, NonZeroI8, NonZeroIsize, NonZeroU128,
    NonZeroU16, NonZeroU32, NonZeroU64, NonZeroU8, NonZeroUsize,
};

/// An indexable ordered sequence of values.
pub trait Sequence: Ord + Clone + Copy {
    /// The minimum value of the sequence.
    const MIN: Self;

    /// The maximum value of the sequence.
    const MAX: Self;

    /// Returns a `u64` representation of this sequence value.
    fn as_u64(&self) -> u64;

    /// Returns the index of `value` within the range min..-max, or None if the value is outside the range.
    #[must_use]
    fn offset(min: &Self, max: &Self, value: &Self) -> Option<usize>;

    /// Returns the number of entries needed to represent the closed range min..=max
    #[must_use]
    fn count(min: &Self, max: &Self) -> Option<usize>;
}

macro_rules! impl_sequence {
    ($($t:ty: $unsigned_t:ty),*) => {
        $(
            impl Sequence for $t {
                const MIN: Self = Self::MIN;
                const MAX: Self = Self::MAX;

                #[allow(clippy::cast_possible_truncation)]
                #[allow(clippy::cast_sign_loss)]
                #[allow(clippy::cast_lossless)]
                #[inline]
                fn as_u64(&self) -> u64 {
                    (*self as $unsigned_t).wrapping_sub(Self::MIN as $unsigned_t) as u64
                }

                #[allow(clippy::cast_possible_truncation)]
                #[allow(clippy::cast_sign_loss)]
                #[inline]
                fn offset(min: &Self, max:&Self, value: &Self) -> Option<usize> {
                    let max_index = (*max as $unsigned_t).wrapping_sub(*min as $unsigned_t) as usize;
                    let value_index = (*value as $unsigned_t).wrapping_sub(*min as $unsigned_t) as usize;

                    if value_index > max_index {
                        None
                    } else {
                        Some(value_index)
                    }
                }

                #[allow(clippy::cast_possible_truncation)]
                #[allow(clippy::cast_sign_loss)]
                fn count(min: &Self, max: &Self) -> Option<usize> {
                    if max < min {
                        None
                    } else if max.abs_diff(*min) as u128 > usize::MAX as u128 {
                        None
                    } else {
                        Some(max.abs_diff(*min) as usize + 1)
                    }
                }
            }
        )*
    };
}

macro_rules! impl_sequence_nz {
    ($($t:ty:$unsigned_t:ty),*) => {
        $(
            impl Sequence for $t {
                const MIN: Self = Self::MIN;
                const MAX: Self = Self::MAX;

                #[allow(clippy::cast_possible_truncation)]
                #[allow(clippy::cast_sign_loss)]
                #[allow(clippy::cast_lossless)]
                #[inline]
                fn as_u64(&self) -> u64 {
                    (self.get() as $unsigned_t).wrapping_sub(Self::MIN.get() as $unsigned_t) as u64
                }


                #[allow(clippy::cast_possible_truncation)]
                #[allow(clippy::cast_sign_loss)]
                #[inline]
                fn offset(min: &Self, max:&Self, value: &Self) -> Option<usize> {
                    let max_index = (max.get() as $unsigned_t).wrapping_sub(min.get() as $unsigned_t) as usize;
                    let value_index = (value.get() as $unsigned_t).wrapping_sub(min.get() as $unsigned_t) as usize;

                    if value_index > max_index {
                        None
                    } else {
                        Some(value_index)
                    }
                }

                #[allow(clippy::cast_possible_truncation)]
                #[allow(clippy::cast_sign_loss)]
                fn count(min: &Self, max: &Self) -> Option<usize> {
                    if max.get() < min.get() {
                        None
                    } else if max.get().abs_diff(min.get()) as u128 > usize::MAX as u128 {
                        None
                    } else {
                        Some(max.get().abs_diff(min.get()) as usize + 1)
                    }
                }
            }
        )*
    };
}

impl_sequence!(u8:u8, u16:u16, u32:u32, u64:u64, u128:u128, usize:usize);
impl_sequence!(i8:u8, i16:u16, i32:u32, i64:u64, i128:u128, isize:usize);
impl_sequence_nz!(
    NonZeroU8:u8,
    NonZeroU16:u16,
    NonZeroU32:u32,
    NonZeroU64:u64,
    NonZeroU128:u128,
    NonZeroUsize:usize
);
impl_sequence_nz!(
    NonZeroI8:u8,
    NonZeroI16:u16,
    NonZeroI32:u32,
    NonZeroI64:u64,
    NonZeroI128:u128,
    NonZeroIsize:usize
);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_count_signed() {
        assert_eq!(i8::count(&-5, &5), Some(11));
        assert_eq!(i8::count(&-128, &127), Some(256));
        assert_eq!(i8::count(&0, &0), Some(1));
        assert_eq!(i8::count(&5, &-5), None); // Invalid range
    }

    #[test]
    fn test_count_unsigned() {
        assert_eq!(u8::count(&5, &10), Some(6));
        assert_eq!(u8::count(&0, &255), Some(256));
        assert_eq!(u8::count(&0, &0), Some(1));
        assert_eq!(u8::count(&10, &5), None); // Invalid range
    }

    #[test]
    fn test_count_nonzero() {
        assert_eq!(
            NonZeroU8::count(&NonZeroU8::new(1).unwrap(), &NonZeroU8::new(5).unwrap()),
            Some(5)
        );
        assert_eq!(
            NonZeroU8::count(&NonZeroU8::new(1).unwrap(), &NonZeroU8::new(255).unwrap()),
            Some(255)
        );
        assert_eq!(
            NonZeroU8::count(&NonZeroU8::new(1).unwrap(), &NonZeroU8::new(1).unwrap()),
            Some(1)
        );
        assert_eq!(
            NonZeroU8::count(&NonZeroU8::new(5).unwrap(), &NonZeroU8::new(1).unwrap()),
            None
        ); // Invalid range
    }

    #[test]
    fn test_offset_signed() {
        assert_eq!(i8::offset(&-5, &5, &0), Some(5));
        assert_eq!(i8::offset(&-128, &127, &0), Some(128));
        assert_eq!(i8::offset(&0, &0, &0), Some(0));
        assert_eq!(i8::offset(&5, &-5, &0), None); // Invalid range
        assert_eq!(i8::offset(&-5, &5, &10), None); // Value out of range
    }

    #[test]
    fn test_offset_unsigned() {
        assert_eq!(u8::offset(&5, &10, &7), Some(2));
        assert_eq!(u8::offset(&0, &255, &128), Some(128));
        assert_eq!(u8::offset(&0, &0, &0), Some(0));
        assert_eq!(u8::offset(&10, &5, &7), None); // Invalid range
        assert_eq!(u8::offset(&5, &10, &11), None); // Value out of range
    }

    #[test]
    fn test_offset_nonzero() {
        assert_eq!(
            NonZeroU8::offset(
                &NonZeroU8::new(1).unwrap(),
                &NonZeroU8::new(5).unwrap(),
                &NonZeroU8::new(3).unwrap()
            ),
            Some(2)
        );
        assert_eq!(
            NonZeroU8::offset(
                &NonZeroU8::new(1).unwrap(),
                &NonZeroU8::new(255).unwrap(),
                &NonZeroU8::new(128).unwrap()
            ),
            Some(127)
        );
        assert_eq!(
            NonZeroU8::offset(
                &NonZeroU8::new(1).unwrap(),
                &NonZeroU8::new(1).unwrap(),
                &NonZeroU8::new(1).unwrap()
            ),
            Some(0)
        );
        assert_eq!(
            NonZeroU8::offset(
                &NonZeroU8::new(5).unwrap(),
                &NonZeroU8::new(1).unwrap(),
                &NonZeroU8::new(3).unwrap()
            ),
            None
        ); // Invalid range
        assert_eq!(
            NonZeroU8::offset(
                &NonZeroU8::new(1).unwrap(),
                &NonZeroU8::new(5).unwrap(),
                &NonZeroU8::new(6).unwrap()
            ),
            None
        ); // Value out of range
    }
}
