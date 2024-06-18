//! Implements Eytzinger search over slices.
//! Refer to this research paper for more information: [Eytzinger layout](https://arxiv.org/pdf/1509.05053.pdf)
//!
//! This code is adapted and heavily modified from <https://github.com/main--/rust-eytzinger/blob/master/src/lib.rs>

use core::borrow::Borrow;
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

/*
/// Searches for a given key in the slice.
///
/// The slice must have been previously sorted with the `eytzinger` method.
fn eytzinger_search_by<'a, T: 'a, F>(data: &'a [T], mut f: F) -> Option<usize>
where
    F: FnMut(&'a T) -> Ordering,
{
    let mut i = 0;
    loop {
        match data.get(i) {
            Some(v) => {
                match f(v) {
                    Ordering::Equal => return Some(i),
                    o => {
                        // I was hoping the optimizer could handle this but it can't
                        // So here goes the evil hack: Ordering is -1/0/1
                        // So we use this dirty trick to map this to +2/X/+1
                        let o = o as usize;
                        let o = (o >> 1) & 1;
                        i = 2 * i + 1 + o;
                    }
                };
            }
            None => return None,
        }
    }
}
*/

/// Searches for a given key in the slice.
///
/// The slice must have been previously sorted with the `eytzinger` method.
fn eytzinger_search_by<'a, T: 'a, F>(data: &'a [T], mut f: F) -> Option<usize>
where
    F: FnMut(&'a T) -> Ordering,
{
    let mut i = 0;
    while i < data.len() {
        let v = &data[i]; // this range check is optimized out :D
        i = match f(v) {
            Ordering::Greater | Ordering::Equal => 2 * i + 1,
            Ordering::Less => 2 * i + 2,
        };
    }

    // magic from the paper to fix up the (incomplete) final tree layer
    // (only difference is that we recheck f() because this is exact search)
    let p = i + 1;
    let j = p >> (1 + (!p).trailing_zeros());
    if j != 0 && (f(&data[j - 1]) == Ordering::Equal) {
        Some(j - 1)
    } else {
        None
    }
}

/// Searches for an element in a slice sorted in the Eytzinger layout.
#[allow(clippy::module_name_repetitions)]
pub fn eytzinger_search_by_key<'a, T, K, F, Q>(data: &'a [T], key: &Q, mut f: F) -> Option<usize>
where
    T: 'a,
    K: Borrow<Q>,
    F: FnMut(&'a T) -> K,
    Q: ?Sized + Ord,
{
    eytzinger_search_by(data, |k| f(k).borrow().cmp(key))
}
