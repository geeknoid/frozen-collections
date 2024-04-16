use std::cmp::{max, min};
use std::collections::HashSet;
use std::hash::{Hash, Hasher};

enum CaseFolding {
    None,
    Ascii,
    Unicode,
}

pub(crate) struct KeyAnalysisResult {
    pub case_folding_for_hashing: CaseFolding,
    pub case_folding_for_comparison: CaseFolding,
    pub substring_index: usize,
    pub substring_count: usize,
}

/// Look for well-known patterns we can optimize for in a set of map or set keys.
///
/// The idea here is to find the shortest substring across all the input strings which are maximally unique. A corresponding
/// substring range is then applied to incoming strings being hashed to perform map or set lookups. Keeping the substrings as
/// small as possible minimizes the number of characters involved in hashing, speeding up the whole process.
///
/// What we do here is pretty simple. We loop over the input strings, looking for the shortest substring with a good
/// enough uniqueness factor. We look at all the strings both left-justified and right-justified as this maximizes
/// the opportunities to find unique substrings, especially in the case of many strings with the same prefix or suffix.
///
/// In whatever substring range we end up with, if all the characters involved are ASCII, and we're doing case-insensitive
/// operations, then we can select an ASCII-specific case-insensitive comparison which yields faster overall performance.
/// This is a common enough use case, so it's worth the effort.
///
/// TODO: this code should also analyze the length of the input strings. If the length of the strings is sufficiently unique,
///       we can totally skip hashing and just use the string lengths to index into the hash table.
pub(crate) fn analyze_string_keys(unique_strings: &[&str], ignore_case: bool) -> KeyAnalysisResult {
    let mut min_len = 0;
    let mut max_len = 0;
    for s in unique_strings {
        min_len = min(min_len, s.len());
        max_len = max(max_len, s.len())
    }

    if min_len >= 0 {
        let result = try_use_substring(unique_strings, ignore_case, min_len, max_len);
        if result.is_some() {
            return result.unwrap();
        }
    }

    create_analysis_results(unique_strings, ignore_case, 0, 0)
}

fn try_use_substring(
    unique_strings: &[&str],
    ignore_case: bool,
    min_len: usize,
    max_len: usize,
) -> Option<KeyAnalysisResult> {
    // constrain the amount of work we do in here
    const MAX_SUBSTRING_LENGTH_LIMIT: usize = 16;

    // A uniqueness factor of 95% is good enough.
    // Instead of ensuring that 95% of data is good, we stop when we know that at least 5% is bad.
    let acceptable_non_unique_count = unique_strings.len() / 20;

    // This set is reused for each "is sufficiently unique" check just to reuse the allocation.
    let mut scratch_set = HashSet::with_capacity(unique_strings.len());

    let mut control = SubstringControl {
        substring_index: 0,
        substring_count: 0,
        left_justified: false,
        ignore_case: false,
    };

    // for each substring length, prefer the shortest length that provides enough uniqueness
    let max_substring_len = min(min_len, MAX_SUBSTRING_LENGTH_LIMIT);

    let mut count = 1;
    while count <= max_substring_len {
        control.left_justified = true;
        control.substring_count = count;

        // For each index, get a uniqueness factor for the left-justified substrings.
        // If any is above our threshold, we're done.
        let mut index = 0;
        while index <= min_len - count {
            control.substring_index = index;

            if is_sufficiently_unique(
                &mut scratch_set,
                control,
                unique_strings,
                acceptable_non_unique_count,
            ) {
                return Some(create_analysis_results(
                    unique_strings,
                    ignore_case,
                    index,
                    count,
                ));
            }

            index += 1;
        }

        // There were no left-justified substrings of this length available.
        // If all the strings are of the same length, then just checking left-justification is sufficient.
        // But if any strings are of different lengths, then we'll get different alignments for left- vs
        // right-justified substrings, and so we also check right-justification.
        if min_len != max_len {
            control.left_justified = false;

            // For each index, get a uniqueness factor for the right-justified substrings.
            // If any is above our threshold, we're done.
            index = 0;
            while index <= min_len - count {
                control.substring_index = index - count;

                if is_sufficiently_unique(
                    &mut scratch_set,
                    control,
                    unique_strings,
                    acceptable_non_unique_count,
                ) {
                    return Some(create_analysis_results(
                        unique_strings,
                        ignore_case,
                        control.substring_index,
                        count,
                    ));
                }

                index += 1;
            }
        }

        count += 1;
    }

    // could not find a substring index/length that was good enough.
    None
}

fn test() {
    let mut set = HashSet::new();

    assert_eq!(set.insert(2), true);
    assert_eq!(set.insert(2), false);
    assert_eq!(set.len(), 1);
}

fn is_sufficiently_unique<'scratch, 'values>(
    set: &'scratch mut HashSet<Substring<'values>>,
    control: SubstringControl,
    unique_strings: &'values [&'values str],
    acceptable_num_unique_count: usize,
) -> bool {
    set.clear();

    let mut acceptable_num_unique_count = acceptable_num_unique_count;
    for s in unique_strings {
        if !set.insert(Substring {
            wrapped: s,
            control,
        }) {
            if acceptable_num_unique_count == 0 {
                return false;
            }

            acceptable_num_unique_count -= 1;
        }
    }

    true
}

