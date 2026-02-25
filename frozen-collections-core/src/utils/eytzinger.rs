//! Implements Eytzinger search over slices.
//! Refer to this research paper for more information: [Eytzinger layout](https://arxiv.org/pdf/1509.05053.pdf)
//!
//! We don't use a branchless implementation, and we don't use a prefetching
//! implementation since they actually slow things down in our benchmarks. They might
//! show value with very large lookup tables, but that's not our sweat spot.

use core::cmp::Ordering;

/// Sorts the slice in-place using the Eytzinger layout.
pub fn eytzinger_layout<T>(sorted_entries: &mut [T]) {
    const fn get_eytzinger_index(original_index: usize, slice_len: usize) -> usize {
        let ipk = (original_index + 2).next_power_of_two().trailing_zeros() as usize;
        let li = original_index + 1 - (1 << (ipk - 1));
        let zk = li * 2 + 1;
        let last_power_of_two = (slice_len + 2).next_power_of_two() / 2;
        let y = (last_power_of_two >> (ipk - 1)) * zk;
        let kp = y >> 1;
        let x = kp + last_power_of_two; // (1+k) * last_power_of_two
        let x = x.saturating_sub(slice_len + 1);
        y - x - 1
    }

    let mut redirects = alloc::vec![usize::MAX; sorted_entries.len()];
    for mut i in 0..sorted_entries.len() {
        let mut target = get_eytzinger_index(i, sorted_entries.len());
        if redirects[target] != usize::MAX {
            let t = redirects[target];
            redirects[target] = usize::MAX;
            target = t;
        }

        sorted_entries.swap(i, target);

        if redirects[i] != usize::MAX {
            let t = redirects[i];
            redirects[i] = usize::MAX;
            i = t;
        }

        if target != i {
            redirects[target] = i;
            redirects[i] = target;
        }
    }
}

/// Searches for a given key using branching Eytzinger search.
///
/// The slice must have been previously sorted with the [`eytzinger_layout`] method.
#[inline]
pub fn eytzinger_search_by<'a, T: 'a>(data: &'a [T], f: impl Fn(&'a T) -> Ordering) -> Option<usize> {
    let mut i = 0;
    loop {
        match data.get(i) {
            Some(v) => {
                let order = f(v);
                if order == Ordering::Equal {
                    return Some(i);
                }

                // Leverage the fact Ordering is repr(i8) with values -1/0/1.
                // For Less (-1): as usize wraps to usize::MAX, (usize::MAX >> 1) & 1 = 1, so i = 2*i+2 (right child).
                // For Greater (1): as usize is 1, (1 >> 1) & 1 = 0, so i = 2*i+1 (left child).
                let o = order as usize;
                let o = (o >> 1) & 1;
                i = 2 * i + 1 + o;
            }
            None => return None,
        }
    }
}

// /// Searches for a given key using branchless Eytzinger search.
// ///
// /// The slice must have been previously sorted with the [`eytzinger_layout`] method.
// /// This version never branches on the comparison result during traversal, instead
// /// walking all the way to a leaf and then backtracking to the candidate node.
// #[inline]
// pub fn eytzinger_search_by_branchless<'a, T: 'a>(
//     data: &'a [T],
//     f: impl Fn(&'a T) -> Ordering,
// ) -> Option<usize> {
//     let len = data.len();
//     if len == 0 {
//         return None;
//     }
//
//     let mut i = 0;
//     while i < len {
//         // Safe: i < len is guaranteed by the while condition
//         let v = unsafe { data.get_unchecked(i) };
//         i = 2 * i + 1 + usize::from(f(v) == Ordering::Less);
//     }
//
//     eytzinger_backtrack(data, i, &f)
// }
//
// /// Number of tree levels to prefetch ahead during branchless search with prefetching.
// const PREFETCH_LEVELS: u32 = 3;
//
// /// Searches for a given key using branchless Eytzinger search with prefetching.
// ///
// /// The slice must have been previously sorted with the [`eytzinger_layout`] method.
// /// This version prefetches cache lines several levels ahead in the tree to reduce
// /// memory latency on large collections.
// #[inline]
// pub fn eytzinger_search_by_branchless_prefetch<'a, T: 'a>(
//     data: &'a [T],
//     f: impl Fn(&'a T) -> Ordering,
// ) -> Option<usize> {
//     let len = data.len();
//     if len == 0 {
//         return None;
//     }
//
//     let mut i = 0;
//     while i < len {
//         // Prefetch PREFETCH_LEVELS deeper in the tree. In 0-indexed Eytzinger,
//         // the descendant at depth K from node i starts around (2^K * (i+1)) - 1.
//         let prefetch_idx = ((i + 1) << PREFETCH_LEVELS) - 1;
//         if prefetch_idx < len {
//             // Safe: prefetch_idx < len is verified by the check above
//             let ptr = unsafe { data.as_ptr().add(prefetch_idx) };
//             prefetch_read_data(ptr);
//         }
//
//         // Safe: i < len is guaranteed by the while condition
//         let v = unsafe { data.get_unchecked(i) };
//         i = 2 * i + 1 + usize::from(f(v) == Ordering::Less);
//     }
//
//     eytzinger_backtrack(data, i, &f)
// }
//
// /// Backtracks from the leaf position to find the candidate node and checks for equality.
// ///
// /// After a branchless traversal, `i` is past the end of the array. This function
// /// uses bit manipulation to find the last node where we "went right", which is the
// /// lower-bound candidate.
// #[inline]
// fn eytzinger_backtrack<'a, T: 'a>(
//     data: &'a [T],
//     i: usize,
//     f: &impl Fn(&'a T) -> Ordering,
// ) -> Option<usize> {
//     // Convert to 1-indexed and strip trailing "went left" bits from the path
//     let p = i + 1;
//     let j = p >> (1 + (!p).trailing_zeros());
//
//     if j > 0 {
//         let candidate = j - 1;
//         // Safe: candidate < data.len() because it corresponds to a node visited during traversal
//         let val = unsafe { data.get_unchecked(candidate) };
//         if f(val) == Ordering::Equal {
//             return Some(candidate);
//         }
//     }
//
//     None
// }
//
// /// Hints the CPU to prefetch the cache line containing the given address for reading.
// #[inline]
// fn prefetch_read_data<T>(ptr: *const T) {
//     #[cfg(target_arch = "x86_64")]
//     {
//         // Safe: prefetch is a performance hint that does not affect correctness;
//         // the caller has verified the address is within slice bounds
//         unsafe {
//             core::arch::x86_64::_mm_prefetch::<{ core::arch::x86_64::_MM_HINT_T0 }>(
//                 ptr.cast::<i8>(),
//             );
//         }
//     }
//
//     #[cfg(target_arch = "x86")]
//     {
//         // Safe: same as above
//         unsafe {
//             core::arch::x86::_mm_prefetch::<{ core::arch::x86::_MM_HINT_T0 }>(ptr.cast::<i8>());
//         }
//     }
//
//     #[cfg(target_arch = "aarch64")]
//     {
//         // Safe: same as above
//         unsafe {
//             core::arch::aarch64::_prefetch::<
//                 { core::arch::aarch64::_PREFETCH_READ },
//                 { core::arch::aarch64::_PREFETCH_LOCALITY3 },
//             >(ptr.cast::<i8>());
//         }
//     }
//
//     #[cfg(not(any(target_arch = "x86_64", target_arch = "x86", target_arch = "aarch64")))]
//     {
//         let _ = ptr;
//     }
// }

