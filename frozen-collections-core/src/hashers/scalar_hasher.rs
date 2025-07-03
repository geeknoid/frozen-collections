use crate::traits::{Hasher, Scalar};

/// Returns the value itself as the hash.
///
#[doc = include_str!("../doc_snippets/private_api_warning.md")]
#[derive(Clone, Debug, Default)]
pub struct ScalarHasher;

impl<S> Hasher<S> for ScalarHasher
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
    use super::ScalarHasher;
    use crate::traits::Hasher;

    #[test]
    fn test_scalar_hasher_hash_one() {
        let hasher = ScalarHasher;

        assert_eq!(hasher.hash_one(&42u64), 42u64);
        assert_eq!(hasher.hash_one(&0u64), 0u64);
    }
}
