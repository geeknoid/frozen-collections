//! Traits to support frozen collections.

pub use crate::traits::collection_magnitude::{
    CollectionMagnitude, LargeCollection, MediumCollection, SmallCollection,
};
pub use crate::traits::hasher::Hasher;
pub use crate::traits::len::Len;
pub use crate::traits::map::Map;
pub use crate::traits::map_iteration::MapIteration;
pub use crate::traits::map_query::MapQuery;
pub use crate::traits::scalar::Scalar;
pub use crate::traits::set::Set;
pub use crate::traits::set_iteration::SetIteration;
pub use crate::traits::set_ops::SetOps;
pub use crate::traits::set_query::SetQuery;

mod collection_magnitude;
mod hasher;
mod len;
mod map;
mod map_iteration;
mod map_query;
mod scalar;
mod set;
mod set_iteration;
mod set_ops;
mod set_query;
