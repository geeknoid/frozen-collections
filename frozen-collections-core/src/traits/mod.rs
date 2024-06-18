//! Traits to support frozen collections.

pub use crate::traits::len::Len;
pub use crate::traits::range_hash::RangeHash;
pub use crate::traits::set::Set;

mod len;
mod range_hash;
mod set;
