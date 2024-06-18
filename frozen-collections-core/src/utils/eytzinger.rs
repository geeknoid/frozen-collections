//! Implements Eytzinger search over slices.
//! Refer to this research paper for more information: [Eytzinger layout](https://arxiv.org/pdf/1509.05053.pdf)
//!
//! This code is adapted and heavily modified from <https://github.com/main--/rust-eytzinger/blob/master/src/lib.rs>

use core::cmp::Ordering;

/// Sorts the slice in-place using the Eytzinger layout.
#[allow(clippy::module_name_repetitions)]
pub fn eytzinger_sort<T>(data: &mut [T]) {
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

    let mut map = hashbrown::HashMap::new();
    for mut i in 0..data.len() {
        let mut target = get_eytzinger_index(i, data.len());
        if target < i {
            target = map.remove(&target).unwrap();
        }

        data.swap(i, target);

        if let Some(x) = map.remove(&i) {
            i = x;
        }

        if target != i {
            _ = map.insert(target, i);
            _ = map.insert(i, target);
        }
    }
}

/// Searches for a given key in the slice.
///
/// The slice must have been previously sorted with the `eytzinger` method.
#[inline]
#[allow(clippy::module_name_repetitions)]
pub fn eytzinger_search_by<'a, T: 'a, F>(data: &'a [T], mut f: F) -> Option<usize>
where
    F: FnMut(&'a T) -> Ordering,
{
    let mut i = 0;
    loop {
        match data.get(i) {
            Some(v) => {
                let order = f(v);
                if order == Ordering::Equal {
                    return Some(i);
                }

                // Leverage the fact Ordering is defined as -1/0/1
                let o = order as usize;
                let o = (o >> 1) & 1;
                i = 2 * i + 1 + o;
            }
            None => return None,
        }
    }
}
