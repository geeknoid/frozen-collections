//! Specialized static-friendly read-only set types.

pub use binary_search_set::BinarySearchSet;
pub use dense_sequence_lookup_set::DenseSequenceLookupSet;
pub use hash_set::HashSet;
pub use iterators::*;
pub use ordered_scan_set::OrderedScanSet;
pub use scan_set::ScanSet;
pub use sparse_sequence_lookup_set::SparseSequenceLookupSet;

mod binary_search_set;
pub(crate) mod decl_macros;
mod dense_sequence_lookup_set;
mod hash_set;
mod iterators;
mod ordered_scan_set;
mod scan_set;
mod sparse_sequence_lookup_set;

#[cfg(test)]
mod set_tests;
