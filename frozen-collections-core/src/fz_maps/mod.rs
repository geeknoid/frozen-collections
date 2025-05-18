//! Wrappers around other map types allowing runtime selection of implementation types based on input.

pub use fz_hash_map::FzHashMap;
pub use fz_ordered_map::FzOrderedMap;
pub use fz_scalar_map::FzScalarMap;
pub use fz_string_map::FzStringMap;

mod fz_hash_map;
mod fz_ordered_map;
mod fz_scalar_map;
mod fz_string_map;
