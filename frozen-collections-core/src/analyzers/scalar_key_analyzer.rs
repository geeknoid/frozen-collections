use crate::traits::Scalar;

/// How to treat integer keys for the best performance.
#[derive(PartialEq, Eq, Debug)]
pub enum ScalarKeyAnalysisResult {
    /// No special optimization possible.
    General,

    /// All keys are in a continuous range.
    DenseRange,

    /// All keys are in a sparse range.
    SparseRange,
}

/// Look for well-known patterns we can optimize for with integer keys.
#[mutants::skip]
pub fn analyze_scalar_keys<I>(keys: I) -> ScalarKeyAnalysisResult
where
    I: Iterator<Item: Scalar>,
{
    const MAX_SPARSE_MULTIPLIER: usize = 10;
    const ALWAYS_SPARSE_THRESHOLD: usize = 128;

    let mut min = usize::MAX;
    let mut max = usize::MIN;
    let mut count = 0;
    for key in keys {
        min = min.min(key.index());
        max = max.max(key.index());
        count += 1;
    }

    if count == 0 {
        return ScalarKeyAnalysisResult::General;
    }

    let needed_count = max - min + 1;
    if needed_count == count {
        ScalarKeyAnalysisResult::DenseRange
    } else if needed_count <= ALWAYS_SPARSE_THRESHOLD || needed_count < count.saturating_mul(MAX_SPARSE_MULTIPLIER) {
        ScalarKeyAnalysisResult::SparseRange
    } else {
        ScalarKeyAnalysisResult::General
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use alloc::vec;
    use alloc::vec::Vec;

    #[test]
    fn test_analyze_scalar_keys_empty() {
        let keys = Vec::<i32>::new().into_iter();
        assert_eq!(analyze_scalar_keys(keys), ScalarKeyAnalysisResult::General);
    }

    #[test]
    fn test_analyze_scalar_keys_dense_range() {
        let keys = 1..=5;
        assert_eq!(analyze_scalar_keys(keys), ScalarKeyAnalysisResult::DenseRange);
    }

    #[test]
    fn test_analyze_scalar_keys_sparse_range() {
        let keys = vec![1, 3, 5, 7, 128].into_iter();
        assert_eq!(analyze_scalar_keys(keys), ScalarKeyAnalysisResult::SparseRange);
    }

    #[test]
    fn test_analyze_scalar_keys_general() {
        let keys = vec![1, 2, 4, 8, 129].into_iter();
        assert_eq!(analyze_scalar_keys(keys), ScalarKeyAnalysisResult::General);
    }
}