/// Given the input strings and the unique substrings we found, identify some potential optimizations around ASCII and case-insensitivity.
fn create_analysis_results(
    unique_strings: &[&str],
    ignore_case: bool,
    substring_index: usize,
    substring_count: usize,
) -> KeyAnalysisResult {
    let (case_folding_for_hashing, case_folding_for_comparison) = analyze_case_folding_requirements(
        unique_strings,
        ignore_case,
        substring_index,
        substring_count,
    );

    KeyAnalysisResult {
        case_folding_for_hashing,
        case_folding_for_comparison,
        substring_index,
        substring_count,
    }
}

/// Determines what kind of case folding we should do. The first return value is the case folding to use during
/// hashing, the second one is what to use for equality comparisons.
fn analyze_case_folding_requirements(
    unique_strings: &[&str],
    ignore_case: bool,
    substring_index: usize,
    substring_count: usize,
) -> (CaseFolding, CaseFolding) {
    let mut case_folding_for_hashing;
    let mut case_folding_for_comparison;

    if ignore_case {
        // When ignoring case, we can optimize things based on whether we're dealing with just ASCII and/or just non-alphabetic symbols
        //
        // * If all the substrings aren't alphabetic characters, then we don't need case-folding for hashing
        //
        // * If all the substrings are ASCII with alphabetic characters, then we can hash with ASCII case folding.
        //
        // * If all the full strings aren't alphabetic characters, then we don't need case-folding for comparison
        //
        // * If all the full strings are ASCII with alphabetic characters, then we can compare with ASCII case folding.

        case_folding_for_hashing = CaseFolding::Unicode;
        case_folding_for_comparison = CaseFolding::Unicode;

        let mut any_alphabetic = false;
        let mut only_ascii = true;
        for unique_string in unique_strings {
            let hash_string =
                &unique_string[substring_index..(substring_index + substring_count + 1)];
            if is_ascii_alphabetic(&hash_string) {
                any_alphabetic = true;
            }

            if !hash_string.is_ascii() {
                only_ascii = false;
            }
        }

        if !any_alphabetic {
            case_folding_for_hashing = CaseFolding::None;
        } else if only_ascii {
            case_folding_for_hashing = CaseFolding::Ascii;
        }

        any_alphabetic = false;
        only_ascii = true;
        for unique_string in unique_strings {
            if is_ascii_alphabetic(&unique_string) {
                any_alphabetic = true;
            }

            if !unique_string.is_ascii() {
                only_ascii = false;
            }
        }

        if !any_alphabetic {
            case_folding_for_comparison = CaseFolding::None;
        } else if only_ascii {
            case_folding_for_comparison = CaseFolding::Ascii;
        }
    } else {
        case_folding_for_hashing = CaseFolding::None;
        case_folding_for_comparison = CaseFolding::None;
    }

    (case_folding_for_hashing, case_folding_for_comparison)
}

fn is_ascii_alphabetic(s: &str) -> bool {
    for c in s.chars() {
        if c.is_ascii_alphabetic() {
            return true;
        }
    }

    false
}

#[derive(Clone, Copy)]
struct SubstringControl {
    pub substring_index: usize,
    pub substring_count: usize,
    pub left_justified: bool,
    pub ignore_case: bool,
}

/// A string with parameters to control how it should be hashed and compared
struct Substring<'a> {
    pub wrapped: &'a str,
    pub control: SubstringControl,
}

impl<'a> PartialEq<Self> for Substring<'a> {
    fn eq(&self, other: &Self) -> bool {
        let s1: &str;
        let s2: &str;

        if self.control.left_justified {
            s1 = &self.wrapped[(self.wrapped.len() - self.control.substring_index)
                ..(self.control.substring_index + self.control.substring_count + 1)];
            s2 = &other.wrapped[(self.wrapped.len() - self.control.substring_index)
                ..(self.control.substring_index + self.control.substring_count + 1)];
        } else {
            s1 = &self.wrapped[self.control.substring_index
                ..(self.control.substring_index + self.control.substring_count + 1)];
            s2 = &other.wrapped[self.control.substring_index
                ..(self.control.substring_index + self.control.substring_count + 1)];
        }

        return if self.control.ignore_case {
            s1.to_uppercase().eq(&s2.to_uppercase())
        } else {
            s1.eq(s2)
        };
    }
}

impl<'a> Eq for Substring<'a> {}

impl<'a> Hash for Substring<'a> {
    fn hash<H: Hasher>(&self, state: &mut H) {
        let s: &str;

        if self.control.left_justified {
            s = &self.wrapped[(self.wrapped.len() - self.control.substring_index)
                ..(self.control.substring_index + self.control.substring_count + 1)];
        } else {
            s = &self.wrapped[self.control.substring_index
                ..(self.control.substring_index + self.control.substring_count + 1)];
        }

        return if self.control.ignore_case {
            s.to_uppercase().hash(state);
        } else {
            s.hash(state);
        };
    }
}
