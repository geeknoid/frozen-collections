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
