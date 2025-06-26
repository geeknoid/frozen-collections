use crate::DefaultBuildHasher;
use crate::traits::Hasher;
use crate::utils::cold;
use core::hash::{BuildHasher, Hash};
use core::ops::Range;

/// Hashes a portion of a right-aligned slice.
///
#[doc = include_str!("../doc_snippets/private_api_warning.md")]
#[derive(Clone, Debug)]
pub struct RightRangeHasher<BH = DefaultBuildHasher> {
    bh: BH,
    range: Range<usize>,
}

impl<BH> RightRangeHasher<BH> {
    /// Creates a new `RightRangeHasher` with the given `BuildHasher` and range.
    #[must_use]
    pub const fn new(bh: BH, range: Range<usize>) -> Self {
        Self { bh, range }
    }
}

impl<T, BH> Hasher<[T]> for RightRangeHasher<BH>
where
    T: Hash,
    BH: BuildHasher,
{
    #[inline]
    fn hash_one(&self, value: &[T]) -> u64 {
        if value.len() < self.range.end {
            cold();
            return 0;
        }

        let effective_range = value.len() - self.range.end..value.len() - self.range.start;
        self.bh.hash_one(&value[effective_range])
    }
}

impl<BH> Hasher<str> for RightRangeHasher<BH>
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

        let effective_range = value.len() - self.range.end..value.len() - self.range.start;
        self.bh.hash_one(&b[effective_range])
    }
}

impl<AR, BH> Hasher<AR> for RightRangeHasher<BH>
where
    AR: AsRef<str>,
    BH: BuildHasher,
{
    #[inline]
    fn hash_one(&self, value: &AR) -> u64 {
        let value = value.as_ref();
        let b = value.as_bytes();
        if b.len() < self.range.end {
            cold();
            return 0;
        }

        let effective_range = value.len() - self.range.end..value.len() - self.range.start;
        self.bh.hash_one(&b[effective_range])
    }
}

impl<BH> Default for RightRangeHasher<BH>
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
    fn test_right_range_hasher_hash_slice() {
        let hasher = RightRangeHasher::new(RandomState::default(), 1..3);
        assert_eq!(
            hasher.hash_one(vec![1, 2, 3, 4].as_slice()),
            hasher.bh.hash_one(vec![2, 3].as_slice())
        );
        assert_eq!(
            hasher.hash_one(vec![1, 2, 3, 4, 5, 6].as_slice()),
            hasher.bh.hash_one(vec![4, 5].as_slice())
        );
        assert_eq!(hasher.hash_one(vec![1, 2].as_slice()), 0);
    }

    #[test]
    fn test_right_range_hasher_hash_string() {
        let hasher = RightRangeHasher::new(RandomState::default(), 3..5);
        assert_eq!(hasher.hash_one(&"abcdef".to_string()), hasher.bh.hash_one(b"bc"));
        assert_eq!(hasher.hash_one(&"abcdefghijklmn".to_string()), hasher.bh.hash_one(b"jk"));
        assert_eq!(hasher.hash_one(&"a".to_string()), 0);
    }

    #[test]
    fn test_right_range_hasher_hash_str_ref() {
        let hasher = RightRangeHasher::new(RandomState::default(), 1..3);
        assert_eq!(hasher.hash_one(&"abcd"), hasher.bh.hash_one(b"bc"));
        assert_eq!(hasher.hash_one(&"a"), 0);
    }

    #[test]
    fn test_right_range_hasher_hash_str() {
        let hasher = RightRangeHasher::new(RandomState::default(), 1..3);
        assert_eq!(hasher.hash_one("abcd"), hasher.bh.hash_one(b"bc"));
        assert_eq!(hasher.hash_one("abcdefghijklmn"), hasher.bh.hash_one(b"lm"));
        assert_eq!(hasher.hash_one("a"), 0);
    }

    #[test]
    fn test_right_range_hasher_default() {
        let hasher: RightRangeHasher = RightRangeHasher::default();
        assert_eq!(hasher.range, 0..0);
    }
}
