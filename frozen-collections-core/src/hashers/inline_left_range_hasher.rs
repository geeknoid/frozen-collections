use crate::traits::Hasher;
use ahash::RandomState;
use core::hash::{BuildHasher, Hash};

/// Hashes a portion of a left-aligned slice.
///
#[doc = include_str!("../doc_snippets/type_compat_warning.md")]
#[derive(Clone)]
pub struct InlineLeftRangeHasher<const RANGE_START: usize, const RANGE_END: usize, BH = RandomState>
{
    bh: BH,
}

impl<const RANGE_START: usize, const RANGE_END: usize, BH>
    InlineLeftRangeHasher<RANGE_START, RANGE_END, BH>
{
    #[must_use]
    pub const fn new(bh: BH) -> Self {
        Self { bh }
    }
}

impl<T, const RANGE_START: usize, const RANGE_END: usize, BH> Hasher<[T]>
    for InlineLeftRangeHasher<RANGE_START, RANGE_END, BH>
where
    T: Hash,
    BH: BuildHasher,
{
    fn hash(&self, value: &[T]) -> u64 {
        self.bh.hash_one(&value[RANGE_START..RANGE_END])
    }
}

impl<const RANGE_START: usize, const RANGE_END: usize, BH> Hasher<String>
    for InlineLeftRangeHasher<RANGE_START, RANGE_END, BH>
where
    BH: BuildHasher,
{
    fn hash(&self, value: &String) -> u64 {
        self.bh.hash_one(&value.as_bytes()[RANGE_START..RANGE_END])
    }
}

impl<const RANGE_START: usize, const RANGE_END: usize, BH> Hasher<&str>
    for InlineLeftRangeHasher<RANGE_START, RANGE_END, BH>
where
    BH: BuildHasher,
{
    fn hash(&self, value: &&str) -> u64 {
        self.bh.hash_one(&value.as_bytes()[RANGE_START..RANGE_END])
    }
}

impl<const RANGE_START: usize, const RANGE_END: usize, BH> Hasher<str>
    for InlineLeftRangeHasher<RANGE_START, RANGE_END, BH>
where
    BH: BuildHasher,
{
    fn hash(&self, value: &str) -> u64 {
        self.bh.hash_one(&value.as_bytes()[RANGE_START..RANGE_END])
    }
}

impl<const RANGE_START: usize, const RANGE_END: usize, BH> Default
    for InlineLeftRangeHasher<RANGE_START, RANGE_END, BH>
where
    BH: Default,
{
    fn default() -> Self {
        Self::new(BH::default())
    }
}
