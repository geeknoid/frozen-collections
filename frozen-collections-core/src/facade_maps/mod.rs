//! Wrappers around other map types allowing runtime selection of implementation type based on input.

pub use facade_hash_map::FacadeHashMap;
pub use facade_ordered_map::FacadeOrderedMap;
pub use facade_scalar_map::FacadeScalarMap;
pub use facade_string_map::FacadeStringMap;

mod facade_hash_map;
mod facade_ordered_map;
mod facade_scalar_map;
mod facade_string_map;
