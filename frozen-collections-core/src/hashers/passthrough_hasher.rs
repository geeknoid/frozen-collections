use crate::traits::{Hasher, Scalar};
use alloc::string::String;

/// A hasher that simply returns the value as the hash.
///
#[doc = include_str!("../doc_snippets/private_api_warning.md")]
#[derive(Clone)]
pub struct PassthroughHasher {}

impl PassthroughHasher {
    #[must_use]
    pub const fn new() -> Self {
        Self {}
    }
}

impl<T> Hasher<[T]> for PassthroughHasher {
    fn hash(&self, value: &[T]) -> u64 {
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
#[cfg(test)]
mod tests {
    use super::*;
    use alloc::vec;

    #[test]
    fn hash_string_returns_length() {
        let hasher = PassthroughHasher::new();
        let value = String::from("hello");
        assert_eq!(hasher.hash(&value), 5);
    }

    #[test]
    fn hash_str_returns_length() {
        let hasher = PassthroughHasher::new();
        let value = "world";
        assert_eq!(hasher.hash(value), 5);
    }

    #[test]
    fn hash_slice_returns_length() {
        let hasher = PassthroughHasher::new();
        let binding = vec![1, 2, 3, 4];
        let value = binding.as_slice();
        assert_eq!(hasher.hash(value), 4);
    }

    #[test]
    fn hash_scalar_returns_index() {
        let hasher = PassthroughHasher::new();
        let index = Scalar::index(&42) as u64;
        assert_eq!(hasher.hash(&42), index);
    }

    #[test]
    fn default_creates_instance() {
        let hasher = PassthroughHasher::default();
        assert_eq!(hasher.hash(&"default"), 7);
    }
}
