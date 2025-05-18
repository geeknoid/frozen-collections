use alloc::vec::Vec;
use core::cmp::{max, min};
use core::hash::{BuildHasher, Hash};
use core::ops::Range;
use hashbrown::HashMap as HashbrownMap;
use hashbrown::HashSet as HashbrownSet;

/// How to treat keys which are slices for the best performance.
#[derive(PartialEq, Eq, Debug)]
pub enum SliceKeyAnalysisResult {
    /// No special optimization possible.
    General,

    /// Hash left-justified subslices
    LeftHandSubslice(Range<usize>),

    /// Hash right-justified subslices
    RightHandSubslice(Range<usize>),

    /// Use the length of the slices as hash codes instead of hashing the slices
    Length,
}

/// Look for well-known patterns we can optimize for when keys are slices.
///
/// The idea here is to find the shortest subslice across all the input slices which are maximally unique. A corresponding
/// subslice range is then applied to incoming slices being hashed to perform lookups. Keeping the subslices as
/// short as possible minimizes the number of bytes involved in hashing, speeding up the whole process.
///
/// What we do here is pretty simple. We loop over the input slices, looking for the shortest subslice with a good
/// enough uniqueness factor. We look at all the slices both left-justified and right-justified as this maximizes
/// the opportunities to find unique subslices, especially in the case of many slices with the same prefix or suffix.
///
/// We also analyze the length of the input slices. If the lengths of the slices are sufficiently unique,
/// we can totally skip hashing and just use their lengths as hash codes.
pub fn analyze_slice_keys<'a, K, I, BH>(keys: I, bh: &BH) -> SliceKeyAnalysisResult
where
    K: Hash + Eq + 'a,
    I: Iterator<Item = &'a [K]>,
    BH: BuildHasher,
{
    let keys: Vec<&[K]> = keys.collect();

    // first, see if we can just use slice lengths as hash codes
    let result = analyze_lengths(&keys);

    if result == SliceKeyAnalysisResult::General {
        // if we can't use slice lengths, look for suitable subslices
        analyze_subslices(&keys, bh)
    } else {
        result
    }
}

/// See if we can use slice lengths instead of hashing
fn analyze_lengths<T>(keys: &Vec<&[T]>) -> SliceKeyAnalysisResult {
    const ACCEPTABLE_DUPLICATE_RATIO: usize = 20; // 5% duplicates are acceptable

    let max_identical = keys.len() / ACCEPTABLE_DUPLICATE_RATIO;
    let mut lengths = HashbrownMap::<usize, usize>::new();
    for s in keys {
        let v = lengths.get(&s.len());
        if let Some(count) = v {
            if *count >= max_identical {
                return SliceKeyAnalysisResult::General;
            }

            _ = lengths.insert(s.len(), count + 1);
        } else {
            _ = lengths.insert(s.len(), 1);
        }
    }

    SliceKeyAnalysisResult::Length
}

/// See if we can use subslices to reduce the time spent hashing
fn analyze_subslices<T, BH>(keys: &[&[T]], bh: &BH) -> SliceKeyAnalysisResult
where
    T: Hash + Eq,
    BH: BuildHasher,
{
    // constrain the amount of work we do in this code
    const MAX_SUBSLICE_LENGTH_LIMIT: usize = 16;
    const ACCEPTABLE_DUPLICATE_RATIO: usize = 20; // 5% duplicates are acceptable

    let mut min_len = usize::MAX;
    let mut max_len = 0;
    let mut prefix_len = usize::MAX;
    let mut suffix_len = usize::MAX;

    for s in keys {
        min_len = min(min_len, s.len());
        max_len = max(max_len, s.len());

        if s.len() < prefix_len {
            prefix_len = s.len();
        }

        if s.len() < suffix_len {
            suffix_len = s.len();
        }

        for i in 0..prefix_len {
            if s[i] != keys[0][i] {
                prefix_len = i;
                break;
            }
        }

        for i in 0..suffix_len {
            if s[s.len() - i - 1] != keys[0][keys[0].len() - i - 1] {
                suffix_len = i;
                break;
            }
        }
    }

    // tolerate a certain number of duplicate subslices
    let acceptable_duplicates = keys.len() / ACCEPTABLE_DUPLICATE_RATIO;

    // this set is reused for each call to is_sufficiently_unique
    let mut set = HashbrownSet::with_capacity(keys.len());

    // for each subslice length, prefer the shortest length that provides enough uniqueness
    let max_subslice_len = min(min_len, MAX_SUBSLICE_LENGTH_LIMIT);

    let mut subslice_len = 1;
    while subslice_len <= max_subslice_len {
        // For each index, get a uniqueness factor for the left-justified subslices.
        // If any is above our threshold, we're done.
        let mut subslice_index = prefix_len;
        while subslice_index <= min_len - subslice_len {
            if is_sufficiently_unique(
                keys,
                subslice_index,
                subslice_len,
                true,
                &mut set,
                acceptable_duplicates,
                bh,
            ) {
                return if subslice_len == max_len {
                    SliceKeyAnalysisResult::General
                } else {
                    SliceKeyAnalysisResult::LeftHandSubslice(
                        subslice_index..subslice_index + subslice_len,
                    )
                };
            }

            subslice_index += 1;
        }

        // There were no left-justified slices of this length available.
        // If all the slices are of the same length, then just checking left-justification is sufficient.
        // But if any slices are of different lengths, then we'll get different alignments for left- vs
        // right-justified subslices, and so we also check right-justification.
        if min_len != max_len {
            // For each index, get a uniqueness factor for the right-justified subslices.
            // If any is above our threshold, we're done.
            subslice_index = suffix_len;
            while subslice_index <= min_len - subslice_len {
                if is_sufficiently_unique(
                    keys,
                    subslice_index,
                    subslice_len,
                    false,
                    &mut set,
                    acceptable_duplicates,
                    bh,
                ) {
                    return SliceKeyAnalysisResult::RightHandSubslice(
                        subslice_index..subslice_index + subslice_len,
                    );
                }

                subslice_index += 1;
            }
        }

        subslice_len += 1;
    }

    // could not find a subslice that was good enough.
    SliceKeyAnalysisResult::General
}

