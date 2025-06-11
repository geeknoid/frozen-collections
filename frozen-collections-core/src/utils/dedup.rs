//! Duplicate removal utility functions for frozen collections.

use hashbrown::HashSet as HashbrownSet;
use hashbrown::HashTable as HashbrownTable;

#[cfg(not(feature = "std"))]
use alloc::vec::Vec;

/// Remove duplicates from a vector, keeping the last occurrence of each duplicate.
///
/// This assumes the input vector is fairly short as time complexity is very high.
#[mutants::skip]
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
