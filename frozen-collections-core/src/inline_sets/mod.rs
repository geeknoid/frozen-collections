//! Specialized static-friendly read-only set types.

pub use inline_dense_scalar_lookup_set::InlineDenseScalarLookupSet;
pub use inline_hash_set::InlineHashSet;
pub use inline_ordered_scan_set::InlineOrderedScanSet;
pub use inline_scan_set::InlineScanSet;
pub use inline_sparse_scalar_lookup_set::InlineSparseScalarLookupSet;

mod inline_dense_scalar_lookup_set;
mod inline_hash_set;
mod inline_ordered_scan_set;
mod inline_scan_set;
mod inline_sparse_scalar_lookup_set;