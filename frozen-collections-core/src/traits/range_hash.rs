use std::hash::{BuildHasher, Hasher};
use std::ops::Range;

/// Enables hashing over a range of an input.
pub trait RangeHash {
    /// Hash a range of a value.
    #[must_use]
    fn hash_range<BH>(&self, bh: &BH, range: Range<usize>) -> u64
    where
        BH: BuildHasher;
}

impl RangeHash for String {
    #[inline]
    fn hash_range<BH>(&self, bh: &BH, range: Range<usize>) -> u64
    where
        BH: BuildHasher,
    {
        let mut h = bh.build_hasher();
        let b = unsafe { &self.as_bytes().get_unchecked(range) };
        h.write(b);
        h.finish()
    }
}

impl RangeHash for str {
    #[inline]
    fn hash_range<BH>(&self, bh: &BH, range: Range<usize>) -> u64
    where
        BH: BuildHasher,
    {
        let mut h = bh.build_hasher();
        let b = unsafe { &self.as_bytes().get_unchecked(range) };
        h.write(b);
        h.finish()
    }
}

impl RangeHash for [u8] {
    #[inline]
    fn hash_range<BH>(&self, bh: &BH, range: Range<usize>) -> u64
    where
        BH: BuildHasher,
    {
        let mut h = bh.build_hasher();
        let b = unsafe { &self.get_unchecked(range) };
        h.write(b);
        h.finish()
    }
}

#[cfg(test)]
mod tests {
    use std::hash::RandomState;

    use super::*;

    #[test]
    fn test_hash_range_for_slice_u8() {
        let data = [1, 2, 3, 4, 5];
        let hasher = RandomState::new();

        let hash_full = data.hash_range(&hasher, 0..5);
        let hash_partial = data.hash_range(&hasher, 1..4);

        assert_ne!(
            hash_full, hash_partial,
            "Hashes should differ for different ranges"
        );
    }
}
