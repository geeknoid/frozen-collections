use crate::traits::Hasher;
use crate::utils::cold;
use crate::DefaultHashBuilder;
use alloc::string::String;
use core::hash::{BuildHasher, Hash};
use core::ops::Range;

/// Hashes a portion of a left-aligned slice.
///
#[doc = include_str!("../doc_snippets/private_api_warning.md")]
#[derive(Clone)]
pub struct LeftRangeHasher<BH = DefaultHashBuilder> {
    bh: BH,
    range: Range<usize>,
}

impl<BH> LeftRangeHasher<BH> {
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
    fn hash(&self, value: &[T]) -> u64 {
        if value.len() < self.range.end {
            return 0;
        }

        self.bh.hash_one(&value[self.range.clone()])
    }
}

impl<BH> Hasher<String> for LeftRangeHasher<BH>
where
    BH: BuildHasher,
{
    #[inline]
    fn hash(&self, value: &String) -> u64 {
        let b = value.as_bytes();
        if b.len() < self.range.end {
            cold();
            return 0;
        }

        self.bh.hash_one(&b[self.range.clone()])
        /*
               let mut hash_code = [0, 0, 0, 0, 0, 0, 0, 0];
               for (index, i) in self.range.clone().enumerate() {
                   hash_code[index] = b[i];
               }

               u64::from_ne_bytes(hash_code)
        */
    }
}

impl<BH> Hasher<&str> for LeftRangeHasher<BH>
where
    BH: BuildHasher,
{
    #[inline]
    fn hash(&self, value: &&str) -> u64 {
        let b = value.as_bytes();
        if b.len() < self.range.end {
            cold();
            return 0;
        }

        self.bh.hash_one(&b[self.range.clone()])
        /*
               let mut hash_code = [0, 0, 0, 0, 0, 0, 0, 0];
               for (index, i) in self.range.clone().enumerate() {
                   hash_code[index] = b[i];
               }

               u64::from_ne_bytes(hash_code)
        */
    }
}

impl<BH> Hasher<str> for LeftRangeHasher<BH>
where
    BH: BuildHasher,
{
    #[inline]
    fn hash(&self, value: &str) -> u64 {
        let b = value.as_bytes();
        if b.len() < self.range.end {
            cold();
            return 0;
        }

        self.bh.hash_one(&b[self.range.clone()])
        /*
               let mut hash_code = [0, 0, 0, 0, 0, 0, 0, 0];
               for (index, i) in self.range.clone().enumerate() {
                   hash_code[index] = b[i];
               }

               u64::from_ne_bytes(hash_code)
        */
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
    //    use alloc::string::ToString;
    use alloc::vec;
    use foldhash::fast::RandomState;

    #[test]
    fn test_left_range_hasher_hash_slice() {
        let hasher = LeftRangeHasher::new(RandomState::default(), 0..3);
        assert_eq!(
            hasher.hash(vec![1, 2, 3, 4].as_slice()),
            hasher.bh.hash_one(vec![1, 2, 3].as_slice())
        );
        assert_eq!(hasher.hash(vec![1, 2].as_slice()), 0);
    }
    /*
        #[test]
        fn test_left_range_hasher_hash_string() {
            let hasher = LeftRangeHasher::new(RandomState::default(), 0..3);
            assert_eq!(hasher.hash(&"abcd".to_string()), hasher.bh.hash_one(b"abc"));
            assert_eq!(hasher.hash(&"ab".to_string()), 0);
        }

        #[test]
        fn test_left_range_hasher_hash_str_ref() {
            let hasher = LeftRangeHasher::new(RandomState::default(), 0..3);
            assert_eq!(hasher.hash(&"abcd"), hasher.bh.hash_one(b"abc"));
            assert_eq!(hasher.hash(&"ab"), 0);
        }

        #[test]
        fn test_left_range_hasher_hash_str() {
            let hasher = LeftRangeHasher::new(RandomState::default(), 0..3);
            assert_eq!(hasher.hash("abcd"), hasher.bh.hash_one(b"abc"));
            assert_eq!(hasher.hash("ab"), 0);
        }
    */
    #[test]
    fn test_left_range_hasher_default() {
        let hasher: LeftRangeHasher = LeftRangeHasher::default();
        assert_eq!(hasher.range, 0..0);
    }
}
