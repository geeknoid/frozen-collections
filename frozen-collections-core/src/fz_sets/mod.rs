//! Wrappers around other set types allowing runtime selection of implementation types based on input.

pub use fz_hash_set::FzHashSet;
pub use fz_ordered_set::FzOrderedSet;
pub use fz_scalar_set::FzScalarSet;
pub use fz_string_set::FzStringSet;

mod fz_hash_set;
mod fz_ordered_set;
mod fz_scalar_set;
mod fz_string_set;
