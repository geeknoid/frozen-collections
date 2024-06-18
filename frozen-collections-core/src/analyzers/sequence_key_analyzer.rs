use crate::traits::Sequence;

/// How to treat integer keys for best performance.
#[derive(PartialEq, Eq, Debug)]
pub enum SequenceKeyAnalysisResult {
    /// No special optimization possible.
    General,

    /// All keys are in a continuous range.
    DenseRange,

    /// All keys are in a sparse range.
    SparseRange,
}

/// Look for well-known patterns we can optimize for with integer keys.
pub fn analyze_sequence_keys<K, I>(keys: I) -> SequenceKeyAnalysisResult
where
    K: Sequence,
    I: Iterator<Item = K>,
{
    const MAX_SPARSE_MULTIPLIER: usize = 10;
    const ALWAYS_SPARSE_THRESHOLD: usize = 128;

    let mut min = K::MAX;
    let mut max = K::MIN;
    let mut count = 0;
    for key in keys {
        min = min.min(key);
        max = max.max(key);
        count += 1;
    }

    if count == 0 {
        return SequenceKeyAnalysisResult::General;
    }

    let needed_count = K::count(&min, &max);
    if let Some(needed_count) = needed_count {
        if needed_count == count {
            return SequenceKeyAnalysisResult::DenseRange;
        } else if needed_count <= ALWAYS_SPARSE_THRESHOLD
            || needed_count < count.saturating_mul(MAX_SPARSE_MULTIPLIER)
        {
            return SequenceKeyAnalysisResult::SparseRange;
        }
    }

    SequenceKeyAnalysisResult::General
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_analyze_sequence_keys_empty() {
        let keys = vec![].into_iter();
        assert_eq!(
            analyze_sequence_keys::<i32, _>(keys),
            SequenceKeyAnalysisResult::General
        );
    }

    #[test]
    fn test_analyze_sequence_keys_dense_range() {
        let keys = 1..=5;
        assert_eq!(
            analyze_sequence_keys(keys),
            SequenceKeyAnalysisResult::DenseRange
        );
    }

    #[test]
    fn test_analyze_sequence_keys_sparse_range() {
        let keys = vec![1, 3, 5, 7, 128].into_iter();
        assert_eq!(
            analyze_sequence_keys(keys),
            SequenceKeyAnalysisResult::SparseRange
        );
    }

    #[test]
    fn test_analyze_sequence_keys_general() {
        let keys = vec![1, 2, 4, 8, 129].into_iter();
        assert_eq!(
            analyze_sequence_keys(keys),
            SequenceKeyAnalysisResult::General
        );
    }
}
