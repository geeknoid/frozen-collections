mod sealed {
    pub trait Sealed {}
    impl Sealed for u8 {}
    impl Sealed for u16 {}
    impl Sealed for usize {}
}

/// Controls the magnitude of collection types.
///
/// This trait indicates that a collection's layout can be optimized at compile-time depending on
/// the max capacity that the collection can hold.
///
/// This trait is sealed and cannot be implemented outside of this crate.
pub trait CollectionMagnitude: sealed::Sealed + Copy + TryFrom<usize> + Into<usize> {
    /// The maximum number of entries supported in the collection.
    const MAX_CAPACITY: usize;

    /// The zero value for the magnitude.
    const ZERO: Self;
}

/// Converts a [`CollectionMagnitude`] value to `usize` in a const context.
///
/// This is needed because `Into::<usize>::into()` is not a `const fn`,
/// so it cannot be used in const constructors. This function uses
/// `transmute_copy` with `size_of`-based dispatch to handle the three
/// known magnitude types (`u8`, `u16`, `usize`).
///
/// # Panics
///
/// Panics if `CM` is not one of the supported magnitude types
/// (`u8`, `u16`, or `usize`).
pub const fn cm_to_usize<CM: Copy>(val: CM) -> usize {
    match size_of::<CM>() {
        1 => {
            // SAFETY: CM is u8 (SmallCollection), which is 1 byte
            let b: [u8; 1] = unsafe { core::mem::transmute_copy(&val) };
            b[0] as usize
        }
        2 => {
            // SAFETY: CM is u16 (MediumCollection), which is 2 bytes
            let b: [u8; 2] = unsafe { core::mem::transmute_copy(&val) };
            u16::from_ne_bytes(b) as usize
        }
        _ => {
            assert!(size_of::<CM>() == size_of::<usize>(), "unsupported CollectionMagnitude type");
            // SAFETY: CM is usize (LargeCollection), same size as usize
            unsafe { core::mem::transmute_copy(&val) }
        }
    }
}

/// A small collection that can hold up to 255 entries.
pub type SmallCollection = u8;

impl CollectionMagnitude for SmallCollection {
    const MAX_CAPACITY: usize = Self::MAX as usize;
    const ZERO: Self = 0;
}

/// A medium collection that can hold up to 65,535 entries.
pub type MediumCollection = u16;

impl CollectionMagnitude for MediumCollection {
    const MAX_CAPACITY: usize = Self::MAX as usize;
    const ZERO: Self = 0;
}

/// A large collection that can hold up to [`usize::MAX`] entries.
pub type LargeCollection = usize;

impl CollectionMagnitude for LargeCollection {
    const MAX_CAPACITY: Self = Self::MAX;
    const ZERO: Self = 0;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn cm_to_usize_small_collection() {
        assert_eq!(cm_to_usize(0u8), 0);
        assert_eq!(cm_to_usize(42u8), 42);
        assert_eq!(cm_to_usize(u8::MAX), 255);
    }

    #[test]
    fn cm_to_usize_medium_collection() {
        assert_eq!(cm_to_usize(0u16), 0);
        assert_eq!(cm_to_usize(1000u16), 1000);
        assert_eq!(cm_to_usize(u16::MAX), 65535);
    }

    #[test]
    fn cm_to_usize_large_collection() {
        assert_eq!(cm_to_usize(0usize), 0);
        assert_eq!(cm_to_usize(123_456_usize), 123_456);
        assert_eq!(cm_to_usize(usize::MAX), usize::MAX);
    }
}
