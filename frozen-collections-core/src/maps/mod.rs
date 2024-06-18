//! Specialized read-only map types.

pub use binary_search_map::BinarySearchMap;
pub use dense_sequence_lookup_map::DenseSequenceLookupMap;
pub use hash_map::HashMap;
pub use iterators::*;
pub use ordered_scan_map::OrderedScanMap;
pub use scan_map::ScanMap;
pub use sparse_sequence_lookup_map::SparseSequenceLookupMap;

mod binary_search_map;
pub(crate) mod decl_macros;
mod dense_sequence_lookup_map;
mod hash_map;
mod hash_table;
mod iterators;
mod ordered_scan_map;
mod scan_map;
mod sparse_sequence_lookup_map;

#[cfg(test)]
mod map_tests;
