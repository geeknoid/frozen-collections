/// Controls the magnitude of collection types.
///
/// This trait indicates that a collection's layout can be optimized at compile-time depending on
/// the max capacity that the collection can hold.
pub trait CollectionMagnitude: Copy + TryFrom<usize> + Into<usize> {
    /// The maximum number of entries supported in the collection.
    const MAX_CAPACITY: usize;

    /// The zero value for the magnitude.
    const ZERO: Self;
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
