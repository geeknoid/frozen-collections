//! Simple bit vectors.

#[cfg(not(feature = "std"))]
use alloc::boxed::Box;

pub struct BitVec {
    bits: Box<[u64]>,
    len: usize,
}

impl BitVec {
    pub(crate) fn with_capacity(capacity: usize) -> Self {
        Self {
            bits: (0..(capacity as u64).div_ceil(64)).collect(),
            len: capacity,
        }
    }

    pub(crate) fn clear_all(&mut self) {
        self.bits.fill(0);
    }

    pub(crate) fn set(&mut self, index: usize) {
        debug_assert!(index < self.len, "Out of bounds");

        self.bits[index / 64] |= 1 << (index % 64);
    }

    pub(crate) fn get(&self, index: usize) -> bool {
        debug_assert!(index < self.len, "Out of bounds");

        (self.bits[index / 64] & (1 << (index % 64))) != 0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_bitvec() {
        const LEN: usize = 125;

        let mut bitvec = BitVec::with_capacity(LEN);
        assert_eq!(2, bitvec.bits.len());

        bitvec.clear_all();
        for i in 0..LEN {
            assert!(!bitvec.get(i));
        }

        for i in 0..LEN {
            bitvec.set(i);
            bitvec.set(i);
            assert!(bitvec.get(i));
        }
    }

    #[test]
    #[should_panic(expected = "Out of bounds")]
    fn get_panic() {
        let bitvec = BitVec::with_capacity(12);
        _ = bitvec.get(12);
    }

    #[test]
    #[should_panic(expected = "Out of bounds")]
    fn set_panic() {
        let mut bitvec = BitVec::with_capacity(12);
        bitvec.set(12);
    }
}
