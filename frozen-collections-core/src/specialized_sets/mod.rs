//! Specialized read-only sets used as implementation details of frozen sets.

pub use common_set::{CommonSet, CommonSetNoCollisions};
pub use integer_range_set::IntegerRangeSet;
pub use integer_set::{IntegerSet, IntegerSetNoCollisions};
pub use iterators::*;
pub use length_set::{LengthSet, LengthSetNoCollisions};
pub use scanning_set::ScanningSet;
pub use slice_set::{
    LeftSliceSet, LeftSliceSetNoCollisions, RightSliceSet, RightSliceSetNoCollisions,
};

mod common_set;
mod integer_range_set;
mod integer_set;
mod iterators;
mod length_set;
mod scanning_set;
mod slice_set;
mod utils;
