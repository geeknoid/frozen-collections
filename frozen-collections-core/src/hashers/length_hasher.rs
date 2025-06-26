use crate::traits::{Hasher, Len};

/// Returns the value's length as the hash.
///
#[doc = include_str!("../doc_snippets/private_api_warning.md")]
#[derive(Clone, Debug, Default)]
pub struct LengthHasher;

impl<T> Hasher<T> for LengthHasher
where
    T: ?Sized + Len,
{
    #[inline]
    fn hash_one(&self, value: &T) -> u64 {
        value.len() as u64
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use alloc::vec;

    #[test]
    fn hash_string_returns_length() {
        let hasher = LengthHasher {};
        let value = String::from("hello");
        assert_eq!(hasher.hash_one(&value), 5);
    }

    #[test]
    fn hash_str_returns_length() {
        let hasher = LengthHasher {};
        let value = "world";
        assert_eq!(hasher.hash_one(value), 5);
    }

    #[test]
    fn hash_slice_returns_length() {
        let hasher = LengthHasher {};
        let binding = vec![1, 2, 3, 4];
        let value = binding.as_slice();
        assert_eq!(hasher.hash_one(value), 4);
    }

    #[test]
    fn default_creates_instance() {
        #[expect(clippy::default_constructed_unit_structs, reason = "Testing Default trait")]
        let hasher = LengthHasher::default();

        assert_eq!(hasher.hash_one(&"default"), 7);
    }
}
