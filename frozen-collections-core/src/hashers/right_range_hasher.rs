use crate::traits::Hasher;
use ahash::RandomState;
use core::hash::{BuildHasher, Hash};
use core::ops::Range;

/// Hashes a portion of a right-aligned slice.
///
#[doc = include_str!("../doc_snippets/type_compat_warning.md")]
#[derive(Clone)]
pub struct RightRangeHasher<BH = RandomState> {
    bh: BH,
    range: Range<usize>,
}

impl<BH> RightRangeHasher<BH> {
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
    fn hash(&self, value: &[T]) -> u64 {
        let effective_range = value.len() - self.range.end..value.len() - self.range.start;
        self.bh.hash_one(&value[effective_range])
    }
}

impl<BH> Hasher<String> for RightRangeHasher<BH>
where
    BH: BuildHasher,
{
    fn hash(&self, value: &String) -> u64 {
        let effective_range = value.len() - self.range.end..value.len() - self.range.start;
        self.bh.hash_one(&value.as_bytes()[effective_range])
    }
}

impl<BH> Hasher<&str> for RightRangeHasher<BH>
where
    BH: BuildHasher,
{
    fn hash(&self, value: &&str) -> u64 {
        let effective_range = value.len() - self.range.end..value.len() - self.range.start;
        self.bh.hash_one(&value.as_bytes()[effective_range])
    }
}

impl<BH> Hasher<str> for RightRangeHasher<BH>
where
    BH: BuildHasher,
{
    fn hash(&self, value: &str) -> u64 {
        let effective_range = value.len() - self.range.end..value.len() - self.range.start;
        self.bh.hash_one(&value.as_bytes()[effective_range])
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
