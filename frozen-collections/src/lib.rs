//! Frozen collections: fast partially immutable collections
//!
//! Frozen collections are designed to trade creation time for improved
//! read performance. They are ideal for use with long-lasting collections
//! which get initialized when an application starts and remain unchanged
//! permanently, or at least extended periods of time. This is a common
//! pattern in service applications.
//!
//! During creation, the frozen collections perform analysis over the data they
//! will hold to determine the best layout and algorithm for the specific case.
//! This analysis can take some time. But the value in spending this time up front
//! is that the collections provide blazingly fast read-time performance.
//!
//! # Analysis and Optimizations
//!
//! Unlike normal collections, the frozen collections require you to provide all the data for
//! the collection when you create the collection. The data you supply is analyzed which determines
//! what specific underlying implementation strategy to use and how to lay out data in the hash tables
//! (assuming the implementation uses hash tables at all)
//!
//! The available implementation strategies are:
//!
//! - **Integer as Hash**. When the keys are of an integer type, this uses the keys themselves
//! as hash codes, avoiding the overhead of hashing.
//!
//! - **Integer Range**. When the keys represent a contiguous range of integer values, this eliminates
//! hashing completely and uses direct indexing instead.
//!
//! - **Length as Hash**. When the keys are of a slice type, this uses the length of the slices
//! as hash codes, avoiding the overhead of hashing.
//!
//! - **Left Hand Hashing**. When the keys are of a slice type, this uses sub-slices of the keys
//! for hashing, reducing the overhead of hashing.
//!
//! - **Right Hand Hashing**. Similar to the Right Hand Hashing from above, but using right-aligned
//! sub-slices instead.
//!
//! - **Linear Scan**. For very small maps, this avoids hashing completely by scanning through the
//! keys in linear order.
//!
//! - **Classic Hashing**. This is the fallback when none of the previous strategies apply. This
//! benefits from a read-optimized data layout which delivers faster performance than normal
//! collections.
//!
//! # Macros and Structs
//!
//! Frozen collections can be created in one of two ways:
//!
//! - **via Macros**. When you know the data to load into the collection at build time, you can use the
//! [`frozen_set!`] or [`frozen_map!`] macros. Analysis of the input
//! data is done at build time, so there isn't any analysis cost spent when creating the
//! collections.
//!
//! * **via Facade Types**. When you don't know the data to load into the collection at build time, you must use
//! the [`FrozenSet`] and [`FrozenMap`] types. Analysis is performed at runtime when
//! the collections are created, which can take a while if the payload is made up of millions of
//! entries.
//!
//! Using the macros results in faster runtime performance, so they are the preferred choice if
//! possible.
//!
//! The [`FrozenSet`] or [`FrozenMap`] types are facades which dispatch at runtime to
//! different specialized implementation types. When you use the macros instead, the selection
//! of implementation type is done at build time, and thus the cost of the runtime dispatch is
//! completely eliminated.
//!
//! The specialized collection types returned by the macros are slightly more feature-rich
//! than the [`FrozenSet`] and [`FrozenMap`] types. Specifically, the specialized
//! types support the [`Borrow`](std::borrow::Borrow) trait for key specification, which makes
//! them more flexible.
//!
//! # Implementation Limits
//!
//! Although frozen collections are always faster when reading than traditional collections, there are some
//! caveats to be aware of:
//!
//! - [`FrozenMap`] and [`FrozenSet`] have optimized implementations for the case where the keys are
//!   of type [`u32`], but not any other integer types. This limitation doesn't exist
//!   for the [`frozen_map!`](crate::frozen_map!) and [`frozen_set!`](crate::frozen_set!) macros.
//!
//! - [`FrozenMap`] and [`FrozenSet`] have optimized implementations for the case where the keys are
//!   of type [`String`], but not for the type `&str`. You will generally get considerably faster
//!   performance using [`String`].
//!
//! # Traits
//!
//! The frozen collections define three custom traits which you can use to
//! integrate with frozen collections:
//!
//! - [`Len`]. Used to represent keys that have lengths. This is used by the Length as Hash,
//! Left Hand Hashing, and Right Hand Hashing strategies.
//!
//! - [`RangeHash`]. Used to enable hashing of a sub-slice of a value. This is used by the
//! Left Hand Hashing and Right Hand Hashing strategies.
//!
//! - [`Set`]. Used to represent common features of a set. This makes it possible for
//! frozen collections to do logical operations, such as union or intersection, between various
//! concrete set types.

pub use frozen_collections_core::traits::Len;
pub use frozen_collections_core::traits::RangeHash;
pub use frozen_collections_core::traits::Set;
/// Create an optimized read-only map.
///
/// You give this macro the type of the map's keys, and then enumerate the key/value pairs that
/// should be added to the map. Analysis to select the implementation strategy and data layout
/// is done at build time.
///
/// This macro works by returning different implementation types based on the specific details
/// of the input data.
///
/// # Examples
///
/// ```
/// # use frozen_collections_macros::frozen_map;
/// #
/// let m = frozen_map!(&str,
///     "Red": 1,
///     "Green": 2,
///     "Blue": 3,
/// );
///
/// assert!(m.contains_key("Red"));
/// ```
pub use frozen_collections_macros::frozen_map;
/// Create an optimized read-only set.
///
/// You give this macro the type of the set's values, and then enumerate the values that
/// should be added to the set. Analysis to select the implementation strategy and data layout
/// is done at build time.
///
/// This macro works by returning different implementation types based on the specific details
/// of the input data.
///
/// # Examples
///
/// ```
/// # use frozen_collections_macros::frozen_set;
/// #
/// let s = frozen_set!(&str,
///     "Red",
///     "Green",
///     "Blue",
/// );
///
/// assert!(s.contains("Red"));
/// ```
pub use frozen_collections_macros::frozen_set;
pub use frozen_map::FrozenMap;
pub use frozen_set::FrozenSet;

mod frozen_map;
mod frozen_set;

#[cfg(test)]
mod frozen_map_tests;

#[cfg(test)]
mod frozen_set_tests;

#[doc(hidden)]
pub mod specialized_sets {
    pub use frozen_collections_core::specialized_sets::*;
}

#[doc(hidden)]
pub mod specialized_maps {
    pub use frozen_collections_core::specialized_maps::*;
}

/*
#[doc(hidden)]
pub mod traits {
    pub use frozen_collections_core::traits::*;
}
 */
