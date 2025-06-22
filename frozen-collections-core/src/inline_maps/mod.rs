//! Specialized static-friendly read-only map types.

pub use inline_binary_search_map::InlineBinarySearchMap;
pub use inline_dense_scalar_lookup_map::InlineDenseScalarLookupMap;
pub use inline_eytzinger_search_map::InlineEytzingerSearchMap;
pub use inline_hash_map::InlineHashMap;
pub use inline_hash_map_no_collisions::InlineHashMapNoCollisions;
pub use inline_scan_map::InlineScanMap;
pub use inline_sparse_scalar_lookup_map::InlineSparseScalarLookupMap;

mod inline_binary_search_map;
mod inline_dense_scalar_lookup_map;
mod inline_eytzinger_search_map;
mod inline_hash_map;
mod inline_hash_map_no_collisions;
mod inline_scan_map;
mod inline_sparse_scalar_lookup_map;
