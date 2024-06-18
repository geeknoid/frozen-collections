use crate::traits::Hasher;
use ahash::RandomState;
use core::hash::{BuildHasher, Hash};

/// Hashes a portion of a right-aligned slice.
///
#[doc = include_str!("../doc_snippets/type_compat_warning.md")]
#[derive(Clone)]
pub struct InlineRightRangeHasher<
    const RANGE_START: usize,
    const RANGE_END: usize,
    BH = RandomState,
> {
    bh: BH,
}

impl<const RANGE_START: usize, const RANGE_END: usize, BH>
    InlineRightRangeHasher<RANGE_START, RANGE_END, BH>
{
    #[must_use]
    pub const fn new(bh: BH) -> Self {
        Self { bh }
    }
}

impl<T, const RANGE_START: usize, const RANGE_END: usize, BH> Hasher<[T]>
    for InlineRightRangeHasher<RANGE_START, RANGE_END, BH>
where
    T: Hash,
    BH: BuildHasher,
{
    fn hash(&self, value: &[T]) -> u64 {
        let effective_range = value.len() - RANGE_END..value.len() - RANGE_START;
        self.bh.hash_one(&value[effective_range])
    }
}

impl<const RANGE_START: usize, const RANGE_END: usize, BH> Hasher<String>
    for InlineRightRangeHasher<RANGE_START, RANGE_END, BH>
where
    BH: BuildHasher,
{
    fn hash(&self, value: &String) -> u64 {
        let effective_range = value.len() - RANGE_END..value.len() - RANGE_START;
        self.bh.hash_one(&value.as_bytes()[effective_range])
    }
}

impl<const RANGE_START: usize, const RANGE_END: usize, BH> Hasher<&str>
    for InlineRightRangeHasher<RANGE_START, RANGE_END, BH>
where
    BH: BuildHasher,
{
    fn hash(&self, value: &&str) -> u64 {
        let effective_range = value.len() - RANGE_END..value.len() - RANGE_START;
        self.bh.hash_one(&value.as_bytes()[effective_range])
    }
}

impl<const RANGE_START: usize, const RANGE_END: usize, BH> Hasher<str>
    for InlineRightRangeHasher<RANGE_START, RANGE_END, BH>
where
    BH: BuildHasher,
{
    fn hash(&self, value: &str) -> u64 {
        let effective_range = value.len() - RANGE_END..value.len() - RANGE_START;
        self.bh.hash_one(&value.as_bytes()[effective_range])
    }
}

impl<const RANGE_START: usize, const RANGE_END: usize, BH> Default
    for InlineRightRangeHasher<RANGE_START, RANGE_END, BH>
where
    BH: Default,
{
    fn default() -> Self {
        Self::new(BH::default())
    }
}
