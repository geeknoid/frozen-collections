//! Specialized static-friendly read-only set types.

pub use inline_binary_search_set::InlineBinarySearchSet;
pub use inline_dense_sequence_lookup_set::InlineDenseSequenceLookupSet;
pub use inline_hash_set::InlineHashSet;
pub use inline_ordered_scan_set::InlineOrderedScanSet;
pub use inline_scan_set::InlineScanSet;
pub use inline_sparse_sequence_lookup_set::InlineSparseSequenceLookupSet;

mod inline_binary_search_set;
mod inline_dense_sequence_lookup_set;
mod inline_hash_set;
mod inline_ordered_scan_set;
mod inline_scan_set;
mod inline_sparse_sequence_lookup_set;
