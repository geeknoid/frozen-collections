use bitvec::prelude::*;
use num_integer::sqrt;

const ACCEPTABLE_COLLISION_RATE: f64 = 0.05; // What is a satisfactory rate of hash collisions?
const LARGE_INPUT_SIZE_THRESHOLD: usize = 1000; // What is the limit for an input to be considered "small"?
const MAX_SMALL_INPUT_MULTIPLIER: usize = 16; // How large a table should be allowed for small inputs?
const MAX_LARGE_INPUT_MULTIPLIER: usize = 3; // How large a table should be allowed for large inputs?

// Table of prime numbers to use as hash table sizes.
const PRIMES: [u32; 49] = [
    1103, 1327, 1597, 1931, 2333, 2801, 3371, 4049, 4861, 5839, 7013, 8419, 10103, 12143, 14591,
    17519, 21023, 25229, 30293, 36353, 43627, 52361, 62851, 75431, 90523, 108631, 130363, 156437,
    187751, 225307, 270371, 324449, 389357, 467237, 560689, 672827, 807403, 968897, 1162687,
    1395263, 1674319, 2009191, 2411033, 2893249, 3471899, 4166287, 4999559, 5999471, 7199369,
];

/// Given a slice of unique hash codes, figures out the best hash table size to use to minimize both table size snd collisions.
/// The returned value is an approximation; a good value but not necessarily the optimal value. The boolean return indicates
/// whether there are hash collisions at the given table size or not.
pub(crate) fn analyze_hash_codes(unique_hash_codes: &[u64]) -> (usize, bool) {
    // the minimum table size we can tolerate, given the acceptable collision rate
    let min_size = unique_hash_codes.len()
        - (((unique_hash_codes.len() as f64) * ACCEPTABLE_COLLISION_RATE) as usize);

    // the max table size we consider, given the growth factor for different input sizes
    let max_size;
    if unique_hash_codes.len() >= LARGE_INPUT_SIZE_THRESHOLD {
        max_size = unique_hash_codes.len() * MAX_LARGE_INPUT_MULTIPLIER;
    } else {
        max_size = unique_hash_codes.len() * MAX_SMALL_INPUT_MULTIPLIER;
    }

    let mut use_table: BitVec = BitVec::with_capacity(max_size - min_size + 1);
    let mut best_size = 0;
    let mut best_num_collisions = unique_hash_codes.len();

    if unique_hash_codes.len() < LARGE_INPUT_SIZE_THRESHOLD {
        // for small enough inputs, we try every size possible within our allowed range

        for size in min_size..(max_size + 1) {
            use_table.fill(false);
            let mut num_collisions = 0;

            for code in unique_hash_codes {
                let slot = (code % (size as u64)) as usize;
                if use_table[slot] {
                    num_collisions += 1;
                    if num_collisions >= best_num_collisions {
                        // no sense in continuing, we've seen a better fit before
                        break;
                    }
                } else {
                    use_table.set(slot, true);
                }
            }

            if num_collisions < best_num_collisions {
                best_num_collisions = num_collisions;
                best_size = size;

                if num_collisions == 0 {
                    // found a no-collision case, hurray
                    break;
                }
            }
        }
    } else if min_size < PRIMES[PRIMES.len() - 1] as usize {
        // For larger input sizes, we only consider a set of prime number table sizes rather than being exhaustive as in the
        // case for smaller input sizes. This is to constrain the total amount of compute time that gets spent in this code.

        let acceptable_collisions =
            ((unique_hash_codes.len() as f64) * ACCEPTABLE_COLLISION_RATE) as usize;
        for size in PRIMES {
            if size < min_size as u32 {
                continue;
            } else if size > max_size as u32 {
                break;
            }

            use_table.fill(false);
            let mut num_collisions = 0;

            for code in unique_hash_codes {
                let slot = (code % (size as u64)) as usize;
                if use_table[slot] {
                    num_collisions += 1;
                    if num_collisions >= best_num_collisions {
                        break;
                    }
                } else {
                    use_table.set(slot, true);
                }
            }

            if num_collisions < best_num_collisions {
                best_num_collisions = num_collisions;
                best_size = size as usize;

                if num_collisions <= acceptable_collisions {
                    // we have a winner!
                    break;
                }
            }
        }
    } else {
        // if the input size is bigger than what our prime table holds, then just return a table size that is the smallest prime number that is larger than 2x the input size
        best_size = unique_hash_codes.len() * 2 + 1;
        while !is_prime(best_size as u64) {
            best_size += 2;
        }

        for code in unique_hash_codes {
            let slot = (code % (best_size as u64)) as usize;
            if use_table[slot] {
                best_num_collisions = 1;
                break;
            } else {
                use_table.set(slot, true);
            }
        }
    }

    (best_size, best_num_collisions > 0)
}

fn is_prime(candidate: u64) -> bool {
    if candidate % 3 == 0 || candidate % 5 == 0 {
        return false;
    }

    let limit = sqrt(candidate) as u64;
    let mut divisor = 3;
    while divisor <= limit {
        if candidate % divisor == 0 {
            return false;
        }

        divisor += 2;
    }

    return true;
}
