use crate::DefaultBuildHasher;
use crate::traits::Hasher;
use core::hash::{BuildHasher, Hash};

/// Wraps a normal [`BuildHasher`].
///
#[doc = include_str!("../doc_snippets/private_api_warning.md")]
#[derive(Clone, Debug)]
pub struct BridgeHasher<BH = DefaultBuildHasher> {
    bh: BH,
}

impl<BH> BridgeHasher<BH> {
    /// Creates a new `BridgeHasher` with the given `BuildHasher` to bridge to.
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
    #[inline]
    fn hash_one(&self, value: &T) -> u64 {
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hash_one() {
        let bh = DefaultBuildHasher::default();
        let hasher = BridgeHasher::new(bh);
        let value = "test_string";
        assert_eq!(hasher.hash_one(&value), bh.hash_one(value));
    }
}