fn is_sufficiently_unique<T, BH>(
    keys: &[&[T]],
    subslice_index: usize,
    subslice_len: usize,
    left_justified: bool,
    set: &mut HashbrownSet<u64>,
    mut acceptable_duplicates: usize,
    bh: &BH,
) -> bool
where
    T: Hash,
    BH: BuildHasher,
{
    set.clear();

    for s in keys {
        let sub = if left_justified {
            &s[subslice_index..subslice_index + subslice_len]
        } else {
            let start = s.len() - subslice_index - subslice_len;
            &s[start..start + subslice_len]
        };

        if !set.insert(bh.hash_one(sub)) {
            if acceptable_duplicates == 0 {
                return false;
            }

            acceptable_duplicates -= 1;
        }
    }

    true
}

#[cfg(test)]
mod tests {
    use super::*;
    use alloc::string::{String, ToString};
    use foldhash::fast::RandomState;

    struct AnalysisTestCase<'a> {
        slices: &'a [&'a str],
        expected: SliceKeyAnalysisResult,
    }

    #[test]
    fn analyze_string_keys_test() {
        const ANALYSIS_TEST_CASES: [AnalysisTestCase; 9] = [
            AnalysisTestCase {
                slices: &[
                    "AAA", "ABB", "ACC", "ADD", "AEE", "AFF", "AGG", "AHH", "AII", "AJJ", "AKK",
                    "ALL", "AMM", "ANN", "AOO", "APP", "AQQ", "ARR", "ASS", "ATT", "AUU",
                ],
                expected: SliceKeyAnalysisResult::LeftHandSubslice(1..2),
            },
            AnalysisTestCase {
                slices: &["A00", "B00", "C00", "D00"],
                expected: SliceKeyAnalysisResult::LeftHandSubslice(0..1),
            },
            AnalysisTestCase {
                slices: &["A", "B", "C", "D", "E2"],
                expected: SliceKeyAnalysisResult::LeftHandSubslice(0..1),
            },
            AnalysisTestCase {
                slices: &["A", "B", "C", "D", "E2", ""],
                expected: SliceKeyAnalysisResult::General,
            },
            AnalysisTestCase {
                slices: &["XA", "XB", "XC", "XD", "XE2"],
                expected: SliceKeyAnalysisResult::LeftHandSubslice(1..2),
            },
            AnalysisTestCase {
                slices: &["XXA", "XXB", "XXC", "XXD", "XXX", "XXXE"],
                expected: SliceKeyAnalysisResult::RightHandSubslice(0..1),
            },
            AnalysisTestCase {
                slices: &["ABC", "DEFG", "HIJKL", "MNOPQR", "STUVWXY", "YZ"],
                expected: SliceKeyAnalysisResult::Length,
            },
            AnalysisTestCase {
                slices: &[
                    "ABC", "DEFG", "HIJKL", "MNOPQR", "STUVWX", "YZ", "D2", "D3", "D4",
                ],
                expected: SliceKeyAnalysisResult::LeftHandSubslice(1..2),
            },
            AnalysisTestCase {
                slices: &["AAA", "1AA", "A1A", "AA1", "BBB", "1BB", "B1B", "BB1"],
                expected: SliceKeyAnalysisResult::General,
            },
        ];

        for case in &ANALYSIS_TEST_CASES {
            let keys = case.slices.iter().map(|x| x.as_bytes());
            assert_eq!(
                case.expected,
                analyze_slice_keys(keys, &RandomState::default())
            );
        }
    }

    #[test]
    fn out_of_range_bug() {
        let mut v = Vec::new();
        for i in 0..30 {
            let mut s = i.to_string();
            s.push('1');
            v.push(s);
        }

        let x = v.iter().map(String::as_bytes);
        let y = &RandomState::default();
        _ = analyze_slice_keys(x, y);
    }

    #[test]
    fn too_many_slices() {
        let mut v = Vec::new();
        for i in 0..300 {
            let mut s = i.to_string();
            s.push('1');
            v.push(s);
        }

        let x = v.iter().map(String::as_bytes);
        let y = &RandomState::default();

        assert_eq!(analyze_slice_keys(x, y), SliceKeyAnalysisResult::General);
    }
}
