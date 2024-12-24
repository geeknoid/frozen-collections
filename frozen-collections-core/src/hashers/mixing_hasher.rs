// NOTE: Currently unused, keeping it around just in case...

use crate::traits::{Hasher, Scalar};

/// A hasher for scalar values which performs mixing on the values.
///
#[doc = include_str!("../doc_snippets/private_api_warning.md")]
#[derive(Clone)]
pub struct MixingHasher {}

impl MixingHasher {
    #[must_use]
    pub const fn new() -> Self {
        Self {}
    }
}

impl<T> Hasher<&[T]> for MixingHasher {
    fn hash(&self, value: &&[T]) -> u64 {
        mix(value.len() as u64)
    }
}

impl Hasher<String> for MixingHasher {
    fn hash(&self, value: &String) -> u64 {
        mix(value.len() as u64)
    }
}

impl Hasher<str> for MixingHasher {
    fn hash(&self, value: &str) -> u64 {
        mix(value.len() as u64)
    }
}

impl Hasher<&str> for MixingHasher {
    fn hash(&self, value: &&str) -> u64 {
        mix(value.len() as u64)
    }
}

impl<S> Hasher<S> for MixingHasher
where
    S: Scalar,
{
    fn hash(&self, value: &S) -> u64 {
        mix(value.as_u64())
    }
}

impl Default for MixingHasher {
    fn default() -> Self {
        Self::new()
    }
}

/// Take an integer value and 'mix' it into a hash code.
///
/// This function is based on code from <https://github.com/ztanml/fast-hash>.
#[must_use]
#[inline]
const fn mix(mut value: u64) -> u64 {
    value ^= value.wrapping_shr(23);
    value = value.wrapping_mul(0x2127_599b_f432_5c37);
    value ^= value.wrapping_shr(47);
    value
}
