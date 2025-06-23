use crate::DefaultHashBuilder;
use crate::traits::Hasher;
use crate::utils::cold;
use core::hash::{BuildHasher, Hash};
use core::ops::Range;

/// Hashes a portion of a left-aligned slice.
///
#[doc = include_str!("../doc_snippets/private_api_warning.md")]
#[derive(Clone, Debug)]
pub struct LeftRangeHasher<BH = DefaultHashBuilder> {
    bh: BH,
    range: Range<usize>,
}

impl<BH> LeftRangeHasher<BH> {
    /// Creates a new `LeftRangeHasher` with the specified hash builder and range.
    #[must_use]
    pub const fn new(bh: BH, range: Range<usize>) -> Self {
        Self { bh, range }
    }
}

impl<T, BH> Hasher<[T]> for LeftRangeHasher<BH>
where
    T: Hash,
    BH: BuildHasher,
{
    #[inline]
    fn hash_one(&self, value: &[T]) -> u64 {
        if value.len() < self.range.end {
            return 0;
        }

        self.bh.hash_one(&value[self.range.clone()])
    }
}

impl<BH> Hasher<str> for LeftRangeHasher<BH>
where
    BH: BuildHasher,
{
    #[inline]
    fn hash_one(&self, value: &str) -> u64 {
        let b = value.as_bytes();
        if b.len() < self.range.end {
            cold();
            return 0;
        }

        self.bh.hash_one(&b[self.range.clone()])
    }
}

impl<AR, BH> Hasher<AR> for LeftRangeHasher<BH>
where
    BH: BuildHasher,
    AR: AsRef<str>,
{
    #[inline]
    fn hash_one(&self, value: &AR) -> u64 {
        let b = value.as_ref().as_bytes();
        if b.len() < self.range.end {
            cold();
            return 0;
        }

        self.bh.hash_one(&b[self.range.clone()])
    }
}

impl<BH> Default for LeftRangeHasher<BH>
where
    BH: Default,
{
    fn default() -> Self {
        Self::new(BH::default(), 0..0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use alloc::vec;
    use foldhash::fast::RandomState;

    #[test]
    fn test_left_range_hasher_hash_slice() {
        let hasher = LeftRangeHasher::new(RandomState::default(), 0..3);
        assert_eq!(
            hasher.hash_one(vec![1, 2, 3, 4].as_slice()),
            hasher.bh.hash_one(vec![1, 2, 3].as_slice())
        );
        assert_eq!(hasher.hash_one(vec![1, 2].as_slice()), 0);
    }

    #[test]
    fn test_left_range_hasher_hash_string() {
        let hasher = LeftRangeHasher::new(RandomState::default(), 0..3);
        assert_eq!(hasher.hash_one(&"abcd".to_string()), hasher.bh.hash_one(b"abc"));
        assert_eq!(hasher.hash_one(&"ab".to_string()), 0);
    }

    #[test]
    fn test_left_range_hasher_hash_str_ref() {
        let hasher = LeftRangeHasher::new(RandomState::default(), 0..3);
        assert_eq!(hasher.hash_one(&"abcd"), hasher.bh.hash_one(b"abc"));
        assert_eq!(hasher.hash_one(&"ab"), 0);
    }

    #[test]
    fn test_left_range_hasher_hash_str() {
        let hasher = LeftRangeHasher::new(RandomState::default(), 0..3);
        assert_eq!(hasher.hash_one("abcd"), hasher.bh.hash_one(b"abc"));
        assert_eq!(hasher.hash_one("ab"), 0);
    }

    #[test]
    fn test_left_range_hasher_default() {
        let hasher: LeftRangeHasher = LeftRangeHasher::default();
        assert_eq!(hasher.range, 0..0);
    }
}
