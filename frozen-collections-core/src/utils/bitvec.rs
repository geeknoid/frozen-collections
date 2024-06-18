//! Simple bit vectors.

use alloc::vec;
use alloc::vec::Vec;

pub struct BitVec {
    bits: Vec<u64>,
    len: usize,
}

impl BitVec {
    pub fn with_capacity(capacity: usize) -> Self {
        Self {
            bits: vec![0; (capacity + 63) / 64],
            len: capacity,
        }
    }

    pub fn fill(&mut self, value: bool) {
        if value {
            self.bits.fill(0xffff_ffff_ffff_ffff);
        } else {
            self.bits.fill(0);
        }
    }

    pub fn set(&mut self, index: usize, value: bool) {
        assert!(index < self.len, "Out of bounds");

        if value {
            self.bits[index / 64] |= 1 << (index % 64);
        } else {
            self.bits[index / 64] &= !(1 << (index % 64));
        }
    }

    pub fn get(&self, index: usize) -> bool {
        assert!(index < self.len, "Out of bounds");

        (self.bits[index / 64] & 1 << (index % 64)) != 0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_bitvec() {
        const LEN: usize = 128;

        let mut bitvec = BitVec::with_capacity(LEN);

        bitvec.fill(false);
        for i in 0..LEN {
            assert!(!bitvec.get(i));
        }

        bitvec.fill(true);
        for i in 0..LEN {
            assert!(bitvec.get(i));
        }

        for i in 0..LEN {
            bitvec.set(i, false);
            assert!(!bitvec.get(i));
        }
    }

    #[test]
    #[should_panic]
    #[allow(clippy::should_panic_without_expect)]
    fn get_panic() {
        let bitvec = BitVec::with_capacity(12);
        bitvec.get(12);
    }

    #[test]
    #[should_panic]
    #[allow(clippy::should_panic_without_expect)]
    fn set_panic() {
        let mut bitvec = BitVec::with_capacity(12);
        bitvec.set(12, false);
    }
}
