//! Specialized read-only set types.

pub use binary_search_set::BinarySearchSet;
pub use dense_scalar_lookup_set::DenseScalarLookupSet;
pub use eytzinger_search_set::EytzingerSearchSet;
pub use hash_set::HashSet;
pub use iterators::*;
pub use ordered_scan_set::OrderedScanSet;
pub use scan_set::ScanSet;
pub use sparse_scalar_lookup_set::SparseScalarLookupSet;

mod binary_search_set;
pub(crate) mod decl_macros;
mod dense_scalar_lookup_set;
mod eytzinger_search_set;
mod hash_set;
mod iterators;
mod ordered_scan_set;
mod scan_set;
mod sparse_scalar_lookup_set;
