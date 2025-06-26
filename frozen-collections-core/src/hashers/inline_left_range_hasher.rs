use crate::DefaultBuildHasher;
use crate::traits::Hasher;
use crate::utils::cold;
use core::hash::{BuildHasher, Hash};

#[cfg(not(feature = "std"))]
use alloc::string::String;

/// Hashes a portion of a left-aligned slice.
///
#[doc = include_str!("../doc_snippets/private_api_warning.md")]
#[derive(Clone, Debug)]
pub struct InlineLeftRangeHasher<const RANGE_START: usize, const RANGE_END: usize, BH = DefaultBuildHasher> {
    bh: BH,
}

impl<const RANGE_START: usize, const RANGE_END: usize, BH> InlineLeftRangeHasher<RANGE_START, RANGE_END, BH> {
    /// Creates a new `InlineLeftRangeHasher` with the specified `BuildHasher`.
    #[must_use]
    pub const fn new(bh: BH) -> Self {
        Self { bh }
    }
}

impl<T, const RANGE_START: usize, const RANGE_END: usize, BH> Hasher<[T]> for InlineLeftRangeHasher<RANGE_START, RANGE_END, BH>
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

        self.bh.hash_one(&value[RANGE_START..RANGE_END])
    }
}

impl<const RANGE_START: usize, const RANGE_END: usize, BH> Hasher<str> for InlineLeftRangeHasher<RANGE_START, RANGE_END, BH>
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

        self.bh.hash_one(&b[RANGE_START..RANGE_END])
    }
}

impl<AR, const RANGE_START: usize, const RANGE_END: usize, BH> Hasher<AR> for InlineLeftRangeHasher<RANGE_START, RANGE_END, BH>
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

        self.bh.hash_one(&b[RANGE_START..RANGE_END])
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use alloc::vec;

    #[test]
    fn test_left_range_hasher_hash_slice() {
        let hasher = InlineLeftRangeHasher::<0, 3>::new(DefaultBuildHasher::default());
        assert_eq!(
            hasher.hash_one(vec![1, 2, 3, 4].as_slice()),
            hasher.bh.hash_one(vec![1, 2, 3].as_slice())
        );
        assert_eq!(hasher.hash_one(vec![1, 2].as_slice()), 0);
    }

    #[test]
    fn test_left_range_hasher_hash_string() {
        let hasher = InlineLeftRangeHasher::<0, 3>::new(DefaultBuildHasher::default());
        assert_eq!(hasher.hash_one(&"abcd".to_string()), hasher.bh.hash_one(b"abc"));
        assert_eq!(hasher.hash_one(&"ab".to_string()), 0);
    }

    #[test]
    fn test_left_range_hasher_hash_str_ref() {
        let hasher = InlineLeftRangeHasher::<0, 3>::new(DefaultBuildHasher::default());
        assert_eq!(hasher.hash_one(&"abcd"), hasher.bh.hash_one(b"abc"));
        assert_eq!(hasher.hash_one(&"ab"), 0);
    }

    #[test]
    fn test_left_range_hasher_hash_str() {
        let hasher = InlineLeftRangeHasher::<0, 3>::new(DefaultBuildHasher::default());
        assert_eq!(hasher.hash_one("abcd"), hasher.bh.hash_one(b"abc"));
        assert_eq!(hasher.hash_one("ab"), 0);
    }
}
