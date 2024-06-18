//! Duplicate removal utility functions for frozen collections.

use crate::traits::Hasher;
use core::hash::Hash;
use hashbrown::HashSet as HashbrownSet;

/// Remove duplicates from a vector, keeping the last occurrence of each duplicate.
///
/// This assumes the input vector is fairly short as time complexity is very high.
pub fn slow_dedup_by_keep_last<T, F>(unsorted_entries: &mut Vec<T>, mut cmp: F)
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
                dupes.insert(i);
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
            dupes.insert(i);
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

struct Entry<'a, K> {
    pub hash: u64,
    pub index: usize,
    pub key: &'a K,
}

impl<'a, K> Hash for Entry<'a, K>
where
    K: Eq,
{
    fn hash<H: core::hash::Hasher>(&self, state: &mut H) {
        state.write_u64(self.hash);
    }
}

impl<'a, K: Eq> PartialEq<Self> for Entry<'a, K> {
    fn eq(&self, other: &Self) -> bool {
        self.key.eq(other.key)
    }
}

impl<'a, K: Eq> Eq for Entry<'a, K> {}

/// Remove duplicates from a vector, keeping the last occurrence of each duplicate.
#[allow(clippy::module_name_repetitions)]
pub fn dedup_by_hash_keep_last<K, V, H>(unsorted_entries: &mut Vec<(K, V)>, hasher: &H)
where
    K: Eq,
    H: Hasher<K>,
{
    if unsorted_entries.len() < 2 {
        return;
    }

    let mut dupes = Vec::new();
    {
        let mut keep = HashbrownSet::with_capacity(unsorted_entries.len());
        for (i, pair) in unsorted_entries.iter().enumerate() {
            let hash = hasher.hash(&pair.0);

            let entry = Entry {
                hash,
                index: i,
                key: &pair.0,
            };

            let old = keep.replace(entry);
            if let Some(old) = old {
                dupes.push(old.index);
            }
        }
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

/// Look for the first duplicate value if any.
pub fn find_duplicate<'a, T, I>(values: I) -> Option<usize>
where
    T: Hash + Eq + 'a,
    I: Iterator<Item = &'a T>,
{
    let mut s = HashbrownSet::new();

    for (i, v) in values.enumerate() {
        if !s.insert(v) {
            return Some(i);
        }
    }

    None
}

/// Look for the first duplicate value if any (assumes `values` is a relatively small array).
pub fn slow_find_duplicate<T: Eq>(values: &[T]) -> Option<usize> {
    for i in 0..values.len() {
        for j in 0..i {
            if values[j].eq(&values[i]) {
                return Some(i);
            }
        }
    }

    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_slow_dedup_by_keep_last_no_duplicates() {
        let mut vec = vec![(1, "one"), (2, "two"), (3, "three")];
        slow_dedup_by_keep_last(&mut vec, |x, y| x.0.eq(&y.0));
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
        slow_dedup_by_keep_last(&mut vec, |x, y| x.0.eq(&y.0));
        assert_eq!(
            vec,
            vec![(1, "one"), (2, "two duplicate"), (3, "three last")]
        );
    }

    #[test]
    fn test_slow_dedup_by_keep_last_empty_vector() {
        let mut vec: Vec<(u8, &str)> = Vec::new();
        slow_dedup_by_keep_last(&mut vec, |x, y| x.0.eq(&y.0));
        assert!(vec.is_empty());
    }

    #[test]
    fn test_slow_dedup_by_key_keep_last_all_same_entries() {
        let mut vec = vec![(1, "one"), (1, "one duplicate"), (1, "one last")];
        slow_dedup_by_keep_last(&mut vec, |x, y| x.0.eq(&y.0));
        assert_eq!(vec, vec![(1, "one last")]);
    }

    #[test]
    fn test_find_duplicate_no_duplicates() {
        let vec = [1, 2, 3];
        assert_eq!(find_duplicate(vec.iter()), None);
    }

    #[test]
    fn test_find_duplicate_with_duplicates() {
        let vec = [1, 2, 2, 3];
        assert_eq!(find_duplicate(vec.iter()), Some(2));
    }

    #[test]
    fn test_find_duplicate_empty_slice() {
        let vec: Vec<i32> = Vec::new();
        assert_eq!(find_duplicate(vec.iter()), None);
    }

    #[test]
    fn test_find_duplicate_all_same_entries() {
        let vec = [1, 1, 1];
        assert_eq!(find_duplicate(vec.iter()), Some(1));
    }

    #[test]
    fn test_find_duplicate_slow_no_duplicates() {
        let vec = vec![1, 2, 3];
        assert_eq!(slow_find_duplicate(&vec), None);
    }

    #[test]
    fn test_find_duplicate_slow_with_duplicates() {
        let vec = vec![1, 2, 2, 3];
        assert_eq!(slow_find_duplicate(&vec), Some(2));
    }

    #[test]
    fn test_find_duplicate_slow_empty_slice() {
        let vec: Vec<i32> = Vec::new();
        assert_eq!(slow_find_duplicate(&vec), None);
    }

    #[test]
    fn test_find_duplicate_slow_all_same_entries() {
        let vec = vec![1, 1, 1];
        assert_eq!(slow_find_duplicate(&vec), Some(1));
    }
}
