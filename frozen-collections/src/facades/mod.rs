//! Implementation crate for the frozen collections.
//!
//! # Compatibility Note
//!
//! This crate is not intended to be used directly. It is an implementation
//! detail of the frozen-collections crate. The API of this crate is therefore
//! not stable and may change at any time. If you need to use the functionality
//! of this crate, please use the `frozen-collections` crate instead which has
//! a stable API.

pub use fz_hash_map::FzHashMap;
pub use fz_hash_set::FzHashSet;
pub use fz_ordered_map::FzOrderedMap;
pub use fz_ordered_set::FzOrderedSet;
pub use fz_scalar_map::FzScalarMap;
pub use fz_scalar_set::FzScalarSet;
pub use fz_string_map::FzStringMap;
pub use fz_string_set::FzStringSet;

mod fz_hash_map;
mod fz_hash_set;
mod fz_ordered_map;
mod fz_ordered_set;
mod fz_scalar_map;
mod fz_scalar_set;
mod fz_string_map;
mod fz_string_set;
