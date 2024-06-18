//! Misc utility functions for frozen collections.

use core::cmp::Ordering;
use core::hash::Hash;
use hashbrown::HashSet as HashbrownSet;

use const_random::const_random;

/// Remove duplicates from a vector, keeping the last occurrence of each duplicate.
///
/// This assumes the input vector is fairly short as time complexity is very high.
///
/// # Compatibility Note
///
/// This function is an implementation detail of the `frozen-collections` crate.
/// This API is therefore not stable and may change at any time.
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
/// This assumes the input vector is fairly short as time complexity is very high.
///
/// # Compatibility Note
///
/// This function is an implementation detail of the `frozen-collections` crate.
/// This API is therefore not stable and may change at any time.
pub fn slow_dedup_keep_last<T: Eq>(unsorted_entries: &mut Vec<T>) {
    slow_dedup_by_keep_last(unsorted_entries, |x, y| x.eq(&y));
}

/// Remove duplicates from a vector, keeping the last occurrence of each duplicate.
///
/// This assumes the input vector is sorted.
///
/// # Compatibility Note
///
/// This function is an implementation detail of the `frozen-collections` crate.
/// This API is therefore not stable and may change at any time.
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

/// Remove duplicates from a vector, keeping the last occurrence of each duplicate.
///
/// This assumes the input vector is sorted.
///
/// # Compatibility Note
///
/// This function is an implementation detail of the `frozen-collections` crate.
/// This API is therefore not stable and may change at any time.
pub fn dedup_by_key_keep_last<T, K, F>(sorted_entries: &mut Vec<T>, key: F)
where
    F: Fn(&T) -> &K,
    K: Eq,
{
    if sorted_entries.len() < 2 {
        return;
    }

    let mut dupes = HashbrownSet::new();
    for i in 0..sorted_entries.len() - 1 {
        if key(&sorted_entries[i]) == key(&sorted_entries[i + 1]) {
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

/// Remove duplicates from a vector, keeping the last occurrence of each duplicate.
///
/// This assumes the input vector is sorted.
///
/// # Compatibility Note
///
/// This function is an implementation detail of the `frozen-collections` crate.
/// This API is therefore not stable and may change at any time.
pub fn dedup_keep_last<T: Eq>(sorted_entries: &mut Vec<T>) {
    dedup_by_keep_last(sorted_entries, |x, y| x.eq(&y));
}

/// Look for the first duplicate value if any.
///
/// # Compatibility Note
///
/// This function is an implementation detail of the `frozen-collections` crate.
/// This API is therefore not stable and may change at any time.
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
///
/// # Compatibility Note
///
/// This function is an implementation detail of the `frozen-collections` crate.
/// This API is therefore not stable and may change at any time.
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

/// Perform a binary search on a slice of values.
///
/// # Returns
///
/// An `Option<usize>` containing the index of the target value if found, or `None` if not found.
#[inline]
pub fn binary_search_by_key<T, K, F>(values: &[T], target: &K, key: F) -> Option<usize>
where
    K: Ord + ?Sized,
    F: Fn(&T) -> &K,
{
    let mut low = 0;
    let mut high = values.len();

    while low < high {
        let mid = low + (high - low) / 2;
        let v = &unsafe { values.get_unchecked(mid) };

        match key(v).cmp(target) {
            Ordering::Less => low = mid + 1,
            Ordering::Greater => high = mid,
            Ordering::Equal => return Some(mid),
        }
    }

    None
}

/// Perform a binary search on a slice of values.
///
/// # Returns
///
/// An `Option<usize>` containing the index of the target value if found, or `None` if not found.
#[inline]
pub fn binary_search<T: Ord>(values: &[T], target: &T) -> Option<usize> {
    binary_search_by_key(values, target, |entry| entry)
}

/// Pick four random seeds at compile time.
#[must_use]
pub const fn pick_compile_time_random_seeds() -> (u64, u64, u64, u64) {
    (
        const_random!(u64),
        const_random!(u64),
        const_random!(u64),
        const_random!(u64),
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_dedup_by_key_keep_last_no_duplicates() {
        let mut vec = vec![(1, "one"), (2, "two"), (3, "three")];
        dedup_by_key_keep_last(&mut vec, |entry| &entry.0);
        assert_eq!(vec, vec![(1, "one"), (2, "two"), (3, "three")]);
    }

    #[test]
    fn test_dedup_by_key_keep_last_with_duplicates() {
        let mut vec = vec![
            (1, "one"),
            (2, "two"),
            (2, "two duplicate"),
            (3, "three"),
            (3, "three duplicate"),
            (3, "three last"),
        ];
        dedup_by_key_keep_last(&mut vec, |entry| &entry.0);
        assert_eq!(
            vec,
            vec![(1, "one"), (2, "two duplicate"), (3, "three last")]
        );
    }

    #[test]
    fn test_dedup_by_key_keep_last_empty_vector() {
        let mut vec: Vec<(u8, &str)> = Vec::new();
        dedup_by_key_keep_last(&mut vec, |entry| &entry.0);
        assert!(vec.is_empty());
    }

    #[test]
    fn test_dedup_by_key_keep_last_all_same_elements() {
        let mut vec = vec![(1, "one"), (1, "one duplicate"), (1, "one last")];
        dedup_by_key_keep_last(&mut vec, |entry| &entry.0);
        assert_eq!(vec, vec![(1, "one last")]);
    }

    #[test]
    fn test_dedup_keep_last_no_duplicates() {
        let mut vec = vec![1, 2, 3];
        dedup_keep_last(&mut vec);
        assert_eq!(vec, vec![1, 2, 3]);
    }

    #[test]
    fn test_dedup_keep_last_with_duplicates() {
        let mut vec = vec![1, 2, 2, 3, 3, 3];
        dedup_keep_last(&mut vec);
        assert_eq!(vec, vec![1, 2, 3]);
    }

    #[test]
    fn test_dedup_keep_last_empty_vector() {
        let mut vec: Vec<i32> = Vec::new();
        dedup_keep_last(&mut vec);
        assert!(vec.is_empty());
    }

    #[test]
    fn test_dedup_keep_last_all_same_elements() {
        let mut vec = vec![1, 1, 1];
        dedup_keep_last(&mut vec);
        assert_eq!(vec, vec![1]);
    }

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
    fn test_slow_dedup_by_key_keep_last_all_same_elements() {
        let mut vec = vec![(1, "one"), (1, "one duplicate"), (1, "one last")];
        slow_dedup_by_keep_last(&mut vec, |x, y| x.0.eq(&y.0));
        assert_eq!(vec, vec![(1, "one last")]);
    }

    #[test]
    fn test_slow_dedup_keep_last_no_duplicates() {
        let mut vec = vec![1, 2, 3];
        slow_dedup_keep_last(&mut vec);
        assert_eq!(vec, vec![1, 2, 3]);
    }

    #[test]
    fn test_slow_dedup_keep_last_with_duplicates() {
        let mut vec = vec![1, 2, 2, 3, 3, 3];
        slow_dedup_keep_last(&mut vec);
        assert_eq!(vec, vec![1, 2, 3]);
    }

    #[test]
    fn test_slow_dedup_keep_last_empty_vector() {
        let mut vec: Vec<i32> = Vec::new();
        slow_dedup_keep_last(&mut vec);
        assert!(vec.is_empty());
    }

    #[test]
    fn test_slow_dedup_keep_last_all_same_elements() {
        let mut vec = vec![1, 1, 1];
        slow_dedup_keep_last(&mut vec);
        assert_eq!(vec, vec![1]);
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
    fn test_find_duplicate_all_same_elements() {
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
    fn test_find_duplicate_slow_all_same_elements() {
        let vec = vec![1, 1, 1];
        assert_eq!(slow_find_duplicate(&vec), Some(1));
    }

    #[test]
    fn test_binary_search_empty_vector() {
        let vec: Vec<i32> = Vec::new();
        assert_eq!(binary_search(&vec, &1), None);
    }

    #[test]
    fn test_binary_search_single_element_present() {
        let vec = vec![1];
        assert_eq!(binary_search(&vec, &1), Some(0));
    }

    #[test]
    fn test_binary_search_single_element_not_present() {
        let vec = vec![1];
        assert_eq!(binary_search(&vec, &2), None);
    }

    #[test]
    fn test_binary_search_multiple_elements_present() {
        let vec = vec![1, 2, 3, 4, 5];
        assert_eq!(binary_search(&vec, &3), Some(2));
    }

    #[test]
    fn test_binary_search_multiple_elements_not_present() {
        let vec = vec![1, 2, 3, 4, 5];
        assert_eq!(binary_search(&vec, &6), None);
    }

    #[test]
    fn test_binary_search_with_duplicates() {
        let vec = vec![1, 2, 2, 2, 3, 4, 5];
        assert!(matches!(binary_search(&vec, &2), Some(1..=3)));
    }
}
