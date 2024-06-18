use std::collections::HashSet;
use std::hash::Hash;

/// Look for duplicate keys.
///
/// # Errors
///
/// This fails if any keys appear twice in the input.
pub fn check_duplicate_keys<'a, K, I>(keys: I) -> Result<(), &'static str>
where
    K: Hash + Eq + 'a,
    I: Iterator<Item = &'a K>,
{
    let mut s = HashSet::new();

    for key in keys {
        if !s.insert(key) {
            return Err("duplicate keys detected in input payload");
        }
    }

    Ok(())
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_no_duplicates() {
        let keys = [1, 2, 3, 4, 5];
        let result = check_duplicate_keys(keys.iter());
        assert!(result.is_ok());
    }

    #[test]
    fn test_with_duplicates() {
        let keys = [1, 2, 3, 3, 4];
        let result = check_duplicate_keys(keys.iter());
        assert!(result.is_err());
        assert_eq!(result, Err("duplicate keys detected in input payload"));
    }

    #[test]
    fn test_empty_input() {
        let keys: Vec<i32> = vec![];
        let result = check_duplicate_keys(keys.iter());
        assert!(result.is_ok());
    }

    #[test]
    fn test_single_element() {
        let keys = [1];
        let result = check_duplicate_keys(keys.iter());
        assert!(result.is_ok());
    }
}
