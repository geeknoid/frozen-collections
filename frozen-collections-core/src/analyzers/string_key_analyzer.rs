/// The recommended key type for a string collection.
#[derive(PartialEq, Eq, Debug)]
pub enum StringKeyType {
    /// Use `CompactString` — enough keys fit in inline storage to justify the larger per-entry size.
    CompactString,

    /// Use `Box<str>` — most keys exceed the inline threshold, so `Box<str>` is more space-efficient.
    BoxStr,
}

/// Analyzes string key lengths to determine the optimal owned string type.
///
/// `CompactString` stores strings ≤ [`MAX_INLINE`] bytes inline (no heap allocation),
/// but each entry is 24 bytes on the stack versus 16 for `Box<str>` (fat pointer).
/// When most strings are long, the extra 8 bytes per entry adds up with no SSO benefit.
///
/// [`MAX_INLINE`]: Self::MAX_INLINE
///
/// Returns [`StringKeyType::CompactString`] when at least half of the keys fit inline,
/// [`StringKeyType::BoxStr`] otherwise.
#[expect(single_use_lifetimes, reason = "Required for MSRV compatibility")]
pub fn analyze_string_key_type<'a>(keys: impl Iterator<Item = &'a str>) -> StringKeyType {
    // CompactString's inline capacity: one byte for the discriminant/length tag.
    const MAX_INLINE: usize = size_of::<usize>() * 3 - 1;

    let mut total = 0usize;
    let mut inline_count = 0usize;

    for key in keys {
        total += 1;
        if key.len() <= MAX_INLINE {
            inline_count += 1;
        }
    }

    if total == 0 || inline_count * 2 >= total {
        StringKeyType::CompactString
    } else {
        StringKeyType::BoxStr
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn empty_defaults_to_compact() {
        let keys: Vec<&str> = vec![];
        assert_eq!(analyze_string_key_type(keys.into_iter()), StringKeyType::CompactString);
    }

    #[test]
    fn all_short_keys_use_compact() {
        let keys = vec!["a", "bb", "ccc", "dddd"];
        assert_eq!(analyze_string_key_type(keys.into_iter()), StringKeyType::CompactString);
    }

    #[test]
    fn all_long_keys_use_box() {
        let keys = vec![
            "this is a very long string key!",
            "another long string that exceeds limit",
            "yet another string exceeding the inline limit",
            "and one more long string for good measure",
        ];
        assert_eq!(analyze_string_key_type(keys.into_iter()), StringKeyType::BoxStr);
    }

    #[test]
    fn mixed_majority_short_uses_compact() {
        let keys = vec!["a", "bb", "ccc", "this is a very long string key!"];
        // 3 out of 4 are short (75%) >= 50% threshold
        assert_eq!(analyze_string_key_type(keys.into_iter()), StringKeyType::CompactString);
    }

    #[test]
    fn mixed_majority_long_uses_box() {
        let keys = vec![
            "a",
            "this is a very long string key!",
            "another long string that exceeds limit",
            "yet another string exceeding the inline limit",
        ];
        // 1 out of 4 short (25%) < 50% threshold
        assert_eq!(analyze_string_key_type(keys.into_iter()), StringKeyType::BoxStr);
    }

    #[test]
    fn exactly_half_uses_compact() {
        let keys = vec!["short", "also short", "this is a very long string key!", "another long string that exceeds limit"];
        // 2 out of 4 (50%) → uses CompactString (threshold is >=)
        assert_eq!(analyze_string_key_type(keys.into_iter()), StringKeyType::CompactString);
    }

    #[test]
    fn boundary_length_key() {
        // MAX_INLINE = 23 on 64-bit
        let max_inline = size_of::<usize>() * 3 - 1;
        let exactly_max: String = "x".repeat(max_inline);
        let one_over: String = "x".repeat(max_inline + 1);

        let keys = vec![exactly_max.as_str()];
        assert_eq!(analyze_string_key_type(keys.into_iter()), StringKeyType::CompactString);

        let keys = vec![one_over.as_str()];
        assert_eq!(analyze_string_key_type(keys.into_iter()), StringKeyType::BoxStr);
    }
}
