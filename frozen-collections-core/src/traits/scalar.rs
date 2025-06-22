use core::num::{NonZeroI8, NonZeroI16, NonZeroI32, NonZeroIsize, NonZeroU8, NonZeroU16, NonZeroU32, NonZeroUsize};

/// A scalar value with an index in its sequence of possible values.
pub trait Scalar: Ord + Clone + Copy {
    /// Returns a value's index into its containing sequence.
    fn index(&self) -> usize;
}

macro_rules! impl_unsigned_scalar {
    ($($t:ty),*) => {
        $(
            impl Scalar for $t {
                #[inline]
                #[allow(clippy::cast_possible_truncation, reason = "Normal")]
                #[allow(trivial_numeric_casts, reason = "Normal")]
                fn index(&self) -> usize {
                    *self as usize
                }
            }
        )*
    };
}

macro_rules! impl_signed_scalar {
    ($($t:ty: $unsigned_ty:ty: $mask:expr),*) => {
        $(
            impl Scalar for $t {
                #[inline]
                #[allow(clippy::cast_sign_loss, reason = "Normal")]
                #[allow(clippy::cast_possible_truncation, reason = "Normal")]
                #[allow(trivial_numeric_casts, reason = "Normal")]
                fn index(&self) -> usize {
                    ((*self as $unsigned_ty) ^ $mask) as usize
                }
            }
        )*
    };
}

macro_rules! impl_unsigned_nz_scalar {
    ($($t:ty),*) => {
        $(
            impl Scalar for $t {
                #[inline]
                #[allow(clippy::cast_possible_truncation, reason = "Normal")]
                #[allow(trivial_numeric_casts, reason = "Normal")]
                fn index(&self) -> usize {
                    (*self).get() as usize
                }
            }
        )*
    };
}

macro_rules! impl_signed_nz_scalar {
    ($($t:ty: $unsigned_ty:ty: $mask:expr),*) => {
        $(
            impl Scalar for $t {
                #[inline]
                #[allow(clippy::cast_sign_loss, reason = "Normal")]
                #[allow(clippy::cast_possible_truncation, reason = "Normal")]
                #[allow(trivial_numeric_casts, reason = "Normal")]
                fn index(&self) -> usize {
                    (((*self).get() as $unsigned_ty) ^ $mask) as usize
                }
            }
        )*
    };
}

#[cfg(target_pointer_width = "64")]
impl_unsigned_scalar!(u8, u16, u32, u64, usize);
#[cfg(target_pointer_width = "64")]
impl_signed_scalar!(i8:u8:0x80, i16:u16:0x8000, i32:u32:0x8000_0000, i64:u64:0x8000_0000_0000_0000, isize:usize:0x8000_0000_0000_0000);
#[cfg(target_pointer_width = "64")]
impl_unsigned_nz_scalar!(NonZeroU8, NonZeroU16, NonZeroU32, core::num::NonZeroU64, NonZeroUsize);
#[cfg(target_pointer_width = "64")]
impl_signed_nz_scalar!(NonZeroI8:u8:0x80, NonZeroI16:u16:0x8000, NonZeroI32:u32:0x8000_0000, core::num::NonZeroI64:u64:0x8000_0000_0000_0000, NonZeroIsize:usize:0x8000_0000_0000_0000);