#[cfg(test)]
mod tests {
    use super::*;

    #[expect(clippy::cast_possible_truncation, clippy::cast_possible_wrap, reason = "test values are tiny")]
    fn make_eytzinger(n: usize) -> Vec<i32> {
        let mut data: Vec<i32> = (0..n as i32).collect();
        eytzinger_layout(&mut data);
        data
    }

    fn cmp_fn(key: i32) -> impl Fn(&i32) -> Ordering {
        move |entry| entry.cmp(&key)
    }

    #[test]
    #[expect(clippy::cast_possible_truncation, clippy::cast_possible_wrap, reason = "test values are tiny")]
    fn branching_search_hits() {
        for n in 1..=128 {
            let data = make_eytzinger(n);
            for key in 0..n as i32 {
                let branching = eytzinger_search_by(&data, cmp_fn(key));

                assert!(branching.is_some(), "branching miss for key={key}, n={n}");
                assert_eq!(data[branching.unwrap()], key);
            }
        }
    }

    #[test]
    #[expect(clippy::cast_possible_truncation, clippy::cast_possible_wrap, reason = "test values are tiny")]
    fn branching_search_misses() {
        for n in 0..=64 {
            let data = make_eytzinger(n);
            for &key in &[-1, n as i32, n as i32 + 100] {
                let branching = eytzinger_search_by(&data, cmp_fn(key));

                assert!(branching.is_none(), "branching false hit for key={key}, n={n}");
            }
        }
    }

    #[test]
    fn empty_slice() {
        let data: Vec<i32> = vec![];
        assert!(eytzinger_search_by(&data, cmp_fn(0)).is_none());
    }

    #[test]
    fn single_element() {
        let data = make_eytzinger(1);
        assert!(eytzinger_search_by(&data, cmp_fn(0)).is_some());
        assert!(eytzinger_search_by(&data, cmp_fn(1)).is_none());
    }

    #[test]
    #[expect(clippy::cast_possible_truncation, clippy::cast_possible_wrap, reason = "test values are tiny")]
    fn layout_matches_expected() {
        // For a sorted input [0,1,2,3,4,5,6], the Eytzinger layout should be [3,1,5,0,2,4,6]
        let data = make_eytzinger(7);
        assert_eq!(data, vec![3, 1, 5, 0, 2, 4, 6]);

        let data = make_eytzinger(3);
        assert_eq!(data, vec![1, 0, 2]);

        let data = make_eytzinger(5);
        assert_eq!(data, vec![3, 1, 4, 0, 2]);

        // Verify the layout is a valid Eytzinger tree for all sizes up to 128:
        // every element from the sorted input must appear exactly once
        for n in 1..=128 {
            let data = make_eytzinger(n);
            let mut sorted = data.clone();
            sorted.sort_unstable();
            let expected: Vec<i32> = (0..n as i32).collect();
            assert_eq!(sorted, expected, "layout is not a permutation for n={n}");
        }
    }
}
