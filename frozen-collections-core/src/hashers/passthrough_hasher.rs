use crate::traits::{Hasher, Scalar};

#[cfg(not(feature = "std"))]
use alloc::string::String;

/// A hasher that simply returns the value as the hash.
///
#[doc = include_str!("../doc_snippets/private_api_warning.md")]
#[derive(Clone, Debug, Default)]
pub struct PassthroughHasher;

impl<T> Hasher<[T]> for PassthroughHasher {
    #[inline]
    fn hash_one(&self, value: &[T]) -> u64 {
        value.len() as u64
    }
}

impl Hasher<String> for PassthroughHasher {
    #[inline]
    fn hash_one(&self, value: &String) -> u64 {
        value.len() as u64
    }
}

impl Hasher<str> for PassthroughHasher {
    #[inline]
    fn hash_one(&self, value: &str) -> u64 {
        value.len() as u64
    }
}

impl Hasher<&str> for PassthroughHasher {
    #[inline]
    fn hash_one(&self, value: &&str) -> u64 {
        value.len() as u64
    }
}

impl<S> Hasher<S> for PassthroughHasher
where
    S: Scalar,
{
    #[inline]
    fn hash_one(&self, value: &S) -> u64 {
        value.index() as u64
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use alloc::vec;

    #[test]
    fn hash_string_returns_length() {
        let hasher = PassthroughHasher {};
        let value = String::from("hello");
        assert_eq!(hasher.hash_one(&value), 5);
    }

    #[test]
    fn hash_str_returns_length() {
        let hasher = PassthroughHasher {};
        let value = "world";
        assert_eq!(hasher.hash_one(value), 5);
    }

    #[test]
    fn hash_slice_returns_length() {
        let hasher = PassthroughHasher {};
        let binding = vec![1, 2, 3, 4];
        let value = binding.as_slice();
        assert_eq!(hasher.hash_one(value), 4);
    }

    #[test]
    fn hash_scalar_returns_index() {
        let hasher = PassthroughHasher {};
        let index = Scalar::index(&42) as u64;
        assert_eq!(hasher.hash_one(&42), index);
    }

    #[test]
    fn default_creates_instance() {
        #[expect(clippy::default_constructed_unit_structs, reason = "Testing Default trait")]
        let hasher = PassthroughHasher::default();

        assert_eq!(hasher.hash_one(&"default"), 7);
    }
}
