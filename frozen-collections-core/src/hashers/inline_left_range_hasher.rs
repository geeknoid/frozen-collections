use crate::DefaultHashBuilder;
use crate::traits::Hasher;
use crate::utils::cold;
use alloc::string::String;
use core::hash::{BuildHasher, Hash};

/// Hashes a portion of a left-aligned slice.
///
#[doc = include_str!("../doc_snippets/private_api_warning.md")]
#[derive(Clone, Debug)]
pub struct InlineLeftRangeHasher<const RANGE_START: usize, const RANGE_END: usize, BH = DefaultHashBuilder> {
    bh: BH,
}

impl<const RANGE_START: usize, const RANGE_END: usize, BH> InlineLeftRangeHasher<RANGE_START, RANGE_END, BH> {
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
    fn hash(&self, value: &[T]) -> u64 {
        if value.len() < RANGE_END {
            cold();
            return 0;
        }

        self.bh.hash_one(&value[RANGE_START..RANGE_END])
    }
}

impl<const RANGE_START: usize, const RANGE_END: usize, BH> Hasher<String> for InlineLeftRangeHasher<RANGE_START, RANGE_END, BH>
where
    BH: BuildHasher,
{
    #[inline]
    fn hash(&self, value: &String) -> u64 {
        let b = value.as_bytes();
        if b.len() < RANGE_END {
            cold();
            return 0;
        }

        self.bh.hash_one(&b[RANGE_START..RANGE_END])
    }
}

impl<const RANGE_START: usize, const RANGE_END: usize, BH> Hasher<&str> for InlineLeftRangeHasher<RANGE_START, RANGE_END, BH>
where
    BH: BuildHasher,
{
    #[inline]
    fn hash(&self, value: &&str) -> u64 {
        let b = value.as_bytes();
        if b.len() < RANGE_END {
            cold();
            return 0;
        }

        self.bh.hash_one(&b[RANGE_START..RANGE_END])
    }
}

impl<const RANGE_START: usize, const RANGE_END: usize, BH> Hasher<str> for InlineLeftRangeHasher<RANGE_START, RANGE_END, BH>
where
    BH: BuildHasher,
{
    #[inline]
    fn hash(&self, value: &str) -> u64 {
        let b = value.as_bytes();
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
    use alloc::string::ToString;
    use alloc::vec;
    use foldhash::fast::RandomState;

    #[test]
    fn test_left_range_hasher_hash_slice() {
        let hasher = InlineLeftRangeHasher::<0, 3>::new(RandomState::default());
        assert_eq!(
            hasher.hash(vec![1, 2, 3, 4].as_slice()),
            hasher.bh.hash_one(vec![1, 2, 3].as_slice())
        );
        assert_eq!(hasher.hash(vec![1, 2].as_slice()), 0);
    }

    #[test]
    fn test_left_range_hasher_hash_string() {
        let hasher = InlineLeftRangeHasher::<0, 3>::new(RandomState::default());
        assert_eq!(hasher.hash(&"abcd".to_string()), hasher.bh.hash_one(b"abc"));
        assert_eq!(hasher.hash(&"ab".to_string()), 0);
    }

    #[test]
    fn test_left_range_hasher_hash_str_ref() {
        let hasher = InlineLeftRangeHasher::<0, 3>::new(RandomState::default());
        assert_eq!(hasher.hash(&"abcd"), hasher.bh.hash_one(b"abc"));
        assert_eq!(hasher.hash(&"ab"), 0);
    }

    #[test]
    fn test_left_range_hasher_hash_str() {
        let hasher = InlineLeftRangeHasher::<0, 3>::new(RandomState::default());
        assert_eq!(hasher.hash("abcd"), hasher.bh.hash_one(b"abc"));
        assert_eq!(hasher.hash("ab"), 0);
    }
}
