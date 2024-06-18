//! Wrappers around other set types allowing runtime selection of implementation type based on input.

pub use facade_hash_set::FacadeHashSet;
pub use facade_ordered_set::FacadeOrderedSet;
pub use facade_scalar_set::FacadeScalarSet;
pub use facade_string_set::FacadeStringSet;

mod facade_hash_set;
mod facade_ordered_set;
mod facade_scalar_set;
mod facade_string_set;
