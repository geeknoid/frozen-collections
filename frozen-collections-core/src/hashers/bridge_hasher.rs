use crate::DefaultHashBuilder;
use crate::traits::Hasher;
use core::hash::{BuildHasher, Hash};

/// Wraps a normal [`BuildHasher`].
///
#[doc = include_str!("../doc_snippets/private_api_warning.md")]
#[derive(Clone)]
pub struct BridgeHasher<BH = DefaultHashBuilder> {
    bh: BH,
}

impl<BH> BridgeHasher<BH> {
    #[must_use]
    pub const fn new(bh: BH) -> Self {
        Self { bh }
    }
}

impl<T, BH> Hasher<T> for BridgeHasher<BH>
where
    T: ?Sized + Hash,
    BH: BuildHasher,
{
    fn hash(&self, value: &T) -> u64 {
        self.bh.hash_one(value)
    }
}

impl<BH> Default for BridgeHasher<BH>
where
    BH: Default,
{
    fn default() -> Self {
        Self::new(BH::default())
    }
}
