//! Traits to support frozen collections.

pub use crate::traits::collection_magnitude::{
    CollectionMagnitude, LargeCollection, MediumCollection, SmallCollection,
};
pub use crate::traits::hasher::Hasher;
pub use crate::traits::len::Len;
pub use crate::traits::map::Map;
pub use crate::traits::map_iterator::MapIterator;
pub use crate::traits::scalar::Scalar;
pub use crate::traits::set::Set;
pub use crate::traits::set_iterator::SetIterator;

mod collection_magnitude;
mod hasher;
mod len;
mod map;
mod map_iterator;
mod scalar;
mod set;
mod set_iterator;
