use crate::utils::BitVec;
use alloc::vec::Vec;

/// How to treat a collection of hash codes for best performance.
//#[derive(Clone)]
pub struct HashCodeAnalysisResult {
    /// The recommended hash table size. This is not necessarily optimal, but it's good enough.
    pub num_hash_slots: usize,

    /// The number of collisions when using the recommended table size.
    #[allow(dead_code)]
    pub num_hash_collisions: usize,
}

/// Look for an "optimal" hash table size for a given set of hash codes.
#[allow(clippy::cast_possible_truncation)]
#[mutants::skip]
pub fn analyze_hash_codes<I>(hash_codes: I) -> HashCodeAnalysisResult
where
    I: Iterator<Item = u64>,
{
    // What is a satisfactory rate of hash collisions?
    const ACCEPTABLE_COLLISION_PERCENTAGE: usize = 5;

    // By how much do we shrink the acceptable # collisions per iteration?
    const ACCEPTABLE_COLLISION_PERCENTAGE_OF_REDUCTION: usize = 20;

    // thresholds to categorize input sizes
    const MEDIUM_INPUT_SIZE_THRESHOLD: usize = 128;
    const LARGE_INPUT_SIZE_THRESHOLD: usize = 1000;

    // amount by which the table can be larger than the input
    const MAX_SMALL_INPUT_MULTIPLIER: usize = 10;
    const MAX_MEDIUM_INPUT_MULTIPLIER: usize = 7;
    const MAX_LARGE_INPUT_MULTIPLIER: usize = 3;

    let hash_codes: Vec<u64> = hash_codes.collect();
    let mut acceptable_collisions = if hash_codes.len() < MEDIUM_INPUT_SIZE_THRESHOLD {
        // for small enough inputs, we try for perfection
        0
    } else {
        (hash_codes.len() / 100) * ACCEPTABLE_COLLISION_PERCENTAGE
    };

    // the minimum table size we can tolerate, given the acceptable collision rate
    let mut min_size = hash_codes.len() - acceptable_collisions;
    if !min_size.is_power_of_two() {
        min_size = min_size.next_power_of_two();
    }

    // the maximum table size we consider, given a scaled growth factor for different input sizes
    let mut max_size = if hash_codes.len() < MEDIUM_INPUT_SIZE_THRESHOLD {
        hash_codes.len() * MAX_SMALL_INPUT_MULTIPLIER
    } else if hash_codes.len() < LARGE_INPUT_SIZE_THRESHOLD {
        hash_codes.len() * MAX_MEDIUM_INPUT_MULTIPLIER
    } else {
        hash_codes.len() * MAX_LARGE_INPUT_MULTIPLIER
    };

    if !max_size.is_power_of_two() {
        max_size = max_size.next_power_of_two();
    }

    let mut use_table = BitVec::with_capacity(max_size);

    let mut best_num_slots = 0;
    let mut best_num_collisions = hash_codes.len();

    let mut num_slots = min_size;
    while num_slots <= max_size {
        use_table.fill(false);
        let mut num_collisions = 0;

        for code in &hash_codes {
            let slot = (code % (num_slots as u64)) as usize;
            if use_table.get(slot) {
                num_collisions += 1;
                if num_collisions >= best_num_collisions {
                    break;
                }
            } else {
                use_table.set(slot, true);
            }
        }

        if num_collisions < best_num_collisions {
            if best_num_slots == 0 || num_collisions <= acceptable_collisions {
                best_num_collisions = num_collisions;
                best_num_slots = num_slots;
            }

            if num_collisions <= acceptable_collisions {
                // we have a winner!
                break;
            }
        }

        if acceptable_collisions > 0 {
            // The larger the table, the fewer collisions we tolerate. The idea
            // here is to reduce the risk of a table getting very big and still
            // having a relatively high count of collisions.
            acceptable_collisions =
                (acceptable_collisions / 100) * ACCEPTABLE_COLLISION_PERCENTAGE_OF_REDUCTION;
        }

        num_slots = (num_slots + 1).next_power_of_two();
    }

    HashCodeAnalysisResult {
        num_hash_slots: best_num_slots,
        num_hash_collisions: best_num_collisions,
    }
}

#[cfg(test)]
mod tests {
    use rand::rngs::StdRng;
    use rand::{Rng, SeedableRng};

    use super::*;

    struct AnalysisTestCase {
        num_hash_codes: usize,
        randomize_hash_codes: bool,
        expected_num_hash_slots: usize,
        expected_num_hash_collisions: usize,
    }

    #[test]
    fn analyze_hash_codes_test() {
        const ANALYSIS_TEST_CASES: [AnalysisTestCase; 5] = [
            AnalysisTestCase {
                num_hash_codes: 0,
                randomize_hash_codes: true,
                expected_num_hash_slots: 0,
                expected_num_hash_collisions: 0,
            },
            AnalysisTestCase {
                num_hash_codes: 2,
                randomize_hash_codes: true,
                expected_num_hash_slots: 2,
                expected_num_hash_collisions: 0,
            },
            AnalysisTestCase {
                num_hash_codes: 1000,
                randomize_hash_codes: true,
                expected_num_hash_slots: 1024,
                expected_num_hash_collisions: 349,
            },
            AnalysisTestCase {
                num_hash_codes: 8_000_000,
                randomize_hash_codes: false,
                expected_num_hash_slots: 8_388_608,
                expected_num_hash_collisions: 0,
            },
            AnalysisTestCase {
                num_hash_codes: 8_000_000,
                randomize_hash_codes: true,
                expected_num_hash_slots: 8_388_608,
                expected_num_hash_collisions: 2_843_788,
            },
        ];

        for case in &ANALYSIS_TEST_CASES {
            let mut rng = StdRng::seed_from_u64(42);
            let mut hash_codes = Vec::with_capacity(case.num_hash_codes);

            if case.randomize_hash_codes {
                for _ in 0..case.num_hash_codes {
                    hash_codes.push(rng.random());
                }
            } else {
                for count in 0..case.num_hash_codes {
                    hash_codes.push(count as u64);
                }
            }

            let result = analyze_hash_codes(hash_codes.iter().copied());

            assert_eq!(case.expected_num_hash_slots, result.num_hash_slots);
            assert_eq!(
                case.expected_num_hash_collisions,
                result.num_hash_collisions
            );
        }
    }
}