#[cfg(target_pointer_width = "32")]
impl_unsigned_scalar!(u8, u16, u32, usize);
#[cfg(target_pointer_width = "32")]
impl_signed_scalar!(i8:u8:0x80, i16:u16:0x8000, i32:u32:0x8000_0000, isize:usize:0x8000_0000);
#[cfg(target_pointer_width = "32")]
impl_unsigned_nz_scalar!(NonZeroU8, NonZeroU16, NonZeroU32, NonZeroUsize);
#[cfg(target_pointer_width = "32")]
impl_signed_nz_scalar!(NonZeroI8:u8:0x80, NonZeroI16:u16:0x8000, NonZeroI32:u32:0x8000_0000, NonZeroIsize:usize:0x8000_0000);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_unsigned_scalar() {
        assert_eq!(5_u8.index(), 5);
        assert_eq!(10_u16.index(), 10);
        assert_eq!(20_u32.index(), 20);
        assert_eq!(30_usize.index(), 30);
    }

    #[test]
    #[cfg(target_pointer_width = "64")]
    fn test_unsigned_scalar_64() {
        assert_eq!(40_u64.index(), 40);
    }

    #[test]
    fn test_signed_scalar() {
        assert_eq!((-5_i8).index(), 0x80 - 5);
        assert_eq!((-10_i16).index(), 0x8000 - 10);
        assert_eq!((-20_i32).index(), 0x8000_0000 - 20);

        assert_eq!(5_i8.index(), 0x80 + 5);
        assert_eq!(10_i16.index(), 0x8000 + 10);
        assert_eq!(20_i32.index(), 0x8000_0000 + 20);
    }

    #[test]
    #[cfg(target_pointer_width = "32")]
    fn test_signed_scalar_32() {
        assert_eq!((-30_isize).index(), 0x8000_0000 - 30);
        assert_eq!(30_isize.index(), 0x8000_0000 + 30);
    }

    #[test]
    #[cfg(target_pointer_width = "64")]
    fn test_signed_scalar_64() {
        assert_eq!((-30_isize).index(), 0x8000_0000_0000_0000 - 30);
        assert_eq!((-40_i64).index(), 0x8000_0000_0000_0000 - 40);

        assert_eq!(30_isize.index(), 0x8000_0000_0000_0000 + 30);
        assert_eq!(40_i64.index(), 0x8000_0000_0000_0000 + 40);
    }

    #[test]
    fn test_unsigned_nz_scalar() {
        assert_eq!(NonZeroU8::new(5).unwrap().index(), 5);
        assert_eq!(NonZeroU16::new(10).unwrap().index(), 10);
        assert_eq!(NonZeroU32::new(20).unwrap().index(), 20);
        assert_eq!(NonZeroUsize::new(30).unwrap().index(), 30);
    }

    #[test]
    #[cfg(target_pointer_width = "64")]
    fn test_unsigned_nz_scalar_64() {
        assert_eq!(core::num::NonZeroU64::new(40).unwrap().index(), 40);
    }

    #[test]
    fn test_signed_nz_scalar() {
        assert_eq!(NonZeroI8::new(-5).unwrap().index(), 0x80 - 5);
        assert_eq!(NonZeroI16::new(-10).unwrap().index(), 0x8000 - 10);
        assert_eq!(NonZeroI32::new(-20).unwrap().index(), 0x8000_0000 - 20);

        assert_eq!(NonZeroI8::new(5).unwrap().index(), 0x80 + 5);
        assert_eq!(NonZeroI16::new(10).unwrap().index(), 0x8000 + 10);
        assert_eq!(NonZeroI32::new(20).unwrap().index(), 0x8000_0000 + 20);
    }

    #[test]
    #[cfg(target_pointer_width = "32")]
    fn test_signed_nz_scalar_32() {
        assert_eq!(NonZeroIsize::new(-30).unwrap().index(), 0x8000_0000 - 30);
        assert_eq!(NonZeroIsize::new(30).unwrap().index(), 0x8000_0000 + 30);
    }

    #[test]
    #[cfg(target_pointer_width = "64")]
    fn test_signed_nz_scalar_64() {
        assert_eq!(NonZeroIsize::new(-30).unwrap().index(), 0x8000_0000_0000_0000 - 30);
        assert_eq!(core::num::NonZeroI64::new(-40).unwrap().index(), 0x8000_0000_0000_0000 - 40);

        assert_eq!(NonZeroIsize::new(30).unwrap().index(), 0x8000_0000_0000_0000 + 30);
        assert_eq!(core::num::NonZeroI64::new(40).unwrap().index(), 0x8000_0000_0000_0000 + 40);
    }
}
