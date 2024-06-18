use crate::traits::{Hasher, Scalar};

/// A hasher that simply returns the value as the hash.
///
#[doc = include_str!("../doc_snippets/type_compat_warning.md")]
#[derive(Clone)]
pub struct PassthroughHasher {}

impl PassthroughHasher {
    #[must_use]
    pub const fn new() -> Self {
        Self {}
    }
}

impl<T> Hasher<&[T]> for PassthroughHasher {
    fn hash(&self, value: &&[T]) -> u64 {
        value.len() as u64
    }
}

impl Hasher<String> for PassthroughHasher {
    fn hash(&self, value: &String) -> u64 {
        value.len() as u64
    }
}

impl Hasher<str> for PassthroughHasher {
    fn hash(&self, value: &str) -> u64 {
        value.len() as u64
    }
}

impl Hasher<&str> for PassthroughHasher {
    fn hash(&self, value: &&str) -> u64 {
        value.len() as u64
    }
}

impl<S> Hasher<S> for PassthroughHasher
where
    S: Scalar,
{
    fn hash(&self, value: &S) -> u64 {
        value.index() as u64
    }
}

impl Default for PassthroughHasher {
    fn default() -> Self {
        Self::new()
    }
}
