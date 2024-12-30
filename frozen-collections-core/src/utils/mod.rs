//! Utility functions and types for internal use.

pub use bitvec::*;
pub use dedup::*;
pub use eytzinger::*;

#[cfg(any(feature = "macros", feature = "emit"))]
pub use random::*;

mod bitvec;
mod dedup;
mod eytzinger;

#[cfg(any(feature = "macros", feature = "emit"))]
mod random;
