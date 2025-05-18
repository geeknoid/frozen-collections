//! Duplicate removal utility functions for frozen collections.

use crate::traits::Hasher;
use alloc::vec::Vec;
use core::hash::Hash;
use hashbrown::HashSet as HashbrownSet;
use hashbrown::HashTable as HashbrownTable;

/// Remove duplicates from a vector, keeping the last occurrence of each duplicate.
///
/// This assumes the input vector is fairly short as time complexity is very high.
#[mutants::skip]
#[allow(clippy::module_name_repetitions)]
pub fn dedup_by_keep_last_slow<T, F>(unsorted_entries: &mut Vec<T>, mut cmp: F)
where
    F: FnMut(&mut T, &mut T) -> bool,
{
    if unsorted_entries.len() < 2 {
        return;
    }

    let mut dupes = HashbrownSet::new();
    for i in 0..unsorted_entries.len() {
        for j in (i + 1)..unsorted_entries.len() {
            let (s0, s1) = unsorted_entries.split_at_mut(j);
            if cmp(&mut s0[i], &mut s1[0]) {
                _ = dupes.insert(i);
                break;
            }
        }
    }

    if dupes.is_empty() {
        return;
    }

    let mut index = 0;
    unsorted_entries.retain(|_| {
        let result = !dupes.contains(&index);
        index += 1;
        result
    });
}

/// Remove duplicates from a vector, keeping the last occurrence of each duplicate.
///
/// This assumes the input vector is sorted.
#[allow(clippy::module_name_repetitions)]
pub fn dedup_by_keep_last<T, F>(sorted_entries: &mut Vec<T>, mut cmp: F)
where
    F: FnMut(&mut T, &mut T) -> bool,
{
    if sorted_entries.len() < 2 {
        return;
    }

    let mut dupes = HashbrownSet::new();
    for i in 0..sorted_entries.len() - 1 {
        let (s0, s1) = sorted_entries.split_at_mut(i + 1);
        if cmp(&mut s0[i], &mut s1[0]) {
            _ = dupes.insert(i);
        }
    }

    if dupes.is_empty() {
        return;
    }

    let mut index = 0;
    sorted_entries.retain(|_| {
        let result = !dupes.contains(&index);
        index += 1;
        result
    });
}

/// Remove duplicates from a vector, keeping the last occurrence of each duplicate.
#[allow(clippy::module_name_repetitions)]
#[mutants::skip]
pub fn dedup_by_hash_keep_last<T, F, G>(unsorted_entries: &mut Vec<T>, hasher: F, mut eq: G)
where
    F: Fn(&T) -> u64,
    G: FnMut(&T, &T) -> bool,
{
    if unsorted_entries.len() < 2 {
        return;
    }

    let mut dupes = Vec::new();
    let mut keep = HashbrownTable::with_capacity(unsorted_entries.len());
    for (index, value) in unsorted_entries.iter().enumerate() {
        let hash = hasher(value);

        let r = keep.find_entry(hash, |other| eq(value, &unsorted_entries[*other]));
        if let Ok(entry) = r {
            dupes.push(*entry.get());
            _ = entry.remove();
        }

        _ = keep.insert_unique(hash, index, |x| hasher(&unsorted_entries[*x]));
    }

    if dupes.is_empty() {
        // no duplicates found, we're done
        return;
    }

    // remove the duplicates from the input vector

    let mut index = 0;
    unsorted_entries.retain(|_| {
        let result = !dupes.contains(&index);
        index += 1;
        result
    });
}

#[derive(PartialEq, Eq)]
struct Wrapper<T> {
    value: T,
    hash_code: u64,
}

impl<T> Hash for Wrapper<T> {
    fn hash<H: core::hash::Hasher>(&self, state: &mut H) {
        state.write_u64(self.hash_code);
    }
}

/// Look for the first duplicate value, if any.
pub fn has_duplicates_with_hasher<H, T>(values: &[&T], hasher: &H) -> bool
where
    H: Hasher<T>,
    T: ?Sized + Eq,
{
    let mut s = HashbrownSet::new();
    for value in values {
        let hash_code = hasher.hash(value);
        if !s.insert(Wrapper { value, hash_code }) {
            return true;
        }
    }

    false
}

/// Look for the first duplicate value if any (assumes `values` is a relatively small array).
pub fn has_duplicates_slow<T>(values: &[T]) -> bool
where
    T: Eq,
{
    for i in 0..values.len() {
        for j in 0..i {
            if values[j].eq(&values[i]) {
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
        let mut vec = vec![(1, "one"), (2, "two"), (3, "three")];
        dedup_by_keep_last_slow(&mut vec, |x, y| x.0.eq(&y.0));
        assert_eq!(vec, vec![(1, "one"), (2, "two"), (3, "three")]);
    }

    #[test]
    fn test_slow_dedup_by_keep_last_with_duplicates() {
        let mut vec = vec![
            (1, "one"),
            (2, "two"),
            (2, "two duplicate"),
            (3, "three"),
            (3, "three duplicate"),
            (3, "three last"),
        ];
        dedup_by_keep_last_slow(&mut vec, |x, y| x.0.eq(&y.0));
        assert_eq!(vec, vec![(1, "one"), (2, "two duplicate"), (3, "three last")]);
    }

    #[test]
    fn test_slow_dedup_by_keep_last_empty_vector() {
        let mut vec: Vec<(u8, &str)> = Vec::new();
        dedup_by_keep_last_slow(&mut vec, |x, y| x.0.eq(&y.0));
        assert!(vec.is_empty());
    }

    #[test]
    fn test_slow_dedup_by_key_keep_last_all_same_entries() {
        let mut vec = vec![(1, "one"), (1, "one duplicate"), (1, "one last")];
        dedup_by_keep_last_slow(&mut vec, |x, y| x.0.eq(&y.0));
        assert_eq!(vec, vec![(1, "one last")]);
    }

    #[test]
    fn test_find_duplicate_slow_no_duplicates() {
        let vec = vec![1, 2, 3];
        assert!(!has_duplicates_slow(&vec));
    }

    #[test]
    fn test_find_duplicate_slow_with_duplicates() {
        let vec = vec![1, 2, 2, 3];
        assert!(has_duplicates_slow(&vec));
    }

    #[test]
    fn test_find_duplicate_slow_empty_slice() {
        let vec: Vec<i32> = Vec::new();
        assert!(!has_duplicates_slow(&vec));
    }

    #[test]
    fn test_find_duplicate_slow_all_same_entries() {
        let vec = vec![1, 1, 1];
        assert!(has_duplicates_slow(&vec));
    }
}
