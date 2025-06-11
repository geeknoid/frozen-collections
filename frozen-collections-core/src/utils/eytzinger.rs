//! Implements Eytzinger search over slices.
//! Refer to this research paper for more information: [Eytzinger layout](https://arxiv.org/pdf/1509.05053.pdf)
//!
//! This code is adapted and heavily modified from <https://github.com/main--/rust-eytzinger/blob/master/src/lib.rs>

use core::cmp::Ordering;

/// Sorts the slice in-place using the Eytzinger layout.
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

/*
//const NUM_PREFETCH_LEVELS: usize = 3;

#[inline]
#[expect(clippy::module_name_repetitions, "This is fine")]
pub fn eytzinger_search_by<'a, T: 'a, F>(data: &'a [T], mut f: F) -> Option<usize>
where
    F: FnMut(&'a T) -> Ordering,
{
/*
    let mut i = 1;
    while (1 << NUM_PREFETCH_LEVELS) * i < data.len() {
        i = 2 * i + (f(&data[i]) == Ordering::Greater) as usize;
        prefetch_for_read(data, (1 << NUM_PREFETCH_LEVELS) * i);
    }
*/
    let mut i = 0;
    while i < data.len() {
        let v = &data[i];
        i = match f(v) {
            Ordering::Greater | Ordering::Equal => 2 * i + 1,
            Ordering::Less => 2 * i + 2,
        };
    }

    let p = i + 1;
    let j = p >> (1 + (!p).trailing_zeros());
    let val = unsafe { data.get_unchecked(j - 1) };
    if j != 0 && (f(val) == Ordering::Equal) {
        Some(j - 1)
    } else {
        None
    }
}
*/
