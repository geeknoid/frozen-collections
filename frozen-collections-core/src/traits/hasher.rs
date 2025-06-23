/// Hashes values of a specific type.
///
/// This provides a hashing mechanism which is orthogonal to the normal
/// [`Hash`] trait. This allows for the creation of hashers that are
/// specialized for specific types and can be used in contexts where the
/// standard `Hash` trait is not desirable.
pub trait Hasher<T>
where
    T: ?Sized,
{
    /// Produce a hash value for the given value.
    fn hash_one(&self, value: &T) -> u64;
}
