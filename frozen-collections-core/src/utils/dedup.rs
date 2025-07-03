//! Duplicate removal utility functions for frozen collections.

use core::cmp::Ordering;
use core::ops::Index;
use hashbrown::HashSet as HashbrownSet;
use hashbrown::HashTable as HashbrownTable;

#[cfg(not(feature = "std"))]
use {alloc::boxed::Box, alloc::vec::Vec};

pub struct DeduppedVec<T> {
    inner: Vec<T>,
}

impl<T> DeduppedVec<T> {
    pub fn using_eq(mut entries: Vec<T>, cmp: impl Fn(&T, &T) -> bool) -> Self {
        if entries.len() >= 2 {
            let mut dupes = HashbrownSet::new();
            for i in 0..entries.len() {
                for j in (i + 1)..entries.len() {
                    let (s0, s1) = entries.split_at_mut(j);
                    if cmp(&mut s0[i], &mut s1[0]) {
                        _ = dupes.insert(i);
                        break;
                    }
                }
            }

            if !dupes.is_empty() {
                let mut index = 0;
                entries.retain(|_| {
                    let result = !dupes.contains(&index);
                    index += 1;
                    result
                });
            }
        }

        Self { inner: entries }
    }

    pub fn using_cmp(entries: Vec<T>, cmp: impl Fn(&T, &T) -> Ordering) -> Self {
        let v = SortedAndDeduppedVec::new(entries, cmp);
        Self { inner: v.inner }
    }

    pub fn using_hash(mut entries: Vec<T>, hasher: impl Fn(&T) -> u64, eq: impl Fn(&T, &T) -> bool) -> Self {
        if entries.len() >= 2 {
            let mut dupes = Vec::new();
            let mut keep = HashbrownTable::with_capacity(entries.len());
            for (index, value) in entries.iter().enumerate() {
                let hash = hasher(value);

                let r = keep.find_entry(hash, |other| eq(value, &entries[*other]));
                if let Ok(entry) = r {
                    dupes.push(*entry.get());
                    _ = entry.remove();
                }

                _ = keep.insert_unique(hash, index, |x| hasher(&entries[*x]));
            }

            if !dupes.is_empty() {
                // remove the duplicates from the input vector

                let mut index = 0;
                entries.retain(|_| {
                    let result = !dupes.contains(&index);
                    index += 1;
                    result
                });
            }
        }

        Self { inner: entries }
    }

    pub fn into_boxed_slice(self) -> Box<[T]> {
        self.inner.into()
    }

    pub fn into_vec(self) -> Vec<T> {
        self.inner
    }

    pub fn into_array<const N: usize>(self) -> [T; N] {
        self.inner
            .try_into()
            .unwrap_or_else(|_| panic!("Cannot convert to array of size {N}: length mismatch"))
    }

    pub const fn is_empty(&self) -> bool {
        self.inner.is_empty()
    }

    pub const fn len(&self) -> usize {
        self.inner.len()
    }

    pub fn iter(&self) -> impl Iterator<Item = &T> {
        self.inner.iter()
    }
}

impl<T> From<SortedAndDeduppedVec<T>> for DeduppedVec<T> {
    fn from(sorted_and_dedupped: SortedAndDeduppedVec<T>) -> Self {
        Self {
            inner: sorted_and_dedupped.inner,
        }
    }
}

pub struct SortedAndDeduppedVec<T> {
    inner: Vec<T>,
}

impl<T> SortedAndDeduppedVec<T> {
    pub fn new(mut entries: Vec<T>, cmp: impl Fn(&T, &T) -> Ordering) -> Self {
        entries.sort_by(|x, y| cmp(x, y));

        if entries.len() >= 2 {
            let mut dupes = HashbrownSet::new();
            for i in 0..entries.len() - 1 {
                let (s0, s1) = entries.split_at_mut(i + 1);
                if cmp(&s0[i], &s1[0]) == Ordering::Equal {
                    _ = dupes.insert(i);
                }
            }

            if !dupes.is_empty() {
                let mut index = 0;
                entries.retain(|_| {
                    let result = !dupes.contains(&index);
                    index += 1;
                    result
                });
            }
        }

        Self { inner: entries }
    }

    pub fn into_boxed_slice(self) -> Box<[T]> {
        self.inner.into()
    }

    pub fn into_vec(self) -> Vec<T> {
        self.inner
    }

    pub fn iter(&self) -> impl Iterator<Item = &T> {
        self.inner.iter()
    }

    pub const fn is_empty(&self) -> bool {
        self.inner.is_empty()
    }

    pub const fn len(&self) -> usize {
        self.inner.len()
    }
}

impl<T> Index<usize> for SortedAndDeduppedVec<T> {
    type Output = T;

    fn index(&self, index: usize) -> &Self::Output {
        &self.inner[index]
    }
}

/// Look for the first duplicate value if any (assumes `values` is a relatively small array).
pub fn has_duplicates<T>(values: &[T]) -> bool
where
    T: Eq,
{
    for i in 0..values.len() {
        for j in 0..i {
            if values[j] == values[i] {
                return true;
            }
        }
    }

    false
}

#[cfg(test)]
mod tests {
    use super::*;
    use alloc::vec;

    #[test]
    fn test_slow_dedup_by_keep_last_no_duplicates() {
        let vec = vec![(1, "one"), (2, "two"), (3, "three")];
        let entries = DeduppedVec::using_eq(vec, |x, y| x.0.eq(&y.0));
        assert_eq!(entries.inner, vec![(1, "one"), (2, "two"), (3, "three")]);
    }

    #[test]
    fn test_slow_dedup_by_keep_last_with_duplicates() {
        let vec = vec![
            (1, "one"),
            (2, "two"),
            (2, "two duplicate"),
            (3, "three"),
            (3, "three duplicate"),
            (3, "three last"),
        ];

        let entries = DeduppedVec::using_eq(vec, |x, y| x.0.eq(&y.0));
        assert_eq!(entries.inner, vec![(1, "one"), (2, "two duplicate"), (3, "three last")]);
    }

    #[test]
    fn test_slow_dedup_by_keep_last_empty_vector() {
        let vec: Vec<(u32, &str)> = Vec::new();
        let entries = DeduppedVec::using_eq(vec, |x, y| x.0.eq(&y.0));
        assert!(entries.inner.is_empty());
    }

    #[test]
    fn test_slow_dedup_by_key_keep_last_all_same_entries() {
        let vec = vec![(1, "one"), (1, "one duplicate"), (1, "one last")];
        let entries = DeduppedVec::using_eq(vec, |x, y| x.0.eq(&y.0));
        assert_eq!(entries.inner, vec![(1, "one last")]);
    }

    #[test]
    fn test_find_duplicate_no_duplicates() {
        let vec = vec![1, 2, 3];
        assert!(!has_duplicates(&vec));
    }

    #[test]
    fn test_find_duplicate_with_duplicates() {
        let vec = vec![1, 2, 2, 3];
        assert!(has_duplicates(&vec));
    }

    #[test]
    fn test_find_duplicate_empty_slice() {
        let vec: Vec<i32> = Vec::new();
        assert!(!has_duplicates(&vec));
    }

    #[test]
    fn test_find_duplicate_all_same_entries() {
        let vec = vec![1, 1, 1];
        assert!(has_duplicates(&vec));
    }
}
