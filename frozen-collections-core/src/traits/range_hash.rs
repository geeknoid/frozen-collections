use std::hash::{BuildHasher, Hasher};
use std::ops::Range;

/// Enables hashing over a range of an input.
pub trait RangeHash {
    /// Hash a range of a value.
    #[must_use]
    fn hash_range<BH>(&self, bh: &BH, range: Range<usize>) -> u64
    where
        BH: BuildHasher;

    /// Hash a range of a value.
    ///
    /// # Safety
    ///
    /// The range argument is assumed to be valid. The behavior is undefined if
    /// it isn't.
    #[must_use]
    unsafe fn hash_range_unchecked<BH>(&self, bh: &BH, range: Range<usize>) -> u64
    where
        BH: BuildHasher,
    {
        self.hash_range(bh, range)
    }
}

macro_rules! range_hash_impl {
    ($type:ty, $($method:ident)*) => {
        impl RangeHash for $type {
            #[inline]
            fn hash_range<BH>(&self, bh: &BH, range: Range<usize>) -> u64
            where
                BH: BuildHasher,
            {
                let mut h = bh.build_hasher();
                let b = self.$($method().)*get(range).unwrap();
                h.write(b);
                h.finish()
            }

            #[inline]
            unsafe fn hash_range_unchecked<BH>(&self, bh: &BH, range: Range<usize>) -> u64
            where
                BH: BuildHasher
            {
                let mut h = bh.build_hasher();
                let b = unsafe { self.$($method().)*get_unchecked(range) };
                h.write(b);
                h.finish()
            }
        }
    };
}

range_hash_impl!(String, as_bytes);
range_hash_impl!(str, as_bytes);
range_hash_impl!([u8],);

#[cfg(test)]
mod tests {
    use std::hash::{Hash, RandomState};

    use super::*;

    #[test]
    fn test_range_hash_for_string() {
        let s = "12345678".to_string();
        test_range_hash(&s, s.len());
    }

    #[test]
    fn test_range_hash_for_str() {
        let s = "12345678";
        test_range_hash(s, s.len());
    }

    #[test]
    fn test_range_hash_for_u8() {
        let v = vec![0u8, 1, 2, 3, 4, 5, 6, 7];
        let b = v.as_slice();
        test_range_hash(b, b.len());
    }

    fn test_range_hash<T>(data: &T, len: usize)
    where
        T: RangeHash + Hash + ?Sized,
    {
        let bh = RandomState::new();

        let range_hash_full = data.hash_range(&bh, 0..len);
        let range_hash_partial = data.hash_range(&bh, 1..4);
        let range_hash_empty = data.hash_range(&bh, 0..0);
        let classic_hash_empty = bh.build_hasher().finish();

        assert_ne!(
            range_hash_full, range_hash_partial,
            "Hashes should differ for different ranges"
        );

        assert_eq!(
            range_hash_empty, classic_hash_empty,
            "Empty range hash should match classic hash"
        );
    }
}
