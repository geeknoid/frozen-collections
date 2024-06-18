use crate::traits::Hasher;
use ahash::RandomState;
use core::hash::{BuildHasher, Hash};
use core::ops::Range;

/// Hashes a portion of a left-aligned slice.
///
#[doc = include_str!("../doc_snippets/type_compat_warning.md")]
#[derive(Clone)]
pub struct LeftRangeHasher<BH = RandomState> {
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
    fn hash(&self, value: &[T]) -> u64 {
        self.bh.hash_one(&value[self.range.clone()])
    }
}

impl<BH> Hasher<String> for LeftRangeHasher<BH>
where
    BH: BuildHasher,
{
    fn hash(&self, value: &String) -> u64 {
        self.bh.hash_one(&value.as_bytes()[self.range.clone()])
    }
}

impl<BH> Hasher<&str> for LeftRangeHasher<BH>
where
    BH: BuildHasher,
{
    fn hash(&self, value: &&str) -> u64 {
        self.bh.hash_one(&value.as_bytes()[self.range.clone()])
    }
}

impl<BH> Hasher<str> for LeftRangeHasher<BH>
where
    BH: BuildHasher,
{
    fn hash(&self, value: &str) -> u64 {
        self.bh.hash_one(&value.as_bytes()[self.range.clone()])
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
