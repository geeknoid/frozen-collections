use crate::DefaultBuildHasher;
use crate::hashers::BridgeHasher;
use crate::traits::Hasher;
use core::fmt::{self, Debug};
use core::panic::{RefUnwindSafe, UnwindSafe};

#[cfg(not(feature = "std"))]
use alloc::boxed::Box;

/// Object-safe trait for dynamic string hashing dispatch.
trait DynStringHasher: Send + Sync {
    fn hash_str(&self, value: &str) -> u64;
    fn clone_box(&self) -> Box<dyn DynStringHasher>;
}

impl<H> DynStringHasher for H
where
    H: Hasher<str> + Clone + Send + Sync + 'static,
{
    fn hash_str(&self, value: &str) -> u64 {
        self.hash_one(value)
    }

    fn clone_box(&self) -> Box<dyn DynStringHasher> {
        Box::new(self.clone())
    }
}

/// Dispatches to one of several string-oriented hashers chosen at construction time.
///
#[doc = include_str!("../doc_snippets/private_api_warning.md")]
pub struct StringHasher {
    inner: Box<dyn DynStringHasher>,
}

// The inner hashers contain no interior mutability.
impl UnwindSafe for StringHasher {}
impl RefUnwindSafe for StringHasher {}

impl StringHasher {
    /// Creates a new `StringHasher` wrapping the given hasher.
    pub fn new<H>(hasher: H) -> Self
    where
        H: Hasher<str> + Clone + Send + Sync + 'static,
    {
        Self { inner: Box::new(hasher) }
    }
}

impl Hasher<str> for StringHasher {
    #[inline]
    fn hash_one(&self, value: &str) -> u64 {
        self.inner.hash_str(value)
    }
}

impl Hasher<Box<str>> for StringHasher {
    #[inline]
    fn hash_one(&self, value: &Box<str>) -> u64 {
        self.inner.hash_str(value.as_ref())
    }
}

impl Hasher<&str> for StringHasher {
    #[inline]
    fn hash_one(&self, value: &&str) -> u64 {
        self.inner.hash_str(value)
    }
}

impl Clone for StringHasher {
    fn clone(&self) -> Self {
        Self {
            inner: self.inner.clone_box(),
        }
    }
}

impl Debug for StringHasher {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("StringHasher").finish()
    }
}

impl Default for StringHasher {
    fn default() -> Self {
        Self::new(BridgeHasher::<DefaultBuildHasher>::default())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn hash_str_returns_correct_value() {
        let hasher = StringHasher::default();
        let h1 = hasher.hash_one("hello");
        let h2 = hasher.hash_one("hello");
        assert_eq!(h1, h2);

        let h3 = hasher.hash_one("world");
        assert_ne!(h1, h3);

        // Verify non-trivial: not 0 or 1
        assert_ne!(h1, 0);
        assert_ne!(h1, 1);
    }

    #[test]
    fn hash_ref_str_returns_correct_value() {
        let hasher = StringHasher::default();
        let s = "hello";
        let h1: u64 = Hasher::<&str>::hash_one(&hasher, &s);
        let h2: u64 = Hasher::<str>::hash_one(&hasher, s);
        assert_eq!(h1, h2);

        let s2 = "world";
        let h3: u64 = Hasher::<&str>::hash_one(&hasher, &s2);
        assert_ne!(h1, h3);

        // Verify non-trivial: not 0 or 1
        assert_ne!(h1, 0);
        assert_ne!(h1, 1);
    }

    #[test]
    fn debug_output_is_non_empty() {
        let hasher = StringHasher::default();
        let debug = format!("{hasher:?}");
        assert!(debug.contains("StringHasher"));
    }
}
