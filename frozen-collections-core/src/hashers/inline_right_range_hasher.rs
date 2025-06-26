use crate::DefaultBuildHasher;
use crate::traits::Hasher;
use crate::utils::cold;
use core::hash::{BuildHasher, Hash};

#[cfg(not(feature = "std"))]
use alloc::string::String;

/// Hashes a portion of a right-aligned slice.
///
#[doc = include_str!("../doc_snippets/private_api_warning.md")]
#[derive(Clone, Debug)]
pub struct InlineRightRangeHasher<const RANGE_START: usize, const RANGE_END: usize, BH = DefaultBuildHasher> {
    bh: BH,
}

impl<const RANGE_START: usize, const RANGE_END: usize, BH> InlineRightRangeHasher<RANGE_START, RANGE_END, BH> {
    /// Creates a new `InlineRightRangeHasher` with the specified `BuildHasher`.
    #[must_use]
    pub const fn new(bh: BH) -> Self {
        Self { bh }
    }
}

impl<T, const RANGE_START: usize, const RANGE_END: usize, BH> Hasher<[T]> for InlineRightRangeHasher<RANGE_START, RANGE_END, BH>
where
    T: Hash,
    BH: BuildHasher,
{
    #[inline]
    fn hash_one(&self, value: &[T]) -> u64 {
        if value.len() < RANGE_END {
            cold();
            return 0;
        }

        let effective_range = value.len() - RANGE_END..value.len() - RANGE_START;
        self.bh.hash_one(&value[effective_range])
    }
}

impl<const RANGE_START: usize, const RANGE_END: usize, BH> Hasher<str> for InlineRightRangeHasher<RANGE_START, RANGE_END, BH>
where
    BH: BuildHasher,
{
    #[inline]
    fn hash_one(&self, value: &str) -> u64 {
        let b = value.as_bytes();
        if b.len() < RANGE_END {
            cold();
            return 0;
        }

        let effective_range = value.len() - RANGE_END..value.len() - RANGE_START;
        self.bh.hash_one(&b[effective_range])
    }
}

impl<AR, const RANGE_START: usize, const RANGE_END: usize, BH> Hasher<AR> for InlineRightRangeHasher<RANGE_START, RANGE_END, BH>
where
    AR: AsRef<str>,
    BH: BuildHasher,
{
    #[inline]
    fn hash_one(&self, value: &AR) -> u64 {
        let b = value.as_ref().as_bytes();
        if b.len() < RANGE_END {
            cold();
            return 0;
        }

        let effective_range = value.as_ref().len() - RANGE_END..value.as_ref().len() - RANGE_START;
        self.bh.hash_one(&b[effective_range])
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use alloc::vec;

    #[test]
    fn test_right_range_hasher_hash_slice() {
        let hasher = InlineRightRangeHasher::<1, 3>::new(DefaultBuildHasher::default());
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
        let hasher = InlineRightRangeHasher::<3, 5>::new(DefaultBuildHasher::default());
        assert_eq!(hasher.hash_one(&"abcdef".to_string()), hasher.bh.hash_one(b"bc"));
        assert_eq!(hasher.hash_one(&"abcdefghijklmn".to_string()), hasher.bh.hash_one(b"jk"));
        assert_eq!(hasher.hash_one(&"a".to_string()), 0);
    }

    #[test]
    fn test_right_range_hasher_hash_str_ref() {
        let hasher = InlineRightRangeHasher::<1, 3>::new(DefaultBuildHasher::default());
        assert_eq!(hasher.hash_one(&"abcd"), hasher.bh.hash_one(b"bc"));
        assert_eq!(hasher.hash_one(&"abcdefghijklmn"), hasher.bh.hash_one(b"lm"));
        assert_eq!(hasher.hash_one(&"a"), 0);
    }

    #[test]
    fn test_right_range_hasher_hash_str() {
        let hasher = InlineRightRangeHasher::<1, 3>::new(DefaultBuildHasher::default());
        assert_eq!(hasher.hash_one("abcd"), hasher.bh.hash_one(b"bc"));
        assert_eq!(hasher.hash_one("abcdefghijklmn"), hasher.bh.hash_one(b"lm"));
        assert_eq!(hasher.hash_one("a"), 0);
    }
}
